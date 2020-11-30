use crate::dictionary_builder;

pub fn get_anagram_view(anagram: Vec<usize>, dictionary: &dictionary_builder::Dictionary) -> String {
    anagram.iter()
        .map(|&index| {
            let word_options = &dictionary.words[index];
            if word_options.len() == 1 {
                word_options[0].clone()
            } else {
                format!("[{}]", word_options.join(","))
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}