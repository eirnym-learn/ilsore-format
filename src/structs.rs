use std::path;

/// Options for git status reporter
#[derive(Debug)]
pub(crate) struct GetGitInfoOptions<'a> {
    /// Start forlder. None value means current folder
    pub start_folder: &'a Option<path::PathBuf>,

    /// Reference name to ask information for
    pub reference_name: &'a str,

    /// Flag if git status should include submodules information
    pub include_submodules: bool,

    /// Flag if git status should include untracked files
    pub include_untracked: bool,

    /// Flag if git status should do soft refresh
    pub refresh_status: bool,

    /// Flag if git status should include ahead/behind information
    pub include_ahead_behind: bool,

    /// Flag if git status should include workdir check
    pub include_workdir_stats: bool,
}

#[derive(Debug)]
pub(crate) struct ThemeData {
    pub last_exit_status: u8,
    pub datetime: DateTime,
    pub hostname: Option<String>,
    pub username: Option<String>,
    pub python: Option<String>,
    pub git: Option<GitOutputOptions>,
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
impl ThemeSymbols {
    pub(crate) fn utf8_power() -> Self {
        ThemeSymbols {
            git_branch: "\u{e0a0}",          // 
            git_has_no_upstream: "\u{25B2}", // ▲
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
    pub(crate) fn utf8() -> Self {
        ThemeSymbols {
            git_branch: "ᚠ",
            git_has_no_upstream: "ᛘ",
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
