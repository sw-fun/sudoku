//! `SplitMix64`: tiny, pure, byte-stable across platforms and releases, so
//! seeded generation tests stay reproducible forever.

const GOLDEN_GAMMA: u64 = 0x9E37_79B9_7F4A_7C15;
const MIX1: u64 = 0xBF58_476D_1CE4_E5B9;
const MIX2: u64 = 0x94D0_49BB_1331_11EB;

pub struct Rng {
    state: u64,
}

impl Rng {
    #[must_use]
    pub fn new(seed: u64) -> Self {
        Rng { state: seed }
    }

    pub fn next_u64(&mut self) -> u64 {
        self.state = self.state.wrapping_add(GOLDEN_GAMMA);
        let mut z = self.state;
        z = (z ^ (z >> 30)).wrapping_mul(MIX1);
        z = (z ^ (z >> 27)).wrapping_mul(MIX2);
        z ^ (z >> 31)
    }

    /// In-place Fisher-Yates shuffle. Modulo bias is negligible for game use.
    ///
    /// # Panics
    ///
    /// Only via the unreachable `expect("index in range")` (the modulo
    /// result is bounded by the slice length).
    pub fn shuffle<T>(&mut self, slice: &mut [T]) {
        for i in (1..slice.len()).rev() {
            let j = usize::try_from(self.next_u64() % (i + 1) as u64).expect("index in range");
            slice.swap(i, j);
        }
    }
}
