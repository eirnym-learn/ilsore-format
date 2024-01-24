#[derive(Debug)]
pub struct StatusOptions {
    include_untracked: bool,
    include_ignored: bool,
    include_submodules: bool,
    include_unreadable: bool,
}

#[derive(Debug)]
pub struct HeadInfo {
    pub(crate) full_name: Option<String>,
    pub(crate) name: Option<String>,
    pub(crate) oid: Option<git2::Oid>, // Short oid
    pub(crate) detached: bool,
}

#[derive(Debug)]
pub struct FileStatus {
    pub(crate) conflict: bool,
    pub(crate) untracked: bool,
    pub(crate) typechange: bool,
    pub(crate) unstaged: bool,
    pub(crate) staged: bool,
}

#[derive(Debug)]
pub struct OutputOptions {
    head_options: Option<HeadInfo>,
    file_status: Option<FileStatus>,
}
