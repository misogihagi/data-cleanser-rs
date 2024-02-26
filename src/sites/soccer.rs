use crate::utils::{use_write, Flow, FlowA};
use clap::Parser;
use futures::future::join_all;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value_t = false)]
    soccer: bool,
}

pub async fn soccer(output: &str) {
    let flow = FlowA {
        index: "https://www.homemate-research-soccer.com/useful/glossary/soccer/",
        base: "https://www.homemate-research-soccer.com",
        link_selector: "section.sec_cmn:nth-child(4) > div > ul > li > a",
        title_selector: ".post_btn > h1:nth-child(1)",
        body_selector: ".post_box",
        image_selector: Some(".post_box_img > img:nth-child(1)"),
        ..Default::default()
    };

    let terms = flow.get_terms().await;

    use_write(output.to_string())(&terms);
}
