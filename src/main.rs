use rand::Rng;
use std::io::{self, Write};

fn one_time_pad(message: &[u8], key: &[u8]) -> Vec<u8> {
    message.iter()
        .zip(key.iter())
        .map(|(&msg_byte, &key_byte)| msg_byte ^ key_byte)
        .collect()
}

fn generate_random_key(length: usize) -> Vec<u8> {
    let mut rng = rand::thread_rng();
    (0..length).map(|_| rng.gen::<u8>()).collect()
}

fn get_input(prompt: &str) -> String {
    let mut input = String::new();
    print!("{}", prompt);
    io::stdout().flush().unwrap();  
    io::stdin().read_line(&mut input).expect("Failed to read input");
    input.trim().to_string()
}

fn main() {
    let mode = get_input("Do you want to (E)ncrypt or (D)ecrypt? [E/D]: ").to_lowercase();

    let message = get_input("Enter the message: ");
    
    let key: Vec<u8> = if mode == "e" {
        let key_choice = get_input("Do you want to (G)enerate a random key or (P)rovide a key? [G/P]: ").to_lowercase();
        if key_choice == "g" {
            let key = generate_random_key(message.len());
            println!("Generated Key: {:?}", key);
            key
        } else {
            let key_input = get_input("Enter a key (must be same length as the message): ");
            if key_input.len() != message.len() {
                println!("Error: The key must be the same length as the message.");
                return;
            }
            key_input.into_bytes()
        }
    } else {
        let key_input = get_input("Enter the key used for encryption: ");
        if key_input.len() != message.len() {
            println!("Error: The key must be the same length as the message.");
            return;
        }
        key_input.into_bytes()
    };

    let result = one_time_pad(message.as_bytes(), &key);

    if mode == "e" {
        println!("Encrypted message (in bytes): {:?}", result);
    } else {
        let decrypted_message = String::from_utf8(result).expect("Failed to decode the message");
        println!("Decrypted message: {}", decrypted_message);
    }
}

