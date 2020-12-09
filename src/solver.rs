use std::cmp;
use rayon::prelude::*;

use crate::anagram_analyzer;
use crate::anagram_finder;
use crate::dictionary_builder::Dictionary;
use crate::hash_computer;
use crate::permutations_cache::PermutationsCache;
use crate::solution::Solution;

pub struct Solver {
    dictionary: Dictionary,
    hashes_to_find: Vec<u32>,
    phrase_byte_length_without_spaces: usize,
    max_number_of_words: usize,
}

impl Solver {
    pub fn create_from_input_data(words: Vec<String>, hashes_strings: Vec<String>, max_requested_number_of_words: usize, phrase: &str) -> Solver {
        let phrase_byte_length_without_spaces = phrase.as_bytes().into_iter().filter(|&b| *b != b' ').count();
        let max_supported_number_of_words = (hash_computer::MAX_PHRASE_LENGTH - phrase_byte_length_without_spaces) + 1;
    
        if max_requested_number_of_words > max_supported_number_of_words {
            println!("Requested number of words unsupported; using {} as maximum number of words", max_supported_number_of_words);
        }
        let max_number_of_words = cmp::min(max_requested_number_of_words, max_supported_number_of_words);
    
        let mut words = words;
        words.sort();
        words.dedup();
    
        let dictionary = Dictionary::from_phrase_and_words(phrase, words);
    
        let mut hashes_to_find: Vec<u32> = Vec::new();
        for hash_string in hashes_strings {
            let hash: u128 = u128::from_str_radix(&hash_string, 16).unwrap();
            hashes_to_find.push(((hash >> 96) as u32).to_be());
        }

        Solver {
            dictionary,
            hashes_to_find,
            phrase_byte_length_without_spaces,
            max_number_of_words,
        }
    }

    fn solve_for_anagram_vectors<'a>(&'a self, anagram_vectors: Vec<Vec<usize>>, permutations: PermutationsCache, phrase_length: usize) -> impl ParallelIterator<Item = Solution> + 'a {
        anagram_vectors.into_par_iter()
            .flat_map(move |anagram_vector| {
                anagram_analyzer::analyze_anagrams(anagram_vector, &self.dictionary, &permutations, phrase_length, &self.hashes_to_find)
            })
    }
    
    fn solve_for_number_of_words<'a>(&'a self, number_of_words: usize) -> impl ParallelIterator<Item = Solution> + 'a {
        let phrase_length = self.phrase_byte_length_without_spaces + number_of_words - 1;
        let permutations = PermutationsCache::new(number_of_words);
        let anagram_vectors = anagram_finder::find_anagrams(&self.dictionary, number_of_words);
        self.solve_for_anagram_vectors(anagram_vectors, permutations, phrase_length)
    }
    
    pub fn find_solutions<'a>(&'a self) -> impl ParallelIterator<Item = Solution> + 'a {
        (1..=self.max_number_of_words).into_par_iter()
            .flat_map(move |number_of_words| {
                self.solve_for_number_of_words(number_of_words)
            })
    }
}
