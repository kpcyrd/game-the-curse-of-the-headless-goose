pub enum Story {
    Dialogue(&'static [(Decoration, &'static str)]),
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum Decoration {
    Blank,
    Chapter,
    Sloth,
    Goose,
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
            "I'm driving you home, you should recover there.",
        ),
        (
            Decoration::Sloth,
            "You know ever since I was a small sloth, I always knew I wanted to be a driver.",
        ),
        (Decoration::Sloth, "I used to drive for sushi delivery."),
        (
            Decoration::Sloth,
            "But too many people complained about stale food.",
        ),
        (
            Decoration::Sloth,
            "Now I work as a driver for underground kickboxing clubs.",
        ),
        (
            Decoration::Sloth,
            "Less complaints, and to my surprise, this job event turned out to be less",
        ),
        (Decoration::Sloth, "fishy."),
        (Decoration::Sloth, "heh."),
        (Decoration::Sloth, "Anyway, I digress. We're almost there."),
    ]),
    Story::Dialogue(&[(Decoration::Chapter, "Chapter 2")]),
    Story::Dialogue(&[(Decoration::Chapter, "Chapter 3")]),
    Story::Dialogue(&[(Decoration::Chapter, "Epilogue")]),
];
