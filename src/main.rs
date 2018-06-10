extern crate byteorder;
extern crate crypto_hash;
extern crate hex;

use byteorder::{LittleEndian, WriteBytesExt};
use crypto_hash::{Algorithm, Hasher};
use std::io::Write;
use std::str;
use std::vec::Vec;
use std::{thread, time};

fn send(msg: &str, payload: String) {
    let magic = hex::decode("fabfb5da").unwrap();
    let command = pad_command(msg.as_bytes());
    let payload_raw = hex::decode(payload).unwrap();
    let p_len = payload_len(&payload_raw);
    let checksum = checksum(&payload_raw);

    let mut message: Vec<u8> = vec![];
    message.extend(&magic);
    message.extend(&command);
    message.extend(&p_len);
    message.extend(&checksum[..4]);
    message.extend(&payload_raw);

    unsafe {
        print!("{}", String::from_utf8_unchecked(message));
    }

    thread::sleep(time::Duration::from_secs(5))
}

fn pad_command(cmd: &[u8]) -> Vec<u8> {
    let mut result = [0; 12];

    for i in 0..cmd.len() {
        result[i] = cmd[i];
    }

    result.to_vec()
}

fn checksum(payload: &[u8]) -> Vec<u8> {
    let mut hasher1 = Hasher::new(Algorithm::SHA256);
    let _ = hasher1.write_all(&payload);
    let hash1 = hasher1.finish();

    let mut hasher2 = Hasher::new(Algorithm::SHA256);
    let _ = hasher2.write_all(&hash1);
    let result = hasher2.finish();

    result
}

fn payload_len(payload: &Vec<u8>) -> Vec<u8> {
    let size = payload.len();
    let mut result = vec![];
    result.write_u32::<LittleEndian>(size as u32).unwrap();

    result
}

fn main() {
    let proto_version = String::from("7d110100");
    let services = String::from("0000000000000000");
    let time = String::from("c6925e5400000000");
    let recv_addr = String::from("000000000000000000000000000000000000ffff7f000001480c");
    let send_addr = String::from("000000000000000000000000000000000000ffff7f000001480c");
    let nonce = String::from("0000000000000000");
    let user_agent_len = String::from("0a");
    let user_agent = String::from(hex::encode("/rbc:alpha/"));
    let start_height = String::from("09000000");
    let relay = String::from("00");
    let payload = proto_version
        + &services
        + &time
        + &recv_addr
        + &send_addr
        + &nonce
        + &user_agent_len
        + &user_agent
        + &start_height
        + &relay;
    send("version", payload);
}
