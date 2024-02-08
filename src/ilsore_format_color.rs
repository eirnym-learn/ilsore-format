use crate::structs;

static RESET_COLOR: &str = "%{[0m%}";

pub(crate) fn format_ilsore_color(
    data: &structs::ThemeData,
    symbols: &structs::ThemeSymbols,
) -> String {
    let date_time = format!(
        "[{}{}{RESET_COLOR} {}{}{RESET_COLOR}]",
        format_color("165"),
        data.datetime.date,
        format_color("226"),
        data.datetime.time,
    );

    let user_host = format!(
        "{}{}{RESET_COLOR}@{}{}{RESET_COLOR}",
        format_color("214"),
        data.username.as_deref().unwrap_or_default(),
        format_color("46"),
        data.hostname.as_deref().unwrap_or_default(),
    );

    let python = data
        .python
        .as_ref()
        .map(|v| format!("[{}{}{RESET_COLOR}]", format_color_bold("42"), v));

    let git = data.git.as_ref().map(|v| format_ilsore_git(v, symbols));

    let last_status = if data.last_exit_status != 0 {
        format!(
            "[{}{}{RESET_COLOR}]",
            format_color_bold("196"),
            data.last_exit_status
        )
    } else {
        "".to_string()
    };

    format!(
        "{}{}{}{}{}\n{}%~{RESET_COLOR}>",
        date_time,
        user_host,
        last_status,
        python.as_deref().unwrap_or_default(),
        git.as_deref().unwrap_or_default(),
        format_color("87"),
    )
}

#[inline]
fn format_color(color: &str) -> String { // TODO: write macro
    format!("%{{%F{{{color}}}%}}")
}

#[inline]
fn format_color_bold(color: &str) -> String { // TODO: write macro
    format!("%{{%B%F{{{color}}}%}}")
}

#[inline]
fn format_ilsore_git(data: &structs::GitOutputOptions, symbols: &structs::ThemeSymbols) -> String {
    if data.head_info.is_none() {
        return "".to_string();
    }

    let git_info = vec![
        data.head_info
            .as_ref()
            .and_then(|h| format_ilsore_git_branch(h, symbols))
            .unwrap_or_default(),
        format_ilsore_git_symbols(
            &data.head_info,
            &data.file_status,
            &data.branch_ahead_behind,
            symbols,
        )
        .unwrap_or_default(),
    ];

    format!(
        "({}Git: {}{RESET_COLOR})",
        format_color("magenta"),
        git_info.join(" ")
    )
}

#[inline]
fn format_ilsore_git_branch(
    head_info: &structs::GitHeadInfo,
    symbols: &structs::ThemeSymbols,
) -> Option<String> {
    if head_info.reference_short.is_none() && head_info.oid_short.is_none() {
        return None;
    };
    if head_info.reference_short.is_none() || head_info.detached {
        Some(format!(
            "{}{}{RESET_COLOR}",
            format_color_bold("201"),
            head_info.oid_short.as_deref().unwrap_or_default()
        ))
    } else {
        Some(format!(
            "{}{} {}{RESET_COLOR}",
            format_color_bold("226"),
            symbols.git_branch,
            head_info.reference_short.as_deref().unwrap_or_default()
        ))
    }
}

#[inline]
fn format_ilsore_git_symbols(
    head_info: &Option<structs::GitHeadInfo>,
    file_status: &Option<structs::GitFileStatus>,
    branch_ahead_behind: &Option<structs::GitBranchAheadBehind>,
    symbols: &structs::ThemeSymbols,
) -> Option<String> {
    let detached = head_info.as_ref().map_or(false, |b| b.detached);
    let no_upstream = branch_ahead_behind.is_none();
    let is_ahead = branch_ahead_behind.as_ref().map_or(false, |b| b.ahead > 0);
    let is_behind = branch_ahead_behind.as_ref().map_or(false, |b| b.behind > 0);
    let has_staged = file_status.as_ref().map_or(false, |b| b.staged);
    let has_unstaged = file_status.as_ref().map_or(false, |b| b.unstaged);
    let has_typechange = file_status.as_ref().map_or(false, |b| b.typechange);
    let has_conflict = file_status.as_ref().map_or(false, |b| b.conflict);
    let has_untracked = file_status.as_ref().map_or(false, |b| b.untracked);

    let detached_branch_symbols = vec![match (detached, no_upstream) {
        (true, _) => symbol_bold(true, symbols.git_branch_detached, "26"),
        (false, true) => symbol_bold(true, symbols.git_has_no_upstream, "red"),
        (false, false) => Some(
            vec![
                symbol_bold(is_ahead, symbols.git_is_ahead, "magenta"),
                symbol_bold(is_behind, symbols.git_is_behind, "green"),
            ]
            .i_join(),
        ),
    }];

    let file_status_symbols = vec![
        symbol_bold(has_staged, symbols.git_has_staged, "green"),
        symbol_bold(has_unstaged, symbols.git_has_unstaged, "red"),
        symbol_bold(has_typechange, symbols.git_has_typechange, "magenta"),
        symbol_bold(has_conflict, symbols.git_has_conflict, "red"),
        symbol(has_untracked, symbols.git_has_untracked, "magenta"),
    ];

    let result_data = vec![
        detached_branch_symbols.i_join(),
        file_status_symbols.i_join(),
    ];

    let result = result_data.join(" "); // TODO: spaces at the end

    if !result.is_empty() {
        Some(format!("{}{RESET_COLOR}", result))
    } else {
        None
    }
}

#[inline]
fn symbol_bold(present: bool, symbol: &'static str, color: &'static str) -> Option<String> {
    match present {
        true => Some(format!("{}{}", format_color_bold(color), symbol)),
        false => None,
    }
}

#[inline]
fn symbol(present: bool, symbol: &'static str, color: &'static str) -> Option<String> {
    match present {
        true => Some(format!("{}{}", format_color(color), symbol)),
        false => None,
    }
}

trait Joiner {
    fn i_join(&self) -> String;
}

impl Joiner for Vec<Option<String>> {
    fn i_join(&self) -> String {
        return self.iter().filter_map(|p| p.as_deref()).collect::<String>();
    }
}
