use ring::digest;
use crate::types::Hash256;

//calculate the double hash use by bitcoin sha256(sha256(data))
pub fn hash256(inputs: Vec<&[u8]>) -> Hash256 {
    let mut context = digest::Context::new(&digest::SHA256);
    for input in inputs {
        context.update(input);
    }
    let dhash = digest::digest(&digest::SHA256, context.finish().as_ref());
    Hash256::new(dhash.as_ref())
}

#[cfg(test)]
mod test {
    use super::*;
    use hex;
    #[test]
    fn test_hash256() {
        let data = vec!["this ".as_bytes(), "is a ".as_bytes(), "test\n".as_bytes()];
        assert_eq!(hash256(data.clone()), Hash256::new(&hex::decode("8ba59f5716befd88800962b68fcc7543750a149612139b992ab9bbd4adc79291").unwrap()));
        assert_eq!(hash256(data), hash256(vec!["this is a test\n".as_bytes()]));

        let data = vec!["this is a completely ".as_bytes(), "different test\n".as_bytes()];
        assert_eq!(hash256(data.clone()), Hash256::new(&hex::decode("e775db9845d42cf12987598c62a69fd843a1207c93d2202540e1762a80049847").unwrap()));
        assert_eq!(hash256(data), hash256(vec!["this is a completely different test\n".as_bytes()]));

        let data = vec![&[0x00][..], &[0x0b][..], &[0xff][..], &[0xef][..]];
        assert_eq!(hash256(data.clone()), Hash256::new(&hex::decode("055826442580450cc41e82a745d16993eb00487d818fe028b0b46a50015c9f8d").unwrap()));
        assert_eq!(hash256(data), hash256(vec![&[0x00, 0x0b, 0xff, 0xef][..]]));
    }
}
