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
        .map(|v| format!("[{}{}{RESET_COLOR}]", format_color_bold("green"), v));

    let git = data.git.as_ref().map(|v| format_ilsore_git(v, symbols));

    let last_status = if data.last_exit_status != 0 {
        format!(
            "[{}{}{RESET_COLOR}]",
            format_color_bold("red"),
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
fn format_color(color: &str) -> String {
    format!("%{{%F{{{color}}}%}}")
}

#[inline]
fn format_color_bold(color: &str) -> String {
    format!("%{{%B%F{{{color}}}%}}")
}

#[inline]
fn format_ilsore_git(data: &structs::GitOutputOptions, symbols: &structs::ThemeSymbols) -> String {
    if data.head_info.is_none() {
        return "".to_string();
    }

    // TODO: if no symbols - don't add space, join?
    format!(
        "({}Git:{RESET_COLOR} {} {})",
        format_color("magenta"),
        format_ilsore_git_head_info(&data.head_info, symbols)
            .as_deref()
            .unwrap_or_default(),
        format_ilsore_git_symbols(
            &data.head_info,
            &data.file_status,
            &data.branch_ahead_behind,
            symbols
        )
    )
}

#[inline]
fn format_ilsore_git_head_info<'a>(
    head_info: &'a Option<structs::GitHeadInfo>,
    symbols: &'a structs::ThemeSymbols,
) -> Option<String> {
    head_info.as_ref().and_then(|h| {
        h.reference_short
            .as_ref()
            .map(|v| {
                format!(
                    "{}{} {}{RESET_COLOR}",
                    format_color_bold("yellow"),
                    symbols.git_branch,
                    v
                )
            })
            .or(h
                .oid_short
                .as_ref()
                .map(|o| format!("{}{}{RESET_COLOR}", format_color_bold("magenta"), o)))
    })
}

#[inline]
fn format_ilsore_git_symbols(
    head_info: &Option<structs::GitHeadInfo>,
    file_status: &Option<structs::GitFileStatus>,
    branch_ahead_behind: &Option<structs::GitBranchAheadBehind>,
    symbols: &structs::ThemeSymbols,
) -> String {
    // TODO: if b.detached and other symbols - add space. join?
    format!(
        " {}{}{}{}{}{}{}{}{}{RESET_COLOR}",
        symbol(
            branch_ahead_behind.is_none(),
            symbols.git_has_no_upstream,
            "red"
        ),
        symbol(
            head_info.as_ref().map_or(false, |b| b.detached),
            symbols.git_branch_detached,
            "magenta",
        ),
        symbol(
            branch_ahead_behind.as_ref().map_or(false, |b| b.ahead > 0),
            symbols.git_is_ahead,
            "magenta"
        ),
        symbol(
            file_status.as_ref().map_or(false, |b| b.unstaged),
            symbols.git_has_unstaged,
            "red"
        ),
        symbol(
            branch_ahead_behind.as_ref().map_or(false, |b| b.behind > 0),
            symbols.git_is_behind,
            "green"
        ),
        symbol(
            file_status.as_ref().map_or(false, |b| b.typechange),
            symbols.git_has_typechange,
            "magenta"
        ),
        symbol(
            file_status.as_ref().map_or(false, |b| b.staged),
            symbols.git_has_staged,
            "green"
        ),
        symbol(
            file_status.as_ref().map_or(false, |b| b.conflict),
            symbols.git_has_conflict,
            "red"
        ),
        symbol(
            file_status.as_ref().map_or(false, |b| b.untracked),
            symbols.git_has_untracked,
            "magenta"
        ),
    )
}

#[inline]
fn symbol(present: bool, symbol: &'static str, color: &'static str) -> String {
    match present {
        true => format!("{}{}", format_color_bold(color), symbol),
        false => "".to_string(),
    }
}
