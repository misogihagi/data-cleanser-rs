use crate::utils::{use_write, Flow, FlowA};
use clap::Parser;
use futures::future::join_all;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value_t = false)]
    nomura: bool,
}

pub async fn nomura(output: &str) {
    let flow = FlowA {
        index: "https://www.nomura.co.jp/terms/",
        base: "https://www.nomura.co.jp",
        link_link_selector: ".tbl > tbody:nth-child(1) > tr > td > p > a",
        link_selector: ".-transform > li > a",
        title_selector: "#term_id",
        body_selector: ".glossary-block",
        ..Default::default()
    };

    let terms = flow.get_terms().await;

    use_write(output.to_string())(&terms);
}
