// A simple wrapper around the `sp1_verifier` and `sp1_sdk` crates.
use sp1_sdk::{ProverClient, SP1Stdin};
use sp1_verifier::{Groth16Verifier, PlonkVerifier, GROTH16_VK_BYTES, PLONK_VK_BYTES};
use serde::{Serialize, Deserialize};
use wasm_bindgen::prelude::*;
use hex;

#[derive(Serialize, Deserialize)]
struct Move {
    action: String,
    score: u32,
    time: u32,
}

#[derive(Serialize, Deserialize)]
struct GameState {
    moves: Vec<Move>,
    final_score: u32,
    final_time: u32,
}

/// Generate a proof for Tetris game state.
#[wasm_bindgen]
pub fn generate_proof(json_input: &str) -> Result<String, JsValue> {
    // Parse JSON from JavaScript
    let game_state: GameState = serde_json::from_str(json_input)
        .map_err(|e| JsValue::from_str(&format!("JSON Error: {}", e)))?;

    // Setup SP1
    let client = ProverClient::new();
    let mut stdin = SP1Stdin::new();
    stdin.write(&game_state);

    // Load ELF file from the program (example/fibonacci-program/)
    let elf = include_bytes!("../../example/fibonacci-program/elf/riscv32im-succinct-zkvm-elf");

    // Generate proof
    let (mut public_values, proof) = client
        .prove(elf, stdin)
        .plonk()
        .run()
        .map_err(|e| JsValue::from_str(&format!("Proof generation failed: {}", e)))?;

    // Read public values
    let final_score = public_values.read::<u32>();
    let final_time = public_values.read::<u32>();

    // Return result as JSON
    let result = serde_json::to_string(&serde_json::json!({
        "score": final_score,
        "time": final_time,
        "proof": hex::encode(proof.bytes()) // Proof in hex format
    })).map_err(|e| JsValue::from_str(&format!("Serialization error: {}", e)))?;

    Ok(result)
}

/// Wrapper around [`sp1_verifier::Groth16Verifier::verify`].
/// We hardcode the Groth16 VK bytes to only verify SP1 proofs.
#[wasm_bindgen]
pub fn verify_groth16(proof: &[u8], public_inputs: &[u8], sp1_vk_hash: &str) -> bool {
    Groth16Verifier::verify(proof, public_inputs, sp1_vk_hash, GROTH16_VK_BYTES).is_ok()
}

/// Wrapper around [`sp1_verifier::PlonkVerifier::verify`].
/// We hardcode the Plonk VK bytes to only verify SP1 proofs.
#[wasm_bindgen]
pub fn verify_plonk(proof: &[u8], public_inputs: &[u8], sp1_vk_hash: &str) -> bool {
    PlonkVerifier::verify(proof, public_inputs, sp1_vk_hash, PLONK_VK_BYTES).is_ok()
}
