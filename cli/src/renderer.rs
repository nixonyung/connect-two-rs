use anyhow::Context;
use once_cell::sync::Lazy;
use std::io::Write;

static LINE: Lazy<String> = Lazy::new(|| "-".repeat(*game::BOARD_SIZE * 4 + 1));

pub fn read_action(state: &game::State) -> game::Action {
    loop {
        match || -> anyhow::Result<game::Action> {
            if !*crate::IS_TESTING {
                {
                    print!(
                        "Player {} action? {{",
                        match state.player_to_act {
                            game::Player::P1 => "1",
                            game::Player::P2 => "2",
                        }
                    );
                    for (i, act) in state.all_actions().iter().enumerate() {
                        if i != 0 {
                            print!(", ");
                        }
                        print!("{act}");
                    }
                    print!("}}: ");
                }
                std::io::stdout().flush().context("failed stdout.flush")?;
            }

            let mut buf = String::new();
            std::io::stdin()
                .read_line(&mut buf)
                .context("failed stdin.read_line")?;

            let parsed = buf
                .trim()
                .parse::<usize>()
                .context("you should enter a nonnegative integer!")?;

            game::Action::new(state, parsed)
        }() {
            Ok(val) => break val,
            Err(err) => println!("{err}"),
        }
    }
}

pub fn print_state(state: &game::State) {
    if *crate::IS_TESTING {
        return;
    }

    println!();
    println!("{}", *LINE);
    print!("|");
    for cell in &state.board {
        print!(
            " {} |",
            match cell {
                Some(game::Player::P1) => "1",
                Some(game::Player::P2) => "2",
                None => " ",
            }
        );
    }
    println!("");
    println!("{}", *LINE);
}

pub fn print_result(result: &game::Result, orig_state: &game::State) {
    match result {
        game::Result::Win => println!(
            "Player {} wins!",
            match orig_state.player_to_act {
                game::Player::P1 => "1",
                game::Player::P2 => "2",
            }
        ),
        game::Result::Draw => println!("draw!"),
        game::Result::WaitingNextAction => (),
    }
}
