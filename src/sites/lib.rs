use super::customized::{CustomizedWorkFlow, CustomizedWorkFlowTrait};
use super::handmade::{HandmadeWorkFlow, HandmadeWorkFlowTrait};
use super::interface::WorkFlowTrait;
use super::simple::{SimpleWorkFlow, SimpleWorkFlowTrait};
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
