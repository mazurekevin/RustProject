use std::fmt::format;
use std::io::Split;
use md5;
use md5::Digest;

mod Input;
mod Output;

struct StructInput {
    complexity: u32,
    message: String,
}

fn main() {
    let input = StructInput {
        complexity: 3,
        message: String::from("hello"),
    };

    let mut finish = false;
    let mut seed = 0;

    while finish == false {
        let hexSeed = format_dec_to_hex(seed);
        let concatSeed = concat_string(hexSeed.to_string(), input.message.to_string());
        let digest = md5::compute(concatSeed);
        let hashCode = format_digest_to_hex(digest);
        let mut binaryHash: String = format_to_binary(hashCode);
        finish = check_seed(binaryHash, input.complexity);
        seed += 1;
    }
}

fn concat_string(seed: String, message: String) -> String {
    format!("{}{}\n", seed, message)
}

fn format_dec_to_hex(seed: i32) -> String {
    format!("{:016X}", seed)
}

fn format_digest_to_hex(digest: Digest) -> String {
    format!("{:032X}", digest)
}

fn format_to_binary(hashcode: String) -> String {
    hashcode.chars().map(to_binary).collect()
}

fn check_seed(binaryHash: String, complexity: u32) -> bool {
    let mut index = 0;
    for character in binaryHash.chars() {
        if character == '1' && index < complexity {
            print!("false\n");
            return false;
        } else if index >= complexity {
            print!("good ");//envoie du resultat au server
            return true;
        }
        index += 1;
    }
    return false;
}


fn to_binary(c: char) -> String {
    let b = match c {
        '0' => "0000",
        '1' => "0001",
        '2' => "0010",
        '3' => "0011",
        '4' => "0100",
        '5' => "0101",
        '6' => "0110",
        '7' => "0111",
        '8' => "1000",
        '9' => "1001",
        'A' => "1010",
        'B' => "1011",
        'C' => "1100",
        'D' => "1101",
        'E' => "1110",
        'F' => "1111",
        _ => "",
    };
    return String::from(b);
}


