use futures::future::join_all;
use std::time::Duration;

use scraper::{Html, Selector};

use super::interface::WorkFlowTrait;
use crate::utils::{
    get_html, get_links, get_term, get_terms, get_text, Flow, FlowA, LinkQuery, Term,
};

pub enum SiteKindHandmade {
    Ajima,
    Coocan,
    Efjapan,
    Footballbox,
    Hiroshima,
    Jfadocuments,
    MoonLight,
    Nikken,
    Toraiz,
    Yodosha,
}

pub struct HandmadeWorkFlow {
    pub kind: SiteKindHandmade,
}
impl HandmadeWorkFlow {
    pub fn new(kind_str: &'static str) -> HandmadeWorkFlow {
        HandmadeWorkFlow {
            kind: HandmadeWorkFlow::my_kind(kind_str).unwrap(),
        }
    }
    pub fn my_kind(kind_str: &'static str) -> Option<SiteKindHandmade> {
        match kind_str {
            "ajima" => Some(SiteKindHandmade::Ajima),
            "coocan" => Some(SiteKindHandmade::Coocan),
            "efjapan" => Some(SiteKindHandmade::Efjapan),
            "footballbox" => Some(SiteKindHandmade::Footballbox),
            "hiroshima" => Some(SiteKindHandmade::Hiroshima),
            "jfadocuments" => Some(SiteKindHandmade::Jfadocuments),
            "moonlight" => Some(SiteKindHandmade::MoonLight),
            "nikken" => Some(SiteKindHandmade::Nikken),
            "toraiz" => Some(SiteKindHandmade::Toraiz),
            "yodosha" => Some(SiteKindHandmade::Yodosha),
            _ => None,
        }
    }
}

impl WorkFlowTrait for HandmadeWorkFlow {
    fn is_my_kind(kind_str: &'static str) -> bool {
        match HandmadeWorkFlow::my_kind(kind_str) {
            Some(_) => true,
            None => false,
        }
    }
    async fn get_terms(&self) -> Vec<Term> {
        match &self.kind {
            &SiteKindHandmade::Ajima => ajima().await,
            &SiteKindHandmade::Coocan => coocan().await,
            &SiteKindHandmade::Efjapan => efjapan().await,
            &SiteKindHandmade::Footballbox => footballbox().await,
            &SiteKindHandmade::Hiroshima => hiroshima().await,
            &SiteKindHandmade::Jfadocuments => jfadocuments().await,
            &SiteKindHandmade::MoonLight => moonlight().await,
            &SiteKindHandmade::Nikken => nikken().await,
            &SiteKindHandmade::Toraiz => toraiz().await,
            &SiteKindHandmade::Yodosha => yodosha().await,
        }
    }
}

pub async fn ajima() -> Vec<Term> {
    let links = FlowA {
        index: "https://hougen.ajima.jp/gojyuon/",
        base: "https://hougen.ajima.jp",
        level2_selector: "ul.gojyuon > li > a",
        level1_selector: ".list_hougen > li > article > a",
        ..Default::default()
    }
    .get_links()
    .await;

    join_all(links.iter().map(|l| get_html(l, "utf-8")))
        .await
        .into_iter()
        .map(|x| x.unwrap())
        .map(|html| {
            let title_selector = Selector::parse(".midashi").unwrap();
            let fragment = Html::parse_fragment(&html);

            let title: String = get_text(fragment.clone(), title_selector.clone());

            let meaning_str = ".detail_body > ol:nth-child(2)";
            let meaning_selector = Selector::parse(meaning_str).unwrap();
            let meaning = get_text(fragment.clone(), meaning_selector.clone());
            let commentary_str = ".kaisetsu";
            let commentary_selector = Selector::parse(commentary_str).unwrap();
            let commentary = get_text(fragment.clone(), commentary_selector.clone());
            let frequency_str = ".kanren > dd:nth-child(2) > img:nth-child(1)";
            let frequency_selector = Selector::parse(frequency_str).unwrap();
            let frequency_url: String = fragment
                .select(&frequency_selector)
                .map(|e| e.value().attr("src").unwrap().to_string())
                .collect();

            let frequency = match frequency_url.as_str() {
                "./img/hindo1.png" => "1",
                "./img/hindo2.png" => "2",
                "./img/hindo3.png" => "3",
                "./img/hindo4.png" => "4",
                "./img/hindo5.png" => "5",
                _ => "",
            };

            let image_str = ".detail_image > img:nth-child(1)";
            let image_selector = Selector::parse(image_str).unwrap();
            let image: String = fragment
                .select(&image_selector)
                .map(|e| e.value().attr("src").unwrap().to_string())
                .collect();

            Term {
                title: title,
                body: String::new()
                    + "意味：\n"
                    + &meaning
                    + "\n解説: \n"
                    + &commentary
                    + "\n耳にする度: "
                    + frequency,
                images: vec![image],
            }
        })
        .collect()
}

pub async fn coocan() -> Vec<Term> {
    let links: Vec<String> =
        get_links(LinkQuery {
            url: "https://moz.la.coocan.jp/fussball/lexikon/top.html",
            base: "https://moz.la.coocan.jp/fussball/lexikon/",
            selector_string: "div > table > tbody > tr > td > font:not(:nth-child(3)):not(:nth-child(7)) a, div > table > tbody > tr:nth-child(5) > td > a",
            encoding: "utf-8",
        })
    .await.unwrap();

    let s_title = "div > table > tbody > tr > td > dl:nth-of-type(odd) > dt";
    let s_body = "div > table > tbody > tr > td > dl:nth-of-type(odd) > dd";
    let s_title2 =
        "div > table > tbody > tr > td > dl:nth-of-type(even) > dd > font:nth-of-type(2)";
    let s_body2 = "div > table > tbody > tr > td > dl:nth-of-type(even) > dd";
    let encoding = "utf-8";
    join_all(links.iter().map(|link| async move {
        if link.ends_with("b.html") {
            let (res1, res2) = futures::join!(
                get_terms(link.to_string(), s_title, s_body, None, encoding),
                get_term(
                    link.to_string(),
                    "body > div > table > tbody > tr:nth-child(1) > td > dl:nth-child(3) > dd:nth-child(312) > blockquote > dl > dt > font:nth-child(2) > b",
                    "body > div > table > tbody > tr:nth-child(1) > td > dl:nth-child(3) > dd:nth-child(312) > blockquote > dl > dt > font:nth-child(2)",
                    None,
                    encoding
                ),
            );
            match (res1, res2) {
                (Ok(mut r1), Ok(r2)) => {
                    r1.push(r2);
                    Ok(r1)
                }
                (Err(e), _) => Err(e),
                (_, Err(e)) => Err(e),
            }
        } else if link.ends_with("f.html") {
            let (res1, res2) = futures::join!(
                get_terms(link.to_string(), s_title, s_body, None, encoding),
                get_terms(
                    link.to_string(),
                    "body > div > table > tbody > tr:nth-child(1) > td > dl:nth-child(even) > dd:nth-of-type(1)",
                    "body > div > table > tbody > tr:nth-child(1) > td > dl:nth-child(even) > dd:nth-of-type(2)",
                    None,
                    encoding
                ),
            );
            match (res1, res2) {
                (Ok(mut r1), Ok(r2)) => {
                    r1.extend(r2);
                    Ok(r1)
                }
                (Err(e), _) => Err(e),
                (_, Err(e)) => Err(e),
            }
        } else if link.ends_with("k.html") {
            let (res1, res2, res3, res4) = futures::join!(
                get_terms(link.to_string(), s_title, s_body, None, encoding),
                get_term(
                    link.to_string(),
                    "div > table > tbody > tr:nth-child(1) > td > dl:nth-child(6) > dd:nth-child(1)",
                    "div > table > tbody > tr:nth-child(1) > td > dl:nth-child(6) > dd:nth-child(2)",
                    None, encoding
                ),
                get_term(
                    link.to_string(),
                    "div > table > tbody > tr:nth-child(1) > td > dl:nth-child(8) > dd > font:nth-child(2) > b",
                    "div > table > tbody > tr:nth-child(1) > td > dl:nth-child(8) > dd > font:nth-child(2),div > table > tbody > tr:nth-child(1) > td > dl:nth-child(8) > dd > font:nth-child(3)",
                    None, encoding
                ),
                get_term(
                    link.to_string(),
                    "div > table > tbody > tr:nth-child(1) > td > dl:nth-child(4) > dd > font:nth-child(2) > b",
                    "div > table > tbody > tr:nth-child(1) > td > dl:nth-child(4) > dd > font:nth-child(2)", 
                    None, encoding
                ),
            );
            match (res1, res2, res3, res4) {
                (Ok(mut r1), Ok(r2), Ok(r3), Ok(r4)) => {
                    r1.push(r2);
                    r1.push(r3);
                    r1.push(r4);
                    Ok(r1)
                }
                (Err(e), _, _, _) => Err(e),
                (_, Err(e), _, _) => Err(e),
                (_, _, Err(e), _) => Err(e),
                (_, _, _, Err(e)) => Err(e),
            }
        } else if link.ends_with("m.html") {
            let (res1, res2) = futures::join!(
                get_terms(link.to_string(), s_title, s_body, None, encoding),
                get_term(
                    link.to_string(),
                    "div > table > tbody > tr > td > dl:nth-of-type(even) > dd:nth-of-type(1)",
                    "div > table > tbody > tr > td > dl:nth-of-type(even) > dd:nth-of-type(2)",
                    None,
                    encoding
                )
            );
            match (res1, res2) {
                (Ok(mut r1), Ok(r2)) => {
                    r1.push(r2);
                    Ok(r1)
                }
                (Err(e), _) => Err(e),
                (_, Err(e)) => Err(e),
            }
         } else if link.ends_with("s.html") {
            let (res1, res2) = futures::join!(
                get_terms(
                    link.to_string(),
                    "body > div > table > tbody > tr:nth-child(1) > td > dl:nth-child(even) > dt",
                    "body > div > table > tbody > tr:nth-child(1) > td > dl:nth-child(even) > dd",
                    None,
                    encoding
                ),
                get_terms(
                    link.to_string(),
                    "div > table > tbody > tr:nth-child(1) > td > dl:nth-child(odd) > dd:nth-of-type(1)",
                    "div > table > tbody > tr:nth-child(1) > td > dl:nth-child(odd) > dd:nth-of-type(2)",
                    None,
                    encoding
                ),
            );
            match (res1, res2) {
                (Ok(mut r1), Ok(r2)) => {
                    r1.extend(r2);
                    Ok(r1)
                }
                (Err(e), _) => Err(e),
                (_, Err(e)) => Err(e),
            }
        } else if link.ends_with("w.html") {
            let (res1, res2, res3) = futures::join!(
                get_terms(link.to_string(), s_title, s_body, None, encoding),
                get_term(
                    link.to_string(),
                    "body > div > table > tbody > tr:nth-child(1) > td > dl:nth-child(4) > dd:nth-child(1) > font:nth-child(2)",
                    "body > div > table > tbody > tr:nth-child(1) > td > dl:nth-child(4) > dd:nth-child(2),body > div > table > tbody > tr:nth-child(1) > td > dl:nth-child(4) > dd:nth-child(3),body > div > table > tbody > tr:nth-child(1) > td > dl:nth-child(4) > dd:nth-child(4)",
                    None,
                    encoding
                ),
                get_term(
                    link.to_string(),
                    "body > div > table > tbody > tr:nth-child(1) > td > dl:nth-child(6) > dd:nth-child(1) > font:nth-child(2)",
                    "body > div > table > tbody > tr:nth-child(1) > td > dl:nth-child(6) > dd:nth-child(2) > font",
                    None,
                    encoding
                )
            );
            match (res1, res2, res3) {
                (Ok(mut r1), Ok(r2), Ok(r3)) => {
                    r1.push(r2);
                    r1.push(r3);
                    Ok(r1)
                }
                (Err(e), _, _) => Err(e),
                (_, Err(e), _) => Err(e),
                (_, _, Err(e)) => Err(e),
            }
        } else if link.ends_with("y.html") {
            let (res1, res2) = futures::join!(
                get_terms(
                    link.to_string(),
                    "div > table > tbody > tr > td > dl > dt:first-child",
                    s_body,
                    None,
                    encoding
                ),
                get_terms(link.to_string(), s_title2, s_body2, None, encoding)
            );
            match (res1, res2) {
                (Ok(mut r1), Ok(r2)) => {
                    r1.extend(r2);
                    Ok(r1)
                }
                (Err(e), _) => Err(e),
                (_, Err(e)) => Err(e),
            }
        } else if link.ends_with("z.html") {
            let (res1, res2) = futures::join!(
                get_terms(link.to_string(), s_title, s_body, None, encoding),
                get_term(
                    link.to_string(),
                    "body > div > table > tbody > tr:nth-child(1) > td > dl:nth-child(4) > dd:nth-child(1) > font:nth-child(2)", 
                    "body > div > table > tbody > tr:nth-child(1) > td > dl:nth-child(4) > dd:nth-child(2), body > div > table > tbody > tr:nth-child(1) > td > dl:nth-child(4) > dd:nth-child(3)", 
                    None,
                    encoding
                )
            );
            match (res1, res2) {
                (Ok(mut r1), Ok(r2)) => {
                    r1.push(r2);
                    Ok(r1)
                }
                (Err(e), _) => Err(e),
                (_, Err(e)) => Err(e),
            }
        } else {
            let (res1, res2) = futures::join!(
                get_terms(link.to_string(), s_title, s_body, None, encoding),
                get_terms(link.to_string(), s_title2, s_body2, None, encoding)
            );
            match (res1, res2) {
                (Ok(mut r1), Ok(r2)) => {
                    r1.extend(r2);
                    Ok(r1)
                }
                (Err(e), _) => Err(e),
                (_, Err(e)) => Err(e),
            }
        }
    }))
    .await
    .into_iter()
    .flat_map(|x| x.unwrap())
    .collect()
}

pub async fn efjapan() -> Vec<Term> {
    let body = get_html(
        "https://www.efjapan.co.jp/blog/language/useful-football-terms/",
        "utf-8",
    )
    .await
    .unwrap();

    let fragment = Html::parse_fragment(&body);

    let selector = Selector::parse("#__next > div.cn-content-area.content-article > div > div > article > div.Post_content__qHjjk.insert-tldr-above > p:not(:first-child)").unwrap();

    let g: Vec<_> = fragment.select(&selector).flat_map(|e| e.text()).collect();

    g.chunks(2)
        .map(|chunk| Term {
            title: chunk[0].to_string(),
            body: chunk.get(1).copied().unwrap_or("").to_string(),
            images: vec![],
        })
        .collect()
}

// nth-childでは指定できなかったので手書き
pub async fn footballbox() -> Vec<Term> {
    let body = get_html("https://footballbox.club/word.html", "utf-8")
        .await
        .unwrap();

    let fragment = Html::parse_fragment(&body);

    let titles_selector =
        Selector::parse("#leftside > article > section > section > dl > dt").unwrap();
    let bodies_selector =
        Selector::parse("#leftside > article > section > section > dl > dd").unwrap();

    let titles: Vec<_> = fragment
        .select(&titles_selector)
        .flat_map(|e| e.text())
        .collect();
    let bodies: Vec<_> = fragment
        .select(&bodies_selector)
        .enumerate()
        .filter(|(index, _)| *index != 104 && *index != 152 && *index != 180)
        .map(|(_, e)| e.text().collect::<String>())
        .collect();

    titles
        .iter()
        .zip(bodies.iter())
        .map(|(t, b)| Term {
            title: t.to_string(),
            body: b.to_string(),
            images: vec![],
        })
        .collect()
}

pub async fn hiroshima() -> Vec<Term> {
    /*
    FlowB {
        index: "https://www.pref.hiroshima.lg.jp/soshiki/19/1178070843217.html",
        // html broken?
        //            titles_selector: ".sp_table_wrap > div:nth-child(1) > table:nth-child(1) > tbody:nth-child(1) > tr > td:nth-child(1)",
        //            bodies_selector: ".sp_table_wrap > div:nth-child(1) > table:nth-child(1) > tbody:nth-child(1) > tr > td:nth-child(2)",
        links: vec![String::from(
            "https://www.pref.hiroshima.lg.jp/soshiki/19/1178070843217.html",
        )],
        ..Default::default()
    }
    */
    let body = get_html(
        "https://www.pref.hiroshima.lg.jp/soshiki/19/1178070843217.html",
        "utf-8",
    )
    .await
    .unwrap();

    let fragment = Html::parse_fragment(&body);

    let selector = Selector::parse(".detail_free").unwrap();

    let g: Vec<_> = fragment.select(&selector).flat_map(|e| e.text()).collect();
    (28..434)
        .step_by(14)
        .map(|i| Term {
            title: g[i].to_string(),
            body: g[i + 4].to_string(),
            images: vec![],
        })
        .collect()
}

pub async fn jfadocuments() -> Vec<Term> {
    let body = get_html("https://www.jfa.jp/documents/faq/terminology.html", "utf-8")
        .await
        .unwrap();

    let fragment = Html::parse_fragment(&body);

    let selector =
        Selector::parse("#main-colum > div.section-block.doc_QA > div > table > tbody > tr > td")
            .unwrap();

    let g: Vec<_> = fragment.select(&selector).flat_map(|e| e.text()).collect();

    g.chunks(4)
        .map(|chunk| Term {
            title: chunk[0].to_string(),
            body: chunk.get(1).copied().unwrap_or("").to_string()
                + "\n"
                + chunk.get(2).copied().unwrap_or("")
                + "\n"
                + chunk.get(3).copied().unwrap_or(""),
            images: vec![],
        })
        .collect()
}

pub async fn moonlight() -> Vec<Term> {
    let mut links = FlowA {
        level2_links: vec![String::from("http://www.moon-light.ne.jp/termi-nology/")],
        base: "http://www.moon-light.ne.jp/termi-nology/",
        level1_selector:
            ".entry > table:nth-child(8) > tbody > tr > td > table > tbody > tr > td > a",
        ..Default::default()
    }
    .get_links()
    .await;

    links.dedup();

    let terms = FlowA {
        links: links.clone(),
        base: "http://www.moon-light.ne.jp/termi-nology/",
        title_selector: ".date",
        body_selector: ".entry-more",
        ..Default::default()
    }
    .get_terms()
    .await;

    let mut renews = vec![];

    for (i, t) in terms.into_iter().enumerate() {
        if t.body != "" {
            renews.push(t);
            continue;
        }
        let link = links.get(i).unwrap();

        // not found
        if link == "http://www.moon-light.ne.jp/termi-nology/meaning/sliding-stage" {
            continue;
        }

        let html = get_html(link, "shift-jis").await.unwrap();

        let mut left = 0;
        for left_selector in [
            "</noscript></iframe> \n",
            "</noscript></iframe><br>\n",
            "</a></noscript></IFRAME>",
            "</noscript></iframe>",
            "border=\"0\"></a>",
            "</table>",
        ] {
            if let Some(s) = html.find(left_selector) {
                left = s + left_selector.len();
                break;
            }
        }
        if left == 0 {
            panic!("no selectors match!")
        };
        let right = html
            .find("<a href=\"http://www.moon-light.ne.jp/termi-nology/")
            .unwrap();

        let body = html[left..right].replace("<br>\n", "");

        let title = get_term(link.to_string(), "title", "none", None, "shift-jis")
            .await
            .unwrap()
            .title;

        renews.push(Term {
            title: title,
            body: body,
            images: t.images,
        });
    }
    renews
}

// CSSでは指定できなかったので手書き
pub async fn toraiz() -> Vec<Term> {
    let body = get_html("https://toraiz.jp/english-times/book/10573/", "utf-8")
        .await
        .unwrap();

    let fragment = Html::parse_fragment(&body);

    let titles_selector = Selector::parse("div.relative.hidden > main > div.l-main__inner > div > article > div.articleSection__content > h4").unwrap();
    let titles: Vec<_> = fragment
        .select(&titles_selector)
        .map(|e| e.text().collect::<String>())
        .collect();

    println!("{:?}", titles);

    // called `Result::unwrap()` on an `Err` value: UnexpectedSelectorParseError(UnsupportedPseudoClassOrElement("has"))
    // let bodies_selector = Selector::parse("div.relative.hidden > main > div.l-main__inner > div > article > div.articleSection__content > p:not(:has(img)))").unwrap();
    // JSでクラスいじってた
    // let bodies_selector = Selector::parse("div.relative.hidden > main > div.l-main__inner > div > article > div.articleSection__content > p:not(.hasImg)").unwrap();
    let bodies_selector = Selector::parse("div.relative.hidden > main > div.l-main__inner > div > article > div.articleSection__content > p").unwrap();
    let img_selector = Selector::parse("img").unwrap();
    let bodies: Vec<_> = fragment
        .select(&bodies_selector)
        .filter(|e| e.select(&img_selector).next().is_none())
        .map(|e| e.text().collect::<String>())
        .skip(4)
        .collect();

    titles
        .iter()
        .zip(bodies.iter())
        .map(|(t, b): (_, _)| Term {
            title: t.to_string(),
            body: b.to_string(),
            images: vec![],
        })
        .collect()
}

pub async fn yodosha() -> Vec<Term> {
    let level2_links = FlowA {
        index: "https://www.yodosha.co.jp/jikkenigaku/keyword/",
        base: "https://www.yodosha.co.jp/jikkenigaku/keyword/",
        level2_selector: "div.indexes > table > tbody > tr> td > a",
        ..Default::default()
    }
    .get_level2_links()
    .await;
    let links: Vec<String> = join_all(level2_links.iter().map(|link_link| {
        get_links(LinkQuery {
            url: link_link,
            base: "https://www.yodosha.co.jp/jikkenigaku/keyword/",
            selector_string: "#indexlistbox > ul > li > a",
            encoding: "utf-8",
        })
    }))
    .await
    .into_iter()
    .flat_map(|r| r.unwrap())
    .collect();

    FlowA {
        links: links,
        title_selector: "div.col-sm-8:nth-child(1) > div > h1",
        body_selector: "#ruledline > p:nth-child(1)",
        pool_size: 100,
        rest: 30,
        ..Default::default()
    }
    .get_terms()
    .await
}

pub async fn nikken() -> Vec<Term> {
    let level3_links = get_links(LinkQuery {
        url: "https://www.nikken-times.co.jp/dictionary/",
        base: "https://www.nikken-times.co.jp",
        selector_string: "#content > div:nth-child(2) > a",
        encoding: "utf-8",
    })
    .await
    .unwrap();

    let mut level2_links = vec![];

    for level3_link in level3_links {
        let link = get_links(LinkQuery {
            url: &level3_link,
            base: "https://www.nikken-times.co.jp",
            selector_string: ".charNext > a",
            encoding: "utf-8",
        })
        .await
        .unwrap();

        if link.is_empty() {
            continue;
        }

        let mut tmp_level2_links = vec![link.first().unwrap().to_string()];

        loop {
            tokio::time::sleep(Duration::from_secs(1)).await;
            let next = get_links(LinkQuery {
                url: &tmp_level2_links.last().unwrap(),
                base: "https://www.nikken-times.co.jp",
                selector_string: ".charNext > a",
                encoding: "utf-8",
            })
            .await
            .unwrap();

            if next.is_empty() {
                break;
            } else {
                tmp_level2_links.push(next.first().unwrap().to_string());
            }
        }

        level2_links.append(&mut tmp_level2_links);
    }

    FlowA {
        index: "https://www.nikken-times.co.jp/dictionary/",
        base: "https://www.nikken-times.co.jp",
        level2_links: level2_links,
        level1_selector: ".list > ul:nth-child(1) > li > a",
        title_selector: ".post-title",
        body_selector: ".post > p:nth-child(2)",
        ..Default::default()
    }
    .get_terms()
    .await
}
