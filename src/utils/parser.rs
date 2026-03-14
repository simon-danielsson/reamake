use std::{fs, io};

use crate::{DateFormat, ProjectBlock, Reamake};

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

#[derive(Debug, Clone, PartialEq, Eq)]
enum Token {
    Folder,
    File,
    Rpp,
    LBrace,
    RBrace,
    Name(String),
}

// *brakoll - d: rpp is treated as a dir and should instead be treated as a file, p: 100, t: fix, s: closed
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub enum Node {
    Folder { name: String, children: Vec<Node> },
    File { name: String },
    Rpp { name: String },
}

impl Node {
    fn rename_all<F>(&mut self, f: &F)
where
        F: Fn(&str) -> String,
    {
        match self {
            Node::Folder { name, children } => {
                *name = f(name);
                for child in children {
                    child.rename_all(f);
                }
            }
            Node::File { name } => {
                *name = f(name);
            }
            Node::Rpp { name } => {
                *name = f(name);
            }
        }
    }
}

struct HierParser {
    tokens: Vec<Token>,
    pos: usize,
}

impl HierParser {
    fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, pos: 0 }
    }

    fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.pos)
    }

    fn next(&mut self) -> Option<Token> {
        let tok = self.tokens.get(self.pos).cloned();
        if tok.is_some() {
            self.pos += 1;
        }
        tok
    }

    fn expect(&mut self, expected: Token) -> Result<(), String> {
        let got = self
            .next()
            .ok_or_else(|| format!("expected {:?}, got EOF", expected))?;
        if got == expected {
            Ok(())
        } else {
            Err(format!("expected {:?}, got {:?}", expected, got))
        }
    }

    fn expect_name(&mut self) -> Result<String, String> {
        match self.next() {
            Some(Token::Name(name)) => Ok(name),
            other => Err(format!("expected name, got {:?}", other)),
        }
    }

    fn parse_all(&mut self) -> Result<Vec<Node>, String> {
        let mut nodes = Vec::new();
        while self.peek().is_some() {
            nodes.push(self.parse_node()?);
        }
        Ok(nodes)
    }

    fn parse_node(&mut self) -> Result<Node, String> {
        match self.peek() {
            Some(Token::Folder) => self.parse_folder(),
            Some(Token::File) => self.parse_file(),
            Some(Token::Rpp) => self.parse_rpp(),
            other => Err(format!("expected node, got {:?}", other)),
        }
    }

    fn parse_folder(&mut self) -> Result<Node, String> {
        self.expect(Token::Folder)?;
        let name = self.expect_name()?;
        self.expect(Token::LBrace)?;

        let mut children = Vec::new();
        while self.peek() != Some(&Token::RBrace) {
            children.push(self.parse_node()?);
        }

        self.expect(Token::RBrace)?;

        Ok(Node::Folder { name, children })
    }

    fn parse_file(&mut self) -> Result<Node, String> {
        self.expect(Token::File)?;
        let name = self.expect_name()?;
        Ok(Node::File { name })
    }

    fn parse_rpp(&mut self) -> Result<Node, String> {
        self.expect(Token::Rpp)?;
        let name = self.expect_name()?;
        Ok(Node::Rpp { name })
    }
}

impl Reamake {
    // *brakoll - d: have the file structure be part of the batch file! the batch parser can call the structure parser from within itself and create a hierarchy struct directly in each projectblock struct (in place of a string path) , p: 90, t: feature, s: closed
    // *brakoll - d: custom folder structure hierarchy parser, p: 100, t: feature, s: closed

    /// helper: parse_reamake_file
    pub fn parse_hierarchy(
        &mut self,
        b: &ProjectBlock,
        batch_file_path: &String,
    ) -> io::Result<Vec<Node>> {
        // derive raw source hierarchy from batch file
        let contents = fs::read_to_string(batch_file_path)?;
        let (_, src_r) = contents.rsplit_once("[hierarchy]").unwrap();

        let tokens = self.hierarchy_lexer(&src_r).unwrap();
        let mut ast = HierParser::new(tokens).parse_all().unwrap();

        for n in ast.iter_mut() {
            n.rename_all(&|name| rename_node(name, b));
        }

        Ok(ast)
    }

    fn hierarchy_lexer(&mut self, input: &str) -> Result<Vec<Token>, String> {
        let mut tokens = Vec::new();
        let mut chars = input.chars().peekable();

        while let Some(&ch) = chars.peek() {
            match ch {
                c if c.is_whitespace() => {
                    chars.next();
                }

                '#' => {
                    // skip comment until end of line
                    while let Some(c) = chars.next() {
                        if c == '\n' {
                            break;
                        }
                    }
                }

                '{' => {
                    chars.next();
                    tokens.push(Token::LBrace);
                }

                '}' => {
                    chars.next();
                    tokens.push(Token::RBrace);
                }

                '"' => {
                    chars.next(); // consume opening quote
                    let mut s = String::new();

                    while let Some(c) = chars.next() {
                        if c == '"' {
                            break;
                        }
                        s.push(c);
                    }

                    tokens.push(Token::Name(s));
                }

                c if c.is_ascii_alphabetic() => {
                    let mut word = String::new();
                    while let Some(&c) = chars.peek() {
                        if c.is_ascii_alphanumeric() || c == '_' {
                            word.push(c);
                            chars.next();
                        } else {
                            break;
                        }
                    }

                    match word.as_str() {
                        "folder" => tokens.push(Token::Folder),
                        "file" => tokens.push(Token::File),
                        "rpp" => tokens.push(Token::Rpp),
                        _ => {
                            return Err(format!(
                            "unexpected identifier: {word}"
                        ));
                        }
                    }
                }

                _ => {
                    return Err(format!("unexpected character: {ch}"));
                }
            }
        }

        Ok(tokens)
    }

    // *brakoll - d: add logic so that override flags for variable fields are respected by the parser, p: 30, t: refactor, s: open
    // *brakoll - d: custom batch file parser, p: 100, t: feature, s: closed
    pub fn parse_reamake_file(&mut self, batch_file_path: String) -> io::Result<ProjectBlock> {
        let mut b: ProjectBlock = ProjectBlock::new()?;
        let contents = fs::read_to_string(&batch_file_path)?;

        let mut lines = contents.lines().peekable();

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

        b.hierarchy = self.parse_hierarchy(&b, &batch_file_path)?;

        Ok(b)
    }
}

fn get_current_date(f: &DateFormat) -> String {
    use chrono::prelude::*;
    let local: DateTime<Local> = Local::now();

    match f {
        DateFormat::US => {
            return local.format("%m-%d-%Y").to_string();
        }
        DateFormat::EU => {
            return local.format("%d-%m-%Y").to_string();
        }
        DateFormat::ISO => {
            return local.format("%Y-%m-%d").to_string();
        }
    }
}

fn format_name_to_kebab(name: String) -> String {
    name.replace(" ", "-").to_lowercase()
}

/// helper: parse_hierarchy
pub fn rename_node(name: &str, b: &ProjectBlock) -> String {
    let date = get_current_date(&b.set_date);
    let new_name = name
        .replace("$client", &b.var_client.trim())
        .replace("$project", &b.var_project.trim())
        .replace("$service", &b.var_service.trim())
        .replace("$date", &date);
    if b.set_kebab {
        return format_name_to_kebab(new_name);
    } else {
        return new_name;
    }
}