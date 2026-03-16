use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VarType {
    String,
    File,
    Folder,
}

#[derive(Debug, Clone)]
pub struct VariableDecl {
    pub name: String,
    pub ty: VarType,
    pub value: String,
}

#[derive(Debug, Clone)]
pub struct Settings {
    pub format_names: bool,
    pub format_date: String,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            format_names: false,
            format_date: "%d-%m-%Y".to_string(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct TemplateAst {
    pub variables: Vec<VariableDecl>,
    pub settings: Settings,
    pub hierarchy: Vec<HierarchyNode>,
}

#[derive(Debug, Clone)]
pub enum HierarchyNode {
    Folder {
        name: String,
        children: Vec<HierarchyNode>,
    },
    File {
        name: String,
    },
    VariableUse {
        var_name: String,
        alias: Option<String>,
        children: Vec<HierarchyNode>,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Ident(String),
    StringLit(String),

    LBracket, // [
    RBracket, // ]
    LBrace,   // {
    RBrace,   // }
    Colon,    // :
    Equal,    // =

    True,
    False,

    Eof,
}

#[derive(Debug, Clone)]
pub struct ParseError {
    pub message: String,
    pub position: usize,
}

impl ParseError {
    fn new(message: impl Into<String>, position: usize) -> Self {
        Self {
            message: message.into(),
            position,
        }
    }
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "parse error at token {}: {}",
            self.position, self.message
        )
    }
}

impl std::error::Error for ParseError {}

pub fn lex(input: &str) -> Result<Vec<Token>, ParseError> {
    let mut tokens = Vec::new();
    let mut chars = input.chars().peekable();

    while let Some(&ch) = chars.peek() {
        match ch {
            c if c.is_whitespace() => {
                chars.next();
            }

            '#' => {
                // skip comment to end of line
                while let Some(c) = chars.next() {
                    if c == '\n' {
                        break;
                    }
                }
            }

            '[' => {
                chars.next();
                tokens.push(Token::LBracket);
            }
            ']' => {
                chars.next();
                tokens.push(Token::RBracket);
            }
            '{' => {
                chars.next();
                tokens.push(Token::LBrace);
            }
            '}' => {
                chars.next();
                tokens.push(Token::RBrace);
            }
            ':' => {
                chars.next();
                tokens.push(Token::Colon);
            }
            '=' => {
                chars.next();
                tokens.push(Token::Equal);
            }

            '"' => {
                chars.next(); // consume opening quote
                let mut s = String::new();
                let mut terminated = false;

                while let Some(c) = chars.next() {
                    if c == '"' {
                        terminated = true;
                        break;
                    }
                    s.push(c);
                }

                if !terminated {
                    return Err(ParseError::new(
                        "unterminated string literal",
                        tokens.len(),
                    ));
                }

                tokens.push(Token::StringLit(s));
            }

            c if c.is_ascii_alphabetic() || c == '_' => {
                let mut word = String::new();

                while let Some(&c2) = chars.peek() {
                    if c2.is_ascii_alphanumeric() || c2 == '_' {
                        word.push(c2);
                        chars.next();
                    } else {
                        break;
                    }
                }

                match word.as_str() {
                    "true" => tokens.push(Token::True),
                    "false" => tokens.push(Token::False),
                    _ => tokens.push(Token::Ident(word)),
                }
            }

            _ => {
                return Err(ParseError::new(
                    format!("unexpected character '{}'", ch),
                    tokens.len(),
                ));
            }
        }
    }

    tokens.push(Token::Eof);
    Ok(tokens)
}

pub struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, pos: 0 }
    }

    pub fn parse_template(&mut self) -> Result<TemplateAst, ParseError> {
        let mut variables = Vec::new();
        let mut settings = Settings::default();
        let mut hierarchy = Vec::new();

        while !self.is_eof() {
            match self.peek_ident().as_deref() {
                Some("variables") => variables = self.parse_variables_section()?,
                Some("settings") => settings = self.parse_settings_section()?,
                Some("hierarchy") => hierarchy = self.parse_hierarchy_section()?,
                Some(other) => {
                    return Err(self
                        .error(format!("unexpected section '{}'", other)));
                }
                None => {
                    return Err(self.error(format!(
                        "expected section identifier, got {:?}",
                        self.peek()
                    )));
                }
            }
        }

        Ok(TemplateAst {
            variables,
            settings,
            hierarchy,
        })
    }

    fn parse_variables_section(&mut self) -> Result<Vec<VariableDecl>, ParseError> {
        self.expect_ident("variables")?;
        self.expect(Token::LBracket)?;

        let mut vars = Vec::new();
        while !self.check(&Token::RBracket) {
            vars.push(self.parse_variable_decl()?);
        }

        self.expect(Token::RBracket)?;
        Ok(vars)
    }

    fn parse_variable_decl(&mut self) -> Result<VariableDecl, ParseError> {
        let name = self.expect_any_ident()?;
        self.expect(Token::Colon)?;
        let ty_name = self.expect_any_ident()?;
        self.expect(Token::Equal)?;
        let value = self.expect_string()?;

        let ty = match ty_name.as_str() {
            "string" => VarType::String,
            "file" => VarType::File,
            "folder" => VarType::Folder,
            _ => return Err(self.error(format!("unknown variable type '{}'", ty_name))),
        };

        Ok(VariableDecl { name, ty, value })
    }

    fn parse_settings_section(&mut self) -> Result<Settings, ParseError> {
        self.expect_ident("settings")?;
        self.expect(Token::LBracket)?;

        let mut settings = Settings::default();

        while !self.check(&Token::RBracket) {
            let key = self.expect_any_ident()?;
            self.expect(Token::Colon)?;

            match key.as_str() {
                "format_names" => {
                    settings.format_names = self.expect_bool()?;
                }
                "format_date" => {
                    settings.format_date = self.expect_string()?;
                }
                _ => return Err(self.error(format!("unknown setting '{}'", key))),
            }
        }

        self.expect(Token::RBracket)?;
        Ok(settings)
    }

    fn parse_hierarchy_section(&mut self) -> Result<Vec<HierarchyNode>, ParseError> {
        self.expect_ident("hierarchy")?;
        self.expect(Token::LBracket)?;

        let mut nodes = Vec::new();
        while !self.check(&Token::RBracket) {
            nodes.push(self.parse_hierarchy_node()?);
        }

        self.expect(Token::RBracket)?;
        Ok(nodes)
    }

    fn parse_hierarchy_node(&mut self) -> Result<HierarchyNode, ParseError> {
        let kind = self.expect_any_ident()?;

        match kind.as_str() {
            "folder" => self.parse_folder_node(),
            "file" => self.parse_file_node(),
            _ => self.parse_variable_use_node(kind),
        }
    }

    fn parse_folder_node(&mut self) -> Result<HierarchyNode, ParseError> {
        let name = self.expect_string()?;
        self.expect(Token::LBrace)?;

        let mut children = Vec::new();
        while !self.check(&Token::RBrace) {
            children.push(self.parse_hierarchy_node()?);
        }

        self.expect(Token::RBrace)?;
        Ok(HierarchyNode::Folder { name, children })
    }

    fn parse_file_node(&mut self) -> Result<HierarchyNode, ParseError> {
        let name = self.expect_string()?;
        Ok(HierarchyNode::File { name })
    }

    fn parse_variable_use_node(
        &mut self,
        var_name: String,
    ) -> Result<HierarchyNode, ParseError> {
        let alias = if matches!(self.peek(), Some(Token::StringLit(_))) {
            Some(self.expect_string()?)
        } else {
            None
        };

        let children = if self.check(&Token::LBrace) {
            self.expect(Token::LBrace)?;
            let mut children = Vec::new();

            while !self.check(&Token::RBrace) {
                children.push(self.parse_hierarchy_node()?);
            }

            self.expect(Token::RBrace)?;
            children
        } else {
            Vec::new()
        };

        Ok(HierarchyNode::VariableUse {
            var_name,
            alias,
            children,
        })
    }

    fn is_eof(&self) -> bool {
        matches!(self.peek(), Some(Token::Eof))
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

    fn check(&self, expected: &Token) -> bool {
        self.peek() == Some(expected)
    }

    fn expect(&mut self, expected: Token) -> Result<(), ParseError> {
        let got = self.next();
        if got == Some(expected.clone()) {
            Ok(())
        } else {
            Err(self.error(format!("expected {:?}, got {:?}", expected, got)))
        }
    }

    fn expect_ident(&mut self, expected: &str) -> Result<(), ParseError> {
        match self.next() {
            Some(Token::Ident(s)) if s == expected => Ok(()),
            other => Err(self.error(format!(
                "expected identifier '{}', got {:?}",
                expected, other
            ))),
        }
    }

    fn expect_any_ident(&mut self) -> Result<String, ParseError> {
        match self.next() {
            Some(Token::Ident(s)) => Ok(s),
            other => Err(self.error(format!("expected identifier, got {:?}", other))),
        }
    }

    fn expect_string(&mut self) -> Result<String, ParseError> {
        match self.next() {
            Some(Token::StringLit(s)) => Ok(s),
            other => Err(
                self.error(format!("expected string literal, got {:?}", other))
            ),
        }
    }

    fn expect_bool(&mut self) -> Result<bool, ParseError> {
        match self.next() {
            Some(Token::True) => Ok(true),
            Some(Token::False) => Ok(false),
            other => Err(self.error(format!("expected boolean, got {:?}", other))),
        }
    }

    fn peek_ident(&self) -> Option<String> {
        match self.peek() {
            Some(Token::Ident(s)) => Some(s.clone()),
            _ => None,
        }
    }

    fn error(&self, message: impl Into<String>) -> ParseError {
        ParseError::new(message, self.pos)
    }
}
