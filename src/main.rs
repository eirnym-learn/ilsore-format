use std::env;
use std::io;
use std::path;

mod error;
mod structs;

type Result<T, E = error::Error> = std::result::Result<T, E>;

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
    let _ = process_current_dir()?;
    Ok(())
}

fn process_current_dir() -> Result<(), error::Error> {
    let git_dir_buf = git_subfolder()?
        .ok_or_else(|| error::Error::Message("Not found .git folder".to_string()))?;

    println!(
        "Folder {:?} is repo: {:?}",
        git_dir_buf,
        git_dir_buf.exists()
    );
    process_repo(&git_dir_buf)?;
    Ok(())
}

fn process_repo(path: &path::PathBuf) -> Result<()> {
    let repo = git2::Repository::open(path)?;
    let head = head_info(&repo)?;
    let file_status = file_status(&repo)?;

    println!("head info: {:?}", head);
    println!("file status: {:?}", file_status);

    print_type_of(&head);
    // let full_name = head.map(|h| h.full_name);
    let branch_ahead_behind: Option<(usize, usize)> = graph_ahead_behind(&repo, &head)?;

    print_type_of(&branch_ahead_behind);
    // println("branch_ahead_behind: {:?}", branch_ahead_behind);
    //    graph_ahead_behind(
    Ok(())
}

fn head_info(repo: &git2::Repository) -> Result<Option<structs::HeadInfo>> {
    let head = repo.head()?;
    let is_detached = repo.head_detached().ok().unwrap_or_default();
    let oid = head.target();

    Ok(Some(structs::HeadInfo {
        full_name: head.name().map(|oid| oid.to_string()),
        name: head.shorthand().map(|oid| oid.to_string()),
        oid,
        detached: is_detached,
    }))
}

fn file_status(repo: &git2::Repository) -> Result<structs::FileStatus> {
    let status_options = &mut git2::StatusOptions::new();
    status_options.show(git2::StatusShow::IndexAndWorkdir);
    status_options.exclude_submodules(true); // TODO: Investigate it further
    status_options.include_ignored(false);
    status_options.include_unreadable(false);
    status_options.include_untracked(true); // TODO: make an option for that

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
) -> Result<Option<(usize, usize)>> {
    let full_name: Option<&String> = head.as_ref().map_or_else(|| None, |h| h.full_name.as_ref());
    let head_oid: Option<&git2::Oid> = head.as_ref().map_or_else(|| None, |h| h.oid.as_ref());

    if full_name.is_none() || head_oid.is_none() {
        return Ok(None);
    }

    let tracking_branch_buf = repo.branch_upstream_name(full_name.as_deref().unwrap())?;
    let tracking_branch = tracking_branch_buf.as_str();
    if tracking_branch.is_none() {
        return Err("tracking branch can't be converted to an UTF-8 string".into());
    }
    print_type_of(&tracking_branch);
    println!("tracking branch is {:?}", tracking_branch);
    let tracking_reference = repo.find_reference(tracking_branch.unwrap())?;
    let tracking_oid = tracking_reference.target();
    if (tracking_oid.is_none()) {
        return Err("tracking branch {:?} has no oid".into());
    }
    let ahead_behind =
        repo.graph_ahead_behind(*head_oid.as_deref().unwrap(), tracking_oid.unwrap())?;
    println!("ahead-behind: {:?}", ahead_behind);

    Ok(Some((0, 0)))
}
