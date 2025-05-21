use anyhow::{Context, Result, anyhow};
use serde::Deserialize;
use std::fs::{self};
use std::path::Path;
use std::process::Command;

#[derive(Debug, Deserialize)]
pub struct Inputs {
    pub price_start: String,
    pub price_end: String,
    pub timestamp_start: String,
    pub timestamp_end: String,
    pub expected_apy: String,
}

#[derive(Debug)]
pub struct ProofResult {
    pub proof: Vec<u8>,
    pub public_inputs: Vec<String>,
}

/// Generate a ZK proof from a Noir circuit using nargo and Barretenberg CLI.
///
/// # Arguments
/// * `input_path` - Path to the input.json file
/// * `circuit_dir` - Root directory of the Noir project
/// * `profile_path` - Path to the custom profile TOML for nargo
/// * `circuit_name` - The name of the compiled circuit (used for file lookup)
///
/// # Returns
/// `ProofResult` containing the proof bytes and public inputs (empty for now)
pub fn generate_proof_from_file(
    input_path: &str,
    circuit_dir: &str,
    profile_path: &str,
    circuit_name: &str,
) -> Result<ProofResult> {
    let input_dest = Path::new(circuit_dir).join("inputs/input.json");
    let target_dir = Path::new(circuit_dir).join("target");
    let proof_dir = target_dir.join("proof");

    // Step 1: Copy input.json
    fs::create_dir_all(input_dest.parent().unwrap())
        .context("Failed to create inputs directory")?;
    fs::copy(input_path, &input_dest).context("Failed to copy input.json")?;

    // Step 2: Run `nargo execute`
    let output = Command::new("nargo")
        .args(["execute", "-p", profile_path])
        .current_dir(circuit_dir)
        .output()
        .context("Failed to run nargo execute")?;

    if !output.status.success() {
        return Err(anyhow!(
            "nargo execute failed: {}",
            String::from_utf8_lossy(&output.stderr)
        ));
    }

    // Step 3: Run `bb prove`
    if proof_dir.exists() && !proof_dir.is_dir() {
        fs::remove_file(&proof_dir).context("Failed to remove file named 'proof'")?;
    }
    fs::create_dir_all(&proof_dir).context("Failed to create proof directory")?;

    let output = Command::new("bb")
        .args([
            "prove",
            "-b",
            &target_dir
                .join(format!("{}.json", circuit_name))
                .to_string_lossy(),
            "-w",
            &target_dir
                .join(format!("{}.gz", circuit_name))
                .to_string_lossy(),
            "-o",
            &proof_dir.to_string_lossy(),
        ])
        .output()
        .context("Failed to run bb prove")?;

    if !output.status.success() {
        return Err(anyhow!(
            "bb prove failed: {}",
            String::from_utf8_lossy(&output.stderr)
        ));
    }

    // Step 4: Read proof
    let proof = fs::read(proof_dir.join("proof")).context("Failed to read proof file")?;

    // Step 5: Placeholder for public inputs
    let public_inputs = vec![];

    Ok(ProofResult {
        proof,
        public_inputs,
    })
}
