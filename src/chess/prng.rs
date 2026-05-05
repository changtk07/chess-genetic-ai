struct PRNG {
    state: u64,
}

impl PRNG {
    const fn new(seed: u64) -> Self {
        Self { state: seed }
    }

    const fn next(&mut self) -> u64 {
        self.state ^= self.state >> 12;
        self.state ^= self.state << 25;
        self.state ^= self.state >> 27;
        self.state.wrapping_mul(0x2545F4914F6CDD1D)
    }

    const fn next_n<const N: usize>(&mut self) -> [u64; N] {
        let mut rands = [0u64; N];
        let mut i = 0;
        while i < N {
            rands[i] = self.next();
            i += 1;
        }
        rands
    }
}

pub(crate) const RAND_PLACEMENT: [[u64; 64]; 12] = {
    let mut rands: [[u64; 64]; 12] = [[0u64; 64]; 12];
    let mut prng = PRNG::new(0xf25d6975821a1158);

    let mut i = 0;
    while i < 12 {
        rands[i] = prng.next_n::<64>();
        i += 1;
    }

    rands
};

pub(crate) const RAND_EN_PASSANT: [u64; 8] = {
    let mut prng = PRNG::new(0xae31a6122e0f157f);
    prng.next_n::<8>()
};

pub(crate) const RAND_CASTLING: [u64; 16] = {
    let mut prng = PRNG::new(0xd20f9fc3b45ed697);
    prng.next_n::<16>()
};

pub(crate) const RAND_COLOR: [u64; 2] = {
    let mut prng = PRNG::new(0x49ed8ca7d81692aa);
    prng.next_n::<2>()
};
