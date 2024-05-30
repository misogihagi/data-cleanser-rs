use super::interface::WorkFlowTrait;
use crate::utils::{Flow, FlowA, Term};

pub enum SiteKindCustomized {
    Hrpro,
}

pub struct CustomizedWorkFlow {
    pub kind: SiteKindCustomized,
}
pub trait CustomizedWorkFlowTrait: WorkFlowTrait {
    fn new(kind_str: &'static str) -> CustomizedWorkFlow;
    fn my_kind(kind_str: &'static str) -> Option<SiteKindCustomized>;
}

impl CustomizedWorkFlowTrait for CustomizedWorkFlow {
    fn new(kind_str: &'static str) -> CustomizedWorkFlow {
        CustomizedWorkFlow {
            kind: CustomizedWorkFlow::my_kind(kind_str).unwrap(),
        }
    }
    fn my_kind(kind_str: &'static str) -> Option<SiteKindCustomized> {
        match kind_str {
            "hrpro" => Some(SiteKindCustomized::Hrpro),
            _ => None,
        }
    }
}

impl WorkFlowTrait for CustomizedWorkFlow {
    fn is_my_kind(kind_str: &'static str) -> bool {
        match CustomizedWorkFlow::my_kind(kind_str) {
            Some(_) => true,
            None => false,
        }
    }
    async fn get_terms(&self) -> Vec<Term> {
        customize(&self.kind).get_terms().await
    }
}

fn customize(kind: &SiteKindCustomized) -> impl Flow {
    match kind {
        SiteKindCustomized::Hrpro => {
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
