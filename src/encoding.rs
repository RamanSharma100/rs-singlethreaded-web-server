
pub struct Encoding{
    pub precentage_encode: fn(&str) -> String,
    pub precentage_decode: fn(&str) -> String,
    pub base64_encode: fn(&str) -> String,
}


impl Encoding {
    #[allow(dead_code)]
    pub fn precentage_encode(input: &str) -> String {
        let mut encoded = String::new();
        for byte in input.bytes() {
            match byte {
                0x30..=0x39 | 0x41..=0x5A | 0x61..=0x7A => {
                    encoded.push(byte as char);
                }
                _ => {
                    encoded.push('%');
                    encoded.push_str(&format!("{:X}", byte));
                }
            }
        }
        encoded
    }

    pub  fn precentage_decode(input: &str) -> String {
        let mut decoded = String::new();
        let mut bytes = input.bytes();
        while let Some(byte) = bytes.next() {
            match byte {
                0x25 => {
                    let byte1 = bytes.next().unwrap();
                    let byte2 = bytes.next().unwrap();
                    let hex = u8::from_str_radix(&format!("{}{}", byte1 as char, byte2 as char), 16).unwrap();
                    decoded.push(hex as char);
                }
                _ => {
                    decoded.push(byte as char);
                }
            }
        }
        decoded
    }

    #[allow(dead_code)]
    pub fn base64_encode(input: &str) -> String {
        base64::encode(input)
    }
}