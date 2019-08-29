#![feature(const_generics)]

use std::ops::{BitXor, BitXorAssign, Deref, DerefMut};

#[derive(Clone)]
pub struct Oddsketch<const LEN: usize>([u8; LEN]);

impl<const LEN: usize> Oddsketch<{LEN}> {
    #[inline]
    pub fn new(raw: [u8; LEN]) -> Self {
        Oddsketch(raw)
    }

    #[inline]
    pub fn insert(&mut self, short_id: u64) {
        let os_index = (short_id % (LEN as u64 * 8)) as usize;
        self.0[os_index / 8] ^= 1 << (os_index % 8);
    }

    #[inline]
    pub fn insert_batch(&mut self, short_ids: &[u64]) {
        for id in short_ids {
            self.insert(*id);
        }
    }

    #[inline]
    pub fn to_vec(self) -> Vec<u8> {
        self.0
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
        let LEN = 8. * (LEN as f64);
        let weight = f64::from(self.hamming_weight());

        let size_approx = -LEN / 2.  * f64::ln(1. - 2. * weight / LEN);

        size_approx as u32
    }

    #[inline]
    pub fn size_alt(&self) -> u32 {
        let LEN = 8. * (LEN as f64);
        let weight = f64::from(self.hamming_weight());

        let size_approx = f64::ln(1. - 2. * weight / LEN) / f64::ln(1. - 2. / LEN);

        size_approx as u32
    }

    pub fn fold(self, size: usize) -> Vec<u8> {
        let chunk_size = LEN / size;
        self.chunks(chunk_size)
            .map(|chunk| chunk.iter().fold(0, |acc, x| acc ^ x))
            .collect()
    }
}

impl<const LEN: usize> BitXor for Oddsketch<{LEN}> {
    type Output = Self;

    #[inline]
    fn bitxor(self, rhs: Self) -> Self::Output {
        let mut out = [0; std::mem::size_of::<Self>()];
        for (i, item) in out.iter_mut().enumerate() {
            *item = self.0[i] ^ rhs.0[i];
        }

        Oddsketch(out)
    }
}

impl<const LEN: usize> BitXorAssign for Oddsketch<{LEN}> {
    #[inline]
    fn bitxor_assign(&mut self, rhs: Self) {
        for (i, item) in self.iter_mut().enumerate() {
            *item ^= rhs.0[i];
        }
    }
}

impl<const LEN: usize> Default for Oddsketch<{LEN}> {
    #[inline]
    fn default() -> Self {
        Oddsketch([0; LEN])
    }
}

impl<const LEN: usize> Deref for Oddsketch<{LEN}> {
    type Target = [u8; LEN];

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<const LEN: usize> DerefMut for Oddsketch<{LEN}> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
