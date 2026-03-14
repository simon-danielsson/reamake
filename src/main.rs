// *brakoll - d: initial setup, p: 0, t: feature, s: closed

use std::{
    env, fmt, fs, io,
    path::{Path, PathBuf},
};

use crate::utils::parser::Node;

mod subcommands;
mod utils;

fn main() -> io::Result<()> {
    // *brakoll - d: implement flag -f <file.reamake>, p: 60, t: feature, s: closed
    // *brakoll - d: implement flag -c <client> flag -p <project> and flag -s <service>, p: 50, t: feature, s: open
    // *brakoll - d: subcommand to normalize and fold audio files in target directory, p: 20, t: feature, s: open
    // *brakoll - d: the def operation will be to create project in cd but add arg for a target directory as well (automatically recognized as a path by the arg parser), p: 100, t: feature, s: closed
    // === get args ===
    let args = utils::args::parse()?;

    if args.help {
        subcommands::help::print();
        return Ok(());
    }

    if args.reamake_file_path.is_empty() {
        eprintln!(
        "You didn't supply a path to a reamake file!\nUse the 'help' subcommand if you're feeling lost."
    );
        return Ok(());
    }

    let mut r = Reamake::new();

    // parse batch file
    let block = r.parse_reamake_file(args.reamake_file_path)?;
    r.project_blocks.push(block);

    for p in r.project_blocks {
        generate_structure(&p.target_dir, &p.hierarchy, &p)?;
        // p.debug_print();
    }

    Ok(())
}

#[derive(Debug)]
enum DateFormat {
    US,
    EU,
    ISO,
}
impl fmt::Display for DateFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            DateFormat::US => "US",
            DateFormat::EU => "EU",
            DateFormat::ISO => "ISO",
        };
        write!(f, "{s}")
    }
}

#[derive(Debug)]
pub struct ProjectBlock {
    // arg
    target_dir: PathBuf,
    // variables
    var_client: String,
    var_project: String,
    var_service: String,
    // sources
    src_rpp: String,
    // settings
    set_kebab: bool,
    set_date: DateFormat,
    // hierarchy
    hierarchy: Vec<Node>,
}

impl ProjectBlock {
    fn new() -> io::Result<Self> {
        Ok(Self {
            //arg
            target_dir: env::current_dir()?,
            // variables
            var_client: String::new(),
            var_project: String::new(),
            var_service: String::new(),
            // sources
            src_rpp: String::new(),
            // settings
            set_date: DateFormat::EU,
            set_kebab: false,
            // hierarchy
            hierarchy: Vec::new(),
        })
    }

    #[allow(dead_code)]
    fn debug_print(&mut self) {
        println!("[variables]");
        println!("client: {}", self.var_client);
        println!("project: {}", self.var_project);
        println!("service: {}", self.var_service);
        println!("[sources]");
        println!("rpp: {}", self.src_rpp);
        println!("[settings]");
        println!("date: {}", self.set_date);
        println!("text: {}", self.set_kebab);
        println!("[hierarchy]");
        println!("{:#?}", self.hierarchy);
    }
}

#[allow(dead_code)]
struct Reamake {
    project_blocks: Vec<ProjectBlock>,
}

impl Reamake {
    fn new() -> Self {
        Self {
            project_blocks: Vec::new(),
        }
    }
}

// *brakoll - d: generation of folder structure is not getting far down in the ast, p: 90, t: fix, s: closed
pub fn generate_structure(
    root: impl AsRef<Path>,
    nodes: &[Node],
    b: &ProjectBlock,
) -> io::Result<()> {
    let root = root.as_ref();
    fs::create_dir_all(root)?;

    for node in nodes {
        materialize_node(root, node, b)?;
    }

    Ok(())
}

fn materialize_node(base: &Path, node: &Node, b: &ProjectBlock) -> io::Result<()> {
    match node {
        Node::Folder { name, children } => {
            let dir = base.join(name);
            fs::create_dir_all(&dir)?;
            println!("folder created");

            for child in children {
                materialize_node(&dir, child, b)?;
            }
        }

        Node::File { name } => {
            let path = base.join(name);

            if let Some(parent) = path.parent() {
                fs::create_dir_all(parent)?;
            }
            println!("file created");

            fs::File::create(path)?;
        }

        Node::Rpp { name } => {
            let mut filename = name.clone();

            if !filename.ends_with(".rpp") {
                filename.push_str(".rpp");
            }

            let path = base.join(filename);

            if let Some(parent) = path.parent() {
                fs::create_dir_all(parent)?;
            }
            println!("rpp created");

            if b.src_rpp.trim().is_empty() {
                fs::File::create(path)?;
            } else {
                fs::copy(&b.src_rpp, path)?;
            }
        }
    }

    Ok(())
}