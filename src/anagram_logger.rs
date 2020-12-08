use md5;
use packed_simd::u8x32;

fn get_anagram_string_from_simd(simd_vector: u8x32, phrase_length: usize) -> String {
    let mut string_bytes: [u8; 32] = [0; 32];
    simd_vector.write_to_slice_unaligned(&mut string_bytes);

    String::from_utf8_lossy(&string_bytes[0..phrase_length]).into_owned()
}

pub fn log_anagram(simd_vector: u8x32, phrase_length: usize) -> () {
    let anagram_string = get_anagram_string_from_simd(simd_vector, phrase_length);
    let hash = md5::compute(anagram_string.as_bytes());
    println!("{:x} {}", hash, anagram_string);
}
