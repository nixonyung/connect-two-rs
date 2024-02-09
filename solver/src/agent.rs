use std::collections::{HashMap, HashSet};

pub type StateAction = (crate::EncodedState, game::Action);

pub struct Agent(HashMap<crate::EncodedState, HashMap<game::Action, crate::Reward>>);

impl Agent {
    fn new() -> Agent {
        Agent(HashMap::default())
    }

    pub fn reward(
        &self,
        at_encoded_state: &crate::EncodedState,
        action: &game::Action,
    ) -> &crate::Reward {
        &self.0[at_encoded_state][action]
    }

    pub fn max_value(&self, at_encoded_state: &crate::EncodedState) -> crate::Value {
        self.0[at_encoded_state]
            .values()
            .max_by(|a, b| a.value.cmp(&b.value))
            .and_then(|r| Some(r.value))
            .unwrap_or_default()
    }

    pub fn optimal_actions(&self, at_encoded_state: &crate::EncodedState) -> Vec<game::Action> {
        self.0[at_encoded_state]
            .iter()
            // (ref.) [How do I idiomatically convert a bool to an Option or Result in Rust?](https://stackoverflow.com/questions/54841351/how-do-i-idiomatically-convert-a-bool-to-an-option-or-result-in-rust)
            .filter_map(|(action, reward)| {
                (reward.value == self.max_value(at_encoded_state)).then_some(*action)
            })
            .collect()
    }

    fn init_agents(p1_agent: &mut Agent, p2_agent: &mut Agent) {
        type Unexplored = HashSet<(
            game::State,
            crate::EncodedState, // OPTIMIZATION: reuse encoded_state during exploring
        )>;

        // BFS
        // ASSUMPTION: there are no loops in states, i.e. the state-space is a one-way tree without loops
        let mut curr_player = game::Player::new();
        let mut unexplored: Unexplored = HashSet::from([(
            game::State::new().clone(),
            crate::EncodedState::new(&game::INITIAL_STATE),
        )]);
        while !unexplored.is_empty() {
            let curr_agent = match curr_player {
                game::Player::P1 => &mut *p1_agent,
                game::Player::P2 => &mut *p2_agent,
            };
            let mut new_unexplored: Unexplored = HashSet::new();
            for (s, encoded_s) in unexplored {
                for act in s.all_actions() {
                    let (s_next, result) = s.step(&act);
                    let encoded_s_next = crate::EncodedState::new(&s_next);

                    // init agent
                    // (ref.) [How to lookup from and insert into a HashMap efficiently?](https://stackoverflow.com/questions/28512394/how-to-lookup-from-and-insert-into-a-hashmap-efficiently)
                    curr_agent
                        .0
                        .entry(encoded_s.clone())
                        .or_insert(HashMap::new())
                        .entry(act)
                        .or_insert(crate::Reward::new(&encoded_s_next, &result));

                    if matches!(result, game::Result::WaitingNextAction) {
                        new_unexplored.insert((s_next, encoded_s_next));
                    }
                }
            }
            unexplored = new_unexplored;
            curr_player = curr_player.next();
        }
    }

    /// return `true` if any value is updated, `false` otherwise
    fn backtrack(
        agent: &mut Agent,
        trajectory: &Vec<crate::StateAction>,
        value: crate::Value,
        curr_epoch: &u32,
    ) -> bool {
        let mut has_update = false;
        for state_action in trajectory.iter().rev() {
            let curr_reward = agent
                .0
                .get_mut(&state_action.0)
                .and_then(|m| m.get_mut(&state_action.1))
                .unwrap();

            if curr_reward.last_visited_at == *curr_epoch {
                if value > curr_reward.value {
                    curr_reward.value = value;
                    has_update = true;
                }
            } else {
                if value != curr_reward.value {
                    curr_reward.value = value;
                    has_update = true;
                }
            }

            curr_reward.last_visited_at = *curr_epoch;
        }
        // debug
        println!(
            "backtracking...  [{value}] ({})  (has_update={has_update})",
            trajectory
                .iter()
                .map(|(state, action)| format!("{} -> {}", state, action))
                .collect::<Vec<_>>()
                .join(" ")
        );
        has_update
    }

    /// return `true` if any value is updated, `false` otherwise
    fn train(
        target: &mut Agent,
        opponent: &Agent,
        curr_epoch: &u32,
        initial_encoded_states: &Vec<&crate::EncodedState>,
    ) -> bool {
        // debug
        {
            println!();
            println!("curr_epoch = {curr_epoch}:");
        }

        let mut has_update = false;

        // DFS without recursion
        // ASSUMPTION: there are no loops in states, i.e. the state-space is a one-way tree without loops
        let mut trajectory: Vec<crate::StateAction> = Vec::new(); // LIFO
        let mut unexplored: Vec<(
            crate::StateAction,
            usize, // trajectory_len: a helper state storing the previous trajectory length
        )> = initial_encoded_states
            .iter()
            .flat_map(|s| {
                target
                    .optimal_actions(s)
                    .iter()
                    .map(|act| (((*s).clone(), *act), 0usize))
                    .collect::<Vec<_>>()
            })
            .collect();
        while let Some(((s, act), trajectory_len)) = unexplored.pop() {
            // (ref.) [How do I get n elements from Vec?](https://www.reddit.com/r/rust/comments/2ooe03/how_do_i_get_n_elements_from_vec/)
            trajectory = {
                let mut v = trajectory[0..trajectory_len].to_vec();
                v.push((s.clone(), act));
                v
            };

            let reward = target.reward(&s, &act);
            if let Some(value) = {
                // OPTIMIZATION: use the reward's value as is if the choice is already explored in this epoch
                if reward.last_visited_at == *curr_epoch {
                    Some(reward.value)
                } else {
                    match reward.result {
                        game::Result::Win => Some(crate::Value::WIN),
                        game::Result::Draw => Some(crate::Value::DRAW),
                        game::Result::WaitingNextAction => {
                            let mut max_value = crate::Value::new();
                            let s_next = &reward.to_encoded_state;
                            for reward_oppo in opponent
                                .optimal_actions(&s_next)
                                .into_iter()
                                .map(|act_oppo| opponent.reward(&s_next, &act_oppo))
                            {
                                match reward_oppo.result {
                                    game::Result::Win => {
                                        max_value = max_value.max(crate::Value::LOSE)
                                    }
                                    game::Result::Draw => {
                                        max_value = max_value.max(crate::Value::DRAW)
                                    }
                                    game::Result::WaitingNextAction => {
                                        let s_next_next = &reward_oppo.to_encoded_state;
                                        unexplored.extend(
                                            target.optimal_actions(s_next_next).into_iter().map(
                                                |act| {
                                                    ((s_next_next.clone(), act), trajectory.len())
                                                },
                                            ),
                                        )
                                    }
                                }
                            }
                            (!matches!(max_value, crate::Value::UNDEFINED)).then_some(max_value)
                        }
                    }
                }
            } {
                if Agent::backtrack(target, &trajectory, value, &curr_epoch) {
                    has_update = true;
                }
            }
        }
        has_update
    }

    pub fn new_trained() -> (Agent, Agent) {
        let encoded_initial_state: &crate::EncodedState =
            &crate::EncodedState::new(&game::INITIAL_STATE);

        let mut p1_agent = Agent::new();
        let mut p2_agent = Agent::new();

        Agent::init_agents(&mut p1_agent, &mut p2_agent);

        let mut curr_epoch = 0;
        let mut curr_player = game::Player::new();
        loop {
            curr_epoch += 1;
            if !{
                match curr_player {
                    game::Player::P1 => Agent::train(
                        &mut p1_agent,
                        &p2_agent,
                        &curr_epoch,
                        &vec![encoded_initial_state],
                    ),
                    game::Player::P2 => Agent::train(
                        &mut p2_agent,
                        &p1_agent,
                        &curr_epoch,
                        &p1_agent
                            .optimal_actions(encoded_initial_state)
                            .iter()
                            .map(|act| {
                                &p1_agent.reward(encoded_initial_state, act).to_encoded_state
                            })
                            .collect(),
                    ),
                }
            } {
                break;
            }
            // debug
            match curr_player {
                game::Player::P1 => println!("{p1_agent}"),
                game::Player::P2 => println!("{p2_agent}"),
            }
            curr_player = curr_player.next();
        }

        (p1_agent, p2_agent)
    }
}

impl std::fmt::Display for Agent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let encoded_states = {
            let mut v = self.0.keys().collect::<Vec<_>>();
            v.sort_unstable();
            v
        };
        for (i, encoded_state) in encoded_states.iter().enumerate() {
            let optimal_actions = {
                let mut v = self.optimal_actions(encoded_state);
                v.sort_unstable();
                v
            };
            {
                write!(
                    f,
                    "{encoded_state} [{}] -> {{",
                    self.max_value(encoded_state)
                )?;
                for (i, act) in optimal_actions.iter().enumerate() {
                    if i != 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{act}")?;
                }
                write!(f, "}}")?;
                if i != encoded_states.len() - 1 {
                    writeln!(f, "")?;
                }
            }
        }
        Ok(())
    }
}
