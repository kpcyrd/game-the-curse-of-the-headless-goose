use crate::{
    action,
    fighter::{self, Fighter},
    random::FastRandom,
};
use env_logger::Env;
use std::io::{self, Write};

pub fn main() -> anyhow::Result<()> {
    env_logger::init_from_env(Env::default().default_filter_or("debug"));

    let mut stdout = io::stdout();
    let mut rng = FastRandom::new();

    let stdin = io::stdin();
    let mut lines = stdin.lines();

    let mut us = Fighter::new(fighter::Stats {
        health: 30,
        energy: 10,
        recharge: 1,
        cooldown: 3,
        abilities: 5,
    });

    let mut them = Fighter::new(fighter::Stats {
        health: 30,
        energy: 10,
        recharge: 1,
        cooldown: 2,
        abilities: 10,
    });

    loop {
        println!("Us: {us:?}");
        println!("Them: {them:?}");

        let their_move = them.random_move(&mut rng);
        println!(
            "Their move: {:?} ({:?})",
            their_move,
            their_move.to_decision()
        );

        stdout.write_all(b"> ")?;
        stdout.flush()?;

        let Some(line) = lines.next().transpose()? else {
            break;
        };

        // println!("line={line:?}");
        let Ok(our_move) = line.parse::<action::Move>() else {
            continue;
        };

        println!("Our move: {:?} ({:?})", our_move, our_move.to_decision());

        fighter::turn(&mut rng, &mut us, &mut them, &our_move, &their_move);

        if us.defeated() {
            println!("We have been defeated! Game over.");
            break;
        } else if them.defeated() {
            println!("They have been defeated! You win!");
            break;
        }

        // Recharge energy for both fighters
        for fighter in [&mut us, &mut them] {
            fighter.auto_recharge_energy();
        }

        println!("---");
    }

    Ok(())
}
