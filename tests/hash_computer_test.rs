use packed_simd::u8x32;

extern crate trustpilot_challenge_rust;
use trustpilot_challenge_rust::hash_computer;

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
    let messages: [&str; 8] = /*[""; 8];*/[
        "DAPUpOGHw620yalJA0vjFPK7ThgHyAN",
        "4xRslaTeBCNyRu2EiIDueEx3BTbIP5H",
        "kFPd2zk60eEFpNwgEOZAcyDcxRVv0Y8",
        "bm6VQr6w9plie0G8XoOb4wChJXB0vCm",
        "gbFrtHcqOTkeG1QxT8YEMSio1ahAYNq",
        "T0GmOLB2WH04oIrhB3JCyPHFxI8UOow",
        "TWUCy0B0JG5KjQvsu4YUFC5IR5ByS2W",
        "VXqOIzYdLIqx6tw8LJbR7SqR5iYgTlQ"
    ];

    let expected: [u128; 8] = [
        0xC6F9E9B203CEA81A7BA28BE276B96A6F,
        0x45BF15D8B08E1AEADE1305B8E43B8F2C,
        0x7AA53B4627C8DD3714F2874EDE04DA7D,
        0x93B650B474B6FDE6B902A76B1DDA10BB,
        0xBBF89511DAC63A516ADDB9BEC79241A5,
        0x7E53E601351A47500BF4A2B7EAD077C3,
        0xFC6FEB2CC3191198E87ECB9D7626580A,
        0xCED45BA82BD1BDCF546255CB6A530FE3,
    ];

    let messages_simd = prepare_messages(messages);

    let hashes = hash_computer::compute_hashes(messages_simd, messages[0].len());

    for i in 0..8 {
        assert_eq!((expected[i] >> 96) as u32, hashes[i].to_be());
    }
}