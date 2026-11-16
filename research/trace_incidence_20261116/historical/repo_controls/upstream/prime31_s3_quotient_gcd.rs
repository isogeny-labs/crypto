//! Target-dependent S3 resultant/GCD oracle on the inherited ordinary prime31
//! curve. Uses factor-base abscissae + public target only for algebraic discovery.
//! No EC additions, known ordinates, signed-sum tables or rho are consulted
//! until the independently checked candidate-lift step.
//!
//! Development experiment only; classical elimination, not a sub-rho claim.
//! cargo test --locked --release --manifest-path research/prime31_ic_20261115/native/Cargo.toml --example prime31_s3_quotient_gcd
//! cargo run --locked --release --manifest-path research/prime31_ic_20261115/native/Cargo.toml --example prime31_s3_quotient_gcd

use prime31_algebraic_ic::{check_curve, Arithmetic, Base, Point, Relation, PRIME31};
use serde_json::json;

const P: u64 = 2_147_483_647;
type Pol = Vec<u64>;

fn add(a: u64, b: u64) -> u64 { let t = a + b; if t >= P { t-P } else { t } }
fn sub(a: u64, b: u64) -> u64 { if a >= b { a-b } else { a+P-b } }
fn mul(a: u64, b: u64) -> u64 { ((a as u128 * b as u128) % P as u128) as u64 }
fn power(mut b: u64, mut e: u64) -> u64 {
    let mut a = 1;
    while e != 0 { if e & 1 != 0 { a = mul(a,b); } b=mul(b,b); e >>= 1; }
    a
}
fn trim(mut p: Pol) -> Pol { while p.last() == Some(&0) { p.pop(); } p }
fn sum(a: &[u64], b: &[u64]) -> Pol {
    let mut r = vec![0; a.len().max(b.len())];
    for (i,&v) in a.iter().enumerate() { r[i]=add(r[i],v); }
    for (i,&v) in b.iter().enumerate() { r[i]=add(r[i],v); }
    trim(r)
}
fn neg(a: &[u64]) -> Pol { a.iter().map(|&x| sub(0,x)).collect() }
fn diff(a: &[u64], b: &[u64]) -> Pol { sum(a, &neg(b)) }
fn scale(a: &[u64], k: u64) -> Pol { trim(a.iter().map(|&v| mul(v,k)).collect()) }
fn prod(a: &[u64], b: &[u64]) -> Pol {
    if a.is_empty() || b.is_empty() { return vec![]; }
    let mut r=vec![0; a.len()+b.len()-1];
    for (i,&x) in a.iter().enumerate() {
        if x==0 { continue; }
        for (j,&y) in b.iter().enumerate() {
            if y!=0 { r[i+j]=add(r[i+j],mul(x,y)); }
        }
    }
    trim(r)
}
fn divrem(f: &[u64], d: &[u64]) -> (Pol,Pol) {
    assert!(!d.is_empty());
    let mut r=f.to_vec();
    let mut q=vec![0; f.len().saturating_sub(d.len()).saturating_add(1)];
    let inv=power(*d.last().unwrap(),P-2);
    while r.len()>=d.len() {
        let j=r.len()-d.len();
        let c=mul(*r.last().unwrap(),inv);
        q[j]=c;
        for (i,&v) in d.iter().enumerate() { r[i+j]=sub(r[i+j],mul(c,v)); }
        r=trim(r);
    }
    (trim(q),r)
}
fn rem(f: &[u64], h: &[u64]) -> Pol { divrem(f,h).1 }
fn gcd(mut a: Pol, mut b: Pol) -> Pol {
    while !b.is_empty() { let r=rem(&a,&b); a=b; b=r; }
    if a.is_empty() { return a; }
    let inv=power(*a.last().unwrap(), P-2);
    scale(&a,inv)
}
fn invert(f: &[u64], h: &[u64]) -> Result<Pol,String> {
    let (mut r0,mut r1)=(h.to_vec(),rem(f,h));
    let (mut t0,mut t1)=(vec![],vec![1]);
    while !r1.is_empty() {
        let (q,r)=divrem(&r0,&r1);
        let t=diff(&t0,&prod(&q,&t1));
        r0=r1; r1=r; t0=t1; t1=t;
    }
    if r0.len()!=1 { return Err("nonunit leading coefficient in factor-base quotient".into()); }
    Ok(rem(&scale(&t0,power(r0[0],P-2)),h))
}
fn mprod(a: &[u64], b: &[u64], h: &[u64]) -> Pol { rem(&prod(a,b),h) }
fn eval(p: &[u64], x: u64) -> u64 {
    p.iter().rev().fold(0,|acc,&v| add(mul(acc,x),v))
}
fn base_poly(xs: &[u64]) -> Pol {
    xs.iter().fold(vec![1],|p,&x| prod(&p,&[sub(0,x),1]))
}
// S3(X,Y,t) viewed as quadratic in Y. No ordinates appear anywhere here.
fn s3_y(x: &[u64], t: u64) -> (Pol,Pol,Pol) {
    let tpoly=vec![t];
    let x_t=diff(x,&tpoly);
    let xt=prod(x,&tpoly);
    let x_plus_t=sum(x,&tpoly);
    let aa=prod(&x_t,&x_t);
    let bb=neg(&scale(&sum(&prod(&x_plus_t,&sum(&xt,&[PRIME31.a])),
                            &[mul(2,PRIME31.b)]),2));
    let xterm=diff(&xt,&[PRIME31.a]);
    let cc=diff(&prod(&xterm,&xterm),&scale(&x_plus_t,mul(4,PRIME31.b)));
    (trim(aa),trim(bb),trim(cc))
}
fn s3_scalar(x: u64, t: u64) -> Pol {
    let (a,b,c)=s3_y(&[x],t);
    vec![*c.first().unwrap_or(&0),*b.first().unwrap_or(&0),
         *a.first().unwrap_or(&0)]
        .into_iter().collect::<Pol>().pipe_trim()
}
trait TrimPol { fn pipe_trim(self) -> Self; }
impl TrimPol for Pol { fn pipe_trim(self) -> Self { trim(self) } }

// Coordinate-only m=4 pilot. K(U) is the finite pair-sum x-support
// polynomial. Its factors use S3(x_i,x_j,U) and no point ordinates.
// Deliberately NOT a fast or complete index-calculus oracle.
fn m4_pair_support(xs: &[u64]) -> Result<Pol,String> {
    let mut k=vec![1];
    for (i,&x) in xs.iter().enumerate() {
        for &y in &xs[i..] {
            let f=s3_scalar(x,y);
            if f.len()<2 { return Err("degenerate S3 pair factor".into()); }
            k=prod(&k,&f);
        }
    }
    if k.is_empty() { return Err("zero pair-support polynomial".into()); }
    let leading=*k.last().unwrap();
    Ok(scale(&k,power(leading,P-2)))
}

// Project the S3(U,V,t) intersection of the finite pair-support K(U),
// K(V) onto U via the same quadratic quotient norm as the m=2 oracle.
// This is a membership eliminant only: no factorization, sign lifts,
// rank extraction or complete ECDLP recovery is hidden in this function.
// With naive polynomial arithmetic its worst-case cost is cubic in deg K.
// The exceptional U=t factor is separated before quotient inversion.
fn m4_first_eliminant(k: &[u64], t: u64) -> Result<(Pol,bool),String> {
    if k.is_empty() || *k.last().unwrap()!=1 {
        return Err("expected monic pair support".into());
    }
    let exceptional=eval(k,t)==0;
    let regular=if exceptional {
        let (q,r)=divrem(k,&[sub(0,t),1]);
        if !r.is_empty() { return Err("exceptional pair factor division failed".into()); }
        q
    } else { k.to_vec() };
    let mut d=vec![1];
    if regular.len()>1 {
        let (aa,bb,cc)=s3_y(&[0,1],t);
        let ia=invert(&aa,&regular)?;
        let beta=mprod(&bb,&ia,&regular);
        let gamma=mprod(&cc,&ia,&regular);
        let (mut r0,mut r1)=(vec![],vec![]);
        for &c in k.iter().rev() {
            let nr0=diff(&[c],&mprod(&gamma,&r1,&regular));
            let nr1=diff(&r0,&mprod(&beta,&r1,&regular));
            r0=rem(&nr0,&regular);
            r1=rem(&nr1,&regular);
        }
        let r01=mprod(&r0,&r1,&regular);
        let norm=sum(
            &diff(&mprod(&r0,&r0,&regular),&mprod(&beta,&r01,&regular)),
            &mprod(&gamma,&mprod(&r1,&r1,&regular),&regular));
        d=gcd(regular.clone(),rem(&norm,&regular));
    }
    if exceptional {
        // The omitted specialization U=t is a linear/constant S3 in V.
        // Include its factor iff it has a common finite root with K(V).
        let common=gcd(k.to_vec(),s3_scalar(t,t));
        if common.len()>1 { d=prod(&d,&[sub(0,t),1]); }
    }
    Ok((d,exceptional))
}

// A division-free factorized S4 resultant. Reuses the same coordinate-only
// pair support K and S3 coefficients; no group operation/ordinate is an input.
//
// Res_V(S3(U,V,t), product_ij S3(xi,xj,V)) is multiplicative. Since both
// quadratics have degree <= 2 in V, each individual Sylvester determinant
// is a polynomial of degree <= 4 in U. Multiplying those short polynomials
// modulo K(U) avoids Horner evaluation of a degree-B^2 K(V) in a quadratic
// quotient over Fp[U]/K(U). At U=t, leading (U-t)^2 vanishes, so split this
// branch before evaluating the regular quotient and replay it separately.
// For diagonal i=j, the padded quadratic resultant includes a factor
// (U-t)^2; this is a unit in the regular quotient and hence harmless.
fn m4_first_eliminant_factorized(
    xs: &[u64], k: &[u64], t: u64,
) -> Result<(Pol, bool, usize, usize), String> {
    if k.is_empty() || *k.last().unwrap() != 1 {
        return Err("factorized S4 expects monic pair support".into());
    }
    let exceptional = eval(k, t) == 0;
    let regular = if exceptional {
        let (q, r) = divrem(k, &[sub(0, t), 1]);
        if !r.is_empty() {
            return Err("factorized S4 exceptional division failed".into());
        }
        if eval(&q, t) == 0 {
            return Err("factorized S4 requires squarefree exceptional U=t root".into());
        }
        q
    } else {
        k.to_vec()
    };
    let (a, b, c) = s3_y(&[0, 1], t);
    let mut product = vec![1];
    let mut factors = 0usize;
    let mut max_factor_degree = 0usize;
    if regular.len() > 1 {
        for (i, &xi) in xs.iter().enumerate() {
            for &xj in &xs[i..] {
                let g = s3_scalar(xi, xj);
                if g.len() < 2 || g.len() > 3 {
                    return Err("invalid degree of S3 pair factor".into());
                }
                let (d0, d1, d2) = (g[0], g[1], *g.get(2).unwrap_or(&0));
                // Res of a V^2 + b V + c and d2 V^2 + d1 V + d0:
                // (a*d0-c*d2)^2 - (a*d1-b*d2)*(b*d0-c*d1).
                let ad0_cd2 = diff(&scale(&a, d0), &scale(&c, d2));
                let ad1_bd2 = diff(&scale(&a, d1), &scale(&b, d2));
                let bd0_cd1 = diff(&scale(&b, d0), &scale(&c, d1));
                let resultant = diff(
                    &prod(&ad0_cd2, &ad0_cd2),
                    &prod(&ad1_bd2, &bd0_cd1),
                );
                let degree = resultant.len().saturating_sub(1);
                if degree > 4 {
                    return Err("unexpected S4 quartic degree".into());
                }
                max_factor_degree = max_factor_degree.max(degree);
                product = mprod(&product, &resultant, &regular);
                factors += 1;
            }
        }
    }
    let mut d = if regular.len() > 1 {
        gcd(regular, product)
    } else {
        vec![1]
    };
    if exceptional {
        let common = gcd(k.to_vec(), s3_scalar(t, t));
        if common.len() > 1 {
            d = prod(&d, &[sub(0, t), 1]);
        }
    }
    Ok((d, exceptional, factors, max_factor_degree))
}

// Deterministic, capped Cantor-Zassenhaus style splitting for known
// squarefree, completely Fp-split gcd factors. A cap is an explicit failure,
// not a zero-relation result. Input factors come from the coordinate algebra.
fn powmod_poly(mut b: Pol, mut e: u64, f: &[u64]) -> Pol {
    let mut r=vec![1];
    b=rem(&b,f);
    while e>0 {
        if e&1!=0 { r=mprod(&r,&b,f); }
        b=mprod(&b,&b,f);
        e>>=1;
    }
    r
}
fn split_linear_roots(f: &[u64], remaining_depth: usize) -> Result<Vec<u64>,String> {
    if f.len()<=1 { return Ok(Vec::new()); }
    if f.len()==2 {
        return Ok(vec![mul(sub(0,f[0]),power(f[1],P-2))]);
    }
    if remaining_depth==0 { return Err("m4 root factorization depth cap".into()); }
    let deriv:Pol=trim(f.iter().enumerate().skip(1)
        .map(|(i,&v)| mul((i as u64)%P,v)).collect());
    if gcd(f.to_vec(),deriv).len()>1 {
        return Err("m4 root factor has repeated roots".into());
    }
    for shift in 0..128u64 {
        let xshift=vec![shift,1];
        let z=powmod_poly(xshift.clone(),(P-1)/2,f);
        // Include zero, nonzero squares and non-squares. Any nontrivial
        // factor is valid; retaining every branch makes splitting exact.
        let candidates=[xshift,diff(&z,&[1]),sum(&z,&[1])];
        for divisor in &candidates {
            let g=gcd(f.to_vec(),divisor.clone());
            if g.len()<=1 || g.len()>=f.len() { continue; }
            let (q,r)=divrem(f,&g);
            if !r.is_empty() { return Err("nonexact m4 factor split".into()); }
            let mut roots=split_linear_roots(&g,remaining_depth-1)?;
            roots.extend(split_linear_roots(&q,remaining_depth-1)?);
            roots.sort_unstable();
            roots.dedup();
            return Ok(roots);
        }
    }
    Err("m4 root split cap reached; undetermined".into())
}

fn m4_extract_from_eliminant(xs: &[u64], k: &[u64], t: u64, d: &[u64])
    -> Result<(Vec<(u64,u64)>,Vec<[usize;4]>),String>
{
    let u_roots=split_linear_roots(d,32)?;
    let mut uv=Vec::new();
    let mut raw=Vec::new();
    for u in u_roots {
        let vpoly=gcd(k.to_vec(),s3_scalar(u,t));
        for v in split_linear_roots(&vpoly,32)? {
            if eval(&s3_scalar(u,v),t)!=0 {
                return Err("m4 resultants emitted invalid central S3 root".into());
            }
            uv.push((u,v));
            let left:Vec<(usize,usize)>=(0..xs.len())
                .flat_map(|i| (i..xs.len()).map(move |j| (i,j)))
                .filter(|&(i,j)| eval(&s3_scalar(xs[i],xs[j]),u)==0)
                .collect();
            let right:Vec<(usize,usize)>=(0..xs.len())
                .flat_map(|i| (i..xs.len()).map(move |j| (i,j)))
                .filter(|&(i,j)| eval(&s3_scalar(xs[i],xs[j]),v)==0)
                .collect();
            for &(i,j) in &left {
                for &(l,m) in &right { raw.push([i,j,l,m]); }
            }
        }
    }
    uv.sort_unstable();uv.dedup();
    raw.sort_unstable();raw.dedup();
    Ok((uv,raw))
}

fn m4_extract_pairs(xs: &[u64], k: &[u64], t: u64)
    -> Result<(Vec<(u64,u64)>,Vec<[usize;4]>),String>
{
    let (d,_)=m4_first_eliminant(k,t)?;
    m4_extract_from_eliminant(xs,k,t,&d)
}

// All ordinate/group operations are quarantined to this final semantic
// verifier. The algebraic discovery above receives only (K, xs, x_R).
fn m4_signed_lifts(base: &Base,target: Point,tuples: &[[usize;4]],
                   ar: &mut Arithmetic)->Vec<Vec<(usize,i32)>>
{
    let mut rows=Vec::new();
    for ids in tuples {
        for mask in 0..16u8 {
            let mut sum=None;
            let mut entries=Vec::new();
            for (j,&i) in ids.iter().enumerate() {
                let sign=if mask&(1<<j)==0 {1} else {-1};
                let mut pt=Some(base.points[i]);
                if sign<0 {pt=ar.neg(PRIME31,pt);}
                sum=ar.add(PRIME31,sum,pt);
                entries.push((i,sign));
            }
            if sum==target {
                entries.sort_unstable();
                rows.push(entries);
            }
        }
    }
    rows.sort_unstable();rows.dedup();rows
}

// Frozen, bounded developmental scaling on the SAME prime31 curve and
// Dickson depth-six factor base. Fixed B=4/8/16 slices and identical two
// public target encodings. It is NOT the full ECDLP workload or a rho A/B.
fn m4_scale(base: &Base, ar: &mut Arithmetic) -> Result<(),String> {
    use std::time::Instant;
    let left=ar.add(PRIME31,Some(base.points[0]),Some(base.points[1]));
    let right=ar.add(PRIME31,Some(base.points[2]),Some(base.points[3]));
    let positive=ar.add(PRIME31,left,right)
        .ok_or("unexpected planted m4 identity")?;
    let public=ar.scalar(PRIME31,17,Some(PRIME31.g))
        .ok_or("unexpected public [17]G identity")?;
    let mut panel=Vec::new();
    for size in [4usize,8,16] {
        let xs:Vec<u64>=base.points[..size].iter().map(|p|p.0).collect();
        let start=Instant::now();
        let k=m4_pair_support(&xs)?;
        let support_ns=start.elapsed().as_nanos();
        if k.len()-1!=size*size {
            return Err("m4 pair support degree/collision changed".into());
        }
        for (label,target) in [("planted-positive",positive),("fixed-public",public)] {
            let start=Instant::now();
            let (d,exceptional)=m4_first_eliminant(&k,target.0)?;
            let membership_ns=start.elapsed().as_nanos();
            let start=Instant::now();
            let (uv,raw)=m4_extract_from_eliminant(&xs,&k,target.0,&d)?;
            let algebraic_extraction_ns=start.elapsed().as_nanos();
            let additions_before=ar.counts.group_additions;
            let start=Instant::now();
            let rows=m4_signed_lifts(base,Some(target),&raw,ar);
            let validation_ns=start.elapsed().as_nanos();
            let validation_group_additions=ar.counts.group_additions-additions_before;
            if label=="planted-positive" && rows.is_empty() {
                return Err("m4 algebraic positive extraction failed".into());
            }
            panel.push(json!({
                "base_size":size, "target_kind":label, "public_target":target,
                "pair_support_degree":k.len()-1,
                "precomputed_pair_support_build_ns":support_ns,
                "eliminant_degree":d.len()-1,
                "exceptional_target_abscissa":exceptional,
                "target_dependent_eliminant_ns":membership_ns,
                "target_dependent_algebraic_root_extraction_ns":algebraic_extraction_ns,
                "eliminant_recomputed_during_extraction":false,
                "ordered_uv":uv.len(),"ordered_pair_index_tuples":raw.len(),
                "post_extraction_signed_validation_ns":validation_ns,
                "post_extraction_validation_group_additions":validation_group_additions,
                "projectively_normalized_signed_rows":rows.len(),
                "complete_ecdlp_recovery":false
            }));
        }
    }
    println!("{}",json!({
        "standing":"[pcc:unverified]",
        "study":"frozen m4 coordinate-only index-calculus relation extraction B4/B8/B16",
        "curve":"ordinary full-prime-order F_(2^31-1)",
        "source_ordinate_or_group_sum_used_in_polynomial_stage":false,
        "cold_factor_base_construction_charged":false,
        "amortized_pair_support_precompute_reported_separately":true,
        "end_to_end_ECDLP_recovery":false,
        "matched_rho_reference_run":false,
        "rows":panel
    }));
    Ok(())
}

// Deterministic, naturally sampled public targets. No known scalar is passed
// into the algebraic discovery stage. Every attempted query, even a miss,
// contributes to the charged target-dependent time. Production usage must
// validate recovered signed relations independently.
fn splitmix64_next(state: &mut u64) -> u64 {
    *state = state.wrapping_add(0x9e3779b97f4a7c15);
    let mut z = *state;
    z = (z ^ (z >> 30)).wrapping_mul(0xbf58476d1ce4e5b9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94d049bb133111eb);
    z ^ (z >> 31)
}

fn modmul_n(a: u64, b: u64, n: u64) -> u64 {
    ((a as u128 * b as u128) % n as u128) as u64
}
fn modpow_n(mut a: u64, mut e: u64, n: u64) -> u64 {
    let mut r = 1u64;
    while e != 0 {
        if e & 1 != 0 { r = modmul_n(r, a, n); }
        a = modmul_n(a, a, n);
        e >>= 1;
    }
    r
}
// A matrix rank of signed coefficient rows, not a certified rank of a
// complete logarithm linear system. n is prime on the frozen curve.
fn signed_coefficient_rank(rows: &[Vec<(usize,i32)>], width: usize, n: u64)
    -> Result<usize, String>
{
    let mut pivots: Vec<Option<Vec<u64>>> = vec![None; width];
    let mut rank = 0;
    for row in rows {
        let mut v = vec![0u64; width];
        for &(i, sign) in row {
            if i >= width || (sign != 1 && sign != -1) {
                return Err("invalid signed coefficient".into());
            }
            let c = if sign == 1 { 1 } else { n-1 };
            v[i] = (v[i] + c) % n;
        }
        for col in 0..width {
            if v[col] == 0 { continue; }
            if let Some(pivot) = &pivots[col] {
                let c = v[col];
                for j in col..width {
                    v[j] = (v[j] + n - modmul_n(c, pivot[j], n)) % n;
                }
            } else {
                let inv = modpow_n(v[col], n-2, n);
                for j in col..width { v[j] = modmul_n(v[j], inv, n); }
                pivots[col] = Some(v);
                rank += 1;
                break;
            }
        }
    }
    Ok(rank)
}

// Experimental 4096-query Rust-only natural-public panel. The target is
// generated from a known scalar fixture, but that scalar never enters the
// oracle and is reported only to enable independent ex-post replay.
// This is not a complete index-calculus ECDLP, nor a cold rho comparison.
fn m4_factorized_natural_4096(base: &Base, ar: &mut Arithmetic)
    -> Result<(), String>
{
    use std::collections::BTreeSet;
    use std::time::Instant;
    const TRIALS: usize = 4096;
    const SEED: u64 = 0x525553545f4d3401;
    let xs: Vec<u64> = base.points.iter().map(|q| q.0).collect();
    if xs.len() != 31 { return Err("natural gate requires canonical B31 base".into()); }
    let start = Instant::now();
    let k = m4_pair_support(&xs)?;
    let support_build_ns = start.elapsed().as_nanos();
    if k.len() != 31*31+1 { return Err("natural gate K degree changed".into()); }
    let mut state = SEED;
    let mut seen = BTreeSet::new();
    let mut fingerprint = 0xcbf29ce484222325u64;
    let mut query_ns = Vec::with_capacity(TRIALS);
    let mut target_generation_ns = 0u128;
    let mut target_generation_additions = 0u64;
    let mut extraction_ns = 0u128;
    let mut validation_ns = 0u128;
    let mut verification_additions = 0u64;
    let mut nonconstant_gcds = 0usize;
    let mut certified_positive_targets = 0usize;
    let mut undetermined = 0usize;
    let mut exceptional_targets = 0usize;
    let mut signed_rows = Vec::new();
    let mut positives = Vec::new();
    let mut failures = Vec::new();
    for sample in 0..TRIALS {
        let scalar = loop {
            let next = (splitmix64_next(&mut state) % (PRIME31.n-1)) + 1;
            if seen.insert(next) { break next; }
        };
        fingerprint = (fingerprint ^ scalar).wrapping_mul(0x100000001b3);
        let before = ar.counts.group_additions;
        let started = Instant::now();
        let target = ar.scalar(PRIME31, scalar, Some(PRIME31.g))
            .ok_or("nonzero natural scalar unexpectedly mapped to identity")?;
        target_generation_ns += started.elapsed().as_nanos();
        target_generation_additions += ar.counts.group_additions - before;

        let started = Instant::now();
        let (d, exceptional, _, _) =
            m4_first_eliminant_factorized(&xs, &k, target.0)?;
        query_ns.push(started.elapsed().as_nanos());
        if exceptional { exceptional_targets += 1; }
        if d.len() <= 1 { continue; }
        nonconstant_gcds += 1;
        let started = Instant::now();
        let extraction = m4_extract_from_eliminant(&xs, &k, target.0, &d);
        extraction_ns += started.elapsed().as_nanos();
        let (_uv, tuples) = match extraction {
            Ok(ok) => ok,
            Err(error) => {
                undetermined += 1;
                failures.push(json!({
                    "sample":sample, "target":target,
                    "reason":error, "status":"undetermined"
                }));
                continue;
            }
        };
        let before = ar.counts.group_additions;
        let started = Instant::now();
        let rows = m4_signed_lifts(base, Some(target), &tuples, ar);
        validation_ns += started.elapsed().as_nanos();
        verification_additions += ar.counts.group_additions - before;
        if !rows.is_empty() {
            certified_positive_targets += 1;
            for row in &rows { signed_rows.push(row.clone()); }
        }
        positives.push(json!({
            "sample":sample, "target":target,
            "fixture_scalar_after_discovery":scalar,
            "gcd_degree":d.len()-1,
            "raw_pair_tuples":tuples.len(),
            "verified_signed_rows":rows
        }));
    }
    query_ns.sort_unstable();
    let total_query_ns: u128 = query_ns.iter().sum();
    let sampled_rank = signed_coefficient_rank(&signed_rows, xs.len(), PRIME31.n)?;
    let p50 = query_ns[(TRIALS-1)/2];
    let p95 = query_ns[(TRIALS-1)*95/100];
    println!("{}", json!({
        "standing":"[pcc:unverified]",
        "study":"factorized S4 Rust-only naturally sampled prime31 B31 query gate",
        "sample_method":"splitmix64 distinct nonzero fixture scalars, generator multiplication excluded from oracle",
        "seed_hex":"525553545f4d3401",
        "sample_count":TRIALS, "distinct_public_targets":seen.len(),
        "fixture_scalar_fingerprint_fnv64":format!("{fingerprint:016x}"),
        "curve_p":PRIME31.p,"curve_n":PRIME31.n,
        "factor_base_size":xs.len(),"pair_support_degree":k.len()-1,
        "pair_support_build_ns":support_build_ns,
        "target_generation_ns":target_generation_ns,
        "target_generation_group_additions":target_generation_additions,
        "total_algebraic_query_ns":total_query_ns,
        "algebraic_query_median_ns":p50,"algebraic_query_p95_ns":p95,
        "nonconstant_gcd_targets":nonconstant_gcds,
        "verified_positive_targets":certified_positive_targets,
        "exceptional_target_x":exceptional_targets,
        "undetermined_extraction_targets":undetermined,
        "completed_negative_targets":TRIALS-nonconstant_gcds,
        "additional_algebraic_extraction_ns":extraction_ns,
        "post_extraction_signed_validation_ns":validation_ns,
        "post_extraction_validation_group_additions":verification_additions,
        "verified_signed_coefficient_rows":signed_rows.len(),
        "signed_coefficient_rank_mod_n":sampled_rank,
        "caveat":"coefficient rank, not a completed factor-base logarithm system",
        "natural_nonconstant_cases":positives,"censored_cases":failures,
        "cold_ECDLP_solved":false,"rho_matched":false
    }));
    Ok(())
}

// Paired Rust-only A/B on the SAME K(U) and SAME public target, with
// algebraic discovery separated from any signed EC witness verification.
// B31 original Horner is omitted because it scales prohibitively.
fn m4_factorized_scale(base: &Base, ar: &mut Arithmetic) -> Result<(), String> {
    use std::time::Instant;
    let l = ar.add(PRIME31, Some(base.points[0]), Some(base.points[1]));
    let r = ar.add(PRIME31, Some(base.points[2]), Some(base.points[3]));
    let positive = ar.add(PRIME31, l, r)
        .ok_or("factorized positive fixture is identity")?;
    let exceptional = l.ok_or("factorized exceptional fixture is identity")?;
    let public = ar.scalar(PRIME31, 17, Some(PRIME31.g))
        .ok_or("public fixture is identity")?;
    let mut records = Vec::new();
    for size in [4usize, 8, 16, 31] {
        let xs: Vec<u64> = base.points[..size].iter().map(|q| q.0).collect();
        let begin = Instant::now();
        let k = m4_pair_support(&xs)?;
        let support_build_ns = begin.elapsed().as_nanos();
        if k.len() != size * size + 1 {
            return Err(format!("factorized K degree changed at B={size}"));
        }
        for (kind, target) in [
            ("planted-four", positive),
            ("public-17G", public),
            ("exceptional-two", exceptional),
        ] {
            let started = Instant::now();
            let (fast, exc, factors, maximum_degree) =
                m4_first_eliminant_factorized(&xs, &k, target.0)?;
            let factorized_ns = started.elapsed().as_nanos();
            let reference_ns = if size <= 16 {
                let started = Instant::now();
                let (old, old_exc) = m4_first_eliminant(&k, target.0)?;
                let elapsed = started.elapsed().as_nanos();
                if fast != old || exc != old_exc {
                    return Err(format!("factorized/reference mismatch at B={size} kind={kind} (fast_degree={} old_degree={})",
                        fast.len().saturating_sub(1), old.len().saturating_sub(1)));
                }
                Some(elapsed)
            } else {
                None
            };
            if maximum_degree > 4 {
                return Err("factorized S4 quartic bound broken".into());
            }
            if size == 31 && factors != size * (size + 1) / 2 {
                return Err("factorized B31 factor count changed".into());
            }
            // Root splitting remains capped: test both small-fixture and B31
            // planted witnesses, never an unbounded target enumeration.
            let verified_signed_rows = if (size == 4 || size == 31) && kind == "planted-four" {
                let (_, tuples) = m4_extract_from_eliminant(&xs, &k, target.0, &fast)?;
                let rows = m4_signed_lifts(base, Some(target), &tuples, ar);
                if kind == "planted-four" && rows.is_empty() {
                    return Err("factorized S4 lost planted four-term relation".into());
                }
                Some(rows.len())
            } else {
                None
            };
            records.push(json!({
                "B":size, "target_kind":kind, "target_x":target.0,
                "pair_support_degree":k.len()-1, "pair_support_precompute_ns":support_build_ns,
                "short_resultant_factors":factors,
                "max_short_resultant_degree":maximum_degree,
                "factorized_target_dependent_ns":factorized_ns,
                "reference_target_dependent_ns":reference_ns,
                "same_monic_eliminant_as_reference":reference_ns.is_some(),
                "exceptional_u_equals_target_x":exc,
                "gcd_degree":fast.len().saturating_sub(1),
                "post_extraction_signed_rows_planted_B4_B31":verified_signed_rows,
                "measured_cold_ECDLP":false
            }));
        }
    }
    println!("{}",json!({
        "standing":"[pcc:unverified]",
        "method":"multiplicative short-quartic S4 resultants modulo algebraic pair support",
        "curve_p":PRIME31.p,"curve_order":PRIME31.n,
        "no_group_sums_or_ordinates_in_discovery":true,
        "no_generic_table_used_by_algebraic_oracle":true,
        "reference_B31_excluded_from_censored_comparison":true,
        "same_Rust_process_matched_inputs":true,
        "full_ECDLP_solution":false,"rows":records
    }));
    Ok(())
}

fn m4_preview(base: &Base, ar: &mut Arithmetic) -> Result<(),String> {
    use std::time::Instant;
    if base.points.len()<4 { return Err("m4 preview requires B>=4".into()); }
    let xs:Vec<u64>=base.points.iter().map(|v|v.0).collect();
    let started=Instant::now();
    let k31=m4_pair_support(&xs)?;
    let b31_support_ns=started.elapsed().as_nanos();
    if k31.len()!=962 { return Err("B31 pair-support degree changed".into()); }
    let derivative:Pol=trim(k31.iter().enumerate().skip(1)
        .map(|(i,&v)| mul((i as u64)%P,v)).collect());
    if gcd(k31.clone(),derivative).len()!=1 {
        return Err("B31 pair-support lost squarefreeness".into());
    }
    let k4=m4_pair_support(&xs[..4])?;
    if k4.len()!=17 { return Err("B4 pair-support degree changed".into()); }
    // Test-only construction is not an input to the polynomial discovery.
    let left=ar.add(PRIME31,Some(base.points[0]),Some(base.points[1]));
    let right=ar.add(PRIME31,Some(base.points[2]),Some(base.points[3]));
    let planted=ar.add(PRIME31,left,right)
        .ok_or("unexpected identity planted m4 relation")?;
    let public=ar.scalar(PRIME31,17,Some(PRIME31.g))
        .ok_or("unexpected zero public target")?;
    let started=Instant::now();
    let (positive,positive_exception)=m4_first_eliminant(&k4,planted.0)?;
    let positive_ns=started.elapsed().as_nanos();
    let started=Instant::now();
    let (negative,negative_exception)=m4_first_eliminant(&k4,public.0)?;
    let negative_ns=started.elapsed().as_nanos();
    if positive.len()!=7 || negative.len()!=1 {
        return Err("frozen m4 B4 projection positive/negative degrees changed".into());
    }
    let (uvpos,rawpos)=m4_extract_pairs(&xs[..4],&k4,planted.0)?;
    let validation_start=ar.counts.group_additions;
    let rowspos=m4_signed_lifts(base,Some(planted),&rawpos,ar);
    let validation_adds=ar.counts.group_additions-validation_start;
    let (uvneg,rawneg)=m4_extract_pairs(&xs[..4],&k4,public.0)?;
    let rowsneg=m4_signed_lifts(base,Some(public),&rawneg,ar);
    if uvpos.len()!=6 || rowspos.len()!=1 || !rowsneg.is_empty() {
        return Err("m4 algebraic extraction or signed EC replay mismatch".into());
    }
    println!("{}",json!({
        "standing":"[pcc:unverified]",
        "study":"m4 finite pair-support resultant; algebraic membership only",
        "curve":"ordinary full-prime-order F_(2^31-1)",
        "pair_support_B31_degree":k31.len()-1,
        "pair_support_B31_squarefree":true,
        "pair_support_B31_build_ns":b31_support_ns,
        "pair_support_B4_degree":k4.len()-1,
        "positive_eliminant_degree":positive.len()-1,
        "positive_ordered_uv":uvpos.len(),
        "positive_four_index_tuples":rawpos.len(),
        "positive_signed_rows":rowspos.len(),
        "positive_validation_group_additions":validation_adds,
        "positive_target":planted,
        "positive_exceptional":positive_exception,
        "positive_eliminant_ns":positive_ns,
        "public_17G_target":public,
        "public_eliminant_degree":negative.len()-1,
        "public_ordered_uv":uvneg.len(),
        "public_four_index_tuples":rawneg.len(),
        "public_signed_rows":rowsneg.len(),
        "public_exceptional":negative_exception,
        "public_eliminant_ns":negative_ns,
        "known_ordinates_in_polynomial_stage":false,
        "known_group_sums_in_polynomial_stage":false,
        "candidate_factorization_performed":true,
        "signed_relation_extraction_performed":true,
        "complete_ECDLP_recovery":false
    }));
    Ok(())
}

#[derive(Debug)]
struct Algebraic {
    // Candidate abscissa index pairs only. No ordinates are supplied.
    pairs: Vec<(usize,usize)>,
    first_eliminant_degree: usize,
    target_x_in_base: bool,
}
fn quotient_oracle(xs: &[u64], t: u64) -> Result<Algebraic,String> {
    let h=base_poly(xs);
    // (X-t)^2 is a unit modulo h unless t is itself a factor-base x.
    // Split off that linear factor, then solve its exceptional branch directly.
    let special=xs.binary_search(&t).is_ok();
    let hregular=if special {
        let (q,r)=divrem(&h,&[sub(0,t),1]);
        if !r.is_empty() { return Err("failed to split exceptional factor".into()); }
        q
    } else { h.clone() };
    let mut roots=vec![];
    let mut degree=0;
    if hregular.len()>1 {
        let (aa,bb,cc)=s3_y(&[0,1],t);
        let ia=invert(&aa,&hregular)?;
        let beta=mprod(&bb,&ia,&hregular);
        let gamma=mprod(&cc,&ia,&hregular);
        let (mut r0,mut r1)=(vec![],vec![]);
        // Horner reduction of h(Y) by Y^2+beta Y+gamma, in Fp[X]/hregular.
        for &c in h.iter().rev() {
            let nr0=diff(&[c],&mprod(&gamma,&r1,&hregular));
            let nr1=diff(&r0,&mprod(&beta,&r1,&hregular));
            r0=rem(&nr0,&hregular); r1=rem(&nr1,&hregular);
        }
        // resultant of the linear remainder and the monic quadratic:
        // r0^2 - beta*r0*r1 + gamma*r1^2.
        let r01=mprod(&r0,&r1,&hregular);
        let tpol=sum(&diff(&mprod(&r0,&r0,&hregular),
                          &mprod(&beta,&r01,&hregular)),
                     &mprod(&gamma,&mprod(&r1,&r1,&hregular),&hregular));
        let d=gcd(hregular.clone(),rem(&tpol,&hregular));
        degree=d.len().saturating_sub(1);
        for (i,&x) in xs.iter().enumerate() {
            if x!=t && eval(&d,x)==0 { roots.push(i); }
        }
    }
    if special { roots.push(xs.binary_search(&t).unwrap()); }
    let mut pairs=vec![];
    for i in roots {
        // One-variable GCD, not an EC group-sum or a pair-table lookup.
        let q=s3_scalar(xs[i],t);
        let g=if q.is_empty() { h.clone() } else { gcd(h.clone(),q) };
        for (j,&y) in xs.iter().enumerate() {
            if eval(&g,y)==0 { pairs.push((i,j)); }
        }
    }
    pairs.sort_unstable();pairs.dedup();
    Ok(Algebraic {pairs,first_eliminant_degree:degree,target_x_in_base:special})
}
fn signed_lifts(base: &Base, target: Point, pairs: &[(usize,usize)],
                ar: &mut Arithmetic) -> Vec<Relation> {
    let mut rows=vec![];
    for &(i,j) in pairs {
        for si in [-1,1] {
            for sj in [-1,1] {
                let u=Some(base.points[i]);
                let v=Some(base.points[j]);
                let u=if si==1 {u} else {ar.neg(PRIME31,u)};
                let v=if sj==1 {v} else {ar.neg(PRIME31,v)};
                if ar.add(PRIME31,u,v)==target {
                    let mut entries=[(i,si),(j,sj)];
                    entries.sort_unstable();
                    rows.push(Relation{entries});
                }
            }
        }
    }
    rows.sort_unstable();rows.dedup();
    rows
}
fn evaluate(base: &Base, target: Point, ar: &mut Arithmetic)
    -> Result<(Algebraic,Vec<Relation>,u64),String>
{
    let r=target.ok_or("S3 affine target identity excluded; use direct inverse-pair treatment")?;
    if !ar.on_curve(PRIME31,target) {return Err("off-curve target".into());}
    let xs:Vec<u64>=base.points.iter().map(|v|v.0).collect();
    let result=quotient_oracle(&xs,r.0)?;
    let start=ar.counts.group_additions;
    let rows=signed_lifts(base,target,&result.pairs,ar);
    Ok((result,rows,ar.counts.group_additions-start))
}
fn main() -> Result<(),String> {
    let mut ar=Arithmetic::default();
    check_curve(PRIME31,&mut ar)?;
    assert_eq!(PRIME31.p,P);
    let base=Base::build(PRIME31,6,&mut ar)?;
    assert_eq!(base.points.len(),31);
    let positives=[
        ar.add(PRIME31,Some(base.points[3]),Some(base.points[9])),
        ar.add(PRIME31,Some(base.points[4]),Some(base.points[4])),
        ar.add(PRIME31,Some(base.points[0]),ar.neg(PRIME31,Some(base.points[7]))),
    ];
    let mut samples=vec![];
    for target in positives {
        let target=target.ok_or("unexpected identity planted positive")?;
        let (v,rows,c)=evaluate(&base,Some(target),&mut ar)?;
        if rows.is_empty() {return Err("algebraic positive extraction failed".into());}
        samples.push(json!({"kind":"held-apart positive control","target":target,
            "gcd_degree":v.first_eliminant_degree,"xy_pairs":v.pairs.len(),
            "signed_rows":rows.len(),"candidate_validation_group_additions":c,
            "exceptional_x":v.target_x_in_base}));
    }
    for k in [1,17,137,20261115] {
        let target=ar.scalar(PRIME31,k,Some(PRIME31.g))
            .ok_or("unexpected zero scalar")?;
        let (v,rows,c)=evaluate(&base,Some(target),&mut ar)?;
        // A random public point can genuinely have a relation; do not assume
        // all deterministic controls are negative.
        samples.push(json!({"kind":"public multiple, post-hoc scalar fixture only",
            "target":target,"gcd_degree":v.first_eliminant_degree,
            "xy_pairs":v.pairs.len(),"signed_rows":rows.len(),
            "candidate_validation_group_additions":c,
            "exceptional_x":v.target_x_in_base}));
    }
    println!("{}",json!({"standing":"[pcc:unverified]",
        "curve":"ordinary prime-order F_(2^31-1)","B":base.points.len(),
        "method":"target-dependent S3 univariate resultant/remainder/GCD",
        "polynomial_input_uses_known_ordinates":false,
        "polynomial_input_uses_EC_group_sums":false,
        "full_ECDLP_recovery":false,"samples":samples}));
    if std::env::args().any(|arg| arg == "--m4-preview") {
        m4_preview(&base,&mut ar)?;
    }
    if std::env::args().any(|arg| arg == "--m4-scale") {
        m4_scale(&base,&mut ar)?;
    }
    if std::env::args().any(|arg| arg == "--m4-factorized") {
        m4_factorized_scale(&base, &mut ar)?;
    }
    if std::env::args().any(|arg| arg == "--m4-natural-4096") {
        m4_factorized_natural_4096(&base, &mut ar)?;
    }
    if std::env::args().any(|arg| arg == "--compare") {
        // Diagnostic A/B only: same Rust host/inputs, fixed base constructed
        // before timings. A bounded F4 "undetermined" is not a timed loss.
        use prime31_algebraic_ic::{decompose, Limits};
        use std::time::Instant;
        let positive=ar.add(PRIME31,Some(base.points[3]),Some(base.points[9]))
            .ok_or("missing positive target")?;
        let public=ar.scalar(PRIME31,17,Some(PRIME31.g))
            .ok_or("missing public target")?;
        let mut controls=vec![];
        for (kind,target) in [("positive",positive),("public",public)] {
            let started=Instant::now();
            let (algebra,rows,valid_adds)=evaluate(&base,Some(target),&mut ar)?;
            let gcd_ns=started.elapsed().as_nanos();
            let started=Instant::now();
            let old=decompose(PRIME31,&base,Some(target),
                Limits {degree:12,seconds:1,dense_entries:15_000,
                        extraction_nodes:50_000},&mut ar);
            let f4_ns=started.elapsed().as_nanos();
            if old.status=="complete" && old.relations!=rows {
                return Err("existing F4 and exact quotient give different signed relations".into());
            }
            controls.push(json!({
                "kind":kind, "fixed_base_excluded_from_both_stage_intervals":true,
                "gcd_stage_ns":gcd_ns, "algebraic_xy_pairs":algebra.pairs.len(),
                "gcd_post_extraction_validation_group_additions":valid_adds,
                "new_relations":rows.len(),
                "f4_stage_ns":f4_ns, "f4_status":old.status,
                "f4_relations":old.relations.len(),"f4_partial_muladds":old.f4_muladds,
                "f4_solving_degree":old.solving_degree,
                "stage_only_not_cold_solver":true
            }));
        }
        println!("{}",json!({"standing":"[pcc:unverified]","study":"paired developmental S3 oracle stages, NOT full ECDLP costs",
            "same_rust_host":true,"B":base.points.len(),"rows":controls}));
    }
    if std::env::args().any(|arg| arg == "--depth8") {
        use std::time::Instant;
        let b8=Base::build(PRIME31,8,&mut ar)?;
        if b8.points.len()!=135 {return Err("registered depth8 base changed".into());}
        let planted=ar.add(PRIME31,Some(b8.points[0]),Some(b8.points[14]))
            .ok_or("unexpected depth8 identity")?;
        let public=ar.scalar(PRIME31,17,Some(PRIME31.g))
            .ok_or("unexpected zero public")?;
        let mut panel=vec![];
        for (kind,target) in [("positive",planted),("public",public)] {
            let start=Instant::now();
            let (algebra,rows,valid_adds)=evaluate(&b8,Some(target),&mut ar)?;
            let stage_ns=start.elapsed().as_nanos();
            if kind=="positive" && rows.is_empty() {
                return Err("depth8 positive algebraic quotient oracle missed".into());
            }
            panel.push(json!({"kind":kind,"stage_ns":stage_ns,
              "algebraic_candidate_pairs":algebra.pairs.len(),
              "gcd_degree":algebra.first_eliminant_degree,
              "signed_rows":rows.len(),"validation_group_additions":valid_adds}));
        }
        println!("{}",json!({"standing":"[pcc:unverified]",
            "study":"depth8 B135 exact S3 quotient positive/public stage only",
            "same_prime31_curve":true,"no_target_scalar_in_oracle":true,
            "B":b8.points.len(),"not_cold_ECDLP":true,"rows":panel}));
    }
    Ok(())
}
#[cfg(test)]
mod tests {
    use super::*;
    fn exhaustive(base: &Base,target: Point,ar: &mut Arithmetic) -> Vec<Relation> {
        let mut all=vec![];
        for i in 0..base.points.len() {
            for j in 0..base.points.len() {
                all.extend(signed_lifts(base,target,&[(i,j)],ar));
            }
        }
        all.sort_unstable();all.dedup();all
    }
    #[test]
    fn m4_pair_support_resultant_on_same_prime31_curve() {
        let mut ar=Arithmetic::default();
        check_curve(PRIME31,&mut ar).unwrap();
        let base=Base::build(PRIME31,6,&mut ar).unwrap();
        let xs:Vec<u64>=base.points[..4].iter().map(|v|v.0).collect();
        let k=m4_pair_support(&xs).unwrap();
        assert_eq!(k.len()-1,16);
        let mut support=std::collections::BTreeSet::new();
        // Post-hoc verifier only; no point arithmetic enters the eliminant.
        for i in 0..4 {
            for j in i..4 {
                for sign in [-1,1] {
                    if i==j && sign<0 { continue; }
                    let q=if sign<0 {
                        ar.neg(PRIME31,Some(base.points[j]))
                    } else { Some(base.points[j]) };
                    if let Some(v)=ar.add(PRIME31,Some(base.points[i]),q) {
                        assert_eq!(eval(&k,v.0),0);
                        support.insert(v.0);
                    }
                }
            }
        }
        assert_eq!(support.len(),16);
        let l=ar.add(PRIME31,Some(base.points[0]),Some(base.points[1]));
        let r=ar.add(PRIME31,Some(base.points[2]),Some(base.points[3]));
        let t=ar.add(PRIME31,l,r).unwrap();
        let (d,exc)=m4_first_eliminant(&k,t.0).unwrap();
        assert!(!exc);
        assert_eq!(d.len()-1,6);
        let (uv,tuples)=m4_extract_pairs(&xs,&k,t.0).unwrap();
        let (uv_reused,tuples_reused)=m4_extract_from_eliminant(&xs,&k,t.0,&d).unwrap();
        assert_eq!(uv,uv_reused);
        assert_eq!(tuples,tuples_reused);
        assert_eq!(uv.len(),6);
        let rows=m4_signed_lifts(&base,Some(t),&tuples,&mut ar);
        assert_eq!(rows,vec![vec![(0,1),(1,1),(2,1),(3,1)]]);
        let public=ar.scalar(PRIME31,17,Some(PRIME31.g)).unwrap();
        let (d,exc)=m4_first_eliminant(&k,public.0).unwrap();
        assert!(!exc);
        assert_eq!(d.len()-1,0);
    }

    #[test]
    fn factorized_s4_matches_quotient_horner_and_exceptional_branch() {
        let mut ar = Arithmetic::default();
        check_curve(PRIME31, &mut ar).unwrap();
        let base = Base::build(PRIME31, 6, &mut ar).unwrap();
        let left = ar.add(PRIME31, Some(base.points[0]), Some(base.points[1]));
        let right = ar.add(PRIME31, Some(base.points[2]), Some(base.points[3]));
        let positive = ar.add(PRIME31, left, right).unwrap();
        let public = ar.scalar(PRIME31, 17, Some(PRIME31.g)).unwrap();
        let exception = left.unwrap();
        for size in [4usize, 8, 16] {
            let xs: Vec<u64> = base.points[..size].iter().map(|pt| pt.0).collect();
            let k = m4_pair_support(&xs).unwrap();
            for (kind, target) in [
                ("planted", positive),
                ("public", public),
                ("exceptional", exception),
            ] {
                let (fast, exc, count, maximum_degree) =
                    m4_first_eliminant_factorized(&xs, &k, target.0).unwrap();
                let (old, reference_exceptional) = m4_first_eliminant(&k, target.0).unwrap();
                assert_eq!(exc, reference_exceptional, "exception kind={kind} B={size}");
                assert_eq!(fast, old, "exact monic eliminant kind={kind} B={size}");
                assert_eq!(count, size * (size + 1) / 2);
                assert!(maximum_degree <= 4);
                if kind == "planted" && size == 4 {
                    let (_, tuples) = m4_extract_from_eliminant(&xs, &k, target.0, &fast).unwrap();
                    let rows = m4_signed_lifts(&base, Some(target), &tuples, &mut ar);
                    assert!(!rows.is_empty());
                }
            }
        }
    }

    #[test]
    fn symbolic_oracle_matches_independent_generic_control() {
        let mut ar=Arithmetic::default();
        for depth in [2,6] {
            let base=Base::build(PRIME31,depth,&mut ar).unwrap();
            let mut targets=vec![];
            targets.push(ar.scalar(PRIME31,17,Some(PRIME31.g)));
            for i in 0..base.points.len().min(3) {
                for j in 0..base.points.len().min(3) {
                    targets.push(ar.add(PRIME31,Some(base.points[i]),Some(base.points[j])));
                }
                targets.push(Some(base.points[i])); // exceptional t = base x
            }
            for target in targets.into_iter().flatten() {
                let (found,rows,_)=evaluate(&base,Some(target),&mut ar).unwrap();
                let check=exhaustive(&base,Some(target),&mut ar);
                assert_eq!(rows,check,"algebraic vs signed point addition");
                for &(i,j) in &found.pairs {
                    assert!(i<base.points.len() && j<base.points.len());
                }
            }
        }
    }
}
