use crate::utils::Term;

pub trait WorkFlowTrait {
    fn is_my_kind(s: &'static str) -> bool;
    fn get_terms(&self) -> impl std::future::Future<Output = Vec<Term>>;
}
