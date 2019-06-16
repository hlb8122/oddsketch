use std::ops::{BitXor, BitXorAssign, Deref, DerefMut};

const DEFAULT_LEN: usize = 256;

#[derive(Clone)]
pub struct Oddsketch([u8; DEFAULT_LEN]);

impl Oddsketch {
    #[inline]
    pub fn insert(&mut self, short_id: u64) {
        let os_index = (short_id % (DEFAULT_LEN as u64 * 8)) as usize;
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
        let length = 8. * (DEFAULT_LEN as f64);
        let weight = f64::from(self.hamming_weight());

        let size_approx = f64::ln(1. - 2. * weight / length) / f64::ln(1. - 2. / length);

        size_approx as u32
    }

    pub fn fold(self, size: usize) -> Vec<u8> {
        let chunk_size = DEFAULT_LEN / size;
        self.chunks(chunk_size)
            .map(|chunk| chunk.into_iter().fold(0, |acc, x| acc ^ x))
            .collect()
    }
}

impl BitXor for Oddsketch {
    type Output = Self;

    #[inline]
    fn bitxor(self, rhs: Self) -> Self::Output {
        let mut out = [0; DEFAULT_LEN];
        for (i, item) in out.iter_mut().enumerate() {
            *item ^= rhs.0[i];
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

impl Default for Oddsketch {
    #[inline]
    fn default() -> Self {
        Oddsketch([0; DEFAULT_LEN])
    }
}

impl Deref for Oddsketch {
    type Target = [u8; DEFAULT_LEN];

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