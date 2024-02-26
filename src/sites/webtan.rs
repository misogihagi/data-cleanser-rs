use crate::utils::{use_write, Flow, FlowA};
use clap::Parser;
use futures::future::join_all;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value_t = false)]
    webtan: bool,
}

pub async fn webtan(output: &str) {
    let flow = FlowA {
        index: "https://webtan.impress.co.jp/glossary/list/1",
        base: "https://webtan.impress.co.jp",
        link_selector: ".node > div:nth-child(1) > ul > li > a",
        title_selector: "h1.title",
        body_selector: ".glossary_description",
        ..Default::default()
    };

    let terms = flow.get_terms().await;

    use_write(output.to_string())(&terms);
}
