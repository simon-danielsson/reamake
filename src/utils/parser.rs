use std::{fs, io};

use crate::{DateFormat, ProjectBlock, Reamake};

impl Reamake {
    // *brakoll - d: have the file structure be part of the batch file! the batch parser can call the structure parser from within itself and create a hierarchy struct directly in each projectblock struct (in place of a string path) , p: 90, t: feature, s: closed
    // *brakoll - d: file structure parser (perhaps yaml or something else), p: 100, t: feature, s: prog
    /// helper: parse_reamake_file
    pub fn parse_hierarchy(&mut self, batch_file_path: &String) -> io::Result<()> {
        let contents = fs::read_to_string(batch_file_path)?;
        Ok(())
    }

    // *brakoll - d: custom batch file parser, p: 100, t: feature, s: closed
    pub fn parse_reamake_file(&mut self, batch_file_path: String) -> io::Result<ProjectBlock> {
        let mut b: ProjectBlock = ProjectBlock::new();
        let contents = fs::read_to_string(&batch_file_path)?;

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
            if line.starts_with("[variables]") {
                while let Some(next) = lines.peek() {
                    if next.starts_with("[") {
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
                            b.var_client = v.to_string();
                        }
                        "project" => {
                            b.var_project = v.to_string();
                        }
                        "service" => {
                            b.var_service = v.to_string();
                        }
                        _ => {
                            continue;
                        }
                    };
                }
            }
            if line.starts_with("[sources]") {
                while let Some(next) = lines.peek() {
                    if next.starts_with("[") {
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
                        "rpp" => {
                            b.src_rpp = v.to_string();
                        }
                        _ => {
                            continue;
                        }
                    };
                }
            }
            if line.starts_with("[sources]") {
                while let Some(next) = lines.peek() {
                    if next.starts_with("[") {
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
                        "rpp" => {
                            b.src_rpp = v.to_string();
                        }
                        _ => {
                            continue;
                        }
                    };
                }
            }
            if line.starts_with("[settings]") {
                while let Some(next) = lines.peek() {
                    if next.starts_with("[") {
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
                        "format_names" => {
                            if v.to_string().contains("tr") {
                                b.set_kebab = true;
                            } else {
                                b.set_kebab = false;
                            }
                        }
                        "format_date" => {
                            if v.to_string().contains("US") {
                                b.set_date = DateFormat::US;
                            } else if v.to_string().contains("ISO") {
                                b.set_date = DateFormat::ISO;
                            } else {
                                b.set_date = DateFormat::EU;
                            }
                        }
                        _ => {
                            continue;
                        }
                    };
                }
            }
        }

        self.parse_hierarchy(&batch_file_path);

        Ok(b)
    }
}