//! Point encoding, hashed factor bases and the whole-point relation network
//! (Go: relation.go).

use std::collections::HashMap;
use std::rc::Rc;

use rand::rngs::StdRng;
use rand::RngCore;
use sha2::{Digest, Sha256};

use crate::curve::{Curve, Point, INFINITY};
use crate::field::{add_mod, mul_mod, rand_below};
use crate::tensor::{bits_key, FuncVertex, Network, PredVertex, SupportVertex, Vertex};

impl Curve {
    /// PointWidth is the number of bits used to encode a point: an infinity
    /// flag followed by x and y, each FieldBits wide, least significant bit
    /// first. (`impl Curve` block lives here because Go defines it here.)
    pub fn point_width(&self) -> usize {
        2 * self.field_bits() + 1
    }

    /// EncodePoint gives the canonical bit encoding of pt (O is the flag alone).
    pub fn encode_point(&self, pt: Point) -> Vec<bool> {
        let b = self.field_bits();
        let mut v = vec![false; self.point_width()];
        if pt.inf {
            v[0] = true;
            return v;
        }
        for i in 0..b {
            v[1 + i] = (pt.x >> i) & 1 == 1;
            v[1 + b + i] = (pt.y >> i) & 1 == 1;
        }
        v
    }

    /// DecodePoint inverts EncodePoint. `None` (Go: ok == false) for
    /// non-canonical encodings and for points not on the curve: this is the
    /// point-validity predicate.
    pub fn decode_point(&self, v: &[bool]) -> Option<Point> {
        let b = self.field_bits();
        if v[0] {
            if v[1..].iter().any(|&x| x) {
                return None;
            }
            return Some(INFINITY);
        }
        let mut pt = Point::default();
        for i in 0..b {
            if v[1 + i] {
                pt.x |= 1u64 << i;
            }
            if v[1 + b + i] {
                pt.y |= 1u64 << i;
            }
        }
        if self.on_curve(pt) {
            Some(pt)
        } else {
            None
        }
    }
}

/// FactorBases holds m randomly hashed factor bases F_1..F_m. Membership in
/// F_i is decided by a keyed hash of the point, not by any notion of
/// smoothness. Columns for linear algebra are indexed by point, since one
/// point can lie in several F_i.
#[derive(Debug, Clone, Default)]
pub struct FactorBases {
    pub sets: Vec<Vec<Point>>,
    pub points: Vec<Point>,
    pub index: HashMap<Point, usize>,
    pub keys: Vec<[u8; 16]>,
    pub mods: Vec<u64>,

    /// Affine is set for factor bases from `new_affine_factor_bases`: A is in
    /// F_i iff (Alpha_i x + Beta_i y + Gamma_i) mod p < Bound_i.
    pub affine: bool,
    pub alpha: Vec<u64>,
    pub beta: Vec<u64>,
    pub gamma: Vec<u64>,
    pub bound: Vec<u64>,
    pub p: u64,

    /// x_bound, if non-empty, adds a compact coordinate constraint: A is in
    /// F_i only if x(A) < x_bound[i], on top of the hash test. window_bits is
    /// the k with x_bound = p >> k. The window is deterministic, so unlike the
    /// hash it is not random: it trades some independence between the F_i for
    /// membership tests that fix the high bits of x.
    pub x_bound: Vec<u64>,
    pub window_bits: usize,
}

/// NewFactorBases draws m hash keys and sizes each F_i near (2r)^(1/m), so the
/// expected number of decompositions of a random target is about 2.
/// Keys: `rng.fill_bytes` on a 16-byte array (Go: rng.Read).
pub fn new_factor_bases(c: &Curve, m: usize, rng: &mut StdRng) -> FactorBases {
    let target = (2.0 * c.order as f64).powf(1.0 / m as f64).ceil();
    let modulus = (c.order as f64 / target).round().max(1.0) as u64;
    // Unlike Go, redraw the keys while any F_i is empty, as
    // new_affine_factor_bases does: an empty F_i admits no relation, and the
    // solver's collection loop would never finish.
    loop {
        let mut fb = FactorBases {
            sets: vec![Vec::new(); m],
            ..Default::default()
        };
        for _ in 0..m {
            let mut key = [0u8; 16];
            rng.fill_bytes(&mut key);
            fb.keys.push(key);
            fb.mods.push(modulus);
        }
        fb.list(c);
        if fb.sets.iter().all(|set| !set.is_empty()) {
            return fb;
        }
    }
}

/// NewAffineFactorBases draws F_i = {A : (alpha x + beta y + gamma) mod p <
/// U} with random alpha, beta, gamma and U chosen so |F_i| is near (2r)^(1/m).
/// The hash family is pairwise independent on distinct (x, y) (up to the
/// excluded alpha = beta = 0), and unlike SHA-256 its membership test is cheap
/// to encode with the low-degree gadgets (Builder::membership_check).
pub fn new_affine_factor_bases(c: &Curve, m: usize, rng: &mut StdRng) -> FactorBases {
    new_windowed_factor_bases(c, m, rng, 0)
}

/// NewWindowedFactorBases is NewAffineFactorBases with the extra constraint
/// x < p >> k, so F_i = {A : x(A) < p>>k and hash_i(A) < U_i}. U_i is scaled
/// by 2^k to keep |F_i| near (2r)^(1/m). If the window alone would be smaller
/// than that, k is lowered until it fits. k = 0 means no window (Go:
/// NewWindowedFactorBases).
pub fn new_windowed_factor_bases(c: &Curve, m: usize, rng: &mut StdRng, k: usize) -> FactorBases {
    let target = (2.0 * c.order as f64).powf(1.0 / m as f64).ceil();
    // The window holds about (r-1) * 2^-k points; it must hold at least target.
    let mut k = k;
    while k > 0 && (c.order - 1) as f64 / 2f64.powi(k as i32) < target {
        k -= 1;
    }
    let xb = (c.p >> k).max(1);
    let frac = xb as f64 / c.p as f64;
    let mut bound = (c.p as f64 * target / ((c.order - 1) as f64 * frac))
        .round()
        .max(1.0) as u64;
    if bound > c.p {
        bound = c.p;
    }
    loop {
        let mut fb = FactorBases {
            sets: vec![Vec::new(); m],
            affine: true,
            p: c.p,
            ..Default::default()
        };
        if k > 0 {
            fb.window_bits = k;
            fb.x_bound = vec![xb; m];
        }
        for _ in 0..m {
            let mut a = rand_below(rng, c.p);
            let mut b = rand_below(rng, c.p);
            while a == 0 && b == 0 {
                a = rand_below(rng, c.p);
                b = rand_below(rng, c.p);
            }
            fb.alpha.push(a);
            fb.beta.push(b);
            fb.gamma.push(rand_below(rng, c.p));
            fb.bound.push(bound);
        }
        fb.list(c);
        let mut empty = false;
        for set in &fb.sets {
            empty = empty || set.is_empty();
        }
        if !empty {
            return fb;
        }
    }
}

impl FactorBases {
    /// list fills Sets, Points and Index. Toy shortcut: it enumerates the
    /// group. At scale F_i is only ever accessed through its membership
    /// predicate.
    /// list fills sets, points and index. With a coordinate window it only
    /// visits the window; otherwise it enumerates the whole group (Go: list).
    pub(crate) fn list(&mut self, c: &Curve) {
        if !self.x_bound.is_empty() {
            self.list_window(c);
            return;
        }
        self.list_all(c);
    }

    /// list_all is list over the whole group (Go: listAll).
    pub(crate) fn list_all(&mut self, c: &Curve) {
        let m = self.sets.len();
        let points = c.points();
        crate::opcount::bump("fb_points_listed_uncharged", points.len() as u64);
        crate::opcount::bump("fb_membership_evals_uncharged", (points.len() * m) as u64);
        for pt in points {
            for i in 0..m {
                if self.member(i, pt) {
                    self.sets[i].push(pt);
                    if !self.index.contains_key(&pt) {
                        self.index.insert(pt, self.points.len());
                        self.points.push(pt);
                    }
                }
            }
        }
    }

    /// window_candidates returns F_i without listing the whole group when the
    /// factor bases carry a coordinate window: it enumerates the window's
    /// points by square root and keeps those that pass the hash. Without a
    /// window it returns a copy of sets[i] (Go: WindowCandidates).
    pub fn window_candidates(&self, c: &Curve, i: usize) -> Vec<Point> {
        if self.x_bound.is_empty() {
            return self.sets[i].clone();
        }
        c.window_points(self.x_bound[i])
            .into_iter()
            .filter(|&pt| self.member(i, pt))
            .collect()
    }

    /// list_window is list_all restricted to the union of the coordinate
    /// windows: it takes square roots of x^3 + a x + b for x below the largest
    /// x_bound, then applies each F_i's membership predicate (Go: listWindow).
    pub(crate) fn list_window(&mut self, c: &Curve) {
        let xmax = self.x_bound.iter().copied().max().unwrap_or(0);
        let pts = c.window_points(xmax);
        let m = self.sets.len();
        crate::opcount::bump("fb_points_listed_uncharged", pts.len() as u64);
        crate::opcount::bump("fb_membership_evals_uncharged", (pts.len() * m) as u64);
        for pt in pts {
            for i in 0..m {
                if self.member(i, pt) {
                    self.sets[i].push(pt);
                    if !self.index.contains_key(&pt) {
                        self.index.insert(pt, self.points.len());
                        self.points.push(pt);
                    }
                }
            }
        }
    }

    /// AffineHash returns (Alpha_i x + Beta_i y + Gamma_i) mod p.
    pub fn affine_hash(&self, i: usize, pt: Point) -> u64 {
        let p = self.p;
        let h = add_mod(
            mul_mod(self.alpha[i], pt.x, p),
            mul_mod(self.beta[i], pt.y, p),
            p,
        );
        add_mod(h, self.gamma[i], p)
    }

    /// Member is the membership predicate of F_i. O is never a member.
    /// Non-affine: SHA-256(keys[i] || x_le8 || y_le8), first 8 bytes read
    /// little-endian, `% mods[i] == 0`.
    pub fn member(&self, i: usize, pt: Point) -> bool {
        if pt.inf {
            return false;
        }
        if !self.x_bound.is_empty() && pt.x >= self.x_bound[i] {
            return false;
        }
        if self.affine {
            return self.affine_hash(i, pt) < self.bound[i];
        }
        let mut buf = [0u8; 16];
        buf[..8].copy_from_slice(&pt.x.to_le_bytes());
        buf[8..].copy_from_slice(&pt.y.to_le_bytes());
        let mut h = Sha256::new();
        h.update(self.keys[i]);
        h.update(buf);
        let sum = h.finalize();
        let v = u64::from_le_bytes(sum[..8].try_into().unwrap());
        v % self.mods[i] == 0
    }
}

/// RelationNetwork encodes the search for A_1 + ... + A_m = R with A_i in F_i
/// as a Boolean tensor network. Edges are the bits of A_1..A_m and of the
/// partial sums S_2..S_m (S_1 = A_1). Vertices are:
///   - membership(i): A_i is a valid point in F_i (a sparse support tensor);
///   - add(k): S_k = S_{k-1} + A_k on valid points (a function tensor);
///   - target: S_m = R.
///
/// Every edge has exactly two endpoints, so this is a graph. At toy scale each
/// vertex reads whole points (degree 3w for add, where w = 2b+1); the paper's
/// construction splits these into gadgets of degree O(b^(3/4)).
/// It returns the network and the edge indices of each A_i.
pub fn relation_network(c: &Curve, fb: &FactorBases, r: Point) -> (Network, Vec<Vec<usize>>) {
    let w = c.point_width();
    let m = fb.sets.len();
    let mut next = 0usize;
    let mut block = || -> Vec<usize> {
        let e: Vec<usize> = (next..next + w).collect();
        next += w;
        e
    };
    let mut a: Vec<Vec<usize>> = Vec::with_capacity(m);
    for _ in 0..m {
        a.push(block());
    }
    let mut n = Network::default();
    let member = |i: usize| -> Vertex {
        let rows: Vec<Vec<bool>> = fb.sets[i].iter().map(|&pt| c.encode_point(pt)).collect();
        Vertex::Support(SupportVertex::new(a[i].clone(), rows))
    };
    n.vertices.push(member(0));
    let mut s = a[0].clone();
    for k in 1..m {
        n.vertices.push(member(k));
        let out = block();
        let mut in_ = s.clone();
        in_.extend_from_slice(&a[k]);
        let cc = *c;
        n.vertices.push(Vertex::Func(FuncVertex {
            in_,
            out: out.clone(),
            f: Rc::new(move |inp: &[bool]| -> Option<Vec<bool>> {
                let s = cc.decode_point(&inp[..w]);
                let a = cc.decode_point(&inp[w..]);
                match (s, a) {
                    (Some(s), Some(a)) => Some(cc.encode_point(cc.add(s, a))),
                    _ => None,
                }
            }),
        }));
        s = out;
    }
    let want = bits_key(&c.encode_point(r));
    n.vertices.push(Vertex::Pred(PredVertex {
        e: s,
        p: Rc::new(move |vals: &[bool]| bits_key(vals) == want),
    }));
    n.num_edges = next;
    (n, a)
}

// ---------------------------------------------------------------------------
// Meet-in-the-middle relation search.
//
// The relation network fixes an order in which its edges are eliminated: a
// forward contraction enumerates F_1 x ... x F_m, which is the whole product.
// The same network can be eliminated from both ends toward the middle instead.
// For a chain that changes the cost from |F_1| x ... x |F_m| to
// |F|^{ceil(m/2)} + |F|^{floor(m/2)}, which for |F_i| ~ (2r)^(1/m) is about
// 2 sqrt(2r) instead of 2r. Nothing about the count changes; this is only a
// different elimination order, and it returns one witness instead of counting.

fn mitm_left(
    c: &Curve,
    sets: &[Vec<Point>],
    i: usize,
    sum: Point,
    picks: &mut Vec<u32>,
    out: &mut HashMap<Point, Vec<u32>>,
) {
    if i == sets.len() {
        out.entry(sum).or_insert_with(|| picks.clone());
        return;
    }
    for (j, &a) in sets[i].iter().enumerate() {
        picks.push(j as u32);
        mitm_left(c, sets, i + 1, c.add(sum, a), picks, out);
        picks.pop();
    }
}

#[allow(clippy::too_many_arguments)]
fn mitm_right(
    c: &Curve,
    sets: &[Vec<Point>],
    i: usize,
    sum: Point,
    r: Point,
    picks: &mut Vec<u32>,
    table: &HashMap<Point, Vec<u32>>,
) -> Option<(Point, Vec<u32>)> {
    if i == sets.len() {
        let need = c.add(r, c.neg(sum));
        if table.contains_key(&need) {
            return Some((need, picks.clone()));
        }
        return None;
    }
    for (j, &a) in sets[i].iter().enumerate() {
        picks.push(j as u32);
        if let found @ Some(_) = mitm_right(c, sets, i + 1, c.add(sum, a), r, picks, table) {
            picks.pop();
            return found;
        }
        picks.pop();
    }
    None
}

/// mitm_find_relation returns A_1 + ... + A_m = r with A_i in F_i, if one
/// exists, by meeting in the middle: the first half of the factor bases is
/// enumerated into a table keyed by its partial sum, then the second half is
/// enumerated and its complement is looked up. It performs about
/// |F|^(m/2) + |F|^(m/2) point additions instead of the product over all m
/// sets, so it is the same relation network contracted from both ends. The
/// witness is not checked here; callers verify membership and the sum.
pub fn mitm_find_relation(c: &Curve, fb: &FactorBases, r: Point) -> Option<Vec<Point>> {
    let m = fb.sets.len();
    if m < 2 {
        return None;
    }
    if fb.sets.iter().any(|s| s.is_empty()) {
        return None;
    }
    let h = m / 2;
    let mut table: HashMap<Point, Vec<u32>> = HashMap::new();
    let mut picks: Vec<u32> = Vec::with_capacity(h);
    mitm_left(c, &fb.sets[..h], 0, INFINITY, &mut picks, &mut table);
    let mut rpicks: Vec<u32> = Vec::with_capacity(m - h);
    let (need, right) = mitm_right(c, &fb.sets[h..], 0, INFINITY, r, &mut rpicks, &table)?;
    let left = table.get(&need)?.clone();
    let mut pts: Vec<Point> = Vec::with_capacity(m);
    for (i, &j) in left.iter().enumerate() {
        pts.push(fb.sets[i][j as usize]);
    }
    for (i, &j) in right.iter().enumerate() {
        pts.push(fb.sets[h + i][j as usize]);
    }
    Some(pts)
}
