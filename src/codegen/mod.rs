use crate::{
    lexer::Token,
    parser::{
        AssignmentStmts, Ast, DeclarationStmts, Declarations, Expression, FnSignature, Functions,
        ReturnStmts, Scope, Statement,
    },
};
use std::fmt::Display;

#[derive(Debug)]
pub struct CodegenError;
impl std::error::Error for CodegenError {}
impl Display for CodegenError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "codegen error")
    }
}

pub fn c_codegen(ast: Ast) -> Result<String, CodegenError> {
    let codegen = ast
        .functions
        .into_iter()
        .map(|func| codegen_function(func))
        .fold(Ok(String::new()), |sum_code, fn_string| {
            Ok(sum_code? + &fn_string?.to_string())
        })?;

    let result = format!("#include <stdbool.h>\n{codegen}");

    Ok(result)
}

fn codegen_function(func: Functions) -> Result<String, CodegenError> {
    let fn_signature = codegen_function_signature(func.fn_signature)?;
    let scope = codegen_scope(func.scope)?;

    let buf = format!("{fn_signature}{scope}");

    Ok(buf)
}

fn codegen_function_signature(fn_signature: FnSignature) -> Result<String, CodegenError> {
    let ret_type = to_native_type(&fn_signature.return_type).ok_or(CodegenError)?;

    Ok(format!("{} {}()", ret_type, fn_signature.name))
}

fn codegen_scope(func: Scope) -> Result<String, CodegenError> {
    let start = format!("{{\n");
    let end = format!("}}\n");
    let statements = func
        .stmts
        .into_iter()
        .map(|stmt| codegen_statement(stmt))
        .fold(Ok(String::new()), |sum_code, fn_string| {
            Ok(sum_code? + &fn_string?.to_string())
        })?;

    let return_statement = func.ret.map_or(Ok("".to_string()), |return_stmt| {
        codegen_return_statement(return_stmt)
    })?;

    Ok(format!("{start}{statements}{return_statement}{end}"))
}

fn codegen_statement(statement: Statement) -> Result<String, CodegenError> {
    match statement {
        Statement::Return(return_stmt) => codegen_return_statement(return_stmt),
        Statement::Assignment(assignment_stmt) => codegen_assignment(assignment_stmt),
        Statement::MutDeclaration(mut_decl_stmt) => codegen_mut_declaration(mut_decl_stmt),
        Statement::ConstDeclaration(const_decl_stmt) => codegen_const_declaration(const_decl_stmt),
    }
}

fn codegen_return_statement(statement: ReturnStmts) -> Result<String, CodegenError> {
    let expr = codegen_expression(statement.ret)?;
    Ok(format!("\treturn {};\n", expr))
}

fn codegen_assignment(assignment_stmts: AssignmentStmts) -> Result<String, CodegenError> {
    let ident = match assignment_stmts.lhs.token {
        Token::Ident(ident) => ident,
        _ => return Err(CodegenError),
    };

    let expr = codegen_expression(assignment_stmts.rhs)?;

    Ok(format!("\t{ident} = {expr};\n"))
}

fn codegen_mut_declaration(decl_stmts: DeclarationStmts) -> Result<String, CodegenError> {
    let declaration = codegen_declaration(decl_stmts.lhs)?;

    let expr = codegen_expression(decl_stmts.rhs)?;

    Ok(format!("\t{declaration} = {expr};\n"))
}

fn codegen_const_declaration(decl_stmts: DeclarationStmts) -> Result<String, CodegenError> {
    let declaration = codegen_declaration(decl_stmts.lhs)?;
    let expr = codegen_expression(decl_stmts.rhs)?;

    Ok(format!("\t{declaration} = {expr};\n"))
}

fn codegen_declaration(decl: Declarations) -> Result<String, CodegenError> {
    let data_type = to_native_type(&decl.data_type).ok_or(CodegenError)?;

    Ok(format!("{} {}", data_type, decl.ident))
}

fn codegen_expression(expr: Expression) -> Result<String, CodegenError> {
    Ok(match expr {
        Expression::Value(tok) => match tok.token {
            Token::Ident(name) => name,
            Token::IntLit(num) => num.to_string(),
            _ => return Err(CodegenError),
        },
        Expression::Addition(l_expr, r_expr) => {
            let l_expr = codegen_expression(*l_expr)?;
            let r_expr = codegen_expression(*r_expr)?;
            format!("{l_expr} + {r_expr}")
        }
    })
}

fn to_native_type(t: &str) -> Option<&str> {
    Some(match t {
        "i32" => "int",
        "bool" => "bool",
        "char" => "char",
        _ => return None,
    })
}
