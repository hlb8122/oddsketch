use core::ops::{BitXor, BitXorAssign, Deref, DerefMut};

#[derive(Clone)]
pub struct Oddsketch(Vec<u8>);

impl Oddsketch {
    #[inline]
    pub fn new(raw: Vec<u8>) -> Oddsketch {
        Oddsketch(raw)
    }

    #[inline]
    pub fn insert(&mut self, short_id: u64) {
        let os_index = (short_id % (self.0.len() as u64 * 8)) as usize;
        self.0[os_index / 8] ^= 1 << (os_index % 8);
    }

    #[inline]
    pub fn insert_batch(&mut self, short_ids: &[u64]) {
        for id in short_ids {
            self.insert(*id);
        }
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.iter().all(|x| *x == 0)
    }

    #[inline]
    pub fn hamming_weight(&self) -> u32 {
        self.iter().map(|b| b.count_ones()).sum()
    }

    #[inline]
    pub fn size(&self) -> u32 {
        let length = 8. * (self.0.len() as f64);
        let weight = f64::from(self.hamming_weight());

        let size_approx = -length / 2. * f64::ln(1. - 2. * weight / length);

        size_approx as u32
    }

    #[inline]
    pub fn size_alt(&self) -> u32 {
        let length = 8. * (self.0.len() as f64);
        let weight = f64::from(self.hamming_weight());

        let size_approx = f64::ln(1. - 2. * weight / length) / f64::ln(1. - 2. / length);

        size_approx as u32
    }

    pub fn fold(self, size: usize) -> Vec<u8> {
        let chunk_size = self.0.len() / size;
        self.chunks(chunk_size)
            .map(|chunk| chunk.iter().fold(0, |acc, x| acc ^ x))
            .collect()
    }
}

impl BitXor for Oddsketch {
    type Output = Self;

    #[inline]
    fn bitxor(self, rhs: Self) -> Self::Output {
        let mut out = vec![0; self.0.len()];
        for (i, item) in out.iter_mut().enumerate() {
            *item = self.0[i] ^ rhs.0[i];
        }

        Oddsketch(out)
    }
}

impl BitXorAssign for Oddsketch {
    #[inline]
    fn bitxor_assign(&mut self, rhs: Self) {
        for (i, item) in self.iter_mut().enumerate() {
            *item ^= rhs.0[i];
        }
    }
}

pub const DEFAULT_LEN: usize = 256;

impl Default for Oddsketch {
    #[inline]
    fn default() -> Self {
        Oddsketch(vec![0; DEFAULT_LEN])
    }
}

impl Deref for Oddsketch {
    type Target = Vec<u8>;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Oddsketch {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
