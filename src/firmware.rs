// use eh0::timer::CountDown;
use embedded_graphics::{
    draw_target::DrawTarget,
    pixelcolor::{Rgb666, RgbColor},
};
use waveshare_rp2040_zero::{
    Pins, XOSC_CRYSTAL_FREQ, entry,
    hal::{
        Sio,
        clocks::{Clock, init_clocks_and_plls},
        fugit::{/*ExtU32,*/ RateExtU32},
        gpio, pac,
        rosc::RingOscillator,
        spi,
        timer::Timer,
        watchdog::Watchdog,
    },
};

#[entry]
fn main() -> ! {
    let mut pac = pac::Peripherals::take().unwrap();

    // Configure clocks and timers
    let mut watchdog = Watchdog::new(pac.WATCHDOG);
    let clocks = init_clocks_and_plls(
        XOSC_CRYSTAL_FREQ,
        pac.XOSC,
        pac.CLOCKS,
        pac.PLL_SYS,
        pac.PLL_USB,
        &mut pac.RESETS,
        &mut watchdog,
    )
    .ok()
    .unwrap();
    let mut timer = Timer::new(pac.TIMER, &mut pac.RESETS, &clocks);

    let rosc = RingOscillator::new(pac.ROSC).initialize();
    // let mut rng = Random::new(rosc);

    // Configure gpio
    let sio = Sio::new(pac.SIO);
    let pins = Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,
    );

    loop {}
}
