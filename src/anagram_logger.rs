use crate::dictionary_builder;

pub fn get_anagram_view(anagram: Vec<usize>, dictionary: &dictionary_builder::Dictionary) -> String {
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
