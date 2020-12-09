use std::collections::HashMap;
use packed_simd::u8x32;
use crate::vector_alphabet;

pub struct WordInfo {
    simd_words: [u8x32; 32],
    pub length: usize,
    pub word: String,
}

impl WordInfo {
    fn new(word: String) -> WordInfo {
        let mut byte_array: [u8; 64] = [0; 64];
        let bytes = word.as_bytes();
        let length = bytes.len();
        byte_array[32 + length] = b' ';
        for i in 0..length {
            byte_array[32 + i] = bytes[i];
        }

        let simd_word_zero: u8x32 = u8x32::from_slice_unaligned(&[0; 32]);
        let mut simd_words: [u8x32; 32] = [simd_word_zero; 32];
        for i in 0..31 {
            simd_words[i] = u8x32::from_slice_unaligned(&byte_array[32-i..64-i]);
        }

        WordInfo {
            simd_words,
            length,
            word,
        }
    }

    pub fn get_simd_word_for_offset(&self, offset: usize) -> u8x32 {
        self.simd_words[offset]
    }
}

pub struct Dictionary {
    pub phrase_vector: vector_alphabet::Vector,
    pub vectors: Vec<vector_alphabet::Vector>,
    pub words: Vec<Vec<WordInfo>>,
}

impl Dictionary {
    pub fn from_phrase_and_words(phrase: &str, unique_words: Vec<String>) -> Dictionary {
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
            words_for_vector.push(WordInfo::new(word));
        }

        let mut words_by_vectors: Vec<_> = words_by_vectors.into_values().collect();
        words_by_vectors.sort_by_cached_key(|(vector, _)| (vector.norm, vector.get_key()));
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
}
