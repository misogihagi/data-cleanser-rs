use std::vec;

use super::interface::WorkFlowTrait;
use crate::utils::{Flow, HierarchicalFlow, PageLinkFlow, SinglepageFlow, Term};

pub enum SiteKindSimple {
    A(SiteKindSimpleA),
    B(SiteKindSimpleB),
    C(SiteKindSimpleC),
}

pub struct SimpleWorkFlow {
    pub kind: SiteKindSimple,
}
impl SimpleWorkFlow {
    pub fn new(kind_str: &'static str) -> SimpleWorkFlow {
        SimpleWorkFlow {
            kind: SimpleWorkFlow::my_kind(kind_str).unwrap(),
        }
    }
    pub fn my_kind(kind_str: &'static str) -> Option<SiteKindSimple> {
        match kind_str {
            "aritayaki" => Some(SiteKindSimple::A(SiteKindSimpleA::Aritayaki)),
            "athome" => Some(SiteKindSimple::A(SiteKindSimpleA::Athome)),
            "beer" => Some(SiteKindSimple::A(SiteKindSimpleA::Beer)),
            "chemicoat" => Some(SiteKindSimple::A(SiteKindSimpleA::Chemicoat)),
            "chintai" => Some(SiteKindSimple::A(SiteKindSimpleA::Chintai)),
            "civileng" => Some(SiteKindSimple::B(SiteKindSimpleB::Civileng)),
            "cybernet" => Some(SiteKindSimple::A(SiteKindSimpleA::Cybernet)),
            "ena" => Some(SiteKindSimple::A(SiteKindSimpleA::Ena)),
            "esp" => Some(SiteKindSimple::A(SiteKindSimpleA::ESP)),
            "fastretailing" => Some(SiteKindSimple::A(SiteKindSimpleA::Fastretailing)),
            "felissimo" => Some(SiteKindSimple::A(SiteKindSimpleA::Felissimo)),
            "fooddies" => Some(SiteKindSimple::C(SiteKindSimpleC::Fooddies)),
            "footballcottage" => Some(SiteKindSimple::A(SiteKindSimpleA::Footballcottage)),
            "footballzone" => Some(SiteKindSimple::A(SiteKindSimpleA::Footballzone)),
            "fukuwatanabe" => Some(SiteKindSimple::A(SiteKindSimpleA::Fukuwatanabe)),
            "globis" => Some(SiteKindSimple::C(SiteKindSimpleC::Globis)),
            "goonet" => Some(SiteKindSimple::A(SiteKindSimpleA::Goonet)),
            "gurubi" => Some(SiteKindSimple::A(SiteKindSimpleA::Gurubi)),
            "jhs" => Some(SiteKindSimple::A(SiteKindSimpleA::JHS)),
            "jmac" => Some(SiteKindSimple::A(SiteKindSimpleA::JMAC)),
            "kabuwatanabe" => Some(SiteKindSimple::A(SiteKindSimpleA::Kabuwatanabe)),
            "jfa" => Some(SiteKindSimple::B(SiteKindSimpleB::Jfa)),
            "jiki" => Some(SiteKindSimple::B(SiteKindSimpleB::Jiki)),
            "kenchikuyogo" => Some(SiteKindSimple::A(SiteKindSimpleA::Kenchikuyogo)),
            "kddi" => Some(SiteKindSimple::A(SiteKindSimpleA::Kddi)),
            "kuraemon" => Some(SiteKindSimple::A(SiteKindSimpleA::Kuraemon)),
            "kyokutok" => Some(SiteKindSimple::A(SiteKindSimpleA::Kyokutok)),
            "konest" => Some(SiteKindSimple::C(SiteKindSimpleC::Konest)),
            "livable" => Some(SiteKindSimple::A(SiteKindSimpleA::Livable)),
            "macromill" => Some(SiteKindSimple::A(SiteKindSimpleA::Macromill)),
            "meiwakaiun" => Some(SiteKindSimple::B(SiteKindSimpleB::Meiwakaiun)),
            "mintetsu" => Some(SiteKindSimple::A(SiteKindSimpleA::Mintetsu)),
            "mizuho" => Some(SiteKindSimple::A(SiteKindSimpleA::Mizuho)),
            "naigai" => Some(SiteKindSimple::A(SiteKindSimpleA::Naigai)),
            "nichiren" => Some(SiteKindSimple::A(SiteKindSimpleA::Nichiren)),
            "nisso" => Some(SiteKindSimple::B(SiteKindSimpleB::Nisso)),
            "nittsu" => Some(SiteKindSimple::A(SiteKindSimpleA::Nittsu)),
            "nomura" => Some(SiteKindSimple::A(SiteKindSimpleA::Nomura)),
            "nrisecure" => Some(SiteKindSimple::A(SiteKindSimpleA::Nrisecure)),
            "pfa" => Some(SiteKindSimple::A(SiteKindSimpleA::Pfa)),
            "rewords" => Some(SiteKindSimple::A(SiteKindSimpleA::Rewords)),
            "sakaiku" => Some(SiteKindSimple::A(SiteKindSimpleA::Sakaiku)),
            "ri" => Some(SiteKindSimple::B(SiteKindSimpleB::Ri)),
            "ryugaku" => Some(SiteKindSimple::A(SiteKindSimpleA::Ryugaku)),
            "sumai1" => Some(SiteKindSimple::A(SiteKindSimpleA::Sumai1)),
            "smbcnikko" => Some(SiteKindSimple::A(SiteKindSimpleA::Smbcnikko)),
            "smtrc" => Some(SiteKindSimple::A(SiteKindSimpleA::Smtrc)),
            "sobien" => Some(SiteKindSimple::A(SiteKindSimpleA::Sobien)),
            "soccer" => Some(SiteKindSimple::A(SiteKindSimpleA::Soccer)),
            "sompocybersecurity" => Some(SiteKindSimple::A(SiteKindSimpleA::Sompocybersecurity)),
            "suumo" => Some(SiteKindSimple::A(SiteKindSimpleA::Suumo)),
            "sufu" => Some(SiteKindSimple::A(SiteKindSimpleA::Sufu)),
            "theglenlivet" => Some(SiteKindSimple::B(SiteKindSimpleB::Theglenlivet)),
            "unew" => Some(SiteKindSimple::A(SiteKindSimpleA::Unew)),
            "universalooh" => Some(SiteKindSimple::B(SiteKindSimpleB::UniversalOOH)),
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
        SiteKindSimple::C(k) => simple_c(k).get_terms().await,
    }
}

pub enum SiteKindSimpleA {
    Aritayaki,
    Athome,
    Beer,
    Chemicoat,
    Chintai,
    Cybernet,
    Ena,
    ESP,
    Fastretailing,
    Felissimo,
    Footballcottage,
    Footballzone,
    Fukuwatanabe,
    Goonet,
    Gurubi,
    JHS,
    JMAC,
    Kabuwatanabe,
    Kenchikuyogo,
    Kddi,
    Kuraemon,
    Kyokutok,
    Livable,
    Macromill,
    Mintetsu,
    Mizuho,
    Nittsu,
    Naigai,
    Nomura,
    Nrisecure,
    Pfa,
    Rewords,
    Ryugaku,
    Sakaiku,
    Sumai1,
    Smbcnikko,
    Smtrc,
    Sobien,
    Soccer,
    Sompocybersecurity,
    Suumo,
    WaferMeasurementInspection,
    Webtan,
    Nichiren,
    Sufu,
    Unew,
}

fn simple_a(kind: &SiteKindSimpleA) -> HierarchicalFlow<'static> {
    match kind {
        SiteKindSimpleA::Aritayaki => HierarchicalFlow {
            level2_links: (1..8).map(|i| "http://www.aritayaki-fun.com/?cat=7&paged=".to_string()+&i.to_string()).collect(),
            level1_selector: ".main-conts > article > div > div > p > a",
            title_selector: "h1.section-title",
            body_selector: ".article-body > p:nth-child(1)",
            pool_size:40,
            rest:10,
            ..Default::default()
        },
        SiteKindSimpleA::Athome => HierarchicalFlow {
            index: "https://www.athome.co.jp/contents/words/",
            base: "https://www.athome.co.jp",
            level2_selector: ".f_l  li a,.f_r li a,.wbox:nth-child(2) li a",
            level1_selector: "ul.cf > li > a",
            title_selector: "#h1_title > h1",
            body_selector: ".textarea",
            ..Default::default()
        },
        SiteKindSimpleA::Beer => HierarchicalFlow {
            level2_links: vec![String::from("https://craft-beer.life/dictionary")],
            level1_selector: "li.dictionary_section > ol > li > a",
            title_selector: ".article_headline",
            body_selector: ".article_body > p",
            ..Default::default()
        },
        SiteKindSimpleA::Chemicoat => HierarchicalFlow {
            level2_links: vec![String::from("https://www.chemicoat.co.jp/knowledge/")],
            level1_selector: ".content-list > ul > li > a",
            title_selector: ".columnh2",
            body_selector: ".columntext",
            ..Default::default()
        },
        SiteKindSimpleA::Chintai => HierarchicalFlow {
            index: "https://chintai-keiei.com/dictionary/",
            level2_selector: "ul.dic_index > li > a",
            level1_selector: ".ico_list > li > a",
            level1_base: "https://chintai-keiei.com/dictionary/a/",
            title_selector: ".blog_title",
            body_selector: "#main > p:nth-child(4)",
            encoding: "euc-jp",
            pool_size:40,
            rest:30,
            ..Default::default()
        },
        SiteKindSimpleA::Cybernet => HierarchicalFlow {
            level2_links: vec!["https://www.cybernet.co.jp/optical/glossary/".to_string()], 
            base: "https://www.cybernet.co.jp",
            level1_selector: "div.u-mb-40 > div > div > ul > li > a",
            title_selector: "#title01",
            body_selector: "div.u-mb-40:not(.c-link-block), div.c-image-block-row",
            ..Default::default()
        },
        SiteKindSimpleA::Ena => HierarchicalFlow {
            level2_links: vec!["https://www.ena.travel/glossary/all/".to_string()],
            base: "https://www.ena.travel",
            level1_selector: "div.body > dl > dd:nth-child(2) > ul:nth-child(1) > li > a",
            title_selector: "div.guide > h2 > span > span",
            body_selector: "div.text > p",
            ..Default::default()
        },
        SiteKindSimpleA::ESP => HierarchicalFlow {
            index: "https://www.esp.ac.jp/epv/glossary/index.html",
            base: "https://www.esp.ac.jp/epv/glossary/",
            level2_selector: "#glossary-navi > li > a",
            level1_selector: "#glossary-list > li > a",
            title_selector: "#glossary-name",
            body_selector: "#glossary-text",
            ..Default::default()
        },
        SiteKindSimpleA::Fastretailing => HierarchicalFlow {
            index: "https://www.fastretailing.com/jp/glossary",
            base: "https://www.fastretailing.com",
            level2_selector:".menuli > li > a",
            level1_selector: "#alphabet-index > dd > ul > li > a",
            title_selector: "#content > h1:nth-child(3)",
            body_selector: "#entry-535",
            image_selector: Some("#entry-535 img"),
            ..Default::default()
        },
        SiteKindSimpleA::Felissimo => HierarchicalFlow {
            level2_links: vec![String::from("https://www.felissimo.co.jp/niau/words/")],
            level1_selector: "div.words-tabContents:nth-child(3) > div:nth-child(2) > div > div > ul > li > a",
            title_selector: ".cmn-pageTitle_main > span:nth-child(1)",
            body_selector: ".single-wordsArticleBody_contents > p",
            ..Default::default()
        },
        SiteKindSimpleA::Footballcottage => HierarchicalFlow {
            level2_links: vec![String::from("https://footballcottage.com/article/soccer_words/")],
            level1_selector: "#nocopy > div.entry-content > ul > li > a",
            title_selector: "#nocopy > div.entry-content > h1",
            body_selector: "#nocopy > div.entry-content > p",
            ..Default::default()
        },
        SiteKindSimpleA::Footballzone => HierarchicalFlow {
            level2_links: vec![String::from("https://www.football-zone.net/archives/411025")],
            level1_selector: "#content > section.detail > div.paragraph > p:not(:nth-child(3)) > a",
            title_selector: "#content > section.detail > div.paragraph > p:nth-child(2) > strong",
            body_selector: "#content > section.detail > div.paragraph > p:nth-child(2)",
            ..Default::default()
        },
        SiteKindSimpleA::Fukuwatanabe => HierarchicalFlow {
            index: "https://fuku-watanabe.com/ec/glossary/tunnel/01a/index.html",
            level2_selector:"div.box-tags-tunnel > ul.clearfix > li > a",
            level1_selector: "center > table:nth-child(1) > tbody:nth-child(1) > tr > td:nth-child(1) > a",
            title_selector: ".button_tunnel_midasi",
            body_selector: ".col-md-8 > div:nth-child(1) > center:nth-child(3) > table:nth-child(1) > tbody:nth-child(1)",
            ..Default::default()
        },
        SiteKindSimpleA::Goonet => HierarchicalFlow {
            index: "https://www.goo-net.com/knowledge/",
            base: "https://www.goo-net.com",
            level2_selector: "#main > section > div > dl > dd:nth-child(2) > ul > li > a",
            level1_selector: ".column2 > li > a",
            title_selector: ".h3box > h5",
            body_selector: ".text",
            encoding: "euc-jp",
            ..Default::default()
        },
        SiteKindSimpleA::Gurubi => HierarchicalFlow {
            level2_links: vec![String::from("https://gurubi.ac.jp/glossary/")],
            base: "https://gurubi.ac.jp/glossary/",
            level1_selector: "div.glossary-list > ul > li > a",
            title_selector: ".yogo > h2",
            body_selector: ".yogo > p",
            ..Default::default()
        },
        SiteKindSimpleA::JHS => HierarchicalFlow {
            index: "https://www.jhs.ac.jp/guide/glossary/",
            level2_selector:".glossary_words > dl > dd > a",            
            level1_selector: ".glossary_category > div > a",
            title_selector: ".title > h1",
            body_selector: ".explain > p",
            ..Default::default()
        },
        SiteKindSimpleA::JMAC => HierarchicalFlow {
            index: "https://www.jmac.co.jp/glossary/",
            base: "https://www.jmac.co.jp",
            level2_selector:"section.l-pageSection:nth-child(4) > ul > li > a, section.l-pageSection:nth-child(5) > ul > li > a, section.l-pageSection:nth-child(6) > ul > li > a",
            level1_selector:".c-glossaryList > li > a",
            title_selector: "h1.c-simpleHeader_title",
            body_selector: ".l-wysiwyg",
            ..Default::default()
        },
        SiteKindSimpleA::Kabuwatanabe => HierarchicalFlow {
            index: "https://kabu-watanabe.com/glossary/tonneru/",
            level2_selector:".all__sidebar-item-post > div:nth-child(1) > table:nth-child(1) > tbody > tr > td > a",
            level1_selector: ".tablemokuj > tbody:nth-child(1) > tr > td:nth-child(1) > a",
            title_selector: "h3.unttonneru_0",
            body_selector: ".blog__details-area-box > table:nth-child(1) > tbody:nth-child(1)",
            ..Default::default()
        },
        SiteKindSimpleA::Kenchikuyogo => HierarchicalFlow {
            index: "https://kenchikuyogo.com/",
            level2_selector:"figure.wp-block-table:nth-child(3) > table:nth-child(1) > tbody:nth-child(1) > tr > td > a",
            level1_selector:".is-style-stripes > table:nth-child(1) > tbody:nth-child(1) > tr > td > a",
            title_selector: "h1.alignwide",
            body_selector: ".entry-content > p:not([class])",
            ..Default::default()
        },
        SiteKindSimpleA::Kddi => HierarchicalFlow {
            index: "https://biz.kddi.com/content/glossary/",
            level2_selector:"#termInitialsJapanese > li > a, #termInitialsAlphabet > li > a",
            level1_selector:"li.biz-p-glossary__ListItem > a",
            title_selector: ".biz-c-glossary__detailHeader__mainText",
            body_selector: "p.biz-c-glossary__detailHeader__text, .biz-c-glossary__detailContent",
            ..Default::default()
        },
        SiteKindSimpleA::Kuraemon => HierarchicalFlow {
            level2_links: ["あ行",
            "か行",
            "さ行",
            "た行",
            "な行",
            "は行",
            "ま行",
            "や行",
            "ら行",
            "わ行+/+記号"].map(|q| "https://www.kuraemon.com/special/dictionary?gyo=".to_string()+q).to_vec(),
            level1_selector:"div.glossary-item > a",
            title_selector: ".term",
            body_selector: ".meaning,.kana",
            ..Default::default()
        },
        SiteKindSimpleA::Kyokutok => HierarchicalFlow {
            index: "https://www.kyokuto-k.co.jp/glossary/",
            level2_selector:".blog-in > div:nth-child(2) > table:nth-child(1) > tbody:nth-child(1) > tr > td > a",
            level1_selector:".col-md-7 > div:nth-child(1) > center:nth-child(2) > table:nth-child(1) > tbody:nth-child(1) > tr > td > a",
            title_selector: ".button12_000",
            body_selector: ".button12_002",
            ..Default::default()
        },
        SiteKindSimpleA::Livable => HierarchicalFlow {
            level2_links:vec![String::from("https://www.livable.co.jp/yogo/list/")],
            level1_selector:"section.s-content__box> ul > li > a",
            title_selector: ".a-headline",
            body_selector: ".s-content__detail > p",
            ..Default::default()
        },
        SiteKindSimpleA::Macromill => HierarchicalFlow {
            index: "https://www.macromill.com/service/words/",
            level2_selector: ".indexPc > ul > li > a",
            level1_selector: ".main > .posts > .pnl > a",
            title_selector: ".head > div > h1",
            body_selector: ".un_secBlock:not(.lo_mgnTopL):not(.lo_mgnTopM),.hp_mgnTopM",
            ..Default::default()
        },
        SiteKindSimpleA::Mintetsu => HierarchicalFlow {
            index: "https://www.mintetsu.or.jp/knowledge/",
            level2_base: "https://www.mintetsu.or.jp",
            level2_selector: ".ContentsList01 > ul:nth-child(1) > li:nth-child(2) > ul:nth-child(2) > li  > a",
            level1_selector: "ul.wordList > li > a",
            title_selector: "h1",
            body_selector: ".section.clearfix",
            ..Default::default()
        },
        SiteKindSimpleA::Mizuho => HierarchicalFlow {
            index: "https://www.mizuho-re.co.jp/knowledge/dictionary/",
            base: "https://www.mizuho-re.co.jp",
            level2_selector: "ul.colspan > li > a, ul.colspan2 > a",
            level1_selector: "#list > ul > li > a",
            title_selector: "#select_word",
            body_selector: "#ue > div:nth-child(1) > div:nth-child(2)",
            ..Default::default()
        },
        SiteKindSimpleA::Naigai => HierarchicalFlow {
            index: "https://www.ntl-naigai.co.jp/glossary/",
            base: "https://www.ntl-naigai.co.jp",
            level2_selector: "li.-nt-naviAnchorFrame__item > a",
            level1_selector: "li.-nt-glossaryList__item > a",
            title_selector: "h2.-nt-title6",
            body_selector: ".-nt-note",
            ..Default::default()
        },
        SiteKindSimpleA::Nichiren => HierarchicalFlow {
            level2_links: vec![String::from("https://www.nichiren.or.jp/glossary/")],
            level1_selector: ".glossary-table01 > tbody:nth-child(1) > tr > td > a",
            title_selector: ".glossary-post .head .title",
            body_selector: ".glossary-post .head .ruby, .glossary-post .body",
           pool_size:10,
            rest:10,
             ..Default::default()
        },
        SiteKindSimpleA::Nittsu => HierarchicalFlow {
            index: "https://www.nittsu.co.jp/support/words/",
            base: "https://www.nittsu.co.jp",
            level2_selector: "ul.clm4:nth-child(2) > li > a",
            level1_selector: "ul.clm2 > li > a",
            title_selector: ".h1Design2",
            body_selector: ".section",
            ..Default::default()
        },
        SiteKindSimpleA::Nomura => HierarchicalFlow {
            index: "https://www.nomura.co.jp/terms/",
            base: "https://www.nomura.co.jp",
            level2_selector: ".tbl > tbody:nth-child(1) > tr > td > p > a",
            level1_selector: ".-transform > li > a",
            title_selector: "#term_id",
            body_selector: ".glossary-block",
            ..Default::default()
        },
        SiteKindSimpleA::Nrisecure => HierarchicalFlow {
            level2_links: vec![String::from("https://www.nri-secure.co.jp/glossary")],
            level1_selector: "div.glossary-post > ul> li > a",
            title_selector: "#hs_cos_wrapper_name",
            body_selector: "#hs_cos_wrapper_post_body",
            ..Default::default()
        },
        SiteKindSimpleA::Pfa => HierarchicalFlow {
            index: "https://www.pfa.or.jp/yogoshu/",
            level2_selector: ".colLeft > div > ul > li > a, .colRight > div > ul > li > a",
            level1_selector: "div.colLeft > div > div > div > p  a",
            title_selector: ".textHeader",
            body_selector: ".textIndent",
            ..Default::default()
        },
        SiteKindSimpleA::Rewords => HierarchicalFlow {
            index: "https://www.re-words.net/japan/",
            base: "https://www.re-words.net/japan/",
            level2_selector: "ul.colspan > li > a, ul.colspan2 > li > a",
            level1_selector: "#list > ul:nth-child(1) > li > a",
            title_selector: "#ue > div:nth-child(1) > h2:nth-child(1)",
            body_selector: "div.contents:nth-child(3)",
            ..Default::default()
        },
        SiteKindSimpleA::Sakaiku => HierarchicalFlow {
            index: "https://www.sakaiku.jp/words/",
            level2_selector: "#main > div:nth-child(3) > ul > li > a",
            level1_selector: "#main > dl > dt > a",
            title_selector: "#main > div.explanation-article > h2",
            body_selector: "#main > div.explanation-article",
            ..Default::default()
        },
        SiteKindSimpleA::Ryugaku => HierarchicalFlow {
            index: "https://ryugaku.kuraveil.jp/dictionaries",
            base: "https://ryugaku.kuraveil.jp",
            level2_selector: "div.initial-index:nth-child(4) > a",
            level1_selector: ".word-list > li > a",
            title_selector: ".header-title",
            body_selector: ".markdown",
            ..Default::default()
        },
        SiteKindSimpleA::Sumai1 => HierarchicalFlow {
            index: "https://www.sumai1.com/useful/words/",
            base: "https://www.sumai1.com",
            level2_selector: ".innerbody > div > div > ul > li > a, .index-alphabet > div > div:nth-child(1) > ul > li:nth-child(1) > a , div.tb-row:nth-child(3) > div:nth-child(2) > ul:nth-child(1) > li:nth-child(1) > a, .index-number > div:nth-child(1) > div > ul > li:nth-child(2) > a",
            level1_selector: "ul.col2:nth-child(1) > li > a",
            title_selector: ".images > h1",
            body_selector: ".description",
            ..Default::default()
        },
        SiteKindSimpleA::Smbcnikko => HierarchicalFlow {
            index: "https://www.smbcnikko.co.jp/terms/index.html",
            base: "https://www.smbcnikko.co.jp",
            level2_selector: ".A > li > a, .B > li > a, #anchor02 > li > a",
            level1_selector: ".box-release-inner > ul > li > a",
            title_selector: "#main > section > section:nth-child(2) > h2 > span",
            body_selector: "#main > section:nth-child(1) > section:nth-child(2) > div:nth-child(2) > p:nth-child(1), p.box-img:nth-child(2), #main > section:nth-child(1) > section:nth-child(2) > div:nth-child(2) > section:nth-child(3)",
            encoding: "shift-jis",
            pool_size:100,
            rest:60,
            ..Default::default()
        },
        SiteKindSimpleA::Smtrc => HierarchicalFlow {
            index: "https://smtrc.jp/useful/glossary/",
            base: "https://smtrc.jp",
            level2_selector: ".table_wrap_50on > ul > li > a, .table_wrap_alphabet > ul > li > a",
            level1_selector: "#list > ul> li > a",
            title_selector: ".title",
            body_selector: ".text",
            ..Default::default()
        },
        SiteKindSimpleA::Sobien => HierarchicalFlow {
            index: "http://www.so-bien.com/kimono/",
            level2_selector: "div.widget-tag-cloud:nth-child(3) > table:nth-child(2) > tbody:nth-child(1) > tr > td > a",
            level1_selector: ".asset-body > div:nth-child(1) > ul > li > a",
            title_selector: "#page-title",
            body_selector: ".asset-body > div:nth-child(1) > p",
            ..Default::default()
        },
        SiteKindSimpleA::Soccer => HierarchicalFlow {
            index: "https://www.homemate-research-soccer.com/useful/glossary/soccer/",
            base: "https://www.homemate-research-soccer.com",
            level1_selector: "section.sec_cmn:nth-child(4) > div > ul > li > a",
            title_selector: ".post_btn > h1:nth-child(1)",
            body_selector: ".post_box",
            image_selector: Some(".post_box_img > img:nth-child(1)"),
            ..Default::default()
        },
        SiteKindSimpleA::Sompocybersecurity => HierarchicalFlow {
            level2_links: vec![String::from("https://www.sompocybersecurity.com/glossary.html")],
            level1_selector: "div.glossary-list-wrap > div > div > div > div > div > div > h3 > a",
            title_selector: ".title-blog",
            body_selector: "#main > p",
            ..Default::default()
        },
        SiteKindSimpleA::Sufu => HierarchicalFlow {
            level2_links: vec![String::from("https://sufu.lifull.net/category/glossary/6")],
            level1_selector: "div.container.Glossary_index.is_wide > div > div.glossary_list > div > div > div > a",
            title_selector: "div.container.Glossary_detail.is_wide > div > div.main_contents > div.glossary_list > h1",
            body_selector: "div.container.Glossary_detail.is_wide > div > div.main_contents > div.glossary_list > section:nth-child(2) > p",
            ..Default::default()
        },
        SiteKindSimpleA::Suumo => HierarchicalFlow {
            index: "https://suumo.jp/yougo/",
            base: "https://suumo.jp",
            level2_selector: "ul.syllabary-list > li:nth-child(1) > a",
            level1_selector: "div.ui-section_h3 > div > div > ul > li > div > a",
            title_selector: ".ui-section--h1 > div > h1",
            body_selector: ".pagecaption-txt",
            ..Default::default()
        },
        SiteKindSimpleA::Unew => HierarchicalFlow {
            index: "http://www.u-new.com/word/",
            level2_selector: "table.wording_list_table01 > tbody > tr > td > a, div.width_700 > table > tbody > tr> td > a",
            level1_selector: "div.text_04 > a",
            title_selector: ".obi_02 > h2",
            body_selector: ".text_01, .text_02",
            ..Default::default()
        },
        SiteKindSimpleA::WaferMeasurementInspection => HierarchicalFlow {
            level2_links: vec![String::from("https://www.wafer-measurement-inspection.com/words/")],
            base: "https://www.wafer-measurement-inspection.com/words/",
            level1_selector: ".newslist > li > a",
            title_selector: ".ts3",
            body_selector: ".longComment",
            ..Default::default()
        },
        SiteKindSimpleA::Webtan => HierarchicalFlow {
            index: "https://webtan.impress.co.jp/glossary/list/1",
            base: "https://webtan.impress.co.jp",
            level1_selector: ".node > div:nth-child(1) > ul > li > a",
            title_selector: "h1.title",
            body_selector: ".glossary_description",
            ..Default::default()
        },
    }
}

pub enum SiteKindSimpleB {
    Civileng,
    Jiki,
    Ri,
    Meiwakaiun,
    Nisso,
    Theglenlivet,
    UniversalOOH,
    Jfa,
}

fn simple_b(kind: &SiteKindSimpleB) -> SinglepageFlow {
    match kind {
        SiteKindSimpleB::Civileng => SinglepageFlow {
                links: ["a","ka","sa","ta","na","ha","ma","ya","ra","wa"].map(|s| "http://civileng.ec-net.jp/yougo/".to_string()+s+".htm").to_vec(),
                titles_selector:"table > tbody > tr > td:nth-child(2)",
                bodies_selector: "table > tbody > tr > td:nth-child(3)",
                encoding: "shift-jis",
                ..Default::default()
        },
        SiteKindSimpleB::Jfa => SinglepageFlow {
            index: "https://www.jfa.jp/laws/soccer/glossary/",
            titles_selector: "#main-colum > div:nth-child(3) > h5, #main-colum > div:nth-child(3) > h4:nth-child(187)",
            bodies_selector: "#main-colum > div:nth-child(3) > p:not(:last-child)",
            ..Default::default()
        },
        SiteKindSimpleB::Jiki => SinglepageFlow {
                index: "https://www.jiki.jp/words/",
                level1_selector: "div.entry > table > tbody:nth-child(1) > tr > td > a",
                titles_selector:".entry > h3",
                bodies_selector: ".entry > p",
                ..Default::default()
        },
        SiteKindSimpleB::Nisso => SinglepageFlow {
                links: vec!["https://www.nisso-sangyo.co.jp/glossary".to_string()],
                titles_selector:"section.dic-index__section > div > div > h3",
                bodies_selector: "section.dic-index__section > div > div.dic-item__body",
                ..Default::default()
        },
        SiteKindSimpleB::Ri => SinglepageFlow {
                index: "https://r-i.jp/glossary/",
                base: "https://r-i.jp",
                level1_selector: "ul.tabInitial:nth-child(1) > li > a",
                titles_selector:".glossary_entry_title",
                bodies_selector: ".glossary_entry_body",
                ..Default::default()
        },
        SiteKindSimpleB::Meiwakaiun => SinglepageFlow {
            index: "https://www.meiwakaiun.com/meiwalabo/yougo/",
            // section.grossary typo!
            titles_selector: "main > div.page-box > div.container > div.glossary-details > section.grossary-details-box > table > tbody > tr > td > div.glossary-details > section.grossary-details-box > table > tbody > tr:not(#ki02a) > th",
            bodies_selector: "main > div.page-box > div.container > div.glossary-details > section.grossary-details-box > table > tbody > tr > td > div.glossary-details > section.grossary-details-box > table > tbody > tr:not(#ki02a) > td",
            ..Default::default()
        },
        SiteKindSimpleB::Theglenlivet => SinglepageFlow {
            index: "https://www.theglenlivet.jp/craft/whisky-words.html",
            titles_selector: "section.producttext > div > h3",
            bodies_selector: "section.producttext > div > h3 + p",
            ..Default::default()
        },
        SiteKindSimpleB::UniversalOOH => SinglepageFlow {
                index: "https://universal-ooh.jeki.co.jp/course_ooh/%e5%9f%ba%e6%9c%ac%e7%94%a8%e8%aa%9e%e3%81%ae%e8%a7%a3%e8%aa%ac/",
                level1_selector: "ul.is-open > li > a",
                titles_selector:".c-chapter__ttl",
                bodies_selector: ".c-chapter__txt",
                ..Default::default()
        },
    }
}

pub enum SiteKindSimpleC {
    Fooddies,
    Globis,
    Konest,
}

fn simple_c(kind: &SiteKindSimpleC) -> PageLinkFlow {
    match kind {
        SiteKindSimpleC::Fooddies => PageLinkFlow {
            level2_links: vec!["https://fooddies.tokyo/".to_string()],
            level2_selector: ".next",
            level1_selector: "a.entry-card-wrap",
            title_selector: ".entry-title",
            body_selector: "p.whitespace-pre-wrap:nth-child(4), p.whitespace-pre-wrap:nth-child(5), .entry-content > p:nth-child(1), .entry-content > p:nth-child(2), .entry-content > p:nth-child(3), div.group:nth-child(1) > div:nth-child(1) > div:nth-child(1) > div:nth-child(1) > div:nth-child(1) > div:nth-child(1) > p:nth-child(1), .markdown, .entry-content > p:nth-child(6)",
            ..Default::default()
        },
        SiteKindSimpleC::Globis => PageLinkFlow {
            index: "https://mba.globis.ac.jp/about_mba/glossary/",
            level3_selector: ".hiragana > li > a, ul.link_list:nth-child(2) > li:nth-child(1) > a:nth-child(1), .alphabet > li > a",
            level2_selector: ".pagination_anchor_next",
            level1_selector: ".column_main > div > a",
            title_selector: "h1.section_title",
            body_selector: ".cms_section",
            ..Default::default()
        },
        SiteKindSimpleC::Konest => PageLinkFlow {
            index: "https://www.konest.com/contents/todays_korean_list.html",
            base: "https://www.konest.com",
            level1_base: "https://www.konest.com/contents/",
            level2_selector:
                "li.c-pagination__item:nth-last-child(2) > a:nth-child(1):not(.is-disabled)",
            level1_selector: "li.c-card > a:nth-child(1)",
            title_selector: "#korean_title",
            body_selector:
                ".c-hangul__content-main--translate, .c-hangul__content-description--item",
            pool_size: 10,
            rest: 240,
            ..Default::default()
        },
    }
}
