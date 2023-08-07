use std::env;
use std::fs::File;
use std::io::{Read, Write};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 7 {
        println!("Usage: ./unxor -f <input_file> -k <key> -o <output_file>", args[0]);
        return;
    }

    let (mut input_file, mut key, mut output_file) = (String::default(), String::default(), String::default());

    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "-f" | "--file" => {
                input_file = args[i + 1].clone();
                i += 1;
            }
            "-k" | "--key" => {
                key = args[i + 1].clone();
                i += 1;
            }
            "-o" | "--output" => {
                output_file = args[i + 1].clone();
                i += 1;
            }
            _ => {
                println!("Unknown option: {}", args[i]);
                return;
            }
        }
        i += 1;
    }
    
    let mut contents = match File::open(&input_file).and_then(|mut f| {
        let mut contents = Vec::new();
        f.read_to_end(&mut contents).map(|_| contents)
    }) {
        Ok(contents) => contents,
        Err(_) => {
            println!("Failed to load file {}", input_file);
            return;
        }
    };

    let decrypted_data: Vec<u8> = contents
        .iter()
        .zip(key.as_bytes().iter().cycle())
        .map(|(cipher_byte, key_byte)| cipher_byte ^ key_byte)
        .collect();
    
    let mut output = match File::create(&output_file) {
        Ok(output) => output,
        Err(_) => {
            println!("Failed to create the output file {}", output_file);
            return;
        }
    };

    if let Err(_) = output.write_all(&decrypted_data) {
        println!("Failed to write the decrypted data to the output file {}", output_file);
        return;
    }

    println!("The file has been successfully decrypted and saved to {}", output_file);
}
