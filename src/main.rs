use std::env;

mod anagram_finder;
mod hash_computer;
mod read_lines;
mod vector_alphabet;

fn main() {
    let args: Vec<_> = env::args().collect();
    println!("{:?}", args);

    let words_file_path = &args[1];
    let hashes_file_path = &args[2];
    let max_number_of_words = &args[3].parse::<usize>().unwrap();
    let phrase = &args[4];

    let phrase_byte_length_without_spaces = phrase.as_bytes().into_iter().filter(|&b| *b != b' ').count();
    let result_byte_length = phrase_byte_length_without_spaces + max_number_of_words - 1;

    if result_byte_length > hash_computer::MAX_PHRASE_LENGTH {
        panic!("Words number limit exceeded")
    }

    let alphabet = vector_alphabet::Alphabet::new(phrase).unwrap();

    let phrase_with_metadata = alphabet.vectorize(phrase).unwrap();

    let mut words = read_lines::lines_from_file(words_file_path).unwrap();
    words.sort();
    words.dedup();

    let words_with_vectors: Vec<_> = words
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

    let result = anagram_finder::find_anagrams(&phrase_with_metadata.vector, *max_number_of_words, &words_with_vectors);
    for result_words in result {
        println!("{}", result_words.join(" "))
    }
}
