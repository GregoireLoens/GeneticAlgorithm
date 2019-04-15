use crate::problem::data::IndiData;
use rand::prelude::*;

pub fn one_point(best_p: (usize, usize) , parent: &mut IndiData, child: &mut IndiData) {
    let point = rand::thread_rng().gen_range(1,  parent.pop[0].len() -1);

    child.pop.clear();
    child.pop.push(Vec::new());
    child.pop.push(Vec::new());
    for i in 0..point {
        child.pop[0].push(parent.pop.clone()[best_p.1][i]);
        child.pop[1].push(parent.pop.clone()[best_p.0][i]);
    }
    for i in point..parent.pop[0].len(){
        child.pop[0].push(parent.pop.clone()[best_p.0][i]);
        child.pop[1].push(parent.pop.clone()[best_p.1][i]);
    }
}