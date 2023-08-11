#![allow(clippy::precedence)]

use std::mem::transmute_copy;

#[allow(non_snake_case)]
pub fn lua_bind_hash(data_bytes: &[u8]) -> u64 {
    let mut len = data_bytes.len() as u64;

    let mut hash_vals: [u64; 4] = [
        0x60ea27eeadc0b5d5,
        0xc2b2ae3d27d4eb4e,
        0xffffffffffffffff,
        0x61c8864e7a143578,
    ];

    let mut hash_vals_mid = [0u64; 4];

    let hash_vals_end: [u64; 4] = [
        0x3c6ef3630bd7950e,
        0x1bbcd8c2f5e54380,
        0x779b185ebca87000,
        0xe6c617af2a1c0000,
    ];

    let hash_vals_end_shift: [u64; 4] = [0x3f, 0x39, 0x34, 0x2e];

    const MULT1: u64 = 0x87bcb65480000000;
    const MULT2: u64 = 0xdef35b010f796ca9;
    const MULT3: u64 = 0x9e3779b185ebca87; // -0x61c8864e7a143579
    const MULT4: u64 = 0xc2b2ae3d27d4eb4f; // -0x3d4d51c2d82b14b1
    const MULT5: u64 = 0x93ea75a780000000; // -0x6c158a5880000000
    const MULT6: u64 = 0x27d4eb2f165667c5;
    const ADD1: u64 = 0x85ebca77c2b2ae63;
    const ADD2: u64 = 0x165667b19e3779f9;

    let mut lVar5 = 0x27d4eb2f165667c4;
    let (_, chunks_32, rest) = unsafe { data_bytes.align_to::<[[u8; 8]; 4]>() };
    let (_, chunks_8, rest) = unsafe { rest.align_to::<[u8; 8]>() };
    let (_, chunks_4, data_1) = unsafe { rest.align_to::<[u8; 4]>() };

    if len >= 0x20 {
        for chunk in chunks_32 {
            for (i, block) in chunk.iter().enumerate() {
                hash_vals[i] += unsafe { transmute_copy::<[u8; 8], u64>(block) } * MULT4;
                hash_vals_mid[i] = hash_vals[i] >> 0x21 | hash_vals[i] * 0x80000000;
                hash_vals[i] = hash_vals_mid[i] * MULT3;
            }
        }

        let mut val: u64 = 0;
        for i in 0..4 {
            val += hash_vals_mid[i] * hash_vals_end[i] | hash_vals[i] >> hash_vals_end_shift[i];
        }

        val = (val ^ (hash_vals_mid[0] * MULT1 | hash_vals_mid[0] * MULT2 >> 0x21) * MULT3) * MULT3;
        for i in 1..4 {
            val = (val + ADD1
                ^ (hash_vals_mid[i] * MULT1 | hash_vals_mid[i] * MULT2 >> 0x21) * MULT3)
                * MULT3;
        }
        val += ADD1;

        lVar5 = val;
    }

    len += lVar5;

    for block in chunks_8 {
        let n = unsafe { transmute_copy::<[u8; 8], u64>(block) };
        len ^= (n * MULT5 | (n * MULT4) >> 0x21) * MULT3;
        len = (len >> 0x25 | len << 0x1b) * MULT3 + ADD1;
    }

    if let Some(block) = chunks_4.first() {
        let n = unsafe { transmute_copy::<[u8; 4], u32>(block) };
        len ^= n as u64 * MULT3;
        len = (len >> 0x29 | len << 0x17) * MULT4 + ADD2;
    }

    for n in data_1 {
        len ^= *n as u64 * MULT6;
        len = (len >> 0x35 | len << 0xb) * MULT3;
    }

    let uVar3 = (len ^ len >> 0x21) * MULT4;
    let uVar3 = (uVar3 ^ uVar3 >> 0x1d) * ADD2;

    uVar3 ^ uVar3 >> 0x20
}

pub fn lua_bind_hash_str<S: AsRef<str>>(string: S) -> u64 {
    lua_bind_hash(string.as_ref().as_bytes())
}

fn main() {
    let tests = vec![
        "password",
        "password1234",
        "FIGHTER_STATUS_DAMAGE_WORK_FLOAT_VECOR_CORRECT_STICK_X",
    ];
    for test in tests {
        println!("Hash of '{}' = {:x}", test, lua_bind_hash_str(test));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_test_strings() {
        let tests = vec![
            ("password", 0xe58325ff3537c13a),
            ("password1234", 0xa26a67b576fe1e83),
            ("FIGHTER_STATUS_DAMAGE_WORK_FLOAT_VECOR_CORRECT_STICK_X", 0xa4d50a730e36970e),
        ];
        for test in tests {
            assert_eq!(lua_bind_hash_str(test.0), test.1);
        }
    }

    #[repr(C, align(8))]
    struct AlignBytes<const N: usize>([u8; N]);

    static _ALIGNED: AlignBytes<55> =
        AlignBytes(*b"_FIGHTER_STATUS_DAMAGE_WORK_FLOAT_VECOR_CORRECT_STICK_X");
    static ALIGNED: &str = unsafe { std::str::from_utf8_unchecked(_ALIGNED.0.as_slice()) };

    #[test]
    #[allow(non_snake_case)]
    fn test_misaligned_str() {
        let MISALIGNED = &ALIGNED[1..];
        assert_eq!(MISALIGNED.as_ptr() as usize % 4, 1);
        assert_eq!(lua_bind_hash_str(MISALIGNED), 0xa4d50a730e36970e);
    }
}
