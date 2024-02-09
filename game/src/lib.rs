mod player;
mod state;
pub use player::*;
pub use state::*;

use once_cell::sync::Lazy;

pub static BOARD_SIZE: Lazy<usize> = Lazy::new(|| {
    match || -> anyhow::Result<usize> {
        let val = std::env::var("BOARD_SIZE")?;
        let val = val.parse::<usize>()?;
        Ok(val)
    }() {
        Ok(val) => val,
        Err(_) => 4,
    }
});
