use crate::graph::{Graph,Row as GraphRow};
// Bounded 0/1/2-large-prime baseline using exact partial elimination.
// The full catalogue is materialized; this is a correctness baseline.
use hec_native_ic::{
    fp::Fp,
    instance::{factor_base, order_bsgs, Instance},
    oracle::{split, OracleStats},
    rng::Rng,
    solve::{solve_fast, Row, SolveStats},
};
use std::time::Instant;

#[derive(Default, Debug)]
pub struct Report {
    pub k: Option<u64>,
    pub trials: u64,
    pub partials: u64,
    pub partial_pivots: usize,
    pub rows: usize,
    pub relation_checks: u64,
    pub projected_checks: u64,
    pub max_partial_entries: usize,
    pub solve: SolveStats,
    pub total_ns: u128,
}
fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        (a, b) = (b, a % b);
    }
    a
}

/// All validation, catalogue, projection, and exact elimination are charged.
pub fn run(inst: &Instance, m: usize, max_lp: usize, seed: u64, limit: u64) -> Report {
    let clock = Instant::now();
    let mut rep = Report::default();
    let curve = &inst.curve;
    let fp = &curve.fp;
    let f = Fp::new(inst.ell);
    let fb = factor_base(curve);
    let len = fb.len();
    assert!(m > 0 && m <= len && max_lp <= 2);
    // Find a common annihilator for this catalogue, then remove its single
    // ell factor. This certifies a projection on every place actually used.
    let mut common = inst.ell;
    for &(x, y) in &fb {
        let order =
            order_bsgs(curve, &curve.place(x, y), 200_000).expect("bounded catalogue order");
        common = (common / gcd(common, order))
            .checked_mul(order)
            .expect("catalogue exponent overflow");
    }
    assert_eq!(common % inst.ell, 0);
    let cofactor = common / inst.ell;
    assert_ne!(cofactor % inst.ell, 0);
    let projected: Vec<_> = fb
        .iter()
        .map(|&(x, y)| curve.mul(&curve.place(x, y), cofactor))
        .collect();
    for d in &projected {
        assert!(curve.is_zero(&curve.mul(d, inst.ell)));
    }
    let d1 = curve.mul(&inst.d1, cofactor);
    assert!(!curve.is_zero(&d1));
    let kt = inst.ell / 3 + 7;
    let target = curve.mul(&inst.d1, kt);
    let d2 = curve.mul(&target, cofactor);
    let mut by_x = vec![usize::MAX; fp.p as usize];
    for (j, &(x, _)) in fb.iter().enumerate() {
        by_x[x as usize] = j;
    }
    let mut rng = Rng::new(seed);
    let branches = m.min(64);
    let mut steps: Vec<usize> = (0..m).collect();
    for i in 0..branches {
        let j = i + rng.below((m - i) as u64) as usize;
        steps.swap(i, j);
    }
    steps.truncate(branches);
    let mut taken = vec![0u64; m];
    let (mut a, mut b) = (1, 1);
    let mut r = curve.add(&inst.d1, &target);
    let mut lp_graph=Graph::new(len-m,m+1,inst.ell);
    let mut rows: Vec<(Row, u64)> = Vec::new();
    let mut wanted = m + 8;
    while rep.trials < limit {
        rep.trials += 1;
        match rng.below(4) {
            0 => {
                a = f.add(a, 1);
                r = curve.add(&r, &inst.d1);
            }
            1 => {
                b = f.add(b, 1);
                r = curve.add(&r, &target);
            }
            _ => {
                let j = steps[rng.below(branches as u64) as usize];
                taken[j] += 1;
                let (x, y) = fb[j];
                r = curve.add_place(&r, x, y);
            }
        }
        let Some(roots) = split(&r.u, fp, &mut OracleStats::default()) else {
            continue;
        };
        let mut off = 0;
        for &(x, _) in roots.as_slice() {
            if by_x[x as usize] >= m {
                off += 1;
            }
        }
        if off > max_lp {
            continue;
        }
        if off > 0 {
            rep.partials += 1;
        }
        let mut v = vec![0; len + 1];
        let mut decomposition = curve.zero();
        for &(x, mult) in roots.as_slice() {
            let j = by_x[x as usize];
            assert_ne!(j, usize::MAX);
            let positive = r.v.eval(x, fp) == fb[j].1;
            v[j] = f.add(
                v[j],
                if positive {
                    mult as u64
                } else {
                    f.neg(mult as u64)
                },
            );
            let d = curve.mul(&curve.place(x, fb[j].1), mult as u64);
            decomposition = curve.add(&decomposition, &if positive { d } else { curve.neg(&d) });
        }
        assert_eq!(decomposition, r, "decomposition identity");
        rep.relation_checks += 1;
        for j in 0..m {
            v[j] = f.sub(v[j], taken[j] % inst.ell);
        }
        v[len] = f.neg(b);
let large:Vec<_>=(m..len).filter(|&j|v[j]!=0).map(|j|(j-m+1,v[j])).collect();
let mut affine:GraphRow=(0..m).filter(|&j|v[j]!=0).map(|j|(j as u32,v[j])).collect();if v[len]!=0{affine.push((m as u32,v[len]));}if a!=0{affine.push(((m+1)as u32,f.neg(a)));}
let reduced=lp_graph.insert(&large,affine);
rep.partial_pivots=(lp_graph.stats.merges+lp_graph.stats.anchors)as usize;
for &(node,_) in &large{let(root,c,e)=lp_graph.find(node);let mut check=if root==0{curve.zero()}else{curve.mul(&projected[m+root-1],c)};for &(col,a)in &e{let point=if(col as usize)<m{&projected[col as usize]}else if col as usize==m{&d2}else{assert_eq!(col as usize,m+1);&d1};check=curve.add(&check,&curve.mul(point,a));}assert_eq!(check,projected[m+node-1],"LP graph potential identity");rep.projected_checks+=1;rep.max_partial_entries=rep.max_partial_entries.max(e.len());}
let Some(reduced)=reduced else{continue};let mut check=curve.zero();for &(col,a)in &reduced{let point=if(col as usize)<m{&projected[col as usize]}else if col as usize==m{&d2}else{assert_eq!(col as usize,m+1);&d1};check=curve.add(&check,&curve.mul(point,a));}assert!(curve.is_zero(&check),"full graph recombination identity");rep.projected_checks+=1;
let rhs=f.neg(reduced.iter().find(|&&(c,_)|c as usize==m+1).map(|&(_,a)|a).unwrap_or(0));let row:Row=reduced.into_iter().filter(|&(c,_)|c as usize<=m).collect();if row.is_empty(){assert_eq!(rhs,0);continue;}rows.push((row,rhs));
        rep.rows = rows.len();
        if rows.len() >= wanted {
            if let Some(k) = solve_fast(&rows, m, inst.ell, &mut rep.solve) {
                if curve.mul(&inst.d1, k) == target {
                    rep.k = Some(k);
                    break;
                }
            }
            wanted += 8 + m / 16;
        }
    }
    println!("LP_GRAPH stats={:?} signed_nonunit_coefficients=true projected_potential_and_full_row_checks=true",lp_graph.stats);
    rep.total_ns = clock.elapsed().as_nanos();
    rep
}

