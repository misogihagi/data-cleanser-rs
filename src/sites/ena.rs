use crate::utils::{use_write, Flow, FlowA};
use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value_t = false)]
    ena: bool,
}

pub async fn ena(output: &str) {
    let flow = FlowA {
        index: "https://www.ena.travel/glossary/all/",
        base: "https://www.ena.travel",
        link_selector: "div.body > dl > dd:nth-child(2) > ul:nth-child(1) > li > a",
        title_selector: "div.guide > h2 > span > span",
        body_selector: "div.text > p",
        ..Default::default()
    };

    let terms = flow.get_terms().await;
    use_write(output.to_string())(&terms);
}
