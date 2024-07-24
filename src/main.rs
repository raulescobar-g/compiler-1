use clap::Parser;
use color_eyre::Result;
use compiler_1::codegen::c_codegen;
use compiler_1::parser::parse_toplevel;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use std::process::Command;
use std::process::ExitCode;

use compiler_1::lexer::*;
use compiler_1::CodeFile;
use compiler_1::Config;

fn main() -> Result<ExitCode> {
    let config = Config::parse();

    let file = CodeFile::new(&config.entry)?;

    let tokens = lexical_analysis(file)?;
    println!("{:?}", tokens.clone());

    let ast = parse_toplevel(tokens)?;
    println!("{:?}", ast);

    let c_code = c_codegen(ast)?;
    println!("\n\n{}", c_code);

    let out_path = PathBuf::from(config.outfile);
    let mut outfile = File::create(out_path)?;

    let _ = outfile.write_all(c_code.as_bytes())?;

    Command::new("gcc")
        .args(["./bin/main.c", "-o", "./bin/main"])
        .output()?;

    Command::new("./bin/main").output()?;

    return Ok(ExitCode::SUCCESS);
}

// const : Tokens
//
//
// let lexed_tokens =   lexical_analysis( Read )?;
// let ast =            parse_tokens( IntoIterator<lexed_tokens> )?;
// let c_code =         transpile_to_c( Codegen )?;
