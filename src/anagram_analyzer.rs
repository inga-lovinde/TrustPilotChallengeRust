use packed_simd::u8x32;
use crate::anagram_logger::log_anagram;
use crate::dictionary_builder::Dictionary;
use crate::hash_computer::CHUNK_SIZE;
use crate::hash_computer::find_hashes;
use crate::permutations_cache::PermutationsCache;

fn generate_vector_substitutions<'a>(simple_dictionary: &'a Dictionary, permutation: &'a [usize], current_phrase: u8x32, current_phrase_length: usize) -> Box<dyn Iterator<Item = u8x32> + 'a> {
    if permutation.len() == 0 {
        return Box::new(std::iter::once(current_phrase.clone()));
    }

    let result = simple_dictionary.words[permutation[0]].iter()
        .flat_map(move |word_info| {
            generate_vector_substitutions(&simple_dictionary, &permutation[1..], current_phrase ^ word_info.get_simd_word_for_offset(current_phrase_length), current_phrase_length + word_info.length + 1).into_iter()
        });
    return Box::new(result);
}

fn process_anagram_chunk(chunk: &[u8x32; CHUNK_SIZE], phrase_length: usize, hashes_to_find: &[u32]) -> () {
    match find_hashes(chunk, phrase_length, hashes_to_find) {
        Some(anagrams) => {
            for anagram in anagrams {
                log_anagram(anagram, phrase_length);
            }
        }
        _ => ()
    }
}

pub fn analyze_anagrams(anagram_vector: &Vec<usize>, dictionary: &Dictionary, permutations: &PermutationsCache, phrase_length: usize, hashes_to_find: &[u32]) -> () {
    let mut chunk: [u8x32; CHUNK_SIZE] = [u8x32::splat(0); CHUNK_SIZE];
    let mut chunk_position: usize = 0;

    permutations.get_permuted_vectors(&anagram_vector).iter()
        .flat_map(|permuted_vector| {
            generate_vector_substitutions(&dictionary, &permuted_vector, u8x32::splat(0), 0)
        })
        .for_each(|anagram| {
            chunk[chunk_position] = anagram;
            chunk_position = (chunk_position + 1) % CHUNK_SIZE;
            if chunk_position == 0 {
                process_anagram_chunk(&chunk, phrase_length, hashes_to_find);
            }
        });

    process_anagram_chunk(&chunk, phrase_length, hashes_to_find);
}