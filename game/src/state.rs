use once_cell::sync::Lazy;

#[derive(
    // sane defaults for value objects:
    Clone,
    Copy,
    derive_more::Display,
    Debug,
    // hashable:
    PartialEq,
    Eq,
    Hash,
    // sortable:
    PartialOrd,
    Ord,
)]
#[display(fmt = "({})", col)]
pub struct Action {
    pub col: usize,
}

impl Action {
    pub fn new(state: &State, col: usize) -> anyhow::Result<Action> {
        let new_action = Action { col };
        anyhow::ensure!(
            state.all_actions().contains(&new_action),
            "invalid action for the current state!"
        );
        Ok(new_action)
    }
}

#[derive(
    // sane defaults for unit-like enums:
    Clone,
    Copy,
    derive_more::Display,
    Debug,
)]
pub enum Result {
    WaitingNextAction,
    Draw,
    Win,
}

pub type Cell = Option<crate::Player>;

#[derive(
    Clone,
    // hashable:
    PartialEq,
    Eq,
    Hash,
)]
pub struct State {
    pub board: Vec<Cell>,
    pub player_to_act: crate::Player,
}

// OPTIMIZATION: can be referenced by outside
pub static INITIAL_STATE: Lazy<State> = Lazy::new(|| State {
    // (ref.) [How to initialize Vec<Option<T>> with None](https://users.rust-lang.org/t/how-to-initialize-vec-option-t-with-none/30580)
    board: std::iter::repeat(None).take(*crate::BOARD_SIZE).collect(),
    player_to_act: crate::Player::new(),
});

impl State {
    pub fn new() -> State {
        INITIAL_STATE.clone()
    }

    pub fn all_actions(&self) -> Vec<crate::Action> {
        self.board
            .iter()
            .enumerate()
            .filter_map(|(col, cell)| match cell {
                None => Some(crate::Action { col }),
                Some(_) => None,
            })
            .collect()
    }

    pub fn step(&self, action: &crate::Action) -> (State, crate::Result) {
        let new_state = State {
            board: self
                .board
                .iter()
                .enumerate()
                .map(|(col, cell)| {
                    if col == action.col {
                        Some(self.player_to_act)
                    } else {
                        *cell
                    }
                })
                .collect(),
            player_to_act: self.player_to_act.next(),
        };

        let result = if action.col > 0
            && self.board[action.col - 1].is_some_and(|p| p == self.player_to_act)
        {
            crate::Result::Win
        } else if action.col < *crate::BOARD_SIZE - 1
            && self.board[action.col + 1].is_some_and(|p| p == self.player_to_act)
        {
            crate::Result::Win
        } else if new_state.all_actions().is_empty() {
            crate::Result::Draw
        } else {
            crate::Result::WaitingNextAction
        };

        (new_state, result)
    }
}
