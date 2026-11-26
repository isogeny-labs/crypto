//! EXP-R31-APS: full single-relation labelled-base ECDLP on ordinary prime31.
//!
//! Published APS (Amadori–Pintore–Sala, FFA 2018, section 3) methodology:
//! sample public labelled R_i = a_i G + b_i Q, then find ONE signed
//! S3(x_i,x_j,x_k)=0 relation. No factor-base logarithms or linear algebra.
//!
//! THIS SPECIALIZED m=3 IMPLEMENTATION IS GENERIC MEET-IN-THE-MIDDLE EQUIVALENT:
//! evaluating every pair S3 quadratic is not a novel non-generic attack.
//! The algebraic arm uses only Fp operations to discover candidates; EC point
//! addition and the scalar replay are confined to candidate verification.
//! A separate direct group-join arm is a methodology/cost control, not fallback.
//! No fee-based compute or source-independent scientific validation assumed.
//!
//! cargo +1.100.0 test --locked --release --manifest-path research/prime31_ic_20261115/native/Cargo.toml --example prime31_aps_one_relation
//! cargo +1.100.0 run --locked --release --manifest-path research/prime31_ic_20261115/native/Cargo.toml --example prime31_aps_one_relation -- --one
//! cargo +1.100.0 run --locked --release --manifest-path research/prime31_ic_20261115/native/Cargo.toml --example prime31_aps_one_relation -- --compare
use prime31_algebraic_ic::{check_curve, Arithmetic, Point, PRIME31};
use serde_json::json;
use std::collections::{HashMap, HashSet};
use std::time::Instant;

const P: u64 = 2_147_483_647;
const N: u64 = 2_147_446_831;
const T: usize = 1_536;
const ROUNDS: usize = 3;
// Identical caller-public Q used by the earlier complete Rust S3-index gate.
const PUBLIC_Q: (u64,u64) = (1_477_799_919, 824_532_935);
const SEED: u64 = 0x4150_535f_5052_3331;

#[derive(Clone, Copy)]
struct Labelled {
    pt: (u64,u64),
    a: u64,
    b: u64,
}
#[derive(Default, Clone)]
struct Stats {
    pair_s3: u64,
    s3_sqrt: u64,
    batch_inversions: u64,
    matched_x: u64,
    signed_candidates: u64,
    unusable_beta: u64,
}
struct Solution {
    log: u64,
    indices: [usize; 3],
    signs: [i8; 3],
    alpha: u64,
    beta: u64,
}
fn addp(x:u64,y:u64)->u64 {let t=x+y;if t>=P{t-P}else{t}}
fn subp(x:u64,y:u64)->u64 {if x>=y{x-y}else{x+P-y}}
fn mulp(x:u64,y:u64)->u64 {((x as u128*y as u128)%P as u128) as u64}
fn pown(mut x:u64,mut k:u64,m:u64)->u64 {
    let mut a=1u64;
    while k!=0 {
        if k&1!=0 {a=((a as u128*x as u128)%m as u128) as u64;}
        x=((x as u128*x as u128)%m as u128) as u64;
        k>>=1;
    }
    a
}
fn sqrtp(x:u64)->Option<u64> {
    let r=pown(x,(P+1)/4,P);
    (mulp(r,r)==x).then_some(r)
}
fn addn(x:u64,y:u64)->u64 {let t=x+y;if t>=N{t-N}else{t}}
fn subn(x:u64,y:u64)->u64 {if x>=y{x-y}else{x+N-y}}
fn muln(x:u64,y:u64)->u64 {((x as u128*y as u128)%N as u128) as u64}
fn prng(state:&mut u64)->u64 {
    *state=state.wrapping_add(0x9e37_79b9_7f4a_7c15);
    let mut z=*state;
    z=(z^(z>>30)).wrapping_mul(0xbf58_476d_1ce4_e5b9);
    z=(z^(z>>27)).wrapping_mul(0x94d0_49bb_1331_11eb);
    z^(z>>31)
}
fn source(q:(u64,u64),seed:u64,ar:&mut Arithmetic)
    ->Result<([Vec<Labelled>;3],u64,u64),String> {
    let mut rng=seed;
    let mut out:[Vec<Labelled>;3]=std::array::from_fn(|_|Vec::with_capacity(T));
    let mut xs=HashSet::with_capacity(3*T);
    let mut identity=0_u64;
    let mut duplicated=0_u64;
    for part in &mut out {
        while part.len()<T {
            let a=prng(&mut rng)%N;
            let b=prng(&mut rng)%N;
            if a==0&&b==0 {continue;}
            let ag=ar.scalar(PRIME31,a,Some(PRIME31.g));
            let bq=ar.scalar(PRIME31,b,Some(q));
            let Some(pt)=ar.add(PRIME31,ag,bq) else {
                identity+=1;continue;
            };
            if !xs.insert(pt.0) {duplicated+=1;continue;}
            part.push(Labelled{pt,a,b});
        }
    }
    Ok((out,identity,duplicated))
}
// S3(X,Y,U)= a U^2 + b U + c on the original prime31 curve.
fn s3_coeff(x:u64,y:u64)->(u64,u64,u64) {
    let sum=addp(x,y);
    let xy=mulp(x,y);
    let a=mulp(subp(x,y),subp(x,y));
    let b=subp(0,mulp(2,addp(mulp(sum,addp(xy,PRIME31.a)),mulp(2,PRIME31.b))));
    let c=subp(mulp(subp(xy,PRIME31.a),subp(xy,PRIME31.a)),
               mulp(mulp(4,PRIME31.b),sum));
    (a,b,c)
}
fn s3_eval(coeff:(u64,u64,u64),z:u64)->u64 {
    let (a,b,c)=coeff;addp(addp(mulp(a,mulp(z,z)),mulp(b,z)),c)
}
fn signed(ar:&Arithmetic,pt:(u64,u64),s:i8)->Point {
    if s==1 {Some(pt)} else {ar.neg(PRIME31,Some(pt))}
}
// Only called AFTER an algebraic x-match (or during the explicitly
// separate group-join control). It verifies true signed group identity,
// nonzero target coefficient and the COMPLETE recovered target scalar.
fn verify_candidate(
    a:&Labelled,b:&Labelled,c:&Labelled,
    indices:[usize;3],q:(u64,u64),ar:&mut Arithmetic,stats:&mut Stats
)->Option<Solution> {
    stats.signed_candidates+=1;
    for sa in [-1i8,1] {
        for sb in [-1i8,1] {
            for sc in [-1i8,1] {
                let pa=signed(ar,a.pt,sa);
                let pb=signed(ar,b.pt,sb);
                let pc=signed(ar,c.pt,sc);
                let v=ar.add(PRIME31,pa,pb);
                if ar.add(PRIME31,v,pc).is_some() {continue;}
                let sum_coeff=|x:u64,y:u64,s:i8| {
                    addn(x,if s==1{y}else{subn(0,y)})
                };
                let alpha=sum_coeff(sum_coeff(sum_coeff(0,a.a,sa),b.a,sb),c.a,sc);
                let beta=sum_coeff(sum_coeff(sum_coeff(0,a.b,sa),b.b,sb),c.b,sc);
                if beta==0 {stats.unusable_beta+=1;continue;}
                let log=muln(subn(0,alpha),pown(beta,N-2,N));
                if ar.scalar(PRIME31,log,Some(PRIME31.g))==Some(q) {
                    return Some(Solution{
                        log,indices,signs:[sa,sb,sc],alpha,beta
                    });
                }
            }
        }
    }
    None
}
fn reverse_index(f:&[Labelled])->HashMap<u64,Vec<usize>> {
    let mut out=HashMap::with_capacity(f.len());
    for (i,p) in f.iter().enumerate() {
        out.entry(p.pt.0).or_insert_with(Vec::new).push(i);
    }
    out
}
/// Algebraic discovery: for each first pair, compute BOTH finite roots of S3
/// using a single row batch inversion; compare with public third-list x's.
/// No elliptic additions or known ordinates enter this discovery stage.
fn algebraic(
    parts:&[Vec<Labelled>;3],q:(u64,u64),ar:&mut Arithmetic
)->Result<(Option<Solution>,Stats),String> {
    let lookup=reverse_index(&parts[2]);
    let mut stats=Stats::default();
    for (i,p) in parts[0].iter().enumerate() {
        let mut coefficients=Vec::with_capacity(T);
        let mut sqrts=Vec::with_capacity(T);
        let mut prefixes=Vec::with_capacity(T+1);
        let mut denominators=Vec::with_capacity(T);
        prefixes.push(1u64);
        for other in &parts[1] {
            let f=s3_coeff(p.pt.0,other.pt.0);
            let (a,b,c)=f;
            let (denom,sqr) = if a!=0 {
                let disc=subp(mulp(b,b),mulp(mulp(4,a),c));
                stats.s3_sqrt+=1;
                let root=sqrtp(disc).ok_or("S3 Fp discriminant nonsquare on rational points")?;
                (mulp(2,a),Some(root))
            } else if b!=0 { (b,None) }
            else {return Err("zero S3 polynomial; cannot discard exceptional branch".into());};
            stats.pair_s3+=1;
            prefixes.push(mulp(*prefixes.last().ok_or("prefix empty")?,denom));
            denominators.push(denom);
            coefficients.push(f);
            sqrts.push(sqr);
        }
        let last=*prefixes.last().ok_or("missing batch prefix")?;
        if last==0 {return Err("zero denominator in rational S3 batch".into());}
        let mut prod_inv=pown(last,P-2,P);
        stats.batch_inversions+=1;
        let mut inverses=vec![0u64;parts[1].len()];
        for j in (0..parts[1].len()).rev() {
            inverses[j]=mulp(prod_inv,prefixes[j]);
            prod_inv=mulp(prod_inv,denominators[j]);
        }
        if prod_inv!=1 {return Err("batch inverse invariant failed".into());}
        for (j,f) in coefficients.into_iter().enumerate() {
            let (a,b,c)=f;
            let mut roots=Vec::with_capacity(2);
            if let Some(root)=sqrts[j] {
                roots.push(mulp(subp(root,b),inverses[j]));
                roots.push(mulp(subp(subp(0,root),b),inverses[j]));
            } else {
                roots.push(mulp(subp(0,c),inverses[j]));
            }
            roots.sort_unstable();roots.dedup();
            for z in roots {
                if s3_eval(f,z)!=0 {return Err("incorrect S3 root extraction".into());}
                if let Some(candidates)=lookup.get(&z) {
                    for &k in candidates {
                        stats.matched_x+=1;
                        if let Some(solution)=verify_candidate(
                            p,&parts[1][j],&parts[2][k],[i,j,k],q,ar,&mut stats
                        ) {return Ok((Some(solution),stats));}
                    }
                }
            }
        }
    }
    Ok((None,stats))
}
// Exact same admitted public labelled sets, but group sums generate the x
// candidates. This is a SEPARATE generic baseline; it never feeds algebraic.
fn direct_group(
    parts:&[Vec<Labelled>;3],q:(u64,u64),ar:&mut Arithmetic
)->(Option<Solution>,Stats) {
    let lookup=reverse_index(&parts[2]);
    let mut stats=Stats::default();
    for (i,p) in parts[0].iter().enumerate() {
        for (j,r) in parts[1].iter().enumerate() {
            stats.pair_s3+=1; // same number of visited pairs, not Fp S3 calls
            for other in [Some(r.pt),ar.neg(PRIME31,Some(r.pt))] {
                let Some(v)=ar.add(PRIME31,Some(p.pt),other) else {continue;};
                if let Some(indices)=lookup.get(&v.0) {
                    for &k in indices {
                        stats.matched_x+=1;
                        if let Some(solution)=verify_candidate(
                            p,r,&parts[2][k],[i,j,k],q,ar,&mut stats
                        ) {return (Some(solution),stats);}
                    }
                }
            }
        }
    }
    (None,stats)
}
fn json_solution(s:&Option<Solution>)->serde_json::Value {
    if let Some(v)=s {
        json!({"status":"verified","log":v.log,"partition_indices":v.indices,
             "signs":v.signs,"alpha":v.alpha,"beta":v.beta,
             "identity_equation":"alpha + beta * log = 0 (mod n)"})
    } else {json!({"status":"no_relation_in_bounded_sample"})}
}
fn execute(compare:bool,round_limit:usize,q:(u64,u64))
    ->Result<serde_json::Value,String> {
    let cold=Instant::now();
    let mut arithmetic=Arithmetic::default();
    check_curve(PRIME31,&mut arithmetic)?;
    if !arithmetic.on_curve(PRIME31,Some(q))||q==PRIME31.g {
        return Err("public Q invalid or trivial".into());
    }
    let mut attempts=Vec::new();
    for round in 0..round_limit {
        let seed=SEED.wrapping_add((round as u64).wrapping_mul(0xd1b5_4a32_d192_ed03));
        let source_start=Instant::now();
        let (parts,identities,duplicate_x)=source(q,seed,&mut arithmetic)?;
        let source_ns=source_start.elapsed().as_nanos();
        let alg_start=Instant::now();
        let additions_before=arithmetic.counts.group_additions;
        let (solution,stats)=algebraic(&parts,q,&mut arithmetic)?;
        let alg_ns=alg_start.elapsed().as_nanos();
        let algebraic_validation_additions=
            arithmetic.counts.group_additions-additions_before;
        let cold_at_solution_ns=cold.elapsed().as_nanos();
        let mut generic=serde_json::Value::Null;
        if compare {
            let mut control=Arithmetic::default();
            let control_start=Instant::now();
            let (control_solution,control_stats)=direct_group(&parts,q,&mut control);
            let control_ns=control_start.elapsed().as_nanos();
            if control_solution.as_ref().map(|s|s.log)!=solution.as_ref().map(|s|s.log)
               || control_solution.is_some()!=solution.is_some() {
                return Err("algebraic and direct-group controls disagree on recovery".into());
            }
            generic=json!({
                "separate_generic_control":true,
                "same_public_labelled_sources":true,
                "group_lookup_ns":control_ns,
                "source_plus_generic_control_ns":source_ns+control_ns,
                "visited_pairs":control_stats.pair_s3,
                "matching_x":control_stats.matched_x,
                "signed_candidates":control_stats.signed_candidates,
                "group_additions":control.counts.group_additions,
                "solution":json_solution(&control_solution),
                "warning":"development same-dataset sequential A/B, no isolation or A/A calibration"
            });
        }
        let success=solution.is_some();
        attempts.push(json!({
            "round":round,"public_seed":format!("{seed:016x}"),
            "points_per_partition":T,"precompute_labels_ns":source_ns,
            "rejected_identity":identities,"rejected_global_duplicate_x":duplicate_x,
            "algebraic_S3_pair_roots_ns":alg_ns,"algebraic_visited_pairs":stats.pair_s3,
            "algebraic_field_square_roots":stats.s3_sqrt,
            "algebraic_batch_field_inversions":stats.batch_inversions,
            "algebraic_matching_x_candidates":stats.matched_x,
            "algebraic_signed_candidates":stats.signed_candidates,
            "algebraic_unusable_beta":stats.unusable_beta,
            "ec_group_additions_candidate_verification_only":algebraic_validation_additions,
            "algebraic_cold_wall_until_output_ns":cold_at_solution_ns,
            "solution":json_solution(&solution),
            "generic_direct_join_control":generic
        }));
        if success {
            return Ok(json!({
                "standing":"[pcc:unverified]",
                "method":"published APS 2018 dynamic labelled one-relation S3, m=3",
                "classification":"index-calculus methodology, generic pair-MITM equivalent, not a novel non-generic attack",
                "p":P,"n":N,"curve_a":PRIME31.a,"curve_b":PRIME31.b,
                "cofactor":1,"generator":PRIME31.g,"public_target":q,
                "complete_target_scalar_recovered":true,
                "no_known_target_log_in_solver":true,
                "no_f4_or_factor_base_linear_algebra":true,
                "cold_rust_wall_ns":cold_at_solution_ns,
                "attempts":attempts
            }));
        }
    }
    Ok(json!({
        "standing":"[pcc:unverified]",
        "method":"published APS 2018 one relation",
        "p":P,"n":N,"public_target":q,
        "complete_target_scalar_recovered":false,
        "status":"bounded_exhaustion_not_a_negative_algebraic_ideal_certificate",
        "cold_rust_wall_ns":cold.elapsed().as_nanos(),
        "attempts":attempts
    }))
}
fn main()->Result<(),String> {
    let args:Vec<String>=std::env::args().skip(1).collect();
    let compare=match args.as_slice() {
        [] => false,
        [a] if a=="--one" => false,
        [a] if a=="--compare" => true,
        _ => return Err("usage: prime31_aps_one_relation --one|--compare".into())
    };
    let v=execute(compare,ROUNDS,PUBLIC_Q)?;
    println!("{}",serde_json::to_string_pretty(&v).map_err(|e|e.to_string())?);
    if v["complete_target_scalar_recovered"].as_bool()!=Some(true) {
        return Err("published APS m3 bounded sample did not find a valid relation".into());
    }
    Ok(())
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn s3_field_roots_match_ec_semantics() {
        let mut ar=Arithmetic::default();
        check_curve(PRIME31,&mut ar).unwrap();
        let p=PRIME31.g;
        let r=ar.scalar(PRIME31,7,Some(p)).unwrap();
        let f=s3_coeff(p.0,r.0);
        let (a,b,c)=f;
        assert_ne!(a,0);
        let d=subp(mulp(b,b),mulp(mulp(4,a),c));
        let root=sqrtp(d).unwrap();
        let inverse=pown(mulp(2,a),P-2,P);
        let mut xs=[mulp(subp(root,b),inverse),
                    mulp(subp(subp(0,root),b),inverse)];
        xs.sort_unstable();
        let v=ar.add(PRIME31,Some(p),Some(r)).unwrap();
        let w=ar.add(PRIME31,Some(p),ar.neg(PRIME31,Some(r))).unwrap();
        let mut expected=[v.0,w.0];
        expected.sort_unstable();
        assert_eq!(xs,expected);
        assert!(xs.iter().all(|&x|s3_eval(f,x)==0));
    }
    #[test]
    fn one_relation_congruence() {
        let aa=7u64;let bb=43u64;let k=12345u64;
        let alpha=subn(0,muln(bb,k));
        assert_eq!(addn(alpha,muln(bb,k)),0);
        assert_eq!(muln(subn(0,alpha),pown(bb,N-2,N)),k);
        assert!(aa<N);
    }
}
