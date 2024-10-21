use alloy_sol_types::sol;
use sha2_v0_10_8::{Digest as Digest_10_6, Sha256 as Sha256_10_6};

sol! {
    /// The public values encoded as a struct that can be easily deserialized inside Solidity.
    struct PublicValuesStruct {
        bytes data;
        bytes hash;
    }
}

pub fn hash_sha256(data: &[u8]) -> [u8; 32] {
    let mut hasher = Sha256_10_6::new();
    hasher.update(data);
    hasher.finalize().into()
}
