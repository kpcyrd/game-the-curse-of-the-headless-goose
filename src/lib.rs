#![cfg_attr(target_os = "none", no_std)]

pub mod action;
#[cfg(not(target_os = "none"))]
pub mod cli;
pub mod cooldown;
pub mod fighter;
#[cfg(target_os = "none")]
pub mod firmware;
pub mod gfx;
pub mod input;
#[cfg(target_os = "none")]
pub mod keypad;
pub mod machine;
pub mod random;
pub mod save;
pub mod specials;
pub mod story;
pub mod timer;
pub mod turn;
