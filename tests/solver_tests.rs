use rayon::iter::ParallelIterator;

extern crate trustpilot_challenge_rust;
use trustpilot_challenge_rust::solver::Solver;
/*
const WORDS: Vec<String> = vec!["a", "b", "aa", "bb", "ab", "ba", "aaa", "bbb", "abc", "c", "ccc", "d", "dddd"];
const HASHES: [String; _] = [
    "b2ef629aeb4deac769c3b476387c7e9f", // a b c ab c dddd
    "17fd9c64127f42baeff12fba8038a7f0", // ab c dddd abc
    "73b50bfa6b3b941a53f7e3ac0d99c8a5", // c ba ba c dddd
    "c475a98ce0cdfcf5640f981b98d427c3", // c c ab ba dddd
    "96f824d59eea869e02f0dbfa23f5676d", // dddd abc abc
];
*/

fn check_solutions(max_number_of_words: usize, expected: &[&str]) -> () {
    let solver = Solver::create_from_input_data(
        ["a", "b", "aa", "bb", "ab", "ba", "aaa", "bbb", "abc", "c", "ccc", "d", "dddd"].iter()
            .map(|&s| s.to_owned()).collect(),
        [
            "b2ef629aeb4deac769c3b476387c7e9f", // a b c ab c dddd
            "17fd9c64127f42baeff12fba8038a7f0", // ab c dddd abc
            "73b50bfa6b3b941a53f7e3ac0d99c8a5", // c ba ba c dddd
            "c475a98ce0cdfcf5640f981b98d427c3", // c c ab ba dddd
            "96f824d59eea869e02f0dbfa23f5676d", // dddd abc abc
        ].iter().map(|&s| s.to_owned()).collect(),
        max_number_of_words,
        "abcdd cbadd"
    );
    let mut result: Vec<_> = solver.find_solutions()
        .map(|solution| solution.anagram_string)
        .collect();

    result.sort();
    result.dedup();

    assert_eq!(result, expected);
}

#[test]
fn it_solves() {
    check_solutions(1, &[]);
    check_solutions(2, &[]);
    check_solutions(3, &["dddd abc abc"]);
    check_solutions(4, &["ab c dddd abc", "dddd abc abc"]);
    check_solutions(5, &["ab c dddd abc", "c ba ba c dddd", "c c ab ba dddd", "dddd abc abc"]);
    check_solutions(6, &["a b c ab c dddd", "ab c dddd abc", "c ba ba c dddd", "c c ab ba dddd", "dddd abc abc"]);
}
