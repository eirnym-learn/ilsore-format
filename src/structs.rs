#[derive(Debug, Default)]
pub(crate) struct GetGitInfoOptions {
    pub start_folder: Option<String>,
    pub reference_name: String,
    pub include_submodules: bool,
    pub include_untracked: bool,
}

#[derive(Debug)]
pub(crate) struct HeadInfo {
    pub reference_name: Option<String>,
    pub reference_short: Option<String>,
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
pub(crate) struct GitOutputOptions {
    pub head_info: Option<HeadInfo>,
    pub file_status: Option<FileStatus>,
    pub branch_ahead_behind: Option<(bool, usize, usize)>,
}

impl GetGitInfoOptions {
    pub(crate) fn default() -> Self {
        GetGitInfoOptions {
            start_folder: None,
            reference_name: "HEAD".to_string(),
            include_submodules: false,
            include_untracked: true,
        }
    }
}
