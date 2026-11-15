//! Deterministic count screen, not a timing benchmark. Protocol frozen separately.
mod dense;
mod rational;
#[path = "../../crates/hec-native-ic/src/table_factor.rs"]
mod table_factor;
use hec_native_ic::{
    fp::Fp,
    instance::{factor_base, pick_instance},
    poly::Poly,
    rng::Rng,
    solve::Row,
};
use std::collections::{BTreeMap, HashSet};

fn choose(n: usize, k: usize) -> usize {
    let mut v = 1;
    for j in 0..k.min(n - k) {
        v = v * (n - j) / (j + 1);
    }
    v
}

// Independent dense Gaussian rank; no forest or anchor Echelon methods.
fn gaussian_rank(rows: &[Row], columns: usize, skip: usize, f: &Fp) -> usize {
    let mut basis: Vec<Option<Vec<u64>>> = vec![None; columns];
    let mut rank = 0;
    for row in rows {
        let mut v = vec![0; columns];
        for &(j, x) in row {
            if j as usize >= skip {
                v[j as usize] = x;
            }
        }
        for j in skip..columns {
            if v[j] == 0 {
                continue;
            }
            if let Some(b) = &basis[j] {
                let scale = v[j];
                for k in j..columns {
                    v[k] = f.sub(v[k], f.mul(scale, b[k]));
                }
            } else {
                let scale = f.inv(v[j]);
                for x in &mut v[j..] {
                    *x = f.mul(*x, scale);
                }
                basis[j] = Some(v);
                rank += 1;
                break;
            }
        }
    }
    rank
}

fn lower_pair(points: &[(u64, u64)], signs: usize, f: &Fp) -> (Poly, Poly) {
    let mut a = Poly::zero();
    let mut basis = Poly::one();
    for (i, &(x, y)) in points.iter().enumerate() {
        let y = if i > 0 && signs & (1 << (i - 1)) != 0 {
            f.neg(y)
        } else {
            y
        };
        let w = f.mul(f.sub(y, a.eval(x, f)), f.inv(basis.eval(x, f)));
        a = a.add(&basis.scale(w, f), f);
        basis = basis.mul(&Poly::x_minus(x, f), f);
    }
    (a, Poly::one())
}

fn main() {
    let args: Vec<_> = std::env::args().skip(1).collect();
    let primitive_mode = match args.as_slice() {
        [] => false,
        [arg] if arg == "--primitive" => true,
        _ => panic!("usage: rational_rank [--primitive]"),
    };
    for (g, p) in [(2usize, 1009u64), (3, 503), (4, 127)] {
        let Some(inst) = pick_instance(p, g) else {
            println!("{{\"g\":{g},\"p\":{p},\"status\":\"unavailable\"}}");
            continue;
        };
        let c = &inst.curve;
        let f = &c.fp;
        let ell = inst.ell;
        let lf = Fp::new(ell);
        let mut sqrt = 0u64;
        while sqrt * sqrt < p {
            sqrt += 1;
        }
        assert!((ell as u128).pow(2) > (sqrt as u128 + 1).pow(2 * g as u32));
        let fb = factor_base(c);
        let tables = table_factor::Tables::for_degree(p, g);
        let mut by = vec![usize::MAX; p as usize];
        for (j, &(x, _)) in fb.iter().enumerate() {
            by[x as usize] = j;
        }
        for h in [g + 6, 2 * g + 8, 4 * g + 12] {
            for d in 0..=3 {
                let m = if d == 0 { g + 1 } else { g + 2 * d };
                if h > fb.len() || m > h {
                    println!(
                        "{{\"g\":{g},\"p\":{p},\"h\":{h},\"d\":{d},\"status\":\"unavailable\"}}"
                    );
                    continue;
                }
                let mut rng = Rng::new(2026111503 ^ ((g as u64) << 40) ^ ((h as u64) << 16));
                let max_tuples = choose(h, m).min(1024);
                let mut tuples = HashSet::new();
                let mut relations = HashSet::new();
                let mut admitted_rows = Vec::new();
                let mut forest = dense::DenseForest::new(fb.len(), h, ell);
                let mut echelon = dense::Echelon::new(h, ell);
                let (
                    mut visited,
                    mut constructed,
                    mut zero,
                    mut nonprimitive,
                    mut degree_drop,
                    mut nonsplit,
                    mut many_off,
                    mut empty,
                    mut duplicate,
                    mut admitted,
                    mut projected,
                ) = (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0);
                let (mut normalized_admitted, mut normalized_projected) = (0, 0);
                let status;
                loop {
                    if echelon.rank == h - 1 {
                        status = "rank_reached";
                        break;
                    }
                    if visited >= 32768 {
                        status = "class_cap";
                        break;
                    }
                    if tuples.len() == max_tuples {
                        status = if max_tuples == choose(h, m) {
                            "exhausted"
                        } else {
                            "tuple_cap"
                        };
                        break;
                    }
                    let mut chosen = Vec::new();
                    while chosen.len() < m {
                        let j = rng.below(h as u64) as usize;
                        if !chosen.contains(&j) {
                            chosen.push(j);
                        }
                    }
                    chosen.sort_unstable();
                    if !tuples.insert(chosen.clone()) {
                        continue;
                    }
                    let pts: Vec<_> = chosen.iter().map(|&j| fb[j]).collect();
                    let mut a_poly = Poly::one();
                    for &(x, _) in &pts {
                        a_poly = a_poly.mul(&Poly::x_minus(x, f), f);
                    }
                    let batch = if d > 0 {
                        rational::SignedFamily::new(g, d, &pts, f).walsh(f)
                    } else {
                        (0..1 << (m - 1)).map(|s| lower_pair(&pts, s, f)).collect()
                    };
                    constructed += batch.len();
                    for (s, (a, b)) in batch.into_iter().enumerate() {
                        if echelon.rank == h - 1 || visited >= 32768 {
                            break;
                        }
                        visited += 1;
                        for (i, &(x, y)) in pts.iter().enumerate() {
                            let y = if i > 0 && s & (1 << (i - 1)) != 0 {
                                f.neg(y)
                            } else {
                                y
                            };
                            assert_eq!(a.eval(x, f), f.mul(y, b.eval(x, f)));
                        }
                        let norm = a.mul(&a, f).sub(&c.f.mul(&b.mul(&b, f), f), f);
                        let (q, rem) = norm.divrem(&a_poly, f);
                        assert!(rem.is_zero());
                        assert!(q.deg() <= g as isize);
                        if a.is_zero() && b.is_zero() {
                            zero += 1;
                            continue;
                        }
                        assert!(!q.is_zero());
                        let common = Poly::gcd(&a, &b, f).deg() > 0;
                        let (a, b, q, keep_mask) = if common {
                            nonprimitive += 1;
                            if !primitive_mode {
                                continue;
                            }
                            let z = rational::primitive(c, &pts, a, b).expect("nonzero pair");
                            (z.a, z.b, z.q, z.keep_mask)
                        } else {
                            (a, b, q, (1usize << m) - 1)
                        };
                        if q.deg() < g as isize {
                            degree_drop += 1;
                        }
                        let q = q.monic(f);
                        let Some(roots) = tables.split(&q) else {
                            nonsplit += 1;
                            continue;
                        };
                        let mut raw = BTreeMap::<usize, i64>::new();
                        for (i, &j) in chosen.iter().enumerate() {
                            if keep_mask & (1 << i) == 0 {
                                continue;
                            }
                            *raw.entry(j).or_default() += if i > 0 && s & (1 << (i - 1)) != 0 {
                                -1
                            } else {
                                1
                            };
                        }
                        for &(x, mult) in roots.as_slice() {
                            let j = by[x as usize];
                            assert_ne!(j, usize::MAX);
                            let bx = b.eval(x, f);
                            assert_ne!(bx, 0);
                            let y = f.mul(a.eval(x, f), f.inv(bx));
                            assert_eq!(f.mul(y, y), c.f.eval(x, f));
                            let sign = if y == fb[j].1 { 1 } else { -1 };
                            *raw.entry(j).or_default() += sign * mult as i64;
                        }
                        raw.retain(|_, v| *v != 0);
                        if raw.keys().filter(|&&j| j >= h).count() > 2 {
                            many_off += 1;
                            continue;
                        }
                        let mut row: Row = raw
                            .iter()
                            .filter_map(|(&j, &v)| {
                                let v = lf.from_i64(v);
                                (v != 0).then_some((j as u32, v))
                            })
                            .collect();
                        if row.is_empty() {
                            empty += 1;
                            continue;
                        }
                        let scale = lf.inv(row[0].1);
                        for (_, v) in &mut row {
                            *v = lf.mul(*v, scale);
                        }
                        if !relations.insert(row.clone()) {
                            duplicate += 1;
                            continue;
                        }
                        let mut sum = c.zero();
                        for (&j, &v) in &raw {
                            let (x, y) = fb[j];
                            let mut place = c.place(x, y);
                            if v < 0 {
                                place = c.neg(&place);
                            }
                            sum = c.add(&sum, &c.mul(&place, v.unsigned_abs()));
                        }
                        assert!(c.is_zero(&sum), "principal g={g} h={h} d={d} signs={s}");
                        admitted += 1;
                        if common {
                            normalized_admitted += 1;
                        }
                        admitted_rows.push(row.clone());
                        let mut off = Vec::new();
                        let mut on = Row::new();
                        for (j, v) in row {
                            if (j as usize) < h {
                                on.push((j, v));
                            } else {
                                off.push((j as usize + 1, v));
                            }
                        }
                        if let Some(pure) = forest.insert(&off, &on) {
                            projected += 1;
                            if common {
                                normalized_projected += 1;
                            }
                            echelon.insert(&pure, 0);
                            assert!(echelon.rank <= h - 1, "excessive rank");
                        }
                    }
                }
                let full_rank = gaussian_rank(&admitted_rows, fb.len(), 0, &lf);
                let off_rank = gaussian_rank(&admitted_rows, fb.len(), h, &lf);
                assert_eq!(
                    full_rank - off_rank,
                    echelon.rank,
                    "independent anchor-intersection rank"
                );
                let extra = if primitive_mode {
                    format!(",\"mode\":\"primitive\",\"normalized_admitted\":{normalized_admitted},\"normalized_projected\":{normalized_projected}")
                } else {
                    String::new()
                };
                println!("{{\"g\":{g},\"p\":{p},\"c\":{},\"ell\":{ell},\"fb\":{},\"h\":{h},\"d\":{d},\"m\":{m},\"tuples\":{},\"constructed\":{constructed},\"visited\":{visited},\"zero\":{zero},\"nonprimitive\":{nonprimitive},\"degree_drop\":{degree_drop},\"nonsplit\":{nonsplit},\"many_off\":{many_off},\"empty\":{empty},\"duplicate\":{duplicate},\"admitted\":{admitted},\"projected\":{projected},\"rank\":{},\"full_rank\":{full_rank},\"off_rank\":{off_rank},\"status\":\"{status}\"{extra}}}",inst.c,fb.len(),tuples.len(),echelon.rank);
            }
        }
    }
}
