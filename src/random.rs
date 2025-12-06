use rand_core::RngCore;

pub trait Rng {
    fn get(&mut self) -> u8;

    fn get_range(&mut self, range: core::ops::Range<u8>) -> u8 {
        loop {
            // TODO: we can try to optimize by cutting off bits to increase odds
            // of being in range, without introducing bias
            let roll = self.get();
            if range.contains(&roll) {
                return roll;
            }
        }
    }
}

pub struct Random<R: RngCore> {
    ascon: ascon::State,
    hwrng: R,
}

impl<R: RngCore> Random<R> {
    pub fn new(hwrng: R) -> Self {
        let ascon = ascon::State::default();
        let mut random = Random { ascon, hwrng };
        random.absorb();
        random
    }

    pub fn absorb(&mut self) {
        let input = self.hwrng.next_u64();
        self.ascon[0] ^= input;
        self.ascon.permute_6();
    }

    pub fn squeeze(&mut self) -> u64 {
        let num = self.ascon[0];
        self.absorb();
        num
    }
}

impl<R: RngCore> Rng for Random<R> {
    fn get(&mut self) -> u8 {
        self.squeeze() as u8
    }
}

#[cfg(not(target_os = "none"))]
pub struct FastRandom {
    rng: fastrand::Rng,
}

#[cfg(not(target_os = "none"))]
impl FastRandom {
    pub fn new() -> Self {
        Self {
            rng: fastrand::Rng::new(),
        }
    }
}

#[cfg(not(target_os = "none"))]
impl Rng for FastRandom {
    fn get(&mut self) -> u8 {
        self.rng.u8(..)
    }
}
