#[derive(Debug)]
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

#[derive(Debug)]
pub(crate) struct DateTime {
    pub date: String,
    pub time: String,
}

#[derive(Debug)]
pub(crate) struct ThemeData {
    pub datetime: DateTime,
    pub hostname: std::ffi::OsString,
    pub username: Option<std::ffi::OsString>,
    pub python: Option<String>,
    pub git: Option<GitOutputOptions>,
}

#[derive(Debug)]
pub(crate) struct ThemeSymbols {
    pub git_branch: &'static str,
    pub git_has_no_upstream: &'static str,
    pub git_branch_detached: &'static str,
    pub git_has_commits_up: &'static str,
    pub git_has_commits_down: &'static str,
    pub git_has_conflict: &'static str,
    pub git_has_untracked: &'static str,
    pub git_has_typechange: &'static str,
    pub git_has_unstaged: &'static str,
    pub git_has_staged: &'static str,
}

impl Default for GetGitInfoOptions {
    fn default() -> Self {
        GetGitInfoOptions {
            start_folder: None,
            reference_name: "HEAD".to_string(),
            include_submodules: false,
            include_untracked: true,
        }
    }
}

impl ThemeSymbols {
    pub(crate) fn utf_power() -> Self {
        ThemeSymbols {
            git_branch: "\u{e0a0}", // 
            git_has_no_upstream: "ᛘ",
            git_branch_detached: "\u{2630}", // ☰
            git_has_commits_up: "↑",
            git_has_commits_down: "↓",
            git_has_conflict: "✘",
            git_has_untracked: "?",
            git_has_typechange: "‡",
            git_has_unstaged: "●",
            git_has_staged: "●",
        }
    }
    pub(crate) fn utf() -> Self {
        ThemeSymbols {
            git_branch: "ᚠ",
            git_has_no_upstream: "ᛘ",
            git_branch_detached: "\u{2630}", // ☰
            git_has_commits_up: "↑",
            git_has_commits_down: "↓",
            git_has_conflict: "✘",
            git_has_untracked: "?",
            git_has_typechange: "‡",
            git_has_unstaged: "●",
            git_has_staged: "●",
        }
    }

    pub(crate) fn ascii() -> Self {
        ThemeSymbols {
            git_branch: "",
            git_has_no_upstream: "&",
            git_branch_detached: "||",
            git_has_commits_up: "^",
            git_has_commits_down: "v",
            git_has_conflict: "x",
            git_has_untracked: "?",
            git_has_typechange: "T",
            git_has_unstaged: "*",
            git_has_staged: "*",
        }
    }
}
