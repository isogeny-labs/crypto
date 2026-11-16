//! Exact signed m4 target-yield control for ordinary cofactor-one prime31.
//! This is GROUP ENUMERATION and NOT an index-calculus oracle.
//! cargo test --locked --release --manifest-path research/prime31_ic_20261115/native/Cargo.toml --example prime31_m4_natural_yield_gate
//! cargo run  --locked --release --manifest-path research/prime31_ic_20261115/native/Cargo.toml --example prime31_m4_natural_yield_gate
use prime31_algebraic_ic::{check_curve, Arithmetic, Base, Curve, Point, PRIME31};
use serde_json::json;
use std::collections::BTreeSet;

fn choose(n: u64, k: u64) -> u64 {
    if k > n { return 0; }
    let k = k.min(n - k);
    let mut r = 1u64;
    for j in 1..=k { r = r * (n - k + j) / j; }
    r
}
fn bound(b: u64) -> u64 {
    1 + 4 * b + 16 * choose(b, 2) + 24 * choose(b, 3) + 16 * choose(b, 4)
}
fn census(e: Curve, points: &[(u64,u64)], ar: &mut Arithmetic) -> Result<(usize,usize,u64), String> {
    // ±P and repetitions allowed, so O must be included in the 2-sum support.
    let mut signed = BTreeSet::new();
    for &(x,y) in points {
        signed.insert((x,y));
        signed.insert((x, (e.p-y)%e.p));
    }
    if signed.len() != 2*points.len() {
        return Err("unexpected repeated or order-two factor-base point".into());
    }
    let s: Vec<_> = signed.into_iter().collect();
    let mut two: BTreeSet<Point> = BTreeSet::from([None]);
    for (i, &a) in s.iter().enumerate() {
        for &b in &s[i..] { two.insert(ar.add(e,Some(a),Some(b))); }
    }
    let nonzero: Vec<_> = two.iter().filter_map(|&p| p).collect();
    let mut four = two.clone();
    let before = ar.counts.group_additions;
    for (i, &a) in nonzero.iter().enumerate() {
        for &b in &nonzero[i..] { four.insert(ar.add(e,Some(a),Some(b))); }
    }
    let additions = ar.counts.group_additions - before;
    if !four.contains(&None) || four.len() as u64 > bound(points.len() as u64) {
        return Err("m4 support violates exact coefficient-vector upper bound".into());
    }
    Ok((two.len(),four.len(),additions))
}
const RELATIONS: &[&[(usize, i64)]] = &[
    &[(5,-1),(9,1),(16,-1),(17,-1),(18,-1),(19,-1),(29,-1),(31,1)],
    &[(1,-1),(5,1),(16,-1),(20,1),(22,1),(25,-1),(26,1),(30,1)],
    &[(5,-1),(11,-1),(15,1),(16,-1),(22,-1),(23,-1),(25,1),(28,-1)],
    &[(4,-2),(9,-1),(11,-1),(12,-1),(16,-1),(29,1),(30,1)],
];
fn verify_relation(e: Curve, points: &[(u64,u64)], row: &[(usize,i64)], ar: &mut Arithmetic) -> bool {
    let mut sum: Point = None;
    for &(one_based,c) in row {
        if c == 0 || one_based == 0 || one_based > points.len() { return false; }
        let (x,y) = points[one_based-1];
        let q = if c > 0 {Some((x,y))} else {Some((x,(e.p-y)%e.p))};
        for _ in 0..c.unsigned_abs() {sum=ar.add(e,sum,q);}
    }
    sum.is_none()
}
fn rank(rows: &[&[(usize,i64)]], p: u64) -> usize {
    let mut a = vec![vec![0u64; 31];rows.len()];
    for (i,row) in rows.iter().enumerate() {
        for &(j,v) in *row { a[i][j-1]=(v.rem_euclid(p as i64) as u64+a[i][j-1])%p; }
    }
    let mut r=0;
    for c in 0..31 {
        let Some(k)=(r..a.len()).find(|&k| a[k][c]!=0) else {continue;};
        a.swap(r,k);
        let mut v=a[r][c];
        let mut exponent=p-2;
        let mut inv=1u64;
        while exponent>0 {
            if exponent&1==1 {inv=inv*v%p;}
            v=v*v%p;exponent>>=1;
        }
        for j in c..31 {a[r][j]=a[r][j]*inv%p;}
        for i in 0..a.len() {
            if i==r {continue;}
            let factor=a[i][c];
            for j in c..31 {a[i][j]=(a[i][j]+p-factor*a[r][j]%p)%p;}
        }
        r+=1;
        if r==a.len(){break;}
    }
    r
}
fn run()->Result<serde_json::Value,String>{
    let e=PRIME31;
    let mut ar=Arithmetic::default();
    check_curve(e,&mut ar)?;
    let base=Base::build(e,6,&mut ar)?;
    if base.points.len()!=31 {return Err("canonical B31 fixture changed".into());}
    let mut rows=Vec::new();
    for b in [4,8,16,31] {
        let (two,four,adds)=census(e,&base.points[..b],&mut ar)?;
        let expected=bound(b as u64);
        if (two as u64,four as u64)!=(2*(b*b) as u64+1,expected) {
            return Err(format!("canonical m4 count changed at B={b}: two={two} four={four}"));
        }
        rows.push(json!({"selection":"canonical depth6", "base_size":b, "two_support":two,
             "four_support":four,"combinatorial_max":expected,
             "exact_uniform_target_hit_fraction":(four as f64)/(e.n as f64),
             "two_support_pair_additions":2*b*b+b,
             "four_support_additions":adds}));
    }
    let depth7=Base::build(e,7,&mut ar)?;
    let mut injective=Vec::new();
    for &(x,y) in &depth7.points {
        let disc=(e.p-((3*((x as u128*x as u128 % e.p as u128) as u64)+4*e.a)%e.p))%e.p;
        // p == 3 mod 4; direct Euler criterion suffices for selecting
        // the unique rational x-lift, without factoring a cubic.
        let mut z=disc;
        let mut pow=e.p-1;pow/=2;
        let mut out=1u64;
        while pow>0 {
            if pow&1==1 {out=((out as u128*z as u128)%e.p as u128) as u64;}
            z=((z as u128*z as u128)%e.p as u128) as u64;
            pow>>=1;
        }
        if out==e.p-1 {injective.push((x,y));}
    }
    if depth7.points.len()!=64 || injective.len()!=39 {return Err("depth7 injective y count changed".into());}
    let s=&injective[..31];
    let (two,four,adds)=census(e,s,&mut ar)?;
    if (two,four)!=(1923,618625){return Err(format!("injective-y m4 counts changed: {two}, {four}"));}
    for (i,row) in RELATIONS.iter().enumerate(){
        if !verify_relation(e,s,row,&mut ar) {return Err(format!("weight-eight identity {} invalid",i+1));}
    }
    let independent=rank(RELATIONS,e.n);
    if independent!=4 {return Err("four frozen relations lost independent rank".into());}
    rows.push(json!({"selection":"first31 injective-y depth7","base_size":31,
        "two_support":two,"four_support":four,"combinatorial_max":bound(31),
        "support_deficit_vs_generic_coefficient_vectors":bound(31)-four as u64,
        "exact_uniform_target_hit_fraction":(four as f64)/(e.n as f64),
        "four_support_additions":adds,
        "verified_eight_term_zero_relations":RELATIONS.len(),
        "verified_independent_relation_rank":independent}));
    Ok(json!({"standing":"[pcc:unverified]",
      "classification":"generic exact group-support enumeration NEGATIVE CONTROL; not algebraic IC",
      "p":e.p,"n":e.n,"source":"same native Arithmetic/Base/PRIME31",
      "max_four_term_coefficient_vectors":"1+4B+16C(B,2)+24C(B,3)+16C(B,4)",
      "rows":rows}))
}
fn main(){match run(){Ok(j)=>println!("{}",serde_json::to_string_pretty(&j).unwrap()),
 Err(e)=>{eprintln!("{e}");std::process::exit(1);}}}
#[cfg(test)]mod tests{#[test]fn exact_m4_natural_yield_and_relations(){super::run().unwrap();}}
