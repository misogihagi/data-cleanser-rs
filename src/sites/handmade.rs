use futures::future::join_all;
use std::{thread, time};

use scraper::{Html, Selector};

use super::interface::WorkFlowTrait;
use crate::utils::{get_html, get_links, get_term, get_text, Flow, FlowA, LinkQuery, Term};

pub enum SiteKindHandmade {
    Ajima,
    Hiroshima,
    MoonLight,
    Nikken,
    Yodosha,
}

pub struct HandmadeWorkFlow {
    pub kind: SiteKindHandmade,
}
pub trait HandmadeWorkFlowTrait: WorkFlowTrait {
    fn new(kind_str: &'static str) -> HandmadeWorkFlow;
    fn my_kind(kind_str: &'static str) -> Option<SiteKindHandmade>;
}

impl HandmadeWorkFlowTrait for HandmadeWorkFlow {
    fn new(kind_str: &'static str) -> HandmadeWorkFlow {
        HandmadeWorkFlow {
            kind: HandmadeWorkFlow::my_kind(kind_str).unwrap(),
        }
    }
    fn my_kind(kind_str: &'static str) -> Option<SiteKindHandmade> {
        match kind_str {
            "ajima" => Some(SiteKindHandmade::Ajima),
            "hiroshima" => Some(SiteKindHandmade::Hiroshima),
            "moonlight" => Some(SiteKindHandmade::MoonLight),
            "nikken" => Some(SiteKindHandmade::Nikken),
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
            &SiteKindHandmade::Hiroshima => hiroshima().await,
            &SiteKindHandmade::MoonLight => moonlight().await,
            &SiteKindHandmade::Nikken => nikken().await,
            &SiteKindHandmade::Yodosha => yodosha().await,
        }
    }
}

pub async fn ajima() -> Vec<Term> {
    let links = FlowA {
        index: "https://hougen.ajima.jp/gojyuon/",
        base: "https://hougen.ajima.jp",
        link_link_selector: "ul.gojyuon > li > a",
        link_selector: ".list_hougen > li > article > a",
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

pub async fn moonlight() -> Vec<Term> {
    let mut links = FlowA {
        link_links: vec![String::from("http://www.moon-light.ne.jp/termi-nology/")],
        base: "http://www.moon-light.ne.jp/termi-nology/",
        link_selector:
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

pub async fn yodosha() -> Vec<Term> {
    let link_links = FlowA {
        index: "https://www.yodosha.co.jp/jikkenigaku/keyword/",
        base: "https://www.yodosha.co.jp/jikkenigaku/keyword/",
        link_link_selector: "div.indexes > table > tbody > tr> td > a",
        ..Default::default()
    }
    .get_link_links()
    .await;
    let links: Vec<String> = join_all(link_links.iter().map(|link_link| {
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
    let link_link_links = get_links(LinkQuery {
        url: "https://www.nikken-times.co.jp/dictionary/",
        base: "https://www.nikken-times.co.jp",
        selector_string: "#content > div:nth-child(2) > a",
        encoding: "utf-8",
    })
    .await
    .unwrap();

    let mut link_links = vec![];

    for link_link_link in link_link_links {
        let link = get_links(LinkQuery {
            url: &link_link_link,
            base: "https://www.nikken-times.co.jp",
            selector_string: ".charNext > a",
            encoding: "utf-8",
        })
        .await
        .unwrap();

        if link.len() == 0 {
            continue;
        }

        let mut tmp_link_links = vec![link.first().unwrap().to_string()];

        loop {
            thread::sleep(time::Duration::from_secs(1));
            let next = get_links(LinkQuery {
                url: &tmp_link_links.last().unwrap(),
                base: "https://www.nikken-times.co.jp",
                selector_string: ".charNext > a",
                encoding: "utf-8",
            })
            .await
            .unwrap();

            if next.len() == 0 {
                break;
            } else {
                tmp_link_links.push(next.first().unwrap().to_string());
            }
        }

        link_links.append(&mut tmp_link_links);
    }

    FlowA {
        index: "https://www.nikken-times.co.jp/dictionary/",
        base: "https://www.nikken-times.co.jp",
        link_links: link_links,
        link_selector: ".list > ul:nth-child(1) > li > a",
        title_selector: ".post-title",
        body_selector: ".post > p:nth-child(2)",
        ..Default::default()
    }
    .get_terms()
    .await
}
