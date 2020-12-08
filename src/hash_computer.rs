use packed_simd::FromBits;
use packed_simd::u32x8;
use packed_simd::u8x32;

pub const MAX_PHRASE_LENGTH: usize = 31;
pub const CHUNK_SIZE: usize = 8;

#[allow(unused_assignments)]
fn compute_hashes_vector(messages: &[u8x32; CHUNK_SIZE], messages_length: usize) -> u32x8 {
    let mut a: u32x8 = u32x8::splat(0x67452301);
    let mut b: u32x8 = u32x8::splat(0xefcdab89);
    let mut c: u32x8 = u32x8::splat(0x98badcfe);
    let mut d: u32x8 = u32x8::splat(0x10325476);

    let trailer = u8x32::splat(0).replace(messages_length, b' ' ^ 0x80);
    let mut messages_bytes: [u32; 64] = [0; 64];
    {
        macro_rules! write_bytes {
            ($i: expr) => {
                u32x8::from_bits(messages[$i] ^ trailer).write_to_slice_unaligned(&mut messages_bytes[($i*8)..])
            }
        }
        write_bytes!(0);
        write_bytes!(1);
        write_bytes!(2);
        write_bytes!(3);
        write_bytes!(4);
        write_bytes!(5);
        write_bytes!(6);
        write_bytes!(7);
    }

    macro_rules! get_m_value {
        ($i: expr) => {
            u32x8::new(
                messages_bytes[0*8 + $i],
                messages_bytes[1*8 + $i],
                messages_bytes[2*8 + $i],
                messages_bytes[3*8 + $i],
                messages_bytes[4*8 + $i],
                messages_bytes[5*8 + $i],
                messages_bytes[6*8 + $i],
                messages_bytes[7*8 + $i],
            )
        };
    }

    let m0: u32x8 = get_m_value!(0);
    let m1: u32x8 = get_m_value!(1);
    let m2: u32x8 = get_m_value!(2);
    let m3: u32x8 = get_m_value!(3);
    let m4: u32x8 = get_m_value!(4);
    let m5: u32x8 = get_m_value!(5);
    let m6: u32x8 = get_m_value!(6);
    let m7: u32x8 = get_m_value!(7);
    let m14: u32x8 = u32x8::splat((messages_length as u32) * 8);

    macro_rules! lrot {
        ($f: expr, $s: expr) => (($f << $s) | ($f >> (32-$s)));
    }

    macro_rules! blend {
        ($mask: expr, $a: expr, $b: expr) => {
            // andnot (_mm256_andnot_si256) is not implemented in packed_simd
            ($a & $mask) | ($b & !$mask)
        }
    }

    macro_rules! step {
        ($f: expr, $s: expr, $k: expr, $m: expr) => {
            let f = $f + a + u32x8::splat($k) + $m;
            a = d;
            d = c;
            c = b;
            b = b + lrot!(f, $s);
        };
        ($f: expr, $s: expr, $k: expr) => {
            let f = $f + a + u32x8::splat($k);
            a = d;
            d = c;
            c = b;
            b = b + lrot!(f, $s);
        };
    }

    {
        macro_rules! step_1 {
            () => (blend!(b, c, d));
        }

        step!(step_1!(),  7, 0xd76aa478, m0);
        step!(step_1!(), 12, 0xe8c7b756, m1);
        step!(step_1!(), 17, 0x242070db, m2);
        step!(step_1!(), 22, 0xc1bdceee, m3);
        step!(step_1!(),  7, 0xf57c0faf, m4);
        step!(step_1!(), 12, 0x4787c62a, m5);
        step!(step_1!(), 17, 0xa8304613, m6);
        step!(step_1!(), 22, 0xfd469501, m7);
        step!(step_1!(),  7, 0x698098d8);
        step!(step_1!(), 12, 0x8b44f7af);
        step!(step_1!(), 17, 0xffff5bb1);
        step!(step_1!(), 22, 0x895cd7be);
        step!(step_1!(),  7, 0x6b901122);
        step!(step_1!(), 12, 0xfd987193);
        step!(step_1!(), 17, 0xa679438e, m14);
        step!(step_1!(), 22, 0x49b40821);
    }

    {
        macro_rules! step_2 {
            () => (blend!(d, b, c));
        }

        step!(step_2!(),  5, 0xf61e2562, m1);
        step!(step_2!(),  9, 0xc040b340, m6);
        step!(step_2!(), 14, 0x265e5a51);
        step!(step_2!(), 20, 0xe9b6c7aa, m0);
        step!(step_2!(),  5, 0xd62f105d, m5);
        step!(step_2!(),  9, 0x02441453);
        step!(step_2!(), 14, 0xd8a1e681);
        step!(step_2!(), 20, 0xe7d3fbc8, m4);
        step!(step_2!(),  5, 0x21e1cde6);
        step!(step_2!(),  9, 0xc33707d6, m14);
        step!(step_2!(), 14, 0xf4d50d87, m3);
        step!(step_2!(), 20, 0x455a14ed);
        step!(step_2!(),  5, 0xa9e3e905);
        step!(step_2!(),  9, 0xfcefa3f8, m2);
        step!(step_2!(), 14, 0x676f02d9, m7);
        step!(step_2!(), 20, 0x8d2a4c8a);
    }

    {
        macro_rules! step_3 {
            () => (b ^ (c ^ d));
        }

        step!(step_3!(),  4, 0xfffa3942, m5);
        step!(step_3!(), 11, 0x8771f681);
        step!(step_3!(), 16, 0x6d9d6122);
        step!(step_3!(), 23, 0xfde5380c, m14);
        step!(step_3!(),  4, 0xa4beea44, m1);
        step!(step_3!(), 11, 0x4bdecfa9, m4);
        step!(step_3!(), 16, 0xf6bb4b60, m7);
        step!(step_3!(), 23, 0xbebfbc70);
        step!(step_3!(),  4, 0x289b7ec6);
        step!(step_3!(), 11, 0xeaa127fa, m0);
        step!(step_3!(), 16, 0xd4ef3085, m3);
        step!(step_3!(), 23, 0x04881d05, m6);
        step!(step_3!(),  4, 0xd9d4d039);
        step!(step_3!(), 11, 0xe6db99e5);
        step!(step_3!(), 16, 0x1fa27cf8);
        step!(step_3!(), 23, 0xc4ac5665, m2);
    }

    {
        macro_rules! step_4 {
            () => (c ^ (b | !d));
        }

        step!(step_4!(),  6, 0xf4292244, m0);
        step!(step_4!(), 10, 0x432aff97, m7);
        step!(step_4!(), 15, 0xab9423a7, m14);
        step!(step_4!(), 21, 0xfc93a039, m5);
        step!(step_4!(),  6, 0x655b59c3);
        step!(step_4!(), 10, 0x8f0ccc92, m3);
        step!(step_4!(), 15, 0xffeff47d);
        step!(step_4!(), 21, 0x85845dd1, m1);
        step!(step_4!(),  6, 0x6fa87e4f);
        step!(step_4!(), 10, 0xfe2ce6e0);
        step!(step_4!(), 15, 0xa3014314, m6);
        step!(step_4!(), 21, 0x4e0811a1);
        step!(step_4!(),  6, 0xf7537e82, m4);

        // Since we ignore b, c, d values in the end,
        // the remaining three iterations are unnecessary,
        // as the value of a after iteration 64 is equal
        // to the value of b after iteration 61
        return b + u32x8::splat(0x67452301);

    }
}

pub fn compute_hashes(messages: &[u8x32; CHUNK_SIZE], messages_length: usize) -> [u32; CHUNK_SIZE] {
    let hashes_vector = compute_hashes_vector(messages, messages_length);
    let mut result: [u32; CHUNK_SIZE] = [0; CHUNK_SIZE];
    hashes_vector.write_to_slice_unaligned(&mut result);
    result
}

pub fn find_hashes(messages: &[u8x32; CHUNK_SIZE], messages_length: usize, hashes_to_find: &[u32]) -> Option<Vec<u8x32>> {
    let hashes_vector = compute_hashes_vector(messages, messages_length);

    let has_matches: bool = hashes_to_find.iter()
        .any(|&hash| hashes_vector.eq(u32x8::splat(hash)).any());

    if !has_matches {
        return None;
    }

    let mut result: Vec<_> = Vec::new();
    for i in 0..CHUNK_SIZE {
        let hash = hashes_vector.extract(i);
        if hashes_to_find.contains(&hash) {
            result.push(messages[i]);
        }
    }

    Some(result)
}
