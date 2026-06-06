use super::interface::WorkFlowTrait;
use crate::utils::{HierarchicalFlow, Term};

pub enum SiteKindHierarchical {
    Aritayaki,
    Athome,
    Beer,
    Chemicoat,
    Chintai,
    Cybernet,
    Daiichilife,
    Ebcc,
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
    Restec,
    Rewords,
    Ryugaku,
    Sakaiku,
    Scsk,
    Sumai1,
    Smbcnikko,
    Smtrc,
    Sobien,
    Soccer,
    Sompocybersecurity,
    Suumo,
    WaferMeasurementInspection,
    Webtan,
    Yodosha,
    Nichiren,
    Sufu,
    Unew,
}

pub struct HierarchicalWorkFlow {
    pub kind: SiteKindHierarchical,
}

impl HierarchicalWorkFlow {
    pub fn new(kind: SiteKindHierarchical) -> Self {
        Self { kind }
    }

    pub fn my_kind(kind_str: &'static str) -> Option<SiteKindHierarchical> {
        match kind_str {
            "aritayaki" => Some(SiteKindHierarchical::Aritayaki),
            "athome" => Some(SiteKindHierarchical::Athome),
            "beer" => Some(SiteKindHierarchical::Beer),
            "chemicoat" => Some(SiteKindHierarchical::Chemicoat),
            "chintai" => Some(SiteKindHierarchical::Chintai),
            "cybernet" => Some(SiteKindHierarchical::Cybernet),
            "daiichilife" => Some(SiteKindHierarchical::Daiichilife),
            "ebcc" => Some(SiteKindHierarchical::Ebcc),
            "ena" => Some(SiteKindHierarchical::Ena),
            "esp" => Some(SiteKindHierarchical::ESP),
            "fastretailing" => Some(SiteKindHierarchical::Fastretailing),
            "felissimo" => Some(SiteKindHierarchical::Felissimo),
            "footballcottage" => Some(SiteKindHierarchical::Footballcottage),
            "footballzone" => Some(SiteKindHierarchical::Footballzone),
            "fukuwatanabe" => Some(SiteKindHierarchical::Fukuwatanabe),
            "goonet" => Some(SiteKindHierarchical::Goonet),
            "gurubi" => Some(SiteKindHierarchical::Gurubi),
            "jhs" => Some(SiteKindHierarchical::JHS),
            "jmac" => Some(SiteKindHierarchical::JMAC),
            "kabuwatanabe" => Some(SiteKindHierarchical::Kabuwatanabe),
            "kenchikuyogo" => Some(SiteKindHierarchical::Kenchikuyogo),
            "kuraemon" => Some(SiteKindHierarchical::Kuraemon),
            "kyokutok" => Some(SiteKindHierarchical::Kyokutok),
            "livable" => Some(SiteKindHierarchical::Livable),
            "macromill" => Some(SiteKindHierarchical::Macromill),
            "mintetsu" => Some(SiteKindHierarchical::Mintetsu),
            "mitsujp" => None, // Moved to singlepage
            "mizuho" => Some(SiteKindHierarchical::Mizuho),
            "naigai" => Some(SiteKindHierarchical::Naigai),
            "nichiren" => Some(SiteKindHierarchical::Nichiren),
            "nittsu" => Some(SiteKindHierarchical::Nittsu),
            "nomura" => Some(SiteKindHierarchical::Nomura),
            "nrisecure" => Some(SiteKindHierarchical::Nrisecure),
            "pfa" => Some(SiteKindHierarchical::Pfa),
            "restec" => Some(SiteKindHierarchical::Restec),
            "rewords" => Some(SiteKindHierarchical::Rewords),
            "sakaiku" => Some(SiteKindHierarchical::Sakaiku),
            "ryugaku" => Some(SiteKindHierarchical::Ryugaku),
            "scsk" => Some(SiteKindHierarchical::Scsk),
            "sumai1" => Some(SiteKindHierarchical::Sumai1),
            "smbcnikko" => Some(SiteKindHierarchical::Smbcnikko),
            "smtrc" => Some(SiteKindHierarchical::Smtrc),
            "sobien" => Some(SiteKindHierarchical::Sobien),
            "soccer" => Some(SiteKindHierarchical::Soccer),
            "sompocybersecurity" => Some(SiteKindHierarchical::Sompocybersecurity),
            "suumo" => Some(SiteKindHierarchical::Suumo),
            "sufu" => Some(SiteKindHierarchical::Sufu),
            "unew" => Some(SiteKindHierarchical::Unew),
            "wafermeasurementinspection" => Some(SiteKindHierarchical::WaferMeasurementInspection),
            "webtan" => Some(SiteKindHierarchical::Webtan),
            "yodosha" => Some(SiteKindHierarchical::Yodosha),
            _ => None,
        }
    }
}

impl WorkFlowTrait for HierarchicalWorkFlow {
    fn is_my_kind(kind_str: &'static str) -> bool {
        Self::my_kind(kind_str).is_some()
    }

    async fn get_terms(&self) -> Vec<Term> {
        use crate::utils::Flow;
        self.get_flow().get_terms().await
    }
}

impl HierarchicalWorkFlow {
    fn get_flow(&self) -> HierarchicalFlow<'static> {
        match self.kind {
            SiteKindHierarchical::Aritayaki => HierarchicalFlow {
                level2_links: (1..8).map(|i| "http://www.aritayaki-fun.com/?cat=7&paged=".to_string()+&i.to_string()).collect(),
                level1_selector: ".main-conts > article > div > div > p > a",
                title_selector: "h1.section-title",
                body_selector: ".article-body > p:nth-child(1)",
                pool_size:40,
                rest:10,
                ..Default::default()
            },
            SiteKindHierarchical::Athome => HierarchicalFlow {
                index: "https://www.athome.co.jp/contents/words/",
                base: "https://www.athome.co.jp",
                level2_selector: ".f_l  li a,.f_r li a,.wbox:nth-child(2) li a",
                level1_selector: "ul.cf > li > a",
                title_selector: "#h1_title > h1",
                body_selector: ".textarea",
                ..Default::default()
            },
            SiteKindHierarchical::Beer => HierarchicalFlow {
                level2_links: vec![String::from("https://craft-beer.life/dictionary")],
                level1_selector: "li.dictionary_section > ol > li > a",
                title_selector: ".article_headline",
                body_selector: ".article_body > p",
                ..Default::default()
            },
            SiteKindHierarchical::Chemicoat => HierarchicalFlow {
                level2_links: vec![String::from("https://www.chemicoat.co.jp/knowledge/")],
                level1_selector: ".content-list > ul > li > a",
                title_selector: ".columnh2",
                body_selector: ".columntext",
                ..Default::default()
            },
            SiteKindHierarchical::Chintai => HierarchicalFlow {
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
            SiteKindHierarchical::Cybernet => HierarchicalFlow {
                level2_links: vec!["https://www.cybernet.co.jp/optical/glossary/".to_string()], 
                base: "https://www.cybernet.co.jp",
                level1_selector: "div.u-mb-40 > div > div > ul > li > a",
                title_selector: "#title01",
                body_selector: "div.u-mb-40:not(.c-link-block), div.c-image-block-row",
                ..Default::default()
            },
            SiteKindHierarchical::Daiichilife => HierarchicalFlow {
                index: "https://www.dai-ichi-life.co.jp/support/glossary/initial/",
                level2_selector: "div > dl > dd > a, .key > ul> li > a",
                level1_selector: ".textcolumnLink2 > li > a",
                title_selector: ".titleH3",
                body_selector: ".fs14",
                ..Default::default()
            },
            SiteKindHierarchical::Ebcc => HierarchicalFlow {
                index: "https://www.e-bcc.co.jp/glossary/outsourcing/?category=all",
                level2_selector: "li.cmp-filter__item:nth-of-type(n+3) > a",
                level1_selector: "#content-list > li > div > div > div > a",
                title_selector: ".cmp-title > div:nth-child(1) > h1:nth-child(1)",
                body_selector: "div.cmp.cmp-text > div:nth-child(1)",
                ..Default::default()
            },
            SiteKindHierarchical::Ena => HierarchicalFlow {
                level2_links: vec!["https://www.ena.travel/glossary/all/".to_string()],
                base: "https://www.ena.travel",
                level1_selector: "div.body > dl > dd:nth-child(2) > ul:nth-child(1) > li > a",
                title_selector: "div.guide > h2 > span > span",
                body_selector: "div.text > p",
                ..Default::default()
            },
            SiteKindHierarchical::ESP => HierarchicalFlow {
                index: "https://www.esp.ac.jp/epv/glossary/index.html",
                base: "https://www.esp.ac.jp/epv/glossary/",
                level2_selector: "#glossary-navi > li > a",
                level1_selector: "#glossary-list > li > a",
                title_selector: "#glossary-name",
                body_selector: "#glossary-text",
                ..Default::default()
            },
            SiteKindHierarchical::Fastretailing => HierarchicalFlow {
                index: "https://www.fastretailing.com/jp/glossary",
                base: "https://www.fastretailing.com",
                level2_selector:".menuli > li > a",
                level1_selector: "#alphabet-index > dd > ul > li > a",
                title_selector: "#content > h1:nth-child(3)",
                body_selector: "#entry-535",
                image_selector: Some("#entry-535 img"),
                ..Default::default()
            },
            SiteKindHierarchical::Felissimo => HierarchicalFlow {
                level2_links: vec![String::from("https://www.felissimo.co.jp/niau/words/")],
                level1_selector: "div.words-tabContents:nth-child(3) > div:nth-child(2) > div > div > ul > li > a",
                title_selector: ".cmn-pageTitle_main > span:nth-child(1)",
                body_selector: ".single-wordsArticleBody_contents > p",
                ..Default::default()
            },
            SiteKindHierarchical::Footballcottage => HierarchicalFlow {
                level2_links: vec![String::from("https://footballcottage.com/article/soccer_words/")],
                level1_selector: "#nocopy > div.entry-content > ul > li > a",
                title_selector: "#nocopy > div.entry-content > h1",
                body_selector: "#nocopy > div.entry-content > p",
                ..Default::default()
            },
            SiteKindHierarchical::Footballzone => HierarchicalFlow {
                level2_links: vec![String::from("https://www.football-zone.net/archives/411025")],
                level1_selector: "#content > section.detail > div.paragraph > p:not(:nth-child(3)) > a",
                title_selector: "#content > section.detail > div.paragraph > p:nth-child(2) > strong",
                body_selector: "#content > section.detail > div.paragraph > p:nth-child(2)",
                ..Default::default()
            },
            SiteKindHierarchical::Fukuwatanabe => HierarchicalFlow {
                index: "https://fuku-watanabe.com/ec/glossary/tunnel/01a/index.html",
                level2_selector:"div.box-tags-tunnel > ul.clearfix > li > a",
                level1_selector: "center > table:nth-child(1) > tbody:nth-child(1) > tr > td:nth-child(1) > a",
                title_selector: ".button_tunnel_midasi",
                body_selector: ".col-md-8 > div:nth-child(1) > center:nth-child(3) > table:nth-child(1) > tbody:nth-child(1)",
                ..Default::default()
            },
            SiteKindHierarchical::Goonet => HierarchicalFlow {
                index: "https://www.goo-net.com/knowledge/",
                base: "https://www.goo-net.com",
                level2_selector: "#main > section > div > dl > dd:nth-child(2) > ul > li > a",
                level1_selector: ".column2 > li > a",
                title_selector: ".h3box > h5",
                body_selector: ".text",
                encoding: "euc-jp",
                ..Default::default()
            },
            SiteKindHierarchical::Gurubi => HierarchicalFlow {
                level2_links: vec![String::from("https://gurubi.ac.jp/glossary/")],
                base: "https://gurubi.ac.jp/glossary/",
                level1_selector: "div.glossary-list > ul > li > a",
                title_selector: ".yogo > h2",
                body_selector: ".yogo > p",
                ..Default::default()
            },
            SiteKindHierarchical::JHS => HierarchicalFlow {
                index: "https://www.jhs.ac.jp/guide/glossary/",
                level2_selector:".glossary_words > dl > dd > a",            
                level1_selector: ".glossary_category > div > a",
                title_selector: ".title > h1",
                body_selector: ".explain > p",
                ..Default::default()
            },
            SiteKindHierarchical::JMAC => HierarchicalFlow {
                index: "https://www.jmac.co.jp/glossary/",
                base: "https://www.jmac.co.jp",
                level2_selector:"section.l-pageSection:nth-child(4) > ul > li > a, section.l-pageSection:nth-child(5) > ul > li > a, section.l-pageSection:nth-child(6) > ul > li > a",
                level1_selector:".c-glossaryList > li > a",
                title_selector: "h1.c-simpleHeader_title",
                body_selector: ".l-wysiwyg",
                ..Default::default()
            },
            SiteKindHierarchical::Kabuwatanabe => HierarchicalFlow {
                index: "https://kabu-watanabe.com/glossary/tonneru/",
                level2_selector:".all__sidebar-item-post > div:nth-child(1) > table:nth-child(1) > tbody > tr > td > a",
                level1_selector: ".tablemokuj > tbody:nth-child(1) > tr > td:nth-child(1) > a",
                title_selector: "h3.unttonneru_0",
                body_selector: ".blog__details-area-box > table:nth-child(1) > tbody:nth-child(1)",
                ..Default::default()
            },
            SiteKindHierarchical::Kenchikuyogo => HierarchicalFlow {
                index: "https://kenchikuyogo.com/",
                level2_selector:"figure.wp-block-table:nth-child(3) > table:nth-child(1) > tbody:nth-child(1) > tr > td > a",
                level1_selector:".is-style-stripes > table:nth-child(1) > tbody:nth-child(1) > tr > td > a",
                title_selector: "h1.alignwide",
                body_selector: ".entry-content > p:not([class])",
                ..Default::default()
            },
            SiteKindHierarchical::Kuraemon => HierarchicalFlow {
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
            SiteKindHierarchical::Kyokutok => HierarchicalFlow {
                index: "https://www.kyokuto-k.co.jp/glossary/",
                level2_selector:".blog-in > div:nth-child(2) > table:nth-child(1) > tbody:nth-child(1) > tr > td > a",
                level1_selector:".col-md-7 > div:nth-child(1) > center:nth-child(2) > table:nth-child(1) > tbody:nth-child(1) > tr > td > a",
                title_selector: ".button12_000",
                body_selector: ".button12_002",
                ..Default::default()
            },
            SiteKindHierarchical::Livable => HierarchicalFlow {
                level2_links:vec![String::from("https://www.livable.co.jp/yogo/list/")],
                level1_selector:"section.s-content__box> ul > li > a",
                title_selector: ".a-headline",
                body_selector: ".s-content__detail > p",
                ..Default::default()
            },
            SiteKindHierarchical::Macromill => HierarchicalFlow {
                index: "https://www.macromill.com/service/words/",
                level2_selector: ".indexPc > ul > li > a",
                level1_selector: ".main > .posts > .pnl > a",
                title_selector: ".head > div > h1",
                body_selector: ".un_secBlock:not(.lo_mgnTopL):not(.lo_mgnTopM),.hp_mgnTopM",
                ..Default::default()
            },
            SiteKindHierarchical::Mintetsu => HierarchicalFlow {
                index: "https://www.mintetsu.or.jp/knowledge/",
                level2_base: "https://www.mintetsu.or.jp",
                level2_selector: ".ContentsList01 > ul:nth-child(1) > li:nth-child(2) > ul:nth-child(2) > li  > a",
                level1_selector: "ul.wordList > li > a",
                title_selector: "h1",
                body_selector: ".section.clearfix",
                ..Default::default()
            },
            SiteKindHierarchical::Mizuho => HierarchicalFlow {
                index: "https://www.mizuho-re.co.jp/knowledge/dictionary/",
                base: "https://www.mizuho-re.co.jp",
                level2_selector: "ul.colspan > li > a, ul.colspan2 > a",
                level1_selector: "#list > ul > li > a",
                title_selector: "#select_word",
                body_selector: "#ue > div:nth-child(1) > div:nth-child(2)",
                ..Default::default()
            },
            SiteKindHierarchical::Naigai => HierarchicalFlow {
                index: "https://www.ntl-naigai.co.jp/glossary/",
                base: "https://www.ntl-naigai.co.jp",
                level2_selector: "li.-nt-naviAnchorFrame__item > a",
                level1_selector: "li.-nt-glossaryList__item > a",
                title_selector: "h2.-nt-title6",
                body_selector: ".-nt-note",
                ..Default::default()
            },
            SiteKindHierarchical::Nichiren => HierarchicalFlow {
                level2_links: vec![String::from("https://www.nichiren.or.jp/glossary/")],
                level1_selector: ".glossary-table01 > tbody:nth-child(1) > tr > td > a",
                title_selector: ".glossary-post .head .title",
                body_selector: ".glossary-post .head .ruby, .glossary-post .body",
               pool_size:10,
                rest:10,
                 ..Default::default()
            },
            SiteKindHierarchical::Nittsu => HierarchicalFlow {
                index: "https://www.nittsu.co.jp/support/words/",
                base: "https://www.nittsu.co.jp",
                level2_selector: "ul.clm4:nth-child(2) > li > a",
                level1_selector: "ul.clm2 > li > a",
                title_selector: ".h1Design2",
                body_selector: ".section",
                ..Default::default()
            },
            SiteKindHierarchical::Nomura => HierarchicalFlow {
                index: "https://www.nomura.co.jp/terms/",
                base: "https://www.nomura.co.jp",
                level2_selector: ".tbl > tbody:nth-child(1) > tr > td > p > a",
                level1_selector: ".-transform > li > a",
                title_selector: "#term_id",
                body_selector: ".glossary-block",
                ..Default::default()
            },
            SiteKindHierarchical::Nrisecure => HierarchicalFlow {
                level2_links: vec![String::from("https://www.nri-secure.co.jp/glossary")],
                level1_selector: "div.glossary-post > ul> li > a",
                title_selector: "#hs_cos_wrapper_name",
                body_selector: "#hs_cos_wrapper_post_body",
                ..Default::default()
            },
            SiteKindHierarchical::Pfa => HierarchicalFlow {
                index: "https://www.pfa.or.jp/yogoshu/",
                level2_selector: ".colLeft > div > ul > li > a, .colRight > div > ul > li > a",
                level1_selector: "div.colLeft > div > div > div > p  a",
                title_selector: ".textHeader",
                body_selector: ".textIndent",
                ..Default::default()
            },
            SiteKindHierarchical::Restec => HierarchicalFlow {
                index: "https://www.restec.or.jp/glossary/index.html",
                level2_selector: "div.search-initial > ul > li > a:not(.is-disabled)",
                level1_selector: "li.word-item-hiragana > a, li.word-item > a",
                title_selector: ".clearfix > li:nth-child(3)",
                body_selector: ".glossary-detail-info",
                ..Default::default()
            },
            SiteKindHierarchical::Rewords => HierarchicalFlow {
                index: "https://www.re-words.net/japan/",
                base: "https://www.re-words.net/japan/",
                level2_selector: "ul.colspan > li > a, ul.colspan2 > li > a",
                level1_selector: "#list > ul:nth-child(1) > li > a",
                title_selector: "#ue > div:nth-child(1) > h2:nth-child(1)",
                body_selector: "div.contents:nth-child(3)",
                ..Default::default()
            },
            SiteKindHierarchical::Sakaiku => HierarchicalFlow {
                index: "https://www.sakaiku.jp/words/",
                level2_selector: "#main > div:nth-child(3) > ul > li > a",
                level1_selector: "#main > dl > dt > a",
                title_selector: "#main > div.explanation-article > h2",
                body_selector: "#main > div.explanation-article",
                ..Default::default()
            },
            SiteKindHierarchical::Ryugaku => HierarchicalFlow {
                index: "https://ryugaku.kuraveil.jp/dictionaries",
                base: "https://ryugaku.kuraveil.jp",
                level2_selector: "div.initial-index:nth-child(4) > a",
                level1_selector: ".word-list > li > a",
                title_selector: ".header-title",
                body_selector: ".markdown",
                ..Default::default()
            },
            SiteKindHierarchical::Scsk => HierarchicalFlow {
                level2_links: vec![String::from("https://www.scsk.jp/sp/itpnavi/glossary/")],
                level1_selector: "div.listbox_a  a, div.listbox_b a",
                title_selector: "h1.title",
                body_selector: ".lead",
                ..Default::default()
            },
            SiteKindHierarchical::Sumai1 => HierarchicalFlow {
                index: "https://www.sumai1.com/useful/words/",
                base: "https://www.sumai1.com",
                level2_selector: ".innerbody > div > div > ul > li > a, .index-alphabet > div > div:nth-child(1) > ul > li:nth-child(1) > a , div.tb-row:nth-child(3) > div:nth-child(2) > ul:nth-child(1) > li:nth-child(1) > a, .index-number > div:nth-child(1) > div > ul > li:nth-child(2) > a",
                level1_selector: "ul.col2:nth-child(1) > li > a",
                title_selector: ".images > h1",
                body_selector: ".description",
                ..Default::default()
            },
            SiteKindHierarchical::Smbcnikko => HierarchicalFlow {
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
            SiteKindHierarchical::Smtrc => HierarchicalFlow {
                index: "https://smtrc.jp/useful/glossary/",
                base: "https://smtrc.jp",
                level2_selector: ".table_wrap_50on > ul > li > a, .table_wrap_alphabet > ul > li > a",
                level1_selector: "#list > ul> li > a",
                title_selector: ".title",
                body_selector: ".text",
                ..Default::default()
            },
            SiteKindHierarchical::Sobien => HierarchicalFlow {
                index: "http://www.so-bien.com/kimono/",
                level2_selector: "div.widget-tag-cloud:nth-child(3) > table:nth-child(2) > tbody:nth-child(1) > tr > td > a",
                level1_selector: ".asset-body > div:nth-child(1) > ul > li > a",
                title_selector: "#page-title",
                body_selector: ".asset-body > div:nth-child(1) > p",
                pool_size:100,
                rest:60,
                ..Default::default()
            },
            SiteKindHierarchical::Soccer => HierarchicalFlow {
                level2_links: vec![String::from("https://www.homemate-research-soccer.com/useful/glossary/soccer/")],
                base: "https://www.homemate-research-soccer.com",
                level1_selector: "section.sec_cmn:nth-child(4) > div > ul > li > a",
                title_selector: ".post_btn > h1:nth-child(1)",
                body_selector: ".post_box",
                image_selector: Some(".post_box_img > img:nth-child(1)"),
                ..Default::default()
            },
            SiteKindHierarchical::Sompocybersecurity => HierarchicalFlow {
                level2_links: vec![String::from("https://www.sompocybersecurity.com/glossary.html")],
                level1_selector: "div.glossary-list-wrap > div > div > div > div > div > div > h3 > a",
                title_selector: ".title-blog",
                body_selector: "#main > p",
                ..Default::default()
            },
            SiteKindHierarchical::Sufu => HierarchicalFlow {
                level2_links: vec![String::from("https://sufu.lifull.net/category/glossary/6")],
                level1_selector: "div.container.Glossary_index.is_wide > div > div.glossary_list > div > div > div > a",
                title_selector: "div.container.Glossary_detail.is_wide > div > div.main_contents > div.glossary_list > h1",
                body_selector: "div.container.Glossary_detail.is_wide > div > div.main_contents > div.glossary_list > section:nth-child(2) > p",
                ..Default::default()
            },
            SiteKindHierarchical::Suumo => HierarchicalFlow {
                index: "https://suumo.jp/yougo/",
                base: "https://suumo.jp",
                level2_selector: "ul.syllabary-list > li:nth-child(1) > a",
                level1_selector: "div.ui-section_h3 > div > div > ul > li > div > a",
                title_selector: ".ui-section--h1 > div > h1",
                body_selector: ".pagecaption-txt",
                ..Default::default()
            },
            SiteKindHierarchical::Unew => HierarchicalFlow {
                index: "http://www.u-new.com/word/",
                level2_selector: "table.wording_list_table01 > tbody > tr > td > a, div.width_700 > table > tbody > tr> td > a",
                level1_selector: "div.text_04 > a",
                title_selector: ".obi_02 > h2",
                body_selector: ".text_01, .text_02",
                ..Default::default()
            },
            SiteKindHierarchical::WaferMeasurementInspection => HierarchicalFlow {
                level2_links: vec![String::from("https://www.wafer-measurement-inspection.com/words/")],
                base: "https://www.wafer-measurement-inspection.com/words/",
                level1_selector: ".newslist > li > a",
                title_selector: ".ts3",
                body_selector: ".longComment",
                pool_size:1,
                rest:1,
                ..Default::default()
            },
            SiteKindHierarchical::Webtan => HierarchicalFlow {
                level2_links: vec![String::from("https://webtan.impress.co.jp/glossary/list/1")],
                base: "https://webtan.impress.co.jp",
                level1_selector: ".node > div:nth-child(1) > ul > li > a",  
                title_selector: "h1.title",
                body_selector: ".glossary_description",
                ..Default::default()
            },
            SiteKindHierarchical::Yodosha => HierarchicalFlow {
                index: "https://www.yodosha.co.jp/jikkenigaku/keyword/",
                level2_selector: ".index-list-wrap > ul > li > a",
                level1_selector: "div.keyword > a",
                title_selector: "#keyword",
                body_selector: ".definition",
                pool_size: 10,
                rest: 3,
                ..Default::default()
            },
            SiteKindHierarchical::Nichiren => HierarchicalFlow {
                level2_links: vec![String::from("https://www.nichiren.or.jp/glossary/")],
                level1_selector: ".glossary-table01 > tbody:nth-child(1) > tr > td > a",
                title_selector: ".glossary-post .head .title",
                body_selector: ".glossary-post .head .ruby, .glossary-post .body",
               pool_size:10,
                rest:10,
                 ..Default::default()
            },
        }
    }
}
