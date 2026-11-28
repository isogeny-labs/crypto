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

const UNKNOWNS: usize = 32;
#[derive(Default)]
struct System32 {
    pivots: Vec<Option<(Vec<u64>, u64)>>,
    rank: usize,
    dependent: usize,
    equations: usize,
}
// Matrix over the actual prime subgroup order n. The final unknown is log_G(Q).
impl System32 {
    fn new() -> Self {
        Self { pivots:vec![None;UNKNOWNS], rank:0, dependent:0, equations:0 }
    }
    fn include(&mut self,row:&[(usize,i8)],a:u64,b:u64)->Result<bool,String>{
        let mut v=vec![0u64;UNKNOWNS];
        for &(i,s) in row {
            if i>=UNKNOWNS-1 || (s != 1 && s != -1) {
                return Err("invalid factor-base coefficient".into());
            }
            v[i]=(v[i]+if s==1{1}else{N-1})%N;
        }
        v[UNKNOWNS-1]=if b==0{0}else{N-b};
        let mut rhs=a;
        self.equations+=1;
        for col in 0..UNKNOWNS {
            if v[col]==0{continue;}
            if let Some((pivot, pivot_rhs))=&self.pivots[col] {
                let factor=v[col];
                for j in col..UNKNOWNS {
                    v[j]=(v[j]+N-nmul(factor,pivot[j]))%N;
                }
                rhs=(rhs+N-nmul(factor,*pivot_rhs))%N;
            } else {
                let inv=ninv(v[col]);
                for vj in &mut v[col..] {*vj=nmul(*vj,inv);}
                rhs=nmul(rhs,inv);
                self.pivots[col]=Some((v,rhs));
                self.rank+=1;
                return Ok(true);
            }
        }
        if rhs != 0 {
            return Err("verified elliptic-curve relation inconsistent in mod-n linear algebra".into());
        }
        self.dependent+=1;
        Ok(false)
    }
    fn solve(&self)->Result<[u64;UNKNOWNS],String>{
        if self.rank != UNKNOWNS {return Err("incomplete coefficient rank".into());}
        let mut sol=[0u64;UNKNOWNS];
        for col in (0..UNKNOWNS).rev() {
            let (pivot,rhs)=self.pivots[col].as_ref()
                .ok_or("missing pivot column in full-rank matrix")?;
            let mut value=*rhs;
            for j in col+1..UNKNOWNS {
                value=(value+N-nmul(pivot[j],sol[j]))%N;
            }
            sol[col]=value;
        }
        Ok(sol)
    }
}
// Bounded, full relation-collection + modular solve test. This is bona-fide
// factor-base linear algebra, but the coordinate-root oracle is mathematically
// generic MITM-equivalent: not a new non-generic index calculus complexity.
fn cold_full_log(q:(u64,u64), cap:usize)->Result<Option<u64>,String>{
    let started=Instant::now();
    let mut ar=Arithmetic::default();
    check_curve(PRIME31,&mut ar)?;
    if !ar.on_curve(PRIME31,Some(q)){return Err("invalid external Q".into());}
    let base=Base::build(PRIME31,6,&mut ar)?;
    if base.points.len()!=31{return Err("wrong B31 base".into());}
    let index=CoordinateIndex::new(&base.points.iter().map(|p|p.0).collect::<Vec<_>>())?;
    if index.support.len()!=961 {return Err("root index support changed".into());}
    let cold_setup_ns=started.elapsed().as_nanos();
    let mut walk_seed=0x5354525543545f38u64;
    let mut steps=Vec::with_capacity(64);
    let step_started=Instant::now();
    for _ in 0..64 {
        let da=1+splitmix64_next(&mut walk_seed)%(N-1);
        let db=1+splitmix64_next(&mut walk_seed)%(N-1);
        let p=ar.scalar(PRIME31,da,Some(PRIME31.g));
        let r=ar.scalar(PRIME31,db,Some(q));
        let step=ar.add(PRIME31,p,r).ok_or("degenerate public-step point")?;
        steps.push((step,da,db));
    }
    let step_setup_ns=step_started.elapsed().as_nanos();
    let mut position:Point=Some(PRIME31.g);
    let mut alpha=1u64;
    let mut beta=0u64;
    let mut system=System32::new();
    let mut query_ns=0u128;
    let mut validation_ns=0u128;
    let mut LA_ns=0u128;
    let mut target_generation_ns=0u128;
    let mut nonempty=0usize;
    let mut verified_rows=0usize;
    let mut independent=0usize;
    let mut calls=0usize;
    let mut first_hit=None;
    let mut final_log=None;
    let mut replay_additions=0u64;
    let mut seen=BTreeSet::new();
    let mut duplicate_targets=0usize;
    'walk: for turn in 0..cap {
        let choice=(splitmix64_next(&mut walk_seed) as usize)&63;
        let (step,da,db)=steps[choice];
        let timer=Instant::now();
        position=ar.add(PRIME31,position,Some(step));
        alpha=(alpha+da)%N;
        beta=(beta+db)%N;
        target_generation_ns+=timer.elapsed().as_nanos();
        let Some(r)=position else {continue;};
        // Source-only x projection; can be equal for R and -R. Track point
        // encodings to exclude any repeated whole public target.
        if !seen.insert(r) {
            duplicate_targets+=1;
            continue;
        }
        calls+=1;
        let timer=Instant::now();
        let tuples=index.tuples(r.0)?;
        query_ns+=timer.elapsed().as_nanos();
        if tuples.is_empty(){continue;}
        nonempty+=1;
        if first_hit.is_none(){first_hit=Some(turn);}
        let before=ar.counts.group_additions;
        let timer=Instant::now();
        let rows=validate(&base,r,&tuples,&mut ar);
        validation_ns+=timer.elapsed().as_nanos();
        replay_additions+=ar.counts.group_additions-before;
        if rows.is_empty(){return Err("S3 coordinate support gave a false positive on prime-order curve".into());}
        for row in rows {
            verified_rows+=1;
            let timer=Instant::now();
            if system.include(&row,alpha,beta)?{independent+=1;}
            LA_ns+=timer.elapsed().as_nanos();
            if system.rank == UNKNOWNS {
                let sol=system.solve()?;
                // Verify every factor-base log and external target log
                // independently with reused Rust group arithmetic.
                for (i,&p) in base.points.iter().enumerate() {
                    if ar.scalar(PRIME31,sol[i],Some(PRIME31.g))!=Some(p){
                        return Err(format!("factor-base discrete log verification failed at {i}"));
                    }
                }
                if ar.scalar(PRIME31,sol[31],Some(PRIME31.g))!=Some(q){
                    return Err("external Q discrete log verification failed".into());
                }
                final_log=Some(sol[31]);
                // Stop the OUTER target-collection loop at full verified rank;
                // breaking only the inner row loop overcharged the cold cost.
                break 'walk;
            }
        }
    }
    println!("{}",json!({
        "standing":"[pcc:unverified]",
        "method":"Rust factor-base relations plus prime-modulus Gaussian elimination; S3 coordinate root-index discovery (generic-equivalent)",
        "new_non_generic_attack":false,
        "curve_p":P,"curve_n":N,"factor_base_size":31,
        "public_target":q,"target_fixture_scalar_never_passed_to_solver":true,
        "max_public_walk_targets":cap,"visited_distinct_targets":calls,
        "duplicate_public_targets":duplicate_targets,
        "first_nonempty_turn":first_hit,
        "algebraic_nonempty_targets":nonempty,
        "verified_relation_rows":verified_rows,
        "independent_equation_rows":independent,
        "coefficient_rank":system.rank,
        "linear_equations_processed":system.equations,
        "dependent_rows":system.dependent,
        "cold_setup_ns":cold_setup_ns,"step_setup_ns":step_setup_ns,
        "public_target_walk_ns":target_generation_ns,
        "algebraic_discovery_ns":query_ns,
        "post_discovery_verification_ns":validation_ns,
        "linear_algebra_ns":LA_ns,
        "relation_replay_additions":replay_additions,
        "full_cold_wall_ns":started.elapsed().as_nanos(),
        "verified_complete_log":final_log,
        "complete_ECDLP_recovery":final_log.is_some(),
        "matched_optimized_rho":false
    }));
    Ok(final_log)
}


// Honest generic cold-cost comparator on exactly the same public Q, reuse
// the native curve arithmetic. Plain BSGS is not optimized negation-map rho.
// Include curve certification, baby table, giant steps, independent replay,
// memory cardinality and group additions; never treat rho as measured here.
fn cold_bsgs_log(q: (u64,u64))->Result<u64,String>{
    use std::collections::HashMap;
    let cold=Instant::now();
    let mut ar=Arithmetic::default();
    check_curve(PRIME31,&mut ar)?;
    if !ar.on_curve(PRIME31,Some(q)){return Err("invalid public target for BSGS".into());}
    // ceil sqrt(2147446831) = 46341.
    let m=46_341u64;
    if (m-1)* (m-1) >= N || m*m<N {
        return Err("incorrect BSGS sqrt subgroup-order bound".into());
    }
    let mut table=HashMap::<Point,u64>::with_capacity(m as usize);
    let mut baby=None;
    let baby_started=Instant::now();
    for j in 0..m {
        table.entry(baby).or_insert(j);
        baby=ar.add(PRIME31,baby,Some(PRIME31.g));
    }
    let baby_ns=baby_started.elapsed().as_nanos();
    let giant_started=Instant::now();
    let step=ar.scalar(PRIME31,m,Some(PRIME31.g))
        .ok_or("BSGS mG unexpectedly infinity")?;
    let step_neg=ar.neg(PRIME31,Some(step));
    let mut cur=Some(q);
    let mut found=None;
    let mut checked=0u64;
    for i in 0..=m {
        checked+=1;
        if let Some(&j)=table.get(&cur) {
            let k=(i*m+j)%N;
            if ar.scalar(PRIME31,k,Some(PRIME31.g))==Some(q) {
                found=Some(k);
                break;
            }
        }
        cur=ar.add(PRIME31,cur,step_neg);
    }
    let giant_ns=giant_started.elapsed().as_nanos();
    let recovered=found.ok_or("BSGS failed complete prime-order log")?;
    println!("{}",json!({
        "standing":"[pcc:unverified]",
        "method":"plain Rust HashMap BSGS fully cold generic control",
        "not_optimized_negation_rho":true,
        "curve_p":P,"curve_n":N,
        "baby_step_table_entries":table.len(),"m":m,
        "giant_step_probes":checked,
        "baby_step_ns":baby_ns,"giant_step_and_replay_ns":giant_ns,
        "cold_total_wall_ns":cold.elapsed().as_nanos(),
        "group_additions_including_curve_check":ar.counts.group_additions,
        "verified_complete_log":recovered,
        "complete_ECDLP_recovery":true
    }));
    Ok(recovered)
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
    if std::env::args().any(|x|x=="--bsgs-one"){
        const FIXTURE_K:u64=1_038_579_171;
        let public_q=ar.scalar(PRIME31,FIXTURE_K,Some(PRIME31.g))
            .ok_or("BSGS known-answer fixture unexpectedly infinity")?;
        let recovered=cold_bsgs_log(public_q)?;
        if recovered!=FIXTURE_K {return Err("BSGS public target known-answer mismatch".into());}
    }
    if std::env::args().any(|x|x=="--solve-one"){
        // Public synthetic fixture only. The solver receives q, never k.
        const FIXTURE_K:u64=1_038_579_171;
        let public_q=ar.scalar(PRIME31,FIXTURE_K,Some(PRIME31.g))
            .ok_or("known-answer fixture unexpectedly infinity")?;
        let recovered=cold_full_log(public_q,250_000)?;
        if recovered.is_some() && recovered!=Some(FIXTURE_K){
            return Err("held-out known-answer fixture mismatch".into());
        }
        println!("{}",json!({"posthoc_fixture_matches":recovered==Some(FIXTURE_K),
                              "fixture_used_for_relation_oracle":false,
                              "solver_target":"public point only"}));
    }
    Ok(())
}
#[cfg(test)]
mod tests{
    use super::*;
    #[test]
    fn mod_n_full_rank_recovers_factor_base_and_target_unknowns(){
        let mut system=System32::new();
        for i in 0..31 {
            assert!(system.include(&[(i,1)],(i+1) as u64,0).unwrap());
        }
        const HIDDEN: u64 = 1_038_579_171;
        assert!(system.include(&[(0,1)],(1+N-HIDDEN)%N,1).unwrap());
        assert_eq!(system.rank,32);
        let values=system.solve().unwrap();
        for (i,&v) in values[..31].iter().enumerate() {
            assert_eq!(v,(i+1) as u64);
        }
        assert_eq!(values[31],HIDDEN);
    }
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
