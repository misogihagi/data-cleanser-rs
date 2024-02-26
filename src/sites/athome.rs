use crate::utils::{use_write, Flow, FlowA};
use clap::Parser;
use futures::future::join_all;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value_t = false)]
    goonet: bool,
}

pub async fn athome(output: &str) {
    let flow = FlowA {
        index: "https://www.athome.co.jp/contents/words/",
        base: "https://www.athome.co.jp",
        link_link_selector: ".f_l  li a,.f_r li a,.wbox:nth-child(2) li a",
        link_selector: "ul.cf > li > a",
        title_selector: "#h1_title > h1",
        body_selector: ".textarea",
        ..Default::default()
    };

    let terms = flow.get_terms().await;

    use_write(output.to_string())(&terms);
}