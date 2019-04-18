use crate::problem::data::IndiData;
use rand::prelude::*;

pub fn mutation_of_one(child: &mut IndiData) {
    let p = 1.0 / child.pop.len() as f64;
    let mut rand: f64;
    let mut index: usize;

    for i in 0..2 {
        index = rand::thread_rng().gen_range(0, child.pop[0].len());
        rand = rand::thread_rng().gen_range(0.0, 1.0);
        if rand < p {
            if child.pop[i][index] == 0 {
                child.pop[i][index] = 1;
            }
            else {
                child.pop[i][index] = 0;
            }
        }
    }
}

pub fn mutation_on_multiple(child: &mut IndiData) {
    let p = 1.0 / child.pop.len() as f64;
    let mut rand: f64;
    let mut index: usize;

    for i in 0..child.pop.len() {
        for j in 0..child.pop.len() {
            rand = rand::thread_rng().gen_range(0.0, 1.0);
            if rand < p {
                if child.pop[i][j] == 0 {
                    child.pop[i][j] = 1;
                }
                else {
                    child.pop[i][j] = 0;
                }
            }
        }
    }
}