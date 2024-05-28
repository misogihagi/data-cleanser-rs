use std::vec;

use crate::utils::{get_html, get_text, Flow, FlowA, FlowB, Term};
use clap::Parser;
use futures::future::join_all;
use scraper::{Html, Selector};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value_t = false)]
    goonet: bool,
}
pub enum SiteKind {
    SimpleA(SiteKindSimpleA),
    SimpleB(SiteKindSimpleB),
    Customize(SiteKindCustomize),
    Ajima,
    Hiroshima,
}
pub enum SiteKindSimpleA {
    Aritayaki,
    Athome,
    Beer,
    Chemicoat,
    Cybernet,
    Ena,
    ESP,
    Fastretailing,
    Goonet,
    Gurubi,
    JMAC,
    Kenchikuyogo,
    Macromill,
    Mintesu,
    Nomura,
    //    Ntt,
    Ryugaku,
    Sumai1,
    Smbcnikko,
    Soccer,
    WaferMeasurementInspection,
    Webtan,
}

fn simple_a(kind: SiteKindSimpleA) -> FlowA<'static> {
    match kind {
        SiteKindSimpleA::Aritayaki => FlowA {
            link_links: (1..8).map(|i| "http://www.aritayaki-fun.com/?cat=7&paged=".to_string()+&i.to_string()).collect(),
            link_selector: ".main-conts > article > div > div > p > a",
            title_selector: "h1.section-title",
            body_selector: ".article-body > p:nth-child(1)",
            ..Default::default()
        },
        SiteKindSimpleA::Athome => FlowA {
            index: "https://www.athome.co.jp/contents/words/",
            base: "https://www.athome.co.jp",
            link_link_selector: ".f_l  li a,.f_r li a,.wbox:nth-child(2) li a",
            link_selector: "ul.cf > li > a",
            title_selector: "#h1_title > h1",
            body_selector: ".textarea",
            ..Default::default()
        },
        SiteKindSimpleA::Beer => FlowA {
            link_links: vec![String::from("https://craft-beer.life/dictionary")],
            link_selector: "li.dictionary_section > ol > li > a",
            title_selector: ".article_headline",
            body_selector: ".article_body > p",
            ..Default::default()
        },
        SiteKindSimpleA::Chemicoat => FlowA {
            link_links: vec![String::from("https://www.chemicoat.co.jp/knowledge/")],
            link_selector: ".content-list > ul > li > a",
            title_selector: ".columnh2",
            body_selector: ".columntext",
            ..Default::default()
        },
        SiteKindSimpleA::Cybernet => FlowA {
            index: "https://www.cybernet.co.jp/optical/glossary/",
            base: "https://www.cybernet.co.jp",
            link_selector: ".contents_body > section > div > ul > li > a",
            title_selector: ".page_ttl > h1",
            body_selector: ".text_blc",
            ..Default::default()
        },
        SiteKindSimpleA::Ena => FlowA {
            index: "https://www.ena.travel/glossary/all/",
            base: "https://www.ena.travel",
            link_selector: "div.body > dl > dd:nth-child(2) > ul:nth-child(1) > li > a",
            title_selector: "div.guide > h2 > span > span",
            body_selector: "div.text > p",
            ..Default::default()
        },
        SiteKindSimpleA::ESP => FlowA {
            index: "https://www.esp.ac.jp/epv/glossary/index.html",
            base: "https://www.esp.ac.jp/epv/glossary/",
            link_link_selector: "#glossary-navi > li > a",
            link_selector: "#glossary-list > li > a",
            title_selector: "#glossary-name",
            body_selector: "#glossary-text",
            ..Default::default()
        },
        SiteKindSimpleA::Fastretailing => FlowA {
            index: "https://www.fastretailing.com/jp/glossary",
            base: "https://www.fastretailing.com",
            link_link_selector:".menuli > li > a",
            link_selector: "#alphabet-index > dd > ul > li > a",
            title_selector: "#content > h1:nth-child(3)",
            body_selector: "#entry-535",
            image_selector: Some("#entry-535 img"),
            ..Default::default()
        },
        SiteKindSimpleA::Goonet => FlowA {
            index: "https://www.goo-net.com/knowledge/",
            base: "https://www.goo-net.com",
            link_link_selector: "#main > section > div > dl > dd:nth-child(2) > ul > li > a",
            link_selector: ".column2 > li > a",
            title_selector: ".h3box > h5",
            body_selector: ".text",
            encoding: "euc-jp",
            ..Default::default()
        },
        SiteKindSimpleA::Gurubi => FlowA {
            link_links: vec![String::from("https://gurubi.ac.jp/glossary/")],
            base: "https://gurubi.ac.jp/glossary/",
            link_selector: "div.glossary-list > ul > li > a",
            title_selector: ".yogo > h2",
            body_selector: ".yogo > p",
            ..Default::default()
        },
        SiteKindSimpleA::JMAC => FlowA {
            index: "https://www.jmac.co.jp/glossary/",
            base: "https://www.jmac.co.jp",
            link_link_selector:"section.l-pageSection:nth-child(4) > ul > li > a, section.l-pageSection:nth-child(5) > ul > li > a, section.l-pageSection:nth-child(6) > ul > li > a",
            link_selector:".c-glossaryList > li > a",
            title_selector: "h1.c-simpleHeader_title",
            body_selector: ".l-wysiwyg",
            ..Default::default()
        },
        SiteKindSimpleA::Kenchikuyogo => FlowA {
            index: "https://kenchikuyogo.com/",
            link_link_selector:"figure.wp-block-table:nth-child(3) > table:nth-child(1) > tbody:nth-child(1) > tr > td > a",
            link_selector:".is-style-stripes > table:nth-child(1) > tbody:nth-child(1) > tr > td > a",
            title_selector: "h1.alignwide",
            body_selector: ".entry-content > p:not([class])",
            ..Default::default()
        },
        SiteKindSimpleA::Macromill => FlowA {
            index: "https://www.macromill.com/service/words/",
            link_link_selector: ".indexPc > ul > li > a",
            link_selector: ".main > .posts > .pnl > a",
            title_selector: ".head > div > h1",
            body_selector: ".un_secBlock:not(.lo_mgnTopL):not(.lo_mgnTopM),.hp_mgnTopM",
            ..Default::default()
        },
        SiteKindSimpleA::Mintesu => FlowA {
            index: "https://www.mintetsu.or.jp/knowledge/",
            link_link_base: "https://www.mintetsu.or.jp",
            link_link_selector: ".ContentsList01 > ul:nth-child(1) > li:nth-child(2) > ul:nth-child(2) > li  > a",
            link_selector: "ul.wordList > li > a",
            title_selector: "h1",
            body_selector: ".section.clearfix",
            ..Default::default()
        },
        SiteKindSimpleA::Nomura => FlowA {
            index: "https://www.nomura.co.jp/terms/",
            base: "https://www.nomura.co.jp",
            link_link_selector: ".tbl > tbody:nth-child(1) > tr > td > p > a",
            link_selector: ".-transform > li > a",
            title_selector: "#term_id",
            body_selector: ".glossary-block",
            ..Default::default()
        },
        /*
        SiteKindSimpleA::Ntt => FlowA {
            index: "https://www.ntt.com/bizon/glossary.html",
            base: "https://www.ntt.com",
            link_selector: "#it-glossary-list-tab-words > div > div > div > ul > li > a",            
            title_selector: ".article-title",
            body_selector: ".article-contents",
            ..Default::default()
        },
        */
        SiteKindSimpleA::Ryugaku => FlowA {
            index: "https://ryugaku.kuraveil.jp/dictionaries",
            base: "https://ryugaku.kuraveil.jp",
            link_link_selector: "div.initial-index:nth-child(4) > a",
            link_selector: ".word-list > li > a",
            title_selector: ".header-title",
            body_selector: ".markdown",
            ..Default::default()
        },
        SiteKindSimpleA::Sumai1 => FlowA {
            index: "https://www.sumai1.com/useful/words/",
            base: "https://www.sumai1.com",
            link_link_selector: ".innerbody > div > div > ul > li > a, .index-alphabet > div > div:nth-child(1) > ul > li:nth-child(1) > a , div.tb-row:nth-child(3) > div:nth-child(2) > ul:nth-child(1) > li:nth-child(1) > a, .index-number > div:nth-child(1) > div > ul > li:nth-child(2) > a",
            link_selector: "ul.col2:nth-child(1) > li > a",
            title_selector: ".images > h1",
            body_selector: ".description",
            ..Default::default()
        },
        SiteKindSimpleA::Smbcnikko => FlowA {
            index: "https://www.smbcnikko.co.jp/terms/index.html",
            base: "https://www.smbcnikko.co.jp",
            link_link_selector: ".A > li > a, .B > li > a, #anchor02 > li > a",
            link_selector: ".box-release-inner > ul > li > a",
            title_selector: "#main > section > section:nth-child(2) > h2 > span",
            body_selector: "#main > section:nth-child(1) > section:nth-child(2) > div:nth-child(2) > p:nth-child(1), p.box-img:nth-child(2), #main > section:nth-child(1) > section:nth-child(2) > div:nth-child(2) > section:nth-child(3)",
            encoding: "shift-jis",
            pool_size:100,
            rest:60,
            ..Default::default()
        },
        SiteKindSimpleA::Soccer => FlowA {
            index: "https://www.homemate-research-soccer.com/useful/glossary/soccer/",
            base: "https://www.homemate-research-soccer.com",
            link_selector: "section.sec_cmn:nth-child(4) > div > ul > li > a",
            title_selector: ".post_btn > h1:nth-child(1)",
            body_selector: ".post_box",
            image_selector: Some(".post_box_img > img:nth-child(1)"),
            ..Default::default()
        },
        SiteKindSimpleA::WaferMeasurementInspection => FlowA {
            link_links: vec![String::from("https://www.wafer-measurement-inspection.com/words/")],
            base: "https://www.wafer-measurement-inspection.com/words/",
            link_selector: ".newslist > li > a",
            title_selector: ".ts3",
            body_selector: ".longComment",
            ..Default::default()
        },
        SiteKindSimpleA::Webtan => FlowA {
            index: "https://webtan.impress.co.jp/glossary/list/1",
            base: "https://webtan.impress.co.jp",
            link_selector: ".node > div:nth-child(1) > ul > li > a",
            title_selector: "h1.title",
            body_selector: ".glossary_description",
            ..Default::default()
        },
    }
}

pub enum SiteKindSimpleB {
    Meiwakaiun,
    Theglenlivet,
}

fn simple_b(kind: SiteKindSimpleB) -> FlowB {
    match kind {
        SiteKindSimpleB::Meiwakaiun => FlowB {
            index: "https://www.meiwakaiun.com/meiwalabo/yougo/",
            // section.grossary typo!
            titles_selector: "main > div.page-box > div.container > div.glossary-details > section.grossary-details-box > table > tbody > tr > td > div.glossary-details > section.grossary-details-box > table > tbody > tr:not(#ki02a) > th",
            bodies_selector: "main > div.page-box > div.container > div.glossary-details > section.grossary-details-box > table > tbody > tr > td > div.glossary-details > section.grossary-details-box > table > tbody > tr:not(#ki02a) > td",
            ..Default::default()
        },
        SiteKindSimpleB::Theglenlivet => FlowB {
            index: "https://www.theglenlivet.jp/craft/whisky-words.html",
            titles_selector: "section.producttext > div > h3",
            bodies_selector: "section.producttext > div > h3 + p",
            ..Default::default()
        },
    }
}

pub enum SiteKindCustomize {
    Hrpro,
}

fn customize(kind: SiteKindCustomize) -> impl Flow {
    match kind {
        SiteKindCustomize::Hrpro => {
            let resource = "https://www.hrpro.co.jp/glossary.php?";
            let urls = vec!["a", "k", "s", "t", "n", "h", "m", "y", "r", "w"]
                .into_iter()
                .fold(vec![], |mut total, query| {
                    total.append(&mut vec![
                        resource.to_string() + "index=" + query,
                        resource.to_string() + "index=" + query + "&pcnt=2",
                    ]);
                    total
                });
            FlowA {
                index: "https://www.hrpro.co.jp/glossary.php",
                base: "https://www.hrpro.co.jp/",
                link_selector: ".rlt-list > li > a",
                title_selector: "h1.ttl",
                body_selector: ".article-body",
                link_links: urls,
                ..Default::default()
            }
        } //    SiteKindCustomize::Mitsue => { },
    }
}

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

fn str_to_kind(s: &str) -> SiteKind {
    match s {
        "ajima" => SiteKind::Ajima,
        "aritayaki" => SiteKind::SimpleA(SiteKindSimpleA::Aritayaki),
        "athome" => SiteKind::SimpleA(SiteKindSimpleA::Athome),
        "beer" => SiteKind::SimpleA(SiteKindSimpleA::Beer),
        "chemicoat" => SiteKind::SimpleA(SiteKindSimpleA::Chemicoat),
        "cybernet" => SiteKind::SimpleA(SiteKindSimpleA::Cybernet),
        "ena" => SiteKind::SimpleA(SiteKindSimpleA::Ena),
        "esp" => SiteKind::SimpleA(SiteKindSimpleA::ESP),
        "fastretailing" => SiteKind::SimpleA(SiteKindSimpleA::Fastretailing),
        "goonet" => SiteKind::SimpleA(SiteKindSimpleA::Goonet),
        "gurubi" => SiteKind::SimpleA(SiteKindSimpleA::Gurubi),
        "hiroshima" => SiteKind::Hiroshima,
        "hrpro" => SiteKind::Customize(SiteKindCustomize::Hrpro),
        "jmac" => SiteKind::SimpleA(SiteKindSimpleA::JMAC),
        "kenchikuyogo" => SiteKind::SimpleA(SiteKindSimpleA::Kenchikuyogo),
        "macromill" => SiteKind::SimpleA(SiteKindSimpleA::Macromill),
        "meiwakaiun" => SiteKind::SimpleB(SiteKindSimpleB::Meiwakaiun),
        "mintetsu" => SiteKind::SimpleA(SiteKindSimpleA::Mintesu),
        "mitsue" => SiteKind::SimpleA(SiteKindSimpleA::Mintesu),
        "nomura" => SiteKind::SimpleA(SiteKindSimpleA::Nomura),
        //        "ntt" => SiteKind::SimpleA(SiteKindSimpleA::Ntt),
        "ryugaku" => SiteKind::SimpleA(SiteKindSimpleA::Ryugaku),
        "sumai1" => SiteKind::SimpleA(SiteKindSimpleA::Sumai1),
        "smbcnikko" => SiteKind::SimpleA(SiteKindSimpleA::Smbcnikko),
        "soccer" => SiteKind::SimpleA(SiteKindSimpleA::Soccer),
        "theglenlivet" => SiteKind::SimpleB(SiteKindSimpleB::Theglenlivet),
        "wafermeasurementinspection" => SiteKind::SimpleA(SiteKindSimpleA::WaferMeasurementInspection),
        "webtan" => SiteKind::SimpleA(SiteKindSimpleA::Webtan),
        &_ => panic!("not valid kind"),
    }
}

pub async fn run(kind_str: &str) -> Vec<Term> {
    let kind = str_to_kind(kind_str);
    let terms = match kind {
        SiteKind::SimpleA(k) => simple_a(k).get_terms().await,
        SiteKind::SimpleB(k) => simple_b(k).get_terms().await,
        SiteKind::Customize(k) => customize(k).get_terms().await,
        SiteKind::Ajima => ajima().await,
        SiteKind::Hiroshima => hiroshima().await,
    };
    terms
}
