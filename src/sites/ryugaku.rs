use crate::utils::{use_write, Flow, FlowA};
use clap::Parser;
use futures::future::join_all;
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value_t = false)]
    ryugaku: bool,
}

pub async fn ryugaku(output: &str) {
    let flow = FlowA {
        index: "https://ryugaku.kuraveil.jp/dictionaries",
        base: "https://ryugaku.kuraveil.jp",
        link_link_selector: "div.initial-index:nth-child(4) > a",
        link_selector: ".word-list > li > a",
        title_selector: ".header-title",
        body_selector: ".markdown",
        ..Default::default()
    };

    let terms = flow.get_terms().await;

    use_write(output.to_string())(&terms);
}
