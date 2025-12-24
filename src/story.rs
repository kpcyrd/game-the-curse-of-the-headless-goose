use crate::fighter;

pub enum Story {
    Dialogue(&'static [(Decoration, &'static str)]),
    Battle { enemy: fighter::Stats, reward: u16 },
    Hq,
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum Decoration {
    Blank,
    Chapter,
    Sloth,
    Goose,
    Phone,
}

pub const SCENES: &[Story] = &[
    Story::Dialogue(&[
        //
        (Decoration::Blank, "... (Press # to continue)"),
        (Decoration::Blank, "* POW *"),
        (Decoration::Blank, "Get up, get up!"),
        (Decoration::Blank, "..."),
        (Decoration::Blank, "3.."),
        (Decoration::Blank, "2.."),
        (Decoration::Blank, "1.."),
        (Decoration::Blank, "Knockout!"),
    ]),
    Story::Dialogue(&[
        //
        (Decoration::Chapter, "Chapter 1"),
        (Decoration::Sloth, "Oh hey, you're awake."),
        (Decoration::Sloth, "They got you pretty good huh?"),
        (
            Decoration::Sloth,
            "Well, welcome to Silly Con Valley downtown!",
        ),
        (
            Decoration::Sloth,
            "Name's Sloth, I'll be driving you home, you should recover there.",
        ),
        (
            Decoration::Sloth,
            "You know ever since I was a small sloth, I always knew I wanted to move something in this world.",
        ),
        (Decoration::Sloth, "Like from A to B."),
        (
            Decoration::Sloth,
            "I used to drive for sushi delivery. But too many people complained about stale food.",
        ),
        (
            Decoration::Sloth,
            "Now I work as a driver for underground kickboxing clubs.",
        ),
        (
            Decoration::Sloth,
            "Less complaints, and to my surprise, this job even turned out to be less",
        ),
        (Decoration::Sloth, "fishy."),
        (Decoration::Sloth, "heh."),
        (Decoration::Sloth, "Anyway, I digress. We're almost there."),
    ]),
    Story::Dialogue(&[
        //
        (Decoration::Chapter, "Same day, later that night"),
        (Decoration::Phone, "..."),
        (Decoration::Phone, "*RING RING*"),
        (Decoration::Phone, "That fight didn't go so well."),
        (Decoration::Phone, "Maybe I should've actually trained you."),
        (Decoration::Phone, "*sigh*"),
        (
            Decoration::Phone,
            "Okay look, there's 3 different moves you can pick from. Well technically 4, but..",
        ),
        (
            Decoration::Phone,
            "Actually, for you.. there's 2 moves to pick from. You can either attack, or you can block.",
        ),
        (
            Decoration::Phone,
            "You pick a number above zero to attack. You do this amount of damage, but it also costs the same amount of energy.",
        ),
        (
            Decoration::Phone,
            "You can block attacks with zero, which costs no energy, but also deals no damage. And your opponent can obviously do the same.",
        ),
        (
            Decoration::Phone,
            "But there's also cooldown, so picking the same number repeatedly won't work out so well. You can still try, but the more cooldown it has, the higher chance it fails.",
        ),
        (
            Decoration::Phone,
            "Anyway, I hope that makes sense. I arranged a sparring match. Just try not to get knocked out again, okay?",
        ),
        (Decoration::Phone, "*BEEP*"),
        //
        (Decoration::Chapter, "Different place, different time"),
        (Decoration::Blank, "Okay, here goes nothing."),
    ]),
    Story::Battle {
        enemy: fighter::Stats {
            health: 10,
            energy: 10,
            recharge: 1,
            cooldown: 4,
            abilities: 0b1101,
        },
        reward: 0,
    },
    Story::Dialogue(&[
        //
        (
            Decoration::Phone,
            "Excellent! Next is a tougher one, this may take you a few tries.",
        ),
    ]),
    Story::Battle {
        enemy: fighter::Stats {
            health: 10,
            energy: 10,
            recharge: 1,
            cooldown: 2,
            abilities: 0b1101,
        },
        reward: 10,
    },
    Story::Hq,
    Story::Dialogue(&[(Decoration::Chapter, "Epilogue")]),
];
