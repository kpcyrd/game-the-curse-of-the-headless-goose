mod action;
mod cooldown;
mod fighter;
mod specials;

use crate::fighter::Fighter;
use std::io::{self, Write};

fn main() -> anyhow::Result<()> {
    let mut stdout = io::stdout();

    let stdin = io::stdin();
    let mut lines = stdin.lines();

    let mut us = Fighter::new(fighter::Stats {
        health: 30,
        energy: 10,
        recharge: 1,
        cooldown: 2,
    });

    let mut them = Fighter::new(fighter::Stats {
        health: 30,
        energy: 10,
        recharge: 1,
        cooldown: 2,
    });

    loop {
        println!("Us: {us:?}");
        println!("Them: {them:?}");

        let their_move = them.random_move();
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

        fighter::turn(&mut us, &mut them, &our_move, &their_move);

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
