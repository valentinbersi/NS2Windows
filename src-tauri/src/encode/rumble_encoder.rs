use crate::data::ns_controller_kind::NsControllerKind;

#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct RumbleEncoder {
    counter: u8,
    intensity_accumulator: u16,
    last_amplitude: u8,
}

impl RumbleEncoder {
    pub fn new() -> Self {
        Self::default()
    }

    const WEAK: [u8; 5] = [0x4b, 0x7d, 0x80, 0x5a, 0x02];
    const STRONG: [u8; 5] = [0x93, 0x35, 0x36, 0x1c, 0x0d];
    const WEAK_MAX_AMPLITUDE: u8 = 127;
    const ONE_PERCENT_AMPLITUDE: u8 = 3;
    const WEAK_START_DUTY: u16 = 10 * u8::MAX as u16 / 49;
    const STRONG_START_DUTY: u16 = 20 * u8::MAX as u16 / 50;

    fn interpolate_duty(amplitude: u8, input_start: u8, input_end: u8, duty_start: u16) -> u16 {
        let position = amplitude.saturating_sub(input_start) as u16;
        let range = (input_end - input_start) as u16;
        duty_start + position * (u8::MAX as u16 - duty_start) / range
    }

    fn pattern_and_duty(amplitude: u8) -> ([u8; 5], u16) {
        if amplitude <= Self::WEAK_MAX_AMPLITUDE {
            let duty = if amplitude <= Self::ONE_PERCENT_AMPLITUDE {
                amplitude as u16 * Self::WEAK_START_DUTY / Self::ONE_PERCENT_AMPLITUDE as u16
            } else {
                Self::interpolate_duty(
                    amplitude,
                    Self::ONE_PERCENT_AMPLITUDE,
                    Self::WEAK_MAX_AMPLITUDE,
                    Self::WEAK_START_DUTY,
                )
            };

            (Self::WEAK, duty)
        } else {
            let strong_start = Self::WEAK_MAX_AMPLITUDE + 1;
            let duty =
                Self::interpolate_duty(amplitude, strong_start, u8::MAX, Self::STRONG_START_DUTY);

            (Self::STRONG, duty)
        }
    }

    fn pattern(&mut self, amplitude: u8) -> Option<[u8; 5]> {
        if amplitude != self.last_amplitude {
            self.intensity_accumulator = 0;
            self.last_amplitude = amplitude;
        }

        if amplitude == 0 {
            self.intensity_accumulator = 0;
            return None;
        }

        let (pattern, duty) = Self::pattern_and_duty(amplitude);

        self.intensity_accumulator += duty;

        if self.intensity_accumulator >= u8::MAX as u16 {
            self.intensity_accumulator -= u8::MAX as u16;
            Some(pattern)
        } else {
            None
        }
    }

    fn block(&mut self, amplitude: u8) -> [u8; 16] {
        let mut block = [0; 16];

        let Some(pattern) = self.pattern(amplitude) else {
            return block;
        };

        block[0] = 0x50 | (self.counter & 0x0f);
        block[1..6].copy_from_slice(&pattern);
        self.counter = self.counter.wrapping_add(1) & 0x0f;
        block
    }

    pub fn packet(&mut self, kind: NsControllerKind, amplitude: u8) -> Vec<u8> {
        if kind == NsControllerKind::NsoGcController {
            // Deliberately return no output: the GC rumble protocol is not verified.
            return vec![];
        }

        let mut packet = vec![0; 42];

        if amplitude == 0 {
            self.intensity_accumulator = 0;
            self.last_amplitude = 0;
            return packet;
        }

        let block = self.block(amplitude);

        match kind {
            NsControllerKind::LeftJoyCon | NsControllerKind::RightJoyCon => {
                packet[1..17].copy_from_slice(&block);
            }

            NsControllerKind::ProController => {
                packet[1..17].copy_from_slice(&block);
                packet[17..33].copy_from_slice(&block);
            }
            NsControllerKind::NsoGcController => unreachable!(),
        }

        packet
    }
}

#[cfg(test)]
mod tests {
    use super::RumbleEncoder;
    use crate::data::ns_controller_kind::NsControllerKind;

    #[test]
    fn joy_con_and_pro_reports_keep_their_documented_sizes() {
        let mut encoder = RumbleEncoder::default();

        assert_eq!(encoder.packet(NsControllerKind::LeftJoyCon, 1).len(), 42);
        assert_eq!(encoder.packet(NsControllerKind::ProController, 1).len(), 42);
    }

    #[test]
    fn one_percent_maps_to_the_previous_ten_percent_weak_duty() {
        let (pattern, duty) = RumbleEncoder::pattern_and_duty(RumbleEncoder::ONE_PERCENT_AMPLITUDE);

        assert_eq!(pattern, RumbleEncoder::WEAK);
        assert_eq!(duty, RumbleEncoder::WEAK_START_DUTY);
    }

    #[test]
    fn forty_nine_percent_uses_continuous_weak_rumble() {
        let mut encoder = RumbleEncoder::default();

        for _ in 0..32 {
            let packet = encoder.packet(
                NsControllerKind::LeftJoyCon,
                RumbleEncoder::WEAK_MAX_AMPLITUDE,
            );
            assert_eq!(packet[2..7], RumbleEncoder::WEAK);
        }
    }

    #[test]
    fn fifty_percent_maps_to_the_previous_seventy_percent_strong_duty() {
        let (pattern, duty) = RumbleEncoder::pattern_and_duty(128);

        assert_eq!(pattern, RumbleEncoder::STRONG);
        assert_eq!(duty, RumbleEncoder::STRONG_START_DUTY);
    }

    #[test]
    fn both_bands_increase_monotonically() {
        let mut previous_weak_duty = 0;
        for amplitude in 1..=RumbleEncoder::WEAK_MAX_AMPLITUDE {
            let (_, duty) = RumbleEncoder::pattern_and_duty(amplitude);
            assert!(duty >= previous_weak_duty);
            previous_weak_duty = duty;
        }

        let mut previous_strong_duty = 0;
        for amplitude in (RumbleEncoder::WEAK_MAX_AMPLITUDE + 1)..=u8::MAX {
            let (_, duty) = RumbleEncoder::pattern_and_duty(amplitude);
            assert!(duty >= previous_strong_duty);
            previous_strong_duty = duty;
        }
    }

    #[test]
    fn full_intensity_uses_continuous_strong_rumble() {
        let mut encoder = RumbleEncoder::default();

        for _ in 0..32 {
            let packet = encoder.packet(NsControllerKind::LeftJoyCon, u8::MAX);
            assert_eq!(packet[2..7], RumbleEncoder::STRONG);
        }
    }
}
