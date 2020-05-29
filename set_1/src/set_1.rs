use std::u8;
use hex;


pub struct ASCII_lower {
   chars: Vec<char> 
}

impl ASCII_lower {
    pub fn new() -> ASCII_lower {
        let chars = vec!(
            'a', 'b', 'c', 'd', 'e', 
            'f', 'g', 'h', 'i', 'j', 
            'k', 'l', 'm', 'n', 'o',
            'p', 'q', 'r', 's', 't', 
            'u', 'v', 'w', 'x', 'y', 
            'z', 'A', 'B', 'C', 'D',
            'E', 'F', 'G', 'H', 'I',
            'J', 'K', 'L', 'M', 'N',
            'O', 'P', 'Q', 'R', 'S',
            'T', 'U', 'V', 'W', 'X',
            'Y', 'Z',
        );
        ASCII_lower {
            chars
        }
    }

    fn get_score(&mut self, words: String) {
        let score: usize = 0;
        let chars = self.chars.clone();
        for character in words.chars() {
            println!("Character is: {}",character);
            let chars_score = chars.iter().filter(|letter| character == **letter);
            println!("Chars score is: {:#?}",chars_score);
        }
    }
}


pub fn hex_to_b64(hexa: String) -> String {
    base64::encode(&to_bytes(hexa))
}

pub fn to_bytes(data: String) -> Vec<u8> {
    let mut bytes = Vec::new();
    for i in 0..(data.len()/2) {
        let res = u8::from_str_radix(&data[2*i .. 2*i+2], 16);
        match res {
            Ok(v) => bytes.push(v),
            Err(e) => println!("Ocurred an error: {}", e),
        }
    };
    bytes
}

pub fn into_hex(byte: u8) -> String {
    let string: String = format!("{:02X}", byte);
    string
}

pub fn xor(str1: String, str2: String) -> String {
    assert_eq!(str1.len(),str2.len());
    let bytes_a = to_bytes(str1);
    let bytes_b = to_bytes(str2);
    
    let xor = bytes_a.iter()
              .zip(&bytes_b)
              .map(|(x,y)| x ^ y)
              .map(|byte| into_hex(byte))
              .collect::<Vec<String>>()
              .concat();

    xor
}

pub fn decrypt(message: String) {
    let mut ascii_chars = ASCII_lower::new();
    let score: usize = 0;
    
    for ascii_character in ascii_chars.chars {
        let encrypted_message = message.chars().map(|character| xor(character.to_string(), ascii_character.to_string())).collect::<String>();
        println!("Encrypted against {}: {:#?}",ascii_character, encrypted_message);
        ascii_chars.get_score(encrypted_message);
        
    }
}
