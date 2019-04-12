use std::vec::Vec;
use std::io::{BufReader, BufRead};
use std::fs::File;
use std::str;
use super::utils;

const NB_SAMPLE: usize = 10 * 2;
const MAXIT: usize = 200;

pub struct Data {
    problem: u32,
    t_cost: Vec<Vec<u32>>,
    pop: Vec<Vec<u32>>,
    fit: Vec<usize>,
    build_cost: Vec<u32>,
    pop_child: Vec<Vec<u32>>,
    fit_child: Vec<usize>,
    best_fit: u32
}

impl Data {
    pub fn new(prob: u32) -> Data {
        Data {
            problem: prob,
            pop: Vec::new(),
            t_cost: Vec::new(),
            fit: Vec::with_capacity(NB_SAMPLE),
            pop_child: Vec::new(),
            build_cost: Vec::new(),
            fit_child: Vec::with_capacity(NB_SAMPLE),
            best_fit: prob
        }
    }
}

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
    utils::print_double_tab(&mut data.t_cost);
    utils::print_tab(&mut data.build_cost);
}