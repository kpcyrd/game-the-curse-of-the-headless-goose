use crate::input::Event as Key;
use embedded_hal::digital::InputPin;
use waveshare_rp2040_zero::hal::gpio::{
    FunctionSio, Pin, PinId, PullType, PullUp, SioInput, SioOutput, ValidFunction,
};

pub struct Keypad<P1: PinId, P2: PinId, P3: PinId, P4: PinId, P5: PinId, P6: PinId, P7: PinId> {
    pub c1: Pin<P1, FunctionSio<SioInput>, PullUp>,
    pub c2: Pin<P2, FunctionSio<SioInput>, PullUp>,
    pub c3: Pin<P3, FunctionSio<SioInput>, PullUp>,
    pub r1: Pin<P4, FunctionSio<SioInput>, PullUp>,
    pub r2: Pin<P5, FunctionSio<SioInput>, PullUp>,
    pub r3: Pin<P6, FunctionSio<SioInput>, PullUp>,
    pub r4: Pin<P7, FunctionSio<SioInput>, PullUp>,
}

impl<
    P1: PinId + ValidFunction<FunctionSio<SioOutput>> + ValidFunction<FunctionSio<SioInput>>,
    P2: PinId + ValidFunction<FunctionSio<SioOutput>> + ValidFunction<FunctionSio<SioInput>>,
    P3: PinId + ValidFunction<FunctionSio<SioOutput>> + ValidFunction<FunctionSio<SioInput>>,
    P4: PinId + ValidFunction<FunctionSio<SioOutput>> + ValidFunction<FunctionSio<SioInput>>,
    P5: PinId + ValidFunction<FunctionSio<SioOutput>> + ValidFunction<FunctionSio<SioInput>>,
    P6: PinId + ValidFunction<FunctionSio<SioOutput>> + ValidFunction<FunctionSio<SioInput>>,
    P7: PinId + ValidFunction<FunctionSio<SioOutput>> + ValidFunction<FunctionSio<SioInput>>,
> Keypad<P1, P2, P3, P4, P5, P6, P7>
{
    fn probe<I, P>(press: &mut Option<Key>, pin: &mut Pin<I, FunctionSio<SioInput>, P>, key: Key)
    where
        I: PinId,
        P: PullType,
    {
        if pin.is_low().unwrap() {
            *press = Some(key);
        }
    }

    // The return value is slightly awkward, but makes ownership easier to work with
    pub fn read(mut self, press: &mut Option<Key>) -> Self {
        // Check column 1 (keys: 1, 4, 7, *)
        let c1 = self.c1.into_push_pull_output();
        let c1 = c1.into_pull_up_input();

        Self::probe(press, &mut self.r1, Key::One);
        Self::probe(press, &mut self.r2, Key::Four);
        Self::probe(press, &mut self.r3, Key::Seven);
        Self::probe(press, &mut self.r4, Key::Star);

        // Check column 2 (keys: 2, 5, 8, 0)
        let c2 = self.c2.into_push_pull_output();
        let c2 = c2.into_pull_up_input();

        Self::probe(press, &mut self.r1, Key::Two);
        Self::probe(press, &mut self.r2, Key::Five);
        Self::probe(press, &mut self.r3, Key::Eight);
        Self::probe(press, &mut self.r4, Key::Zero);

        // Check column 3 (keys: 3, 6, 9, #)
        let c3 = self.c3.into_push_pull_output();
        let c3 = c3.into_pull_up_input();

        Self::probe(press, &mut self.r1, Key::Three);
        Self::probe(press, &mut self.r2, Key::Six);
        Self::probe(press, &mut self.r3, Key::Nine);
        Self::probe(press, &mut self.r4, Key::Hash);

        Keypad {
            c1,
            c2,
            c3,
            r1: self.r1,
            r2: self.r2,
            r3: self.r3,
            r4: self.r4,
        }
    }
}
