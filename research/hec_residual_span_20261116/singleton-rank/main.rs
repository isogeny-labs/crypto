#[path = "../canonical-root/root.rs"]
mod root;
use hec_native_ic::{
    counted,
    fp::Fp,
    instance::factor_base,
    jac::Curve,
    oracle::{split, OracleStats},
    poly::Poly,
    rng::Rng,
};
use std::collections::{BTreeMap, BTreeSet};
#[derive(Clone, Debug)]
struct Relation {
    a: u64,
    t: u64,
    points: Vec<(u64, u64)>,
}
#[derive(Clone, Debug, PartialEq, Eq)]
enum Answer {
    Inconsistent,
    Free,
    Scalar(u64),
}
fn analyze(mut rows: Vec<Vec<u64>>, n: usize, f: &Fp) -> (usize, Answer) {
    let mut rank = 0;
    let mut target = None;
    for j in 0..n {
        let Some(i) = (rank..rows.len()).find(|&i| rows[i][j] != 0) else {
            continue;
        };
        rows.swap(rank, i);
        let inv = f.inv(rows[rank][j]);
        for k in j..=n {
            rows[rank][k] = f.mul(rows[rank][k], inv);
        }
        for i in 0..rows.len() {
            if i == rank {
                continue;
            }
            let a = rows[i][j];
            if a == 0 {
                continue;
            }
            for k in j..=n {
                rows[i][k] = f.sub(rows[i][k], f.mul(a, rows[rank][k]));
            }
        }
        if j == n - 1 {
            target = Some(rank)
        }
        rank += 1;
        if rank == rows.len() {
            break;
        }
    }
    if rows
        .iter()
        .any(|r| r[..n].iter().all(|&x| x == 0) && r[n] != 0)
    {
        return (rank, Answer::Inconsistent);
    }
    (
        rank,
        if let Some(i) = target {
            Answer::Scalar(rows[i][n])
        } else {
            Answer::Free
        },
    )
}
fn canonical(c: &Curve, z: u64, point: (u64, u64)) -> ((u64, u64), usize, bool) {
    let f = &c.fp;
    let mut minx = point.0;
    let mut x = point.0;
    for _ in 0..5 {
        minx = minx.min(x);
        x = f.mul(x, z);
    }
    let y = point.1.min(f.neg(point.1));
    let mut x = minx;
    for e in 0..5 {
        if x == point.0 {
            return ((minx, y), e, point.1 != y);
        }
        x = f.mul(x, z);
    }
    unreachable!()
}
fn states(
    c: &Curve,
    z: u64,
    core: (u64, u64),
    sm: u64,
    ell: u64,
    rels: &[Relation],
) -> (usize, Vec<(u64, usize, Answer)>) {
    let f = Fp::new(ell);
    let mut reps = BTreeMap::new();
    reps.insert(canonical(c, z, core).0, 0);
    for rel in rels {
        for &pt in &rel.points {
            let key = canonical(c, z, pt).0;
            if !reps.contains_key(&key) {
                let j = reps.len();
                reps.insert(key, j);
            }
        }
    }
    let n = reps.len() + 1;
    let zell = root::power_map(&f, 5).0;
    let mut out = vec![];
    let (_, ce, neg) = canonical(c, z, core);
    for e in 1..5 {
        let r = f.pow(zell, e);
        let mut norm = vec![0; n + 1];
        let coeff = f.pow(r, ce as u64);
        norm[0] = if neg { f.neg(coeff) } else { coeff };
        norm[n] = f.inv(sm);
        let mut rows = vec![norm];
        for rel in rels {
            let mut row = vec![0; n + 1];
            row[n - 1] = 1;
            row[n] = f.neg(f.mul(rel.a, f.inv(sm)));
            for &pt in &rel.points {
                let (key, phase, neg) = canonical(c, z, pt);
                let j = reps[&key];
                let a = f.pow(r, phase as u64);
                row[j] = f.add(row[j], if neg { f.neg(a) } else { a });
            }
            rows.push(row);
        }
        let (rank, answer) = analyze(rows, n, &f);
        out.push((r, rank, answer));
    }
    (reps.len(), out)
}
fn main() {
    counted::disable();
    let (mut cells, mut solved, mut total_trials, mut total_relations) = (0, 0, 0, 0);
    for (p, cc, ell) in [(101, 2, 7481), (251, 18, 67931), (601, 7, 411491)] {
        let c = Curve::new(Fp::new(p), Poly::from_coeffs(&[cc, 0, 0, 0, 0, 1]));
        let f = &c.fp;
        let z = root::power_map(f, 5).0;
        let pt = factor_base(&c)[0];
        assert_ne!(pt.0, 0);
        let base = c.place(pt.0, pt.1);
        let mut rng = Rng::new(2026113000 ^ p);
        for sm in [1, 17] {
            let d = c.mul(&base, sm);
            for case in 0..2 {
                let k = 1 + rng.below(ell - 1);
                let target = c.mul(&d, k);
                let mut anchor = target;
                let mut seen = BTreeSet::new();
                let mut rels = vec![];
                let (
                    mut trials,
                    mut nonsplit,
                    mut duplicate,
                    mut zero,
                    mut skip,
                    mut skipped_anchor,
                    mut first,
                ) = (0, 0, 0, 0, 0, 0, None);
                for ai in 1..=16 {
                    anchor = c.add(&anchor, &base);
                    if anchor.u.deg() != 2 {
                        skipped_anchor += 1;
                        continue;
                    }
                    let mut x = pt.0;
                    for _ in 0..5 {
                        for y in [pt.1, f.neg(pt.1)] {
                            let u = anchor.u.eval(x, f);
                            if u == 0 {
                                skip += 1;
                                continue;
                            }
                            trials += 1;
                            let t = f.mul(f.sub(y, anchor.v.eval(x, f)), f.inv(u));
                            if !seen.insert((ai, t)) {
                                duplicate += 1;
                                continue;
                            }
                            let tt = f.mul(t, t);
                            let h2 = f.neg(f.add(tt, anchor.u.c[1]));
                            let w1 = f.add(anchor.v.c[1], f.mul(t, anchor.u.c[1]));
                            let h1 = f.neg(f.add(
                                f.add(f.mul(f.mul(2, t), w1), f.mul(anchor.u.c[1], h2)),
                                anchor.u.c[0],
                            ));
                            let q1 = f.add(h2, x);
                            let q0 = f.add(h1, f.mul(x, q1));
                            let q = Poly::from_coeffs(&[q0, q1, 1]);
                            let w = anchor.v.add(&anchor.u.scale(t, f), f);
                            let h = c.f.sub(&w.mul(&w, f), f).div_exact(&anchor.u, f);
                            assert_eq!(h.div_exact(&Poly::x_minus(x, f), f), q);
                            let Some(rs) = split(&q, f, &mut OracleStats::default()) else {
                                nonsplit += 1;
                                continue;
                            };
                            let mut points = vec![(x, y)];
                            for &(rx, m) in rs.as_slice() {
                                for _ in 0..m {
                                    points.push((rx, w.eval(rx, f)));
                                }
                            }
                            let mut group = anchor;
                            for &(x, y) in &points {
                                assert_eq!(f.mul(y, y), c.f.eval(x, f));
                                group = c.add_place(&group, x, y);
                            }
                            assert!(c.is_zero(&group));
                            if points.iter().any(|pt| pt.0 == 0) {
                                zero += 1;
                                continue;
                            }
                            rels.push(Relation { a: ai, t, points });
                        }
                        x = f.mul(x, z);
                    }
                    let (orbits, branches) = states(&c, z, pt, sm, ell, &rels);
                    let mut consensus = None;
                    let mut complete = true;
                    let mut survivors = 0;
                    for (_, _, answer) in &branches {
                        match answer {
                            Answer::Inconsistent => {}
                            Answer::Free => {
                                complete = false;
                                survivors += 1
                            }
                            Answer::Scalar(v) => {
                                survivors += 1;
                                if let Some(old) = consensus {
                                    if old != *v {
                                        complete = false
                                    }
                                } else {
                                    consensus = Some(*v)
                                }
                            }
                        }
                    }
                    assert!(survivors > 0);
                    if complete {
                        if let Some(v) = consensus {
                            assert_eq!(c.mul(&d, v), target);
                            assert_eq!(v, k);
                            if first.is_none() {
                                first = Some(ai)
                            }
                        }
                    }
                    println!("PREFIX\tp={p}\tsm={sm}\tcase={case}\tanchor={ai}\tqueries={trials}\trows={}\torbits={orbits}\tbranches={branches:?}\tconsensus={}",rels.len(),if complete{format!("{consensus:?}")}else{"None".into()});
                }
                if first.is_some() {
                    solved += 1
                }
                println!("CELL\tp={p}\tsm={sm}\tcase={case}\tqueries={trials}\trows={}\tnonsplit={nonsplit}\tduplicate_parameter={duplicate}\tu_zero={skip}\tx_zero_rows={zero}\tdegree_skips={skipped_anchor}\tfirst_consensus_anchor={first:?}\trelations={rels:?}",rels.len());
                total_trials += trials;
                total_relations += rels.len();
                cells += 1;
            }
        }
    }
    assert_eq!(cells, 12);
    eprintln!("PASS cells={cells} completed={solved} incomplete={} queries={total_trials} relations={total_relations} native_principal_checks=true all_branch_consensus_checked=true performance_claim=false independent_verification=false",12-solved);
}
