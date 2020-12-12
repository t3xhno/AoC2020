#![allow(unused_variables, unused_assignments, dead_code)]
mod loading;
mod matrix;

use loading::get_input_data;
use matrix::*;

fn main () {
    let input: String = get_input_data("input.txt");
    let matrix = Matrix::new(&input);
    println!("{:?}", matrix.get_element(1, 97).get_neighbors(&matrix));
}