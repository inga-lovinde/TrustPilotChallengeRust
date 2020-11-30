use crate::vector_alphabet;

pub fn find_anagrams(remainder: &vector_alphabet::Vector, remaining_depth: usize, current_words: &[(String, vector_alphabet::VectorWithMetadata)]) -> Vec<Vec<String>> {
    if remaining_depth == 0 {
        if remainder.norm == 0 {
            return vec![vec![]];
        }
        return vec![];
    }

    current_words.iter()
        .enumerate()
        .map(|(index, (word, word_metadata))| match remainder.safe_substract(&word_metadata.vector) {
            Some(new_remainder) => find_anagrams(&new_remainder, remaining_depth-1, &current_words[index..])
                .iter()
                .map(|partial_phrase| {
                    vec![word.clone()].iter().chain(partial_phrase).cloned().collect()
                })
                .collect(),
            _ => vec![],
        })
        .flatten()
        .collect()
}
