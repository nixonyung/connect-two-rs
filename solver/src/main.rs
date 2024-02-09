mod agent;
mod encoding;
mod reward;
use agent::*;
use encoding::*;
use reward::*;

fn main() {
    let (p1_agent, p2_agent) = Agent::new_trained();
    println!("");
    println!("result:");
    println!("");
    println!("{p1_agent}");
    println!("");
    println!("{p2_agent}");
}
