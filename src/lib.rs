use sha3::{Digest, Keccak256};

pub fn checksum(address: &str) -> String {
    println!("{address}");
    let address = address.trim_start_matches("0x").to_lowercase();

    let address_hash = {
        let mut hasher = Keccak256::new();
        hasher.update(address.as_bytes());
        println!("{:?}", hasher);
        hasher.finalize()
    };

    let hash_str = format!("{:x}", address_hash);

    println!("{:?}", address_hash);
    address
        .char_indices()
        .fold(String::from("0x"), |mut acc, (index, address_char)| {
            // this cannot fail since it's Keccak256 hashed
            let n = u16::from_str_radix(&hash_str[index..index + 1], 16).unwrap();

            if n > 7 {
                // make char uppercase if ith character is 9..f
                acc.push_str(&address_char.to_uppercase().to_string())
            } else {
                // already lowercased
                acc.push(address_char)
            }

            acc
        })
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_checksum() {
        let addr_lowercase = "0xe0fc04fa2d34a66b779fd5cee748268032a146c0";
        let checksummed = checksum(addr_lowercase);
        assert_eq!(checksummed, "0xe0FC04FA2d34a66B779fd5CEe748268032a146c0");

        let addr_uppercase = "0xE0FC04FA2D34A66B779FD5CEE748268032A146C0";
        let checksummed = checksum(addr_uppercase);
        assert_eq!(checksummed, "0xe0FC04FA2d34a66B779fd5CEe748268032a146c0");
    }
}
