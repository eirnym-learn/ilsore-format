use crate::structs;

pub(crate) fn format_ilsore_no_color(
    data: &structs::ThemeData,
    symbols: &structs::ThemeSymbols,
) -> String {
    let empty_string: String = "".to_string();

    let date_time = format!("[{} {}]", data.datetime.date, data.datetime.time);

    let user_host = format!(
        "{}@{}",
        data.username.as_ref().unwrap_or(&empty_string),
        data.hostname
    );
    let python = format_ilsore_python(&data.python);
    let git = data
        .git
        .as_ref()
        .map_or(empty_string, |v| format_ilsore_git(v, symbols));

    format!("{}{}{}{}", date_time, user_host, python, git)
}

fn format_ilsore_python(data: &Option<String>) -> String {
    let empty_string: String = "".to_string();

    data.as_ref().map_or(empty_string, |v| format!("[{}]", v))
}
#[inline]
fn format_ilsore_git(data: &structs::GitOutputOptions, symbols: &structs::ThemeSymbols) -> String {
    let empty_string: String = "".to_string();

    if data.head_info.is_none() {
        return empty_string;
    }

    format!(
        "(Git: {} {})",
        format_ilsore_git_head_info(&data.head_info, symbols),
        format_ilsore_git_symbols(
            &data.head_info,
            &data.file_status,
            &data.branch_ahead_behind,
            &symbols
        )
    )
}

#[inline]
fn format_ilsore_git_head_info(
    head_info: &Option<structs::GitHeadInfo>,
    symbols: &structs::ThemeSymbols,
) -> String {
    let empty_string: String = "".to_string();

    head_info
        .as_ref()
        .map(|h| {
            h.reference_short
                .as_ref()
                .map(|v| format!("{} {}", symbols.git_branch, v))
                .or(h.oid_short.as_ref().map(String::to_string))
        })
        .flatten()
        .unwrap_or(empty_string)
}

#[inline]
fn format_ilsore_git_symbols(
    head_info: &Option<structs::GitHeadInfo>,
    file_status: &Option<structs::GitFileStatus>,
    branch_ahead_behind: &Option<structs::GitBranchAheadBehind>,
    symbols: &structs::ThemeSymbols,
) -> String {
    format!(
        "{}{}{}{}{}{}{}{}{}",
        symbol(
            head_info.as_ref().map_or(false, |b| b.detached),
            symbols.git_branch_detached
        ),
        symbol(branch_ahead_behind.is_none(), symbols.git_has_no_upstream),
        symbol(
            branch_ahead_behind.as_ref().map_or(false, |b| b.ahead > 0),
            symbols.git_is_ahead
        ),
        symbol(
            branch_ahead_behind.as_ref().map_or(false, |b| b.behind > 0),
            symbols.git_is_behind
        ),
        symbol(
            file_status.as_ref().map_or(false, |b| b.conflict),
            symbols.git_has_conflict
        ),
        symbol(
            file_status.as_ref().map_or(false, |b| b.untracked),
            symbols.git_has_untracked
        ),
        symbol(
            file_status.as_ref().map_or(false, |b| b.typechange),
            symbols.git_has_typechange
        ),
        symbol(
            file_status.as_ref().map_or(false, |b| b.unstaged),
            symbols.git_has_unstaged
        ),
        symbol(
            file_status.as_ref().map_or(false, |b| b.staged),
            symbols.git_has_staged
        ),
    )
}

#[inline]
fn symbol(present: bool, symbol: &'static str) -> String {
    let value = match present {
        true => Some(symbol),
        false => None,
    };
    value.unwrap_or_default().to_string()
}
