use std::vec::Vec;
use std::io::{BufReader, BufRead};
use std::fs::File;
use std::str;
use std::fmt;
use rand::prelude::*;
use std::i32;


const NB_SAMPLE: usize = 10 * 2;
const MAXIT: usize = 200;

//
// structure for storing data
//
pub struct Data {
    problem: u32,
    t_cost: Vec<Vec<u32>>,
    pop: Vec<Vec<u32>>,
    fit: Vec<u32>,
    build_cost: Vec<u32>,
    pop_child: Vec<Vec<u32>>,
    fit_child: Vec<usize>,
    best_fit: u32
}

//
// function for data
//
impl Data {
    pub fn new(prob: u32, best_f: u32) -> Data {
        Data {
            problem: prob,
            pop: Vec::with_capacity(NB_SAMPLE),
            t_cost: Vec::new(),
            fit: Vec::with_capacity(NB_SAMPLE),
            pop_child: Vec::with_capacity(NB_SAMPLE),
            build_cost: Vec::new(),
            fit_child: Vec::with_capacity(NB_SAMPLE),
            best_fit: best_f
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

pub fn initialisation(data: &mut Data) {
    for mut it in 0..NB_SAMPLE {
        data.pop.push(Vec::new());
        for _i in 0..data.t_cost[0].len(){
            data.pop[it].push(rand::thread_rng().gen_range(0, 2));
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

pub fn evaluation(data: &mut Data){
    let mut ask_fit: u32;

    if data.problem == std::u32::MIN {ask_fit = std::u32::MAX;}
    else {ask_fit = std::u32::MIN;}

    for i in 0..data.pop.len() {
        data.fit.push(fitness(&mut data.t_cost, &mut data.build_cost, &mut data.pop[i]));
        if data.problem == std::u32::MIN {
            if data.fit[i] < data.best_fit{
                data.best_fit = data.fit[i];
            }
            if data.fit[i] < ask_fit {
                ask_fit = data.fit[i];
            }
        }
        else {
            if data.fit[i] > data.best_fit{
                data.best_fit = data.fit[i];
            }
            if data.fit[i] > ask_fit {
                ask_fit = data.fit[i];
            }
        }
    }
    println!("{} {}", ask_fit, data.best_fit);
}