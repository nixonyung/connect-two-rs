mod renderer;
use renderer::*;

use once_cell::sync::Lazy;

static IS_TESTING: Lazy<bool> = Lazy::new(|| {
    match || -> anyhow::Result<bool> {
        let val = std::env::var("TESTING")?;
        Ok(val == "1")
    }() {
        Ok(val) => val,
        Err(_) => false,
    }
});

fn main() {
    let mut state = game::State::new();
    crate::print_state(&state);

    loop {
        let action = read_action(&state);
        let (next_state, result) = state.step(&action);
        crate::print_state(&next_state);
        crate::print_result(&result, &state);
        match result {
            game::Result::WaitingNextAction => state = next_state,
            _ => break,
        }
    }
}
