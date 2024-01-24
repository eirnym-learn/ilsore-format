#[derive(Debug)]
pub(crate) struct GetGitInfoOptions {
    pub(crate) start_folder: Option<String>,
    pub(crate) reference: String,
    pub(crate) include_submodules: bool,
    pub(crate) include_untracked: bool,
}

#[derive(Debug)]
pub(crate) struct HeadInfo {
    pub(crate) reference: Option<String>,
    pub(crate) oid: Option<git2::Oid>, // Short oid
    pub(crate) detached: bool,
}

#[derive(Debug)]
pub(crate) struct FileStatus {
    pub(crate) conflict: bool,
    pub(crate) untracked: bool,
    pub(crate) typechange: bool,
    pub(crate) unstaged: bool,
    pub(crate) staged: bool,
}

#[derive(Debug)]
pub(crate) struct OutputOptions {
    pub(crate) head_info: Option<HeadInfo>,
    pub(crate) file_status: Option<FileStatus>,
    pub(crate) branch_ahead_behind: Option<(usize, usize)>,
}

impl GetGitInfoOptions {
    pub(crate) fn new() -> Self {
        GetGitInfoOptions {
            start_folder: None,
            reference: "HEAD".to_string(),
            include_submodules: false,
            include_untracked: true,
        }
    }
}
