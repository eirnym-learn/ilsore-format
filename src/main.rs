use std::env;
use std::io;
use std::path;

mod error;
mod structs;

type Result<T, E = error::Error> = std::result::Result<T, E>;

static mut VERBOSE_ERRORS: bool = true;

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

fn git_subfolder() -> io::Result<Option<path::PathBuf>> {
    let path = env::current_dir()?;
    for sub_path in path.ancestors() {
        let folder = sub_path.join(".git");
        if folder.exists() {
            return Ok(Some(sub_path.to_path_buf()));
        }
    }
    return Ok(None);
}

fn main() -> Result<()> {
    let full_name = Some(String::from("hello world"));
    let short_name: Option<String> = full_name
        .as_ref()
        .map(|v| v.strip_prefix("refs/heads/").unwrap_or(v))
        .map(|v| String::from(v));

    let a = error_control(process_current_dir())?;
    println!("{:?}", a);
    Ok(())
}

fn error_control<T, E: std::fmt::Debug>(result: Result<T, E>) -> Result<Option<T>> {
    if result.is_ok() {
        return Ok(result.ok());
    }

    let err: E = result.err().unwrap();
    unsafe {
        if VERBOSE_ERRORS == true {
            println!("{:?}", err);
        }
    }

    return Ok(None);
}

fn process_current_dir() -> Result<structs::OutputOptions> {
    let git_dir_buf = git_subfolder()?
        .ok_or_else(|| error::Error::Message("Not found .git folder".to_string()))?;

    return process_repo(&git_dir_buf);
}

fn process_repo(path: &path::PathBuf) -> Result<structs::OutputOptions> {
    let repo_opt = git2::Repository::open(path);

    if repo_opt.is_err() {
        return Err(error::Error::from(repo_opt.err().unwrap()));
    }
    let repo = repo_opt.unwrap();
    let head_info = error_control(head_info(&repo))?;
    let file_status = error_control(file_status(&repo))?;
    let branch_ahead_behind = error_control(graph_ahead_behind(&repo, &head_info))?;

    Ok(structs::OutputOptions {
        head_info,
        file_status,
        branch_ahead_behind,
    })
}

fn head_info(repo: &git2::Repository) -> Result<structs::HeadInfo> {
    let detached = repo.head_detached().ok().unwrap_or_default();
    // TODO: add an option for reference name to lookup
    let reference = repo.find_reference("HEAD")?;

    let head_info = match reference.kind() {
        None => structs::HeadInfo {
            reference: None,
            oid: None,
            detached,
        },
        Some(git2::ReferenceType::Symbolic) => {
            let reference_resolved = reference.resolve().ok();
            structs::HeadInfo {
                reference: reference.symbolic_target().map(|v| String::from(v)),
                oid: reference_resolved.map(|r| r.target()).flatten(),
                detached,
            }
        }
        Some(git2::ReferenceType::Direct) => structs::HeadInfo {
            reference: reference.symbolic_target().map(|v| String::from(v)),
            oid: reference.target(),
            detached,
        },
    };
    return Ok(head_info);
}

fn file_status(repo: &git2::Repository) -> Result<structs::FileStatus> {
    let status_options = &mut git2::StatusOptions::new();
    status_options.show(git2::StatusShow::IndexAndWorkdir);
    // TODO: add an option to exclude submodules (include)
    status_options.exclude_submodules(true);
    status_options.include_ignored(false);
    status_options.include_unreadable(false);
    // TODO: add an option to filter out untracked
    status_options.include_untracked(true);

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
) -> Result<(usize, usize)> {
    let reference: Option<&String> = head.as_ref().map(|h| h.reference.as_ref()).flatten();
    let head_oid: Option<&git2::Oid> = head.as_ref().map(|h| h.oid.as_ref()).flatten();

    if reference.is_none() || head_oid.is_none() {
        return Ok((0, 0));
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

    return Ok((ahead_behind));
}
