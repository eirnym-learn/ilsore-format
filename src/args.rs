use std::path;
use std::sync::OnceLock;

use crate::structs;

pub(crate) static THEME_SYMBOLS: OnceLock<
    enum_map::EnumMap<ThemeSymbolsNames, structs::ThemeSymbols>,
> = OnceLock::new();

#[derive(clap::Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub(crate) struct Cli {
    /// Set if hostname is already known
    #[arg(long, value_name = "HOSTNAME", default_value = None)]
    pub static_hostname: Option<String>,

    /// Don't retrieve git information
    #[arg(long, value_name = "INCLUDE", default_value_t = false)]
    pub disable_git: bool,

    /// Git reference to get information for
    #[arg(long, value_name = "REFERENCE", default_value = "HEAD")]
    pub git_reference: Option<String>,

    /// Working directory to start to search for git information. Default is current folder
    #[arg(long, value_name = "FOLDER")]
    pub git_start_folder: Option<path::PathBuf>,

    /// If git status should include submodules
    #[arg(long, value_name = "INCLUDE", default_value_t = false, action=clap::ArgAction::SetTrue)]
    pub git_include_submodules: bool,

    /// If git status should include untracked files
    #[arg(long, value_name = "INCLUDE", default_value_t = true, action=clap::ArgAction::SetFalse)]
    pub git_exclude_untracked: bool,

    /// If git status should softly refresh indices
    #[arg(long, value_name = "INCLUDE", default_value_t = false, action=clap::ArgAction::SetTrue)]
    pub git_refresh_status: bool,

    /// Last command exit status
    #[arg(long, value_name = "ERROR_CODE", default_value_t = 0)]
    pub last_exit_status: u8,

    /// Theme symbols to use
    #[arg(long, value_name = "SYMBOLS", default_value_t, value_enum)]
    pub theme_symbols: ThemeSymbolsNames,
}

#[derive(clap::ValueEnum, Clone)] // required for clap::ValueEnum
#[derive(Debug)] // for clap parser
#[derive(Default)] // for set default in easier way
#[derive(enum_map::Enum, Copy)] // for EnumMap[] operator
#[clap(rename_all = "kebab_case")]
pub(crate) enum ThemeSymbolsNames {
    #[default]
    Utf8Power,
    Utf8,
    Ascii,
}

pub(crate) fn init_theme_symbols() {
    let _ = THEME_SYMBOLS.get_or_init(|| {
        enum_map::enum_map! {
            ThemeSymbolsNames::Utf8Power => structs::ThemeSymbols::utf8_power(),
            ThemeSymbolsNames::Utf8 => structs::ThemeSymbols::utf8(),
            ThemeSymbolsNames::Ascii => structs::ThemeSymbols::ascii(),
        }
    });
}

impl Cli {
    pub fn symbols(&self) -> &structs::ThemeSymbols {
        &THEME_SYMBOLS.get().expect("Uninitialized symbols")[self.theme_symbols]
    }
}
