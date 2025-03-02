use wasm_bindgen::prelude::*;
use sp1_verifier::{Verifier, Proof};
use serde::{Deserialize, Serialize};

// Структура для входных данных (предполагаемая, адаптируйте под вашу игру)
#[derive(Serialize, Deserialize)]
struct GameState {
    final_score: u32,
    final_time: u32,
}

// Функция верификации, экспортируемая в JS
#[wasm_bindgen]
pub fn verify_proof(proof_json: &str) -> Result<bool, JsValue> {
    // Десериализуем доказательство из JSON
    let proof: Proof = serde_json::from_str(proof_json)
        .map_err(|e| JsValue::from_str(&format!("Failed to parse proof: {}", e)))?;

    // Создаём верификатор (настройте параметры по документации SP1)
    let verifier = Verifier::new(/* параметры верификации, например, ELF или публичные ключи */);

    // Верифицируем доказательство
    let result = verifier.verify(&proof)
        .map_err(|e| JsValue::from_str(&format!("Verification failed: {:?}", e)))?;

    Ok(result)
}

// Логирование для отладки
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}
