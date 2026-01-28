use core::ops::{Range, RangeInclusive};

/// Random number generator based off Splitmix64
#[derive(Copy, Clone, Debug)]
pub struct Rng {
    state: u64,
}

impl Rng {
    #[inline]
    #[must_use]
    pub const fn new(seed: u64) -> Self {
        Self { state: seed }
    }

    #[inline]
    pub const fn next_u64(&mut self) -> u64 {
        self.state = self.state.wrapping_add(0x9E37_79B9_7F4A_7C15_u64);
        let mut z = self.state;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9_u64);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB_u64);
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
        self.gen_range_u64(u64::from(range.start)..u64::from(range.end)) as u32
    }

    #[inline]
    pub fn gen_range_u32_inclusive(&mut self, range: RangeInclusive<u32>) -> u32 {
        self.gen_range_u64_inclusive(u64::from(*range.start())..=u64::from(*range.end())) as u32
    }

    #[inline]
    pub fn gen_range_i64(&mut self, range: Range<i64>) -> i64 {
        let start = i128::from(range.start);
        let end = i128::from(range.end);
        debug_assert!(start < end);
        let size = (end - start) as u64;
        (start + i128::from(self.next_u64_bounded(size))) as i64
    }

    #[inline]
    pub fn gen_range_i64_inclusive(&mut self, range: RangeInclusive<i64>) -> i64 {
        let start = i128::from(*range.start());
        let end = i128::from(*range.end());
        debug_assert!(start <= end);
        let size = (end - start) as u64 + 1;
        (start + i128::from(self.next_u64_bounded(size))) as i64
    }

    #[inline]
    pub fn gen_range_i32(&mut self, range: Range<i32>) -> i32 {
        self.gen_range_i64(i64::from(range.start)..i64::from(range.end)) as i32
    }

    #[inline]
    pub fn gen_range_i32_inclusive(&mut self, range: RangeInclusive<i32>) -> i32 {
        self.gen_range_i64_inclusive(i64::from(*range.start())..=i64::from(*range.end())) as i32
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
        self.next_f32().mul_add(end - start, start)
    }

    #[inline]
    pub fn gen_range_f32_inclusive(&mut self, range: RangeInclusive<f32>) -> f32 {
        let start = *range.start();
        let end = *range.end();
        debug_assert!(start <= end);
        self.next_f32().mul_add(end - start, start)
    }

    #[inline]
    pub fn gen_range_f64(&mut self, range: Range<f64>) -> f64 {
        let start = range.start;
        let end = range.end;
        debug_assert!(start < end);
        self.next_f64().mul_add(end - start, start)
    }

    #[inline]
    pub fn gen_range_f64_inclusive(&mut self, range: RangeInclusive<f64>) -> f64 {
        let start = *range.start();
        let end = *range.end();
        debug_assert!(start <= end);
        self.next_f64().mul_add(end - start, start)
    }
}
