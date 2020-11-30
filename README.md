# TrustPilotChallengeRust

TrustPilot had this challenge (http://followthewhiterabbit.trustpilot.com/)
where you had to, given the dictionary, and given three MD5 hashes,
find anagrams of a phrase *"poultry outwits ants"* which result in these hashes.

My original solution was in mixture of C# and plain C (with a bit of Visual C++
as a bridge), and heavily used AVX2 intrinsics for optimization.

Rust now has a decent API frontend for AVX2 intrinsics 
(https://rust-lang.github.io/packed_simd/packed_simd_2/, and soon-to-be `std::simd`),
so it makes perfect sense to try and reimplement the same ideas with Rust.

The problem will sound a bit different: given a dictionary and given a string,
find all anagrams no longer than N words and no longer than 27 bytes
which produce given MD5 hashes.

(The limit on the number of words is neccessary, because there are single-letter words
in the dictionary; and it makes the total number of anagrams astronomically large)

This is a working draft, so far the code is extremely dirty (this is my first Rust project),
and it only lists all anagrams (not including words reordering)
and does not yet do actual MD5 calculation.

How to run to solve the original task for three-word anagrams:

```
cargo run data\words.txt data\hashes.txt 3 "poultry outwits ants"
```
