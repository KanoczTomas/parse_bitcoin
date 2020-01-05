use std::default::Default;
use crate::types::Hash256;
use crate::utils::hash256;

pub fn calculate_merkle_root(mut v: Vec<Hash256>) -> Hash256 {
    match v.len() {
        0 => Default::default(),
        1 => return v[0],
        _ => {
            if v.len() % 2 != 0 {
                v.push(v[v.len()-1]);
            }
            let mut n = 0;
            let mut res = Vec::new();
            while n < v.len() {
                let idx1 = n;
                let idx2 = n+1;
                res.push(hash256(vec![v[idx1].as_ref(),v[idx2].as_ref()]));
                n += 2;
            }
            calculate_merkle_root(res)
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_calculate_merkle_root() {
        let res = calculate_merkle_root(vec![Hash256::new(&[0u8;32][..])]);
        assert_eq!(res , Default::default());
        let res = calculate_merkle_root(vec![Hash256::new(&hex::decode("4a5e1e4baab89f3a32518a88c31bc87f618f76673e2cc77ab2127b7afdeda33b").unwrap())]);
        assert_eq!(res, Hash256::new(&hex::decode("4a5e1e4baab89f3a32518a88c31bc87f618f76673e2cc77ab2127b7afdeda33b").unwrap()));
        //from testnet block: 000000000000012729930afd5fed3e1f7dfffd6a0e0a2215a81887ed4b785b1b
        let v = vec![
            Hash256::new(&hex::decode("3346c19af961c9107bdd1ca8ed477165dedd56e2a5c911512521c09827d94311").unwrap()),
            Hash256::new(&hex::decode("b6665bee6da789001f6d99296c226315cb0c7413e5669579add27198aec5516e").unwrap()),
            Hash256::new(&hex::decode("abf7af6c442cc2953aa0247dc17c740c4d64af5117ad502a805e994d3d3a11f8").unwrap()),
            Hash256::new(&hex::decode("4a0ab9b04b87e717c0e5c18cd6391c35cc27525ddca2e96adae75c4d71b2dc07").unwrap())
        ];
        assert_eq!(calculate_merkle_root(v), Hash256::new(&hex::decode("f7ff6732787f00803829e73bf8bccdc2f39cd69ac21ffa04e7ab63c1ce11a536").unwrap()));
        //other tests are done in the parse_block tests
    }
}
