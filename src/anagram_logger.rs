use crate::dictionary_builder::Dictionary;
use crate::dictionary_builder::WordInfo;
use crate::permutations_cache::PermutationsCache;

pub fn get_anagram_view(anagram: &Vec<usize>, dictionary: &Dictionary) -> String {
    anagram.iter()
        .map(|&index| {
            let word_options = &dictionary.words[index];
            if word_options.len() == 1 {
                word_options[0].word.clone()
            } else {
                format!("[{}]", word_options.iter().map(|word_info| word_info.word.clone()).collect::<Vec<_>>().join(","))
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}

fn generate_substitutions<T: Copy>(simple_dictionary: &[Vec<T>], remaining_length: usize) -> Vec<Vec<T>> {
    if remaining_length == 0 {
        return vec![Vec::new()];
    }

    return simple_dictionary[remaining_length - 1].iter()
        .flat_map(|&value| {
            generate_substitutions(simple_dictionary, remaining_length - 1).into_iter()
                .map(move |mut partial_substitution| {
                    partial_substitution.push(value);
                    partial_substitution
                })
        })
        .collect();
}

pub fn log_anagrams(anagram_vector: &Vec<usize>, dictionary: &Dictionary, permutations: &PermutationsCache) -> () {
    let simple_vector: Vec<usize> = (0..anagram_vector.len()).collect();
    let simple_dictionary: Vec<Vec<&WordInfo>> = (0..anagram_vector.len())
        .map(|i| dictionary.words[anagram_vector[i]].iter().map(|word_info| word_info).collect())
        .collect();
    let substitutions: Vec<Vec<&WordInfo>> = generate_substitutions::<&WordInfo>(&simple_dictionary, simple_dictionary.len());

    permutations.get_permuted_vectors(&simple_vector).iter()
        .flat_map(|permuted_vector| {
            substitutions.iter().map(move |substitution| {
                permuted_vector.iter().map(|&index| substitution[index]).collect::<Vec<_>>()
            })
        })
        .for_each(|anagram| {
            let phrase = anagram.iter()
                .map(|word_info| word_info.word.clone())
                .collect::<Vec<_>>()
                .join(" ");
            println!("{}", phrase);
        })
}