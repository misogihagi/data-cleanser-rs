use crate::utils::{use_write, Flow, FlowA};
use clap::Parser;
use std::fs;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value_t = false)]
    mitsue: bool,
}

pub async fn mitsue(output: &str) {
    let chars = ['i', 'm', 'c', 'y', 's', 'p', 'l', 'b'];

    fs::create_dir(output).unwrap();

    for (i, c) in chars.iter().enumerate() {
        let link_link = format!(
            "ul.c-list:nth-child(2) > li:nth-child({}) > span > a",
            i + 1
        );
        let flow = FlowA {
            index: "https://www.mitsue.co.jp/case/glossary/",
            base: "https://www.mitsue.co.jp/case/glossary/",
            link_link_selector: &link_link,
            link_selector: "li.c-list__item > span > a",
            title_selector: ".c-headingLv1__text",
            body_selector: ".o-layout__contentColumn",
            ..Default::default()
        };
        let terms = flow.get_terms().await;
        use_write(output.to_string() + "/" + &c.to_string() + ".json")(&terms);
    }
}
