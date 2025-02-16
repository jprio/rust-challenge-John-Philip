fn encode_char(c: char) -> char {
    let a = 5;
    let b = 7;
    let m = 26; //number of letter in alphabet
    let e = (a * (c as u32 - b'a' as u32) + b) % m; //(ai + b) mod m
    char::from_u32(e + b'a' as u32).unwrap()
}
#[cfg(test)]
mod tests {
    use super::encode_char;
    #[test]
    fn char_as_u8() {
        let key: char = 'A';
        println!("{:?}", key as u8);
    }

    #[test]
    fn encode() {
        let key: char = 't';
        println!("{:?}", key as u8);
        println!("Encrypted : {:?}", encode_char('t') as char);
    }
}
