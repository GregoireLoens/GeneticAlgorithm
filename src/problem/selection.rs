use crate::problem::data::{IndiData, Data};
use rand::Rng;


/*
** tournament
*/

fn versus(parent: &mut IndiData, bracket: &mut Vec<usize>, data: &mut Data, index: usize) -> usize {
    if data.problem == std::u32::MIN {
        if parent.fit[bracket[index]] < parent.fit[bracket[index + 1]]{
            return index + 1;
        }
        return index
    }
    else {
        if parent.fit[bracket[index]] > parent.fit[bracket[index + 1]]{
            return index + 1;
        }
        return index
    }
}

fn do_tournament(data: &mut Data, parent: &mut IndiData,
                 bracket: &mut Vec<usize>, nb_turn: usize) -> (usize, usize) {

    for i in 0..nb_turn {
        for j in 0..bracket.len() / 2{
                bracket.remove(versus(parent, &mut bracket.clone(), data, j));
            }
        }
    return (bracket[0], bracket[1]);
}


pub fn tournament(data: &mut Data, parent: &mut IndiData, t_size: usize) -> (usize, usize) {
    let win1: usize;
    let win2: usize;
    let nb_turn: usize;
    let mut bracket = Vec::with_capacity(t_size);

    for i in 0..t_size {
        let mut new_val = rand::thread_rng().gen_range(0, parent.pop.len());
        while bracket.contains(&new_val) {
            new_val = rand::thread_rng().gen_range(0, parent.pop.len());
        }
        bracket.push(new_val);
    }

    if t_size <= 2 {
        return (bracket[0], bracket[1]);
    }
    else {
        nb_turn = t_size / 4;
        return do_tournament(data, parent, &mut bracket, nb_turn);
    }
}

/*
** Ultra elitist selection
*/

pub fn elitist(data: &mut Data, parent: &mut IndiData) -> (usize, usize){
    let mut best_p: (usize, usize) = (0, 0);
    if data.problem == std::u32::MIN {
        for i in 0..parent.fit.len() {
            if parent.fit[i] < parent.fit[best_p.0]{
                best_p.0 = i;
            }
        }
        for i in 0..parent.fit.len() {
            if parent.fit[i] < parent.fit[best_p.1] && i != best_p.0 {
                best_p.1 = i;
            }
        }
        return best_p;
    }
    else {
        for i in 0..parent.fit.len() {
            if parent.fit[i] > parent.fit[best_p.0]{
                best_p.0 = i;
            }
        }
        for i in 0..parent.fit.len() {
            if parent.fit[i] > parent.fit[best_p.1] && i != best_p.0 {
                best_p.1 = i;
            }
        }
        return best_p;
    }
}