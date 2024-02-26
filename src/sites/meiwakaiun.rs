use crate::utils::{use_write, Flow, FlowB};
use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value_t = false)]
    meiwakaiun: bool,
}

pub async fn meiwakaiun(output: &str) {
    let flow = FlowB {
        index: "https://www.meiwakaiun.com/meiwaplus/glossary",
        base: "https://www.meiwakaiun.com",
        link_selector: "td:nth-child(5n+2) > a,ul.flex-ct-r > li:first-child > a ",
        titles_selector: "section.grossary-details-box > table > tbody > tr:not(#si01-2) > th",
        bodies_selector: "section.grossary-details-box > table > tbody > tr:not(#si01-2) > td",
        ..Default::default()
    };

    let terms = flow.get_terms().await;
    use_write(output.to_string())(&terms);
}
