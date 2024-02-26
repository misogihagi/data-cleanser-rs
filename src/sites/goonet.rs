use crate::utils::{use_write, Flow, FlowA};
use clap::Parser;
use futures::future::join_all;
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value_t = false)]
    goonet: bool,
}

pub async fn goonet(output: &str) {
    let flow = FlowA {
        index: "https://www.goo-net.com/knowledge/",
        base: "https://www.goo-net.com",
        link_link_selector: "#main > section > div > dl > dd:nth-child(2) > ul > li > a",
        link_selector: ".column2 > li > a",
        title_selector: ".h3box > h5",
        body_selector: ".text",
        encoding: "euc-jp",
    };

    let terms = flow.get_terms().await;

    use_write(output.to_string())(&terms);
}
/*
.post_btn > h1:nth-child(1)
.post_box
*/
