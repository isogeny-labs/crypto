//! Parity union-find yield (SPEC section 8).

pub struct Graph {
    parent: Vec<usize>,
    parity: Vec<u8>,
    sink: Vec<bool>,
    odd: Vec<bool>,
    pub edges: i64,
    pub verts: i64,
    pub bip: i64,
    pub ops: u64,
    pub f0: i64,
}

impl Graph {
    pub fn new() -> Graph {
        Graph {
            parent: vec![0],
            parity: vec![0],
            sink: vec![true],
            odd: vec![false],
            edges: 0,
            verts: 0,
            bip: 0,
            ops: 0,
            f0: 0,
        }
    }
    pub fn new_vertex(&mut self) -> usize {
        let v = self.parent.len();
        self.parent.push(v);
        self.parity.push(0);
        self.sink.push(false);
        self.odd.push(false);
        self.verts += 1;
        self.bip += 1;
        v
    }
    fn find(&mut self, v: usize) -> (usize, u8) {
        let mut path = vec![];
        let mut cur = v;
        let mut par = 0u8;
        while self.parent[cur] != cur {
            path.push(cur);
            par ^= self.parity[cur];
            cur = self.parent[cur];
            self.ops += 1;
        }
        let root = cur;
        let mut pp = par;
        for &n in &path {
            let own = self.parity[n];
            self.parent[n] = root;
            self.parity[n] = pp;
            pp ^= own;
        }
        (root, par)
    }
    fn is_b(&self, r: usize) -> i64 {
        if !self.sink[r] && !self.odd[r] { 1 } else { 0 }
    }
    pub fn add_edge(&mut self, a: usize, b: usize) {
        self.edges += 1;
        let (ra, pa) = self.find(a);
        let (rb, pb) = self.find(b);
        if ra == rb {
            if pa == pb && !self.odd[ra] {
                self.bip -= self.is_b(ra);
                self.odd[ra] = true;
            }
        } else {
            self.bip -= self.is_b(ra) + self.is_b(rb);
            self.parent[rb] = ra;
            self.parity[rb] = pa ^ pb ^ 1;
            self.sink[ra] = self.sink[ra] || self.sink[rb];
            self.odd[ra] = self.odd[ra] || self.odd[rb];
            self.bip += self.is_b(ra);
        }
    }
    pub fn y(&self) -> i64 {
        self.f0 + self.edges - self.verts + self.bip
    }
    pub fn rank_lp_uf(&self) -> i64 {
        self.verts - self.bip
    }
}
