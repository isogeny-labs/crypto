//! Algebraic S3 root-index CONTROL for the ordinary cofactor-one prime31 curve.
//! No known ordinates or EC group sums enter the coordinate discovery index/query.
//! Crucial caveat: split S3 factors encode pair sums, making this essentially
//! algebraic coordinate enumeration/meet-in-the-middle, NOT a new non-generic
//! ECDLP complexity improvement. It is a correctness/engineering control for
//! the factorized S4 elimination and an m8 design gate.
//!
//! cargo +1.100.0 test --locked --release --manifest-path research/prime31_ic_20261115/native/Cargo.toml --example prime31_s3_root_index
//! cargo +1.100.0 run --locked --release --manifest-path research/prime31_ic_20261115/native/Cargo.toml --example prime31_s3_root_index -- --natural-16384

use prime31_algebraic_ic::{check_curve, Arithmetic, Base, Point, PRIME31};
use serde_json::json;
use std::collections::{BTreeMap, BTreeSet};
use std::time::Instant;

const P: u64 = 2_147_483_647;
const N: u64 = 2_147_446_831;
const SEED: u64 = 0x525553545f4d3401;
const TRIALS: usize = 16_384;

fn add(a: u64, b: u64) -> u64 {
    let r = a + b;
    if r >= P { r - P } else { r }
}
fn sub(a: u64, b: u64) -> u64 {
    if a >= b { a - b } else { a + P - b }
}
fn mul(a: u64, b: u64) -> u64 { ((a as u128 * b as u128) % P as u128) as u64 }
fn power(mut a: u64, mut e: u64) -> u64 {
    let mut r = 1;
    while e != 0 {
        if e & 1 == 1 { r = mul(r, a); }
        a = mul(a, a);
        e >>= 1;
    }
    r
}
fn sqrt(a: u64) -> Option<u64> {
    let r = power(a, (P + 1) / 4);
    (mul(r, r) == a).then_some(r)
}
fn s3(x: u64, y: u64) -> [u64; 3] {
    let sum = add(x, y);
    let xy = mul(x, y);
    let d = sub(x, y);
    let aa = mul(d, d);
    let bb = sub(0, mul(2, add(mul(sum, add(xy, PRIME31.a)), mul(2, PRIME31.b))));
    let cc = sub(mul(sub(xy, PRIME31.a), sub(xy, PRIME31.a)),
                 mul(mul(4, PRIME31.b), sum));
    [cc, bb, aa]
}
fn eval(f: [u64; 3], x: u64) -> u64 {
    add(add(f[0], mul(f[1], x)), mul(f[2], mul(x, x)))
}
// All roots over Fp of one degree <=2 S3 coefficient polynomial.
// Zero polynomial is explicit undetermined; never silently call it empty.
fn roots(f: [u64; 3]) -> Result<Vec<u64>, String> {
    let [c, b, a] = f;
    let mut result = if a != 0 {
        let delta = sub(mul(b, b), mul(mul(4, a), c));
        if let Some(root) = sqrt(delta) {
            let inverse = power(mul(2, a), P - 2);
            vec![mul(sub(root, b), inverse), mul(sub(sub(0, root), b), inverse)]
        } else { vec![] }
    } else if b != 0 {
        vec![mul(sub(0, c), power(b, P - 2))]
    } else if c == 0 {
        return Err("zero S3 polynomial: unresolved all-v exceptional branch".into());
    } else { vec![] };
    result.sort_unstable();
    result.dedup();
    if result.iter().any(|&v| eval(f, v) != 0) {
        return Err("computed S3 root does not satisfy polynomial".into());
    }
    Ok(result)
}

struct CoordinateIndex {
    // Algebraic roots of each S3(x_i,x_j,U); no curve additions or ordinates.
    support: BTreeMap<u64, Vec<(usize, usize)>>,
    root_multiplicity: usize,
    base_size: usize,
}
impl CoordinateIndex {
    fn new(xs: &[u64]) -> Result<Self, String> {
        let mut support: BTreeMap<u64, Vec<(usize,usize)>> = BTreeMap::new();
        let mut multiplicity = 0;
        for (i, &x) in xs.iter().enumerate() {
            for (j, &y) in xs.iter().enumerate().skip(i) {
                let rr = roots(s3(x, y))?;
                // The registered prime-order fixture has neither affine
                // two-torsion nor coincident x values within the base.
                if rr.len() != (if i == j { 1 } else { 2 }) {
                    return Err(format!("unexpected S3 pair-fiber arity at {i},{j}: {}", rr.len()));
                }
                for u in rr {
                    support.entry(u).or_default().push((i,j));
                    multiplicity += 1;
                }
            }
        }
        Ok(Self { support, root_multiplicity:multiplicity, base_size:xs.len() })
    }

    // An exact algebraic x-support intersection, including finite pair
    // intermediates and explicit identity-pair padding for two-sum targets.
    // This is NOT a black-box subgeneric index-calculus oracle: it enumerates
    // the fully split pair-sum x-support, which is generic-MITM-equivalent.
    fn tuples(&self, target_x: u64) -> Result<Vec<[usize;4]>, String> {
        let mut out = BTreeSet::<[usize;4]>::new();
        for (&u, left) in &self.support {
            for v in roots(s3(u, target_x))? {
                if let Some(right) = self.support.get(&v) {
                    for &(i, j) in left {
                        for &(k, l) in right {
                            let mut ids = [i,j,k,l];
                            ids.sort_unstable();
                            out.insert(ids);
                        }
                    }
                }
            }
        }
        // An intermediate P+(-P)=O has no affine x and hence no S3 root.
        // If R already has a two-point representation, append a cancelling
        // factor-base pair; this is a degenerate, non-new useful relation.
        if let Some(two) = self.support.get(&target_x) {
            for &(i,j) in two {
                for k in 0..self.base_size {
                    let mut ids = [i,j,k,k];
                    ids.sort_unstable();
                    out.insert(ids);
                }
            }
        }
        Ok(out.into_iter().collect())
    }
}

// Full y and group law ONLY after algebraic x-coordinate discovery.
fn validate(base: &Base, target: (u64,u64), tuples: &[[usize;4]],
            ar: &mut Arithmetic) -> Vec<Vec<(usize,i8)>>
{
    let mut rows = BTreeSet::new();
    for &ids in tuples {
        for mask in 0..16u8 {
            let mut point:Point = None;
            let mut row = Vec::with_capacity(4);
            for (position, &idx) in ids.iter().enumerate() {
                let sign = if (mask & (1 << position)) == 0 {1i8} else {-1i8};
                let p = Some(base.points[idx]);
                let p = if sign == 1 {p} else {ar.neg(PRIME31,p)};
                point = ar.add(PRIME31, point, p);
                row.push((idx, sign));
            }
            if point == Some(target) {
                row.sort_unstable();
                rows.insert(row);
            }
        }
    }
    rows.into_iter().collect()
}

fn splitmix64_next(state: &mut u64) -> u64 {
    *state = state.wrapping_add(0x9e3779b97f4a7c15);
    let mut z = *state;
    z = (z ^ (z >> 30)).wrapping_mul(0xbf58476d1ce4e5b9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94d049bb133111eb);
    z ^ (z >> 31)
}
fn nmul(x:u64,y:u64)->u64{((x as u128 * y as u128) % N as u128) as u64}
fn ninv(mut a:u64)->u64 {
    let mut e=N-2;
    let mut r=1;
    while e>0 {
        if e&1!=0 {r=nmul(r,a);}
        a=nmul(a,a);
        e >>= 1;
    }
    r
}
fn coefficient_rank(rows: &[Vec<(usize,i8)>], width:usize) -> usize {
    let mut basis:Vec<Option<Vec<u64>>> = vec![None;width];
    let mut rank=0;
    for row in rows {
        let mut v=vec![0u64;width];
        for &(j,s) in row {
            v[j]=(v[j]+ if s>0 {1} else {N-1})%N;
        }
        for col in 0..width {
            if v[col]==0{continue;}
            if let Some(p)=&basis[col] {
                let coef=v[col];
                for j in col..width {
                    v[j]=(v[j]+N-nmul(coef,p[j]))%N;
                }
            } else {
                let inv=ninv(v[col]);
                for value in &mut v[col..] {*value=nmul(*value,inv);}
                basis[col]=Some(v);
                rank+=1;
                break;
            }
        }
    }
    rank
}
fn natural(base:&Base,index:&CoordinateIndex,ar:&mut Arithmetic)->Result<(),String>{
    let mut state=SEED;
    let mut seen=BTreeSet::new();
    let mut fingerprint=0xcbf29ce484222325u64;
    let mut times=Vec::with_capacity(TRIALS);
    let mut generation_ns=0u128;
    let mut validation_ns=0u128;
    let mut generation_adds=0u64;
    let mut validation_adds=0u64;
    let mut tuples_positive=0usize;
    let mut signed_positive=0usize;
    let mut prefix_positive=0usize;
    let mut all_rows=Vec::new();
    let mut outcomes=Vec::new();
    for sample in 0..TRIALS {
        let scalar=loop {
            let next=1+splitmix64_next(&mut state)%(N-1);
            if seen.insert(next) {break next;}
        };
        fingerprint=(fingerprint^scalar).wrapping_mul(0x100000001b3);
        let adds=ar.counts.group_additions;
        let timer=Instant::now();
        let target=ar.scalar(PRIME31,scalar,Some(PRIME31.g))
            .ok_or("nonzero natural scalar yielded infinity")?;
        generation_ns+=timer.elapsed().as_nanos();
        generation_adds+=ar.counts.group_additions-adds;

        let timer=Instant::now();
        let tuples=index.tuples(target.0)?;
        times.push(timer.elapsed().as_nanos());
        if tuples.is_empty(){continue;}
        tuples_positive+=1;
        if sample<4096 {prefix_positive+=1;}
        let adds=ar.counts.group_additions;
        let timer=Instant::now();
        let rows=validate(base,target,&tuples,ar);
        validation_ns+=timer.elapsed().as_nanos();
        validation_adds+=ar.counts.group_additions-adds;
        if !rows.is_empty(){
            signed_positive+=1;
            all_rows.extend(rows.clone());
        }
        outcomes.push(json!({
            "sample":sample,"target":target,"fixture_scalar_after_discovery":scalar,
            "tuples":tuples.len(),"signed_rows":rows
        }));
    }
    // Comparison against exact source CI #43769263626: first 4096 same
    // frozen scalar population. A disagreement is a correctness bug.
    if prefix_positive != 0 {
        return Err(format!("root-index S3 support disagrees with completed 4096-query factorized S4 negative panel: {prefix_positive}"));
    }
    times.sort_unstable();
    println!("{}",json!({
        "standing":"[pcc:unverified]",
        "study":"Rust-only S3 coordinate-root-index algebraic enumeration CONTROL",
        "generic_equivalence_warning":"precomputed split S3 pair support is meet-in-middle-equivalent, NOT evidence of a novel non-generic ECDLP solver",
        "curve_p":P,"curve_n":N,"factor_base_size":index.base_size,
        "support_roots_including_multiplicity":index.root_multiplicity,
        "unique_support_x":index.support.len(),
        "seed_hex":"525553545f4d3401",
        "samples":TRIALS,"prefix_4096_nonconstant_targets":prefix_positive,
        "sample_fingerprint_fnv64":format!("{fingerprint:016x}"),
        "distinct_nonzero_public_targets":seen.len(),
        "target_generation_ns":generation_ns,"target_generation_additions":generation_adds,
        "total_coordinate_query_ns":times.iter().sum::<u128>(),
        "coordinate_query_median_ns":times[(TRIALS-1)/2],
        "coordinate_query_p95_ns":times[(TRIALS-1)*95/100],
        "algebraic_nonempty_targets":tuples_positive,
        "signed_verified_positive_targets":signed_positive,
        "signed_rows":all_rows.len(),
        "signed_coefficient_rank_mod_n":coefficient_rank(&all_rows,index.base_size),
        "signed_verification_ns":validation_ns,"signed_verification_additions":validation_adds,
        "natural_positive_cases":outcomes,
        "complete_ecdLP_solved":false,
        "matched_rho":false
    }));
    Ok(())
}
fn main()->Result<(),String>{
    let mut ar=Arithmetic::default();
    check_curve(PRIME31,&mut ar)?;
    let base=Base::build(PRIME31,6,&mut ar)?;
    if base.points.len()!=31{return Err("registered B31 base changed".into());}
    let xs:Vec<u64>=base.points.iter().map(|q|q.0).collect();
    let started=Instant::now();
    let index=CoordinateIndex::new(&xs)?;
    let index_build_ns=started.elapsed().as_nanos();
    if index.root_multiplicity!=31*31||index.support.len()!=31*31{
        return Err("B31 pair-support squarefreeness changed".into());
    }
    let positive=ar.add(PRIME31,Some(base.points[0]),Some(base.points[1]));
    let positive=ar.add(PRIME31,positive,Some(base.points[2]));
    let positive=ar.add(PRIME31,positive,Some(base.points[3]))
        .ok_or("unexpected planted identity")?;
    let tuples=index.tuples(positive.0)?;
    let rows=validate(&base,positive,&tuples,&mut ar);
    if rows.is_empty(){return Err("planted four-term row not recovered".into());}
    println!("{}",json!({"standing":"[pcc:unverified]",
        "study":"S3 coordinate pair-root index construction",
        "B":xs.len(),"pair_support_degree":index.root_multiplicity,
        "distinct_support_x":index.support.len(),"build_ns":index_build_ns,
        "planted_candidate_tuples":tuples.len(),"planted_verified_rows":rows.len(),
        "no_ordinates_or_group_additions_in_discovery":true,
        "generic_equivalent_not_novel":true}));
    if std::env::args().any(|x|x=="--natural-16384"){natural(&base,&index,&mut ar)?;}
    Ok(())
}
#[cfg(test)]
mod tests{
    use super::*;
    #[test]
    fn s3_quadratic_roots_and_complete_signed_b4_targets(){
        let mut ar=Arithmetic::default();
        check_curve(PRIME31,&mut ar).unwrap();
        let base=Base::build(PRIME31,6,&mut ar).unwrap();
        let xs:Vec<u64>=base.points.iter().take(4).map(|p|p.0).collect();
        let index=CoordinateIndex::new(&xs).unwrap();
        assert_eq!(index.root_multiplicity,16);
        assert_eq!(index.support.len(),16);
        let mut expected=BTreeSet::new();
        // Complete fixed-arity signed four-point image including cancellations;
        // this generic GROUP enumeration is TEST-ONLY, never an oracle input.
        for a in 0..8usize {
            for b in 0..8usize {
                for c in 0..8usize {
                    for d in 0..8usize {
                        let mut r:Point=None;
                        for h in [a,b,c,d] {
                            let point=Some(base.points[h/2]);
                            let point=if h&1==1 {ar.neg(PRIME31,point)} else {point};
                            r=ar.add(PRIME31,r,point);
                        }
                        if let Some(q)=r {expected.insert(q);}
                    }
                }
            }
        }
        assert_eq!(expected.len(),224); // total including O = 225.
        for q in expected {
            let tuples=index.tuples(q.0).unwrap();
            let rows=validate(&base,q,&tuples,&mut ar);
            assert!(!rows.is_empty(),"four-sum target {q:?} missed");
        }
    }
    #[test]
    fn b31_index_matches_registered_squarefree_support_and_planted(){
        let mut ar=Arithmetic::default();
        let base=Base::build(PRIME31,6,&mut ar).unwrap();
        let xs:Vec<u64>=base.points.iter().map(|p|p.0).collect();
        let index=CoordinateIndex::new(&xs).unwrap();
        assert_eq!(index.root_multiplicity,961);
        assert_eq!(index.support.len(),961);
        let target=ar.add(PRIME31,Some(base.points[0]),Some(base.points[1]));
        let target=ar.add(PRIME31,target,Some(base.points[2]));
        let target=ar.add(PRIME31,target,Some(base.points[3])).unwrap();
        assert!(!validate(&base,target,&index.tuples(target.0).unwrap(),&mut ar).is_empty());
    }
}
