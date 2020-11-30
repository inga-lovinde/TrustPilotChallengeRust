# TrustPilotChallengeRust

TrustPilot had this challenge several years ago
(http://followthewhiterabbit.trustpilot.com/)
where you had to, given the dictionary, and given three MD5 hashes,
find three-word anagrams of a phrase *"poultry outwits ants"*
which result in these hashes.

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

## Algorithm description

Notably this solution does not involve string concatenation; strings are only concatenated for debugging purposes.

We could split the problem into three parts: finding all anagrams
(up to words reordering and replacing some of the words with their single-word anagrams),
finding all anagrams taking into account words order,
and checking their hashes against the supplied list.

### Finding all anagrams, pt. 1

For every string (ignoring spaces) we could define a vector in Z^N space, with its i-th coordinate
matching the number of occurrences of character i in the string.

Two strings are anagrams of each other if and only if their vectors are the same.

Vector for a concatenation of two strings is the sum of vectors for these two strings.

This means that the task of finding anagrams for a phrase reduces to the task of finding
subsets of vectors (out of sets of vectors for all dictionary words) which add up
to the vector for original phrase.
Since all coordinates are positive, only vectors which are contained in a hyperrectangle
defined by the target vector (that is, which have all coordinates not larger
than the target vector; that is, corresponding words for which are subsets of the target phrase)
could belong to such subsets.

Additionally, if the source phrase contains no more than 32 different characters,
and each of these no more than 255 times, we could limit ourselves to u8x32 vectors
instead of vectors in Z^N.
That way we can "concatenate" strings or "compare" them for anagrams in a single CPU cycle.

The naive solution of finding fixed-length subsets of vectors which add up to a given vector
could be further optimized, resulting in the following algorithm:

1. Sort all vectors by their norm (length of the original word), largest first;
2. Find all target subsets such that the order of items in subset is compatible with their order in sorted vectors list
2. For number of words N, the requested phrase P, and the offset K (originally 0) check:
    * If N is 0 and P is non-zero, there are no solutions;
    * If N is 0 and P is zero, there is a trivial solution (empty subset);
    * If N is larger than 0, let us find the first vector of a target subset:
        * For every vector W starting with offset K
            (while its norm times N is less than the norm of P)
            * If the norm of W is not larger than the norm of P and all coordinates of W are not larger than of P:
                * W might be one element of a target subset, and the remaining elements could be found
                    by solving the task 2 for N-1, P-W and position of W in the list of vectors.

## How to run

How to run to solve the original task for three-word anagrams:

```
cargo run data\words.txt data\hashes.txt 3 "poultry outwits ants"
```

(Note that CPU with AVX2 support is required; that is, Intel Haswell (2013) or newer, or AMD Excavator (2015) or newer)