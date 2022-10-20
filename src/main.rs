use derive_more::From;
use ff::*;
use mimc_sponge_rs::{constants::C_STR, Fr, FrRepr};
use sha3::{
    digest::generic_array::{typenum::U32, GenericArray},
    Digest, Keccak256,
};

const SEED: &str = "mimcsponge";
// const NROUNDS: usize = 220;

#[derive(Debug, From)]
struct Hash(GenericArray<u8, U32>);

/// Can't construct the second hash from the first one
#[allow(dead_code)]
fn bug() -> Fr {
    let mut hasher = Keccak256::new();
    hasher.update(C_STR[1]);
    let hash = format!("{:x}", hasher.finalize());
    ff::from_hex(&hash).unwrap() // Error
}

fn main() {
    let const_1 = Fr::from_str(C_STR[1]).unwrap();
    let hash_1 = Hash::from(Hash::from(SEED));
    assert_eq!(hash_1.to_fr(), const_1);

    let const_2 = Fr::from_str(C_STR[2]).unwrap();
    let hash_2 = Hash::from(hash_1); // Error
    assert_eq!(hash_2.to_fr(), const_2);
}

impl Hash {
    fn from(x: impl AsRef<[u8]>) -> Self {
        let mut hasher = Keccak256::new();
        hasher.update(x.as_ref());
        hasher.finalize().into()
    }

    fn to_fr(&self) -> Fr {
        let hex = format!("{:x}", self.0);
        ff::from_hex(&hex).unwrap()
    }
}

impl AsRef<[u8]> for Hash {
    fn as_ref(&self) -> &[u8] {
        self.0.as_ref()
    }
}
