//! Run79: exact *REFERENCE-ONLY* two-leaf signed support over the original
//! ordinary p=2^31-1 cofactor-one curve. Reuses the existing native Rust
//! group arithmetic; NEVER used as the candidate index-calculus extractor.
//! Checks the B=64/128/256/677 factor-base x-prefix from the public root
//! oracle and counts exact possible uniform-target hits rather than inferring
//! probability from planted positives.
//!
//! cargo run --locked --release --manifest-path research/prime31_ic_20261115/native/Cargo.toml --example prime31_run79_exact_pair_support
use prime31_algebraic_ic::{check_curve, Arithmetic, Point, PRIME31};
use serde_json::json;
use std::collections::{HashSet};
use std::time::Instant;

const P: u64 = 2_147_483_647;
const GX: u64 = 625_037_715;
const X2G: u64 = 1_923_055_775;
const SIZES: [usize; 4] = [64, 128, 256, 677];

/// Match the existing examples/prime31_monic_s3_pair_oracle.rs factorbase:
/// start with GX, X2G, then increasing rational x<10000. Only x is an
/// algebraic candidate input. Ordinates are lifted HERE for reference only.
fn factorbase_prefix(max: usize, ar: &mut Arithmetic) -> Vec<(u64,u64)> {
    let mut xs=vec![GX,X2G];
    for x in 0..10_000u64 {
        if xs.len()==max {break;}
        if xs.contains(&x) {continue;}
        let rhs=ar.rhs(PRIME31,x);
        if ar.sqrt(rhs,P).is_some() {xs.push(x);}
    }
    assert_eq!(xs.len(),max,"did not reproduce the prior W");
    let mut out=Vec::new();
    for x in xs {
        let rhs=ar.rhs(PRIME31,x);
        let y=ar.sqrt(rhs,P).expect("rational factor base");
        assert_ne!(y,0,"odd-order curve cannot contain 2-torsion");
        let point=(x,y.min(P-y));
        assert!(ar.on_curve(PRIME31,Some(point)));
        out.push(point);
    }
    out
}

fn exact_support(base: &[(u64,u64)], ar: &mut Arithmetic) -> serde_json::Value {
    let began=Instant::now();
    let b=base.len();
    let max_unordered=b*(b+1)/2;
    let bound=4*max_unordered;
    let mut group_hits:HashSet<(u64,u64)>=HashSet::with_capacity(bound);
    let mut x_hits:HashSet<u64>=HashSet::with_capacity(bound/2);
    let additions_before=ar.counts.group_additions;
    for (i,&p) in base.iter().enumerate(){
        for &q in &base[i..] {
            // Group laws applied only in this independent reference scorer.
            // The ± sign orbit of {p+q,p-q} is all 4 signed combinations.
            let minus_q=ar.neg(PRIME31,Some(q));
            let plus=ar.add(PRIME31,Some(p),Some(q));
            let difference=ar.add(PRIME31,Some(p),minus_q);
            for target in [plus,difference] {
                if let Some((x,y))=target {
                    assert!(ar.on_curve(PRIME31,target),"reference off curve");
                    x_hits.insert(x);
                    group_hits.insert((x,y));
                    group_hits.insert((x,if y==0{0}else{P-y}));
                }
            }
        }
    }
    assert!(group_hits.len()<=bound,"support exceeds combinatorial ceiling");
    assert_eq!(group_hits.len(),2*x_hits.len(),"negation closure failed");
    let n=PRIME31.n;
    // Uniform nonzero Q probability exactly #distinct nonzero signed sums/(n-1).
    let hit_probability=group_hits.len() as f64/(n-1) as f64;
    let ceiling=bound as f64/(n-1) as f64;
    assert!(hit_probability <= ceiling);
    json!({
        "B": b,
        "unordered_x_pairs": max_unordered,
        "signed_sum_upper_bound": bound,
        "distinct_nonzero_signed_group_sums": group_hits.len(),
        "distinct_target_x": x_hits.len(),
        "exact_uniform_Q_hit_probability": hit_probability,
        "uniform_Q_hit_ceiling": ceiling,
        "exact_uniform_Q_expected_queries_per_hit": 1.0/hit_probability,
        "support_collision_deficit_vs_4_times_pairs": bound-group_hits.len(),
        "group_additions_reference": ar.counts.group_additions-additions_before,
        "reference_elapsed_ms": began.elapsed().as_secs_f64()*1000.0,
        "reference_only": true
    })
}

fn main() -> Result<(),String> {
    let mut ar=Arithmetic::default();
    check_curve(PRIME31,&mut ar)?;
    assert_eq!(PRIME31.p,P);
    let base=factorbase_prefix(*SIZES.last().unwrap(),&mut ar);
    let mut rows=Vec::new();
    for b in SIZES {
        let row=exact_support(&base[..b],&mut ar);
        eprintln!("run79 B={b}: exact signed support={}",row["distinct_nonzero_signed_group_sums"]);
        rows.push(row);
    }
    println!("{}",json!({
        "schema":"prime31-run79-exact-signed-pair-support-v1",
        "curve": {"p": PRIME31.p,"a":PRIME31.a,"b":PRIME31.b,"n":PRIME31.n,"cofactor":1},
        "source":"existing original-Rust prime31_algebraic_ic Arithmetic and unchanged B-prefix",
        "qualification":"GENERIC_POSTHOC_REFERENCE_ONLY; NOT A CANDIDATE EXTRACTOR",
        "population":"uniform nonidentity Q; x-only planted positives excluded",
        "rows":rows
    }));
    Ok(())
}
