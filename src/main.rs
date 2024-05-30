mod args;
mod sites;
mod utils;
use crate::utils::use_write;
use args::Args;
use clap::Parser;
use sites::special::{elite_network, mitsue};
use std::fs;

macro_rules! cmd {
    ($e:expr) => {
        let terms = sites::lib::run($e).await;
        use_write(format!("{}.json", $e).to_string())(&terms);
    };
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    for mode in args.common() {
        cmd!(mode);
    }
    if args.mitsue {
        fs::create_dir("mitsue").unwrap();
        let books = mitsue();
        for (i, terms) in books.await.iter().enumerate() {
            use_write(format!("mitsue/{}.json", i).to_string())(&terms);
        }
    }
    if args.elitenetwork {
        fs::create_dir("elite-network").unwrap();
        let books = elite_network();
        for (key, terms) in books.await {
            use_write(format!("elite-network/{}.json", key).to_string())(&terms);
        }
    }
}
