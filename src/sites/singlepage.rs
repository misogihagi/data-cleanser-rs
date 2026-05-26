use super::interface::WorkFlowTrait;
use crate::utils::{SinglepageFlow, Term};

pub enum SiteKindSinglepage {
    Amazonpay,
    Civileng,
    Jiki,
    Mitsujp,
    Ri,
    Meiwakaiun,
    Nisso,
    Theglenlivet,
    UniversalOOH,
    Jfa,
}

pub struct SinglepageWorkFlow {
    pub kind: SiteKindSinglepage,
}

impl SinglepageWorkFlow {
    pub fn new(kind: SiteKindSinglepage) -> Self {
        Self { kind }
    }

    pub fn my_kind(kind_str: &'static str) -> Option<SiteKindSinglepage> {
        match kind_str {
            "amazonpay" => Some(SiteKindSinglepage::Amazonpay),
            "civileng" => Some(SiteKindSinglepage::Civileng),
            "jiki" => Some(SiteKindSinglepage::Jiki),
            "mitsujp" => Some(SiteKindSinglepage::Mitsujp),
            "ri" => Some(SiteKindSinglepage::Ri),
            "meiwakaiun" => Some(SiteKindSinglepage::Meiwakaiun),
            "nisso" => Some(SiteKindSinglepage::Nisso),
            "theglenlivet" => Some(SiteKindSinglepage::Theglenlivet),
            "universalooh" => Some(SiteKindSinglepage::UniversalOOH),
            "jfa" => Some(SiteKindSinglepage::Jfa),
            _ => None,
        }
    }
}

impl WorkFlowTrait for SinglepageWorkFlow {
    fn is_my_kind(kind_str: &'static str) -> bool {
        Self::my_kind(kind_str).is_some()
    }

    async fn get_terms(&self) -> Vec<Term> {
        use crate::utils::Flow;
        self.get_flow().get_terms().await
    }
}

impl SinglepageWorkFlow {
    fn get_flow(&self) -> SinglepageFlow {
        match self.kind {
            SiteKindSinglepage::Amazonpay => SinglepageFlow {
                    links: vec!["https://pay.amazon.co.jp/resources/ecommerce-glossary".to_string()],
                    titles_selector:"div > div > bsp-faq-question > .FaqQuestion-header",
                    bodies_selector:"div > div > bsp-faq-question > .FaqQuestion-answer",
                    ..Default::default()
            },
            SiteKindSinglepage::Civileng => SinglepageFlow {
                    links: ["a","ka","sa","ta","na","ha","ma","ya","ra","wa"].map(|s| "http://civileng.ec-net.jp/yougo/".to_string()+s+".htm").to_vec(),
                    titles_selector:"table > tbody > tr > td:nth-child(2)",
                    bodies_selector: "table > tbody > tr > td:nth-child(3)",
                    encoding: "shift-jis",
                    ..Default::default()
            },
            SiteKindSinglepage::Jfa => SinglepageFlow {
                index: "https://www.jfa.jp/laws/soccer/glossary/",
                titles_selector: "#main-colum > div:nth-child(3) > h5, #main-colum > div:nth-child(3) > h4:nth-child(187)",
                bodies_selector: "#main-colum > div:nth-child(3) > p:not(:last-child)",
                ..Default::default()
            },
            SiteKindSinglepage::Jiki => SinglepageFlow {
                    index: "https://www.jiki.jp/words/",
                    level1_selector: "div.entry > table > tbody:nth-child(1) > tr > td > a",
                    titles_selector:".entry > h3",
                    bodies_selector: ".entry > p",
                    ..Default::default()
            },
            SiteKindSinglepage::Mitsujp => SinglepageFlow {
                    links: vec![String::from("https://www.mitsujp.com/glossary1/"), String::from("https://www.mitsujp.com/glossary2/")],
                    titles_selector:"div.post_content > div.wp-block-group > div > h3, details.swell-block-accordion__item > div > div > div > div > div > h3",
                    bodies_selector: "div.post_content > div.wp-block-group > div > p, details.swell-block-accordion__item > div > div > div > div > div",
                    ..Default::default()
            },
            SiteKindSinglepage::Nisso => SinglepageFlow {
                    links: vec!["https://www.nisso-sangyo.co.jp/glossary".to_string()],
                    titles_selector:"section.dic-index__section > div > div > h3",
                    bodies_selector: "section.dic-index__section > div > div.dic-item__body",
                    ..Default::default()
            },
            SiteKindSinglepage::Ri => SinglepageFlow {
                    index: "https://r-i.jp/glossary/",
                    base: "https://r-i.jp",
                    level1_selector: "ul.tabInitial:nth-child(1) > li > a",
                    titles_selector:".glossary_entry_title",
                    bodies_selector: ".glossary_entry_body",
                    ..Default::default()
            },
            SiteKindSinglepage::Meiwakaiun => SinglepageFlow {
                index: "https://www.meiwakaiun.com/meiwalabo/yougo/",
                // section.grossary typo!
                titles_selector: "main > div.page-box > div.container > div.glossary-details > section.grossary-details-box > table > tbody > tr > td > div.glossary-details > section.grossary-details-box > table > tbody > tr:not(#ki02a) > th",
                bodies_selector: "main > div.page-box > div.container > div.glossary-details > section.grossary-details-box > table > tbody > tr > td > div.glossary-details > section.grossary-details-box > table > tbody > tr:not(#ki02a) > td",
                ..Default::default()
            },
            SiteKindSinglepage::Theglenlivet => SinglepageFlow {
                index: "https://www.theglenlivet.jp/craft/whisky-words.html",
                titles_selector: "section.producttext > div > h3",
                bodies_selector: "section.producttext > div > h3 + p",
                ..Default::default()
            },
            SiteKindSinglepage::UniversalOOH => SinglepageFlow {
                    index: "https://universal-ooh.jeki.co.jp/course_ooh/%e5%9f%ba%e6%9c%ac%e7%94%a8%e8%aa%9e%e3%81%ae%e8%a7%a3%e8%aa%ac/",
                    level1_selector: "ul.is-open > li > a",
                    titles_selector:".c-chapter__ttl",
                    bodies_selector: ".c-chapter__txt",
                    ..Default::default()
            },
        }
    }
}
