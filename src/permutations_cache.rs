use permutohedron::Heap;
use crate::permutation_type::get_required_permutation_type;
use crate::permutation_type::get_supported_permutation_types;

pub struct PermutationsCache {
    permutations_by_type: Vec<Vec<Vec<usize>>>,
    set_length: usize,
}

fn invert_permutation(permutation: &Vec<usize>) -> Vec<usize> {
    let mut result = permutation.clone();
    for i in 0..permutation.len() {
        result[permutation[i]] = i;
    }

    result
}

impl PermutationsCache {
    pub fn new(set_length: usize) -> PermutationsCache {
        assert!(set_length <= 16);

        let mut permutations_by_type: Vec<Vec<Vec<usize>>> = Vec::with_capacity(1 << 16);

        for _i in 0..=u16::MAX {
            permutations_by_type.push(Vec::new());
        }

        let mut placeholder: Vec<usize> = (0..set_length).collect();
        let heap = Heap::new(&mut placeholder);
        for permutation in heap {
            for permutation_type in get_supported_permutation_types(&invert_permutation(&permutation)) {
                permutations_by_type[permutation_type as usize].push(permutation.clone());
            }
        }

        PermutationsCache {
            permutations_by_type,
            set_length,
        }
    }

    pub fn get_permuted_vectors<T: Eq + Copy>(&self, ordered_vector_to_permute: &Vec<T>) -> Vec<Vec<T>> {
        //println!("set_length: {}, vector: {:?}", self.set_length, ordered_vector_to_permute);
        assert_eq!(ordered_vector_to_permute.len(), self.set_length);

        let permutation_type = get_required_permutation_type(ordered_vector_to_permute);
        let permutations = &self.permutations_by_type[permutation_type as usize];

        return permutations.iter()
            .map(|permutation| {
                permutation.iter()
                    .map(|&index| ordered_vector_to_permute[index])
                    .collect()
            })
            .collect()
    }
}