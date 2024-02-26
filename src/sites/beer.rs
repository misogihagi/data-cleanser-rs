use crate::utils::{use_write, Flow, FlowA};
use clap::Parser;
use futures::future::join_all;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value_t = false)]
    beer: bool,
}

pub async fn beer(output: &str) {
    let flow = FlowA {
        index: "https://craft-beer.life/dictionary",
        link_selector: "li.dictionary_section > ol > li > a",
        title_selector: ".article_headline",
        body_selector: ".article_body > p",
        ..Default::default()
    };

    let terms = flow.get_terms().await;

    use_write(output.to_string())(&terms);
}
