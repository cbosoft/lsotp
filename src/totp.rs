use std::time::SystemTime;

use sha1::{Sha1, Digest};

const BLOCK_SIZE: usize = 64;


fn clean_k(k: Vec<u8>) -> [u8; BLOCK_SIZE] {
    let mut k_sized: [u8; BLOCK_SIZE] = [0; BLOCK_SIZE];
    if k.len() > BLOCK_SIZE {
        let res = Sha1::digest(k);
        for i in 0..res.len() {
            k_sized[i] = res[i];
        }
    }
    else {
        for i in 0..k.len() {
            k_sized[i] = k[i];
        }
    }

    k_sized
}


fn do_xor(arr: [u8; BLOCK_SIZE], v: u8) -> [u8; BLOCK_SIZE] {
    let mut rv = [0u8; BLOCK_SIZE];
    for i in 0..BLOCK_SIZE {
        rv[i] = arr[i] ^ v;
    }
    rv
}


fn hmac(k_raw: Vec<u8>, m: Vec<u8>) -> Vec<u8> {
    let k = clean_k(k_raw);
    let k_i = do_xor(k, 0x36);
    let k_o = do_xor(k, 0x5c);

    let ihash = Sha1::new()
        .chain_update(k_i)
        .chain_update(m)
        .finalize();

    let ohash = Sha1::new()
        .chain_update(k_o)
        .chain_update(ihash)
        .finalize();

    ohash.to_vec()
}

const POWERS_TEN: [u32; 3] = [
    1_000_000, 10_000_000, 100_000_000
];


fn hotp(k: Vec<u8>, c_num: u64, n: usize) -> String {
    let mut c_arr = [0u8; 8];
    let mut running_c_num = c_num;
    for j in 0..8 {
        let i = 7 - j;
        c_arr[i] = (running_c_num & 255u64) as u8;
        running_c_num >>= 8;
    }

    let c_bytes = c_arr.to_vec();

    let hs = hmac(k, c_bytes);
    let offset = (hs[hs.len()-1] & 0xfu8) as usize;

    let snum = (
        (((hs[offset] & 0x7f) as u32) << 24)
        | (((hs[offset+1] & 0xff) as u32) << 16)
        | (((hs[offset+2] & 0xff) as u32) << 8)
        | ((hs[offset+3] & 0xff) as u32)
        ) % POWERS_TEN[n - 6];

    format!("{:0>n$}", snum)
}


pub fn totp(k: Vec<u8>, t0: u64, x: u64, n: usize)-> String {
    let unix_time = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_secs();

    let adjusted_t = (unix_time - t0) / x;

    hotp(k, adjusted_t, n)
}

pub fn sane_totp(secret: String) -> String {
    let secret = secret.to_lowercase();
    let secret_decoded = koibumi_base32::decode(secret).expect("Failed to decode secret. Is it valid base 32?");
    totp(secret_decoded, 0, 30, 6)
}
