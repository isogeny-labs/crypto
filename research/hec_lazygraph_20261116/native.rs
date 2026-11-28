use crate::lazy::{Graph,Row as GraphRow};
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
pub fn run(inst: &Instance, target: &hec_native_ic::jac::Div, m: usize, max_lp: usize, seed: u64, limit: u64) -> Report {
    assert!(inst.curve.is_valid(target), "invalid public target");
    assert!(!inst.curve.is_zero(target), "zero public target");
    assert!(inst.curve.is_zero(&inst.curve.mul(target, inst.ell)), "target outside ell torsion");
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
    let mut lp_graph=Graph::new(len-m,m+2,inst.ell);
    let mut rows: Vec<(Row, u64)> = Vec::new();let mut expanded=Vec::<GraphRow>::new();
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
        let mut coefficients = std::collections::BTreeMap::<usize,u64>::new();
        let mut decomposition = curve.zero();
        for &(x, mult) in roots.as_slice() {
            let j = by_x[x as usize];
            assert_ne!(j, usize::MAX);
            let positive = r.v.eval(x, fp) == fb[j].1;
            let previous = coefficients.get(&j).copied().unwrap_or(0);
            coefficients.insert(j, f.add(
                previous,
                if positive {
                    mult as u64
                } else {
                    f.neg(mult as u64)
                },
            ));
            let d = curve.mul(&curve.place(x, fb[j].1), mult as u64);
            decomposition = curve.add(&decomposition, &if positive { d } else { curve.neg(&d) });
        }
        assert_eq!(decomposition, r, "decomposition identity");
        rep.relation_checks += 1;
for &j in &steps {
    let previous=coefficients.get(&j).copied().unwrap_or(0);
    coefficients.insert(j,f.sub(previous,taken[j]%inst.ell));
}
let large:Vec<_>=coefficients.iter().filter(|&(&j,&v)|j>=m&&v!=0).map(|(&j,&v)|(j-m+1,v)).collect();
let mut affine:GraphRow=coefficients.iter().filter(|&(&j,&v)|j<m&&v!=0).map(|(&j,&v)|(j as u32,v)).collect();
if b!=0{affine.push((m as u32,f.neg(b)));}
if a!=0{affine.push(((m+1)as u32,f.neg(a)));}
let reduced=lp_graph.insert(&large,affine);
rep.partial_pivots=lp_graph.links;
for &(node,_) in &large{let(root,c,id)=lp_graph.find(node);let e=lp_graph.circuit.expand(id);let mut check=if root==0{curve.zero()}else{curve.mul(&projected[m+root-1],c)};for &(col,a)in &e{let point=if(col as usize)<m{&projected[col as usize]}else if col as usize==m{&d2}else{assert_eq!(col as usize,m+1);&d1};check=curve.add(&check,&curve.mul(point,a));}assert_eq!(check,projected[m+node-1],"LP graph potential identity");rep.projected_checks+=1;rep.max_partial_entries=rep.max_partial_entries.max(e.len());}
let Some(reduced)=reduced else{continue};let reduced=lp_graph.circuit.expand(reduced);let mut check=curve.zero();for &(col,a)in &reduced{let point=if(col as usize)<m{&projected[col as usize]}else if col as usize==m{&d2}else{assert_eq!(col as usize,m+1);&d1};check=curve.add(&check,&curve.mul(point,a));}assert!(curve.is_zero(&check),"full graph recombination identity");rep.projected_checks+=1;
expanded.push(reduced.clone());let rhs=f.neg(reduced.iter().find(|&&(c,_)|c as usize==m+1).map(|&(_,a)|a).unwrap_or(0));let row:Row=reduced.into_iter().filter(|&(c,_)|c as usize<=m).collect();if row.is_empty(){assert_eq!(rhs,0);continue;}rows.push((row,rhs));
        rep.rows = rows.len();
        if rows.len() >= wanted {
            if let Some(k) = solve_fast(&rows, m, inst.ell, &mut rep.solve) {
                if curve.mul(&inst.d1, k) == *target {
                    rep.k = Some(k);
                    break;
                }
            }
            wanted += 8 + m / 16;
        }
    }
assert_eq!(expanded.len(),lp_graph.circuit.roots.len());
for col in 0..m+2{let mut x=vec![0;m+2];x[col]=1;let want:Vec<_>=expanded.iter().map(|r|r.iter().find(|&&(c,_)|c as usize==col).map(|&(_,a)|a).unwrap_or(0)).collect();assert_eq!(lp_graph.circuit.apply(&x),want,"curve_forward_basis");}
for i in 0..expanded.len(){let mut y=vec![0;expanded.len()];y[i]=1;let mut want=vec![0;m+2];for&(c,a)in &expanded[i]{want[c as usize]=a;}assert_eq!(lp_graph.circuit.transpose(&y),want,"curve_transpose_basis");}
println!("CURVE_OPERATOR genus={} p={} seed={seed} lp={max_lp} outputs={} expanded_nnz={} circuit_nodes={} leaf_entries={} apply_mul_upper_bound={} exact_forward_transpose_basis=true explicit_scalar_solver_retained=true",inst.genus,inst.p,expanded.len(),expanded.iter().map(|r|r.len()).sum::<usize>(),lp_graph.circuit.nodes(),lp_graph.circuit.leaf_entries,lp_graph.circuit.leaf_entries+2*lp_graph.circuit.sums());
    rep.total_ns = clock.elapsed().as_nanos();
    rep
}

