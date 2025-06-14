use std::vec;

use super::interface::WorkFlowTrait;
use crate::utils::{Flow, FlowA, FlowB, Term};

pub enum SiteKindSimple {
    A(SiteKindSimpleA),
    B(SiteKindSimpleB),
}

pub struct SimpleWorkFlow {
    pub kind: SiteKindSimple,
}
pub trait SimpleWorkFlowTrait: WorkFlowTrait {
    fn new(kind_str: &'static str) -> SimpleWorkFlow;
    fn my_kind(kind_str: &'static str) -> Option<SiteKindSimple>;
}

impl SimpleWorkFlowTrait for SimpleWorkFlow {
    fn new(kind_str: &'static str) -> SimpleWorkFlow {
        SimpleWorkFlow {
            kind: SimpleWorkFlow::my_kind(kind_str).unwrap(),
        }
    }
    fn my_kind(kind_str: &'static str) -> Option<SiteKindSimple> {
        match kind_str {
            "aritayaki" => Some(SiteKindSimple::A(SiteKindSimpleA::Aritayaki)),
            "athome" => Some(SiteKindSimple::A(SiteKindSimpleA::Athome)),
            "beer" => Some(SiteKindSimple::A(SiteKindSimpleA::Beer)),
            "chemicoat" => Some(SiteKindSimple::A(SiteKindSimpleA::Chemicoat)),
            "cybernet" => Some(SiteKindSimple::A(SiteKindSimpleA::Cybernet)),
            "ena" => Some(SiteKindSimple::A(SiteKindSimpleA::Ena)),
            "esp" => Some(SiteKindSimple::A(SiteKindSimpleA::ESP)),
            "fastretailing" => Some(SiteKindSimple::A(SiteKindSimpleA::Fastretailing)),
            "felissimo" => Some(SiteKindSimple::A(SiteKindSimpleA::Felissimo)),
            "goonet" => Some(SiteKindSimple::A(SiteKindSimpleA::Goonet)),
            "gurubi" => Some(SiteKindSimple::A(SiteKindSimpleA::Gurubi)),
            "jhs" => Some(SiteKindSimple::A(SiteKindSimpleA::JHS)),
            "jmac" => Some(SiteKindSimple::A(SiteKindSimpleA::JMAC)),
            "kenchikuyogo" => Some(SiteKindSimple::A(SiteKindSimpleA::Kenchikuyogo)),
            "livable" => Some(SiteKindSimple::A(SiteKindSimpleA::Livable)),
            "macromill" => Some(SiteKindSimple::A(SiteKindSimpleA::Macromill)),
            "meiwakaiun" => Some(SiteKindSimple::B(SiteKindSimpleB::Meiwakaiun)),
            "mintetsu" => Some(SiteKindSimple::A(SiteKindSimpleA::Mintesu)),
            "mizuho" => Some(SiteKindSimple::A(SiteKindSimpleA::Mizuho)),
            "naigai" => Some(SiteKindSimple::A(SiteKindSimpleA::Naigai)),
            "nittsu" => Some(SiteKindSimple::A(SiteKindSimpleA::Nittsu)),
            "nomura" => Some(SiteKindSimple::A(SiteKindSimpleA::Nomura)),
            "nrisecure" => Some(SiteKindSimple::A(SiteKindSimpleA::Nrisecure)),
            "pfa" => Some(SiteKindSimple::A(SiteKindSimpleA::Pfa)),
            //        "ntt" => Some(SiteKindSimple::A(SiteKindSimpleA::Ntt)),
            "rewords" => Some(SiteKindSimple::A(SiteKindSimpleA::Rewords)),
            "ri" => Some(SiteKindSimple::B(SiteKindSimpleB::Ri)),
            "ryugaku" => Some(SiteKindSimple::A(SiteKindSimpleA::Ryugaku)),
            "sumai1" => Some(SiteKindSimple::A(SiteKindSimpleA::Sumai1)),
            "smbcnikko" => Some(SiteKindSimple::A(SiteKindSimpleA::Smbcnikko)),
            "smtrc" => Some(SiteKindSimple::A(SiteKindSimpleA::Smtrc)),
            "sobien" => Some(SiteKindSimple::A(SiteKindSimpleA::Sobien)),
            "soccer" => Some(SiteKindSimple::A(SiteKindSimpleA::Soccer)),
            "sompocybersecurity" => Some(SiteKindSimple::A(SiteKindSimpleA::Sompocybersecurity)),
            "suumo" => Some(SiteKindSimple::A(SiteKindSimpleA::Suumo)),
            "theglenlivet" => Some(SiteKindSimple::B(SiteKindSimpleB::Theglenlivet)),
            "wafermeasurementinspection" => Some(SiteKindSimple::A(
                SiteKindSimpleA::WaferMeasurementInspection,
            )),
            "webtan" => Some(SiteKindSimple::A(SiteKindSimpleA::Webtan)),
            _ => None,
        }
    }
}

impl WorkFlowTrait for SimpleWorkFlow {
    fn is_my_kind(kind_str: &'static str) -> bool {
        match SimpleWorkFlow::my_kind(kind_str) {
            Some(_) => true,
            None => false,
        }
    }
    async fn get_terms(&self) -> Vec<Term> {
        simple(&self.kind).await
    }
}

pub async fn simple(kind: &SiteKindSimple) -> Vec<Term> {
    match kind {
        SiteKindSimple::A(k) => simple_a(k).get_terms().await,
        SiteKindSimple::B(k) => simple_b(k).get_terms().await,
    }
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
    Felissimo,
    Goonet,
    Gurubi,
    JHS,
    JMAC,
    Kenchikuyogo,
    Livable,
    Macromill,
    Mintesu,
    Mizuho,
    Nittsu,
    Naigai,
    Nomura,
    Nrisecure,
    Pfa,
    //    Ntt,
    Rewords,
    Ryugaku,
    Sumai1,
    Smbcnikko,
    Smtrc,
    Sobien,
    Soccer,
    Sompocybersecurity,
    Suumo,
    WaferMeasurementInspection,
    Webtan,
}

fn simple_a(kind: &SiteKindSimpleA) -> FlowA<'static> {
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
        SiteKindSimpleA::Felissimo => FlowA {
            link_links: vec![String::from("https://www.felissimo.co.jp/niau/words/")],
            link_selector: "div.words-tabContents:nth-child(3) > div:nth-child(2) > div > div > ul > li > a",
            title_selector: ".cmn-pageTitle_main > span:nth-child(1)",
            body_selector: ".single-wordsArticleBody_contents > p",
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
        SiteKindSimpleA::JHS => FlowA {
            index: "https://www.jhs.ac.jp/guide/glossary/",
            link_link_selector:".glossary_words > dl > dd > a",            
            link_selector: ".glossary_category > div > a",
            title_selector: ".title > h1",
            body_selector: ".explain > p",
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
        SiteKindSimpleA::Livable => FlowA {
            link_links:vec![String::from("https://www.livable.co.jp/yogo/list/")],
            link_selector:"section.s-content__box> ul > li > a",
            title_selector: ".a-headline",
            body_selector: ".s-content__detail > p",
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
        SiteKindSimpleA::Mizuho => FlowA {
            index: "https://www.mizuho-re.co.jp/knowledge/dictionary/",
            base: "https://www.mizuho-re.co.jp",
            link_link_selector: "ul.colspan > li > a, ul.colspan2 > a",
            link_selector: "#list > ul > li > a",
            title_selector: "#select_word",
            body_selector: "#ue > div:nth-child(1) > div:nth-child(2)",
            ..Default::default()
        },
        SiteKindSimpleA::Naigai => FlowA {
            index: "https://www.ntl-naigai.co.jp/glossary/",
            base: "https://www.ntl-naigai.co.jp",
            link_link_selector: "li.-nt-naviAnchorFrame__item > a",
            link_selector: "li.-nt-glossaryList__item > a",
            title_selector: "h2.-nt-title6",
            body_selector: ".-nt-note",
            ..Default::default()
        },
        SiteKindSimpleA::Nittsu => FlowA {
            index: "https://www.nittsu.co.jp/support/words/",
            base: "https://www.nittsu.co.jp",
            link_link_selector: "ul.clm4:nth-child(2) > li > a",
            link_selector: "ul.clm2 > li > a",
            title_selector: ".h1Design2",
            body_selector: ".section",
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
        SiteKindSimpleA::Nrisecure => FlowA {
            link_links: vec![String::from("https://www.nri-secure.co.jp/glossary")],
            link_selector: "div.glossary-post > ul> li > a",
            title_selector: "#hs_cos_wrapper_name",
            body_selector: "#hs_cos_wrapper_post_body",
            ..Default::default()
        },
        SiteKindSimpleA::Pfa => FlowA {
            index: "https://www.pfa.or.jp/yogoshu/",
            link_link_selector: ".colLeft > div > ul > li > a, .colRight > div > ul > li > a",
            link_selector: "div.colLeft > div > div > div > p  a",
            title_selector: ".textHeader",
            body_selector: ".textIndent",
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
        SiteKindSimpleA::Rewords => FlowA {
            index: "https://www.re-words.net/japan/",
            base: "https://www.re-words.net/japan/",
            link_link_selector: "ul.colspan > li > a, ul.colspan2 > li > a",
            link_selector: "#list > ul:nth-child(1) > li > a",
            title_selector: "#ue > div:nth-child(1) > h2:nth-child(1)",
            body_selector: "div.contents:nth-child(3)",
            ..Default::default()
        },
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
        SiteKindSimpleA::Smtrc => FlowA {
            index: "https://smtrc.jp/useful/glossary/",
            base: "https://smtrc.jp",
            link_link_selector: ".table_wrap_50on > ul > li > a, .table_wrap_alphabet > ul > li > a",
            link_selector: "#list > ul> li > a",
            title_selector: ".title",
            body_selector: ".text",
            ..Default::default()
        },
        SiteKindSimpleA::Sobien => FlowA {
            index: "http://www.so-bien.com/kimono/",
            link_link_selector: "div.widget-tag-cloud:nth-child(3) > table:nth-child(2) > tbody:nth-child(1) > tr > td > a",
            link_selector: ".asset-body > div:nth-child(1) > ul > li > a",
            title_selector: "#page-title",
            body_selector: ".asset-body > div:nth-child(1) > p",
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
        SiteKindSimpleA::Sompocybersecurity => FlowA {
            link_links: vec![String::from("https://www.sompocybersecurity.com/glossary.html")],
            link_selector: "div.glossary-list-wrap > div > div > div > div > div > div > h3 > a",
            title_selector: ".title-blog",
            body_selector: "#main > p",
            ..Default::default()
        },
        SiteKindSimpleA::Suumo => FlowA {
            index: "https://suumo.jp/yougo/",
            base: "https://suumo.jp",
            link_link_selector: "ul.syllabary-list > li:nth-child(1) > a",
            link_selector: "div.ui-section_h3 > div > div > ul > li > div > a",
            title_selector: ".ui-section--h1 > div > h1",
            body_selector: ".pagecaption-txt",
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
    Ri,
    Meiwakaiun,
    Theglenlivet,
}

fn simple_b(kind: &SiteKindSimpleB) -> FlowB {
    match kind {
        SiteKindSimpleB::Ri => FlowB {
                index: "https://r-i.jp/glossary/",
                base: "https://r-i.jp",
                link_selector: "ul.tabInitial:nth-child(1) > li > a",
                titles_selector:".glossary_entry_title",
                bodies_selector: ".glossary_entry_body",
                ..Default::default()
        },
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
