use super::interface::WorkFlowTrait;
use crate::utils::{PageLinkFlow, Term};

pub enum SiteKindPagelink {
    Comperu,
    Fooddies,
    Globis,
    Mwords,
}

pub struct PagelinkWorkFlow {
    pub kind: SiteKindPagelink,
}

impl PagelinkWorkFlow {
    pub fn new(kind: SiteKindPagelink) -> Self {
        Self { kind }
    }

    pub fn my_kind(kind_str: &'static str) -> Option<SiteKindPagelink> {
        match kind_str {
            "comperu" => Some(SiteKindPagelink::Comperu),
            "fooddies" => Some(SiteKindPagelink::Fooddies),
            "globis" => Some(SiteKindPagelink::Globis),
            "mwords" => Some(SiteKindPagelink::Mwords),
            _ => None,
        }
    }
}

impl WorkFlowTrait for PagelinkWorkFlow {
    fn is_my_kind(kind_str: &'static str) -> bool {
        Self::my_kind(kind_str).is_some()
    }

    async fn get_terms(&self) -> Vec<Term> {
        use crate::utils::Flow;
        self.get_flow().get_terms().await
    }
}

impl PagelinkWorkFlow {
    fn get_flow(&self) -> PageLinkFlow {
        match self.kind {
            SiteKindPagelink::Comperu => PageLinkFlow {
                level2_links: vec![String::from("https://comperu.jp/library/tag/%E3%83%93%E3%82%B8%E3%83%8D%E3%82%B9%E7%94%A8%E8%AA%9E/")],
                level2_selector: ".page-numbers.current + a",
                level1_selector: "article.item > a.image",
                title_selector: "h1.title",
                body_selector: ".tcdce-body",
                ..Default::default()
            },
            SiteKindPagelink::Fooddies => PageLinkFlow {
                level2_links: vec!["https://fooddies.tokyo/".to_string()],
                level2_selector: ".next",
                level1_selector: "a.entry-card-wrap",
                title_selector: ".entry-title",
                body_selector: "p.whitespace-pre-wrap:nth-child(4), p.whitespace-pre-wrap:nth-child(5), .entry-content > p:nth-child(1), .entry-content > p:nth-child(2), .entry-content > p:nth-child(3), div.group:nth-child(1) > div:nth-child(1) > div:nth-child(1) > div:nth-child(1) > div:nth-child(1) > div:nth-child(1) > p:nth-child(1), .markdown, .entry-content > p:nth-child(6)",
                ..Default::default()
            },
            SiteKindPagelink::Globis => PageLinkFlow {
                index: "https://mba.globis.ac.jp/about_mba/glossary/",
                level3_selector: ".hiragana > li > a, ul.link_list:nth-child(2) > li:nth-child(1) > a:nth-child(1), .alphabet > li > a",
                level2_selector: ".pagination_anchor_next",
                level1_selector: ".column_main > div > a",
                title_selector: "h1.section_title",
                body_selector: ".cms_section",
                ..Default::default()
            },
            SiteKindPagelink::Mwords => PageLinkFlow {
                index: "https://m-words.jp/",
                level3_selector: "p.has-text-align-center:nth-child(-n+5) > a",
                level2_selector: "a.page-numbers.-to-next",
                level1_selector: "ul.p-postList:nth-child(1) > li > a",
                title_selector: ".c-postTitle__ttl",
                body_selector: "div.post_content > p",
                ..Default::default()
            },
        }
    }
}
