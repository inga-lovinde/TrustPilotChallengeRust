#![feature(array_map)]

use packed_simd::u8x32;

extern crate trustpilot_challenge_rust;
use trustpilot_challenge_rust::hash_computer;

fn parse_hash(hash: u128) -> u32 {
    ((hash >> 96) as u32).to_be()
}

static MESSAGES: [&str; 8] = [
    "DAPUpOGHw620yalJA0vjFPK7ThgHyAN",
    "4xRslaTeBCNyRu2EiIDueEx3BTbIP5H",
    "kFPd2zk60eEFpNwgEOZAcyDcxRVv0Y8",
    "bm6VQr6w9plie0G8XoOb4wChJXB0vCm",
    "gbFrtHcqOTkeG1QxT8YEMSio1ahAYNq",
    "T0GmOLB2WH04oIrhB3JCyPHFxI8UOow",
    "TWUCy0B0JG5KjQvsu4YUFC5IR5ByS2W",
    "VXqOIzYdLIqx6tw8LJbR7SqR5iYgTlQ"
];

static EXPECTED_HASHES: [u128; 8] = [
    0xC6F9E9B203CEA81A7BA28BE276B96A6F,
    0x45BF15D8B08E1AEADE1305B8E43B8F2C,
    0x7AA53B4627C8DD3714F2874EDE04DA7D,
    0x93B650B474B6FDE6B902A76B1DDA10BB,
    0xBBF89511DAC63A516ADDB9BEC79241A5,
    0x7E53E601351A47500BF4A2B7EAD077C3,
    0xFC6FEB2CC3191198E87ECB9D7626580A,
    0xCED45BA82BD1BDCF546255CB6A530FE3,
];

// just some random u128 numbers
static WRONG_HASHES: [u128; 8] = [
    0xF6999358632CA7EBC3E687329B5B330E,
    0x2DFA25CE0B6E11E794A4BAE87EECBBE4,
    0x24436F08BD36CD5E010C4469D6612462,
    0x78A35F791CA6C57396C1C4CB6A773848,
    0xB6906785D8BD81BAF1AA1E2A9F1D17E9,
    0x7841A7132C2784CE7FAC83AD01A43B45,
    0x10D0C197833BE4365F9D5C8262005C77,
    0x654CB866EE50773F999CD32C123E6C12,
];

static MIXED_HASHES: [u128; 4] = [
    WRONG_HASHES[0],
    EXPECTED_HASHES[3],
    WRONG_HASHES[1],
    EXPECTED_HASHES[6],
];

fn prepare_message(message_string: &str) -> u8x32 {
    let mut bytes_static: [u8; 32] = [0; 32];
    let bytes = message_string.as_bytes();
    for i in 0..bytes.len() {
        bytes_static[i] = bytes[i];
    }

    bytes_static[bytes.len()] = b' ';

    u8x32::from(bytes_static)
}

fn prepare_messages(message_strings: [&str; 8]) -> [u8x32; 8] {
    let mut result: [u8x32; 8] = [u8x32::splat(0); 8];
    for i in 0..8 {
        result[i] = prepare_message(message_strings[i]);
    }
    result
}

#[test]
fn it_computes_hashes() {
    let messages_simd = prepare_messages(MESSAGES);

    let hashes = hash_computer::compute_hashes(&messages_simd, MESSAGES[0].len());
    assert_eq!(EXPECTED_HASHES.map(parse_hash), hashes);
}

#[test]
fn it_verifies_hashes() {
    let messages_simd = prepare_messages(MESSAGES);

    let results = hash_computer::find_hashes(&messages_simd, MESSAGES[0].len(), &(EXPECTED_HASHES.map(parse_hash)));
    assert!(results.is_some());
    assert_eq!(results.unwrap(), messages_simd);

    let results = hash_computer::find_hashes(&messages_simd, MESSAGES[0].len(), &(WRONG_HASHES.map(parse_hash)));
    assert!(results.is_none());

    let results = hash_computer::find_hashes(&messages_simd, MESSAGES[0].len(), &(MIXED_HASHES.map(parse_hash)));
    assert!(results.is_some());
    assert_eq!(results.unwrap(), [messages_simd[3], messages_simd[6]]);
}
