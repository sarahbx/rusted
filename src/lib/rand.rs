use std::fs::File;
use std::io::Read;



fn rand_u8() -> u8  {
    let mut buffer = [0u8; 1];
    let mut file = File::open("/dev/urandom").expect("Unable to open /dev/urandom");

    file.read_exact(&mut buffer).expect("Unable to read from /dev/urandom");
    buffer[0]
}

fn rand_u32() -> u32  {
    let mut output:u32 = 0;
    let mut buffer = [0u8; 4];
    let mut file = File::open("/dev/urandom").expect("Unable to open /dev/urandom");

    file.read_exact(&mut buffer).expect("Unable to read from /dev/urandom");

    for byte in buffer {
        output = output.rotate_left(8);
        output |= byte as u32;
    }
    output
}

fn rand_char_from_rand_u8() -> char {
    loop {
        let possible_value = char::from_u32(rand_u8() as u32);

        match possible_value {
            None => continue,
            _ => return possible_value.unwrap(),
        }
    }
}

fn rand_alphanum() -> char {
    loop {
        let rand_char = rand_char_from_rand_u8();

        if rand_char.is_alphanumeric() {
            return rand_char
        }
    }
}

pub fn rand_alphanum_string(length: u32) -> String {
    let mut output = String::new();

    for _ in 0..length {
        output.push(rand_alphanum());
    }
    output
}
