#[derive(
    Default,
    // sane defaults for unit-like enums:
    Clone,
    Copy,
    derive_more::Display,
    Debug,
    // sortable:
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
)]
pub enum Value {
    #[default]
    UNDEFINED,
    LOSE,
    DRAW,
    WIN,
}

impl Value {
    pub fn new() -> Value {
        Value::default()
    }
}

pub struct Reward {
    // what the agent knows must happen:
    pub to_encoded_state: crate::EncodedState,
    pub result: game::Result,

    // what the agent thinks will happen:
    pub value: crate::Value,

    pub last_visited_at: u32,
    pub last_updated_at: u32,
}

impl Reward {
    pub fn new(to: &crate::EncodedState, result: &game::Result) -> Reward {
        Reward {
            to_encoded_state: to.clone(),
            result: *result,
            value: match result {
                game::Result::WaitingNextAction => crate::Value::new(),
                game::Result::Draw => crate::Value::DRAW,
                game::Result::Win => crate::Value::WIN,
            },
            // note: curr_epoch should start from 1
            last_visited_at: 0,
            last_updated_at: 0,
        }
    }
}
