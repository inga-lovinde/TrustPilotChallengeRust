# TrustPilotChallengeRust

TrustPilot had this challenge several years ago
(http://followthewhiterabbit.trustpilot.com/)
where you had to, given the dictionary, and given three MD5 hashes,
find three-word anagrams of a phrase *"poultry outwits ants"*
which result in these hashes.

My [original solution](https://github.com/inga-lovinde/TrustPilotChallenge)
was in mixture of C# and plain C (with a bit of Visual C++
as a bridge), and heavily used AVX2 intrinsics for optimization.

Rust now has a decent API frontend for AVX2 intrinsics 
(https://rust-lang.github.io/packed_simd/packed_simd_2/, and soon-to-be `std::simd`),
so it makes perfect sense to try and reimplement the same ideas with Rust.

The problem will sound a bit different: given a dictionary and given a string,
find all anagrams no longer than N words and no longer than 27 bytes
which produce given MD5 hashes.

(The limit on the number of words is neccessary, because there are single-letter words
in the dictionary; and it makes the total number of anagrams astronomically large.)

Note that this is my first Rust project.

## Algorithm description

Notably this solution does not involve string concatenation;
strings are only concatenated for debugging purposes.
It also computes eight MD5 hashes at a time *per thread*
(that is, 128 MD5 hashes at once on a modern 8-core CPU),
with some further optimizations which further shave off
several percents from MD5 computation time.
(md5 crate dependency is only used to nicely print results)

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

### Finding all anagrams, pt. 2

In the previous step, we just found all unique tuples of vectors with vectors ordered by norm decreasing
such that they give the required vector.
Now we need to convert these back to phrases.

If for every vector there was only one word which produces that vector,
and if all the vectors in a tuple were different,
we could just look at all their permutations and get n! solutions from a single tuple.

But a tuple can contain several copies of one vector,
and there could be several different words corresponding to one vector.
Computing all possible permutations would result in duplicate solutions
and too much unneccessary work.

So we could:

1. Substitute all possible word values for every vector, getting several (ordered) word solutions;
2. Apply all possible permutations to them such that, if vectors k and k+1 were the same in the vector solution,
    word k should go before word k+1 in the word solution
    (because the solution where word k goes after word k+1 is already obtained by a different substitution on step 1).

Every string shorter than 32 bytes could be represented as a single u8x32 AVX2 register
(with the remaining bytes filled with zeroes).

Concatenating strings could be as simple as XORing the vectors, shifted appropriately.

For example, to create `"a b "` string we would need to compute `"a " xor "␀␀b "`,
which is done in a single cycle on a modern CPU, provided that we have both vectors ready.
This is as opposed to concatenating strings which would require allocating a new string on the heap
and copying the data.

So we could just store all of the original words as such a vectors for all possible offsets
(along with trailing spaces), and when we need to compute a phrase consisting of the word x and the word y,
just do something along the lines of `get_register(x, 0) xor get_register(y, x.length)`

### Computing hashes

MD5 works on input messages in 64 byte blocks; for short strings (shorter than 55 bytes)
it only uses a single blocks: 0x80 byte is appended to the message, then it is padded to 56 bytes with zeroes,
and then the total length of the string in bits is appended as 64-bit number.

So short phrases (shorter than 31 bytes) could be represented with two AVX2 registers:
one containing the phrase itself with the trailing 0x80, and another containing 24 zeroes
and 64-bit length of the phrase in bits (which is the number of non-space bytes
plus the number of words, times 8).

For its internal state, MD5 has four 32-bit variables (u32).
This means that with AVX2, we can use the same operations on 256-bit registers
(u32x8) and compute eight hashes at the same time in a single thread.

MD5 breaks input chunks into 16 u32 words (and for short phrases chunks 8-14 are always zero),
so our algorithm could receive 8x256-bit values and the phrase length,
rearrange these into 9 256-bit values (8 obtained by transposing the original 8 as 8x8 matrix of u32,
and ninth being 8 copies of the phrase length in bits),
and then implement MD5 algorithms using these 9 values as input words 0..7, 15
(substituting 0 as input words 8..14).

That way, MD5 performance would be increased 8x compared to the ordinary library function
which does not use SIMD.

As a minor additional optimization, we could only compute the first u32 part of the MD5 hash
(because we don't need to compute entire hashes for all possible anagrams,
we only need to find anagrams which match the requested hashes.
That way, we'll save some unneeded steps in MD5 computation,
and we also won't have to convert hashes back to separate variables:
we could just compare u32x8 holding the first parts of hashes for eight different anagrams
with u32x8 holding eight copies of the first part of the requested hash.
That way, we'll only have one comparison instead of eight,
at the cost of rare false positives which occur on average with 1/2^29 probability
(1/2^32 chance that a random u32 matches the requested u32, for every of the eight anagrams).
If there is such a semi-match (that is, one of the eight anagrams produces a hash
with first 32 bits matching first 32 bits of the requested hash), we could just
compute MD5 for every of the eight anagrams in the ordinary way and
to compare the whole resulting hashes with the requested ones;
as this is extremely rare (once every 1/29th calls to SIMD MD5 function),
it will not severely affect performance.

## How to run

How to run to solve the original task for three-word anagrams:

```
cargo run data\words.txt data\hashes.txt 4 "poultry outwits ants"
```

(Note that CPU with AVX2 support is required; that is, Intel Haswell (2013) or newer, or AMD Excavator (2015) or newer.)

In addition to the right solutions it will also output some wrong ones,
because for performance and transparency reasons only the first 8 bytes of hashes are compared.
This means that for every requested hash there is 1/1^32 chance of collision,
so for 10 requested hashes you will get one false positive every 430 millions of anagrams, on average,
which allows one to roughly measure the perfomance of MD5 calculation.