use crate::utils::{use_write, Flow, FlowA};
use clap::Parser;
use futures::future::join_all;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value_t = false)]
    hrpro: bool,
}

pub async fn hrpro(output: &str) {
    let flow = FlowA {
        index: "https://www.hrpro.co.jp/glossary.php",
        base: "https://www.hrpro.co.jp/",
        link_selector: ".rlt-list > li > a",
        title_selector: "h1.ttl",
        body_selector: ".article-body",
        link_links: vec![
            "https://www.hrpro.co.jp/glossary.php?index=a",
            "https://www.hrpro.co.jp/glossary.php?index=k",
            "https://www.hrpro.co.jp/glossary.php?index=s",
            "https://www.hrpro.co.jp/glossary.php?index=t",
            "https://www.hrpro.co.jp/glossary.php?index=n",
            "https://www.hrpro.co.jp/glossary.php?index=h",
            "https://www.hrpro.co.jp/glossary.php?index=m",
            "https://www.hrpro.co.jp/glossary.php?index=y",
            "https://www.hrpro.co.jp/glossary.php?index=r",
            "https://www.hrpro.co.jp/glossary.php?index=w",
        ]
        .into_iter()
        .fold(vec![], |mut total, url| {
            total.append(&mut vec![url.to_string(), url.to_string() + "&pcnt=2"]);
            total
        }),
        ..Default::default()
    };

    let terms = flow.get_terms().await;

    use_write(output.to_string())(&terms);
}
