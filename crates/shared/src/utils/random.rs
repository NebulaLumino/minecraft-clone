//! Random number generation utilities

use serde::{Deserialize, Serialize};

/// Seeded Xorshift64 PRNG
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SeededRng {
    seed: u64,
}

impl SeededRng {
    pub fn new(seed: u64) -> Self {
        Self { seed }
    }

    pub fn from_world_seed(seed: u64) -> Self {
        Self::new(seed)
    }

    /// Next random u64
    pub fn next_u64(&mut self) -> u64 {
        let mut x = self.seed;
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        self.seed = x;
        x
    }

    /// Next random u32
    pub fn next_u32(&mut self) -> u32 {
        (self.next_u64() >> 32) as u32
    }

    /// Next random i32
    pub fn next_i32(&mut self) -> i32 {
        self.next_u32() as i32
    }

    /// Next random f32 in [0, 1)
    pub fn next_f32(&mut self) -> f32 {
        (self.next_u32() >> 8) as f32 / 1_677_721.6
    }

    /// Next random f64 in [0, 1)
    pub fn next_f64(&mut self) -> f64 {
        (self.next_u64() >> 11) as f64 / 9_007_199_254_740_992.0
    }

    /// Next random boolean
    pub fn next_bool(&mut self) -> bool {
        (self.next_u64() & 1) != 0
    }

    /// Random integer in range [0, bound)
    pub fn next_u32_bound(&mut self, bound: u32) -> u32 {
        // Rejection sampling for unbiased random in [0, bound)
        let threshold = u32::MAX - u32::MAX % bound;
        loop {
            let r = self.next_u32();
            if r < threshold {
                return r % bound;
            }
        }
    }
}

/// Simple Perlin noise implementation for world generation
#[derive(Debug, Clone)]
pub struct PerlinNoise {
    permutation: Vec<u8>,
}

impl PerlinNoise {
    pub fn new(seed: u64) -> Self {
        let mut rng = SeededRng::new(seed);
        let mut perm = vec![0u8; 256];
        for i in 0..256 {
            perm[i] = i as u8;
        }
        for i in (1..256).rev() {
            let j = rng.next_u32_bound((i + 1) as u32) as usize;
            perm.swap(i, j);
        }
        let mut permutation = perm.clone();
        permutation.extend(perm);
        Self { permutation }
    }

    fn fade(&self, t: f64) -> f64 {
        t * t * t * (t * (t * 6.0 - 15.0) + 10.0)
    }

    fn lerp(&self, t: f64, a: f64, b: f64) -> f64 {
        a + t * (b - a)
    }

    fn grad(&self, hash: u8, x: f64, y: f64, z: f64) -> f64 {
        let h = hash & 15;
        let u = if h < 8 { x } else { y };
        let v = if h < 4 {
            y
        } else if h == 12 || h == 14 {
            x
        } else {
            z
        };
        ((h & 1) as f64 * -1.0 + ((h & 2) as f64) - 1.0) * if (h & 1) != 0 { u } else { v }
    }

    pub fn noise3d(&self, x: f64, y: f64, z: f64) -> f64 {
        let xi = x.floor() as i32 & 255;
        let yi = y.floor() as i32 & 255;
        let zi = z.floor() as i32 & 255;

        let xf = x - x.floor();
        let yf = y - y.floor();
        let zf = z - z.floor();

        let u = self.fade(xf);
        let v = self.fade(yf);
        let w = self.fade(zf);

        let p = &self.permutation;
        let aaa = p[p[p[xi as usize] as usize + yi as usize] as usize + zi as usize] as usize;
        let aba = p[p[p[xi as usize] as usize + ((yi + 1) & 255) as usize] as usize + zi as usize] as usize;
        let aab = p[p[p[xi as usize] as usize + yi as usize] as usize + ((zi + 1) & 255) as usize] as usize;
        let abb = p[p[p[xi as usize] as usize + ((yi + 1) & 255) as usize] as usize + ((zi + 1) & 255) as usize] as usize;
        let baa = p[p[p[((xi + 1) & 255) as usize] as usize + yi as usize] as usize + zi as usize] as usize;
        let bba = p[p[p[((xi + 1) & 255) as usize] as usize + ((yi + 1) & 255) as usize] as usize + zi as usize] as usize;
        let bab = p[p[p[((xi + 1) & 255) as usize] as usize + yi as usize] as usize + ((zi + 1) & 255) as usize] as usize;
        let bbb = p[p[p[((xi + 1) & 255) as usize] as usize + ((yi + 1) & 255) as usize] as usize + ((zi + 1) & 255) as usize] as usize;

        let x1 = self.lerp(u, self.grad(aaa as u8, xf, yf, zf), self.grad(baa as u8, xf - 1.0, yf, zf));
        let x2 = self.lerp(u, self.grad(aba as u8, xf, yf - 1.0, zf), self.grad(bba as u8, xf - 1.0, yf - 1.0, zf));
        let y1 = self.lerp(v, x1, x2);

        let x3 = self.lerp(u, self.grad(aab as u8, xf, yf, zf - 1.0), self.grad(bab as u8, xf - 1.0, yf, zf - 1.0));
        let x4 = self.lerp(u, self.grad(abb as u8, xf, yf - 1.0, zf - 1.0), self.grad(bbb as u8, xf - 1.0, yf - 1.0, zf - 1.0));
        let y2 = self.lerp(v, x3, x4);

        self.lerp(w, y1, y2)
    }
}
