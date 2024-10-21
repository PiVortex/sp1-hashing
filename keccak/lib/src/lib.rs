use alloy_sol_types::sol;
use tiny_keccak::{Hasher, Keccak};

sol! {
    /// The public values encoded as a struct that can be easily deserialized inside Solidity.
    struct PublicValuesStruct {
        bytes data;
        bytes hash;
    }
}

pub fn hash_keccak(data: &[u8]) -> [u8; 32] {
    let mut hasher = Keccak::v256();
    hasher.update(data);
    let mut output = [0u8; 32];
    hasher.finalize(&mut output);
    output
}
