use crate::problem::data::{IndiData, Data, fitness};

/*
** worst individuals => best children
*/

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

pub fn worst_indi(data: &mut Data, parent: &mut IndiData, child: &mut IndiData) {
    let worsts: (usize, usize);

    if data.problem == std::u32::MIN {
        worsts = find_worst_minimum(parent);
        parent.pop[worsts.0] = child.pop[0].clone();
        parent.pop[worsts.1] = child.pop[1].clone();
    }
    else {

    }
}


/*
** worst parent = best child
*/

fn select(parent: &IndiData, parents: (usize, usize), best: bool) -> usize {
    if !best {
        if parent.fit[parents.0] > parent.fit[parents.1]{
            return parents.0;
        }
        return parents.1;
    }
    else {
        if parent.fit[parents.0] < parent.fit[parents.1]{
            return parents.0;
        }
        return parents.1;
    }
}

pub fn worst_parent(data: &mut Data, parent: &mut IndiData, child: &mut IndiData, best_p: (usize, usize)) {
    let mut costs = data.get_costs();
    let worst_parent = select(&parent, best_p, false);
    child.fit.push(fitness(&mut costs.0, &mut costs.1, &mut child.pop[0]));
    child.fit.push(fitness(&mut costs.0, &mut costs.1, &mut child.pop[1]));
    let best_child  = select(&child, (0, 1), true);

    parent.pop[worst_parent] = child.pop[best_child].clone();
}

/*
** replace 2 parents
*/

pub fn all_parent(parent: &mut IndiData, child: &mut IndiData, best_p: (usize, usize)) {
    parent.pop[best_p.0] = child.pop[0].clone();
    parent.pop[best_p.1] = child.pop[1].clone();
}

/*
** full replacement
*/

pub fn full(parent: &mut IndiData, child: &mut IndiData) {
    parent.pop = child.pop.clone();
    child.pop.clear();
    child.fit.clear();
}