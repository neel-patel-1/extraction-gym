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
        let mut constraints = Vec::new();
        let mut enode_vars: HashMap<(ClassId, usize), Variable> = HashMap::new();
        let mut total_cost: Expression = 0.into();

        /* t_m */
        const EPS: f64 = 1e-3;
        const ALPHA: f64 = 2.0;
        let mut topo_vars: HashMap<ClassId, Variable> = HashMap::new();
        for (class_id, class) in egraph.classes() {
            let t_m = vars.add(variable().min(0.0).max(1.0));
            topo_vars.insert(class_id.clone(), t_m);
        }
        for (class_id, class) in egraph.classes() {
            let t_m = topo_vars[&class_id].clone();

            for (node_index, _node) in class.nodes.iter().enumerate() {
                let node_var = { enode_vars
                    .entry((class.id.clone(), node_index))
                    .or_insert_with(|| vars.add(variable().binary()))
                    .clone()
                };

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
                    let child_sum: Expression = child_vars.iter().cloned().sum();
                    constraints.push(Into::<Expression>::into(node_var.clone()).leq(child_sum));
                    println!("Adding constraint: node_var <= sum(child_vars) for class {:?}, node index {}", class_id, node_index); //constraint

                    let node_expr: Expression = node_var.clone().into();
                    let t_child = topo_vars[&child_class.id].clone();
                    let big_m_part: Expression = (Into::<Expression>::into(1.0) - node_expr.clone()) * ALPHA;
                    let acyc_expr: Expression = t_m.clone() - t_child - (Into::<Expression>::into(EPS)) + big_m_part.clone();
                    constraints.push(Into::<Expression>::into(acyc_expr).geq(Into::<Expression>::into(0)));
                    println!("Adding acyclicity constraint: t_m - t_child - EPS + big_m_part >= 0 for parent class {:?}, child class {:?}, node index {}", class_id, child_class.id, node_index); //constraint
                }
                let node = &egraph[_node];
                let cost = node.cost.into_inner();

                total_cost += cost * node_var;
            }
        }
        println!("Total cost function: {:?}", total_cost);

        for root in roots {
            println!("Num Classes: {} Root Class: {}", egraph.classes().len(), root);
            let root_class = &egraph[root];
            let root_vars = root_class
                .nodes
                .iter()
                .enumerate()
                .map(|(node_index, _)| enode_vars[&(root.clone(), node_index)].clone())
                .collect::<Vec<_>>();
            constraints.push(root_vars.iter().cloned().sum::<Expression>().eq(1));
            println!("Adding root constraint: sum(root_vars) == 1 for root_vars = {:?}", root_vars);
        }

        let solution = vars
            .minimise(&total_cost)
            .using(default_solver)
            .with_all(constraints)
            .solve()
            .unwrap();

        let obj_cost = solution.eval(&total_cost);

        let mut result = ExtractionResult::default();

        // Our original implementation performs a depth-first reconstruction, recursively. We do the same here, except instead of building up a `RecExpr`, we simply add the selected nodes to the `ExtractionResult`.
        fn reconstruct(egraph: &EGraph, class_id: &ClassId, solution: &dyn Solution, enode_vars: &HashMap<(ClassId, usize), Variable>, result: &mut ExtractionResult) {
            let class = &egraph[class_id];
            for (node_index, node) in class.nodes.iter().enumerate() {
                let var = enode_vars[&(class_id.clone(), node_index)];
                if solution.value(var) > 0.5 {
                    for child in &egraph.nodes[node_index].children {
                        let child_class = egraph.nid_to_class(child);
                        reconstruct(egraph, &child_class.id, solution, enode_vars, result);
                    }
                    result.choose(class_id.clone(), node.clone());
                    break;
                }
            }
        }

        for root in roots {
            reconstruct(egraph, root, &solution, &enode_vars, &mut result);
        }

        return result
    }
}