use crate::utils::{use_write, Flow, FlowA};
use clap::Parser;
use futures::future::join_all;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value_t = false)]
    goonet: bool,
}

pub async fn macromill(output: &str) {
    let flow = FlowA {
        index: "https://www.macromill.com/service/words/",
        link_link_selector: ".indexPc > ul > li > a",
        link_selector: ".main > .posts > .pnl > a",
        title_selector: ".head > div > h1",
        body_selector: ".un_secBlock",
        ..Default::default()
    };

    let terms = flow.get_terms().await;

    use_write(output.to_string())(&terms);
}
