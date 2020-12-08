use packed_simd::u8x32;
use crate::dictionary_builder::Dictionary;
use crate::dictionary_builder::WordInfo;
use crate::permutations_cache::PermutationsCache;

fn get_anagram_view_from_simd(simd_vector: u8x32, phrase_length: usize) -> String {
    let mut string_bytes: [u8; 32] = [0; 32];
    simd_vector.write_to_slice_unaligned(&mut string_bytes);

    String::from_utf8_lossy(&string_bytes[0..phrase_length]).into_owned()
}

fn generate_vector_substitutions<'a>(simple_dictionary: &'a Vec<Vec<&WordInfo>>, permutation: &'a [usize], current_phrase: u8x32, current_phrase_length: usize) -> Box<dyn Iterator<Item = u8x32> + 'a> {
    if permutation.len() == 0 {
        return Box::new(std::iter::once(current_phrase.clone()));
    }

    let result = simple_dictionary[permutation[0]].iter()
        .flat_map(move |&word_info| {
            generate_vector_substitutions(&simple_dictionary, &permutation[1..], current_phrase ^ word_info.get_simd_word_for_offset(current_phrase_length), current_phrase_length + word_info.length + 1).into_iter()
        });
    return Box::new(result);
}

pub fn log_anagrams(anagram_vector: &Vec<usize>, dictionary: &Dictionary, permutations: &PermutationsCache, phrase_length: usize) -> () {
    let simple_vector: Vec<usize> = (0..anagram_vector.len()).collect();
    let simple_dictionary: Vec<Vec<&WordInfo>> = (0..anagram_vector.len())
        .map(|i| dictionary.words[anagram_vector[i]].iter().map(|word_info| word_info).collect())
        .collect();

    permutations.get_permuted_vectors(&simple_vector).iter()
        .flat_map(|permuted_vector| {
            generate_vector_substitutions(&simple_dictionary, &permuted_vector, u8x32::splat(0), 0)
        })
        .for_each(|anagram| {
            println!("{}", get_anagram_view_from_simd(anagram, phrase_length));
        })
}