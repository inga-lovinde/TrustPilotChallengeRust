use md5;
use packed_simd::u8x32;
use crate::dictionary_builder::Dictionary;

#[derive(Debug)]
pub struct Solution {
    pub anagram_string: String,
}

impl Solution {
    pub fn from_simd(simd_vector: u8x32, phrase_length: usize) -> Solution {
        let mut string_bytes: [u8; 32] = [0; 32];
        simd_vector.write_to_slice_unaligned(&mut string_bytes);
        let anagram_string = String::from_utf8_lossy(&string_bytes[0..phrase_length]).into_owned();

        Solution {
            anagram_string,
        }
    }

    pub fn get_hash(&self) -> String {
        format!("{:x}", md5::compute(self.anagram_string.as_bytes()))
    }
}

pub fn get_anagram_vector_view(anagram: &Vec<usize>, dictionary: &Dictionary) -> String {
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

pub fn log_anagram(simd_vector: u8x32, phrase_length: usize) -> () {
    println!("{}", Solution::from_simd(simd_vector, phrase_length).anagram_string);
}
