//! Genus-5 curves over F_2 for the étale screen at K-409 and K-571.
//!
//! Every genus-5 curve over F_2 is hyperelliptic, trigonal (a plane quintic with exactly one
//! singular point, of delta invariant 1) or a complete intersection of three quadrics in P^4.
//! Dragutinovic (arXiv:2202.07809) lists all three families with their point counts and
//! automorphism-group sizes. This module does two things.
//!
//! * `hyp-enum` and `trig-enum` enumerate the hyperelliptic and trigonal families
//!   independently of that paper: every model `y^2 + q y = p` (2^20 of them) and every plane
//!   quintic singular at (0:0:1) (2^18 forms, moved to the other six points by PGL_3(F_2)).
//!   The model counts divide by the group orders to the stack counts 512 and 2817.
//! * `sourced` reads the published models as untrusted data, recounts every curve over
//!   F_2..F_64 (hyperelliptic and trigonal also over F_128), recomputes L, checks it against
//!   the published counts, the Weil bounds and the out-of-sample counts, and screens it.
//!
//! The screen records which curves have n | P_B(lambda) for lambda of small order, as in
//! `screen`.

use crate::genus4::{power_sums, within_weil};
use crate::screen::{p_at, print_candidates, print_summary, small_order_units, Candidates};
use crate::{lpoly_from_counts, Stats};
use std::collections::BTreeMap;

const MODULI: [u32; 8] = [
    0, 0b11, 0b111, 0b1011, 0b10011, 0b100101, 0b1000011, 0b10000011,
];

/// GF(2^k), k <= 7, with log tables; elements are bit polynomials, so 0 and 1 are the F_2 part.
pub struct Fq {
    pub q: u32,
    exp: Vec<u8>,
    log: Vec<u8>,
}
impl Fq {
    pub fn new(k: usize) -> Fq {
        let q = 1u32 << k;
        let m = MODULI[k];
        let raw = |a: u32, b: u32| -> u32 {
            let (mut a, mut b, mut r) = (a, b, 0u32);
            while b != 0 {
                if b & 1 == 1 {
                    r ^= a;
                }
                a <<= 1;
                b >>= 1;
            }
            for s in (0..8).rev() {
                if r >> (s + k) & 1 == 1 {
                    r ^= m << s;
                }
            }
            r
        };
        let order = (q - 1) as usize;
        for g in 1..q {
            let mut exp = vec![0u8; 2 * order.max(1)];
            let mut log = vec![0u8; q as usize];
            let mut seen = vec![false; q as usize];
            let mut t = 1u32;
            let mut ok = true;
            for i in 0..order {
                if t == 0 || seen[t as usize] {
                    ok = false;
                    break;
                }
                seen[t as usize] = true;
                exp[i] = t as u8;
                exp[i + order] = t as u8;
                log[t as usize] = i as u8;
                t = raw(t, g);
            }
            if ok && t == 1 {
                return Fq { q, exp, log };
            }
        }
        panic!("GF(2^{}) modulus not primitive", k);
    }
    #[inline]
    pub fn mul(&self, a: u8, b: u8) -> u8 {
        if a == 0 || b == 0 {
            0
        } else {
            self.exp[self.log[a as usize] as usize + self.log[b as usize] as usize]
        }
    }
    fn pow(&self, a: u8, mut e: u64) -> u8 {
        let (mut r, mut b) = (1u8, a);
        while e > 0 {
            if e & 1 == 1 {
                r = self.mul(r, b);
            }
            b = self.mul(b, b);
            e >>= 1;
        }
        r
    }
    /// Square root: a^(q/2).
    fn sqrt(&self, a: u8) -> u8 {
        if self.q == 2 {
            a
        } else {
            self.pow(a, (self.q / 2) as u64)
        }
    }
}

fn fields() -> Vec<Fq> {
    (0..=7).map(|k| Fq::new(k.max(1))).collect()
}

// ---------- shared bookkeeping ----------

/// Checks L (from N_1..N_5) against the Weil bounds and N_6, N_7 predicted from L against the
/// direct counts (`counts[5]`, `counts[6]` when present). Returns L or a reason.
fn finish_curve(counts: &[i64], st: &mut Stats) -> Result<Vec<i64>, String> {
    let lp = lpoly_from_counts(5, &counts[..5]);
    if !within_weil(5, &lp) {
        return Err(format!("Weil bound failed L={:?}", lp));
    }
    let p = power_sums(&lp, 7);
    for k in 6..=7usize {
        if counts.len() >= k {
            let predicted = (1i64 << k) + 1 - p[k];
            st.nk_checked += 1;
            if predicted != counts[k - 1] {
                st.nk_fail += 1;
                return Err(format!(
                    "N_{} mismatch predicted={} counted={} L={:?}",
                    k,
                    predicted,
                    counts[k - 1],
                    lp
                ));
            }
        }
    }
    Ok(lp)
}

/// Candidate bookkeeping for the screen.
struct Acc {
    targets: Vec<u64>,
    kmax: u64,
    units: Vec<Vec<(u64, u64)>>,
    cand: Candidates,
    lcount: BTreeMap<Vec<i64>, u64>,
    curves: u64,
}
impl Acc {
    fn new(targets: &[u64], kmax: u64) -> Acc {
        Acc {
            targets: targets.to_vec(),
            kmax,
            units: targets
                .iter()
                .map(|&n| small_order_units(n, kmax))
                .collect(),
            cand: Candidates::new(),
            lcount: BTreeMap::new(),
            curves: 0,
        }
    }
    fn add(&mut self, lp: &[i64]) {
        self.curves += 1;
        *self.lcount.entry(lp.to_vec()).or_default() += 1;
        for (t, &n) in self.targets.iter().enumerate() {
            let kmin = self.units[t]
                .iter()
                .filter(|&&(l, _)| p_at(lp, l, n) == 0)
                .map(|&(_, k)| k)
                .min();
            if let Some(k) = kmin {
                *self
                    .cand
                    .entry((n, k))
                    .or_default()
                    .entry(lp.to_vec())
                    .or_default() += 1;
            }
        }
    }
    fn report(&self, family: &str, extra: &str) {
        for lp in self.lcount.keys() {
            // `lpoly` records: the sets are compared across sources with sort and diff
            println!(
                "{{\"kind\":\"lpoly\",\"family\":\"{}\",\"L\":{:?}}}",
                family, lp
            );
        }
        print_candidates(family, 5, &self.cand);
        print_summary(
            family,
            5,
            self.curves,
            &self.lcount,
            &self.targets,
            self.kmax,
            &self.cand,
            extra,
        );
    }
}

fn gcd(a: u128, b: u128) -> u128 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

/// sum of 1/|Aut| as a reduced fraction.
fn stack_count(auts: &[u64]) -> (u128, u128) {
    let l = auts
        .iter()
        .fold(1u128, |l, &a| l / gcd(l, a as u128) * a as u128);
    let num: u128 = auts.iter().map(|&a| l / a as u128).sum();
    let g = gcd(num, l);
    (num / g, l / g)
}

fn ints(s: &str) -> Vec<u64> {
    let mut v = vec![];
    let mut cur: Option<u64> = None;
    for c in s.chars() {
        if let Some(d) = c.to_digit(10) {
            cur = Some(cur.unwrap_or(0) * 10 + d as u64);
        } else if let Some(x) = cur.take() {
            v.push(x);
        }
    }
    if let Some(x) = cur {
        v.push(x);
    }
    v
}

fn read(path: &str) -> String {
    std::fs::read_to_string(path).unwrap_or_else(|e| panic!("{}: {}", path, e))
}

// ---------- hyperelliptic: y^2 + q(x) y = p(x), deg q <= 6, deg p <= 12 ----------

struct HypTables {
    f: Vec<Fq>,
    pw: Vec<Vec<[u8; 13]>>,
    /// tab[k][a * q + b] = #{y in F_q : y^2 + a y = b}
    tab: Vec<Vec<u8>>,
}
impl HypTables {
    fn new() -> HypTables {
        let f = fields();
        let mut pw = vec![vec![]];
        let mut tab = vec![vec![]];
        for k in 1..=7usize {
            let q = 1usize << k;
            let mut p = vec![[0u8; 13]; q];
            for (x, row) in p.iter_mut().enumerate() {
                row[0] = 1;
                for i in 1..13 {
                    row[i] = f[k].mul(row[i - 1], x as u8);
                }
            }
            pw.push(p);
            let mut t = vec![0u8; q * q];
            for a in 0..q {
                for y in 0..q {
                    let v = f[k].mul(y as u8, y as u8) ^ f[k].mul(a as u8, y as u8);
                    t[a * q + v as usize] += 1;
                }
            }
            tab.push(t);
        }
        HypTables { f, pw, tab }
    }
    fn eval(&self, k: usize, mask: u32, x: usize) -> u8 {
        let row = &self.pw[k][x];
        (0..13)
            .filter(|&i| mask >> i & 1 == 1)
            .fold(0u8, |a, i| a ^ row[i])
    }
    /// formal derivative over F_2: sum of x^(i-1) over odd i
    fn deriv(&self, k: usize, mask: u32, x: usize) -> u8 {
        let row = &self.pw[k][x];
        (1..13)
            .step_by(2)
            .filter(|&i| mask >> i & 1 == 1)
            .fold(0u8, |a, i| a ^ row[i - 1])
    }
    /// The model y^2 + q y = p is singular somewhere over the algebraic closure (affine
    /// singular points have q(x) = 0, so x lies in F_(2^k) with k <= deg q <= 6).
    fn singular(&self, qm: u32, pm: u32) -> bool {
        if qm == 0 {
            return true;
        }
        let bit = |m: u32, i: u32| m >> i & 1;
        // chart at infinity: Y^2 + Q(u) Y = P(u), Q(0) = q_6, P(0) = p_12, Q'(0) = q_5, P'(0) = p_11
        if bit(qm, 6) == 0 && bit(qm, 5) * bit(pm, 12) == bit(pm, 11) {
            return true;
        }
        for k in 1..=6usize {
            for x in 0..(1usize << k) {
                if self.eval(k, qm, x) != 0 {
                    continue;
                }
                let y = self.f[k].sqrt(self.eval(k, pm, x));
                if self.f[k].mul(self.deriv(k, qm, x), y) ^ self.deriv(k, pm, x) == 0 {
                    return true;
                }
            }
        }
        false
    }
    fn counts(&self, qm: u32, pm: u32) -> Vec<i64> {
        (1..=7usize)
            .map(|k| {
                let q = 1usize << k;
                let aff: i64 = (0..q)
                    .map(|x| {
                        let (a, b) = (self.eval(k, qm, x) as usize, self.eval(k, pm, x) as usize);
                        self.tab[k][a * q + b] as i64
                    })
                    .sum();
                let (q6, p12) = ((qm >> 6 & 1) as usize, (pm >> 12 & 1) as usize);
                aff + self.tab[k][q6 * q + p12] as i64
            })
            .collect()
    }
}

pub fn run_hyp_enum(targets: &[u64], kmax: u64) {
    let t = HypTables::new();
    let mut st = Stats::default();
    let mut acc = Acc::new(targets, kmax);
    let mut bad = 0u64;
    for qm in 1u32..(1 << 7) {
        for pm in 0u32..(1 << 13) {
            if t.singular(qm, pm) {
                continue;
            }
            match finish_curve(&t.counts(qm, pm), &mut st) {
                Ok(lp) => acc.add(&lp),
                Err(e) => {
                    bad += 1;
                    eprintln!("hyp q={} p={}: {}", qm, pm, e);
                }
            }
        }
    }
    // the group of y -> y + r(x) (deg r <= 6) and PGL_2(F_2) has order 2^7 * 6
    let extra = format!(
        "\"models\":{},\"group_order\":768,\"stack_count\":{},\"expected_stack_count\":512,\"rejected\":{},\"nk_checked\":{},\"nk_fail\":{},",
        acc.curves,
        acc.curves as f64 / 768.0,
        bad,
        st.nk_checked,
        st.nk_fail
    );
    acc.report("g5_hyperelliptic_enum", &extra);
}

// ---------- trigonal: plane quintics with one singular point of delta invariant 1 ----------

/// Monomials of a plane quintic in the order of the published 21-tuples: X^a Y^b Z^c with
/// a, then b, descending.
fn quintic_monomials() -> Vec<[u32; 3]> {
    let mut v = vec![];
    for a in (0..=5u32).rev() {
        for b in (0..=5 - a).rev() {
            v.push([a, b, 5 - a - b]);
        }
    }
    assert_eq!(v.len(), 21);
    v
}

struct PlaneTables {
    pts: Vec<Vec<[u8; 3]>>,
    mon: Vec<Vec<[u8; 21]>>,
    dmon: Vec<Vec<[[u8; 21]; 3]>>,
}
impl PlaneTables {
    fn new() -> PlaneTables {
        let f = fields();
        let mons = quintic_monomials();
        let (mut pts, mut mon, mut dmon) = (vec![vec![]], vec![vec![]], vec![vec![]]);
        for k in 1..=7usize {
            let q = 1u32 << k;
            let mut p: Vec<[u8; 3]> = vec![];
            for a in 0..q {
                for b in 0..q {
                    p.push([1, a as u8, b as u8]);
                }
            }
            for b in 0..q {
                p.push([0, 1, b as u8]);
            }
            p.push([0, 0, 1]);
            let pw = |x: u8, e: u32| (0..e).fold(1u8, |acc, _| f[k].mul(acc, x));
            let mut mv = vec![[0u8; 21]; p.len()];
            let mut dv = vec![[[0u8; 21]; 3]; p.len()];
            for (i, pt) in p.iter().enumerate() {
                for (m, e) in mons.iter().enumerate() {
                    mv[i][m] = (0..3).fold(1u8, |acc, j| f[k].mul(acc, pw(pt[j], e[j])));
                    for v in 0..3 {
                        if e[v] % 2 == 1 {
                            dv[i][v][m] = (0..3).fold(1u8, |acc, j| {
                                f[k].mul(acc, pw(pt[j], if j == v { e[j] - 1 } else { e[j] }))
                            });
                        }
                    }
                }
            }
            pts.push(p);
            mon.push(mv);
            dmon.push(dv);
        }
        PlaneTables { pts, mon, dmon }
    }
    fn xor_bits(mask: u32, row: &[u8; 21]) -> u8 {
        (0..21)
            .filter(|&i| mask >> i & 1 == 1)
            .fold(0u8, |a, i| a ^ row[i])
    }
    fn is_singular(&self, k: usize, i: usize, mask: u32) -> bool {
        (0..3).all(|v| Self::xor_bits(mask, &self.dmon[k][i][v]) == 0)
    }
    fn plane_count(&self, k: usize, mask: u32) -> i64 {
        self.mon[k]
            .iter()
            .filter(|row| Self::xor_bits(mask, row) == 0)
            .count() as i64
    }
}

/// Dense homogeneous polynomial in 3 variables over F_2 of degree <= 5, index (a*6 + b)*6 + c.
type P3 = [u8; 216];
fn p3_mul(a: &P3, b: &P3) -> P3 {
    let mut r = [0u8; 216];
    for i in 0..216 {
        if a[i] == 0 {
            continue;
        }
        let (ia, ib, ic) = (i / 36, i / 6 % 6, i % 6);
        for j in 0..216 {
            if b[j] == 0 {
                continue;
            }
            let (ja, jb, jc) = (j / 36, j / 6 % 6, j % 6);
            let (ra, rb, rc) = (ia + ja, ib + jb, ic + jc);
            if ra < 6 && rb < 6 && rc < 6 {
                r[(ra * 6 + rb) * 6 + rc] ^= 1;
            }
        }
    }
    r
}

/// F'(x, y, z) = F(x c1 + y c2 + z P) for the columns c1, c2 completing P to a basis.
fn move_point_to_origin(mask: u32, p: [u8; 3]) -> P3 {
    let basis = [[1u8, 0, 0], [0, 1, 0], [0, 0, 1]];
    let det = |a: [u8; 3], b: [u8; 3], c: [u8; 3]| -> u8 {
        (a[0] & (b[1] & c[2] ^ b[2] & c[1]))
            ^ (a[1] & (b[0] & c[2] ^ b[2] & c[0]))
            ^ (a[2] & (b[0] & c[1] ^ b[1] & c[0]))
    };
    let mut cols = None;
    'o: for i in 0..3 {
        for j in i + 1..3 {
            if det(basis[i], basis[j], p) == 1 {
                cols = Some((basis[i], basis[j]));
                break 'o;
            }
        }
    }
    let (c1, c2) = cols.expect("point completes to a basis");
    // linear forms: X = c1[0] x + c2[0] y + p[0] z, and so on
    let lin = |v: usize| -> P3 {
        let mut r = [0u8; 216];
        if c1[v] == 1 {
            r[6 * 6] = 1; // x^1
        }
        if c2[v] == 1 {
            r[6] = 1; // y^1
        }
        if p[v] == 1 {
            r[1] = 1; // z^1
        }
        r
    };
    let mut one = [0u8; 216];
    one[0] = 1;
    let ls = [lin(0), lin(1), lin(2)];
    let mut pows: Vec<[P3; 6]> = vec![];
    for l in &ls {
        let mut row = [one; 6];
        for e in 1..6 {
            row[e] = p3_mul(&row[e - 1], l);
        }
        pows.push(row);
    }
    let mut out = [0u8; 216];
    for (m, e) in quintic_monomials().iter().enumerate() {
        if mask >> m & 1 == 0 {
            continue;
        }
        let t = p3_mul(
            &p3_mul(&pows[0][e[0] as usize], &pows[1][e[1] as usize]),
            &pows[2][e[2] as usize],
        );
        for i in 0..216 {
            out[i] ^= t[i];
        }
    }
    out
}

#[derive(Clone, Copy, PartialEq, Debug)]
enum Sing {
    Cusp,
    SplitNode,
    NonsplitNode,
}

/// Exactly one singular point over the algebraic closure (checked over F_(2^k), k <= 6; see the
/// argument in the README), rational, of delta invariant 1. Returns its type.
fn trig_analyze(t: &PlaneTables, mask: u32) -> Result<Sing, &'static str> {
    let sing1: Vec<usize> = (0..t.pts[1].len())
        .filter(|&i| t.is_singular(1, i, mask))
        .collect();
    if sing1.len() != 1 {
        return Err("rational singular points");
    }
    let p = t.pts[1][sing1[0]];
    for k in 2..=6usize {
        for i in 0..t.pts[k].len() {
            if t.pts[k][i] == p {
                continue;
            }
            if t.is_singular(k, i, mask) {
                return Err("further singular point");
            }
        }
    }
    let g = move_point_to_origin(mask, p);
    let c = |a: usize, b: usize, cc: usize| g[(a * 6 + b) * 6 + cc];
    // chart z = 1: degree-d part has monomials x^a y^b with a + b = d
    if c(0, 0, 5) != 0 || c(1, 0, 4) != 0 || c(0, 1, 4) != 0 {
        return Err("not singular at the moved point");
    }
    let (a, b, cy) = (c(2, 0, 3), c(1, 1, 3), c(0, 2, 3));
    if a | b | cy == 0 {
        return Err("multiplicity >= 3");
    }
    if b == 1 {
        return Ok(if a == 1 && cy == 1 {
            Sing::NonsplitNode
        } else {
            Sing::SplitNode
        });
    }
    // q_2 = (a x + c y)^2; tangent direction v = (c, a); a cusp needs F_3(v) != 0
    let (v1, v2) = (cy, a);
    let mut f3 = 0u8;
    for i in 0..=3usize {
        let j = 3 - i;
        let m = c(i, j, 2);
        let term = (if i == 0 { 1 } else { v1 }) & (if j == 0 { 1 } else { v2 });
        f3 ^= m & term;
    }
    if f3 == 1 {
        Ok(Sing::Cusp)
    } else {
        Err("delta >= 2")
    }
}

fn trig_counts(t: &PlaneTables, mask: u32, s: Sing) -> Vec<i64> {
    (1..=7usize)
        .map(|k| {
            let b = match s {
                Sing::Cusp => 1,
                Sing::SplitNode => 2,
                Sing::NonsplitNode => {
                    if k % 2 == 0 {
                        2
                    } else {
                        0
                    }
                }
            };
            t.plane_count(k, mask) - 1 + b
        })
        .collect()
}

pub fn run_trig_enum(targets: &[u64], kmax: u64) {
    let t = PlaneTables::new();
    let mons = quintic_monomials();
    // forms singular at (0:0:1): no monomial with a + b < 2
    let free: Vec<usize> = (0..21).filter(|&m| mons[m][0] + mons[m][1] >= 2).collect();
    assert_eq!(free.len(), 18);
    let mut st = Stats::default();
    let mut acc = Acc::new(targets, kmax);
    let mut bad = 0u64;
    let mut kinds = [0u64; 3];
    for bits in 1u32..(1 << 18) {
        let mask = free
            .iter()
            .enumerate()
            .filter(|&(i, _)| bits >> i & 1 == 1)
            .fold(0u32, |m, (_, &mon)| m | 1 << mon);
        let s = match trig_analyze(&t, mask) {
            Ok(s) => s,
            Err(_) => continue,
        };
        kinds[s as usize] += 1;
        match finish_curve(&trig_counts(&t, mask, s), &mut st) {
            Ok(lp) => acc.add(&lp),
            Err(e) => {
                bad += 1;
                eprintln!("trig mask={:#x}: {}", mask, e);
            }
        }
    }
    // each valid quintic has one singular point and PGL_3(F_2) (order 168) is transitive on the
    // 7 points, so all models number 7 x (those singular at (0:0:1)); the stack count is
    // models / 168
    let extra = format!(
        "\"forms_at_origin\":{},\"models\":{},\"group_order\":168,\"stack_count\":{},\"expected_stack_count\":2817,\"cusp\":{},\"split_node\":{},\"nonsplit_node\":{},\"rejected\":{},\"nk_checked\":{},\"nk_fail\":{},",
        acc.curves,
        7 * acc.curves,
        7.0 * acc.curves as f64 / 168.0,
        kinds[0],
        kinds[1],
        kinds[2],
        bad,
        st.nk_checked,
        st.nk_fail
    );
    acc.report("g5_trigonal_enum", &extra);
}

// ---------- complete intersections of three quadrics in P^4 ----------

/// Per field F_(2^k), k <= 6: the 15 products x_i x_j (i <= j, lexicographic) at every point of
/// P^4(F_q), stored by plane so that a quadric is a XOR of planes.
struct Planes {
    n: Vec<usize>,
    planes: Vec<Vec<Vec<u8>>>, // planes[k][monomial][point]
}
impl Planes {
    fn new(f: &[Fq]) -> Planes {
        let mut n = vec![0usize];
        let mut planes = vec![vec![]];
        for k in 1..=6usize {
            let q = 1usize << k;
            let total = (q.pow(5) - 1) / (q - 1);
            let mut pl = vec![vec![0u8; total]; 15];
            let mut idx = 0usize;
            let mut coords = [0u8; 5];
            for lead in 0..5usize {
                let free = 4 - lead;
                for rest in 0..q.pow(free as u32) {
                    coords = [0; 5];
                    coords[lead] = 1;
                    let mut r = rest;
                    for c in coords.iter_mut().skip(lead + 1) {
                        *c = (r % q) as u8;
                        r /= q;
                    }
                    let mut m = 0;
                    for i in 0..5 {
                        for j in i..5 {
                            pl[m][idx] = f[k].mul(coords[i], coords[j]);
                            m += 1;
                        }
                    }
                    idx += 1;
                }
            }
            let _ = coords;
            assert_eq!(idx, total);
            n.push(total);
            planes.push(pl);
        }
        Planes { n, planes }
    }
    /// Number of points of P^4(F_(2^k)) on all three quadrics.
    fn count(&self, k: usize, masks: [u16; 3]) -> i64 {
        const CH: usize = 4096;
        let total = self.n[k];
        let pl = &self.planes[k];
        let mut zeros = 0i64;
        let mut v = [[0u8; CH]; 3];
        let mut start = 0;
        while start < total {
            let len = CH.min(total - start);
            for (qi, &mask) in masks.iter().enumerate() {
                v[qi][..len].fill(0);
                for m in 0..15 {
                    if mask >> m & 1 == 1 {
                        let src = &pl[m][start..start + len];
                        for (d, s) in v[qi][..len].iter_mut().zip(src) {
                            *d ^= *s;
                        }
                    }
                }
            }
            for i in 0..len {
                if v[0][i] | v[1][i] | v[2][i] == 0 {
                    zeros += 1;
                }
            }
            start += len;
        }
        zeros
    }
}

fn mask15(c: &[u64]) -> u16 {
    c.iter()
        .enumerate()
        .fold(0u16, |m, (i, &b)| m | (b as u16) << i)
}

// ---------- the published lists ----------

struct Sourced {
    name: &'static str,
    counts: Vec<Vec<i64>>,
    lps: Vec<Vec<i64>>,
    auts: Vec<u64>,
}

/// Carry-less product of F_2[x] polynomials given as bit masks.
fn clmul(a: u64, b: u64) -> u64 {
    (0..64)
        .filter(|&i| b >> i & 1 == 1)
        .fold(0, |r, i| r ^ a << i)
}

/// F_2[x] expressions as printed by Sage: sums, products, powers and parentheses, e.g.
/// `(x^2 + x + 1)^2*x + 1`. Returns the bit mask of the coefficients.
fn parse_hyp_poly(s: &str) -> u32 {
    struct P<'a> {
        b: &'a [u8],
        i: usize,
    }
    impl P<'_> {
        fn ws(&mut self) {
            while self.i < self.b.len() && self.b[self.i] == b' ' {
                self.i += 1;
            }
        }
        fn peek(&mut self) -> Option<u8> {
            self.ws();
            self.b.get(self.i).copied()
        }
        fn expr(&mut self) -> u64 {
            let mut r = self.term();
            while self.peek() == Some(b'+') {
                self.i += 1;
                r ^= self.term();
            }
            r
        }
        fn term(&mut self) -> u64 {
            let mut r = self.power();
            while self.peek() == Some(b'*') {
                self.i += 1;
                r = clmul(r, self.power());
            }
            r
        }
        fn number(&mut self) -> u64 {
            self.ws();
            let st = self.i;
            while self.i < self.b.len() && self.b[self.i].is_ascii_digit() {
                self.i += 1;
            }
            std::str::from_utf8(&self.b[st..self.i])
                .unwrap()
                .parse()
                .expect("number")
        }
        fn power(&mut self) -> u64 {
            let base = match self.peek() {
                Some(b'(') => {
                    self.i += 1;
                    let e = self.expr();
                    assert_eq!(self.peek(), Some(b')'));
                    self.i += 1;
                    e
                }
                Some(b'x') => {
                    self.i += 1;
                    2
                }
                Some(c) if c.is_ascii_digit() => self.number() & 1,
                c => panic!("unexpected {:?} in {:?}", c, self.b),
            };
            if self.peek() == Some(b'^') {
                self.i += 1;
                let e = self.number();
                (0..e).fold(1, |r, _| clmul(r, base))
            } else {
                base
            }
        }
    }
    let mut p = P {
        b: s.as_bytes(),
        i: 0,
    };
    let r = p.expr();
    assert!(p.peek().is_none(), "trailing input in {:?}", s);
    assert!(r < 1 << 32, "degree");
    r as u32
}

/// Splits `a, b` at the comma outside parentheses.
fn split_top(s: &str) -> (&str, &str) {
    let mut d = 0;
    for (i, c) in s.char_indices() {
        match c {
            '(' => d += 1,
            ')' => d -= 1,
            ',' if d == 0 => return (&s[..i], &s[i + 1..]),
            _ => {}
        }
    }
    panic!("no top-level comma in {:?}", s)
}

pub fn run_sourced(dir_ci: &str, dir_mt: &str, targets: &[u64], kmax: u64) {
    let f = fields();
    let ht = HypTables::new();
    let pt = PlaneTables::new();
    let planes = Planes::new(&f);
    let mut all: Vec<Sourced> = vec![];
    let mut st = Stats::default();
    let mut accs: Vec<Acc> = vec![];

    // hyperelliptic: [[q, p], ...]
    {
        let txt = read(&format!("{}/HyperellipticCurvesData.txt", dir_mt));
        let published = ints(&read(&format!("{}/NumPts_Hyp.txt", dir_mt)));
        let auts = ints(&read(&format!(
            "{}/HyperellipticCurvesSizeOfAutoGrp.txt",
            dir_mt
        )));
        let body = txt.trim().trim_start_matches('[').trim_end_matches(']');
        let mut s = Sourced {
            name: "hyperelliptic",
            counts: vec![],
            lps: vec![],
            auts,
        };
        let mut acc = Acc::new(targets, kmax);
        let (mut singular, mut mismatch, mut bad) = (0u64, 0u64, 0u64);
        for (i, item) in body.split("], [").enumerate() {
            let item = item.trim_matches(|c| c == '[' || c == ']');
            let (qs, ps) = split_top(item);
            let (qm, pm) = (parse_hyp_poly(qs), parse_hyp_poly(ps));
            assert!(qm < 1 << 7 && pm < 1 << 13, "degree bounds, curve {}", i);
            if ht.singular(qm, pm) {
                singular += 1;
            }
            let counts = ht.counts(qm, pm);
            if published[5 * i..5 * i + 5]
                != counts[..5].iter().map(|&c| c as u64).collect::<Vec<_>>()[..]
            {
                mismatch += 1;
            }
            match finish_curve(&counts, &mut st) {
                Ok(lp) => {
                    acc.add(&lp);
                    s.lps.push(lp);
                }
                Err(e) => {
                    bad += 1;
                    eprintln!("sourced hyp {}: {}", i, e);
                }
            }
            s.counts.push(counts);
        }
        assert_eq!(
            published.len(),
            5 * s.counts.len(),
            "published count length"
        );
        println!(
            "{{\"kind\":\"sourced_check\",\"family\":\"hyperelliptic\",\"curves\":{},\"singular_by_own_test\":{},\"count_mismatch_vs_published\":{},\"rejected\":{}}}",
            s.counts.len(),
            singular,
            mismatch,
            bad
        );
        all.push(s);
        accs.push(acc);
    }
    // trigonal: [[(21 coefficients), aut], ...]
    {
        let v = ints(&read(&format!(
            "{}/TrigonalCurvesWithAutomorphisms.txt",
            dir_mt
        )));
        let published = ints(&read(&format!("{}/NumPts_Trig.txt", dir_mt)));
        assert_eq!(v.len() % 22, 0);
        let mut s = Sourced {
            name: "trigonal",
            counts: vec![],
            lps: vec![],
            auts: vec![],
        };
        let mut acc = Acc::new(targets, kmax);
        let (mut notok, mut mismatch, mut bad) = (0u64, 0u64, 0u64);
        for (i, rec) in v.chunks(22).enumerate() {
            let mask = rec[..21]
                .iter()
                .enumerate()
                .fold(0u32, |m, (j, &b)| m | (b as u32) << j);
            s.auts.push(rec[21]);
            let sing = match trig_analyze(&pt, mask) {
                Ok(x) => x,
                Err(e) => {
                    notok += 1;
                    eprintln!("sourced trig {}: {}", i, e);
                    continue;
                }
            };
            let counts = trig_counts(&pt, mask, sing);
            if published[5 * i..5 * i + 5]
                != counts[..5].iter().map(|&c| c as u64).collect::<Vec<_>>()[..]
            {
                mismatch += 1;
            }
            match finish_curve(&counts, &mut st) {
                Ok(lp) => {
                    acc.add(&lp);
                    s.lps.push(lp);
                }
                Err(e) => {
                    bad += 1;
                    eprintln!("sourced trig {}: {}", i, e);
                }
            }
            s.counts.push(counts);
        }
        assert_eq!(
            published.len(),
            5 * (v.len() / 22),
            "published count length"
        );
        println!(
            "{{\"kind\":\"sourced_check\",\"family\":\"trigonal\",\"curves\":{},\"not_one_delta1_point\":{},\"count_mismatch_vs_published\":{},\"rejected\":{}}}",
            v.len() / 22,
            notok,
            mismatch,
            bad
        );
        all.push(s);
        accs.push(acc);
    }
    // complete intersections: three 15-tuples per curve
    {
        let v = ints(&read(&format!("{}/Complete_Intersections.txt", dir_ci)));
        let published = ints(&read(&format!(
            "{}/Pts_Count_Complete_Intersections.txt",
            dir_ci
        )));
        let auts = ints(&read(&format!(
            "{}/Automorphisms_Complete_Intersections.txt",
            dir_ci
        )));
        assert_eq!(v.len() % 45, 0);
        let ncur = v.len() / 45;
        assert_eq!(published.len(), 5 * ncur);
        assert_eq!(auts.len(), ncur);
        let masks: Vec<[u16; 3]> = v
            .chunks(45)
            .map(|c| [mask15(&c[..15]), mask15(&c[15..30]), mask15(&c[30..])])
            .collect();
        let nthreads = 4;
        let per = ncur.div_ceil(nthreads);
        let results: Vec<Vec<[i64; 6]>> = std::thread::scope(|sc| {
            let hs: Vec<_> = masks
                .chunks(per)
                .map(|ch| {
                    let planes = &planes;
                    sc.spawn(move || {
                        ch.iter()
                            .map(|&m| {
                                let mut c = [0i64; 6];
                                for k in 1..=6 {
                                    c[k - 1] = planes.count(k, m);
                                }
                                c
                            })
                            .collect::<Vec<_>>()
                    })
                })
                .collect();
            hs.into_iter().map(|h| h.join().unwrap()).collect()
        });
        let mut s = Sourced {
            name: "complete_intersection",
            counts: vec![],
            lps: vec![],
            auts,
        };
        let mut acc = Acc::new(targets, kmax);
        let (mut mismatch, mut bad) = (0u64, 0u64);
        for (i, c) in results.into_iter().flatten().enumerate() {
            if published[5 * i..5 * i + 5]
                != c[..5].iter().map(|&x| x as u64).collect::<Vec<_>>()[..]
            {
                mismatch += 1;
            }
            match finish_curve(&c, &mut st) {
                Ok(lp) => {
                    acc.add(&lp);
                    s.lps.push(lp);
                }
                Err(e) => {
                    bad += 1;
                    eprintln!("sourced ci {}: {}", i, e);
                }
            }
            s.counts.push(c.to_vec());
        }
        println!(
            "{{\"kind\":\"sourced_check\",\"family\":\"complete_intersection\",\"curves\":{},\"count_mismatch_vs_published\":{},\"rejected\":{}}}",
            ncur, mismatch, bad
        );
        all.push(s);
        accs.push(acc);
    }
    // per-family reports, then the union
    let expected = [
        ("hyperelliptic", 1070usize, "512"),
        ("trigonal", 2854, "2817"),
        ("complete_intersection", 3905, "3584"),
    ];
    let mut union = Acc::new(targets, kmax);
    let mut total_aut: Vec<u64> = vec![];
    for ((s, acc), (name, ncurves, stack)) in all.iter().zip(&accs).zip(expected) {
        let (num, den) = stack_count(&s.auts);
        total_aut.extend(&s.auts);
        let extra = format!(
            "\"source\":\"published\",\"curves_expected\":{},\"stack_count\":\"{}/{}\",\"stack_count_expected\":\"{}\",",
            ncurves, num, den, stack
        );
        assert_eq!(s.name, name);
        acc.report(&format!("g5_{}_sourced", name), &extra);
        for lp in &s.lps {
            union.add(lp);
        }
    }
    let (num, den) = stack_count(&total_aut);
    let extra = format!(
        "\"curves_expected\":7829,\"isogeny_classes_expected\":4339,\"stack_count\":\"{}/{}\",\"stack_count_expected\":\"6913\",\"nk_checked\":{},\"nk_fail\":{},",
        num, den, st.nk_checked, st.nk_fail
    );
    union.report("g5_all_sourced", &extra);
}
