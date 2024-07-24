use clap::Parser;
use compiler_1::codegen::c_codegen;
use compiler_1::parser::parse_toplevel;
use std::error::Error;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use std::process::Command;
use std::process::ExitCode;

use compiler_1::lexer::*;
use compiler_1::CodeFile;
use compiler_1::Config;

fn main() -> Result<ExitCode, Box<dyn Error>> {
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
