use std::{
    fmt::Display,
    io::{BufRead, BufReader},
};

#[derive(Debug)]
pub struct LexerError {
    line: usize,
    col: Option<usize>,
    reason: String,
}

impl Display for LexerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Error at {}:{}\n\t{}",
            self.line,
            self.col.map(|n| n.to_string()).unwrap_or("?".to_string()),
            self.reason
        )
    }
}

impl std::error::Error for LexerError {}

pub fn lexical_analysis<R: std::io::Read>(read: R) -> Result<Vec<Tokens>, LexerError> {
    let buf_reader = BufReader::new(read);
    let tokens = buf_reader
        .lines()
        .enumerate()
        .map(|(line_number, line)| {
            let line = match line {
                Ok(l) => l,
                Err(e) => {
                    return Err(LexerError {
                        line: line_number,
                        col: None,
                        reason: e.to_string(),
                    })
                }
            };

            match lex_lines(line_number, line) {
                LexLineResult::Ok(tok) => Ok(tok),
                LexLineResult::Err {
                    line_number,
                    col,
                    reason,
                } => {
                    return Err(LexerError {
                        line: line_number,
                        col: Some(col),
                        reason,
                    })
                }
            }
        })
        .collect::<Result<Vec<Vec<Tokens>>, LexerError>>()?
        .concat();

    Ok(tokens)
}

pub enum LexLineResult {
    Ok(Vec<Tokens>),
    Err {
        line_number: usize,
        col: usize,
        reason: String,
    },
}

pub fn lex_lines(line_number: usize, line: String) -> LexLineResult {
    let mut char_buf = String::new();
    let mut tokens = Vec::<Tokens>::new();
    let mut char_iter = line.chars().enumerate().peekable();

    while let Some((column, character)) = char_iter.next() {
        let peeked_char = char_iter.peek();
        match character {
            // look for other types of whitespace
            ' ' | '\t' => {
                if !char_buf.is_empty() {
                    let loc = column - char_buf.len();
                    let token = match identify_token(char_buf.to_owned()) {
                        IdentifyTokenResult::Tok(tok) => tok,
                        IdentifyTokenResult::ParseErr(e) => {
                            return LexLineResult::Err {
                                col: loc,
                                line_number,
                                reason: e.to_string(),
                            }
                        }
                    };
                    tokens.push(Tokens {
                        line: line_number,
                        loc,
                        token,
                    });
                    char_buf = String::new();
                }
            }
            '=' => {
                if !char_buf.is_empty() {
                    let loc = column - char_buf.len();
                    let token = match identify_token(char_buf.to_owned()) {
                        IdentifyTokenResult::Tok(tok) => tok,
                        IdentifyTokenResult::ParseErr(e) => {
                            return LexLineResult::Err {
                                col: loc,
                                line_number,
                                reason: e.to_string(),
                            }
                        }
                    };
                    tokens.push(Tokens {
                        line: line_number,
                        loc,
                        token,
                    });
                    char_buf = String::new();
                }
                tokens.push(Tokens {
                    line: line_number,
                    loc: column,
                    token: Token::Eq,
                });
            }
            ';' => {
                if !char_buf.is_empty() {
                    let loc = column - char_buf.len();
                    let token = match identify_token(char_buf.to_owned()) {
                        IdentifyTokenResult::Tok(tok) => tok,
                        IdentifyTokenResult::ParseErr(e) => {
                            return LexLineResult::Err {
                                col: loc,
                                line_number,
                                reason: e.to_string(),
                            }
                        }
                    };
                    tokens.push(Tokens {
                        line: line_number,
                        loc,
                        token,
                    });
                    char_buf = String::new();
                }
                tokens.push(Tokens {
                    line: line_number,
                    loc: column,
                    token: Token::Semi,
                });
            }
            ':' => {
                if !char_buf.is_empty() {
                    let loc = column - char_buf.len();
                    let token = match identify_token(char_buf.to_owned()) {
                        IdentifyTokenResult::Tok(tok) => tok,
                        IdentifyTokenResult::ParseErr(e) => {
                            return LexLineResult::Err {
                                col: loc,
                                line_number,
                                reason: e.to_string(),
                            }
                        }
                    };
                    tokens.push(Tokens {
                        line: line_number,
                        loc,
                        token,
                    });
                    char_buf = String::new();
                }
                tokens.push(Tokens {
                    line: line_number,
                    loc: column,
                    token: Token::Colon,
                });
            }
            '+' => {
                if !char_buf.is_empty() {
                    let loc = column - char_buf.len();
                    let token = match identify_token(char_buf.to_owned()) {
                        IdentifyTokenResult::Tok(tok) => tok,
                        IdentifyTokenResult::ParseErr(e) => {
                            return LexLineResult::Err {
                                col: loc,
                                line_number,
                                reason: e.to_string(),
                            }
                        }
                    };
                    tokens.push(Tokens {
                        line: line_number,
                        loc,
                        token,
                    });
                    char_buf = String::new();
                }
                tokens.push(Tokens {
                    line: line_number,
                    loc: column,
                    token: Token::Plus,
                });
            }
            '(' => {
                if !char_buf.is_empty() {
                    let loc = column - char_buf.len();
                    let token = match identify_token(char_buf.to_owned()) {
                        IdentifyTokenResult::Tok(tok) => tok,
                        IdentifyTokenResult::ParseErr(e) => {
                            return LexLineResult::Err {
                                col: loc,
                                line_number,
                                reason: e.to_string(),
                            }
                        }
                    };
                    tokens.push(Tokens {
                        line: line_number,
                        loc,
                        token,
                    });
                    char_buf = String::new();
                }
                tokens.push(Tokens {
                    line: line_number,
                    loc: column,
                    token: Token::LParen,
                });
            }
            ')' => {
                if !char_buf.is_empty() {
                    let loc = column - char_buf.len();
                    let token = match identify_token(char_buf.to_owned()) {
                        IdentifyTokenResult::Tok(tok) => tok,
                        IdentifyTokenResult::ParseErr(e) => {
                            return LexLineResult::Err {
                                col: loc,
                                line_number,
                                reason: e.to_string(),
                            }
                        }
                    };
                    tokens.push(Tokens {
                        line: line_number,
                        loc,
                        token,
                    });
                    char_buf = String::new();
                }
                tokens.push(Tokens {
                    line: line_number,
                    loc: column,
                    token: Token::RParen,
                });
            }
            '{' => {
                if !char_buf.is_empty() {
                    let loc = column - char_buf.len();
                    let token = match identify_token(char_buf.to_owned()) {
                        IdentifyTokenResult::Tok(tok) => tok,
                        IdentifyTokenResult::ParseErr(e) => {
                            return LexLineResult::Err {
                                col: loc,
                                line_number,
                                reason: e.to_string(),
                            }
                        }
                    };
                    tokens.push(Tokens {
                        line: line_number,
                        loc,
                        token,
                    });
                    char_buf = String::new();
                }
                tokens.push(Tokens {
                    line: line_number,
                    loc: column,
                    token: Token::LBrace,
                });
            }
            '}' => {
                if !char_buf.is_empty() {
                    let loc = column - char_buf.len();
                    let token = match identify_token(char_buf.to_owned()) {
                        IdentifyTokenResult::Tok(tok) => tok,
                        IdentifyTokenResult::ParseErr(e) => {
                            return LexLineResult::Err {
                                col: loc,
                                line_number,
                                reason: e.to_string(),
                            }
                        }
                    };
                    tokens.push(Tokens {
                        line: line_number,
                        loc,
                        token,
                    });
                    char_buf = String::new();
                }
                tokens.push(Tokens {
                    line: line_number,
                    loc: column,
                    token: Token::RBrace,
                });
            }
            _ => {
                char_buf.push(character);
            }
        }
    }
    if !char_buf.is_empty() {
        let loc = line.len() - char_buf.len();
        let token = match identify_token(char_buf.to_owned()) {
            IdentifyTokenResult::Tok(tok) => tok,
            IdentifyTokenResult::ParseErr(e) => {
                return LexLineResult::Err {
                    col: loc,
                    line_number,
                    reason: e.to_string(),
                }
            }
        };

        tokens.push(Tokens {
            line: line_number,
            loc,
            token,
        });
    }
    return LexLineResult::Ok(tokens);
}

enum IdentifyTokenResult {
    Tok(Token),
    ParseErr(String),
}

fn identify_token(lit: String) -> IdentifyTokenResult {
    IdentifyTokenResult::Tok(match lit.as_str() {
        "const" => Token::Const,
        "mut" => Token::Mut,
        "return" => Token::Return,
        "fn" => Token::Fn,
        "->" => Token::Arrow,
        lit => {
            // ident or literal
            let first = lit.chars().nth(0).unwrap();

            match first {
                '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' | '0' | '-' => {
                    let num: i32 = match first.to_string().parse::<i32>() {
                        Ok(n) => n,
                        Err(e) => return IdentifyTokenResult::ParseErr(e.to_string()),
                    };
                    Token::IntLit(num)
                }
                _ => Token::Ident(lit.to_string()),
            }
        }
    })
}

#[derive(Debug, Clone)]
pub struct Tokens {
    pub token: Token,
    line: usize,
    loc: usize,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Token {
    Const,
    Mut,
    Ident(String),
    Fn,
    Arrow,
    Eq,
    Semi,
    Colon,
    LParen,
    RParen,
    IntLit(i32),
    // float lit
    // char lit
    // string lit
    // html lit
    // json lit
    LBrace,
    RBrace,
    Plus,
    Minus,
    Mult,
    FSlash,
    Return,
}
