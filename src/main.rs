mod problem;
mod utils;

fn main() {
    let mut data= problem::Data::new(std::u32::MIN);
    problem::file_reader("problem_sample/123UnifS.txt", &mut data)
}
