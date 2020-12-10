use std::env;
use rayon::prelude::*;

use trustpilot_challenge_rust::read_lines;
use trustpilot_challenge_rust::solver::Solver;

fn main() {
    let args: Vec<_> = env::args().collect();
    println!("{:?}", args);

    let words_file_path = &args[1];
    let hashes_file_path = &args[2];
    let max_requested_number_of_words = (&args[3]).parse::<usize>().unwrap();
    let phrase = &args[4];

    let words = read_lines::lines_from_file(words_file_path).unwrap();
    let hashes_strings = read_lines::lines_from_file(hashes_file_path).unwrap();

    let solver = Solver::create_from_input_data(words, hashes_strings, max_requested_number_of_words, phrase);
    solver.find_solutions()
        .for_each(|solution| println!("{} {}", solution.get_hash(), solution.anagram_string));
}
