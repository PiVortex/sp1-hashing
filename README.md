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

These tests were run on a machine and the prover network.

### Groth16 Benchmarks

#### SHA256

| Environment       | Precompile         | Cycles | Gas   | E2E Time       | kHz  | Proof Size | Link                                                                 | Time                |
|-------------------|--------------------|--------|-------|----------------|------|------------|----------------------------------------------------------------------|---------------------|
| Local Machine       | Without precompile | 22203  | 56393 | 20.040230106s  | 1.11 | 2656912    |                                                                      |                     |
| Local Machine       | With precompile    | 4255   | 20783 | 25.455597017s  | 0.17 | 5518992    |                                                                      |                     |
| On prover network | Without precompile |        |       |                |      |            | [Link](https://explorer.succinct.xyz/proof/01japypg1mf379xq16ex4nw54z) | 2 minutes 32 seconds |
| On prover network | With precompile    |        |       |                |      |            | [Link](https://explorer.succinct.xyz/proof/01japzx0nnerks2p7nsznr1j4n) | 2 minutes 51 seconds |

#### Keccak

| Environment       | Precompile         | Cycles | Gas   | E2E Time       | kHz  | Proof Size | Link                                                                 | Time                |
|-------------------|--------------------|--------|-------|----------------|------|------------|----------------------------------------------------------------------|---------------------|
| Local Machine       | Without precompile | 35022  | 61538 | 20.068248415s  | 1.75 | 3259092    |                                                                      | 2 minutes 34 seconds |
| Local Machine       | With precompile    | 18646  | 42184 | 24.120392861s  | 0.77 | 5894997    |                                                                      |                     |
| On prover network | Without precompile |        |       |                |      |            | [Link](https://explorer.succinct.xyz/proof/01jaq4pnf4e65bpk73s02jg3tx) | 2 minutes 34 seconds |
| On prover network | With precompile    |        |       |                |      |            | [Link](https://explorer.succinct.xyz/proof/01jaq51mpge4ta29gtc4d4c50z) | 2 minutes 56 seconds |

## Analysis 

The number of cycles are are much lower for the precompiles. The e2e time is higher for the precompiles, this seems to be because the cycle rate is lower due to limitations of the machine. Assuming a fixed cycle rate of 1kHz and using the results from the local mahcine we would get the following results:

| Hash Function | Precompile       | Effective Proof Time (seconds) |
|---------------|------------------|----------------------|
| SHA256        | Without precompile | 22.203 |
| SHA256        | With precompile    | 4.225 |
| Keccak        | Without precompile | 35.022 |
| Keccak        | With precompile    | 18.646 |


#### Local Machine System Information

- **OS**: Ubuntu 22.04.4 LTS
- **CPU**: 6 Cores - Intel Core i7-10750H CPU @ 2.60GHz
- **Memory**: 8GB