use num_traits::ToPrimitive;
use num_traits::identities::Zero;
use num_bigint::{ToBigUint, BigUint};


pub struct Base1Producer {
    current: BigUint,
    count: BigUint,
}

impl Base1Producer {
    pub fn new(count: BigUint) -> Self {
        Self {
            current: 0.to_biguint().unwrap(),
            count,
        }
    }
}

impl Iterator for Base1Producer {
    type Item = char;
    fn next(&mut self) -> Option<<Self as Iterator>::Item> {
        if self.current != self.count {
            self.current += 1_u32;
            Some('A')
        } else {
            None
        }
    }
}


pub fn encode_l(arr: &Vec<u8>) -> BigUint {
    let init = BigUint::from_bytes_be(&[0]);
    let a = BigUint::from_slice(&[1]);
    let b = BigUint::from_slice(&[256]);
    let lambda = |acc, x: &u8| acc * &b + x.to_biguint().unwrap() + &a;
    
    arr.iter().fold(init, lambda)   
}

pub fn encode(arr: &Vec<u8>) -> String {
    let l = encode_l(&arr);
    Base1Producer::new(l).collect::<String>()
}

pub fn decode_l(argl: &BigUint) -> Vec<u8> {
    let mut l = argl.clone();
    let mut bytes = Vec::new();
    while l > BigUint::zero() {
        l -= 1_u32;
        bytes.push((&l % 256_u32).to_u8().unwrap());
        l /= 256_u32;
    }
    bytes.reverse();
    bytes
}

pub fn decode(s: &String) -> Vec<u8> {
    if !s.chars().all(|x| x == 'A') {
        panic!("attempted to decode an invalid base1 string!\n");
    } else {
        decode_l(&s.len().to_biguint().unwrap())
    }
}


#[cfg(test)]
mod tests {
    use crate::{encode, encode_l, decode, decode_l};
    use num_bigint::ToBigUint;

    #[test]
    fn test_encode_l() {
        let arr = vec![0x03, 0xc0];
        let expected = 1217.to_biguint().unwrap();
        assert_eq!(expected, encode_l(&arr));
    }

    #[test]
    fn test_encode() {
        let arr = vec![0x03, 0xc0];
        let s = encode(&arr);
        assert!(s.len() == 1217);
    }

    #[test]
    fn test_decode_l() {
        let l = encode_l(&vec![0x03, 0xc0]);
        let arr2 = decode_l(&l);
        assert_eq!(arr2, &[3, 192]);
    }

    #[test]
    fn test_decode() {
        let s = std::iter::repeat('A').take(1217).collect::<String>();
        let res = decode(&s);
        assert_eq!(res, &[3, 192]);
    }
}
