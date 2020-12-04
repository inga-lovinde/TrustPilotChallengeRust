#![feature(map_into_keys_values)]

use std::cmp;
use std::env;
use rayon::prelude::*;

use trustpilot_challenge_rust::anagram_finder;
use trustpilot_challenge_rust::anagram_logger;
use trustpilot_challenge_rust::dictionary_builder;
use trustpilot_challenge_rust::hash_computer;
use trustpilot_challenge_rust::read_lines;

fn main() {
    let args: Vec<_> = env::args().collect();
    println!("{:?}", args);

    let words_file_path = &args[1];
    let hashes_file_path = &args[2];
    let max_requested_number_of_words = (&args[3]).parse::<usize>().unwrap();
    let phrase = &args[4];

    /*let message = hash_computer::prepare_messages(phrase);
    let hashes = hash_computer::compute_hashes(message, phrase.len());
    for hash in hashes.iter() {
        println!("{:#08x}", hash);
    }*/

    let phrase_byte_length_without_spaces = phrase.as_bytes().into_iter().filter(|&b| *b != b' ').count();
    let max_supported_number_of_words = (hash_computer::MAX_PHRASE_LENGTH - phrase_byte_length_without_spaces) + 1;

    if max_requested_number_of_words > max_supported_number_of_words {
        println!("Requested number of words unsupported; using {} as maximum number of words", max_supported_number_of_words);
    }
    let max_number_of_words = cmp::min(max_requested_number_of_words, max_supported_number_of_words);

    let mut words = read_lines::lines_from_file(words_file_path).unwrap();
    words.sort();
    words.dedup();

    let dictionary = dictionary_builder::build_dictionary(phrase, words);

    for number_of_words in 0..=max_number_of_words {
        let result = anagram_finder::find_anagrams(&dictionary, number_of_words);
        result.into_par_iter()
            .for_each(|anagram| println!("{}", anagram_logger::get_anagram_view(anagram, &dictionary)));
    }
}
