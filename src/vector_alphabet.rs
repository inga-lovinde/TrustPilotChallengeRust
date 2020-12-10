use std::collections::HashMap;
use packed_simd;

#[derive(Debug)]
pub struct Vector {
    pub norm: usize,
    simd_vector: packed_simd::u8x32,
}

#[derive(Debug)]
pub struct VectorWithMetadata {
    pub key: String,
    pub vector: Vector,
}

impl Vector {
    fn new(&array: &[u8; 32], norm: usize) -> Vector {
        Vector {
            norm,
            simd_vector: packed_simd::u8x32::from_slice_unaligned(&array),
        }
    }

    pub fn is_subset_of(&self, other: &Vector) -> bool {
        let comparison_result = packed_simd::u8x32::gt(self.simd_vector, other.simd_vector);
        packed_simd::m8x32::none(comparison_result)
    }

    pub fn safe_substract(&self, vector_to_substract: &Vector) -> Option<Vector> {
        if vector_to_substract.is_subset_of(self) {
            return Some(Vector {
                norm: self.norm - vector_to_substract.norm,
                simd_vector: self.simd_vector - vector_to_substract.simd_vector
            });
        } else {
            return None;
        }
    }

    pub fn get_key(&self) -> String {
        format!("{:?}", self.simd_vector)
    }
}

pub struct Alphabet {
    chars_to_offsets: HashMap<char, usize>,
}

impl Alphabet {
    pub fn new(phrase: &str) -> Result<Alphabet, &'static str> {
        let mut chars: Vec<_> = phrase.chars().filter(|&ch| ch != ' ').collect();
        chars.sort();
        chars.dedup();
        
        if chars.len() > 32 {
            return Err("Number of different chars should not exceed 32");
        }

        let mut offsets_to_chars: [char; 32] = [' '; 32];
        let mut chars_to_offsets: HashMap<char, usize> = HashMap::new();
        for (pos, ch) in chars.iter().enumerate() {
            chars_to_offsets.insert(*ch, pos);
            offsets_to_chars[pos] = *ch;
        }

        Ok(Alphabet {
            chars_to_offsets,
        })
    }

    pub fn vectorize(&self, phrase: &str) -> Option<VectorWithMetadata> {
        let mut chars: Vec<_> = phrase.chars().filter(|&ch| ch != ' ').collect();
        chars.sort();

        let norm = chars.len();

        let mut array: [u8; 32] = [0; 32];
        for ch in &chars {
            match self.chars_to_offsets.get(&ch) {
                Some(&index) => {
                    if array[index] >= u8::MAX {
                        return None;
                    }

                    array[index] += 1;
                },
                _ => return None,
            }
        }

        let key: String = chars.into_iter().collect();
        return Some(VectorWithMetadata {
            key,
            vector: Vector::new(&array, norm),
        });
    }
}
