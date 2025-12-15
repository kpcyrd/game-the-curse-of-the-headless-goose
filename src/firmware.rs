use crate::{
    action::Move,
    fighter::{self, Fighter},
    gfx,
    keypad::Keypad,
    machine::{Battle, Render, Scene},
    random::Random,
};
// use eh0::timer::CountDown;
use embedded_graphics::{
    draw_target::DrawTarget,
    pixelcolor::{Rgb666, RgbColor},
};
use embedded_hal_bus::spi::ExclusiveDevice;
use mipidsi::{
    interface::SpiInterface,
    models::ILI9486Rgb666,
    options::{ColorOrder, Orientation, Rotation},
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
    let mut rng = Random::new(rosc);

    // Configure gpio
    let sio = Sio::new(pac.SIO);
    let pins = Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,
    );

    // Set up our SPI pins so they can be used by the SPI driver
    let mosi = pins.gp15.into_function::<gpio::FunctionSpi>();
    let sck = pins.gp14.into_function::<gpio::FunctionSpi>();
    let cs = pins.gp1.into_push_pull_output();
    let dc = pins.gp28.into_push_pull_output();
    let reset = pins.gp0.into_push_pull_output();

    let spi = spi::Spi::<_, _, _, 8>::new(pac.SPI1, (mosi, sck));

    // Exchange the uninitialised SPI driver for an initialised one
    let spi = spi.init(
        &mut pac.RESETS,
        clocks.peripheral_clock.freq(),
        60_u32.MHz(),
        embedded_hal::spi::MODE_0,
    );
    let spi = ExclusiveDevice::new_no_delay(spi, cs).unwrap();

    // Configure display
    let mut buffer = [0_u8; 512];
    let di = SpiInterface::<_, _>::new(spi, dc, &mut buffer);

    let mut display = mipidsi::Builder::new(ILI9486Rgb666, di)
        .reset_pin(reset)
        .orientation(
            Orientation::new()
                .rotate(Rotation::Deg180)
                .flip_horizontal(),
        )
        .color_order(ColorOrder::Bgr)
        .init(&mut timer)
        .unwrap();
    display.clear(Rgb666::BLACK).unwrap();

    // Keypad pins
    let c2 = pins.gp13.into_pull_up_input();
    let r1 = pins.gp12.into_pull_up_input();
    let c1 = pins.gp11.into_pull_up_input();
    let r4 = pins.gp10.into_pull_up_input();
    let c3 = pins.gp9.into_pull_up_input();
    let r3 = pins.gp8.into_pull_up_input();
    let r2 = pins.gp7.into_pull_up_input();

    let mut keypad = Keypad {
        c1,
        c2,
        c3,
        r1,
        r2,
        r3,
        r4,
    };

    // let mut delay = timer.count_down();

    let mut scene = Scene::Battle(Battle {
        player: {
            let mut us = Fighter::new(fighter::Stats {
                health: 10,
                energy: 10,
                recharge: 1,
                cooldown: 4,
                abilities: 5,
            });
            us.cooldown.get_mut(&Move::Zero).unwrap().value = 0;
            us.cooldown.get_mut(&Move::Two).unwrap().value = 1;
            us.cooldown.get_mut(&Move::Three).unwrap().value = 2;
            us.cooldown.get_mut(&Move::Four).unwrap().value = 3;
            us
        },
        enemy: {
            let mut them = Fighter::new(fighter::Stats {
                health: 30,
                energy: 10,
                recharge: 1,
                cooldown: 2,
                abilities: 10,
            });
            them.cooldown.get_mut(&Move::Five).unwrap().value = 1;
            them
        },
        their_move: Move::Five,
    });

    // keypad input handling
    let mut current_key = None;
    let mut render = None;

    // game loop
    loop {
        if render == Some(Render::Clear) {
            display.clear(Rgb666::BLACK).unwrap();
            render = Some(Render::Redraw);
        }

        match &scene {
            Scene::Intro(intro) => gfx::intro::render(&mut display, intro),
            Scene::Dialogue(dialogue) => gfx::dialogue::render(&mut display, dialogue),
            Scene::Battle(battle) => {
                gfx::battle::render(&mut display, battle);
            }
        }

        /*
        delay.start(1.secs());
        let _ = nb::block!(delay.wait());
        */

        // events
        let mut key = None;
        keypad = keypad.read(&mut key);

        current_key = if let Some(key) = key {
            if Some(key) != current_key {
                render = scene.update(&mut rng, key);
            }
            Some(key)
        } else {
            None
        };

        scene.tick();
    }
}
