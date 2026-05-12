pub mod interface;

// directly generate flow
pub mod simple;
// indirectly generate flow
pub mod customized;
// directly generate Vec<Term> without following flow
pub mod handmade;
pub mod pdf;
// others
pub mod special;

use self::customized::CustomizedWorkFlow;
use self::handmade::HandmadeWorkFlow;
use self::interface::WorkFlowTrait;
use self::pdf::PdfWorkFlow;
use self::simple::SimpleWorkFlow;
use crate::utils::Term;

pub enum SiteKind {
    Simple(SimpleWorkFlow),
    Customized(CustomizedWorkFlow),
    Handmade(HandmadeWorkFlow),
    Pdf(PdfWorkFlow),
}

impl SiteKind {
    pub fn from_str(s: &'static str) -> Option<Self> {
        SimpleWorkFlow::my_kind(s)
            .map(|k| SiteKind::Simple(SimpleWorkFlow { kind: k }))
            .or_else(|| {
                CustomizedWorkFlow::my_kind(s)
                    .map(|k| SiteKind::Customized(CustomizedWorkFlow { kind: k }))
            })
            .or_else(|| {
                HandmadeWorkFlow::my_kind(s).map(|k| SiteKind::Handmade(HandmadeWorkFlow { kind: k }))
            })
            .or_else(|| {
                PdfWorkFlow::my_kind(s).map(|k| SiteKind::Pdf(PdfWorkFlow { kind: k }))
            })
    }

    pub async fn get_terms(self) -> Vec<Term> {
        match self {
            SiteKind::Simple(w) => w.get_terms().await,
            SiteKind::Customized(w) => w.get_terms().await,
            SiteKind::Handmade(w) => w.get_terms().await,
            SiteKind::Pdf(w) => w.get_terms().await,
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
