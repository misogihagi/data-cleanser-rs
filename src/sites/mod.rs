pub mod interface;

// directly generate flow
pub mod hierarchical;
pub mod singlepage;
pub mod pagelink;
// indirectly generate flow
pub mod customized;
// directly generate Vec<Term> without following flow
pub mod handmade;
pub mod pdf;
// others
pub mod browser;
pub mod special;

use self::browser::BrowserWorkFlow;
use self::customized::CustomizedWorkFlow;
use self::handmade::HandmadeWorkFlow;
use self::hierarchical::{HierarchicalWorkFlow, SiteKindHierarchical};
use self::interface::WorkFlowTrait;
use self::pagelink::{PagelinkWorkFlow, SiteKindPagelink};
use self::pdf::PdfWorkFlow;
use self::singlepage::{SiteKindSinglepage, SinglepageWorkFlow};

use crate::utils::Term;

pub enum SiteKind {
    Hierarchical(HierarchicalWorkFlow),
    Singlepage(SinglepageWorkFlow),
    Pagelink(PagelinkWorkFlow),
    Customized(CustomizedWorkFlow),
    Handmade(HandmadeWorkFlow),
    Pdf(PdfWorkFlow),
    Browser(BrowserWorkFlow),
}

impl SiteKind {
    pub fn from_str(s: &'static str) -> Option<Self> {
        HierarchicalWorkFlow::my_kind(s)
            .map(|k| SiteKind::Hierarchical(HierarchicalWorkFlow { kind: k }))
            .or_else(|| {
                SinglepageWorkFlow::my_kind(s)
                    .map(|k| SiteKind::Singlepage(SinglepageWorkFlow { kind: k }))
            })
            .or_else(|| {
                PagelinkWorkFlow::my_kind(s)
                    .map(|k| SiteKind::Pagelink(PagelinkWorkFlow { kind: k }))
            })
            .or_else(|| {
                CustomizedWorkFlow::my_kind(s)
                    .map(|k| SiteKind::Customized(CustomizedWorkFlow { kind: k }))
            })
            .or_else(|| {
                HandmadeWorkFlow::my_kind(s)
                    .map(|k| SiteKind::Handmade(HandmadeWorkFlow { kind: k }))
            })
            .or_else(|| PdfWorkFlow::my_kind(s).map(|k| SiteKind::Pdf(PdfWorkFlow { kind: k })))
            .or_else(|| {
                BrowserWorkFlow::my_kind(s).map(|k| SiteKind::Browser(BrowserWorkFlow { kind: k }))
            })
    }

    pub async fn get_terms(self) -> Vec<Term> {
        match self {
            SiteKind::Hierarchical(w) => w.get_terms().await,
            SiteKind::Singlepage(w) => w.get_terms().await,
            SiteKind::Pagelink(w) => w.get_terms().await,
            SiteKind::Customized(w) => w.get_terms().await,
            SiteKind::Handmade(w) => w.get_terms().await,
            SiteKind::Pdf(w) => w.get_terms().await,
            SiteKind::Browser(w) => w.get_terms().await,
        }
    }
}

pub async fn run(kind_str: &'static str) -> Vec<Term> {
    SiteKind::from_str(kind_str)
        .expect("not valid kind")
        .get_terms()
        .await
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_site_kind_from_str() {
        // Simple
        assert!(SiteKind::from_str("aritayaki").is_some());
        // Customized
        assert!(SiteKind::from_str("hrpro").is_some());
        // Handmade
        assert!(SiteKind::from_str("ajima").is_some());
        // Invalid
        assert!(SiteKind::from_str("invalid_site").is_none());
    }
}
