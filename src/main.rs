mod problem;

fn genetic(data: &mut problem::Data, parent: &mut problem::IndiData, child: &mut problem::IndiData) {
    problem::initialisation(data, parent);
    problem::evaluation(data, parent);
    for it in 0..problem::MAXIT {

    }
}

fn main() {
    let mut data= problem::Data::new(std::u32::MIN, std::u32::MAX);
    let mut parent = problem::IndiData::new();
    let mut child = problem::IndiData::new();
    problem::file_reader("problem_sample/123UnifS.txt", &mut data);
    genetic(&mut data, &mut parent, &mut child);
}
