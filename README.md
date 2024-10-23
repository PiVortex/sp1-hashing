## SP1 precompile hash functions

This repo shows examples of generating a groth16 proof for sha256 and keccak hashing functions.
The programs can be run without the precompile by removing the patches from the top level cargo.toml file.

## Running the code

Enter one of the two directories

Build the program

```bash
cd program
cargo prove build
```

Run the script

```bash 
cd script
RUST_LOG=info cargo run --release -- --prove
```

Optionally you can run the script on the prover network 

```bash
cd script
SP1_PROVER=network SP1_PRIVATE_KEY=... RUST_LOG=info cargo run --release -- --prove
```

## Benchmark results

These tests were run on a local machine and the prover network. In these benchmarks a groth 16 proof was generated for a sha256 and keccak hash of a 10KB input.


#### SHA256

| Environment       | Precompile         | Cycles   | Gas     | E2E Time       | kHz  | Proof Size | Link                                                                 | Time                |
|-------------------|--------------------|----------|---------|----------------|------|------------|----------------------------------------------------------------------|---------------------|
| Local Machine     | Without precompile | 1369575  | 1692317 | 139.106343834s | 9.85 | 10099927   |                                                                      |                     |
| Local Machine     | With precompile    | 245166   | 365676  | 48.886080699s  | 5.02 | 5655056    |                                                                      |                     |
| On prover network | Without precompile |          |         |                |      |            | [Link](https://explorer.succinct.xyz/proof/01javw27e4fjbrj7krqzyd5bnw) | 2 minutes 38 seconds |
| On prover network | With precompile    |          |         |                |      |            | [Link](https://explorer.succinct.xyz/proof/01javvq0g3fjbvpvmqcbjezw4x) | 2 minutes 48 seconds |

#### Keccak

| Environment       | Precompile         | Cycles   | Gas     | E2E Time       | kHz  | Proof Size | Link                                                                 | Time                |
|-------------------|--------------------|----------|---------|----------------|------|------------|----------------------------------------------------------------------|---------------------|
| Local Machine     | Without precompile | 2070089  | 2435888 | 188.938836422s | 10.96| 12940745   |                                                                      |                     |
| Local Machine     | With precompile    | 825513   | 1041865 | 95.18422441s   | 8.67 | 9991603    |                                                                      |                     |
| On prover network | Without precompile |          |         |                |      |            | [Link](https://explorer.succinct.xyz/proof/01javwh79df3wbnkpt2vd393f9) | 2 minutes 40 seconds |
| On prover network | With precompile    |          |         |                |      |            | [Link](https://explorer.succinct.xyz/proof/01javwys73fjbtj687dw4d28b4) | 2 minutes 43 seconds |

## Analysis 

- **Cycles**: The number of cycles is significantly reduced when using precompiles for both SHA256 and Keccak.
- **Gas**: Gas consumption is also lower with precompiles.
- **e2e Time**: The end-to-end time is higher for precompiles, likely due to the lower cycle rate.
- **kHz**: The cycle rate (kHz) is lower for precompiles, this could be due to limitations of the machine and a machine with greater resources could leads to a lesser e2e time.
- **Proof Size**: The proof size is smaller with precompiles for SHA256 but slightly larger for Keccak.

#### Local Machine System Information

- **OS**: Ubuntu 22.04.4 LTS
- **CPU**: 6 Cores - Intel Core i7-10750H CPU @ 2.60GHz
- **Memory**: 8GB