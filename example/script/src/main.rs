// A simple script to generate proofs for the tetris program, and serialize them to JSON.
use clap::Parser;
use serde::{Deserialize, Serialize};

use sp1_sdk::{include_elf, utils, HashesKey, ProverClient, SP1ProofWithPublicValues, SP1Stdin};

// The ELF (executable and linkable format) file for the tetris program.
const TETRIS_ELF: &[u8] = include_elf!("tetris-program");

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

#[derive(Serialize, Deserialize)]
struct ProofData {
    proof: String, // hex string
    public_inputs: String, // hex string (score and time)
    vk_key_hash: String,
    mode: String,
}

#[derive(clap::Parser)]
#[command(name = "zkProofGenerator")]
struct Cli {
    #[arg(long, value_name = "prove", default_value = "false", help = "Whether to generate a proof or use the pregenerated proof")]
    prove: bool,

    #[arg(long, value_name = "mode", default_value = "plonk", help = "Specifies the proof mode to use (e.g., groth16, plonk)")]
    mode: String,
}

fn main() {
    // Setup logging for the application
    utils::setup_logger();

    // Parse command line arguments
    let args = Cli::parse();

    // Example game state for Tetris (replace with actual input)
    let game_state = GameState {
        moves: vec![
            Move { action: "ArrowDown".to_string(), score: 1, time: 89 },
            Move { action: "ArrowUp".to_string(), score: 3, time: 88 },
        ],
        final_score: 3,
        final_time: 88,
    };

    let mut stdin = SP1Stdin::new();
    stdin.write(&game_state);

    // Initialize the prover client.
    let (pk, vk) = ProverClient::from_env().setup(TETRIS_ELF);

    if args.prove {
        // Generate proof based on mode
        let proof = match args.mode.as_str() {
            "groth16" => ProverClient::from_env()
                .prove_groth16(&pk, stdin)
                .run()
                .expect("Failed to generate Groth16 proof"),
            "plonk" => ProverClient::from_env()
                .prove_plonk(&pk, stdin)
                .run()
                .expect("Failed to generate Plonk proof"),
            _ => panic!("Unsupported proof mode"),
        };

        // Serialize proof data to JSON
        let proof_data = ProofData {
            proof: hex::encode(proof.bytes()),
            public_inputs: hex::encode(proof.public_values().as_bytes()),
            vk_key_hash: vk.hash().to_string(),
            mode: args.mode,
        };

        // Write proof to JSON file in example/json/
        serde_json::to_writer(
            std::fs::File::create("tetris_proof.json").unwrap(),
            &proof_data,
        ).unwrap();
        println!("Proof generated and saved to tetris_proof.json");
    } else {
        // Use pregenerated proof (example)
        let proof_data = ProofData {
            proof: "pregenerated_proof".to_string(),
            public_inputs: "pregenerated_public_inputs".to_string(),
            vk_key_hash: vk.hash().to_string(),
            mode: args.mode,
        };
        serde_json::to_writer(
            std::fs::File::create("tetris_proof.json").unwrap(),
            &proof_data,
        ).unwrap();
        println!("Used pregenerated proof and saved to tetris_proof.json");
    }
}
