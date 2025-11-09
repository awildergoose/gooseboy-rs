use core::ops::{Range, RangeInclusive};

/// Random number generator based off Splitmix64
#[derive(Copy, Clone, Debug)]
pub struct Rng {
    state: u64,
}

impl Rng {
    #[inline]
    pub const fn new(seed: u64) -> Self {
        Self { state: seed }
    }

    #[inline]
    pub fn next_u64(&mut self) -> u64 {
        self.state = self.state.wrapping_add(0x9E3779B97F4A7C15_u64);
        let mut z = self.state;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58476D1CE4E5B9_u64);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D049BB133111EB_u64);
        z ^ (z >> 31)
    }

    #[inline]
    fn next_u64_bounded(&mut self, bound: u64) -> u64 {
        debug_assert!(bound > 0);
        let threshold = u64::MAX - (u64::MAX % bound);

        loop {
            let r = self.next_u64();
            if r < threshold {
                return r % bound;
            }
        }
    }

    #[inline]
    pub fn gen_range_u64(&mut self, range: Range<u64>) -> u64 {
        let start = range.start;
        let end = range.end;
        debug_assert!(start < end);
        let size = end - start;
        start + self.next_u64_bounded(size)
    }

    #[inline]
    pub fn gen_range_u64_inclusive(&mut self, range: RangeInclusive<u64>) -> u64 {
        let (start, end) = (*range.start(), *range.end());
        debug_assert!(start <= end);
        let size = end.wrapping_sub(start).wrapping_add(1);
        start + self.next_u64_bounded(size)
    }

    #[inline]
    pub fn gen_range_u32(&mut self, range: Range<u32>) -> u32 {
        self.gen_range_u64(range.start as u64..range.end as u64) as u32
    }

    #[inline]
    pub fn gen_range_u32_inclusive(&mut self, range: RangeInclusive<u32>) -> u32 {
        self.gen_range_u64_inclusive((*range.start() as u64)..=(*range.end() as u64)) as u32
    }

    #[inline]
    pub fn gen_range_i64(&mut self, range: Range<i64>) -> i64 {
        let start = range.start as i128;
        let end = range.end as i128;
        debug_assert!(start < end);
        let size = (end - start) as u64;
        (start + (self.next_u64_bounded(size) as i128)) as i64
    }

    #[inline]
    pub fn gen_range_i64_inclusive(&mut self, range: RangeInclusive<i64>) -> i64 {
        let start = *range.start() as i128;
        let end = *range.end() as i128;
        debug_assert!(start <= end);
        let size = (end - start) as u64 + 1;
        (start + (self.next_u64_bounded(size) as i128)) as i64
    }

    #[inline]
    pub fn gen_range_i32(&mut self, range: Range<i32>) -> i32 {
        self.gen_range_i64(range.start as i64..range.end as i64) as i32
    }

    #[inline]
    pub fn gen_range_i32_inclusive(&mut self, range: RangeInclusive<i32>) -> i32 {
        self.gen_range_i64_inclusive((*range.start() as i64)..=(*range.end() as i64)) as i32
    }

    #[inline]
    pub fn next_f32(&mut self) -> f32 {
        ((self.next_u64() >> 40) as u32) as f32 / (1u32 << 24) as f32
    }

    #[inline]
    pub fn next_f64(&mut self) -> f64 {
        (self.next_u64() >> 11) as f64 / ((1u64 << 53) as f64)
    }

    #[inline]
    pub fn gen_range_f32(&mut self, range: Range<f32>) -> f32 {
        let start = range.start;
        let end = range.end;
        debug_assert!(start < end);
        start + (self.next_f32() * (end - start))
    }

    #[inline]
    pub fn gen_range_f32_inclusive(&mut self, range: RangeInclusive<f32>) -> f32 {
        let start = *range.start();
        let end = *range.end();
        debug_assert!(start <= end);
        start + (self.next_f32() * (end - start))
    }

    #[inline]
    pub fn gen_range_f64(&mut self, range: Range<f64>) -> f64 {
        let start = range.start;
        let end = range.end;
        debug_assert!(start < end);
        start + (self.next_f64() * (end - start))
    }

    #[inline]
    pub fn gen_range_f64_inclusive(&mut self, range: RangeInclusive<f64>) -> f64 {
        let start = *range.start();
        let end = *range.end();
        debug_assert!(start <= end);
        start + (self.next_f64() * (end - start))
    }
}
