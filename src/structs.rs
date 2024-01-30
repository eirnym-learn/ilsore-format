#[derive(Debug)]
pub(crate) struct GetGitInfoOptions {
    pub start_folder: Option<String>,
    pub reference_name: String,
    pub include_submodules: bool,
    pub include_untracked: bool,
    pub no_refresh: bool,
}

#[derive(Debug)]
pub(crate) struct GitOutputOptions {
    pub head_info: Option<GitHeadInfo>,
    pub file_status: Option<GitFileStatus>,
    pub branch_ahead_behind: Option<GitBranchAheadBehind>,
}

#[derive(Debug)]
pub(crate) struct DateTime {
    pub date: String,
    pub time: String,
}

#[derive(Debug)]
pub(crate) struct ThemeData {
    pub datetime: DateTime,
    pub hostname: Option<String>,
    pub username: Option<String>,
    pub python: Option<String>,
    pub git: Option<GitOutputOptions>,
}

#[derive(Debug)]
pub(crate) struct GitHeadInfo {
    pub reference_short: Option<String>,
    pub oid_short: Option<String>,
    pub detached: bool,
}

#[derive(Debug)]
pub(crate) struct GitFileStatus {
    pub conflict: bool,
    pub untracked: bool,
    pub typechange: bool,
    pub unstaged: bool,
    pub staged: bool,
}

#[derive(Debug)]
pub(crate) struct GitBranchAheadBehind {
    pub ahead: usize,
    pub behind: usize,
}

#[derive(Debug)]
pub(crate) struct ThemeSymbols {
    pub git_branch: &'static str,
    pub git_has_no_upstream: &'static str,
    pub git_branch_detached: &'static str,
    pub git_is_ahead: &'static str,
    pub git_is_behind: &'static str,
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
            no_refresh: true,
        }
    }
}

impl ThemeSymbols {
    pub(crate) fn utf_power() -> Self {
        ThemeSymbols {
            git_branch: "\u{e0a0}", // 
            git_has_no_upstream: "ᛘ ",
            git_branch_detached: "\u{2630}", // ☰
            git_is_ahead: "↑",
            git_is_behind: "↓",
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
            git_has_no_upstream: "ᛘ ",
            git_branch_detached: "\u{2630}", // ☰
            git_is_ahead: "↑",
            git_is_behind: "↓",
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
            git_is_ahead: "^",
            git_is_behind: "v",
            git_has_conflict: "x",
            git_has_untracked: "?",
            git_has_typechange: "T",
            git_has_unstaged: "*",
            git_has_staged: "*",
        }
    }
}
