use super::interface::WorkFlowTrait;
use crate::utils::{HeadingRangeFlow, Term};

pub enum SiteKindRange {
    Navigateinc,
    Ts4500,
}

pub struct RangeWorkFlow {
    pub kind: SiteKindRange,
}

impl RangeWorkFlow {
    pub fn new(kind: SiteKindRange) -> Self {
        Self { kind }
    }

    pub fn my_kind(kind_str: &'static str) -> Option<SiteKindRange> {
        match kind_str {
            "navigateinc" => Some(SiteKindRange::Navigateinc),
            "ts4500" => Some(SiteKindRange::Ts4500),
            _ => None,
        }
    }
}

impl WorkFlowTrait for RangeWorkFlow {
    fn is_my_kind(kind_str: &'static str) -> bool {
        Self::my_kind(kind_str).is_some()
    }

    async fn get_terms(&self) -> Vec<Term> {
        use crate::utils::Flow;
        self.get_flow().get_terms().await
    }
}

impl RangeWorkFlow {
    fn get_flow(&self) -> HeadingRangeFlow {
        match self.kind {
            SiteKindRange::Navigateinc => HeadingRangeFlow {
                index: "https://www.navigate-inc.co.jp/term/index.html",
                level1_selector: "#linkbtn01 > ul > li > a, h2.sh2:nth-child(2) > a",
                titles_selector: "#bodyZonethird > dl > dt, p.mng1",
                last_body_selector:
                    "#bodyZonethird > dl:last-of-type > dd:last-of-type, ul.mng:last-of-type",
                encoding: "shift-jis",
                ..Default::default()
            },
            SiteKindRange::Ts4500 => HeadingRangeFlow {
                links: vec![
                    "https://www.ibm.com/docs/api/v1/content/STQRQ9%2Fcom.ibm.storage.ts4500.doc%2Fts4500_gl.html?parsebody=true&lang=ja"
                        .to_string(),
                ],
                titles_selector: "dl > dt",
                last_body_selector: "#ic_ts3500_gl__z > dl:nth-child(2) > dd:nth-child(2)",
                ..Default::default()
            },
        }
    }
}
