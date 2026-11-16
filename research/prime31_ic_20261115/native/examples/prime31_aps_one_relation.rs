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
fn source_sized(q:(u64,u64),seed:u64,ar:&mut Arithmetic,count:usize)
    ->Result<([Vec<Labelled>;3],u64,u64),String> {
    let mut rng=seed;
    let mut out:[Vec<Labelled>;3]=std::array::from_fn(|_|Vec::with_capacity(count));
    let mut xs=HashSet::with_capacity(3*count);
    let mut identity=0_u64;
    let mut duplicated=0_u64;
    for part in &mut out {
        while part.len()<count {
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

// Experimental faster cold source for the same *published* APS algebraic
// one-relation oracle: independent seed/step labels then a public additive
// random walk. The finite 64-step table is built and charged for EACH round,
// rather than shared across targets. No target log, group join, or hidden
// scalar enters the algebraic search. Source points are correlated, so its
// observed success probability must be measured, not assumed.
fn source_walk_sized(q:(u64,u64),seed:u64,ar:&mut Arithmetic,count:usize)
    ->Result<([Vec<Labelled>;3],u64,u64),String>{
    const STEPS:usize=64;
    let mut rng=seed^0x5741_4c4b_5f53_5445_u64;
    let mut steps=Vec::with_capacity(STEPS);
    for _ in 0..STEPS {
        let jump=loop {
            let a=prng(&mut rng)%N;
            let b=prng(&mut rng)%N;
            if a==0 && b==0 {continue;}
            let ag=ar.scalar(PRIME31,a,Some(PRIME31.g));
            let bq=ar.scalar(PRIME31,b,Some(q));
            if let Some(pt)=ar.add(PRIME31,ag,bq) {
                break Labelled{pt,a,b};
            }
        };
        steps.push(jump);
    }
    let mut out:[Vec<Labelled>;3]=std::array::from_fn(|_|Vec::with_capacity(count));
    let mut xs=HashSet::with_capacity(3*count);
    let mut identity=0u64;
    let mut duplicated=0u64;
    for part in &mut out {
        let mut cur=loop {
            let a=prng(&mut rng)%N;
            let b=prng(&mut rng)%N;
            if a==0 && b==0 {continue;}
            let ag=ar.scalar(PRIME31,a,Some(PRIME31.g));
            let bq=ar.scalar(PRIME31,b,Some(q));
            if let Some(pt)=ar.add(PRIME31,ag,bq) {
                break Labelled{pt,a,b};
            }
            identity+=1;
        };
        let mut transitions=0usize;
        while part.len()<count {
            transitions+=1;
            if transitions>count.saturating_mul(96).saturating_add(2_048) {
                return Err("walk failed to produce enough distinct public abscissae".into());
            }
            if xs.insert(cur.pt.0) {
                part.push(cur);
            } else {
                duplicated+=1;
            }
            if part.len()==count {break;}
            let s=steps[(prng(&mut rng) as usize)&(STEPS-1)];
            let a=addn(cur.a,s.a);
            let b=addn(cur.b,s.b);
            if let Some(pt)=ar.add(PRIME31,Some(cur.pt),Some(s.pt)) {
                cur=Labelled{pt,a,b};
            } else {
                // No affine abscissa for the identity: count and reseed using
                // public scalars, preserving correct labels in all cases.
                identity+=1;
                cur=loop {
                    let a=prng(&mut rng)%N;
                    let b=prng(&mut rng)%N;
                    if a==0 && b==0 {continue;}
                    let ag=ar.scalar(PRIME31,a,Some(PRIME31.g));
                    let bq=ar.scalar(PRIME31,b,Some(q));
                    if let Some(pt)=ar.add(PRIME31,ag,bq) {
                        break Labelled{pt,a,b};
                    }
                    identity+=1;
                };
            }
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
        let mut coefficients=Vec::with_capacity(parts[1].len());
        let mut sqrts=Vec::with_capacity(parts[1].len());
        let mut prefixes=Vec::with_capacity(parts[1].len()+1);
        let mut denominators=Vec::with_capacity(parts[1].len());
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
            let (_a,b,c)=f;
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
fn execute_sized(compare:bool,round_limit:usize,q:(u64,u64),count:usize)
    ->Result<serde_json::Value,String> {
    execute_sized_mode(compare,round_limit,q,count,false)
}
fn execute_sized_mode(compare:bool,round_limit:usize,q:(u64,u64),count:usize,walk:bool)
    ->Result<serde_json::Value,String> {
    if count<2 || count>8_192 || round_limit==0 || round_limit>64 {
        return Err("inadmissible labelled-list count or trial cap".into());
    }
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
        let start_additions=arithmetic.counts.group_additions;
        let (parts,identities,duplicate_x)=if walk {
            source_walk_sized(q,seed,&mut arithmetic,count)?
        } else {
            source_sized(q,seed,&mut arithmetic,count)?
        };
        let source_additions=arithmetic.counts.group_additions-start_additions;
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
            "points_per_partition":count,"precompute_labels_ns":source_ns,
            "public_label_source":if walk {"public_additive_walk"} else {"independent_two_scalars"},
            "source_point_generation_group_additions":source_additions,
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
                "points_per_partition":count,"round_limit":round_limit,
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
        "points_per_partition":count,"round_limit":round_limit,
        "complete_target_scalar_recovered":false,
        "status":"bounded_exhaustion_not_a_negative_algebraic_ideal_certificate",
        "cold_rust_wall_ns":cold.elapsed().as_nanos(),
        "attempts":attempts
    }))
}

fn execute(compare:bool,round_limit:usize,q:(u64,u64))
    ->Result<serde_json::Value,String>{
    execute_sized(compare,round_limit,q,T)
}

// Development-only partition-size experiment against the same published
// APS one-relation control. Each arm rebuilds its labelled public points,
// charges every unsuccessful round, and verifies target scalars only outside
// execute_sized. Varying T is an engineering search over generic-MITM-equivalent
// arithmetic, never a non-generic complexity claim.
fn partition_size_sweep()->Result<serde_json::Value,String>{
    const SIZES:[(usize,usize);5]=[(768,12),(1024,8),(1536,3),(2048,3),(3072,2)];
    let mut rng=0x484f_4c44_4f55_5453_u64;
    let mut known=HashSet::new();
    let mut targets=Vec::new();
    let mut point_ar=Arithmetic::default();
    for _ in 0..12 {
        let k=loop {
            let candidate=prng(&mut rng)%N;
            if candidate>1 && known.insert(candidate) {break candidate;}
        };
        let q=point_ar.scalar(PRIME31,k,Some(PRIME31.g))
            .ok_or("size sweep target was identity")?;
        targets.push((q,k));
    }
    let mut configurations=Vec::new();
    for (count,round_limit) in SIZES {
        let mut cases=Vec::new();
        let mut successes=0usize;
        let mut total_ns=0u128;
        let mut total_pairs=0u64;
        let mut total_rounds=0usize;
        for (idx,&(q,k)) in targets.iter().enumerate() {
            let v=execute_sized(false,round_limit,q,count)?;
            let solved=v["complete_target_scalar_recovered"].as_bool()==Some(true);
            let attempts=v["attempts"].as_array().ok_or("no sweep attempts")?;
            let last=attempts.last().ok_or("empty sweep attempts")?;
            let found=last.pointer("/solution/log").and_then(|x|x.as_u64());
            if solved && (found!=Some(k)
                || point_ar.scalar(PRIME31,found.unwrap(),Some(PRIME31.g))!=Some(q)) {
                return Err(format!("sweep scalar replay failed for T={count}, case={idx}"));
            }
            let cost=v["cold_rust_wall_ns"].as_u64().ok_or("missing sweep wall")?;
            let pairs=attempts.iter().map(|row|
                row["algebraic_visited_pairs"].as_u64().unwrap_or(0)
            ).sum::<u64>();
            if solved{successes+=1;}
            total_ns+=u128::from(cost);
            total_pairs+=pairs;
            total_rounds+=attempts.len();
            cases.push(json!({
                "case":idx,"public_target":q,
                "expected_known_answer_verifier_only":k,
                "recovered_log":found,"solved":solved,
                "rounds":attempts.len(),"cold_wall_ns":cost,
                "visited_pair_quadratics":pairs,
                "unresolved_within_cap":!solved
            }));
        }
        configurations.push(json!({
            "points_per_partition":count,"max_rounds":round_limit,
            "solved":successes,"cases":targets.len(),
            "total_cold_wall_ns":total_ns.to_string(),
            "total_pair_quadratics":total_pairs,"total_rounds":total_rounds,
            "individual":cases
        }));
    }
    Ok(json!({
        "standing":"[pcc:unverified]",
        "classification":"published APS-style index calculus; pair-root enumeration generic MITM-equivalent",
        "curve_prime":P,"group_order":N,
        "sweep":"12 identical independently generated caller-side held-out public Q per arm; fixed seed, varied T and bounded retries",
        "no_known_scalar_supplied_to_algebraic_solver":true,
        "reports_failures_and_misses":true,
        "calibration":"one sequential Rust release CI development sweep; not an isolated A/A comparison",
        "arms":configurations
    }))
}

// Paired, source-cold, fresh 24-target confirmation. The two APS arms use
// identical public Q and published S3 one-relation extraction. A/A repeats
// the incumbent T=1536 with the same deterministic public labels to expose
// same-host sequential timing instability, not to claim statistical isolation.
// Every trial, including repeats and misses, remains charged in the transcript.
fn fresh_size_confirmation()->Result<serde_json::Value,String>{
    const CASES:usize=24;
    let mut rng=0x4652_4553_485f_5332_u64;
    let mut used=HashSet::new();
    let mut verifier=Arithmetic::default();
    let mut results=Vec::with_capacity(CASES);
    let mut total_a=0u128;
    let mut total_b=0u128;
    let mut total_aa=0u128;
    let mut success_a=0usize;
    let mut success_b=0usize;
    let mut success_aa=0usize;
    for case in 0..CASES {
        let known=loop {
            let candidate=prng(&mut rng)%N;
            if candidate>1 && used.insert(candidate){break candidate;}
        };
        let q=verifier.scalar(PRIME31,known,Some(PRIME31.g))
            .ok_or("confirmation target was identity")?;
        let sequence=if case%2==0 {
            [("A1024",1024usize,8usize),("B1536",1536,3),
             ("B1536_repeat",1536,3)]
        } else {
            [("B1536",1536usize,3usize),
             ("B1536_repeat",1536,3),("A1024",1024,8)]
        };
        let mut outcomes=Vec::new();
        for (position,(label,count,rounds)) in sequence.iter().enumerate() {
            let v=execute_sized(false,*rounds,q,*count)?;
            let attempts=v["attempts"].as_array().ok_or("missing confirm attempts")?;
            let found=attempts.last().and_then(|row|
                row.pointer("/solution/log")).and_then(|x|x.as_u64());
            let solved=v["complete_target_scalar_recovered"].as_bool()==Some(true);
            if solved && (found!=Some(known)
                || verifier.scalar(PRIME31,found.unwrap(),Some(PRIME31.g))!=Some(q)) {
                return Err(format!("confirmation target {case} returned an invalid scalar"));
            }
            let wall=v["cold_rust_wall_ns"].as_u64().ok_or("missing confirm cold time")?;
            let pairs=attempts.iter().map(|row|
                row["algebraic_visited_pairs"].as_u64().unwrap_or(0)
            ).sum::<u64>();
            match *label {
                "A1024"=>{total_a+=u128::from(wall);if solved{success_a+=1;}},
                "B1536"=>{total_b+=u128::from(wall);if solved{success_b+=1;}},
                "B1536_repeat"=>{total_aa+=u128::from(wall);if solved{success_aa+=1;}},
                _=>return Err("unknown confirm arm".into())
            }
            outcomes.push(json!({
                "arm":label,"execution_order":position,
                "points_per_partition":count,"maximum_rounds":rounds,
                "complete_scalar_recovered":solved,
                "recovered_log":found,
                "rounds":attempts.len(),
                "cold_wall_ns":wall,
                "algebraic_pair_quadratics":pairs,
                "unresolved_within_cap":!solved
            }));
        }
        results.push(json!({"case":case,"public_target":q,
            "caller_known_answer_verifier_only":known,
            "execution_order":"alternates A-B-B and B-B-A",
            "outcomes":outcomes}));
    }
    Ok(json!({
        "standing":"[pcc:unverified]",
        "scope":"fresh 24 public Q, complete Rust APS one-relation same-source paired development confirmation",
        "classification":"published IC; finite S3 list enumeration generic MITM-equivalent",
        "curve_prime":P,"group_order":N,
        "known_scalar_never_enters_solver":true,
        "cases":CASES,"successes_1024":success_a,
        "successes_1536":success_b,"successes_1536_repeat":success_aa,
        "cold_total_1024_ns":total_a.to_string(),
        "cold_total_1536_ns":total_b.to_string(),
        "cold_total_1536_repeat_ns":total_aa.to_string(),
        "result_scope":"one runner, alternating order and A/A timing diagnostic; not isolated or external IC-frontier benchmark",
        "cases_detail":results
    }))
}

fn walk_source_confirmation()->Result<serde_json::Value,String>{
    // Fresh public target panel distinct from both historical --holdouts
    // and --size-confirm; caller-side secrets remain outside execute_sized_mode.
    const CASES:usize=24;
    let mut rng=0x5741_4c4b_5f32_3431_u64;
    let mut used=HashSet::new();
    let mut verifier=Arithmetic::default();
    let mut cases=Vec::with_capacity(CASES);
    let mut sums=[0u128;3];
    let mut setups=[0u128;3];
    let mut pairs=[0u64;3];
    let mut successes=[0usize;3];
    for case in 0..CASES {
        let known=loop {
            let k=prng(&mut rng)%N;
            if k>1 && used.insert(k) {break k;}
        };
        let q=verifier.scalar(PRIME31,known,Some(PRIME31.g))
            .ok_or("fresh walk target is identity")?;
        let order:[(usize,&str,bool);3]=if case%2==0 {
            [(0,"independent",false),(1,"walk",true),(2,"walk_repeat",true)]
        } else {
            [(1,"walk",true),(2,"walk_repeat",true),(0,"independent",false)]
        };
        let mut outcomes=Vec::new();
        for (position,(arm,label,walk)) in order.iter().enumerate() {
            let v=execute_sized_mode(false,ROUNDS,q,T,*walk)?;
            let attempts=v["attempts"].as_array().ok_or("walk no attempts")?;
            let last=attempts.last().ok_or("walk no last attempt")?;
            let log=last.pointer("/solution/log").and_then(|x|x.as_u64());
            let solved=v["complete_target_scalar_recovered"].as_bool()==Some(true);
            if solved && (log!=Some(known) ||
                verifier.scalar(PRIME31,log.unwrap(),Some(PRIME31.g))!=Some(q)){
                return Err(format!("walk arm {label} wrong scalar on case {case}"));
            }
            let cost=v["cold_rust_wall_ns"].as_u64().ok_or("walk missing wall")?;
            let setup=attempts.iter().map(|a|
                a["precompute_labels_ns"].as_u64().unwrap_or(0) as u128).sum::<u128>();
            let queries=attempts.iter().map(|a|
                a["algebraic_visited_pairs"].as_u64().unwrap_or(0)).sum::<u64>();
            let group_ops=attempts.iter().map(|a|
                a["source_point_generation_group_additions"].as_u64().unwrap_or(0))
                .sum::<u64>();
            sums[*arm]+=u128::from(cost);
            setups[*arm]+=setup;
            pairs[*arm]+=queries;
            if solved {successes[*arm]+=1;}
            outcomes.push(json!({
                "arm":label,"position":position,"public_target":q,
                "complete_target_scalar_recovered":solved,"verified_log":log,
                "rounds":attempts.len(),"cold_wall_ns":cost,
                "paid_source_setup_ns":setup.to_string(),
                "algebraic_pair_quadratics":queries,
                "source_group_additions":group_ops,
                "unresolved_within_three_rounds":!solved,
                "attempts":attempts
            }));
        }
        cases.push(json!({
            "case":case,"public_target":q,
            "known_answer_caller_verification_only":known,
            "outcomes":outcomes
        }));
    }
    Ok(json!({
        "standing":"[pcc:unverified]",
        "experimental_status":"producer-development Rust paired complete-solver panel, not independent EKS L2/PCC",
        "classification":"APS 2018 one-relation index-calculus methodology; still generic pair-MITM equivalent",
        "p":P,"n":N,"cofactor":1,"T":T,"round_cap":ROUNDS,
        "fresh_public_targets":CASES,
        "arms":["independent","walk","walk_repeat"],
        "successes":successes,
        "cold_total_ns":sums.map(|v|v.to_string()),
        "source_setup_total_ns":setups.map(|v|v.to_string()),
        "algebraic_S3_pair_queries":pairs,
        "all_success_gate":successes.iter().all(|&v|v==CASES),
        "paired_design":"alternate control-walk-walk and walk-walk-control, each cold, all retries paid",
        "scientific_limit":"no independent PCC; no mature external IC frontier matched run; source distributions differ",
        "cases_detail":cases
    }))
}

fn heldout_twelve()->Result<serde_json::Value,String>{
    // Caller-side fixture generation; the scalar is never passed to execute().
    // These known answers are verified only after the complete algebraic solve.
    let mut rng=0x484f_4c44_4f55_5453_u64;
    let mut used=HashSet::new();
    let mut results=Vec::with_capacity(12);
    let mut successes=0usize;
    let mut total_cold_ns=0u128;
    let mut total_s3_pairs=0u64;
    for case in 0..12usize {
        let k=loop {
            let candidate=prng(&mut rng)%N;
            if candidate>1 && used.insert(candidate){break candidate;}
        };
        let mut verifier=Arithmetic::default();
        let q=verifier.scalar(PRIME31,k,Some(PRIME31.g))
            .ok_or("holdout public Q was identity")?;
        let result=execute(false,ROUNDS,q)?;
        let solved=result["complete_target_scalar_recovered"].as_bool()==Some(true);
        let last=result["attempts"].as_array()
            .and_then(|a|a.last()).ok_or("missing holdout transcript")?;
        let found=last.pointer("/solution/log").and_then(|v|v.as_u64());
        if solved && found!=Some(k) {
            return Err(format!("holdout case {case} returned wrong target scalar"));
        }
        let ns=result["cold_rust_wall_ns"].as_u64().ok_or("missing cold wall")?;
        let pairs=result["attempts"].as_array().ok_or("missing attempts")?
            .iter().map(|v|v["algebraic_visited_pairs"].as_u64().unwrap_or(0))
            .sum::<u64>();
        total_cold_ns+=u128::from(ns);
        total_s3_pairs+=pairs;
        if solved {successes+=1;}
        results.push(json!({
            "case":case,"public_target":q,
            "known_answer_verifier_only":k,
            "recovered_log":found,
            "success":solved,
            "attempts":result["attempts"].as_array().map(|a|a.len()),
            "cold_wall_ns":ns,"s3_pairs":pairs
        }));
    }
    Ok(json!({
        "standing":"[pcc:unverified]",
        "classification":"published APS-style S3 pair enumeration; generic MITM equivalent",
        "fixture":"12 deterministic independent nontrivial public Q=[k]G; k not passed to execute",
        "curve_prime":P,"group_order":N,
        "cases":12,"successes":successes,
        "total_cold_wall_ns":total_cold_ns.to_string(),
        "total_s3_pair_roots":total_s3_pairs,
        "cases_detail":results,
        "scientific_limit":"source-specific developer CI, no independent PCC or non-generic cost claim"
    }))
}

fn main()->Result<(),String> {
    let args:Vec<String>=std::env::args().skip(1).collect();
    if args.len()==1 && args[0]=="--walk-confirm" {
        let result=walk_source_confirmation()?;
        println!("{}",serde_json::to_string_pretty(&result).map_err(|e|e.to_string())?);
        return Ok(());
    }
    if args.len()==1 && args[0]=="--size-confirm" {
        let result=fresh_size_confirmation()?;
        println!("{}",serde_json::to_string_pretty(&result).map_err(|e|e.to_string())?);
        return Ok(());
    }
    if args.len()==1 && args[0]=="--size-sweep" {
        let result=partition_size_sweep()?;
        println!("{}",serde_json::to_string_pretty(&result).map_err(|e|e.to_string())?);
        return Ok(());
    }
    if args.len()==1 && args[0]=="--holdouts" {
        let v=heldout_twelve()?;
        println!("{}",serde_json::to_string_pretty(&v).map_err(|e|e.to_string())?);
        if v["successes"].as_u64()!=Some(12) {
            return Err("some held-out prime31 targets did not solve within three bounded rounds".into());
        }
        return Ok(());
    }
    let compare=match args.as_slice() {
        [] => false,
        [a] if a=="--one" => false,
        [a] if a=="--compare" => true,
        _ => return Err("usage: prime31_aps_one_relation --one|--compare|--holdouts|--size-sweep|--size-confirm|--walk-confirm".into())
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
    fn varied_rust_partition_sizes_preserve_public_base_invariants(){
        let mut ar=Arithmetic::default();
        let (parts,identities,duplicates)=source_sized(PUBLIC_Q,SEED,&mut ar,128).unwrap();
        assert_eq!(parts.iter().map(Vec::len).sum::<usize>(),384);
        assert_eq!(parts.iter().flat_map(|v|v.iter().map(|pt|pt.pt.0))
            .collect::<HashSet<_>>().len(),384);
        assert_eq!(identities,0);
        assert!(duplicates<10);
        let trial=execute_sized(false,1,PUBLIC_Q,128).unwrap();
        assert_eq!(trial["points_per_partition"].as_u64(),Some(128));
        assert!(trial["attempts"].as_array().is_some());
    }
    #[test]
    fn walk_labels_replay_to_public_points() {
        let mut arithmetic=Arithmetic::default();
        check_curve(PRIME31,&mut arithmetic).unwrap();
        let (parts,_identities,duplicates)=
            source_walk_sized(PUBLIC_Q,SEED,&mut arithmetic,128).unwrap();
        assert_eq!(parts.iter().map(Vec::len).sum::<usize>(),384);
        let xs:HashSet<_>=parts.iter().flat_map(|part|part.iter().map(|v|v.pt.0))
            .collect();
        assert_eq!(xs.len(),384);
        assert!(duplicates<128);
        for row in parts.iter().flat_map(|part|part.iter()) {
            let ag=arithmetic.scalar(PRIME31,row.a,Some(PRIME31.g));
            let bq=arithmetic.scalar(PRIME31,row.b,Some(PUBLIC_Q));
            assert_eq!(arithmetic.add(PRIME31,ag,bq),Some(row.pt));
            assert!(arithmetic.on_curve(PRIME31,Some(row.pt)));
        }
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
