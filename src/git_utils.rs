use std::borrow::Cow;
use std::env;
use std::path;
use std::path::Path;
use std::thread;

use crate::error;
use crate::error::MapLog;
use crate::error::Result;
use crate::structs;
use crate::util::LastPart;

pub(crate) fn process_current_dir(
    options: &structs::GetGitInfoOptions,
) -> Result<structs::GitOutputOptions> {
    let git_dir_buf =
        git_subfolder(options)?.ok_or_else(|| error::Error::from("Not found .git folder"))?;

    process_repo(&git_dir_buf, options)
}

fn git_subfolder(options: &structs::GetGitInfoOptions) -> Result<Option<path::PathBuf>> {
    let path = options
        .start_folder
        .as_ref()
        .map(Path::new)
        .map(Cow::from)
        .map(Ok)
        .unwrap_or_else(|| env::current_dir().map(Cow::from))?;

    if !path.exists() {
        return Err(format!("Path '{}' doesn't exist", path.display()).into());
    }

    for sub_path in path.ancestors() {
        let folder = sub_path.join(".git");
        if folder.exists() {
            return Ok(Some(sub_path.to_path_buf()));
        }
    }
    Ok(None)
}

fn process_repo(
    path: &Path,
    options: &structs::GetGitInfoOptions,
) -> Result<structs::GitOutputOptions> {
    let mut head_info_result: Option<Option<structs::GitHeadInfo>> = Some(None);
    let mut branch_ahead_behind_result: Option<Option<structs::GitBranchAheadBehind>> = Some(None);
    let mut file_status_result: Option<Option<structs::GitFileStatus>> = Some(None);

    thread::scope(|s| {
        s.spawn(|| {
            let repo_option = git2::Repository::open(path).ok_or_log();
            if repo_option.is_none() {
                return;
            };
            let repo = repo_option.unwrap();
            let head_info_internal = head_info(&repo, options).ok_or_log();
            if options.include_ahead_behind {
                let _ = branch_ahead_behind_result
                    .insert(graph_ahead_behind(&repo, &head_info_internal).ok_or_log());
            };
            let _ = head_info_result.insert(head_info_internal.map(|h| h.into()));
        });

        s.spawn(|| {
            let repo_option = git2::Repository::open(path).ok_or_log();
            if repo_option.is_none() {
                return;
            };
            let repo = repo_option.unwrap();
            let _ = file_status_result.insert(file_status(&repo, options).ok_or_log());
        });
    });

    Ok(structs::GitOutputOptions {
        head_info: head_info_result.flatten(),
        file_status: file_status_result.flatten(),
        branch_ahead_behind: branch_ahead_behind_result.flatten(),
    })
}

#[derive(Debug)]
struct GitHeadInfoInternal {
    pub reference_name: Option<String>,
    pub oid: Option<git2::Oid>,
    pub detached: bool,
}

impl Into<structs::GitHeadInfo> for GitHeadInfoInternal {
    fn into(self) -> structs::GitHeadInfo {
        let reference_short = self
            .reference_name
            .map(|v| v.as_str().last_part().to_string());
        let oid_short = self.oid.map(|v| v.to_string()[0..8].to_string());

        structs::GitHeadInfo {
            reference_short,
            oid_short,
            detached: self.detached,
        }
    }
}

fn head_info(
    repo: &git2::Repository,
    options: &structs::GetGitInfoOptions,
) -> Result<GitHeadInfoInternal> {
    let detached = repo.head_detached().unwrap_or_default();
    let reference = repo.find_reference(options.reference_name)?;

    let head_info = match reference.kind() {
        None => GitHeadInfoInternal {
            reference_name: None,
            oid: None,
            detached,
        },
        Some(git2::ReferenceType::Symbolic) => {
            let reference_name = reference.symbolic_target().map(String::from);

            let reference_resolved = reference.resolve().ok_or_log();
            let oid = reference_resolved.and_then(|r| r.target());

            GitHeadInfoInternal {
                reference_name,
                oid,
                detached,
            }
        }
        Some(git2::ReferenceType::Direct) => {
            let reference_name = reference.name().map(String::from);
            let oid = reference.target();

            GitHeadInfoInternal {
                reference_name,
                oid,
                detached,
            }
        }
    };
    Ok(head_info)
}

fn file_status(
    repo: &git2::Repository,
    options: &structs::GetGitInfoOptions,
) -> Result<structs::GitFileStatus> {
    let status_options = &mut git2::StatusOptions::new();
    let status_show = match options.include_workdir {
        true => git2::StatusShow::IndexAndWorkdir,
        false => git2::StatusShow::Index,
    };
    status_options.show(status_show);
    status_options.no_refresh(options.refresh_status);
    status_options.update_index(options.refresh_status);
    status_options.exclude_submodules(!options.include_submodules);
    status_options.include_ignored(false);
    status_options.include_unreadable(false);
    status_options.include_untracked(options.include_untracked);

    let statuses = repo.statuses(Some(status_options))?;

    let statuses_all = statuses
        .iter()
        .map(|s| s.status())
        .reduce(|a, b| a.union(b))
        .unwrap_or(git2::Status::empty());

    let mut conflict = false;
    let mut staged = false;
    let mut unstaged = false;
    let mut untracked = false;
    let mut typechange = false;

    for status in statuses_all {
        match status {
            git2::Status::CURRENT => conflict = true,
            git2::Status::INDEX_NEW => staged = true,
            git2::Status::INDEX_MODIFIED => staged = true,
            git2::Status::INDEX_DELETED => staged = true,
            git2::Status::INDEX_RENAMED => staged = true,
            git2::Status::INDEX_TYPECHANGE => staged = true,
            git2::Status::WT_NEW => untracked = true,
            git2::Status::WT_MODIFIED => unstaged = true,
            git2::Status::WT_DELETED => unstaged = true,
            git2::Status::WT_TYPECHANGE => typechange = true,
            git2::Status::WT_RENAMED => unstaged = true,
            git2::Status::IGNORED => (),
            git2::Status::CONFLICTED => conflict = true,
            _ => (),
        }
    }

    Ok(structs::GitFileStatus {
        conflict,
        untracked,
        typechange,
        unstaged,
        staged,
    })
}

fn graph_ahead_behind(
    repo: &git2::Repository,
    head: &Option<GitHeadInfoInternal>,
) -> Result<structs::GitBranchAheadBehind> {
    let reference: Option<&String> = head.as_ref().and_then(|h| h.reference_name.as_ref());
    let head_oid: Option<&git2::Oid> = head.as_ref().and_then(|h| h.oid.as_ref());

    if reference.is_none() || head_oid.is_none() {
        return Err("tracking branch doesn't exist".into());
    }

    let tracking_branch_buf = repo.branch_upstream_name(reference.unwrap())?;
    let tracking_branch = tracking_branch_buf.as_str();

    if tracking_branch.is_none() {
        return Err("tracking branch can't be converted to an UTF-8 string".into());
    }

    let tracking_reference = repo.find_reference(tracking_branch.unwrap())?;
    let tracking_oid = tracking_reference.target();

    if tracking_oid.is_none() {
        return Err("tracking branch {:?} has no oid".into());
    }

    let ahead_behind = repo.graph_ahead_behind(*head_oid.unwrap(), tracking_oid.unwrap())?;

    Ok(structs::GitBranchAheadBehind {
        ahead: ahead_behind.0,
        behind: ahead_behind.1,
    })
}
