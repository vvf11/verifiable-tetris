#![no_main]
sp1_zkvm::entrypoint!(main);

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Move {
    action: String, // "ArrowLeft", "ArrowRight", "ArrowDown", "ArrowUp"
    score: u32,     // Счёт на момент хода
    time: u32,      // Оставшееся время
}

#[derive(Serialize, Deserialize)]
struct GameState {
    moves: Vec<Move>,
    final_score: u32,
    final_time: u32,
}

pub fn main() {
    // Читаем входные данные (состояние игры от JavaScript)
    let game_state: GameState = sp1_zkvm::io::read();

    // Проверяем корректность счёта
    let mut computed_score = 0;
    for mov in &game_state.moves {
        match mov.action.as_str() {
            "ArrowDown" => computed_score += 1,  // +1 за движение вниз
            "ArrowUp" => computed_score += 2,    // +2 за поворот (пример)
            _ => (),                             // Другие ходы не влияют
        }
    }

    // Убеждаемся, что счёт совпадает
    assert_eq!(computed_score, game_state.final_score, "Счёт не совпадает!");
    assert!(game_state.final_time <= 90, "Время превышено!");

    // Делаем финальные значения публичными для проверки
    sp1_zkvm::io::commit(&game_state.final_score);
    sp1_zkvm::io::commit(&game_state.final_time);
}
