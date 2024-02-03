use std::path;
use std::sync::OnceLock;

use crate::ilsore_format;
use crate::ilsore_format_color;
use crate::structs;

static THEME_SYMBOLS: OnceLock<enum_map::EnumMap<ThemeSymbolsNames, structs::ThemeSymbols>> =
    OnceLock::new();

static THEME_NAMES: OnceLock<enum_map::EnumMap<ThemeNames, ThemeFunction>> = OnceLock::new();

type ThemeFunction =
    for<'a, 'b> fn(&'a structs::ThemeData, &'b structs::ThemeSymbols) -> std::string::String;

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
    #[arg(long, default_value_t = false, action=clap::ArgAction::SetTrue)]
    pub git_include_submodules: bool,

    /// If git status should include untracked files
    #[arg(long, default_value_t = true, action=clap::ArgAction::SetFalse)]
    pub git_exclude_untracked: bool,

    /// If git status should softly refresh indices
    #[arg(long, default_value_t = false, action=clap::ArgAction::SetTrue)]
    pub git_refresh_status: bool,

    /// If git status won't check tracking branch
    #[arg(long, default_value_t = true, action=clap::ArgAction::SetFalse)]
    pub git_exclude_ahead_behind: bool,

    /// Exclude workdir file stats leaving query index only
    #[arg(long, default_value_t = false, action=clap::ArgAction::SetTrue)]
    pub git_exclude_stats_workdir: bool,

    /// Last command exit status
    #[arg(long, value_name = "ERROR_CODE", default_value_t = 0)]
    pub last_exit_status: u8,

    /// Theme symbols to use
    #[arg(long, value_name = "SYMBOLS", default_value_t, value_enum)]
    pub theme_symbols: ThemeSymbolsNames,

    /// Theme to use
    #[arg(long, value_name = "THEME", default_value_t, value_enum)]
    pub theme_name: ThemeNames,
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

#[derive(clap::ValueEnum, Clone)] // required for clap::ValueEnum
#[derive(Debug)] // for clap parser
#[derive(Default)] // for set default in easier way
#[derive(enum_map::Enum, Copy)] // for EnumMap[] operator
#[clap(rename_all = "kebab_case")]
pub(crate) enum ThemeNames {
    #[default]
    IlsoreColor,
    IlsoreNoColor,
}

pub(crate) fn init_argument_parser() {
    let _ = THEME_NAMES.get_or_init(|| {
        enum_map::enum_map! {
            ThemeNames::IlsoreColor => ilsore_format_color::format_ilsore_color,
            ThemeNames::IlsoreNoColor => ilsore_format::format_ilsore_no_color,
        }
    });

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
        &THEME_SYMBOLS.get().expect("Uninitialized theme symbols")[self.theme_symbols]
    }

    pub fn theme(&self) -> ThemeFunction {
        THEME_NAMES.get().expect("Uninitialized theme names")[self.theme_name]
    }
}
