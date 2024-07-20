use clap::Parser;
#[derive(Parser, Debug, Clone)]
pub struct Config {
    #[clap(default_value = "main._x")]
    entry: String,
}

mod tokenizer;
use tokenizer::*;
