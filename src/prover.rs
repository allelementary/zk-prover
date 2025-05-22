use anyhow::{Context, Result, anyhow};
use serde::{Deserialize, Serialize};
use std::fs::{self};
use std::path::Path;
use std::process::Command;

#[derive(Debug, Serialize, Deserialize)]
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

pub fn generate_proof_from_file(
    inputs: &Inputs,
    circuit_dir: &str,
    profile_name: &str,
    circuit_name: &str,
) -> Result<ProofResult> {
    let profile_path = Path::new(circuit_dir).join(profile_name);
    let compiled_dir = Path::new(circuit_dir).join("target");
    let json_path = compiled_dir.join(format!("{circuit_name}.json"));
    let witness_path = compiled_dir.join(format!("{circuit_name}.gz"));
    let proof_path = compiled_dir.join("proof");
    let vk_dir = compiled_dir.join("vk");
    let vk_path = vk_dir.join("vk");
    let verifier_path = compiled_dir.join("Verifier.sol");

    if vk_dir.exists() && vk_dir.is_dir() {
        fs::remove_dir_all(&vk_dir).context("Failed to remove vk directory")?;
    }
    fs::create_dir_all(&vk_dir).context("Failed to create vk directory")?;

    println!("🔧 Writing inputs to profile: {}", profile_path.display());
    let toml_str = toml::to_string(inputs).context("Failed to serialize inputs to TOML")?;
    fs::write(&profile_path, toml_str).context("Failed to write TOML input file")?;

    println!("⚙️ Running nargo execute...");
    let output = Command::new("nargo")
        .args(["execute", "-p", profile_name])
        .current_dir(circuit_dir)
        .output()
        .context("Failed to run nargo execute")?;
    if !output.status.success() {
        return Err(anyhow!(
            "nargo execute failed: {}",
            String::from_utf8_lossy(&output.stderr)
        ));
    }

    println!("📦 Running bb prove...");
    let output = Command::new("bb")
        .args([
            "prove",
            "-b",
            &json_path.to_string_lossy(),
            "-w",
            &witness_path.to_string_lossy(),
            "-o",
            &compiled_dir.to_string_lossy(),
        ])
        .current_dir(circuit_dir)
        .output()
        .context("Failed to run bb prove")?;
    if !output.status.success() {
        return Err(anyhow!(
            "bb prove failed: {}",
            String::from_utf8_lossy(&output.stderr)
        ));
    }

    println!("📄 Reading generated proof...");
    let proof = fs::read(&proof_path).context("Failed to read proof file")?;

    println!("🔐 Generating verification key...");
    let output = Command::new("bb")
        .args([
            "write_vk",
            "-b",
            &json_path.to_string_lossy(),
            "-o",
            &vk_dir.to_string_lossy(),
        ])
        .current_dir(circuit_dir)
        .output()
        .context("Failed to run bb write_vk")?;
    if !output.status.success() {
        return Err(anyhow!(
            "bb write_vk failed: {}",
            String::from_utf8_lossy(&output.stderr)
        ));
    }

    println!("✅ Verifying proof...");
    let output = Command::new("bb")
        .args([
            "verify",
            "-k",
            &vk_path.to_string_lossy(),
            "-p",
            &proof_path.to_string_lossy(),
        ])
        .current_dir(circuit_dir)
        .output()
        .context("Failed to run bb verify")?;
    if !output.status.success() {
        return Err(anyhow!(
            "bb verify failed: {}",
            String::from_utf8_lossy(&output.stderr)
        ));
    }

    println!("📝 Generating Solidity verifier...");
    let output = Command::new("bb")
        .args([
            "write_solidity_verifier",
            "-k",
            &vk_path.to_string_lossy(),
            "-o",
            &verifier_path.to_string_lossy(),
        ])
        .current_dir(circuit_dir)
        .output()
        .context("Failed to run bb write_solidity_verifier")?;
    if !output.status.success() {
        return Err(anyhow!(
            "bb write_solidity_verifier failed: {}",
            String::from_utf8_lossy(&output.stderr)
        ));
    }

    println!("✅ ZK proof generation complete.");
    Ok(ProofResult {
        proof,
        public_inputs: vec![],
    })
}
