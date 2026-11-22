use crate::linear::{put, Linear};
use crate::table_oracle::{split, OracleStats};
use hec_native_ic::{
    instance::factor_base,
    jac::{Curve, Div},
    poly::Poly,
};
use std::collections::BTreeMap;
#[path = "../hec_residual_span_20261116/batched/inverse.rs"]
mod inverse;

#[derive(Clone, Copy)]
pub struct Options {
    pub normalize: bool,
    pub finish: bool,
    pub seed: u64,
    pub certificate: bool,
    pub rolling: bool,
    pub carry: bool,
    pub indexed: bool,
    pub mode: usize,
    pub eager: bool,
    pub orders: usize,
}
#[derive(Debug, Default)]
pub struct Report {
    pub k: Option<u64>,
    pub multiple: usize,
    pub anchors: usize,
    pub unready: usize,
    pub expanded: usize,
    pub startup: usize,
    pub degree_skips: usize,
    pub scanned: usize,
    pub u_zero: usize,
    pub buckets: usize,
    pub nonsplit: usize,
    pub accepted: usize,
    pub repeated: usize,
    pub overlaps: usize,
    pub dense_audits: usize,
    pub rows: usize,
    pub pivots: usize,
    pub small: usize,
    pub catalogue: usize,
    pub zero_anchor: bool,
    pub final_rejected: bool,
    pub public_split: usize,
    pub public_nonsplit: usize,
    pub public_inserted: usize,
    pub public_pending: usize,
    pub public_probes: usize,
    pub branch_rows: usize,
    pub finish: crate::finish::Stats,
}
fn brute(mut f: Poly, c: &Curve) -> (bool, BTreeMap<u64, u32>) {
    let mut roots = BTreeMap::new();
    for x in 0..c.fp.p {
        while f.deg() > 0 && f.eval(x, &c.fp) == 0 {
            *roots.entry(x).or_insert(0) += 1;
            f = f.div_exact(&Poly::x_minus(x, &c.fp), &c.fp);
        }
    }
    (f.deg() == 0, roots)
}
pub fn solve(
    c: &Curve,
    d: &Div,
    target: &Div,
    ell: u64,
    limit: usize,
    audit: bool,
    prepared: &crate::prepared::Prepared,
    options: Options,
) -> Report {
    assert_eq!(prepared.roots.p, c.fp.p);
    let fb = prepared.fb.clone();
    let len = fb.len();
    let mut m = 1usize;
    while (m as u128).pow(c.g as u32) < (len as u128).pow((c.g - 1) as u32) {
        m += 1;
    }
    let width = match options.mode {
        1 => m,
        2 | 3 => (2 * m).min(len),
        4 => len,
        _ => unreachable!(),
    };
    let threshold = match options.mode {
        1 | 2 => c.g - 1,
        3 => c.g,
        4 => c.g + 1,
        _ => unreachable!(),
    };
    let mut index = vec![usize::MAX; c.fp.p as usize];
    for (j, &(x, _)) in fb.iter().enumerate() {
        index[x as usize] = j;
    }
    let mut linear = crate::dag::Dag::new(m, len, ell, audit, options.eager);
    let mut out = Report {
        small: m,
        catalogue: len,
        ..Report::default()
    };
    if options.normalize {
        for (j, &(_, y)) in fb.iter().enumerate() {
            if y == 0 {
                let mut row = BTreeMap::new();
                put(&mut row, if j < m { j } else { j + 1 }, 1, &linear.f);
                assert_eq!(linear.push(row, 0, audit), None);
                out.branch_rows += 1;
            }
        }
    }
    if options.orders > 0 {
        for (j, &(x, y)) in fb[..m].iter().enumerate() {
            if y == 0 {
                continue;
            }
            let point = c.place(x, y);
            let mut multiple = c.mul(&point, (c.g + 1) as u64);
            for order in c.g + 1..=c.g + options.orders {
                if order > c.g + 1 {
                    multiple = c.add_place(&multiple, x, y);
                }
                let Some(roots) = split(&multiple.u, &c.fp, &mut OracleStats::default(), &prepared.roots) else {
                    continue;
                };
                let mut row = BTreeMap::new();
                put(&mut row, j, linear.f.neg(order as u64), &linear.f);
                let mut recovered = c.zero();
                for &(a, power) in roots.as_slice() {
                    let i = index[a as usize];
                    let b = multiple.v.eval(a, &c.fp);
                    let sign = if b == fb[i].1 { 1 } else {
                        assert_eq!(b, c.fp.neg(fb[i].1));
                        -1
                    };
                    let col = if i < m { i } else { i + 1 };
                    put(&mut row, col, linear.f.from_i64(sign * power as i64), &linear.f);
                    if audit {
                        for _ in 0..power {
                            recovered = c.add_place(&recovered, a, b);
                        }
                    }
                }
                if audit {
                    assert_eq!(recovered, c.mul(&point, order as u64));
                }
                if !row.is_empty() && linear.ready(&row) {
                    assert_eq!(linear.push(row, 0, audit), None);
                    out.startup += 1;
                }
            }
        }
    }
    let (mut pending, nonsplit) = if options.normalize {
        crate::public::rows(c, d, target, m, &fb, &index, ell, audit, prepared)
    } else {
        (vec![], 0)
    };
    out.public_split = pending.len();
    out.public_nonsplit = nonsplit;
    let j = fb[..m]
        .iter()
        .position(|&(_, y)| y != 0)
        .expect("small nonbranch place");
    let (x, y) = fb[j];
    if options.certificate {
        let mut trial = c.zero();
        for n in 1..=32 {
            trial = c.add_place(&trial, x, y);
            if trial == *d {
                let mut row = BTreeMap::new();
                put(&mut row, j, n, &linear.f);
                assert_eq!(linear.push(row, 1, audit), None);
                out.multiple = n as usize;
                break;
            }
        }
    }
    let mut streams = [*target, *d];
    let mut cofactors = streams.map(|s| {
        if options.carry {
            crate::carried::cofactor(c, &s)
        } else {
            Poly::zero()
        }
    });
    let fp = &c.fp;
    let p = fp.p as usize;
    let mut indexed = crate::buckets::Buckets::new(if options.indexed { p } else { 0 });
    let mut inverse = vec![0; p];
    inverse[1] = 1;
    for i in 2..p {
        inverse[i] = fp.neg(fp.mul((p / i) as u64, inverse[p % i]));
    }
    let mut caches = [vec![None::<[u64; 2]>; width], vec![None::<[u64; 2]>; width]];

    'anchors: for a in 1..=limit {
        for entry in &mut pending {
            let Some((row, _)) = entry.as_ref() else {
                continue;
            };
            out.public_probes += 1;
            if !linear.ready(row) {
                continue;
            }
            let (row, rhs) = entry.take().unwrap();
            out.public_inserted += 1;
            let mut answer = linear.push(row, rhs, audit);

            if let Some(k) = answer {
                if c.mul(d, k) == *target {
                    out.k = Some(k);
                } else {
                    out.final_rejected = true;
                }
                break 'anchors;
            }
        }
        out.anchors = a;
        let stream = if out.multiple != 0 { 0 } else { (a - 1) % 2 };
        let n = if out.multiple != 0 { a } else { (a + 1) / 2 };
        let previous = streams[stream];
        let anchor = if options.carry {
            let cached = if previous.u.n == c.g + 1 {
                caches[stream][j].map(|q| q[0])
            } else {
                None
            };
            let next =
                crate::carried::step(c, &previous, &mut cofactors[stream], x, y, &inverse, cached);
            if audit {
                assert_eq!(next, c.add_place(&previous, x, y));
                assert_eq!(cofactors[stream], crate::carried::cofactor(c, &next));
            }
            next
        } else {
            c.add_place(&previous, x, y)
        };

        streams[stream] = anchor;
        if audit {
            let start = if stream == 0 { target } else { d };
            assert_eq!(anchor, c.add(start, &c.mul(&c.place(x, y), n as u64)));
        }
        let rhs = if stream == 0 { 0 } else { linear.f.neg(1) };
        if c.is_zero(&anchor) {
            let mut row = BTreeMap::new();
            put(&mut row, j, n as u64, &linear.f);
            if stream == 0 {
                put(&mut row, m, 1, &linear.f);
            }
            if let Some(k) = linear.push(row, rhs, audit) {
                if c.mul(d, k) == *target {
                    out.k = Some(k)
                } else {
                    out.final_rejected = true
                }
                break;
            }
            continue;
        }
        if anchor.u.deg() != c.g as isize {
            out.degree_skips += 1;
            continue;
        }
        let cache = &mut caches[stream];
        let slope = if options.rolling && previous.u.deg() == c.g as isize {
            cache[j].map(|s| s[0])
        } else {
            None
        };
        let mut buckets: BTreeMap<u64, Vec<(u64, u64)>> = BTreeMap::new();
        indexed.clear();
        let mut put_point = |t, p| {
            if options.indexed {
                indexed.push(t, p, threshold)
            } else {
                buckets.entry(t).or_default().push(p);
            }
        };
        for (i, &(px, py)) in fb[..width].iter().enumerate() {
            out.scanned += 1;
            let direct = || {
                let u = anchor.u.eval(px, fp);
                if u == 0 {
                    return None;
                }
                let v = anchor.v.eval(px, fp);
                Some([py, fp.neg(py)].map(|yy| fp.mul(fp.sub(yy, v), inverse[u as usize])))
            };
            let values = match (slope, cache[i]) {
                (Some(k), Some(old)) if px != x && !old.contains(&k) => {
                    let dx = fp.sub(px, x);
                    Some(old.map(|t| fp.sub(fp.mul(dx, inverse[fp.sub(t, k) as usize]), k)))
                }
                _ => direct(),
            };
            if audit {
                assert_eq!(values, direct(), "slope state versus Mumford evaluation");
            }
            cache[i] = values;
            if let Some(t) = values {
                put_point(t[0], (px, py));
                if py != 0 {
                    put_point(t[1], (px, fp.neg(py)));
                }
            } else {
                out.u_zero += 1;
            }
        }
        for (t, known) in buckets
            .iter()
            .map(|(t, k)| (*t, k))
            .chain(indexed.qualifying())
        {
            if known.len() < threshold {
                continue;
            }
            out.buckets += 1;
            let remainder = crate::tail::extract(c, &anchor, t, &known);
            let mut roots = BTreeMap::<u64, u32>::new();
            for &(x, _) in known {
                *roots.entry(x).or_default() += 1;
            }
            let h = if audit {
                let vt = anchor.v.add(&anchor.u.scale(t, &c.fp), &c.fp);
                let norm = c.f.sub(&vt.mul(&vt, &c.fp), &c.fp);
                let h = norm.div_exact(&anchor.u, &c.fp);
                assert_eq!(h.deg(), (c.g + 1) as isize);
                assert_eq!(h.lc(), 1);
                let mut reference = h;
                let mut known_poly = Poly::one();
                for &(x, y) in known {
                    assert_eq!(vt.eval(x, &c.fp), y);
                    let factor = Poly::x_minus(x, &c.fp);
                    reference = reference.div_exact(&factor, &c.fp);
                    known_poly = known_poly.mul(&factor, &c.fp);
                }
                assert_eq!(reference, remainder);
                assert_eq!(known_poly.mul(&remainder, &c.fp), h);
                assert_eq!(anchor.u.mul(&h, &c.fp), norm);
                Some(h)
            } else {
                None
            };
            let residual = split(
                &remainder,
                &c.fp,
                &mut OracleStats::default(),
                &prepared.roots,
            );
            let truth = if audit && options.mode != 4 {
                Some(brute(h.unwrap(), c))
            } else {
                None
            };
            if let Some((all, _)) = &truth {
                assert_eq!(residual.is_some(), *all);
            }
            let Some(residual) = residual else {
                out.nonsplit += 1;
                continue;
            };
            for &(x, mult) in residual.as_slice() {
                *roots.entry(x).or_default() += mult;
            }
            if let Some((_, expected)) = truth {
                assert_eq!(roots, expected);
            }
            assert_eq!(roots.values().copied().sum::<u32>() as usize, c.g + 1);
            out.repeated += roots.values().filter(|&&v| v > 1).count();
            let unknown: Vec<u64> = roots
                .keys()
                .copied()
                .filter(|x| !known.iter().any(|&(xx, _)| xx == *x))
                .collect();
            let images = crate::images::recover(c, &anchor, t, &remainder, &unknown, audit);
            out.overlaps += images.values().filter(|(_, u)| *u == 0).count();
            let mut row = BTreeMap::new();
            let mut native = c.zero();
            for (&x, &mult) in &roots {
                let j = index[x as usize];
                assert_ne!(j, usize::MAX);
                let y = known
                    .iter()
                    .find(|&&(xx, _)| xx == x)
                    .map(|&(_, yy)| yy)
                    .unwrap_or_else(|| images[&x].0);
                let sign = if y == fb[j].1 {
                    1
                } else {
                    assert_eq!(y, c.fp.neg(fb[j].1));
                    -1
                };
                let col = if j < m { j } else { j + 1 };
                put(
                    &mut row,
                    col,
                    linear.f.from_i64(sign * mult as i64),
                    &linear.f,
                );
                if audit {
                    for _ in 0..mult {
                        native = c.add_place(&native, x, y);
                    }
                }
            }
            if audit {
                assert!(
                    c.is_zero(&c.add(&native, &anchor)),
                    "principal divisor identity"
                );
            }
            put(&mut row, j, n as u64, &linear.f);
            if stream == 0 {
                put(&mut row, m, 1, &linear.f);
            }
            if !linear.ready(&row) {
                out.unready += 1;
                continue;
            }
            out.accepted += 1;
            let mut answer = linear.push(row, rhs, audit);

            if let Some(k) = answer {
                if c.mul(d, k) == *target {
                    out.k = Some(k);
                } else {
                    out.final_rejected = true;
                }
                break 'anchors;
            }
        }
        if out.k.is_some() {
            break;
        }
    }
    out.public_pending = pending.iter().filter(|r| r.is_some()).count();
    out.rows = linear.constraints;
    out.pivots = linear.joins;
    out.expanded = linear.expanded_links();
    out
}
