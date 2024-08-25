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

pub async fn token() -> HashMap<&'static str, Vec<Term>> {
    let mut genre = HashMap::new();

    genre.insert(
        "kenchiku",
        (
            "https://www.token.co.jp/estate/useful/archipedia/row_kana.php?jid=00016",
            ".textbox",
        ),
    );
    genre.insert(
        "syougyoukenchiku",
        (
            "https://www.token.co.jp/estate/useful/archipedia/row_kana.php?jid=00023",
            ".textbox",
        ),
    );
    genre.insert(
        "sumai",
        (
            "https://www.token.co.jp/estate/useful/archipedia/row_kana.php?jid=00022",
            "section.terminology:nth-child(1) > div",
        ),
    );
    genre.insert(
        "omoshirokenchiku",
        (
            "https://www.token.co.jp/estate/useful/archipedia/row_kana.php?jid=00021",
            ".textbox",
        ),
    );
    genre.insert(
        "interior",
        (
            "https://www.token.co.jp/estate/useful/archipedia/row_kana.php?jid=00018",
            ".textbox",
        ),
    );
    genre.insert(
        "takuken",
        (
            "https://www.token.co.jp/estate/useful/archipedia/row_kana.php?jid=00013",
            ".textbox",
        ),
    );
    genre.insert(
        "hudousan",
        (
            "https://www.token.co.jp/estate/useful/archipedia/row_kana.php?jid=00017",
            ".textbox",
        ),
    );
    genre.insert(
        "zeikin",
        (
            "https://www.token.co.jp/estate/useful/archipedia/row_kana.php?jid=00019",
            ".textbox",
        ),
    );

    let mut books = HashMap::new();

    for (g, (url, selector)) in genre {
        books.insert(
            g,
            FlowA {
                link_links: vec![String::from(url)],
                link_base: "https://www.token.co.jp/estate/useful/archipedia/",
                link_selector: ".contents > section > section > ul > li > a",
                title_selector: ".terminology > h1",
                body_selector: selector,
                ..Default::default()
            }
            .get_terms()
            .await,
        );
    }
    books
}
