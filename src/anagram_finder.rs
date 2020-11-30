use crate::dictionary_builder;
use crate::vector_alphabet;

fn find_anagrams_recursive(remainder: &vector_alphabet::Vector, remaining_depth: usize, word_vectors: &[vector_alphabet::Vector], offset: usize) -> Vec<Vec<usize>> {
    if remaining_depth == 0 {
        if remainder.norm == 0 {
            return vec![vec![]];
        }
        return vec![];
    }

    word_vectors.iter()
        .enumerate()
        .skip(offset)
        .take_while(|(_, vector)| vector.norm * remaining_depth >= remainder.norm)
        .map(|(index, vector)| match remainder.safe_substract(&vector) {
            Some(new_remainder) => find_anagrams_recursive(&new_remainder, remaining_depth-1, word_vectors, index)
                .iter()
                .map(|partial_phrase| {
                    vec![index].iter().chain(partial_phrase).cloned().collect()
                })
                .collect(),
            _ => vec![],
        })
        .flatten()
        .collect()
}

pub fn find_anagrams(dictionary: &dictionary_builder::Dictionary, number_of_words: usize) -> Vec<Vec<usize>> {
    find_anagrams_recursive(&dictionary.phrase_vector, number_of_words, &dictionary.vectors, 0)
}
