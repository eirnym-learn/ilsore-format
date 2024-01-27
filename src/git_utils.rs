use std::borrow::Cow;
use std::env;
use std::path;
use std::path::Path;

use crate::error;
use crate::error::MapLog;
use crate::error::Result;
use crate::structs;
use crate::util::LastPart;

pub(crate) fn process_current_dir(
    options: &structs::GetGitInfoOptions,
) -> Result<structs::GitOutputOptions> {
    let git_dir_buf = git_subfolder(options)?
        .ok_or_else(|| error::Error::Message("Not found .git folder".to_string()))?;

    return process_repo(&git_dir_buf, options);
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
        return Err(error::Error::Message(format!(
            "Path '{}' doesn't exist",
            path.display()
        )));
    }

    for sub_path in path.ancestors() {
        let folder = sub_path.join(".git");
        if folder.exists() {
            return Ok(Some(sub_path.to_path_buf()));
        }
    }
    return Ok(None);
}

fn process_repo(
    path: &Path,
    options: &structs::GetGitInfoOptions,
) -> Result<structs::GitOutputOptions> {
    let repo = git2::Repository::open(path)?;
    let head_info = head_info(&repo, options).ok_or_log();
    let file_status = file_status(&repo, options).ok_or_log();
    let branch_ahead_behind = graph_ahead_behind(&repo, &head_info).ok_or_log();

    Ok(structs::GitOutputOptions {
        head_info,
        file_status,
        branch_ahead_behind,
    })
}

fn head_info(
    repo: &git2::Repository,
    options: &structs::GetGitInfoOptions,
) -> Result<structs::HeadInfo> {
    let detached = repo.head_detached().unwrap_or_default();
    let reference = repo.find_reference(options.reference_name.as_str())?;

    let head_info = match reference.kind() {
        None => structs::HeadInfo {
            reference_name: None,
            reference_short: None,
            oid: None,
            detached,
        },
        Some(git2::ReferenceType::Symbolic) => {
            let reference_resolved = reference.resolve().ok_or_log();
            let symbolic_target = reference.symbolic_target();
            let reference_name = symbolic_target.map(|v| String::from(v));
            let reference_short = symbolic_target
                .map(|v| v.last_part())
                .map(|v| String::from(v));

            structs::HeadInfo {
                reference_name,
                reference_short,
                oid: reference_resolved.map(|r| r.target()).flatten(),
                detached,
            }
        }
        Some(git2::ReferenceType::Direct) => {
            let symbolic_target = reference.symbolic_target();
            let reference_name = symbolic_target.map(|v| String::from(v));
            let reference_short = symbolic_target
                .map(|v| v.last_part())
                .map(|v| String::from(v));

            structs::HeadInfo {
                reference_name,
                reference_short,
                oid: reference.target(),
                detached,
            }
        }
    };
    return Ok(head_info);
}

fn file_status(
    repo: &git2::Repository,
    options: &structs::GetGitInfoOptions,
) -> Result<structs::FileStatus> {
    let status_options = &mut git2::StatusOptions::new();
    status_options.show(git2::StatusShow::IndexAndWorkdir);
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

    Ok(structs::FileStatus {
        conflict,
        untracked,
        typechange,
        unstaged,
        staged,
    })
}

fn graph_ahead_behind(
    repo: &git2::Repository,
    head: &Option<structs::HeadInfo>,
) -> Result<(bool, usize, usize)> {
    let reference: Option<&String> = head.as_ref().map(|h| h.reference_name.as_ref()).flatten();
    let head_oid: Option<&git2::Oid> = head.as_ref().map(|h| h.oid.as_ref()).flatten();

    if reference.is_none() || head_oid.is_none() {
        return Ok((false, 0, 0));
    }

    let tracking_branch_buf = repo.branch_upstream_name(reference.as_deref().unwrap())?;
    let tracking_branch = tracking_branch_buf.as_str();

    if tracking_branch.is_none() {
        return Err("tracking branch can't be converted to an UTF-8 string".into());
    }

    let tracking_reference = repo.find_reference(tracking_branch.unwrap())?;
    let tracking_oid = tracking_reference.target();

    if tracking_oid.is_none() {
        return Err("tracking branch {:?} has no oid".into());
    }

    let ahead_behind =
        repo.graph_ahead_behind(*head_oid.as_deref().unwrap(), tracking_oid.unwrap())?;

    return Ok((true, ahead_behind.0, ahead_behind.1));
}
