//! Fixed-capacity ring buffer for incremental TA windows (private helper).
//!
//! Also: monotonic sliding max/min ([`SlidingMax`] / [`SlidingMin`]) for O(1)
//! amortized window extrema (Stoch HH/LL, Donchian).

use std::collections::VecDeque;

/// Ring of `f64` with optional running sum (SMA / RVOL / rolling VWAP).
#[derive(Clone, Debug)]
pub(crate) struct RingF64 {
    buf: Vec<f64>,
    /// Next write index.
    head: usize,
    /// Number of valid elements ≤ capacity.
    len: usize,
    sum: f64,
}

impl RingF64 {
    pub(crate) fn with_capacity(cap: usize) -> Self {
        debug_assert!(cap >= 1);
        Self {
            buf: vec![0.0; cap],
            head: 0,
            len: 0,
            sum: 0.0,
        }
    }

    pub(crate) fn capacity(&self) -> usize {
        self.buf.len()
    }

    pub(crate) fn len(&self) -> usize {
        self.len
    }

    pub(crate) fn is_full(&self) -> bool {
        self.len == self.buf.len()
    }

    pub(crate) fn sum(&self) -> f64 {
        self.sum
    }

    /// Oldest sample still in the ring (`None` if empty).
    ///
    /// When full, this is the value that the next [`push`](Self::push) will evict.
    pub(crate) fn oldest(&self) -> Option<f64> {
        if self.len == 0 {
            return None;
        }
        let cap = self.buf.len();
        let start = if self.len < cap { 0 } else { self.head };
        Some(self.buf[start])
    }

    pub(crate) fn clear(&mut self) {
        self.head = 0;
        self.len = 0;
        self.sum = 0.0;
    }

    /// Push value; if full, overwrites oldest and adjusts sum.
    ///
    /// Returns the **evicted** oldest value when the ring was already full.
    pub(crate) fn push(&mut self, value: f64) -> Option<f64> {
        let cap = self.buf.len();
        if self.len < cap {
            self.buf[self.head] = value;
            self.sum += value;
            self.head = (self.head + 1) % cap;
            self.len += 1;
            None
        } else {
            let old = self.buf[self.head];
            self.sum += value - old;
            self.buf[self.head] = value;
            self.head = (self.head + 1) % cap;
            Some(old)
        }
    }

    /// Logical order oldest → newest into `out` (cleared first).
    pub(crate) fn copy_ordered(&self, out: &mut Vec<f64>) {
        out.clear();
        if self.len == 0 {
            return;
        }
        let cap = self.buf.len();
        let start = if self.len < cap { 0 } else { self.head };
        for i in 0..self.len {
            out.push(self.buf[(start + i) % cap]);
        }
    }

    pub(crate) fn mean(&self) -> Option<f64> {
        if self.len == 0 {
            None
        } else {
            Some(self.sum / self.len as f64)
        }
    }

    pub(crate) fn max(&self) -> Option<f64> {
        if self.len == 0 {
            return None;
        }
        let cap = self.buf.len();
        let start = if self.len < cap { 0 } else { self.head };
        let mut m = f64::NEG_INFINITY;
        for i in 0..self.len {
            m = m.max(self.buf[(start + i) % cap]);
        }
        Some(m)
    }

    pub(crate) fn min(&self) -> Option<f64> {
        if self.len == 0 {
            return None;
        }
        let cap = self.buf.len();
        let start = if self.len < cap { 0 } else { self.head };
        let mut m = f64::INFINITY;
        for i in 0..self.len {
            m = m.min(self.buf[(start + i) % cap]);
        }
        Some(m)
    }
}

/// Ring of (price*volume, volume) pairs for rolling VWAP.
#[derive(Clone, Debug)]
pub(crate) struct RingPv {
    pv: RingF64,
    vol: RingF64,
}

impl RingPv {
    pub(crate) fn with_capacity(cap: usize) -> Self {
        Self {
            pv: RingF64::with_capacity(cap),
            vol: RingF64::with_capacity(cap),
        }
    }

    pub(crate) fn clear(&mut self) {
        self.pv.clear();
        self.vol.clear();
    }

    pub(crate) fn is_full(&self) -> bool {
        self.pv.is_full()
    }

    pub(crate) fn len(&self) -> usize {
        self.pv.len()
    }

    pub(crate) fn push(&mut self, price: f64, volume: f64) {
        let _ = self.pv.push(price * volume);
        let _ = self.vol.push(volume);
    }

    pub(crate) fn vwap(&self) -> Option<f64> {
        let v = self.vol.sum();
        if v > 0.0 {
            Some(self.pv.sum() / v)
        } else {
            None
        }
    }
}

// ---------------------------------------------------------------------------
// Sliding window max / min (monotonic deques) — amortized O(1) per push
// ---------------------------------------------------------------------------

/// Sliding-window maximum over the last `window` samples.
///
/// Classic mono-decreasing deque of (sequence id, value). Each `push` is
/// amortized O(1); `max()` is O(1).
#[derive(Clone, Debug)]
pub(crate) struct SlidingMax {
    /// Decreasing values (front = max). Equal values keep the newest index.
    dq: VecDeque<(u64, f64)>,
    next_id: u64,
    window: usize,
    /// Samples currently in the logical window (≤ `window`).
    count: usize,
}

impl SlidingMax {
    pub(crate) fn with_window(window: usize) -> Self {
        debug_assert!(window >= 1);
        Self {
            dq: VecDeque::with_capacity(window),
            next_id: 0,
            window,
            count: 0,
        }
    }

    pub(crate) fn clear(&mut self) {
        self.dq.clear();
        self.next_id = 0;
        self.count = 0;
    }

    pub(crate) fn is_full(&self) -> bool {
        self.count == self.window
    }

    pub(crate) fn max(&self) -> Option<f64> {
        self.dq.front().map(|(_, v)| *v)
    }

    /// Push a sample; when the window was full, the oldest sample expires first.
    ///
    /// Returns the current window max (always `Some` after the first push).
    pub(crate) fn push(&mut self, value: f64) -> Option<f64> {
        if self.count == self.window {
            let drop_id = self.next_id - self.window as u64;
            if let Some(&(id, _)) = self.dq.front() {
                if id == drop_id {
                    self.dq.pop_front();
                }
            }
        } else {
            self.count += 1;
        }
        while let Some(&(_, back_v)) = self.dq.back() {
            if back_v <= value {
                self.dq.pop_back();
            } else {
                break;
            }
        }
        self.dq.push_back((self.next_id, value));
        self.next_id += 1;
        self.max()
    }
}

/// Sliding-window minimum over the last `window` samples (mono-increasing deque).
#[derive(Clone, Debug)]
pub(crate) struct SlidingMin {
    dq: VecDeque<(u64, f64)>,
    next_id: u64,
    window: usize,
    count: usize,
}

impl SlidingMin {
    pub(crate) fn with_window(window: usize) -> Self {
        debug_assert!(window >= 1);
        Self {
            dq: VecDeque::with_capacity(window),
            next_id: 0,
            window,
            count: 0,
        }
    }

    pub(crate) fn clear(&mut self) {
        self.dq.clear();
        self.next_id = 0;
        self.count = 0;
    }

    pub(crate) fn is_full(&self) -> bool {
        self.count == self.window
    }

    pub(crate) fn min(&self) -> Option<f64> {
        self.dq.front().map(|(_, v)| *v)
    }

    pub(crate) fn push(&mut self, value: f64) -> Option<f64> {
        if self.count == self.window {
            let drop_id = self.next_id - self.window as u64;
            if let Some(&(id, _)) = self.dq.front() {
                if id == drop_id {
                    self.dq.pop_front();
                }
            }
        } else {
            self.count += 1;
        }
        while let Some(&(_, back_v)) = self.dq.back() {
            if back_v >= value {
                self.dq.pop_back();
            } else {
                break;
            }
        }
        self.dq.push_back((self.next_id, value));
        self.next_id += 1;
        self.min()
    }
}

#[cfg(test)]
mod sliding_tests {
    use super::*;

    #[test]
    fn sliding_max_matches_scan() {
        let data = [1.0, 3.0, 2.0, 5.0, 4.0, 0.0, 6.0, 1.0];
        let w = 3usize;
        let mut sm = SlidingMax::with_window(w);
        for (i, &v) in data.iter().enumerate() {
            let got = sm.push(v).unwrap();
            let start = i.saturating_sub(w - 1);
            let exp = data[start..=i]
                .iter()
                .cloned()
                .fold(f64::NEG_INFINITY, f64::max);
            assert!((got - exp).abs() < 1e-15, "i={i} got={got} exp={exp}");
            assert_eq!(sm.is_full(), i + 1 >= w);
        }
    }

    #[test]
    fn sliding_min_matches_scan() {
        let data = [4.0, 2.0, 3.0, 1.0, 5.0, 0.5, 2.0];
        let w = 4usize;
        let mut sm = SlidingMin::with_window(w);
        for (i, &v) in data.iter().enumerate() {
            let got = sm.push(v).unwrap();
            let start = i.saturating_sub(w - 1);
            let exp = data[start..=i]
                .iter()
                .cloned()
                .fold(f64::INFINITY, f64::min);
            assert!((got - exp).abs() < 1e-15, "i={i} got={got} exp={exp}");
        }
    }
}
