type Point = (u64, u64);

pub struct Compact {
    ids: Vec<[u32; 5]>,
    counts: Vec<u8>,
    touched: Vec<usize>,
    ready: Vec<usize>,
    decoded: Vec<[Point; 5]>,
}

impl Compact {
    pub fn new(p: usize) -> Self {
        Self {
            ids: vec![[0; 5]; p],
            counts: vec![0; p],
            touched: Vec::new(),
            ready: Vec::new(),
            decoded: Vec::new(),
        }
    }

    pub fn clear(&mut self) {
        for i in self.touched.drain(..) {
            self.counts[i] = 0;
        }
        self.ready.clear();
    }

    pub fn push(&mut self, slope: u64, tag: u32, threshold: usize) {
        let i = slope as usize;
        let n = self.counts[i] as usize;
        if n == 0 {
            self.touched.push(i);
        }
        self.ids[i][n] = tag;
        self.counts[i] += 1;
        if n + 1 == threshold {
            self.ready.push(i);
        }
    }

    pub fn finish(&mut self, fb: &[Point], p: u64) {
        self.ready.sort_unstable();
        self.decoded.resize(self.ready.len(), [(0, 0); 5]);
        for (&slope, dest) in self.ready.iter().zip(&mut self.decoded) {
            for (out, &tag) in dest
                .iter_mut()
                .zip(&self.ids[slope][..self.counts[slope] as usize])
            {
                let (x, y) = fb[(tag >> 1) as usize];
                *out = (x, if tag & 1 == 0 { y } else { p - y });
            }
        }
    }

    pub fn qualifying(&self) -> impl Iterator<Item = (u64, &[Point])> {
        self.ready
            .iter()
            .zip(&self.decoded)
            .map(|(&slope, points)| (slope as u64, &points[..self.counts[slope] as usize]))
    }
}
