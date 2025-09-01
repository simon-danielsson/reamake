// Fallback defaults at omitted user input
pub const DEF_BPM: u32 = 120;
pub const DEF_CLI: &str = "Default Client";
pub const DEF_PRO: &str = "Default Project";
pub const DEF_RPP: &str = include_str!("../templates/default.RPP");
pub const DEF_CSV: &str = include_str!("../templates/default.csv");
pub const DEF_YAM: &str = include_str!("../templates/default.yaml");

// Version
pub const RMK_VER: &str = env!("CARGO_PKG_VERSION");
