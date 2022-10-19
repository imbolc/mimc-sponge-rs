use derive_more::From;
use ff::*;
use mimc_sponge_rs::{Fr, FrRepr};
use sha3::{
    digest::generic_array::{typenum::U32, GenericArray},
    Digest, Keccak256,
};

const SEED: &str = "mimcsponge";
// const NROUNDS: usize = 220;

#[derive(Debug, From)]
struct Hash(GenericArray<u8, U32>);

fn main() {
    let fr = Fr::from(Hash::from_str(SEED));
    // consts::C_STR.1
    let test = Fr::from_str(
        "7120861356467848435263064379192047478074060781135320967663101236819528304084",
    )
    .unwrap();
    assert_eq!(fr, test);
}

impl Hash {
    fn from_str(s: &str) -> Hash {
        let mut hasher = Keccak256::new();
        hasher.update(s);
        hasher.finalize().into()
    }
}

impl From<Hash> for Fr {
    fn from(value: Hash) -> Self {
        let arr: [u64; 4] = value
            .0
            .chunks(8)
            .map(|x| u64::from_le_bytes(x.try_into().unwrap()))
            // .map(|x| u64::from_ne_bytes(x.try_into().unwrap()))
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();
        Fr::from_repr(FrRepr(arr)).unwrap()
    }
}
