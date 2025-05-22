# zk-prover

A Rust library for generating and verifying zero-knowledge proofs using [Noir](https://noir-lang.org/) and [Barretenberg].

## Features

- Wraps `nargo` and `barretenberg` tools
- Loads Noir inputs as input params
- Generates `.proof` and `.vk` files
- Prepares Solidity verifier contracts

## Usage

```rust
use zk_prover::{prove_and_verify, Inputs};

let inputs = Inputs {
        price_start: "1000000000000000000".to_string(),
        price_end: "1010000000000000000".to_string(),
        timestamp_start: "0".to_string(),
        timestamp_end: "2592000".to_string(),
        expected_apy: "121666666666666666".to_string(),
    };
    let result = zk_prover::generate_proof_from_file(
        &inputs,
        "../proof_of_yield",
        "ApyProver.toml",
        "proof_of_yield",
    )?;
```

## Requirements

- nargo and bb (barretenberg) installed and available in $PATH

## License

ZK Porver is open-source software licensed under the MIT License.
