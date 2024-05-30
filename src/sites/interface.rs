use crate::utils::Term;

pub trait WorkFlowTrait {
    fn is_my_kind(s: &'static str) -> bool;
    async fn get_terms(&self) -> Vec<Term>;
}
