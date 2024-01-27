#[derive(Debug)]
pub(crate) struct GetGitInfoOptions {
    pub start_folder: Option<String>,
    pub reference: String,
    pub include_submodules: bool,
    pub include_untracked: bool,
}

#[derive(Debug)]
pub(crate) struct HeadInfo {
    pub reference: Option<String>,
    pub oid: Option<git2::Oid>, // Short oid
    pub detached: bool,
}

#[derive(Debug)]
pub(crate) struct FileStatus {
    pub conflict: bool,
    pub untracked: bool,
    pub typechange: bool,
    pub unstaged: bool,
    pub staged: bool,
}

#[derive(Debug)]
pub(crate) struct OutputOptions {
    pub head_info: Option<HeadInfo>,
    pub file_status: Option<FileStatus>,
    pub branch_ahead_behind: Option<(usize, usize)>,
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
