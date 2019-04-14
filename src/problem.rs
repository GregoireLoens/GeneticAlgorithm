use std::vec::Vec;
use std::io::{BufReader, BufRead};
use std::fs::File;
use std::str;
use std::fmt;
use rand::prelude::*;
use std::i32;


pub const NB_SAMPLE: usize = 10 * 2;
pub const MAXIT: usize = 200;

//
// structure for storing data
//
pub struct IndiData {
    pub pop:  Vec<Vec<u32>>,
    pub fit:  Vec<u32>,
}

pub struct Data {
    problem: u32,
    t_cost: Vec<Vec<u32>>,
    build_cost: Vec<u32>,
    best_fit: u32
}

//
// function for data
//
impl IndiData{
    pub fn new() -> IndiData {
        IndiData {
                pop: Vec::with_capacity(NB_SAMPLE),
                fit: Vec::with_capacity(NB_SAMPLE)
        }
    }

    pub fn new_copy(pop: &Vec<Vec<u32>>, fit: &Vec<u32>) -> IndiData {
        IndiData {
            pop: pop.clone(),
            fit: fit.clone(),
        }
    }
    pub fn print_pop(&self) {
        for line in self.pop.clone().iter() {
            for elem in line {
                print!("{} ", elem);
            }
            println!();
        }
    }
}

impl Data {
    pub fn new(prob: u32, best_f: u32) -> Data {
        Data {
            problem: prob,
            t_cost: Vec::new(),
            build_cost: Vec::new(),
            best_fit: best_f
        }
    }
}


//
// function for problem
//
pub fn file_reader(filename: &str, data: &mut Data) {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let mut i : i32 = 0;

    for (index, line) in reader.lines().enumerate() {
        let line = line.unwrap(); // Ignore errors.
        if index > 1 {
            let val_array: Vec<_> = line.split(" ").collect();
            data.t_cost.push(Vec::new());
            for elem in val_array {
                if i == 1 {
                    data.build_cost.push(elem.parse().unwrap())
                }
                if elem != "" && i > 1 {
                        data.t_cost[index - 2].push(elem.parse().unwrap());
                    }
                i = i + 1;
                }
            i = 0;
        }
    }
}

pub fn initialisation(data: &mut Data, parent: &mut IndiData) {
    for mut it in 0..NB_SAMPLE {
        parent.pop.push(Vec::new());
        for _i in 0..data.t_cost[0].len(){
            parent.pop[it].push(rand::thread_rng().gen_range(0, 2));
        }
    }
}

pub fn fitness(t_cost: &mut Vec<Vec<u32>>, build_cost: &mut Vec<u32>, indi: &mut Vec<u32>) -> u32 {
    let mut fit = 0;
    let mut tmp_fit = std::u32::MAX;

    for i in 0..indi.len() {
        if indi[i] == 1 {
            fit += build_cost[i];
        }
    }
    for i in 0..t_cost[0].len() {
        for j in 0..t_cost.len() {
            if indi[j] == 1 && t_cost[j][i] < tmp_fit {
                tmp_fit = t_cost[j][i];
            }
        }
        fit += tmp_fit;
        tmp_fit = std::u32::MAX;
    }
    return fit;
}

pub fn evaluation(data: &mut Data, parent: &mut IndiData){
    parent.fit.clear();
    let mut ask_fit: u32;

    if data.problem == std::u32::MIN {ask_fit = std::u32::MAX;}
    else {ask_fit = std::u32::MIN;}

    for i in 0..parent.pop.len() {
        parent.fit.push(fitness(&mut data.t_cost, &mut data.build_cost, &mut parent.pop[i]));
        if data.problem == std::u32::MIN {
            if parent.fit[i] < data.best_fit{
                data.best_fit = parent.fit[i];
            }
            if parent.fit[i] < ask_fit {
                ask_fit = parent.fit[i];
            }
        }
        else {
            if parent.fit[i] > data.best_fit{
                data.best_fit = parent.fit[i];
            }
            if parent.fit[i] > ask_fit {
                ask_fit = parent.fit[i];
            }
        }
    }
    println!("{} {}", ask_fit, data.best_fit);
}

pub fn selection(parent: &mut IndiData) -> usize{
    let s1= rand::thread_rng().gen_range(0, parent.pop.len());
    let s2= rand::thread_rng().gen_range(0, parent.pop.len());

    if parent.fit[s1] < parent.fit[s2]{
        return s1;
    }
    return s2;
}

pub fn crossover(p1: usize, p2: usize, c1: usize, c2: usize,
                 parent: &mut IndiData, child: &mut IndiData) {
    let cut = rand::thread_rng().gen_range(0, parent.pop[0].len());
    for i in 0..cut {
        child.pop[c1][i] = parent.pop[p1][i];
        child.pop[c2][i] = parent.pop[p2][i];

    }
    for i in cut..parent.pop[0].len() {
        child.pop[c1][i] = parent.pop[p2][i];
        child.pop[c2][i] = parent.pop[p1][i];
    }
}

pub fn flip_mutation(child: &mut IndiData, j: usize) {
    let prob: f32 = 1.0 / child.pop[0].len() as f32;
    let mut rand: f32;
    for i in 0..child.pop[j].len() {
        rand = rand::thread_rng().gen();
        if rand < prob {
            if child.pop[j][i] == 0 {
                child.pop[j][i] = 1;
            }
            else {
                child.pop[j][i] = 0;
            }
        }
    }
}

pub fn replacement(parent: &mut IndiData, child: &mut IndiData) {
    parent.pop = child.pop.clone();
}