use std::vec::Vec;


pub fn print_tab(tab: & mut Vec<u32>) {
        for elem in tab {
            print!("{} ", elem)
        }
}

pub fn print_double_tab(tab: & mut Vec<Vec<u32>>) {
    for line in tab {
        for elem in line {
            print!("{} ", elem)
        }
        println!();
    }
}