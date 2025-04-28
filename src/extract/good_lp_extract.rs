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
        let mut vars = variables!();
        let mut enode_vars: HashMap<(ClassId, usize), Variable> = HashMap::new();

        /* t_m */
        const EPS: f64 = 1e-3;
        const ALPHA: f64 = 2.0;
        let mut topo_vars: HashMap<ClassId, Variable> = HashMap::new();
        for (class_id, class) in egraph.classes() {
            let t_m = vars.add(variable().min(0.0).max(1.0));
            topo_vars.insert(class.id.clone(), t_m);

            for (node_index, _node) in class.nodes.iter().enumerate() {
                let node_var = enode_vars
                    .entry((class.id.clone(), node_index))
                    .or_insert_with(|| vars.add(variable().binary()));

                for child in &egraph.nodes[node_index].children {
                    let child_class = egraph.nid_to_class(&child);
                    let mut child_vars = Vec::new();
                    for (c_idx, _c_node) in child_class.nodes.iter().enumerate() {
                        let var = enode_vars
                            .entry((child_class.id.clone(), c_idx))
                            .or_insert_with(|| vars.add(variable().binary()))
                            .clone();
                        child_vars.push(var);
                    }
                }

            }
        }


        let result = ExtractionResult::default();
        result
    }
}