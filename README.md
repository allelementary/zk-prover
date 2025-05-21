# zk-prover

A Rust library for generating and verifying zero-knowledge proofs using [Noir](https://noir-lang.org/) and [Barretenberg].

## Features

- Wraps `nargo` and `barretenberg` tools
- Loads Noir inputs from JSON
- Generates `.proof` and `.vk` files
- Prepares Solidity verifier contracts

## Usage

```rust
use zk_prover::prove_and_verify;

prove_and_verify("target/proof.acir", "input.json")?;
```

## Requirements

- nargo and bb (barretenberg) installed and available in $PATH

## License

ZK Porver is open-source software licensed under the MIT License.
