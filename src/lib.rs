#![feature(slice_split_once)]

use clap::Parser;
use color_eyre::*;
use eyre::Context;
use std::{fs::File, io::Read, path::PathBuf};

#[derive(Parser, Debug, Clone)]
pub struct Config {
    #[clap(default_value = "main._x")]
    pub entry: String,

    #[clap(default_value = "./bin/main.c")]
    pub outfile: String,
}

pub mod codegen;
pub mod lexer;
pub mod parser;

#[derive(Debug)]
pub struct CodeFile<R: Read> {
    path_buf: PathBuf,
    file: R,
}
impl<R: Read> Read for CodeFile<R> {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        self.file.read(buf)
    }
}

impl CodeFile<File> {
    pub fn new(path: impl ToString) -> color_eyre::Result<Self> {
        let path_buf = PathBuf::from(path.to_string());
        let file = File::open(path.to_string())
            .wrap_err(format!("Error opening file {}", path.to_string()))?;

        Ok(Self { path_buf, file })
    }
}
