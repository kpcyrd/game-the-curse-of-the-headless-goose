#![cfg_attr(target_os = "none", no_std)]
#![cfg_attr(target_os = "none", no_main)]

#[cfg(target_os = "none")]
use defmt_rtt as _;
#[cfg(target_os = "none")]
use rp2040_panic_usb_boot as _;

#[cfg(target_os = "none")]
#[allow(unused_imports)]
use game_the_curse_of_the_headless_goose::firmware::*;

#[cfg(not(target_os = "none"))]
#[allow(unused_imports)]
use game_the_curse_of_the_headless_goose::cli::*;
