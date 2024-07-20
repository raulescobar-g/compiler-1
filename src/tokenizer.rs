use color_eyre::{eyre::Context, Result};
use std::{
    fs::File,
    io::{self, BufRead, BufReader, Read},
    path::PathBuf,
    process::ExitCode,
    str::FromStr,
};

struct CodeFile {
    path_buf: PathBuf,
    file: File,
}

impl CodeFile {
    pub fn new(path: &str) -> Result<Self> {
        let path_buf =
            PathBuf::from_str(path).wrap_err(format!("Error creating pathbuf to file {path}"))?;
        let file = File::open(path).wrap_err(format!("Error opening source code file {path}"))?;

        Ok(Self { path_buf, file })
    }

    pub fn tokenize_file(self) -> Result<CodeFileTokens> {
        let buf = BufReader::new(self.file);
        let file_path = self.path_buf.to_string_lossy();
        let tokens = buf
            .lines()
            .enumerate()
            .map(|(line_number, line)| {
                let line = line.wrap_err(format!(
                    "Error reading line {line_number} in file {file_path}"
                ))?;
                Ok((line_number, line))
            })
            .collect::<Result<Vec<(usize, String)>>>()?
            .into_iter()
            .fold(
                Ok(Vec::<Tokens>::new()),
                |token_bag: Result<Vec<Tokens>>, (line_number, line_content)| {
                    Ok(token_bag?
                        .into_iter()
                        .chain(tokenize_line(line_number, line_content)?)
                        .collect::<Vec<Tokens>>())
                },
            )?;
        Ok(CodeFileTokens { tokens })
    }
}

struct LineTokenizer {
    line_number: usize,
    char_buf: Vec<char>,
    token: Vec<Tokens>,
    line: String,
}

fn tokenize_line(line_number: usize, line: String) -> Result<Vec<Tokens>> {
    let mut char_iter = line.chars().enumerate().peekable();
    let mut char_buff = Vec::<char>::new();
    let mut token_buf = Vec::<Tokens>::new();
    while let Some((column, character)) = char_iter.next() {
        let peeked = char_iter.peek();
        match character {
             => continue, 
            _ => unimplemented!()
        }
    }
    todo!()
}

struct CodeFileTokens {
    tokens: Vec<Tokens>,
}

#[derive(Debug)]
struct Tokens {
    token: Token,
    line: usize,
    loc: usize,
}

#[derive(Debug, PartialEq, Eq)]
enum Token {
    TInt32,
    TFloat32,
    LParen,
    RParen,
    Semi,
    RArrow,
    Int(i32),
    Float(i32, u32),
    LBrack,
    RBrack,
    Plus,
    Minus,
    Mult,
    FSlash,
    Ident(String),
    Invalid(String),
    Return,
    Fn,
    Main,
}

fn is_ident(ident: &str) -> bool {
    let valid_nums = "0123456789";
    let valid_chars_upper = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
    let valid_chars_lower = "_abcdefghijklmnopqrstuvwxyz";

    let valid_chars = valid_nums
        .chars()
        .chain(valid_chars_upper.chars())
        .chain(valid_chars_lower.chars())
        .collect::<Vec<char>>();

    return ident.is_ascii()
        && valid_chars_lower.contains(ident.chars().nth(0).unwrap())
        && ident.to_string().chars().all(|c| valid_chars.contains(&c));
}

fn is_int(num: &str) -> bool {
    let numbers = "0123456789";
    let num_string = num.to_string();
    let mut char_iter = num_string.chars();
    let char = char_iter.next().unwrap();
    match char {
        '0' => return false,
        '-' => {}
        n if numbers.contains(n) => {}
        _ => return false,
    }

    return char_iter.all(|c| numbers.contains(c));
}
fn is_float(num: &str) -> bool {
    let numbers = "0123456789";
    let num_string = num.to_string();
    return num_string.chars().all(|c| numbers.contains(c));
}

impl Into<Token> for String {
    fn into(self) -> Token {
        match self.as_str() {
            "+" => Token::Plus,
            "-" => Token::Minus,
            "*" => Token::Mult,
            r#"/"# => Token::FSlash,
            "{" => Token::LBrack,
            "}" => Token::RBrack,
            ";" => Token::Semi,
            "->" => Token::RArrow,
            "(" => Token::LParen,
            ")" => Token::RParen,
            "int" => Token::TInt32,
            "float" => Token::TFloat32,
            "fn" => Token::Fn,
            "main" => Token::Main,
            "return" => Token::Return,
            num if is_int(num) => Token::Int(num.parse().unwrap()),
            num if is_float(num) => todo!(), //Token::Float(num.parse().unwrap()),
            ident if is_ident(ident) => Token::Ident(ident.to_string()),
            invalid => Token::Invalid(invalid.to_string()),
        }
    }
}

#[derive(Debug)]
struct Ast {
    tokens: Vec<Tokens>,
    buff: String,
    line: usize,
    loc: usize,
}

impl Ast {
    fn new() -> Self {
        Self {
            tokens: vec![],
            buff: "".to_string(),
            line: 0,
            loc: 0,
        }
    }
    fn codegen(self) -> String {
        let mut token_iter = self.tokens.into_iter().peekable();

        while let Some(token) = token_iter.next() {
            match token.token {
                Token::Fn if token_iter.peek().is_some_and(|t| t.token == Token::Main) => {}
                _ => {}
            }
        }

        todo!()
    }
}

fn main() -> Result<ExitCode> {
    let config = Config::parse();

    let entry_file = File::open(config.entry).wrap_err("Error openning file")?;
    let source = io::read_to_string(entry_file).wrap_err("Reading source file")?;

    let mut ast = Ast::new();
    let mut char_iter = dbg!(&source).chars().peekable();

    let number_chars = "1234567890";

    while let Some(c) = char_iter.next() {
        match c {
            '-' if Some('>') == char_iter.peek().copied()
                || char_iter
                    .peek()
                    .copied()
                    .is_some_and(|ch| number_chars.contains(ch)) =>
            {
                ast.buff += &c.to_string();
                ast.loc += 1;
            }
            '+' | '*' | '/' | '{' | '}' | ';' | '-' | '(' | ')' => {
                if !ast.buff.is_empty() {
                    let token: Token = ast.buff.clone().into();
                    ast.tokens.push(Tokens {
                        token,
                        line: ast.line,
                        loc: ast.loc - ast.buff.len(),
                    });
                    ast.buff = String::new();
                }
                let token: Token = c.to_string().into();
                ast.tokens.push(Tokens {
                    token,
                    line: ast.line,
                    loc: ast.loc,
                });
                ast.loc += 1;
            }
            ' ' => {
                if !ast.buff.is_empty() {
                    let token: Token = ast.buff.clone().into();
                    ast.tokens.push(Tokens {
                        token,
                        line: ast.line,
                        loc: ast.loc - ast.buff.len(),
                    });
                    ast.buff = String::new();
                }
                ast.loc += 1;
            }
            '\t' => {
                if !ast.buff.is_empty() {
                    let token: Token = ast.buff.clone().into();
                    ast.tokens.push(Tokens {
                        token,
                        line: ast.line,
                        loc: ast.loc - ast.buff.len(),
                    });
                    ast.buff = String::new();
                }
                ast.loc += 2;
            }
            '\n' | '\r' => {
                if !ast.buff.is_empty() {
                    let token: Token = ast.buff.clone().into();
                    ast.tokens.push(Tokens {
                        token,
                        line: ast.line,
                        loc: ast.loc - ast.buff.len(),
                    });
                    ast.buff = String::new();
                }
                ast.line += 1;
                ast.loc = 0;
            }
            c => {
                ast.buff += &c.to_string();
                ast.loc += 1;
            }
        }
    }
    dbg!(&ast);

    let codegen = ast.codegen();

    return Ok(ExitCode::SUCCESS);
}