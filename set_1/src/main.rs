extern crate shared;
mod set_1;

fn main() {
    let hexa = String::from("49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d");
    let b64 = set_1::hex_to_b64(hexa);
    println!("{}",b64);

    let xor = set_1::xor(String::from("1c0111001f010100061a024b53535009181c"), String::from("686974207468652062756c6c277320657965"));
    
    
}
