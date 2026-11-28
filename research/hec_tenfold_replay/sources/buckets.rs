type Point = (u64, u64);
pub struct Buckets {
    lists: Vec<Vec<Point>>,
    touched: Vec<usize>,
    ready: Vec<u64>,
}
impl Buckets {
    pub fn new(p: usize) -> Self {
        Self {
            lists: vec![Vec::new(); p],
            touched: vec![],
            ready: vec![0; (p + 63) / 64],
        }
    }
    pub fn clear(&mut self) {
        for j in self.touched.drain(..) {
            self.lists[j].clear();
            self.ready[j / 64] = 0;
        }
    }
    pub fn push(&mut self, t: u64, p: Point, threshold: usize) {
        let j = t as usize;
        let list = &mut self.lists[j];
        if list.is_empty() {
            self.touched.push(j);
        }
        list.push(p);
        if list.len() == threshold {
            self.ready[j / 64] |= 1 << (j % 64);
        }
    }
    pub fn qualifying(&self) -> impl Iterator<Item = (u64, &Vec<Point>)> {
        self.ready
            .iter()
            .copied()
            .enumerate()
            .flat_map(move |(w, mut bits)| {
                std::iter::from_fn(move || {
                    if bits == 0 {
                        return None;
                    }
                    let j = w * 64 + bits.trailing_zeros() as usize;
                    bits &= bits - 1;
                    Some((j as u64, &self.lists[j]))
                })
            })
    }
}
