use crate::utils::{Flow, FlowA, Term};

use std::collections::HashMap;

pub async fn mitsue() -> Vec<Vec<Term>> {
    let chars = ['i', 'm', 'c', 'y', 's', 'p', 'l', 'b'];

    let mut books = vec![];

    for c in chars {
        let index = format!("https://www.mitsue.co.jp/case/glossary/{}_index.html", c);
        books.push(
            FlowA {
                link_links: vec![String::from(index)],
                base: "https://www.mitsue.co.jp/case/glossary/",
                link_selector: "li.bullet__item > a",
                title_selector: ".level1__heading",
                body_selector: ".text,.bullet,.description,.number,.number__item,.level3",
                ..Default::default()
            }
            .get_terms()
            .await,
        )
    }
    books
}

pub async fn elite_network() -> HashMap<&'static str, Vec<Term>> {
    let business = [
        "it",
        "web",
        "maker",
        "medical",
        "kinyu",
        "consul",
        "kanri",
        "qualification",
        "bizwords",
        "keiri",
        "jinji",
    ];
    let mut books = HashMap::new();

    for c in business {
        let index = format!("https://www.elite-network.co.jp/dictionary/words_{}/", c);
        books.insert(
            c,
            FlowA {
                link_links: vec![String::from(index)],
                link_selector: "div.word_idx_list > a",
                title_selector: ".midasi",
                body_selector: ".contentsleft > div:nth-child(4)",
                ..Default::default()
            }
            .get_terms()
            .await,
        );
    }
    books
}
