pub mod problem;

fn genetic() {
    let mut data = problem::data::Data::new(std::u32::MIN, std::u32::MAX);
    let mut parent = problem::data::IndiData::new();
    let mut child = problem::data::IndiData::new();
    let mut best_p: (usize, usize);

    /*
    ** Initialisation
    */

    problem::data::file_reader("problem_sample/123UnifS.txt", &mut data);
    problem::data::initialisation(&mut data, &mut parent);
    problem::data::evaluation(&mut data, &mut parent);

    /*
    ** Algorithm loop
    */

    for it in 0..problem::data::MAXIT {
        best_p = problem::selection::tournament(&mut data, &mut parent, 18);
        problem::crossover::one_point(best_p, &mut parent, &mut child);
        problem::data::mutation(&mut child);
        problem::replacement::worst(&mut data, &mut parent, &mut child);
        problem::data::evaluation(&mut data, &mut parent);
    }
}

fn main() {
    genetic();
}
