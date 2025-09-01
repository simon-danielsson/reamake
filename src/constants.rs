/// Fallback: BPM
pub const DEF_BPM: u32 = 120;
/// Fallback: Client name
pub const DEF_CLI: &str = "Default Client";
/// Fallback: Project name
pub const DEF_PRO: &str = "Default Project";
/// Fallback: RPP project template
pub const DEF_RPP: &str = include_str!("../templates/default.RPP");
/// Fallback: .yaml structure template
pub const DEF_YAM: &str = include_str!("../templates/default.yaml");
/// Default CSV file for init subcommand
pub const DEF_CSV: &str = include_str!("../templates/default.csv");

/// Reamake project version
pub const RMK_VER: &str = env!("CARGO_PKG_VERSION");
