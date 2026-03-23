// *brakoll - d: initial setup, p: 0, t: feature, s: closed
use std::collections::HashMap;

use std::{
    env, fs, io,
    path::{Path, PathBuf},
};

use crate::parser::{HierarchyNode, Parser, Settings, VarType, VariableDecl, lex};
use crate::utils::args::Arguments;
use crate::utils::messages;

mod parser;
mod subcommands;
mod utils;

// *brakoll - d: give the user power to add any variables they want to their folder structure (with keywords to show if it's a name a file or a folder), p: 0, t: feature, s: closed
// *brakoll - d: add logic so that if a reamake file has not been added to the template file or if the path is invalid generate a fallback, p: 0, t: fix, s: closed
fn main() -> io::Result<()> {
    // *brakoll - d: implement flag -f <file.reamake>, p: 60, t: feature, s: closed
    // *brakoll - d: implement flag -c <client> flag -p <project> and flag -s <service>, p: 50, t: feature, s: closed
    // *brakoll - d: subcommand to normalize and fold audio files in target directory, p: 20, t: feature, s: open
    // *brakoll - d: the def operation will be to create project in cd but add arg for a target directory as well (automatically recognized as a path by the arg parser), p: 100, t: feature, s: closed
    // === get args ===
    let args = utils::args::parse()?;

    // *brakoll - d: add stem sorting subcommand, p: 10, t: feat, s: closed
    if args.sort {
        subcommands::sort::run(&args)?;
        return Ok(());
    }

    if args.norm {
        subcommands::norm::run(&args)?;
        return Ok(());
    }

    if args.init {
        subcommands::init::gen_init(args)?;
        return Ok(());
    }

    if args.help {
        subcommands::help::print();
        return Ok(());
    }

    // *brakoll - d: add extra check for .reamake extension before parsing reamake file, p: 100, t: fix, s: closed
    if args.reamake_file_path.is_empty() {
        eprintln!("{}", messages::NO_PATH_ASSIGNED);

        return Ok(());
    } else if !args.reamake_file_path.trim().ends_with(".reamake") {
        eprintln!("{}", messages::NO_PATH_ASSIGNED);
        return Ok(());
    }

    let mut r = Reamake::new(&args);
    let p = r.parse_reamake_file(&args.reamake_file_path)?;
    generate_structure(&p.opt_target, &p.hierarchy, &p)?;

    Ok(())
}

fn get_current_date(fmt: &str) -> String {
    use chrono::prelude::*;
    let local: DateTime<Local> = Local::now();
    local.format(fmt).to_string()
}

#[derive(Debug)]
pub struct ProjectBlock {
    opt_target: PathBuf,
    variables: HashMap<String, VariableDecl>,
    settings: Settings,
    hierarchy: Vec<HierarchyNode>,
}

impl ProjectBlock {
    fn new() -> io::Result<Self> {
        Ok(Self {
            opt_target: env::current_dir()?,
            variables: HashMap::new(),
            settings: Settings {
                format_names: false,
                format_date: "%d-%m-%Y".to_string(),
            },
            hierarchy: Vec::new(),
        })
    }
}

#[allow(dead_code)]
struct Reamake<'a> {
    args: &'a Arguments,
    project_blocks: Vec<ProjectBlock>,
}

impl<'a> Reamake<'a> {
    fn new(args: &'a Arguments) -> Self {
        Self {
            args,
            project_blocks: Vec::new(),
        }
    }

    pub fn parse_reamake_file(&mut self, batch_file_path: &String) -> io::Result<ProjectBlock> {
        let contents = fs::read_to_string(batch_file_path)?;

        let tokens = lex(&contents)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e.message))?;

        let mut parser = Parser::new(tokens);
        let ast = parser
            .parse_template()
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e.message))?;

        let mut block = ProjectBlock::new()?;
        block.settings = ast.settings.clone();
        block.hierarchy = ast.hierarchy.clone();

        for var in ast.variables {
            block.variables.insert(var.name.clone(), var);
        }

        for (name, new_value) in &self.args.overrides {
            if let Some(v) = block.variables.get_mut(name) {
                v.value = new_value.clone();
            } else {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidInput,
                    format!(
                        "An override was provided for unknown variable '{}'",
                        name
                    ),
                ));
            }
        }

        Ok(block)
    }
}

// *brakoll - d: generation of folder structure is not getting far down in the ast, p: 90, t: fix, s: closed

fn materialize_node(base: &Path, node: &HierarchyNode, b: &ProjectBlock) -> io::Result<()> {
    match node {
        HierarchyNode::Folder { name, children } => {
            let dir_name = interpolate(name, b);
            let dir = base.join(dir_name);
            fs::create_dir_all(&dir)?;

            for child in children {
                materialize_node(&dir, child, b)?;
            }
        }

        HierarchyNode::File { name } => {
            let file_name = interpolate(name, b);
            let path = base.join(file_name);

            if let Some(parent) = path.parent() {
                fs::create_dir_all(parent)?;
            }

            fs::File::create(path)?;
        }

        HierarchyNode::VariableUse {
            var_name,
            alias,
            children,
        } => {
            materialize_variable_use(base, var_name, alias.as_deref(), children, b)?;
        }
    }

    Ok(())
}

fn materialize_variable_use(
    base: &Path,
    var_name: &str,
    alias: Option<&str>,
    children: &[HierarchyNode],
    b: &ProjectBlock,
) -> io::Result<()> {
    let decl = b.variables.get(var_name).ok_or_else(|| {
        io::Error::new(
            io::ErrorKind::InvalidInput,
            format!("Unknown variable used in hierarchy: {var_name}"),
        )
    })?;

    match decl.ty {
        VarType::String => {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                format!(
                "String variable '{var_name}' cannot be used as a hierarchy node!"
            ),
            ));
        }

        VarType::File => {
            let source_path = interpolate(&decl.value, b);

            let output_name = alias.map(|s| interpolate(s, b)).unwrap_or_else(|| {
                Path::new(&source_path)
                    .file_name()
                    .unwrap_or_default()
                    .to_string_lossy()
                    .to_string()
            });

            let path = base.join(output_name);

            if let Some(parent) = path.parent() {
                fs::create_dir_all(parent)?;
            }

            if source_path.trim().is_empty() || !Path::new(&source_path).exists() {
                fs::File::create(path)?;
            } else {
                fs::copy(&source_path, path)?;
            }
        }

        VarType::Folder => {
            let source_path = interpolate(&decl.value, b);

            let dir_name = alias.map(|s| interpolate(s, b)).unwrap_or_else(|| {
                Path::new(&source_path)
                    .file_name()
                    .unwrap_or_default()
                    .to_string_lossy()
                    .to_string()
            });

            let dir = base.join(dir_name);
            fs::create_dir_all(&dir)?;

            if !source_path.trim().is_empty() {
                let src = Path::new(&source_path);
                if src.exists() {
                    copy_dir_contents(src, &dir)?;
                }
            }

            for child in children {
                materialize_node(&dir, child, b)?;
            }
        }
    }

    Ok(())
}

fn copy_dir_contents(src: &Path, dst: &Path) -> io::Result<()> {
    if !src.exists() {
        return Ok(());
    }

    if !src.is_dir() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            format!("Source is not a directory: {}", src.display()),
        ));
    }

    fs::create_dir_all(dst)?;

    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let entry_path = entry.path();
        let target_path = dst.join(entry.file_name());

        if entry_path.is_dir() {
            copy_dir_contents(&entry_path, &target_path)?;
        } else {
            if let Some(parent) = target_path.parent() {
                fs::create_dir_all(parent)?;
            }
            fs::copy(&entry_path, &target_path)?;
        }
    }

    Ok(())
}

fn interpolate(input: &str, b: &ProjectBlock) -> String {
    let mut out = input.to_string();

    for (name, decl) in &b.variables {
        if decl.ty == VarType::String {
            out = out.replace(&format!("${name}"), decl.value.trim());
        }
    }

    let date = get_current_date(&b.settings.format_date);
    out = out.replace("$date", &date);

    if b.settings.format_names {
        out = out.replace(' ', "-").to_lowercase();
    }

    out
}

pub fn generate_structure(
    root: impl AsRef<Path>,
    nodes: &[HierarchyNode],
    b: &ProjectBlock,
) -> io::Result<()> {
    let root = root.as_ref();
    fs::create_dir_all(root)?;

    for node in nodes {
        materialize_node(root, node, b)?;
    }

    Ok(())
}

