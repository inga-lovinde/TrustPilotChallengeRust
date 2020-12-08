#![feature(map_into_keys_values)]

use std::cmp;
use std::env;
use rayon::prelude::*;

use trustpilot_challenge_rust::anagram_analyzer;
use trustpilot_challenge_rust::anagram_finder;
use trustpilot_challenge_rust::dictionary_builder;
use trustpilot_challenge_rust::hash_computer;
use trustpilot_challenge_rust::permutations_cache;
use trustpilot_challenge_rust::read_lines;

fn main() {
    let args: Vec<_> = env::args().collect();
    println!("{:?}", args);

    let words_file_path = &args[1];
    let hashes_file_path = &args[2];
    let max_requested_number_of_words = (&args[3]).parse::<usize>().unwrap();
    let phrase = &args[4];

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

    let hashes_strings = read_lines::lines_from_file(hashes_file_path).unwrap();
    let mut hashes_to_find: Vec<u32> = Vec::new();
    for hash_string in hashes_strings {
        let hash: u128 = u128::from_str_radix(&hash_string, 16).unwrap();
        hashes_to_find.push(((hash >> 96) as u32).to_be());
    }

    for number_of_words in 1..=max_number_of_words {
        let phrase_length = phrase_byte_length_without_spaces + number_of_words - 1;
        let permutations = permutations_cache::PermutationsCache::new(number_of_words);
        let anagram_vectors = anagram_finder::find_anagrams(&dictionary, number_of_words);
        anagram_vectors.par_iter()
            .for_each(|anagram_vector| {
                anagram_analyzer::analyze_anagrams(anagram_vector, &dictionary, &permutations, phrase_length, &hashes_to_find)
            });
    }
}
