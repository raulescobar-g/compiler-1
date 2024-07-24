use crate::lexer::{Token, Tokens};
use std::fmt::{Debug, Display};

#[derive(Debug)]
pub struct ParseError;
impl std::error::Error for ParseError {}
impl Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ast error")
    }
}

#[derive(Debug)]
pub struct Ast {
    pub functions: Vec<Functions>,
}

impl Ast {
    fn empty() -> Self {
        Self { functions: vec![] }
    }
}

pub fn parse_toplevel(tokens: impl IntoIterator<Item = Tokens>) -> Result<Ast, ParseError> {
    let mut token_iter = tokens.into_iter();

    let mut ast = Ast::empty();
    let mut token_buf = Vec::<Tokens>::new();
    let mut brace_counter = 0;
    while let Some(token) = token_iter.next() {
        if brace_counter < 0 {
            return Err(ParseError);
        }
        match token.token {
            Token::Fn => {
                if token_buf.len() > 0 {
                    return Err(ParseError);
                }
                token_buf.push(token);
            }
            Token::LBrace => {
                brace_counter += 1;
                token_buf.push(token);
            }
            Token::RBrace => {
                brace_counter -= 1;
                token_buf.push(token);
                if brace_counter == 0 {
                    ast.functions.push(parse_function(token_buf.drain(..))?);
                }
            }
            _ => token_buf.push(token),
        }
    }

    Ok(ast)
}

#[derive(Debug)]
pub struct Scope {
    pub stmts: Vec<Statement>,
    pub ret: Option<ReturnStmts>,
}

fn parse_scope(tokens: impl IntoIterator<Item = Tokens>) -> Result<Scope, ParseError> {
    let mut token_iter = tokens.into_iter().peekable();
    let mut brace_counter = 0;
    let mut buf = Vec::<Tokens>::new();
    let mut stmts = Vec::<Statement>::new();

    while let Some(token) = token_iter.next() {
        if brace_counter < 0 {
            return Err(ParseError);
        }

        match token.token {
            Token::LBrace => {
                brace_counter += 1;
                if brace_counter > 1 {
                    buf.push(token);
                }
            }
            Token::RBrace => {
                brace_counter -= 1;
                if brace_counter == 0 {
                    break;
                }
                buf.push(token);
            }
            Token::Semi => {
                if !buf.is_empty() {
                    stmts.push(parse_statement(buf.drain(..))?);
                }
            }
            _ => {
                buf.push(token);
            }
        };
    }

    if token_iter.peek().is_some() {
        return Err(ParseError);
    }

    let last_stmt = stmts.pop();
    let ret = match last_stmt.ok_or(ParseError)? {
        Statement::Return(ret) => Some(ret),
        other => {
            stmts.push(other);
            None
        }
    };

    return Ok(Scope { stmts, ret });
}

#[derive(Debug)]
pub struct Functions {
    pub fn_signature: FnSignature,
    pub scope: Scope,
}

fn parse_function(tokens: impl IntoIterator<Item = Tokens>) -> Result<Functions, ParseError> {
    let mut buf = Vec::<Tokens>::new();
    let mut token_iter = tokens.into_iter().peekable();

    while let Some(token) = token_iter.next_if(|t| t.token != Token::LBrace) {
        buf.push(token);
    }

    let fn_signature = parse_fn_signature(buf.drain(..))?;

    let scope = parse_scope(token_iter)?;

    if !buf.is_empty() {
        dbg!(&buf);
        return Err(ParseError);
    }

    Ok(Functions {
        fn_signature,
        scope,
    })
}

#[derive(Debug)]
pub struct FnSignature {
    pub name: String,
    //args
    pub return_type: String,
}
fn parse_fn_signature(tokens: impl IntoIterator<Item = Tokens>) -> Result<FnSignature, ParseError> {
    let mut token_iter = tokens.into_iter();

    let fn_token = token_iter.next().ok_or(ParseError)?;

    if fn_token.token != Token::Fn {
        return Err(ParseError);
    }

    let fn_name = token_iter.next().ok_or(ParseError)?;

    let fn_name = match fn_name.token {
        Token::Ident(name) => name,
        _ => return Err(ParseError),
    };

    let open_p_token = token_iter.next().ok_or(ParseError)?;

    if open_p_token.token != Token::LParen {
        return Err(ParseError);
    }

    // here read args

    let close_p_token = token_iter.next().ok_or(ParseError)?;

    if close_p_token.token != Token::RParen {
        return Err(ParseError);
    }

    let arrow_token = token_iter.next().ok_or(ParseError)?;

    if arrow_token.token != Token::Arrow {
        return Err(ParseError);
    }

    let ret_type = token_iter.next().ok_or(ParseError)?;

    let ret_type = match ret_type.token {
        Token::Ident(ident) => ident,
        _ => return Err(ParseError),
    };

    Ok(FnSignature {
        name: fn_name,
        return_type: ret_type,
    })
}

#[derive(Debug)]
pub struct ReturnStmts {
    pub ret: Expression,
}

fn parse_return(tokens: impl IntoIterator<Item = Tokens>) -> Result<ReturnStmts, ParseError> {
    let mut token_iter = tokens.into_iter();

    let first_token = token_iter.next().ok_or(ParseError)?;

    if first_token.token != Token::Return {
        return Err(ParseError);
    }

    Ok(ReturnStmts {
        ret: parse_expression(token_iter)?,
    })
}

#[derive(Debug)]
pub enum Statement {
    MutDeclaration(DeclarationStmts),
    ConstDeclaration(DeclarationStmts),
    Return(ReturnStmts),
    Assignment(AssignmentStmts),
    Expression(Expression),
}

fn parse_statement(tokens: impl IntoIterator<Item = Tokens>) -> Result<Statement, ParseError> {
    let mut token_iter = tokens.into_iter().peekable(); //used
    let first_token = token_iter.peek().ok_or(ParseError)?.token.clone();
    Ok(match &first_token {
        Token::Return => Statement::Return(parse_return(token_iter)?),
        Token::Mut => Statement::MutDeclaration(parse_mutable_declaration(token_iter)?),
        Token::Const => Statement::MutDeclaration(parse_const_declaration(token_iter)?),
        other => Statement::Assignment(parse_assignment(token_iter)?),
    })
}

#[derive(Debug)]
pub struct AssignmentStmts {
    pub lhs: Tokens,
    pub rhs: Expression,
}
fn parse_assignment(
    tokens: impl IntoIterator<Item = Tokens>,
) -> Result<AssignmentStmts, ParseError> {
    let mut token_iter = tokens.into_iter();

    let first_token = token_iter.next().ok_or(ParseError)?;

    let eq_token = token_iter.next().ok_or(ParseError)?;
    if eq_token.token != Token::Eq {
        return Err(ParseError);
    }

    let expr = parse_expression(token_iter)?;

    Ok(AssignmentStmts {
        lhs: first_token,
        rhs: expr,
    })
}

#[derive(Debug)]
pub struct DeclarationStmts {
    pub lhs: Declarations,
    pub rhs: Expression,
}
fn parse_mutable_declaration(
    tokens: impl IntoIterator<Item = Tokens>,
) -> Result<DeclarationStmts, ParseError> {
    let token_vec = tokens.into_iter().collect::<Vec<_>>();

    let (lhs, rhs) = token_vec
        .split_once(|tok| tok.token == Token::Eq)
        .ok_or(ParseError)?;

    Ok(DeclarationStmts {
        lhs: parse_declaration(lhs.to_vec())?,
        rhs: parse_expression(rhs.to_vec())?,
    })
}

fn parse_const_declaration(
    tokens: impl IntoIterator<Item = Tokens>,
) -> Result<DeclarationStmts, ParseError> {
    let token_vec = tokens.into_iter().collect::<Vec<_>>();

    let (lhs, rhs) = token_vec
        .split_once(|tok| tok.token == Token::Eq)
        .ok_or(ParseError)?;

    Ok(DeclarationStmts {
        lhs: parse_declaration(lhs.to_vec())?,
        rhs: parse_expression(rhs.to_vec())?,
    })
}

#[derive(Debug)]
pub enum Expression {
    Addition(Box<Expression>, Box<Expression>),
    Value(Tokens),
    FnCall(String, FnArgs),
}

#[derive(Debug)]
pub struct FnArgs {
    args: Vec<Expression>,
}

fn parse_expression(tokens: impl IntoIterator<Item = Tokens>) -> Result<Expression, ParseError> {
    let token_iter = tokens.into_iter().peekable(); //.collect::<Vec<Tokens>>();

    let first_tok = token_iter.next().ok_or(ParseError)?;

    let second_tok = match token_iter.next() {
        Some(tok) => tok,
        None => return Ok(Expression::Value(first_tok)),
    };

    /*if token_vec.len() == 1 {
        return Ok(Expression::Value(token_vec[0].clone()));
    } else {
        let (left, right) = token_vec
            .split_once(|t| t.token == Token::Plus)
            .ok_or(ParseError)?;

        return Ok(Expression::Addition(
            Box::new(parse_expression(left.to_vec())?),
            Box::new(parse_expression(right.to_vec())?),
        ));
    }*/
}

#[derive(Debug)]
pub struct Declarations {
    pub ident: String,
    pub data_type: String,
}
fn parse_declaration(tokens: impl IntoIterator<Item = Tokens>) -> Result<Declarations, ParseError> {
    let mut token_iter = tokens.into_iter();

    let mutability = token_iter.next().ok_or(ParseError)?;

    if mutability.token != Token::Const && mutability.token != Token::Mut {
        return Err(ParseError);
    }

    let ident = token_iter.next().ok_or(ParseError)?;
    let ident = match ident.token {
        Token::Ident(ident) => ident,
        _ => return Err(ParseError),
    };

    let colon = token_iter.next().ok_or(ParseError)?;
    if colon.token != Token::Colon {
        return Err(ParseError);
    }

    let data_type = token_iter.next().ok_or(ParseError)?;
    let data_type = match data_type.token {
        Token::Ident(data_type) => data_type,
        _ => return Err(ParseError),
    };

    return Ok(Declarations { ident, data_type });
}
