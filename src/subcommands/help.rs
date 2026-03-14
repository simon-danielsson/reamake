const HELP_CONTENTS: &str = include_str!("../static/help.txt");

pub const APP_NAME: &str = env!("CARGO_PKG_NAME");
pub const APP_VERS: &str = env!("CARGO_PKG_VERSION");
pub const APP_REPO: &str = env!("CARGO_PKG_REPOSITORY");
pub const APP_DESC: &str = env!("CARGO_PKG_DESCRIPTION");
pub const APP_AUTH: &str = env!("CARGO_PKG_AUTHORS");

// *brakoll - d: add nice help command content and the same in the readme, p: 100, t: docs, s: closed
pub fn print() {
    println!("");
    println!("{n} v{v}", n = APP_NAME, v = APP_VERS);
    println!("{APP_AUTH}");
    println!("{APP_REPO}");
    println!("{APP_DESC}");
    println!("");
    print!("{}", HELP_CONTENTS);
}