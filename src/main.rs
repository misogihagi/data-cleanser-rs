mod args;
mod sites;
mod utils;
use crate::utils::use_write;
use args::Args;
use clap::Parser;
use sites::mitsue;
use std::fs;

macro_rules! cmd {
    ($e:expr) => {
        let terms = sites::run($e).await;
        use_write(format!("{}.json", $e).to_string())(&terms);
    };
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    for mode in args.simples() {
        cmd!(mode);
    }
    if args.mitsue {
        fs::create_dir("mitsue").unwrap();
        let books = mitsue();
        for (i, terms) in books.await.iter().enumerate() {
            use_write(format!("mitsue/{}.json", i).to_string())(&terms);
        }
    }
}
