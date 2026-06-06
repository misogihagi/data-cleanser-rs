use futures::future::join_all;
use regex::Regex;
use std::time::Duration;

use scraper::{Html, Selector};

use super::interface::WorkFlowTrait;
use crate::utils::{
    get_html, get_links, get_term, get_terms, get_text, Flow, HierarchicalFlow, LinkQuery, Term,
};

pub enum SiteKindHandmade {
    Ajima,
    Coocan,
    Efjapan,
    Footballbox,
    Hiroshima,
    Jfadocuments,
    Kddi,
    MoonLight,
    Nikken,
    Ntt,
    Toraiz,
    Naganofc,
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
            "kddi" => Some(SiteKindHandmade::Kddi),
            "moonlight" => Some(SiteKindHandmade::MoonLight),
            "ntt" => Some(SiteKindHandmade::Ntt),
            "nikken" => Some(SiteKindHandmade::Nikken),
            "toraiz" => Some(SiteKindHandmade::Toraiz),
            "naganofc" => Some(SiteKindHandmade::Naganofc),
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
            &SiteKindHandmade::Kddi => kddi().await,
            &SiteKindHandmade::MoonLight => moonlight().await,
            &SiteKindHandmade::Nikken => nikken().await,
            &SiteKindHandmade::Ntt => ntt().await,
            &SiteKindHandmade::Toraiz => toraiz().await,
            &SiteKindHandmade::Naganofc => naganofc().await,
        }
    }
}

pub async fn ajima() -> Vec<Term> {
    let links = HierarchicalFlow {
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
    SinglepageFlow {
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

fn get_ids(body: &str, start_marker: &str) -> Vec<String> {
    let mut urls = Vec::new();
    let end_marker = "}]\n  </script>";

    if let Some(start_idx) = body.find(start_marker) {
        let start = start_idx + start_marker.len();
        let tail = &body[start..];
        if let Some(end) = tail.find(end_marker) {
            let json_str = &tail[..=end + 1];
            println!("{}", json_str);

            if let Ok(value) = serde_json::from_str::<serde_json::Value>(json_str) {
                let mut stack = vec![&value];
                while let Some(v) = stack.pop() {
                    match v {
                        serde_json::Value::String(s) => {
                            if s.starts_with("http://") || s.starts_with("https://") {
                                urls.push(s.to_string());
                            }
                        }
                        serde_json::Value::Array(arr) => {
                            for item in arr {
                                stack.push(item);
                            }
                        }
                        serde_json::Value::Object(obj) => {
                            if obj.contains_key("id") {
                                urls.push(obj["id"].as_str().unwrap().to_string());
                            }
                        }
                        _ => {}
                    }
                }
            }
        }
    }

    urls
}

pub async fn kddi() -> Vec<Term> {
    let body = get_html("https://biz.kddi.com/content/glossary/", "utf-8")
        .await
        .unwrap();

    let alphabet_ids = get_ids(&body, "BIZ.Vars.EmbedTaglist.termAlphabet = ");
    let japanese_ids = get_ids(&body, "BIZ.Vars.EmbedTaglist.termJapanese = ");

    let json_bodies = join_all([alphabet_ids, japanese_ids].concat().iter().map(|id| {
        get_html(
            "https://biz.kddi.com/bin/glossary.json?ck_lang=ja&ck_initial=".to_string() + id,
            "utf-8",
        )
    }))
    .await
    .into_iter()
    .map(|b| b.unwrap())
    .map(|b| serde_json::from_str::<serde_json::Value>(&b).unwrap())
    .into_iter()
    .flat_map(|v| {
        v.as_array()
            .unwrap()
            .into_iter()
            .map(|v| v.as_object().unwrap()["url"].as_str().unwrap().to_string())
            .collect::<Vec<_>>()
    })
    .collect::<Vec<_>>();

    let terms = HierarchicalFlow {
        links: json_bodies,
        title_selector: "h2.biz-c-glossary__detailHeading:nth-child(1)",
        body_selector: ".biz-c-glossary__detailParagraph",
        ..Default::default()
    }
    .get_terms()
    .await;

    terms
}

pub async fn moonlight() -> Vec<Term> {
    let mut links = HierarchicalFlow {
        level2_links: vec![String::from("http://www.moon-light.ne.jp/termi-nology/")],
        base: "http://www.moon-light.ne.jp/termi-nology/",
        level1_selector: "table > tbody > tr > td > table > tbody > tr > td > a",
        ..Default::default()
    }
    .get_links()
    .await;

    links.dedup();

    let terms = HierarchicalFlow {
        links: links.clone(),
        base: "http://www.moon-light.ne.jp/termi-nology/",
        title_selector: "h3",
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

fn extract_urls(js_text: &str) -> Vec<String> {
    // `"url": "実際のURL"` というパターンにマッチさせる正規表現
    // キャプチャグループ1（[^"]+）でURLの中身を抽出します
    let re = Regex::new(r#""url"\s*:\s*"([^"]+)""#).unwrap();
    let mut urls = Vec::new();

    // テキスト全体から条件に一致する部分をすべて検索
    for cap in re.captures_iter(js_text) {
        // cap[1] が抽出したURL文字列
        urls.push(cap[1].to_string());
    }

    urls
}
pub async fn ntt() -> Vec<Term> {
    let body = get_html(
        "https://www.ntt.com/etc/designs/nttcom/hq/jp/bizon/js/2024/glossary-list-reading.js",
        "utf-8",
    )
    .await
    .unwrap();

    let extracted_urls = extract_urls(&body);

    let s_title = ".article-title";
    let s_body = ".article-wrap > p";

    let mut results = vec![];

    for chunk in extracted_urls.chunks(10) {
        let futures = chunk.iter().map(|url| {
            get_term(
                "https://www.ntt.com".to_string() + url,
                s_title,
                s_body,
                None,
                "utf-8",
            )
        });

        let chunk_results = join_all(futures).await;
        results.extend(chunk_results.into_iter().map(|res| res.unwrap()));
        tokio::time::sleep(Duration::from_secs(1)).await;
    }

    results
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
    let level2_links = HierarchicalFlow {
        index: "https://www.yodosha.co.jp/jikkenigaku/keyword/",
        level2_selector: "div.indexes > table > tbody > tr> td > a",
        ..Default::default()
    }
    .get_level2_links()
    .await;
    let links: Vec<String> = join_all(level2_links.iter().map(|link_link| {
        get_links(LinkQuery {
            url: link_link,
            base: "",
            selector_string: "#indexlistbox > ul > li > a",
            encoding: "utf-8",
        })
    }))
    .await
    .into_iter()
    .flat_map(|r| r.unwrap())
    .collect();

    HierarchicalFlow {
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

    HierarchicalFlow {
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

pub async fn naganofc() -> Vec<Term> {
    let body = get_html(
        "https://www.naganofc.com/wp/aboutus/knowledge/tactics",
        "utf-8",
    )
    .await
    .unwrap();

    let fragment = Html::parse_fragment(&body);

    // 親要素である .entry-body 直下の全要素を上から順番に取得する
    let selector = Selector::parse(".entry-body > *").unwrap();

    let mut terms = vec![];
    let mut current_title = String::new();
    let mut current_body = String::new();
    let mut is_in_defence = false;

    // 順番にループ
    for el in fragment.select(&selector) {
        let tag_name = el.value().name(); // "h3", "p", "div" 等
        let t = el.text().collect::<String>();
        let trimmed = t.trim();

        if trimmed == "１１．ゾーンプレス" {
            is_in_defence = false;
        }

        if tag_name == "h3" {
            // 新しいh3が来た時点で、これまでにパースしたものを保存する
            if !current_title.is_empty() {
                terms.push(Term {
                    title: current_title.clone(),
                    body: current_body.trim().to_string(),
                    images: vec![],
                });
            }
            // 新しいTitleを設定し、Bodyを空にリセットする
            current_title = el.text().collect::<String>().trim().to_string();
            current_body = String::new();
        } else if is_in_defence {
            if !trimmed.is_empty() && tag_name == "h5" {
                if !current_title.is_empty() {
                    terms.push(Term {
                        title: current_title.clone(),
                        body: current_body.trim().to_string(),
                        images: vec![],
                    });
                }
                // 新しいTitleを設定し、Bodyを空にリセットする
                current_title = trimmed.to_string();
                current_body = String::new();
            } else if !trimmed.is_empty() && tag_name == "p" {
                current_body.push_str(trimmed);
                current_body.push('\n');
            }
        } else if !current_title.is_empty() {
            if trimmed == "(ア)ディフェンスの用語" {
                // ディフェンスの用語と判断して、TitleとBodyを空にリセットする
                current_title = String::new();
                current_body = String::new();
                is_in_defence = true;
            } else if !trimmed.is_empty() {
                current_body.push_str(trimmed);
                current_body.push('\n');
            }
        }
    }

    // 最後のH3以降の段落も忘れずにPushする
    if !current_title.is_empty() {
        terms.push(Term {
            title: current_title,
            body: current_body.trim().to_string(),
            images: vec![],
        });
    }

    terms
}
