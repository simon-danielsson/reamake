const HELP_CONTENTS: &str = include_str!("../static/help.txt");

pub const APP_NAME: &str = env!("CARGO_PKG_NAME");
pub const APP_VERS: &str = env!("CARGO_PKG_VERSION");
pub const APP_REPO: &str = env!("CARGO_PKG_REPOSITORY");
pub const APP_DESC: &str = env!("CARGO_PKG_DESCRIPTION");
pub const APP_AUTH: &str = env!("CARGO_PKG_AUTHORS");
// *brakoll - d: add nice help command content and the same in the readme, p: 100, t: docs, s: closed
// *brakoll - d: change help readme and init file, p: 100, t: docs, s: closed
// *brakoll - d: fix formatting of .reamake file example in readme, p: 20, t: docs, s: closed
// *brakoll - d: fix formatting and add new details readme and help.txt, p: 20, t: docs, s: closed
// *brakoll - d: add disclaimer that this is work in progress, p: 100, t: docs, s: closed
// *brakoll - d: fix typo in readme, p: 10, t: docs, s: closed
pub fn print() {
    println!("");
    println!("{n} v{v}", n = APP_NAME, v = APP_VERS);
    println!("{APP_AUTH}");
    println!("{APP_REPO}");
    println!("{APP_DESC}");
    println!("");
    print!("{}", HELP_CONTENTS);
}
