//! A simple program that takes a number `n` as input, and writes the `n-1`th and `n`th fibonacci
//! number as an output.

// These two lines are necessary for the program to properly compile.
//
// Under the hood, we wrap your main function with some extra code so that it behaves properly
// inside the zkVM.
#![no_main]
sp1_zkvm::entrypoint!(main);

use alloy_sol_types::SolType;
use hash_lib::PublicValuesStruct;

pub fn main() {
    // Read an input to the program.
    //
    // Behind the scenes, this compiles down to a custom system call which handles reading inputs
    // from the prover
    let data = sp1_zkvm::io::read::<Vec<u8>>();

    // Compute the sha256 hash
    let result = hash_lib::hash_sha256(&data);

    // Encode the public values of the program
    let bytes = PublicValuesStruct::abi_encode(&PublicValuesStruct {
        data: data.into(),
        hash: result.to_vec().into(),
    });

    // Commit to the public values of the program. The final proof will have a commitment to all the
    // bytes that were committed to
    sp1_zkvm::io::commit_slice(&bytes);
}
