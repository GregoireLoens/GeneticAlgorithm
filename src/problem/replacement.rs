use crate::problem::data::{IndiData, Data};

fn find_worst_minimum(parent: &mut IndiData) -> (usize, usize) {
    let mut worst = (0, 0);

    for i in 0..parent.fit.len() {
        if parent.fit[worst.0] < parent.fit[i] {
            worst.1 = worst.0;
            worst.0 = i;
        }
    }
    return worst;
}

pub fn worst(data: &mut Data, parent: &mut IndiData, child: &mut IndiData) {
    let mut worsts: (usize, usize);

    if data.problem == std::u32::MIN {
        worsts = find_worst_minimum(parent);
        parent.pop[worsts.0] = child.pop[0].clone();
        parent.pop[worsts.1] = child.pop[1].clone();
    }
    else {

    }
}