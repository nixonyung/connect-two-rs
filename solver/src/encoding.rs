use bimap::BiMap;
use fixedbitset::FixedBitSet;
use once_cell::sync::Lazy;

static N: Lazy<usize> = Lazy::new(|| 2 * *game::BOARD_SIZE);

static PLAYER_ENCODING: Lazy<BiMap<Option<game::Player>, (bool, bool)>> = Lazy::new(|| {
    BiMap::from_iter([
        (None, (false, false) /* 00 */),
        (Some(game::Player::P1), (false, true) /* 01 */),
        (Some(game::Player::P2), (true, false) /* 10 */),
    ])
});

#[derive(
    // sane defaults for value objects:
    Clone,
    Debug,
    // hashable:
    PartialEq,
    Eq,
    Hash,
    // sortable:
    PartialOrd,
    Ord,
)]
pub struct EncodedState(
    pub FixedBitSet, // note: FixedBitSet[0] is the least significant bit
);

impl EncodedState {
    pub fn new(state: &game::State) -> EncodedState {
        let mut bitset = FixedBitSet::with_capacity(*N);
        for (i, player) in state.board.iter().enumerate() {
            let (bit1, bit2) = *PLAYER_ENCODING.get_by_left(&player).unwrap();
            // updating bitset from the most significant bit so that the ordering is more intuitive,
            // i.e. None < Some(P1) < Some(P2), and state.board[0] is the most significant bit
            bitset.set((*N - 1) - (2 * i), bit1);
            bitset.set((*N - 1) - (2 * i + 1), bit2);
        }
        EncodedState(bitset)
    }
}

impl std::fmt::Display for EncodedState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            (0..*game::BOARD_SIZE)
                .into_iter()
                .map(|i| {
                    match PLAYER_ENCODING
                        .get_by_right(&(self.0[(*N - 1) - (2 * i)], self.0[(*N - 1) - (2 * i + 1)]))
                        .unwrap()
                    {
                        Some(game::Player::P1) => "1",
                        Some(game::Player::P2) => "2",
                        None => "_",
                    }
                })
                .collect::<Vec<_>>()
                .join(",")
        )
    }
}
