pub enum Story {
    Dialogue(&'static [&'static str]),
}

pub const SCENES: &[Story] = &[
    Story::Dialogue(&["page1", "page 2"]),
    Story::Dialogue(&["Chapter 1"]),
    Story::Dialogue(&["Chapter 2", "page 2", "page 3"]),
    Story::Dialogue(&["Chapter 3"]),
    Story::Dialogue(&["Epilogue"]),
];
