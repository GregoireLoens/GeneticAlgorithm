use crate::problem::data::{IndiData, Data, total_fit};
use std::thread;
use std::sync::mpsc;

pub mod problem;

fn generate_child(parent: &mut IndiData, data: &mut Data) -> problem::data::IndiData {
    let mut child = problem::data::IndiData::new();
    let mut best_p: (usize, usize);

    for i in 0..parent.pop.len() / 2 {
        best_p = problem::selection::tournament(data, parent, 18);
        problem::crossover::k_point(best_p, parent, &mut child, 2);
        problem::mutation::mutation_of_one(&mut child);
    }
    problem::data::evaluation(data, &mut child);
    return child;
}

fn select_best_child(child: &mut Vec<problem::data::IndiData>) -> usize {
    let mut index = 0;
    for i in 0..6 {
        if total_fit(&mut child[index].fit.clone()) > total_fit(&mut child[i].fit.clone()) {
            index = i;
        }
    }
    return index;
}


fn genetic() {
    let mut data = problem::data::Data::new(std::u32::MIN, std::u32::MAX);
    let mut parent = problem::data::IndiData::new();
    let mut child: Vec<problem::data::IndiData> = Vec::with_capacity(4);
    let mut best_child = problem::data::IndiData::new();
    let mut handles = Vec::with_capacity(4);
    let (tx, rx) = mpsc::channel();

    /*
    ** Initialisation
    */
    problem::data::file_reader("problem_sample/123UnifS.txt", &mut data);
    problem::data::initialisation(&mut data, &mut parent);
    problem::data::evaluation(&mut data, &mut parent);


    /*
    ** Genetic algorithm loop
    */
    for it in 0..problem::data::MAXIT {
        for _ in 0..6 {
            let (mut new_parent,mut new_data, clone_tx)
                = (parent.clone(), data.clone(), tx.clone());
            let mut new_data = data.clone();
            handles.push(thread::spawn(move|| {
                clone_tx.send(generate_child(&mut new_parent, &mut new_data));
            }));
        }

       for _ in 0..6 {
            child.push(rx.recv().unwrap())
       }
        let best = select_best_child(&mut child);
        best_child = child[best].clone();

        problem::replacement::full(&mut parent, &mut best_child);
        problem::data::evaluation(&mut data, &mut parent);
        handles.clear();
        child.clear()
    }
    print!("{}", data.buffer);
}


fn main() {
    genetic();
}
