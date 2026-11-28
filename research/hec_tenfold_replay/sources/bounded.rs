type Point = (u64, u64);
#[derive(Clone, Default)]
struct Hits {
    data: [Point; 5],
    len: usize,
}
impl Hits {
    fn push(&mut self, p: Point) {
        self.data[self.len] = p;
        self.len += 1;
    }
}
pub struct Bounded {
    counts: Vec<u8>,
    touched: Vec<usize>,
    ready: Vec<usize>,
    slots: Vec<usize>,
    hits: Vec<Hits>,
    deferred: bool,
}
impl Bounded {
    pub fn new(p: usize, deferred: bool) -> Self {
        Self {
            counts: vec![0; p],
            touched: vec![],
            ready: vec![],
            slots: if deferred { vec![0; p] } else { vec![] },
            hits: if deferred {
                vec![]
            } else {
                vec![Hits::default(); p]
            },
            deferred,
        }
    }
    pub fn clear(&mut self) {
        for i in self.touched.drain(..) {
            self.counts[i] = 0;
            if !self.deferred {
                self.hits[i].len = 0;
            }
        }
        self.ready.clear();
        if self.deferred {
            self.hits.clear();
        }
    }
    pub fn push(&mut self, t: u64, p: Point, threshold: usize) {
        let i = t as usize;
        if self.counts[i] == 0 {
            self.touched.push(i);
        }
        self.counts[i] += 1;
        if self.counts[i] as usize == threshold {
            self.ready.push(i);
        }
        if !self.deferred {
            self.hits[i].push(p);
        }
    }
    pub fn finish(&mut self, cache: &[Option<[u64; 2]>], fb: &[Point], p: u64, threshold: usize) {
        self.ready.sort_unstable();
        if !self.deferred {
            return;
        }
        self.hits.resize(self.ready.len(), Hits::default());
        for (slot, &i) in self.ready.iter().enumerate() {
            self.slots[i] = slot;
        }
        for (slopes, &(x, y)) in cache.iter().zip(fb) {
            if let Some(pair) = slopes {
                for sign in 0..if y == 0 { 1 } else { 2 } {
                    let i = pair[sign] as usize;
                    if self.counts[i] as usize >= threshold {
                        self.hits[self.slots[i]].push((x, if sign == 0 { y } else { p - y }));
                    }
                }
            }
        }
    }
    pub fn qualifying(&self) -> impl Iterator<Item = (u64, &[Point])> {
        self.ready.iter().enumerate().map(|(slot, &i)| {
            let h = &self.hits[if self.deferred { slot } else { i }];
            (i as u64, &h.data[..h.len])
        })
    }
}
