mod problem;

fn genetic(data: &mut problem::Data) {
    problem::initialisation(data);
    data.print_pop();
}

fn main() {
    let mut data= problem::Data::new(std::u32::MIN);
    problem::file_reader("problem_sample/123UnifS.txt", &mut data);
    genetic(&mut data);
}
