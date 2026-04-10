pub mod interface;

// directly generate flow
pub mod simple;
// indirectly generate flow
pub mod customized;
// directly generate Vec<Term> without following flow
pub mod handmade;
// others
pub mod special;

use self::customized::CustomizedWorkFlow;
use self::handmade::HandmadeWorkFlow;
use self::interface::WorkFlowTrait;
use self::simple::SimpleWorkFlow;
use crate::utils::Term;

pub async fn run(kind_str: &'static str) -> Vec<Term> {
    if SimpleWorkFlow::is_my_kind(kind_str) {
        SimpleWorkFlow::new(kind_str).get_terms().await
    } else if CustomizedWorkFlow::is_my_kind(kind_str) {
        CustomizedWorkFlow::new(kind_str).get_terms().await
    } else if HandmadeWorkFlow::is_my_kind(kind_str) {
        HandmadeWorkFlow::new(kind_str).get_terms().await
    } else {
        panic!("not valid kind");
    }
}
