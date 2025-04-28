use super::*;
use good_lp::{
    variables,
    variable,
    constraint,
    Variable,
    default_solver,
    Solution,
    SolverModel,
    Expression
};
use std::collections::HashMap;

pub struct GoodLPExtractor;
impl Extractor for GoodLPExtractor {
    fn extract(&self, egraph: &EGraph, roots: &[ClassId]) ->
    ExtractionResult {

        for (class_id,class) in egraph.classes() {
            print!("class {:?}: ", class.id);
        }

        let result = ExtractionResult::default();
        result
    }
}