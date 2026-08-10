//! Fixed-capacity ring buffer for incremental TA windows (private helper).

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

    pub(crate) fn clear(&mut self) {
        self.head = 0;
        self.len = 0;
        self.sum = 0.0;
    }

    /// Push value; if full, overwrites oldest and adjusts sum.
    pub(crate) fn push(&mut self, value: f64) {
        let cap = self.buf.len();
        if self.len < cap {
            self.buf[self.head] = value;
            self.sum += value;
            self.head = (self.head + 1) % cap;
            self.len += 1;
        } else {
            let old = self.buf[self.head];
            self.sum += value - old;
            self.buf[self.head] = value;
            self.head = (self.head + 1) % cap;
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
        self.pv.push(price * volume);
        self.vol.push(volume);
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
