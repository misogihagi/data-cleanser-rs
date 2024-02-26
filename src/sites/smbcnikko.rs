use crate::utils::{use_write, Flow, FlowA};
use clap::Parser;
use futures::future::join_all;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value_t = false)]
    smbcnikko: bool,
}

pub async fn smbcnikko(output: &str) {
    let flow = FlowA {
        index: "https://www.smbcnikko.co.jp/terms/index.html",
        base: "https://www.smbcnikko.co.jp",
        link_link_selector: ".A > li > a, .B > li > a, #anchor02 > li > a",
        link_selector: ".box-release-inner > ul > li > a",
        title_selector: "#main > section > section:nth-child(2) > h2 > span",
        body_selector: "#main > section:nth-child(1) > section:nth-child(2) > div:nth-child(2) > p:nth-child(1)",
        encoding:"shift-jis",
        ..Default::default()
    };

    let terms = flow.get_terms().await;

    use_write(output.to_string())(&terms);
}
