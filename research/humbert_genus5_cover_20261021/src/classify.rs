//! Binary-pattern classifier for multiquadratic covers of E (or a genus-g0
//! hyperelliptic source) over F_{q^d}, odd characteristic.
//!
//! f = eta * prod(roots), roots a union of Frobenius-orbit pieces.  Each orbit of
//! size s | d contributes a non-empty mask of Z/s; the oo coordinate is deg f mod 2.
//! v in F_2^{branch points}; sigma shifts every orbit by one.  The cover is
//! F = F_{q^d}(x)(sqrt(f^{sigma^i}) : i < d) with group (Z/2)^m, m = rank<sigma^i v>.
//! Riemann-Hurwitz: 2g - 2 = -2^{m+1} + S 2^{m-1}; Kani-Rosen: g = sum over the
//! 2^m - 1 non-zero span vectors c of (wt(c)/2 - 1), checked independently.

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Orbit {
    pub size: usize,
    pub mask: u32,
}

pub struct Row {
    pub d: usize,
    pub g0: usize,
    pub deg_f: usize,
    pub orbits: Vec<Orbit>,
    pub m: usize,
    pub s: usize,
    pub g: i64,
    pub g_kani_rosen: i64,
    pub conj_distinct: bool,
    pub e: i64,
    pub twist: &'static str,
    pub hyperelliptic: bool,
    pub extra_factors: Vec<(usize, usize)>, // (genus, sigma-orbit size) of non-E positive-genus quotients
    pub target: bool,
}

fn rot(mask: u32, s: usize, j: usize) -> u32 {
    let j = j % s;
    let full = (1u32 << s) - 1;
    ((mask << j) | (mask >> (s - j))) & full
}

fn canonical(orbs: &[Orbit], d: usize) -> Vec<Orbit> {
    let mut best: Option<Vec<Orbit>> = None;
    for j in 0..d {
        let mut o: Vec<Orbit> = orbs.iter().map(|x| Orbit { size: x.size, mask: rot(x.mask, x.size, j) }).collect();
        o.sort();
        if best.as_ref().map_or(true, |b| o < *b) {
            best = Some(o);
        }
    }
    best.unwrap()
}

fn rank_gf2(vs: &[u64]) -> (usize, Vec<u64>) {
    let mut basis: Vec<u64> = Vec::new();
    for &v in vs {
        let mut x = v;
        for &b in &basis {
            let hb = 63 - b.leading_zeros();
            if (x >> hb) & 1 == 1 {
                x ^= b;
            }
        }
        if x != 0 {
            basis.push(x);
            basis.sort_by(|a, b| b.cmp(a));
            // keep reduced echelon by leading bit
            let mut nb: Vec<u64> = Vec::new();
            for &b in &basis {
                let mut y = b;
                for &c in &nb {
                    let hc = 63 - c.leading_zeros();
                    if (y >> hc) & 1 == 1 {
                        y ^= c;
                    }
                }
                if y != 0 {
                    nb.push(y);
                }
            }
            basis = nb;
            basis.sort_by(|a, b| b.cmp(a));
        }
    }
    (basis.len(), basis)
}

pub fn analyse(d: usize, g0: usize, orbs: &[Orbit]) -> Row {
    let nroots: usize = orbs.iter().map(|o| o.mask.count_ones() as usize).sum();
    let deg_f = nroots;
    let inf = deg_f % 2 == 1;
    // coordinates: orbits in order, then oo
    let ncoord: usize = orbs.iter().map(|o| o.size).sum::<usize>() + 1;
    let vec_for = |shift: usize| -> u64 {
        let mut v = 0u64;
        let mut off = 0;
        for o in orbs {
            v |= (rot(o.mask, o.size, shift) as u64) << off;
            off += o.size;
        }
        if inf {
            v |= 1u64 << off;
        }
        v
    };
    let gens: Vec<u64> = (0..d).map(vec_for).collect();
    let (m, basis) = rank_gf2(&gens);
    let support = gens.iter().fold(0u64, |a, &b| a | b);
    let s = support.count_ones() as usize;
    let _ = ncoord;
    // 2g - 2 = -2^{m+1} + S 2^{m-1}
    let two_g = -(1i64 << (m + 1)) + (s as i64) * (1i64 << (m - 1)) + 2;
    let g = two_g / 2;
    // span elements with coordinates in the basis
    let mut span: Vec<(u64, u32)> = Vec::new();
    for c in 1u32..(1 << m) {
        let mut v = 0u64;
        for (i, b) in basis.iter().enumerate() {
            if (c >> i) & 1 == 1 {
                v ^= b;
            }
        }
        span.push((v, c));
    }
    let gen_c = |v: u64| -> i64 { v.count_ones() as i64 / 2 - 1 };
    let g_kr: i64 = span.iter().map(|&(v, _)| gen_c(v)).sum();
    let conj_distinct = (1..d).all(|i| gens[i] != gens[0]);
    let e = g - (d * g0) as i64;
    // twist restriction: dependency sum_{i in I} sigma^i v = 0 with |I| odd
    let mut odd_dep = false;
    for sub in 1u32..(1 << d) {
        let mut x = 0u64;
        for i in 0..d {
            if (sub >> i) & 1 == 1 {
                x ^= gens[i];
            }
        }
        if x == 0 && sub.count_ones() % 2 == 1 {
            odd_dep = true;
        }
    }
    let twist = if odd_dep { "eta must be a square (else constant-field extension)" } else { "both twists" };
    // hyperelliptic: an involution tau (functional on the span) whose fixed quotient has genus 0
    let mut hyper = false;
    for tau in 1u32..(1 << m) {
        let gq: i64 = span.iter().filter(|&&(_, c)| (c & tau).count_ones() % 2 == 0).map(|&(v, _)| gen_c(v)).sum();
        if gq == 0 {
            hyper = true;
        }
    }
    // extra positive-genus quotients and their sigma-orbit sizes
    let sigma = |v: u64| -> u64 {
        // apply one shift to a span vector: recompute coordinate-wise
        let mut out = 0u64;
        let mut off = 0;
        for o in orbs {
            let part = ((v >> off) & ((1u64 << o.size) - 1)) as u32;
            out |= (rot(part, o.size, 1) as u64) << off;
            off += o.size;
        }
        out | (v & (1u64 << off))
    };
    let mut extra = Vec::new();
    let mut seen: Vec<u64> = Vec::new();
    for &(v, _) in &span {
        if gen_c(v) <= 0 || gens.contains(&v) || seen.contains(&v) {
            continue;
        }
        let mut orb = vec![v];
        let mut w = sigma(v);
        while w != v {
            orb.push(w);
            w = sigma(w);
        }
        for &x in &orb {
            seen.push(x);
        }
        extra.push((gen_c(v) as usize, orb.len()));
    }
    let target = d == 4
        && g0 == 1
        && ((deg_f == 3 && orbs == [Orbit { size: 4, mask: 0b0111 }])
            || (deg_f == 4 && orbs == [Orbit { size: 1, mask: 1 }, Orbit { size: 4, mask: 0b0111 }]));
    Row { d, g0, deg_f, orbits: orbs.to_vec(), m, s, g, g_kani_rosen: g_kr, conj_distinct, e, twist, hyperelliptic: hyper, extra_factors: extra, target }
}

/// Enumerate every orbit configuration with deg f in {2 g0 + 1, 2 g0 + 2}.
pub fn enumerate(d: usize, g0: usize) -> Vec<Row> {
    let mut types: Vec<Orbit> = Vec::new();
    for s in 1..=d {
        if d % s != 0 {
            continue;
        }
        for mask in 1u32..(1 << s) {
            types.push(Orbit { size: s, mask });
        }
    }
    let mut out: Vec<Row> = Vec::new();
    let mut seen: Vec<Vec<Orbit>> = Vec::new();
    for deg in [2 * g0 + 1, 2 * g0 + 2] {
        let mut cur: Vec<Orbit> = Vec::new();
        rec(&types, 0, deg, &mut cur, d, g0, &mut seen, &mut out);
    }
    out
}

#[allow(clippy::too_many_arguments)]
fn rec(types: &[Orbit], start: usize, left: usize, cur: &mut Vec<Orbit>, d: usize, g0: usize, seen: &mut Vec<Vec<Orbit>>, out: &mut Vec<Row>) {
    if left == 0 {
        // squarefree is automatic (distinct orbits are distinct roots); require
        // the whole root set not to be defined over a proper subfield trivially? no: keep all
        let c = canonical(cur, d);
        if !seen.contains(&c) {
            seen.push(c.clone());
            out.push(analyse(d, g0, &c));
        }
        return;
    }
    for i in start..types.len() {
        let w = types[i].mask.count_ones() as usize;
        if w > left {
            continue;
        }
        cur.push(types[i].clone());
        rec(types, i, left - w, cur, d, g0, seen, out); // i (not i+1): repeated orbit types are distinct orbits
        cur.pop();
    }
}

pub fn row_json(r: &Row) -> String {
    let orbs: Vec<String> = r.orbits.iter().map(|o| format!("[{},\"{:0w$b}\"]", o.size, o.mask, w = o.size)).collect();
    let extra: Vec<String> = r.extra_factors.iter().map(|(g, o)| format!("[{},{}]", g, o)).collect();
    format!(
        "{{\"d\":{},\"g0\":{},\"deg_f\":{},\"orbits\":[{}],\"m\":{},\"S\":{},\"g\":{},\"g_kani_rosen\":{},\"conjugates_distinct\":{},\"e\":{},\"twist\":\"{}\",\"hyperelliptic\":{},\"extra_factors_genus_orbit\":[{}],\"target_family\":{}}}",
        r.d, r.g0, r.deg_f, orbs.join(","), r.m, r.s, r.g, r.g_kani_rosen, r.conj_distinct, r.e, r.twist, r.hyperelliptic, extra.join(","), r.target
    )
}
