

pub struct Rand {
    seed: u64,
}

impl Default for Rand {
    fn default() -> Rand {
        let time_since_epoch = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        Rand {
            seed: time_since_epoch,
        }
    }
}

impl Rand {
    pub fn new(seed: u64) -> Rand {
        Rand { seed }
    }

    pub fn rand(&mut self) -> u64 {
        let state = self.seed.wrapping_mul(747796405).wrapping_add(2891336453);
        let word = ((state >> ((state >> 28) + 4)) ^ state).wrapping_mul(277803737);
        let result = (word >> 22) ^ word;
        self.seed = result;
        result
    }
}
