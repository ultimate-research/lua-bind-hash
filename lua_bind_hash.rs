use std::{mem, slice};

#[allow(non_snake_case)]
fn lua_bind_hash(data_bytes: &[u8]) -> u64 {
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
        0xe6c617af2a1c0000
    ];

    let hash_vals_end_shift: [u64; 4] = [
        0x3f,
        0x39,
        0x34,
        0x2e
    ];

    const MULT1: u64 = 0x87bcb65480000000;
    const MULT2: u64 = 0xdef35b010f796ca9;
    const MULT3: u64 = 0x9e3779b185ebca87; // -0x61c8864e7a143579
    const MULT4: u64 = 0xc2b2ae3d27d4eb4f; // -0x3d4d51c2d82b14b1
    const MULT5: u64 = 0x93ea75a780000000; // -0x6c158a5880000000
    const MULT6: u64 = 0x27d4eb2f165667c5;
    const ADD1: u64 = 0x85ebca77c2b2ae63;
    const ADD2: u64 = 0x165667b19e3779f9;
    
    let mut lVar5 = 0x27d4eb2f165667c4;
    let chunks_32 = data_bytes.chunks_exact(32);
    let chunks_8 = chunks_32.remainder().chunks_exact(8);
    let chunks_4 = chunks_8.remainder().chunks_exact(4);
    let data_1 = chunks_4.remainder().iter();
    let data_32 = chunks_32.map(|chunk| unsafe {
                    slice::from_raw_parts(
                        chunk.as_ptr() as *const u64,
                        32usize / mem::size_of::<u64>()
                    )
                  });
    let data_8 = chunks_8.map(|chunk| unsafe { *(chunk.as_ptr() as *const u64) });
    let mut data_4 = chunks_4.map(|chunk| unsafe { *(chunk.as_ptr() as *const u32) });
    
    if len >= 0x20 {
        for chunk in data_32 {
            for i in 0..4 {
                hash_vals[i] += chunk[i] * MULT4;
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
            val = (val + ADD1 ^ (hash_vals_mid[i] * MULT1 | hash_vals_mid[i] * MULT2 >> 0x21) * MULT3) * MULT3;
        }
        val += ADD1;

        lVar5 = val;
    }

    len += lVar5;
    
    for n in data_8 {
        len = (n * MULT5 | (n * MULT4) >> 0x21) *
            MULT3 ^ len;
        len = (len >> 0x25 | len << 0x1b) * MULT3 + ADD1;
    }

    if let Some(n) = data_4.nth(0) {
        len = n as u64 * MULT3 ^ len;
        len = (len >> 0x29 | len << 0x17) * MULT4 + ADD2;
    }

    for n in data_1 {
        len = *n as u64 * MULT6 ^ len;
        len = (len >> 0x35 | len << 0xb) * MULT3;
    }

    let uVar3 = (len ^ len >> 0x21) * MULT4;
    let uVar3 = (uVar3 ^ uVar3 >> 0x1d) * ADD2;

    uVar3 ^ uVar3 >> 0x20
}

fn lua_bind_hash_str<S: AsRef<str>>(string: S) -> u64 {
    lua_bind_hash(string.as_ref().as_bytes())   
}

fn main() {
    let tests = vec![
        "password", 
        "password1234",
        "FIGHTER_STATUS_DAMAGE_WORK_FLOAT_VECOR_CORRECT_STICK_X",
    ];
    for test in tests {
        println!("String - '{}', Hash - {:X}", test, lua_bind_hash_str(test));
    }
}

