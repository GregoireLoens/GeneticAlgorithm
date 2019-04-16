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

pub fn k_point(best_p: (usize, usize), parent: &mut IndiData, child: &mut IndiData, k: usize) {
    let mut point_tab = Vec::with_capacity(k);
    let mut point: usize;
    let mut index_child: (usize, usize) = (0, 1);

    child.pop.clear();
    child.pop.push(Vec::new());
    child.pop.push(Vec::new());
    for i in 0..k {
        point = rand::thread_rng().gen_range(0, parent.pop[0].len());
        while point_tab.contains(&point) {
            point = rand::thread_rng().gen_range(0, parent.pop[0].len());
        }
        point_tab.push(point);
    }
    point_tab.push(parent.pop[0].len());
    point_tab.sort();
    point = 0;
    for c_point in point_tab.iter() {
        for i in point..c_point.clone() {
            child.pop[index_child.0].push(parent.pop[best_p.0][i]);
            child.pop[index_child.1].push(parent.pop[best_p.1][i]);
        }
        point = index_child.0;
        index_child.0 = index_child.1;
        index_child.1 = point;
        point = c_point.clone();
    }
}