use crate::utils::{use_write, Flow, FlowA};
use clap::Parser;
use futures::future::join_all;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value_t = false)]
    cybernet: bool,
}

pub async fn cybernet(output: &str) {
    let flow = FlowA {
        index: "https://www.cybernet.co.jp/optical/glossary/",
        base: "https://www.cybernet.co.jp",
        link_selector: ".contents_body > section > div > ul > li > a",
        title_selector: ".page_ttl > h1",
        body_selector: ".text_blc",
        ..Default::default()
    };

    let terms = flow.get_terms().await;

    use_write(output.to_string())(&terms);
}
