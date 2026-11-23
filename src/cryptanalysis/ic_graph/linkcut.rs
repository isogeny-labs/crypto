//! Report section 1: link-cut trees (Sleator-Tarjan 1983), splay-based.
//!
//! Dynamic forests with `O(log v)` amortized `link`, `cut`, `connected`,
//! `find_root` and constant-size path aggregates. The aggregate here is the
//! `F_p` sum of node values, the path length and the minimum value; listing a
//! path of length `a` costs `Omega(a)` ([`LinkCutForest::path_nodes`]), and
//! vector-valued payload aggregation is not constant-size, as the report notes.

use super::field::Fp;

#[derive(Clone, Copy, Debug)]
struct Node {
    ch: [usize; 2],
    parent: usize,
    rev: bool,
    val: u64,
    sum: u64,
    min: u64,
    len: usize,
}

const NIL: usize = usize::MAX;

/// Forest of `n` nodes, each carrying a value in `F_p`.
#[derive(Clone, Debug)]
pub struct LinkCutForest {
    f: Fp,
    t: Vec<Node>,
    pub rotations: u64,
}

/// Constant-size path aggregate.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PathAggregate {
    pub sum: u64,
    pub min: u64,
    /// Number of vertices on the path.
    pub len: usize,
}

impl LinkCutForest {
    pub fn new(values: &[u64], f: Fp) -> Self {
        let t = values
            .iter()
            .map(|&v| {
                let v = f.reduce(v);
                Node {
                    ch: [NIL, NIL],
                    parent: NIL,
                    rev: false,
                    val: v,
                    sum: v,
                    min: v,
                    len: 1,
                }
            })
            .collect();
        Self { f, t, rotations: 0 }
    }

    fn is_root(&self, x: usize) -> bool {
        let p = self.t[x].parent;
        p == NIL || (self.t[p].ch[0] != x && self.t[p].ch[1] != x)
    }
    fn pull(&mut self, x: usize) {
        let [l, r] = self.t[x].ch;
        let mut sum = self.t[x].val;
        let mut min = self.t[x].val;
        let mut len = 1;
        for c in [l, r] {
            if c != NIL {
                sum = self.f.add(sum, self.t[c].sum);
                min = min.min(self.t[c].min);
                len += self.t[c].len;
            }
        }
        self.t[x].sum = sum;
        self.t[x].min = min;
        self.t[x].len = len;
    }
    fn push(&mut self, x: usize) {
        if self.t[x].rev {
            let [l, r] = self.t[x].ch;
            self.t[x].ch = [r, l];
            for c in [l, r] {
                if c != NIL {
                    self.t[c].rev ^= true;
                }
            }
            self.t[x].rev = false;
        }
    }
    fn rotate(&mut self, x: usize) {
        self.rotations += 1;
        let p = self.t[x].parent;
        let g = self.t[p].parent;
        let dir = usize::from(self.t[p].ch[1] == x);
        let b = self.t[x].ch[1 - dir];
        if !self.is_root(p) {
            let pd = usize::from(self.t[g].ch[1] == p);
            self.t[g].ch[pd] = x;
        }
        self.t[x].parent = g;
        self.t[x].ch[1 - dir] = p;
        self.t[p].parent = x;
        self.t[p].ch[dir] = b;
        if b != NIL {
            self.t[b].parent = p;
        }
        self.pull(p);
        self.pull(x);
    }
    fn splay(&mut self, x: usize) {
        // Push pending reversals from the splay root down to x.
        let mut stack = vec![x];
        let mut y = x;
        while !self.is_root(y) {
            y = self.t[y].parent;
            stack.push(y);
        }
        while let Some(z) = stack.pop() {
            self.push(z);
        }
        while !self.is_root(x) {
            let p = self.t[x].parent;
            if !self.is_root(p) {
                let g = self.t[p].parent;
                let zigzig = (self.t[g].ch[0] == p) == (self.t[p].ch[0] == x);
                if zigzig {
                    self.rotate(p);
                } else {
                    self.rotate(x);
                }
            }
            self.rotate(x);
        }
    }
    fn access(&mut self, x: usize) {
        let mut last = NIL;
        let mut y = x;
        while y != NIL {
            self.splay(y);
            self.t[y].ch[1] = last;
            self.pull(y);
            last = y;
            y = self.t[y].parent;
        }
        self.splay(x);
    }
    fn make_root(&mut self, x: usize) {
        self.access(x);
        self.t[x].rev ^= true;
        self.push(x);
    }

    pub fn find_root(&mut self, x: usize) -> usize {
        self.access(x);
        let mut y = x;
        loop {
            self.push(y);
            let l = self.t[y].ch[0];
            if l == NIL {
                break;
            }
            y = l;
        }
        self.splay(y);
        y
    }
    pub fn connected(&mut self, u: usize, v: usize) -> bool {
        u == v || self.find_root(u) == self.find_root(v)
    }
    /// Link `u` and `v` (in different trees); returns `false` otherwise.
    pub fn link(&mut self, u: usize, v: usize) -> bool {
        if self.connected(u, v) {
            return false;
        }
        self.make_root(u);
        self.t[u].parent = v;
        true
    }
    /// Cut the edge `u`-`v`; returns `false` if it is not a tree edge.
    pub fn cut(&mut self, u: usize, v: usize) -> bool {
        self.make_root(u);
        self.access(v);
        // After access(v) with root u, the edge exists iff v's left child is u
        // and u has no right child.
        if self.t[v].ch[0] != u || self.t[u].ch[1] != NIL {
            return false;
        }
        self.t[v].ch[0] = NIL;
        self.t[u].parent = NIL;
        self.pull(v);
        true
    }
    pub fn set_value(&mut self, x: usize, val: u64) {
        self.access(x);
        self.t[x].val = self.f.reduce(val);
        self.pull(x);
    }
    /// Aggregate over the tree path `u .. v`, `None` if disconnected.
    pub fn path_aggregate(&mut self, u: usize, v: usize) -> Option<PathAggregate> {
        if !self.connected(u, v) {
            return None;
        }
        self.make_root(u);
        self.access(v);
        let n = self.t[v];
        Some(PathAggregate {
            sum: n.sum,
            min: n.min,
            len: n.len,
        })
    }
    /// Explicit vertex list of the path `u .. v` (costs `Omega(len)`).
    pub fn path_nodes(&mut self, u: usize, v: usize) -> Option<Vec<usize>> {
        if !self.connected(u, v) {
            return None;
        }
        self.make_root(u);
        self.access(v);
        let mut out = Vec::new();
        // In-order traversal of v's splay tree with lazy reversal.
        let mut stack: Vec<(usize, bool)> = vec![(v, false)];
        while let Some((x, visited)) = stack.pop() {
            if x == NIL {
                continue;
            }
            self.push(x);
            if visited {
                out.push(x);
            } else {
                let [l, r] = self.t[x].ch;
                stack.push((r, false));
                stack.push((x, true));
                stack.push((l, false));
            }
        }
        Some(out)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cryptanalysis::ic_graph::field::SplitMix64;
    use crate::cryptanalysis::ic_graph::graph::Graph;

    #[test]
    fn random_link_cut_against_bfs() {
        let f = Fp::new(1_000_003).unwrap();
        let n = 40;
        let mut rng = SplitMix64::new(9);
        let vals: Vec<u64> = (0..n).map(|_| rng.below(1000)).collect();
        let mut lct = LinkCutForest::new(&vals, f);
        let mut edges: Vec<(usize, usize)> = Vec::new();
        for _ in 0..3000 {
            let u = rng.below(n as u64) as usize;
            let v = rng.below(n as u64) as usize;
            let op = rng.below(3);
            if op == 0 && !edges.is_empty() {
                let i = rng.below(edges.len() as u64) as usize;
                let (a, b) = edges.swap_remove(i);
                assert!(lct.cut(a, b));
            } else if u != v {
                let g = Graph::from_edges(n, &edges);
                let connected = g.bfs(u).0[v] != usize::MAX;
                assert_eq!(lct.connected(u, v), connected);
                if !connected {
                    assert!(lct.link(u, v));
                    edges.push((u, v));
                } else {
                    let f2 = g.spanning_forest();
                    let path = f2.tree_path(&g, u, v).unwrap();
                    let agg = lct.path_aggregate(u, v).unwrap();
                    assert_eq!(agg.len, path.len() + 1);
                    let nodes = lct.path_nodes(u, v).unwrap();
                    assert_eq!(nodes.len(), agg.len);
                    assert_eq!(nodes[0], u);
                    assert_eq!(*nodes.last().unwrap(), v);
                    let s = nodes.iter().fold(0u64, |a, &x| f.add(a, vals[x]));
                    assert_eq!(agg.sum, s);
                    assert_eq!(agg.min, nodes.iter().map(|&x| vals[x]).min().unwrap());
                    assert!(!lct.cut(u, u));
                }
            }
        }
    }
}
