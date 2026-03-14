// *brakoll - d: initial setup, p: 0, t: feature, s: closed

use std::{fs, io};

mod subcommands;
mod utils;

fn main() -> io::Result<()> {
    // === get args ===
    let args = utils::args::parse()?;

    if args.help {
        subcommands::help::print();
        return Ok(());
    }

    let mut r = Reamake::new();

    // parse batch file
    let path = "../test/batch.conf";
    r.batch_parser(path.to_string())?;

    Ok(())
}

#[derive(Debug)]
struct ProjectBlock {
    client_name: String,
    proj_name: String,
    rpp_template_file_contents: String,
    folder_structure_file_contents: String,
    dest_dir_path: String,
}
impl ProjectBlock {
    fn new() -> Self {
        Self {
            client_name: String::new(),
            proj_name: String::new(),
            rpp_template_file_contents: String::new(),
            folder_structure_file_contents: String::new(),
            dest_dir_path: String::new(),
        }
    }
}

struct Reamake {
    project_blocks: Vec<ProjectBlock>,
}

impl Reamake {
    fn new() -> Self {
        Self {
            project_blocks: Vec::new(),
        }
    }

    // *brakoll - d: custom batch file parser, p: 100, t: feature, s: closed
    /// derive structs <ProjectBlock> from batch file path
    fn batch_parser(&mut self, batch_file_path: String) -> io::Result<()> {
        let contents = fs::read_to_string(batch_file_path)?;

        let mut lines = contents.lines().peekable();

        /// ignore comments and empty lines
        macro_rules! ign {
            ($line:expr) => {
                if $line.trim().is_empty() {
                continue;
                }
                if $line.trim().starts_with("#") {
                continue;
                }
            };
        }

        while let Some(line) = lines.next() {
            ign!(line);
            if line.starts_with('[') {
                let mut b: ProjectBlock = ProjectBlock::new();
                while let Some(next) = lines.peek() {
                    if next.starts_with(']') {
                        break;
                    }
                    let next = lines.next().unwrap();
                    ign!(next);
                    // handle key:val fields

                    let (k, v) = next.split_once(':').expect(format!(
                        "Error parsing batch file on line {}: expected 'key: value'",
                        line!()
                    )
                        .as_str());
                    match k.trim() {
                        "client" => {
                            b.client_name = v.to_string();
                        }
                        "project" => {
                            b.proj_name = v.to_string();
                        }
                        "template" => {
                            b.rpp_template_file_contents =
                                v.to_string();
                        }
                        "structure" => {
                            b.folder_structure_file_contents =
                                v.to_string();
                        }
                        "dest_dir" => {
                            b.dest_dir_path = v.to_string();
                        }
                        _ => {
                            continue;
                        }
                    };

                    // process section line
                }

                // check if all fields in the array have been found
                let fields = [
                    &b.client_name,
                    &b.proj_name,
                    &b.rpp_template_file_contents,
                    &b.folder_structure_file_contents,
                    &b.dest_dir_path,
                ];

                let check: bool = {
                    for field in fields {
                        if field.is_empty() {
                            eprintln!(
                            "Error parsing batch file: field(s) in a project array is missing its key or value."
                        )
                        }
                    }
                    true
                };

                if check {
                    self.project_blocks.push(b);
                }
            }
        }

        Ok(())
    }
}