use crate::models::{game_state::GameState, player_action::PlayerAction};
use crate::monte_carlo::bounds::Bounds;
use crate::monte_carlo::function::Function;
use crate::monte_carlo::linear_bounds::LinearBounds;

pub fn decide(game_state: GameState) -> Vec<PlayerAction> {

    // actions to take
    let mut actions: Vec<PlayerAction> = Vec::new();

    // for every base
    game_state.bases.iter().for_each(|base| {
        // create a function with base populations as coefficients
        let func: Function = Function::new(game_state.bases.iter().map(|base| {base.population as f64}).collect());

        if base.player == game_state.game.player {
            // create a vector with bounds
            let mut mixed_bound: Vec<u32> = vec![base.uid, base.population];
            // sort bounds
            mixed_bound.sort();

            // create linear bounds from sorted bounds
            let linear_bounds = LinearBounds::new(mixed_bound[0] as f64, mixed_bound[1] as f64);
            // create bounds
            let bounds = Bounds::new(linear_bounds, LinearBounds::new(-100_000.0, 100_000.0));

            // push action from base to random other base with 1 / 3 of troops
            actions.push(PlayerAction {src: base.uid, dest: (func.approximate(&bounds,  1_000_000, 1).abs() as u32) % game_state.bases.len() as u32, amount: base.population / 3})
        }
    });

    return actions;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decide_test() {
        let want = vec![PlayerAction::default()];

        let result = decide(GameState::default());

        assert!(want == result)
    }
}
