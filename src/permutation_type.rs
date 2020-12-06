use bit_field::BitField;

// permutation type is a bit field,
// 1 at i-th position means that in the permutation,
// value i must occur earlier than the value i+1

// For a given ordered vector (similar to what vec.dedup needs - that is,
// matching values only appear next to each other),
// it should return a permutation type with 1 in position of all values
// which are equal to the next values,
// so that all permutations of this type applied to the vector
// will produce all unique permutations of its values
pub fn get_required_permutation_type<T: Eq>(ordered_vector_to_permute: &[T]) -> u16 {
    let mut result: u16 = 0;
    for i in 0..(ordered_vector_to_permute.len() - 1) {
        if ordered_vector_to_permute[i] == ordered_vector_to_permute[i+1] {
            result.set_bit(i, true);
        }
    }

    result
}

pub fn get_supported_permutation_types(inverse_permutation: &[usize]) -> Vec<u16> {
    assert!(inverse_permutation.len() <= 16);

    let mut max_supported_type: u16 = 0;
    for i in 0..(inverse_permutation.len()-1) {
        if inverse_permutation[i] < inverse_permutation[i+1] {
            max_supported_type.set_bit(i, true);
        }
    }

    let mut supported_types: Vec<u16> = Vec::new();
    for i in 0..=u16::MAX {
        if (i & max_supported_type) == i {
            supported_types.push(i);
        }
    }

    supported_types
}
