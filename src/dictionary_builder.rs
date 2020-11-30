use std::collections::HashMap;
use crate::vector_alphabet;

pub struct Dictionary {
    pub phrase_vector: vector_alphabet::Vector,
    pub vectors: Vec<vector_alphabet::Vector>,
    pub words: Vec<Vec<String>>,
}

pub fn build_dictionary(phrase: &String, unique_words: &[String]) -> Dictionary {
    let alphabet = vector_alphabet::Alphabet::new(phrase).unwrap();

    let phrase_with_metadata = alphabet.vectorize(phrase).unwrap();

    let words_with_vectors: Vec<_> = unique_words
        .into_iter()
        .map(|word| {
            let vector_option = alphabet.vectorize(&word);
            match vector_option {
                Some(vector_with_metadata) => {
                    if vector_with_metadata.vector.is_subset_of(&phrase_with_metadata.vector) {
                        return Some((word, vector_with_metadata));
                    } else {
                        return None;
                    }
                }
                None => {
                    return None;
                }
            }
        })
        .flatten()
        .collect();

    let mut words_by_vectors: HashMap<_, _> = HashMap::new();
    for (word, vector_with_metadata) in words_with_vectors {
        let (_, words_for_vector) = words_by_vectors.entry(vector_with_metadata.key).or_insert((vector_with_metadata.vector, vec![]));
        words_for_vector.push(word.clone());
    }

    let mut words_by_vectors: Vec<_> = words_by_vectors.into_values().collect();
    words_by_vectors.sort_by_key(|(vector, _)| vector.norm);
    words_by_vectors.reverse();

    let mut vectors = vec![];
    let mut words_by_vectors_vec = vec![];

    for (vector, words_by_vector) in words_by_vectors {
        vectors.push(vector);
        words_by_vectors_vec.push(words_by_vector);
    }

    Dictionary {
        phrase_vector: phrase_with_metadata.vector,
        vectors,
        words: words_by_vectors_vec,
    }
}
