mod problem;

fn genetic() {
    let mut data= problem::Data::new(std::u32::MIN, std::u32::MAX);
    let mut parent = problem::IndiData::new();
    let mut child: problem::IndiData;
    let mut p1: usize;
    let mut p2: usize;

    problem::file_reader("problem_sample/123UnifS.txt", &mut data);
    problem::initialisation(&mut data, &mut parent);
    problem::evaluation(&mut data, &mut parent);
    child = problem::IndiData::new_copy(&parent.pop, &parent.fit);
    for it in 0..problem::MAXIT {
        for mut j in 0..problem::NB_SAMPLE -1 {
            p1 = problem::selection(&mut parent);
            p2 = problem::selection(&mut parent);
            problem:: crossover(p1, p2, j, j+1, &mut parent, &mut child);
            j += 1;
        }
        problem::evaluation(&mut data, &mut child);
    }

}

fn main() {
    genetic();
}
