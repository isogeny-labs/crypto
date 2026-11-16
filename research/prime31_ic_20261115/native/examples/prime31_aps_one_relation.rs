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
// Exact 2^31-1 reduction, valid for canonical Fp operands. Unlike a
// u128 general remainder, multiplication fits u64 and two folds suffice.
#[inline(always)]
fn mulp_mersenne(x:u64,y:u64)->u64 {
    debug_assert!(x<P && y<P);
    let t=x*y;
    let folded=(t&P)+(t>>31);
    let reduced=(folded&P)+(folded>>31);
    if reduced>=P {reduced-P} else {reduced}
}
#[inline(always)]
fn mulp_mode<const FAST:bool>(x:u64,y:u64)->u64 {
    if FAST {mulp_mersenne(x,y)} else {mulp(x,y)}
}
fn pownp_mode<const FAST:bool>(mut x:u64,mut k:u64)->u64 {
    let mut acc=1u64;
    while k!=0 {
        if k&1!=0 {acc=mulp_mode::<FAST>(acc,x);}
        x=mulp_mode::<FAST>(x,x);
        k>>=1;
    }
    acc
}
fn sqrtp_mode<const FAST:bool>(x:u64)->Option<u64> {
    let r=pownp_mode::<FAST>(x,(P+1)/4);
    (mulp_mode::<FAST>(r,r)==x).then_some(r)
}
fn pown(mut x:u64,mut k:u64,m:u64)->u64 {
    let mut a=1u64;
    while k!=0 {
        if k&1!=0 {a=((a as u128*x as u128)%m as u128) as u64;}
        x=((x as u128*x as u128)%m as u128) as u64;
        k>>=1;
    }
    a
}
#[cfg(test)]
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
    source_walk_profile(q,seed,ar,[count;3])
}
// The exact same 64-step public random walk as the incumbent, but its three
// labelled-list sizes can differ. A single GLOBAL abscissa dedup set protects
// the independent coordinate partitions. No target secret is available here.
fn source_walk_profile(q:(u64,u64),seed:u64,ar:&mut Arithmetic,sizes:[usize;3])
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
    let mut out:[Vec<Labelled>;3]=std::array::from_fn(|i|Vec::with_capacity(sizes[i]));
    let mut xs=HashSet::with_capacity(sizes.iter().sum());
    let mut identity=0u64;
    let mut duplicated=0u64;
    for (part_idx,part) in out.iter_mut().enumerate() {
        let count=sizes[part_idx];
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

// EXP-R31-APS-JACBATCH: batched projective normalization of the existing
// public-label walk. This changes only source arithmetic, not the APS S3
// oracle, partition labels, PRNG, group points, or candidate search semantics.
// It is classical group arithmetic, not a new non-generic ECDLP method.
#[derive(Clone,Copy,Debug)]
struct JacPoint {x:u64,y:u64,z:u64}
fn jac_affine(pt:(u64,u64))->JacPoint{
    JacPoint{x:pt.0,y:pt.1,z:1}
}
fn jac_double(p:JacPoint)->JacPoint{
    if p.z==0 || p.y==0{return JacPoint{x:0,y:1,z:0};}
    let xx=mulp_mersenne(p.x,p.x);
    let yy=mulp_mersenne(p.y,p.y);
    let yyyy=mulp_mersenne(yy,yy);
    let z2=mulp_mersenne(p.z,p.z);
    let z4=mulp_mersenne(z2,z2);
    let s=mulp_mersenne(4,mulp_mersenne(p.x,yy));
    let m=addp(mulp_mersenne(3,xx),mulp_mersenne(PRIME31.a,z4));
    let x=subp(mulp_mersenne(m,m),addp(s,s));
    let y=subp(mulp_mersenne(m,subp(s,x)),mulp_mersenne(8,yyyy));
    let z=mulp_mersenne(2,mulp_mersenne(p.y,p.z));
    JacPoint{x,y,z}
}
fn jac_mixed(p:JacPoint,q:(u64,u64))->JacPoint{
    if p.z==0{return jac_affine(q);}
    let zz=mulp_mersenne(p.z,p.z);
    let u2=mulp_mersenne(q.0,zz);
    let s2=mulp_mersenne(q.1,mulp_mersenne(p.z,zz));
    let h=subp(u2,p.x);
    let r=subp(s2,p.y);
    if h==0{
        return if r==0{jac_double(p)}else{JacPoint{x:0,y:1,z:0}};
    }
    let hh=mulp_mersenne(h,h);
    let hhh=mulp_mersenne(h,hh);
    let v=mulp_mersenne(p.x,hh);
    let x=subp(subp(mulp_mersenne(r,r),hhh),addp(v,v));
    let y=subp(mulp_mersenne(r,subp(v,x)),mulp_mersenne(p.y,hhh));
    let z=mulp_mersenne(p.z,h);
    JacPoint{x,y,z}
}
// One Fp inversion per bounded block (Montgomery's trick), not one per
// source walk step. Nonidentity is mandatory for an affine labelled point.
fn normalize_jac_batch(block:&[(JacPoint,u64,u64)])
    ->Result<Vec<Labelled>,String>{
    if block.is_empty(){return Ok(Vec::new());}
    let mut prefix=Vec::with_capacity(block.len()+1);
    prefix.push(1u64);
    for (p,_,_) in block{
        if p.z==0 {return Err("identity in projective normalization block".into());}
        prefix.push(mulp_mersenne(*prefix.last().ok_or("missing prefix")?,p.z));
    }
    let total=*prefix.last().ok_or("empty projective product")?;
    let mut inv=pownp_mode::<true>(total,P-2);
    if mulp_mersenne(total,inv)!=1{
        return Err("projective batch denominator not invertible".into());
    }
    let mut out=vec![Labelled{pt:(0,0),a:0,b:0};block.len()];
    for i in (0..block.len()).rev(){
        let (p,a,b)=block[i];
        let zi=mulp_mersenne(prefix[i],inv);
        let z2=mulp_mersenne(zi,zi);
        let z3=mulp_mersenne(z2,zi);
        out[i]=Labelled{
            pt:(mulp_mersenne(p.x,z2),mulp_mersenne(p.y,z3)),
            a,b
        };
        inv=mulp_mersenne(inv,p.z);
    }
    if inv!=1{return Err("projective inverse-prefix invariant failed".into());}
    Ok(out)
}
// Produces *exactly* the affine walk's public points/labels at the same PRNG
// locations. Pending transition is deferred until next candidate so neither
// the last in a batch nor the final accepted point consumes an extra PRNG.
fn source_walk_profile_jac(q:(u64,u64),seed:u64,ar:&mut Arithmetic,
    sizes:[usize;3])->Result<([Vec<Labelled>;3],u64,u64),String>{
    const STEPS:usize=64;
    const BATCH:usize=256;
    let mut rng=seed^0x5741_4c4b_5f53_5445_u64;
    let mut steps=Vec::with_capacity(STEPS);
    for _ in 0..STEPS{
        let jump=loop{
            let a=prng(&mut rng)%N;
            let b=prng(&mut rng)%N;
            if a==0 && b==0 {continue;}
            let ag=ar.scalar(PRIME31,a,Some(PRIME31.g));
            let bq=ar.scalar(PRIME31,b,Some(q));
            if let Some(pt)=ar.add(PRIME31,ag,bq){
                break Labelled{pt,a,b};
            }
        };
        steps.push(jump);
    }
    let mut out:[Vec<Labelled>;3]=std::array::from_fn(|i|
        Vec::with_capacity(sizes[i]));
    let mut xs=HashSet::with_capacity(sizes.iter().sum());
    let mut identity=0u64;
    let mut duplicated=0u64;
    for (part_idx,part) in out.iter_mut().enumerate(){
        let count=sizes[part_idx];
        let initial=loop{
            let a=prng(&mut rng)%N;
            let b=prng(&mut rng)%N;
            if a==0 && b==0{continue;}
            let ag=ar.scalar(PRIME31,a,Some(PRIME31.g));
            let bq=ar.scalar(PRIME31,b,Some(q));
            if let Some(pt)=ar.add(PRIME31,ag,bq){
                break Labelled{pt,a,b};
            }
            identity+=1;
        };
        let mut cur=jac_affine(initial.pt);
        let (mut a,mut b)=(initial.a,initial.b);
        let mut pending=false;
        let mut transitions=0usize;
        while part.len()<count{
            let mut block=Vec::with_capacity(BATCH.min(count-part.len()));
            for _ in 0..BATCH.min(count-part.len()){
                if pending{
                    let s=steps[(prng(&mut rng) as usize)&(STEPS-1)];
                    let next_a=addn(a,s.a);
                    let next_b=addn(b,s.b);
                    let next=jac_mixed(cur,s.pt);
                    if next.z!=0{
                        cur=next;a=next_a;b=next_b;
                    }else{
                        identity+=1;
                        let reset=loop{
                            let ra=prng(&mut rng)%N;
                            let rb=prng(&mut rng)%N;
                            if ra==0 && rb==0{continue;}
                            let ag=ar.scalar(PRIME31,ra,Some(PRIME31.g));
                            let bq=ar.scalar(PRIME31,rb,Some(q));
                            if let Some(pt)=ar.add(PRIME31,ag,bq){
                                break Labelled{pt,a:ra,b:rb};
                            }
                            identity+=1;
                        };
                        cur=jac_affine(reset.pt);a=reset.a;b=reset.b;
                    }
                }
                transitions+=1;
                if transitions>count.saturating_mul(96).saturating_add(2_048){
                    return Err("Jacobian walk failed to produce enough distinct x".into());
                }
                block.push((cur,a,b));
                pending=true;
            }
            for pt in normalize_jac_batch(&block)?{
                if xs.insert(pt.pt.0){part.push(pt);} else {duplicated+=1;}
            }
        }
    }
    Ok((out,identity,duplicated))
}

// S3(X,Y,U)= a U^2 + b U + c on the original prime31 curve.
fn s3_coeff_mode<const FAST:bool>(x:u64,y:u64)->(u64,u64,u64) {
    let sum=addp(x,y);
    let xy=mulp_mode::<FAST>(x,y);
    let a=mulp_mode::<FAST>(subp(x,y),subp(x,y));
    let b=subp(0,mulp_mode::<FAST>(2,addp(
        mulp_mode::<FAST>(sum,addp(xy,PRIME31.a)),
        mulp_mode::<FAST>(2,PRIME31.b))));
    let u=subp(xy,PRIME31.a);
    let cc=mulp_mode::<FAST>(u,u);
    let c=subp(cc,mulp_mode::<FAST>(mulp_mode::<FAST>(4,PRIME31.b),sum));
    (a,b,c)
}
#[cfg(test)]
fn s3_coeff(x:u64,y:u64)->(u64,u64,u64) {
    s3_coeff_mode::<false>(x,y)
}
fn s3_eval_mode<const FAST:bool>(coeff:(u64,u64,u64),z:u64)->u64 {
    let (a,b,c)=coeff;
    addp(addp(mulp_mode::<FAST>(a,mulp_mode::<FAST>(z,z)),
        mulp_mode::<FAST>(b,z)),c)
}
#[cfg(test)]
fn s3_eval(coeff:(u64,u64,u64),z:u64)->u64 {
    s3_eval_mode::<false>(coeff,z)
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
fn algebraic_mode<const FAST:bool>(
    parts:&[Vec<Labelled>;3],q:(u64,u64),ar:&mut Arithmetic
)->Result<(Option<Solution>,Stats),String> {
    algebraic_mode_precomp::<FAST,false>(parts,q,ar)
}

// EXP-R31-XFIBRE: x-only factorized S3 discriminant. For S3(X,Y,Z)=aZ²+bZ+c,
// b²-4ac=16 f(X) f(Y) with f(X)=X³+A X+B. Each first/second-list x
// needs just ONE independently obtained Fp square root, reused for all
// pair queries. No source ordinates, group additions, or scalar labels enter
// candidate root discovery. The method remains generic MITM equivalent.
fn rhs_from_x<const FAST:bool>(x:u64)->u64 {
    let x2=mulp_mode::<FAST>(x,x);
    addp(addp(mulp_mode::<FAST>(x2,x),
        mulp_mode::<FAST>(PRIME31.a,x)),PRIME31.b)
}
fn algebraic_mode_precomp<const FAST:bool,const XFIBRE:bool>(
    parts:&[Vec<Labelled>;3],q:(u64,u64),ar:&mut Arithmetic
)->Result<(Option<Solution>,Stats),String> {
    let lookup=reverse_index(&parts[2]);
    let mut stats=Stats::default();
    let mut roots:[Vec<u64>;2]=std::array::from_fn(|j|
        Vec::with_capacity(if XFIBRE {parts[j].len()} else {0}));
    if XFIBRE {
        for j in 0..2 {
            for point in &parts[j] {
                // Use ONLY public abscissa; even the stored y is excluded.
                let rhs=rhs_from_x::<FAST>(point.pt.0);
                let y=sqrtp_mode::<FAST>(rhs)
                    .ok_or("public abscissa has no rational fibre root")?;
                if mulp_mode::<FAST>(y,y)!=rhs {
                    return Err("incorrect x-fibre square-root certificate".into());
                }
                roots[j].push(y);
                stats.s3_sqrt+=1;
            }
        }
    }
    for (i,p) in parts[0].iter().enumerate() {
        let mut coefficients=Vec::with_capacity(parts[1].len());
        let mut sqrts=Vec::with_capacity(parts[1].len());
        let mut prefixes=Vec::with_capacity(parts[1].len()+1);
        let mut denominators=Vec::with_capacity(parts[1].len());
        prefixes.push(1u64);
        for (j,other) in parts[1].iter().enumerate() {
            let f=s3_coeff_mode::<FAST>(p.pt.0,other.pt.0);
            let (a,b,c)=f;
            let (denom,sqr) = if a!=0 {
                let root=if XFIBRE {
                    mulp_mode::<FAST>(4,mulp_mode::<FAST>(roots[0][i],roots[1][j]))
                } else {
                    let disc=subp(mulp_mode::<FAST>(b,b),
                        mulp_mode::<FAST>(mulp_mode::<FAST>(4,a),c));
                    stats.s3_sqrt+=1;
                    sqrtp_mode::<FAST>(disc)
                        .ok_or("S3 Fp discriminant nonsquare on rational points")?
                };
                (mulp_mode::<FAST>(2,a),Some(root))
            } else if b!=0 { (b,None) }
            else {return Err("zero S3 polynomial; cannot discard exceptional branch".into());};
            stats.pair_s3+=1;
            prefixes.push(mulp_mode::<FAST>(*prefixes.last().ok_or("prefix empty")?,denom));
            denominators.push(denom);
            coefficients.push(f);
            sqrts.push(sqr);
        }
        let last=*prefixes.last().ok_or("missing batch prefix")?;
        if last==0 {return Err("zero denominator in rational S3 batch".into());}
        let mut prod_inv=pownp_mode::<FAST>(last,P-2);
        stats.batch_inversions+=1;
        let mut inverses=vec![0u64;parts[1].len()];
        for j in (0..parts[1].len()).rev() {
            inverses[j]=mulp_mode::<FAST>(prod_inv,prefixes[j]);
            prod_inv=mulp_mode::<FAST>(prod_inv,denominators[j]);
        }
        if prod_inv!=1 {return Err("batch inverse invariant failed".into());}
        for (j,f) in coefficients.into_iter().enumerate() {
            let (_a,b,c)=f;
            let mut roots=Vec::with_capacity(2);
            if let Some(root)=sqrts[j] {
                roots.push(mulp_mode::<FAST>(subp(root,b),inverses[j]));
                roots.push(mulp_mode::<FAST>(subp(subp(0,root),b),inverses[j]));
            } else {
                roots.push(mulp_mode::<FAST>(subp(0,c),inverses[j]));
            }
            roots.sort_unstable();roots.dedup();
            for z in roots {
                if s3_eval_mode::<FAST>(f,z)!=0 {return Err("incorrect S3 root extraction".into());}
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
fn algebraic(
    parts:&[Vec<Labelled>;3],q:(u64,u64),ar:&mut Arithmetic
)->Result<(Option<Solution>,Stats),String> {
    algebraic_mode::<false>(parts,q,ar)
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
    execute_sized_mode_mul(compare,round_limit,q,count,walk,false)
}
fn execute_sized_mode_mul(
    compare:bool,round_limit:usize,q:(u64,u64),count:usize,walk:bool,fast:bool
) ->Result<serde_json::Value,String> {
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
        let (solution,stats)=if fast {
            algebraic_mode::<true>(&parts,q,&mut arithmetic)?
        } else {
            algebraic(&parts,q,&mut arithmetic)?
        };
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
            "field_multiplier":if fast {"mersenne_fold"} else {"u128_mod"},
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

fn mersenne_field_confirmation()->Result<serde_json::Value,String>{
    // New 24-Q target panel distinct from earlier --walk-confirm: identical
    // walk-labelled source and S3 searches, only field multiplication differs.
    const CASES:usize=24;
    let mut rng=0x4d45_5253_5f32_3431_u64;
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
            [(0,"u128_mod",false),(1,"mersenne",true),(2,"mersenne_repeat",true)]
        } else {
            [(1,"mersenne",true),(2,"mersenne_repeat",true),(0,"u128_mod",false)]
        };
        let mut outcomes=Vec::new();
        for (position,(arm,label,fast)) in order.iter().enumerate() {
            let v=execute_sized_mode_mul(false,ROUNDS,q,T,true,*fast)?;
            let attempts=v["attempts"].as_array().ok_or("mersenne no attempts")?;
            let last=attempts.last().ok_or("mersenne no last attempt")?;
            let log=last.pointer("/solution/log").and_then(|x|x.as_u64());
            let solved=v["complete_target_scalar_recovered"].as_bool()==Some(true);
            if solved && (log!=Some(known) ||
                verifier.scalar(PRIME31,log.unwrap(),Some(PRIME31.g))!=Some(q)){
                return Err(format!("mersenne arm {label} wrong scalar on case {case}"));
            }
            let cost=v["cold_rust_wall_ns"].as_u64().ok_or("mersenne missing wall")?;
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
        // Order alternates, but every branch must visit the same algebraic
        // quadratic count on identical public labels; any difference is a
        // candidate-selection or arithmetic-correctness regression.
        let ref_pairs=outcomes[0]["algebraic_pair_quadratics"].as_u64();
        if outcomes.iter().any(|o|o["algebraic_pair_quadratics"].as_u64()!=ref_pairs
            || o["rounds"].as_u64()!=outcomes[0]["rounds"].as_u64()
            || o["verified_log"].as_u64()!=outcomes[0]["verified_log"].as_u64()) {
            return Err(format!("mersenne source/search outputs diverged case {case}"));
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
        "arms":["u128_mod","mersenne","mersenne_repeat"],
        "successes":successes,
        "cold_total_ns":sums.map(|v|v.to_string()),
        "source_setup_total_ns":setups.map(|v|v.to_string()),
        "algebraic_S3_pair_queries":pairs,
        "all_success_gate":successes.iter().all(|&v|v==CASES),
        "paired_design":"alternate original-fast-fast and fast-fast-original; identical walk sources and full cold cost",
        "scientific_limit":"same-sources old/new Rust, producer only; no independent PCC or external IC-frontier claim",
        "cases_detail":cases
    }))
}


 // EXP-R31-APS-UNEQUAL: complete published APS S3 single-relation solver with
 // non-equal labelled partitions. This is a *generic-MITM-equivalent* Rust
 // engineering test, not a claimed non-generic attack or full m8 solver.
fn execute_unequal_profile(q:(u64,u64),sizes:[usize;3],round_limit:usize)
    ->Result<serde_json::Value,String>{
    execute_unequal_profile_variant(q,sizes,round_limit,false)
}
fn execute_unequal_profile_variant(q:(u64,u64),sizes:[usize;3],
    round_limit:usize,x_fibre:bool) ->Result<serde_json::Value,String>{
    execute_unequal_profile_source(q,sizes,round_limit,x_fibre,false)
}
fn execute_unequal_profile_source(q:(u64,u64),sizes:[usize;3],
    round_limit:usize,x_fibre:bool,jac:bool)
    ->Result<serde_json::Value,String>{
    if sizes.iter().any(|&n|n<16 || n>32_768) ||
        sizes.iter().sum::<usize>()>40_000 ||
        round_limit==0 || round_limit>8 {
        return Err("unequal-profile admission limit exceeded".into());
    }
    let cold=Instant::now();
    let mut ar=Arithmetic::default();
    check_curve(PRIME31,&mut ar)?;
    if !ar.on_curve(PRIME31,Some(q)) || q==PRIME31.g {
        return Err("invalid public target for unequal APS".into());
    }
    let mut attempts=Vec::new();
    for round in 0..round_limit {
        let seed=SEED.wrapping_add((round as u64).wrapping_mul(0xd1b5_4a32_d192_ed03));
        let setup_begin=Instant::now();
        let additions_before=ar.counts.group_additions;
        let (parts,identities,duplicates)=if jac{
            source_walk_profile_jac(q,seed,&mut ar,sizes)?
        }else{
            source_walk_profile(q,seed,&mut ar,sizes)?
        };
        let setup_ns=setup_begin.elapsed().as_nanos();
        let source_additions=ar.counts.group_additions-additions_before;
        let root_begin=Instant::now();
        let before_verify=ar.counts.group_additions;
        let (solution,stats)=if x_fibre {
            algebraic_mode_precomp::<true,true>(&parts,q,&mut ar)?
        } else {
            algebraic_mode::<true>(&parts,q,&mut ar)?
        };
        let root_ns=root_begin.elapsed().as_nanos();
        let verification_additions=ar.counts.group_additions-before_verify;
        let found=solution.is_some();
        attempts.push(json!({
            "round":round,"seed":format!("{seed:016x}"),"profile":sizes,
            "source_setup_ns":setup_ns,"source_group_additions":source_additions,
            "rejected_identity":identities,"rejected_duplicate_x":duplicates,
            "s3_query_ns":root_ns,"s3_pair_quadratics":stats.pair_s3,
            "x_fibre_precompute":x_fibre,"jacobian_source":jac,
            "s3_sqrts":stats.s3_sqrt,"s3_batch_inversions":stats.batch_inversions,
            "signed_candidates":stats.signed_candidates,
            "verification_group_additions":verification_additions,
            "solution":json_solution(&solution),
            "elapsed_cold_ns":cold.elapsed().as_nanos(),
        }));
        if found {
            return Ok(json!({
                "standing":"[pcc:unverified]","method":"APS one relation, unequal sampled labelled public lists",
                "classification":"published index calculus, generic pair-MITM equivalent",
                "p":P,"n":N,"cofactor":1,"public_Q":q,
                "profile":sizes,"round_cap":round_limit,
                "complete_target_scalar_recovered":true,"known_answer_in_solver":false,
                "cold_wall_ns":cold.elapsed().as_nanos(),"attempts":attempts
            }));
        }
    }
    Ok(json!({
        "standing":"[pcc:unverified]","method":"APS one relation, unequal sampled labelled public lists",
        "classification":"published index calculus, generic pair-MITM equivalent",
        "p":P,"n":N,"cofactor":1,"public_Q":q,
        "profile":sizes,"round_cap":round_limit,
        "complete_target_scalar_recovered":false,"status":"bounded_exhaustion",
        "cold_wall_ns":cold.elapsed().as_nanos(),"attempts":attempts
    }))
}
// Pre-registered fresh public holdout replication of the 384-384-16384 arm.
// No shared source precomputation across targets or arms, no scalar oracle.
fn unequal_384_confirmation()->Result<serde_json::Value,String>{
    const CASES:usize=24;
    let profiles=[[1536,1536,1536],[384,384,16384],[384,384,16384]];
    let names=["equal1536","unequal384","unequal384_repeat"];
    let mut rng=0x434f_4e46_5f33_3834_u64;
    let mut used=HashSet::new();
    let mut verifier=Arithmetic::default();
    let mut totals=[0u128;3];
    let mut successes=[0usize;3];
    let mut queries=[0u64;3];
    let mut source_groups=[0u64;3];
    let mut cases=Vec::with_capacity(CASES);
    let mut paired_wins=0usize;
    for case in 0..CASES {
        let k=loop {
            let x=prng(&mut rng)%N;
            if x>1 && used.insert(x) {break x;}
        };
        let q=verifier.scalar(PRIME31,k,Some(PRIME31.g))
            .ok_or("confirmation produced identity public Q")?;
        let order: [usize;3]=if case%2==0 {[0,1,2]} else {[1,2,0]};
        let mut arms=Vec::new();
        let mut times=[0u64;3];
        for (position,&arm) in order.iter().enumerate(){
            let result=execute_unequal_profile(q,profiles[arm],6)?;
            let done=result["complete_target_scalar_recovered"].as_bool()==Some(true);
            let attempts=result["attempts"].as_array().ok_or("missing confirmation attempts")?;
            let log=attempts.last().and_then(|a|a.pointer("/solution/log"))
                .and_then(|v|v.as_u64());
            if done && (log!=Some(k) ||
                verifier.scalar(PRIME31,log.unwrap(),Some(PRIME31.g))!=Some(q)){
                return Err(format!("confirm arm {} returned invalid log on case {case}",names[arm]));
            }
            let ns=result["cold_wall_ns"].as_u64().ok_or("missing confirmation wall")?;
            let n_queries=attempts.iter().map(|a|
                a["s3_pair_quadratics"].as_u64().unwrap_or(0)).sum::<u64>();
            let n_groups=attempts.iter().map(|a|
                a["source_group_additions"].as_u64().unwrap_or(0)).sum::<u64>();
            times[arm]=ns;totals[arm]+=u128::from(ns);queries[arm]+=n_queries;
            source_groups[arm]+=n_groups;
            if done {successes[arm]+=1;}
            arms.push(json!({
                "arm":names[arm],"order":position,"profile":profiles[arm],
                "complete_target_scalar_recovered":done,"recovered_log":log,
                "rounds":attempts.len(),"cold_wall_ns":ns,
                "s3_pair_queries":n_queries,"source_group_additions":n_groups,
                "bounded_failure":!done,"attempts":attempts
            }));
        }
        if times[1]<times[0]{paired_wins+=1;}
        cases.push(json!({
            "case":case,"public_Q":q,"known_answer_caller_verifier_only":k,
            "arms":arms
        }));
    }
    let all_complete=successes.iter().all(|&n|n==CASES);
    // Integer-only admissions avoid floating/rounding ambiguity.
    let ratio_gate=totals[1]*5<=totals[0]*4;
    let aa_gate=totals[1].abs_diff(totals[2])*20<=totals[1];
    let paired_gate=paired_wins>=18;
    Ok(json!({
        "standing":"[pcc:unverified]","experiment":"EXP-R31-APS-UNEQUAL-384-CONFIRM",
        "classification":"published APS one-relation index calculus; generic MITM-equivalent",
        "field_prime":P,"group_order":N,"cofactor":1,
        "public_holdouts":CASES,"round_cap":6,
        "names":names,"profiles":profiles,
        "successes":successes,"all_complete":all_complete,
        "cold_total_ns":totals.map(|v|v.to_string()),
        "s3_pair_queries":queries,"source_group_additions":source_groups,
        "candidate_paired_wins":paired_wins,
        "candidate_at_least_20_percent_cheaper":ratio_gate,
        "candidate_aa_within_5_percent":aa_gate,
        "candidate_at_least_18_of_24_paired_wins":paired_gate,
        "passes_preregistered_development_gate":all_complete&&ratio_gate&&aa_gate&&paired_gate,
        "scientific_limit":"not independently signed/isolated; no PKM/APS external IC frontier claim",
        "cases_detail":cases
    }))
}

fn unequal_profile_confirmation()->Result<serde_json::Value,String>{
    // Caller-side deterministic holdouts are distinct from historical
    // APS panels; known k never enters execute_unequal_profile.
    const CASES:usize=24;
    let profiles=[[1536,1536,1536],[256,256,32768],[384,384,16384],[256,256,32768]];
    let names=["control1536","unequal256","unequal384","unequal256_repeat"];
    let mut rng=0x554e_4551_5541_4c32_u64;
    let mut used=HashSet::new();
    let mut verifier=Arithmetic::default();
    let mut totals=[0u128;4];
    let mut successes=[0usize;4];
    let mut queries=[0u64;4];
    let mut source_groups=[0u64;4];
    let mut cases=Vec::with_capacity(CASES);
    for case in 0..CASES {
        let k=loop {
            let x=prng(&mut rng)%N;
            if x>1 && used.insert(x) {break x;}
        };
        let q=verifier.scalar(PRIME31,k,Some(PRIME31.g))
            .ok_or("generated identity public Q")?;
        let order: [usize;4]=if case%2==0 {[0,1,2,3]}else{[1,2,3,0]};
        let mut arms=Vec::new();
        for (position,&arm) in order.iter().enumerate() {
            let result=execute_unequal_profile(q,profiles[arm],6)?;
            let solved=result["complete_target_scalar_recovered"].as_bool()==Some(true);
            let attempts=result["attempts"].as_array().ok_or("missing attempts")?;
            let output=attempts.last().and_then(|v|
                v.pointer("/solution/log")).and_then(|v|v.as_u64());
            if solved && (output!=Some(k) ||
                verifier.scalar(PRIME31,output.unwrap(),Some(PRIME31.g))!=Some(q)){
                return Err(format!("unequal arm {} wrong scalar case {}",names[arm],case));
            }
            let cost=result["cold_wall_ns"].as_u64().ok_or("missing cold time")?;
            let n_queries=attempts.iter().map(|a|
                a["s3_pair_quadratics"].as_u64().unwrap_or(0)).sum::<u64>();
            let n_groups=attempts.iter().map(|a|
                a["source_group_additions"].as_u64().unwrap_or(0)).sum::<u64>();
            totals[arm]+=u128::from(cost);
            queries[arm]+=n_queries;
            source_groups[arm]+=n_groups;
            if solved {successes[arm]+=1;}
            arms.push(json!({
                "arm":names[arm],"order":position,"profile":profiles[arm],
                "complete_target_scalar_recovered":solved,"recovered_log":output,
                "rounds":attempts.len(),"cold_wall_ns":cost,
                "all_s3_pair_queries":n_queries,"all_source_group_additions":n_groups,
                "bounded_failure":!solved,"attempts":attempts
            }));
        }
        cases.push(json!({
            "case":case,"public_Q":q,"known_answer_caller_verifier_only":k,
            "arm_order":arms
        }));
    }
    Ok(json!({
        "standing":"[pcc:unverified]",
        "experiment":"EXP-R31-APS-UNEQUAL-20261116",
        "classification":"published APS prime-field one-relation IC, generic MITM-equivalent",
        "curve_p":P,"curve_n":N,"cofactor":1,
        "public_holdouts":CASES,"profiles":profiles,"names":names,
        "round_limit":6,"successes":successes,
        "all_complete":successes.iter().all(|&count|count==CASES),
        "cold_total_ns":totals.map(|v|v.to_string()),
        "s3_pair_queries":queries,"source_group_additions":source_groups,
        "interpretation":"full in-process cold producer-only, no external PKM/APS/CAS or PCC",
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


// New preregistered 24-public-Q matched complete-solver test. The candidate
// precomputes only S3 x-fibre square roots and uses exactly the same labelled
// public walk source, output checks, and finite S3 roots as the legacy arm.
// Both arms are generic MITM-equivalent APS controls, not a new non-generic IC.
fn s3_xfibre_confirmation()->Result<serde_json::Value,String>{
    const CASES:usize=24;
    const PROFILE:[usize;3]=[384,384,16_384];
    const CAP:usize=6;
    let mut rng=0x5846_4942_5245_3234u64;
    let mut used=HashSet::new();
    let mut verifier=Arithmetic::default();
    let mut totals=[0u128;3];
    let mut sqrts=[0u64;3];
    let mut queries=[0u64;3];
    let mut wins=0usize;
    let mut cases=Vec::with_capacity(CASES);
    for case in 0..CASES {
        let known=loop{
            let k=prng(&mut rng)%N;
            if k>1 && used.insert(k){break k;}
        };
        let q=verifier.scalar(PRIME31,known,Some(PRIME31.g))
            .ok_or("xfibre public target identity")?;
        let order=if case%2==0{
            [(0usize,false),(1usize,true),(2usize,true)]
        }else{
            [(1usize,true),(2usize,true),(0usize,false)]
        };
        let mut results=Vec::with_capacity(3);
        let mut time=[0u64;3];
        let mut trial_pairs=[0u64;3];
        let mut rounds=[0usize;3];
        let mut logs=[0u64;3];
        for (position,(arm,xfibre)) in order.into_iter().enumerate(){
            let v=execute_unequal_profile_variant(q,PROFILE,CAP,xfibre)?;
            if v["complete_target_scalar_recovered"].as_bool()!=Some(true){
                return Err(format!("xfibre arm {arm} failed bounded complete solve Q index {case}"));
            }
            let attempts=v["attempts"].as_array().ok_or("xfibre missing attempts")?;
            let last=attempts.last().ok_or("xfibre empty attempts")?;
            let k=last.pointer("/solution/log").and_then(|r|r.as_u64())
                .ok_or("xfibre no recovered target scalar")?;
            if k!=known || verifier.scalar(PRIME31,k,Some(PRIME31.g))!=Some(q){
                return Err(format!("xfibre arm {arm} incorrect complete Q at {case}"));
            }
            let elapsed=v["cold_wall_ns"].as_u64().ok_or("xfibre missing cold wall")?;
            let pair_count=attempts.iter()
                .map(|r|r["s3_pair_quadratics"].as_u64().unwrap_or(0)).sum::<u64>();
            let sqrt_count=attempts.iter()
                .map(|r|r["s3_sqrts"].as_u64().unwrap_or(0)).sum::<u64>();
            time[arm]=elapsed;trial_pairs[arm]=pair_count;
            rounds[arm]=attempts.len();logs[arm]=k;
            totals[arm]+=u128::from(elapsed);
            sqrts[arm]+=sqrt_count;queries[arm]+=pair_count;
            results.push(json!({
                "arm":match arm{0=>"legacy_s3_sqrt",1=>"xfibre",_=>"xfibre_repeat"},
                "position":position,"x_fibre_precompute":xfibre,
                "recovered_scalar":k,"verified":true,
                "cold_wall_ns":elapsed,"rounds":attempts.len(),
                "s3_pair_quadratics":pair_count,"field_square_roots":sqrt_count,
                "attempts":attempts
            }));
        }
        if trial_pairs[0]!=trial_pairs[1] || trial_pairs[1]!=trial_pairs[2]
            || rounds[0]!=rounds[1] || rounds[1]!=rounds[2]
            || logs[0]!=logs[1] || logs[1]!=logs[2] {
            return Err(format!("xfibre changed S3 search/output semantics at {case}"));
        }
        if time[1]<time[0]{wins+=1;}
        cases.push(json!({
            "case":case,"public_target":q,
            "known_answer_caller_verifier_only":known,
            "outcomes":results
        }));
    }
    let ratio=totals[1] as f64 / totals[0] as f64;
    let drift=((totals[2] as f64 / totals[1] as f64)-1.0).abs();
    let passes=ratio<=0.8 && drift<=0.05 && wins>=18;
    Ok(json!({
        "standing":"[pcc:unverified]",
        "curve_p":P,"curve_n":N,"cofactor":1,
        "method":"APS S3 x-only discriminant fibre precompute versus per-pair square root",
        "classification":"generic pair-MITM equivalent, no new non-generic ECDLP",
        "fresh_target_count":CASES,"profile":PROFILE,"round_cap":CAP,
        "complete_verified_solves_per_arm":[CASES,CASES,CASES],
        "arms":["legacy_s3_sqrt","xfibre","xfibre_repeat"],
        "cold_total_ns":totals.map(|n|n.to_string()),
        "field_square_roots":sqrts,"pair_s3_quadratics":queries,
        "candidate_over_legacy_ratio":ratio,"candidate_repeat_drift":drift,
        "candidate_paired_wins":wins,
        "passes_preregistered_development_gate":passes,
        "no_source_y_or_group_additions_used_for_root_discovery":true,
        "limitations":"development CI, nonisolated, no mature external IC/PCC, not asymptotic",
        "cases_detail":cases
    }))
}

// Frozen 24-target matched cold solve: source only differs in arithmetic.
// Exact per-round semantic counters and final scalar must match all arms.
// Negative speed result is retained as a valid completed experiment.
fn jacobian_batch_confirmation()->Result<serde_json::Value,String>{
    const CASES:usize=24;
    const PROFILE:[usize;3]=[384,384,16_384];
    let mut rng=0x4a41_4342_5f32_3431u64;
    let mut used=HashSet::new();
    let mut verifier=Arithmetic::default();
    let mut totals=[0u128;3];
    let mut setup_costs=[0u128;3];
    let mut queries=[0u64;3];
    let mut wins=0usize;
    let mut cases=Vec::new();
    for case in 0..CASES{
        let known=loop{
            let k=prng(&mut rng)%N;
            if k>1 && used.insert(k){break k;}
        };
        let q=verifier.scalar(PRIME31,known,Some(PRIME31.g))
            .ok_or("Jacobian panel public target is identity")?;
        let order=if case%2==0 {[0usize,1,2]}else{[1usize,2,0]};
        let mut times=[0u64;3];
        let mut expected_transcript:Option<Vec<serde_json::Value>>=None;
        let mut results=Vec::new();
        for (position,&arm) in order.iter().enumerate(){
            let run=execute_unequal_profile_source(q,PROFILE,6,true,arm!=0)?;
            if run["complete_target_scalar_recovered"].as_bool()!=Some(true){
                return Err(format!("Jacobian arm {arm} missed public Q {case}"));
            }
            let attempts=run["attempts"].as_array().ok_or("missing attempt transcript")?;
            let log=attempts.last().and_then(|a|a.pointer("/solution/log"))
                .and_then(|v|v.as_u64()).ok_or("missing recovered scalar")?;
            if log!=known || verifier.scalar(PRIME31,log,Some(PRIME31.g))!=Some(q){
                return Err(format!("incorrect Jacbatch scalar on case {case} arm {arm}"));
            }
            let sem:Vec<_>=attempts.iter().map(|a|json!({
                "seed":a["seed"],"solution":a["solution"],
                "s3_pair_quadratics":a["s3_pair_quadratics"],
                "s3_sqrts":a["s3_sqrts"],
                "s3_batch_inversions":a["s3_batch_inversions"],
                "signed_candidates":a["signed_candidates"],
                "rejected_identity":a["rejected_identity"],
                "rejected_duplicate_x":a["rejected_duplicate_x"]
            })).collect();
            if let Some(ref expected)=expected_transcript{
                if expected!=&sem{
                    return Err(format!("Jacbatch changed relation transcript on case {case} arm {arm}"));
                }
            }else{expected_transcript=Some(sem);}
            let ns=run["cold_wall_ns"].as_u64().ok_or("missing full cold timing")?;
            let source_ns=attempts.iter().map(|a|
                a["source_setup_ns"].as_u64().unwrap_or(0) as u128).sum::<u128>();
            let pair_queries=attempts.iter().map(|a|
                a["s3_pair_quadratics"].as_u64().unwrap_or(0)).sum::<u64>();
            times[arm]=ns;
            totals[arm]+=u128::from(ns);
            setup_costs[arm]+=source_ns;
            queries[arm]+=pair_queries;
            results.push(json!({
                "arm":match arm {0=>"affine",1=>"jacobian_batch",_=>"jacobian_repeat"},
                "position":position,"verified_complete_scalar":log,
                "full_cold_ns":ns,"paid_source_ns":source_ns.to_string(),
                "s3_pair_queries":pair_queries,"attempts":attempts
            }));
        }
        if times[1]<times[0]{wins+=1;}
        cases.push(json!({
            "case":case,"public_target":q,
            "known_scalar_caller_verifier_only":known,"arms":results
        }));
    }
    let ratio=totals[1] as f64/totals[0] as f64;
    let drift=((totals[2] as f64/totals[1] as f64)-1.0).abs();
    let passed=ratio<=0.90 && drift<=0.05 && wins>=18;
    Ok(json!({
        "standing":"[pcc:unverified]","curve_field":P,"group_order":N,"cofactor":1,
        "method":"APS S3 one-relation plus public Jacobian source batch normalization",
        "classification":"index calculus methodology, generic MITM equivalent",
        "full_cold_in_process":true,"fresh_public_targets":CASES,
        "profile":PROFILE,"round_cap":6,
        "all_72_scalars_correct":true,"exact_source_and_relation_transcripts":true,
        "arms":["affine","jacobian_batch","jacobian_repeat"],
        "cold_total_ns":totals.map(|n|n.to_string()),
        "source_total_ns":setup_costs.map(|n|n.to_string()),
        "S3_pair_quadratics":queries,"candidate_over_control":ratio,
        "candidate_aa_drift":drift,"candidate_paired_wins":wins,
        "passes_preregistered_development_gate":passed,
        "no_mature_external_or_independent_PCC":true,
        "cases_detail":cases
    }))
}


// EXP-R31-APS-POST-JAC: prospective *full-cold* source-pinned profile screen.
// Source generator, field arithmetic and target recovery reuse the verified
// exact-source Jacobian+x-fibre Rust implementation. This is not a new
// cryptanalytic complexity class, and no cost is derived from censored F4.
const POST_JAC_PROFILES:[[usize;3];5]=[
    [256,256,32_768],[320,320,24_576],[384,384,16_384],
    [512,512,8_192],[384,384,16_384],
];
const POST_JAC_NAMES:[&str;5]=[
    "A_256_256_32768","B_320_320_24576","C_384_384_16384",
    "D_512_512_8192","C_repeat",
];
fn post_jacobian_partition_screen()->Result<serde_json::Value,String>{
    const CASES:usize=32;
    let mut rng=0x5052_4f46_5f4a_3332u64;
    let mut used=HashSet::new();
    let mut verifier=Arithmetic::default();
    check_curve(PRIME31,&mut verifier)?;
    let mut totals=[0u128;5];
    let mut setup_costs=[0u128;5];
    let mut queries=[0u64;5];
    let mut correct=[0usize;5];
    let mut successful=[0usize;5];
    let mut paired_wins=[0usize;5];
    let mut cases=Vec::with_capacity(CASES);
    for case in 0..CASES{
        let known=loop{
            let k=prng(&mut rng)%N;
            if k>1 && used.insert(k){break k;}
        };
        let q=verifier.scalar(PRIME31,known,Some(PRIME31.g))
            .ok_or("post-Jac panel public Q is identity")?;
        let order:[usize;5]=std::array::from_fn(|j|(j+case%5)%5);
        let mut times=[0u64;5];
        let mut recovered=[false;5];
        let mut arms=Vec::with_capacity(5);
        for (position,&arm) in order.iter().enumerate(){
            let run=execute_unequal_profile_source(q,POST_JAC_PROFILES[arm],6,true,true)?;
            let attempts=run["attempts"].as_array()
                .ok_or("post-Jac profile attempt array missing")?;
            let completed=run["complete_target_scalar_recovered"].as_bool()==Some(true);
            let log=attempts.last().and_then(|a|a.pointer("/solution/log"))
                .and_then(|v|v.as_u64());
            if completed {
                let scalar=log.ok_or("post-Jac completed but scalar missing")?;
                if scalar!=known ||
                    verifier.scalar(PRIME31,scalar,Some(PRIME31.g))!=Some(q){
                    return Err(format!("post-Jac incorrect log on Q {case}, arm {arm}"));
                }
                correct[arm]+=1;
                successful[arm]+=1;
            }else if log.is_some(){
                return Err(format!("post-Jac contradictory missed Q {case}, arm {arm}"));
            }
            recovered[arm]=completed;
            let ns=run["cold_wall_ns"].as_u64()
                .ok_or("post-Jac solver cold wall missing")?;
            let source_ns=attempts.iter().map(|a|
                a["source_setup_ns"].as_u64().unwrap_or(0) as u128).sum::<u128>();
            let pair_queries=attempts.iter().map(|a|
                a["s3_pair_quadratics"].as_u64().unwrap_or(0)).sum::<u64>();
            times[arm]=ns;
            totals[arm]+=u128::from(ns);
            setup_costs[arm]+=source_ns;
            queries[arm]+=pair_queries;
            arms.push(json!({
                "arm_index":arm,"arm":POST_JAC_NAMES[arm],"profile":POST_JAC_PROFILES[arm],
                "position":position,"complete":completed,"recovered_log":log,
                "full_cold_ns":ns,"source_total_ns":source_ns.to_string(),
                "s3_pair_queries":pair_queries,"all_paid_rounds":attempts
            }));
        }
        for arm in [0usize,1,3]{
            if recovered[arm] && recovered[2] && times[arm]<times[2]{
                paired_wins[arm]+=1;
            }
        }
        cases.push(json!({
            "case":case,"public_target":q,
            "known_scalar_caller_verifier_only":known,
            "chronological_arms":arms
        }));
    }
    let baseline_median=(totals[2]+totals[4]) as f64/2.0;
    let candidate=[0usize,1,3].into_iter().min_by_key(|&i|totals[i])
        .ok_or("post-Jac candidate set empty")?;
    let ratio=totals[candidate] as f64/baseline_median;
    let aa_drift=(totals[4] as f64/totals[2] as f64-1.0).abs();
    let all_complete=successful.iter().all(|&s|s==CASES);
    let admissible=all_complete && aa_drift<=0.05 &&
        ratio<=0.90 && paired_wins[candidate]>=24;
    Ok(json!({
        "standing":"[pcc:unverified]",
        "experiment":"prospective post-Jacobian full-cold unequal APS profile screen",
        "classification":"published APS one-relation IC, explicit S3 pair search generic-MITM equivalent",
        "field_prime":P,"group_order":N,"cofactor":1,"cases":CASES,
        "public_target_seed":"0x50524f465f4a3332","round_cap":6,
        "full_cold_per_target":true,"all_failed_rounds_charged":true,
        "source_mode":"Jacobian mixed add and batch normalization",
        "relation_mode":"Mersenne S3 x-fibre roots, signed replay",
        "profiles":POST_JAC_PROFILES,"arm_names":POST_JAC_NAMES,
        "verified_logs_per_arm":correct,"completed_solves_per_arm":successful,
        "all_complete":all_complete,
        "cold_total_ns":totals.map(|n|n.to_string()),
        "source_total_ns":setup_costs.map(|n|n.to_string()),
        "s3_pair_quadratics":queries,
        "paired_wins_against_incumbent":paired_wins,
        "best_candidate_arm":POST_JAC_NAMES[candidate],
        "best_candidate_ratio_to_aa_mean":ratio,
        "incumbent_aa_drift":aa_drift,
        "passes_preregistered_exploratory_gate":admissible,
        "not_an_independent_PCC_or_external_IC_comparison":true,
        "cases_detail":cases
    }))
}


// EXP-R31-S3-FACTORIZED (preregistered): x-only fibre chord-coordinate
// roots of S3(x,z,U), with no public ordinates or EC group operations in
// candidate discovery. This is classical E arithmetic and an explicitly
// enumerated pair support: generic-MITM-equivalent, not novel algebraic IC.
// Distinct abscissae are a checked invariant of the labelled source.
fn algebraic_mode_factorized(
    parts:&[Vec<Labelled>;3],q:(u64,u64),ar:&mut Arithmetic
)->Result<(Option<Solution>,Stats),String>{
    let lookup=reverse_index(&parts[2]);
    let mut stats=Stats::default();
    let mut fibres:[Vec<u64>;2]=std::array::from_fn(|j|
        Vec::with_capacity(parts[j].len()));
    for j in 0..2 {
        for point in &parts[j] {
            // Only the abscissa is read; signs of fibre square roots do
            // not affect the unordered pair of S3 coordinate roots.
            let rhs=rhs_from_x::<true>(point.pt.0);
            let y=sqrtp_mode::<true>(rhs)
                .ok_or("factorized S3: public x has no rational fibre")?;
            if mulp_mersenne(y,y)!=rhs {
                return Err("factorized S3: invalid fibre square root".into());
            }
            fibres[j].push(y);
            stats.s3_sqrt+=1;
        }
    }
    for (i,left) in parts[0].iter().enumerate(){
        let x=left.pt.0;
        let mut denoms=Vec::with_capacity(parts[1].len());
        let mut prefix=Vec::with_capacity(parts[1].len()+1);
        prefix.push(1u64);
        for right in &parts[1]{
            let z=right.pt.0;
            if x==z {
                // x=z needs tangent/identity exceptional branches. The
                // globally x-unique source must make this unreachable.
                return Err("factorized S3: repeated x across partitions".into());
            }
            let diff=subp(x,z);
            let d=mulp_mersenne(diff,diff);
            if d==0 {return Err("factorized S3: zero denominator".into());}
            prefix.push(mulp_mersenne(*prefix.last()
                .ok_or("factorized S3: missing prefix")?,d));
            denoms.push(d);
            stats.pair_s3+=1;
        }
        let mut total_inverse=pownp_mode::<true>(
            *prefix.last().ok_or("factorized S3: empty prefix")?,P-2);
        stats.batch_inversions+=1;
        let mut inverses=vec![0u64;denoms.len()];
        for j in (0..denoms.len()).rev(){
            inverses[j]=mulp_mersenne(total_inverse,prefix[j]);
            total_inverse=mulp_mersenne(total_inverse,denoms[j]);
        }
        if total_inverse!=1 {
            return Err("factorized S3: batch inversion invariant".into());
        }
        let u=fibres[0][i];
        for (j,right) in parts[1].iter().enumerate(){
            let z=right.pt.0;
            let v=fibres[1][j];
            let sum_x=addp(x,z);
            let plus=addp(u,v);
            let minus=subp(u,v);
            let mut roots=[
                subp(mulp_mersenne(mulp_mersenne(plus,plus),inverses[j]),sum_x),
                subp(mulp_mersenne(mulp_mersenne(minus,minus),inverses[j]),sum_x)
            ];
            roots.sort_unstable();
            for (ridx,&root) in roots.iter().enumerate(){
                if ridx==1 && root==roots[0] {continue;}
                if let Some(candidates)=lookup.get(&root){
                    for &k in candidates {
                        stats.matched_x+=1;
                        if let Some(solution)=verify_candidate(
                            left,right,&parts[2][k],[i,j,k],q,ar,&mut stats
                        ){return Ok((Some(solution),stats));}
                    }
                }
            }
        }
    }
    Ok((None,stats))
}

fn execute_factorized_unequal(q:(u64,u64),sizes:[usize;3],round_limit:usize)
    ->Result<serde_json::Value,String>{
    if sizes.iter().any(|&n|n<16 || n>32_768)
       || sizes.iter().sum::<usize>()>40_000
       || round_limit==0 || round_limit>8{
        return Err("factorized S3 admission limit".into());
    }
    let cold=Instant::now();
    let mut ar=Arithmetic::default();
    check_curve(PRIME31,&mut ar)?;
    if !ar.on_curve(PRIME31,Some(q)) || q==PRIME31.g{
        return Err("factorized S3 invalid public target".into());
    }
    let mut attempts=Vec::new();
    for round in 0..round_limit{
        let seed=SEED.wrapping_add((round as u64)
            .wrapping_mul(0xd1b5_4a32_d192_ed03));
        let begin=Instant::now();
        let before=ar.counts.group_additions;
        let (parts,identities,duplicates)=
            source_walk_profile_jac(q,seed,&mut ar,sizes)?;
        let source_ns=begin.elapsed().as_nanos();
        let source_additions=ar.counts.group_additions-before;
        let root_begin=Instant::now();
        let before_verify=ar.counts.group_additions;
        let (solution,stats)=algebraic_mode_factorized(&parts,q,&mut ar)?;
        let root_ns=root_begin.elapsed().as_nanos();
        let verify_additions=ar.counts.group_additions-before_verify;
        let found=solution.is_some();
        attempts.push(json!({
            "round":round,"seed":format!("{seed:016x}"),"profile":sizes,
            "source_setup_ns":source_ns,
            "source_group_additions":source_additions,
            "rejected_identity":identities,
            "rejected_duplicate_x":duplicates,
            "s3_query_ns":root_ns,
            "s3_pair_quadratics":stats.pair_s3,
            "x_fibre_precompute":true,"jacobian_source":true,
            "s3_sqrts":stats.s3_sqrt,
            "s3_batch_inversions":stats.batch_inversions,
            "signed_candidates":stats.signed_candidates,
            "verification_group_additions":verify_additions,
            "solution":json_solution(&solution),
            "elapsed_cold_ns":cold.elapsed().as_nanos()
        }));
        if found{
            return Ok(json!({
                "standing":"[pcc:unverified]",
                "method":"published APS one relation; factored x-fibre S3 roots",
                "classification":"generic decomposition / explicit pair support",
                "p":P,"n":N,"cofactor":1,"public_Q":q,
                "profile":sizes,"round_cap":round_limit,
                "complete_target_scalar_recovered":true,
                "known_answer_in_solver":false,
                "cold_wall_ns":cold.elapsed().as_nanos(),
                "attempts":attempts
            }));
        }
    }
    Ok(json!({
        "standing":"[pcc:unverified]",
        "method":"published APS one relation; factored x-fibre S3 roots",
        "classification":"generic decomposition / explicit pair support",
        "p":P,"n":N,"cofactor":1,"public_Q":q,
        "profile":sizes,"round_cap":round_limit,
        "complete_target_scalar_recovered":false,
        "status":"bounded_exhaustion",
        "cold_wall_ns":cold.elapsed().as_nanos(),
        "attempts":attempts
    }))
}

fn factorized_s3_confirmation_seed(target_seed:u64)->Result<serde_json::Value,String>{
    const CASES:usize=24;
    const PROFILE:[usize;3]=[384,384,16_384];
    let mut rng=target_seed;
    let mut used=HashSet::new();
    let mut verifier=Arithmetic::default();
    check_curve(PRIME31,&mut verifier)?;
    let mut totals=[0u128;3];
    let mut sources=[0u128;3];
    let mut queries=[0u64;3];
    let mut completed=[0usize;3];
    let mut paired_wins=0usize;
    let mut cases=Vec::with_capacity(CASES);
    for case in 0..CASES{
        let known=loop{
            let k=prng(&mut rng)%N;
            if k>1 && used.insert(k){break k;}
        };
        let q=verifier.scalar(PRIME31,known,Some(PRIME31.g))
            .ok_or("factorized S3 fixture Q identity")?;
        let order:[usize;3]=std::array::from_fn(|i|(i+case%3)%3);
        let mut times=[0u64;3];
        let mut reference:Option<Vec<serde_json::Value>>=None;
        let mut arms=Vec::with_capacity(3);
        for (position,&arm) in order.iter().enumerate(){
            let run=if arm==0{
                execute_unequal_profile_source(q,PROFILE,6,true,true)?
            }else{
                execute_factorized_unequal(q,PROFILE,6)?
            };
            let attempts=run["attempts"].as_array()
                .ok_or("factorized S3 missing attempt array")?;
            let complete=run["complete_target_scalar_recovered"].as_bool()==Some(true);
            let found=attempts.last().and_then(|v|v.pointer("/solution/log"))
                .and_then(|v|v.as_u64());
            if !complete || found!=Some(known)
              || verifier.scalar(PRIME31,found.unwrap_or(0),Some(PRIME31.g))!=Some(q){
                return Err(format!("factorized S3 failed complete public log, case={case}, arm={arm}"));
            }
            let signatures=attempts.iter().map(|a|json!({
                "round":a["round"],"seed":a["seed"],
                "rejected_identity":a["rejected_identity"],
                "rejected_duplicate_x":a["rejected_duplicate_x"],
                "s3_pair_quadratics":a["s3_pair_quadratics"],
                "s3_sqrts":a["s3_sqrts"],
                "s3_batch_inversions":a["s3_batch_inversions"],
                "signed_candidates":a["signed_candidates"],
                "solution":a["solution"]
            })).collect::<Vec<_>>();
            if let Some(ref incumbent)=reference{
                if &signatures!=incumbent{
                    return Err(format!("factorized S3 changed the exact signed relation transcript, case={case}, arm={arm}"));
                }
            }else{
                reference=Some(signatures);
            }
            completed[arm]+=1;
            let t=run["cold_wall_ns"].as_u64()
                .ok_or("factorized S3 missing cold wall")?;
            times[arm]=t;
            totals[arm]+=t as u128;
            let source=attempts.iter().map(|a|
                a["source_setup_ns"].as_u64().unwrap_or(0) as u128).sum::<u128>();
            sources[arm]+=source;
            let query=attempts.iter().map(|a|
                a["s3_pair_quadratics"].as_u64().unwrap_or(0)).sum::<u64>();
            queries[arm]+=query;
            arms.push(json!({
                "arm":arm,"position":position,
                "full_cold_ns":t,"source_total_ns":source.to_string(),
                "pair_queries":query,"recovered_log":found,
                "all_paid_rounds":attempts
            }));
        }
        if times[1]<times[0]{paired_wins+=1;}
        cases.push(json!({
            "case":case,"public_target":q,
            "known_scalar_caller_verifier_only":known,
            "chronological_arms":arms
        }));
    }
    let drift=(totals[2] as f64/totals[1] as f64-1.0).abs();
    let candidate_ratio=totals[1] as f64/totals[0] as f64;
    let passes=completed==[CASES;3] && drift<=0.05
        && candidate_ratio<=0.90 && paired_wins>=18;
    Ok(json!({
        "standing":"[pcc:unverified]",
        "experiment":"prospective same-host prime31 S3 chord-coordinate factorization",
        "classification":"generic decomposition: published APS one-relation with MITM equivalent pair enumeration",
        "p":P,"n":N,"cofactor":1,"profile":PROFILE,
        "cases":CASES,"round_cap":6,
        "public_seed":format!("{target_seed:#018x}"),
        "arms":["old_S3_coefficient_xfibre","new_factorized_xfibre","new_factorized_repeat"],
        "complete_cold_per_target":true,
        "all_failed_rounds_charged":true,
        "identical_signed_relation_transcripts":true,
        "completed_solves_per_arm":completed,
        "cold_total_ns":totals.map(|t|t.to_string()),
        "source_total_ns":sources.map(|t|t.to_string()),
        "s3_pair_quadratics":queries,
        "candidate_ratio_to_incumbent":candidate_ratio,
        "candidate_aa_drift":drift,
        "candidate_paired_wins":paired_wins,
        "passes_preregistered_exploratory_gate":passes,
        "requires_fresh_confirmation_and_independent_pcc":true,
        "cases_detail":cases
    }))
}


fn factorized_s3_fresh_confirmation()->Result<serde_json::Value,String>{
    // Caller-only known answers. Verify the preregistered fresh panel
    // does not repeat the exploratory public Q scalars before a single
    // solver call; no k crosses the executor boundary.
    let mut prior_rng=0x4633_4348_4f52_4430u64;
    let mut prior=HashSet::new();
    while prior.len()<24{
        let k=prng(&mut prior_rng)%N;
        if k>1{prior.insert(k);}
    }
    let mut fresh_rng=0x5333_5f43_4f4e_4631u64;
    let mut fresh=HashSet::new();
    while fresh.len()<24{
        let k=prng(&mut fresh_rng)%N;
        if k>1{fresh.insert(k);}
    }
    if !prior.is_disjoint(&fresh) {
        return Err("fresh S3 panel overlaps exploratory known answers".into());
    }
    let mut report=factorized_s3_confirmation_seed(0x5333_5f43_4f4e_4631u64)?;
    report["panel"]=json!("independent prospectively seeded 24-Q confirmation");
    report["overlap_with_prior_24_Q"]=json!(0u64);
    Ok(report)
}


// Complete generic BSGS comparator: a fresh table for *each* public target.
// It shares the tested Rust Mersenne/Jacobian arithmetic with the APS source,
// but neither its table nor its giant walk is used for algebraic discovery.
fn cold_batched_bsgs(q:(u64,u64))->Result<serde_json::Value,String>{
    let cold=Instant::now();
    let mut ar=Arithmetic::default();
    check_curve(PRIME31,&mut ar)?;
    if !ar.on_curve(PRIME31,Some(q)){
        return Err("BSGS comparator: invalid public Q".into());
    }
    let mut m=1u64;
    while m*m<N {m+=1;}
    if (m-1)*(m-1)>=N || m*m<N{
        return Err("BSGS comparator: wrong ceil sqrt(n)".into());
    }
    const BATCH:usize=256;
    let mut baby=HashMap::with_capacity(m as usize);
    let mut cur=jac_affine(PRIME31.g);
    let mut block=Vec::with_capacity(BATCH);
    let mut baby_batches=0u64;
    for j in 1..m{
        block.push((cur,j,0u64));
        if block.len()==BATCH || j+1==m {
            for item in normalize_jac_batch(&block)?{
                if baby.insert(item.pt,item.a).is_some(){
                    return Err("BSGS comparator: baby collision before group order".into());
                }
            }
            baby_batches+=1;
            block.clear();
        }
        if j+1<m {cur=jac_mixed(cur,PRIME31.g);}
    }
    if baby.len() as u64 !=m-1{
        return Err("BSGS comparator: incomplete baby table".into());
    }
    let build_ns=cold.elapsed().as_nanos();
    let step=ar.scalar(PRIME31,m,Some(PRIME31.g))
        .ok_or("BSGS comparator: [m]G identity")?;
    let neg_step=ar.neg(PRIME31,Some(step))
        .ok_or("BSGS comparator: negative giant step identity")?;
    let giant_begin=Instant::now();
    let mut giant=jac_affine(q);
    let mut giant_count=0u64;
    let mut giant_batches=0u64;
    let mut recovered=None;
    let mut giant_block=Vec::with_capacity(BATCH);
    for i in 0..=m{
        giant_count+=1;
        if giant.z==0{
            recovered=Some((i*m)%N);
            break;
        }
        giant_block.push((giant,i,0u64));
        if giant_block.len()==BATCH || i==m{
            for item in normalize_jac_batch(&giant_block)?{
                if let Some(&j)=baby.get(&item.pt){
                    recovered=Some((item.a*m+j)%N);
                    break;
                }
            }
            giant_batches+=1;
            giant_block.clear();
            if recovered.is_some(){break;}
        }
        if i<m {giant=jac_mixed(giant,neg_step);}
    }
    let k=recovered.ok_or("BSGS comparator: no collision in full search")?;
    // This check is internal to BSGS and does not use the known-answer k.
    if ar.scalar(PRIME31,k,Some(PRIME31.g))!=Some(q){
        return Err("BSGS comparator: reconstructed scalar replay mismatch".into());
    }
    Ok(json!({
        "standing":"[pcc:unverified]",
        "method":"classical Shanks BSGS, mixed Jacobian and batch-normalized",
        "p":P,"n":N,"cofactor":1,"public_Q":q,
        "complete_target_scalar_recovered":true,"known_answer_in_solver":false,
        "log":k,"ceil_sqrt_n":m,"baby_count":baby.len(),
        "baby_normalization_batches":baby_batches,
        "giant_steps_visited":giant_count,
        "giant_normalization_batches":giant_batches,
        "cold_baby_setup_ns":build_ns,
        "cold_giant_search_and_replay_ns":giant_begin.elapsed().as_nanos(),
        "cold_wall_ns":cold.elapsed().as_nanos()
    }))
}

// Matched complete cold costs, not a win-only or postselected comparison.
// The verifier-only answers never enter either algorithm.
fn compare_factorized_aps_bsgs_cold()->Result<serde_json::Value,String>{
    const CASES:usize=24;
    const SEED_TARGET:u64=0x4247_5353_434f_4c44;
    const PROFILE:[usize;3]=[384,384,16_384];
    let mut rng=SEED_TARGET;
    let mut used=HashSet::new();
    let mut verifier=Arithmetic::default();
    check_curve(PRIME31,&mut verifier)?;
    let mut totals=[0u128;3];
    let mut wins=[0usize;2];
    let mut results=Vec::with_capacity(CASES);
    for c in 0..CASES{
        let (known,q)=loop{
            let k=prng(&mut rng)%N;
            if k<=1 || !used.insert(k){continue;}
            if let Some(point)=verifier.scalar(PRIME31,k,Some(PRIME31.g)){
                break (k,point);
            }
        };
        let order: [usize;3]=match c%3{
            0=>[0,1,2],1=>[1,2,0],_=>[2,0,1]
        };
        let mut arms:[serde_json::Value;3]=std::array::from_fn(|_|json!(null));
        for idx in order{
            let value=match idx{
                0=>execute_factorized_unequal(q,PROFILE,6)?,
                1|2=>cold_batched_bsgs(q)?,
                _=>return Err("invalid APS/BSGS arm".into())
            };
            if value["complete_target_scalar_recovered"].as_bool()!=Some(true){
                return Err(format!("APS/BSGS incomplete solve at case {c}, arm {idx}"));
            }
            let found=if idx==0{
                value["attempts"].as_array()
                    .and_then(|a|a.last())
                    .and_then(|a|a["solution"]["log"].as_u64())
                    .ok_or("APS complete but missing result log")?
            }else{
                value["log"].as_u64().ok_or("BSGS missing result log")?
            };
            if found!=known || verifier.scalar(PRIME31,found,Some(PRIME31.g))!=Some(q){
                return Err(format!("APS/BSGS wrong scalar at case {c} arm {idx}"));
            }
            totals[idx]+=value["cold_wall_ns"].as_u64()
                .ok_or("cold ns not u64")? as u128;
            arms[idx]=value;
        }
        if arms[0]["cold_wall_ns"].as_u64()<arms[1]["cold_wall_ns"].as_u64(){
            wins[0]+=1;
        }else{
            wins[1]+=1;
        }
        results.push(json!({
            "case":c,"Q":q,"caller_known_scalar_verifier_only":known,
            "rotated_arm_order":order,
            "aps_s3_factorized_full_receipt":arms[0],
            "bsgs_full_cold":arms[1],
            "bsgs_full_cold_repeat":arms[2]
        }));
    }
    Ok(json!({
        "standing":"[pcc:unverified]",
        "protocol":"APS_VS_BSGS_COLD_PROTOCOL_20261116.md",
        "target_seed":format!("{SEED_TARGET:016x}"),
        "p":P,"n":N,"cofactor":1,"profile":PROFILE,
        "case_count":CASES,"total_verified_logs":CASES*3,
        "method_order":["factorized_APS_IC","batched_BSGS","batched_BSGS_AA"],
        "aggregate_fully_cold_ns":totals,
        "aps_per_target_wins":wins[0],
        "bsgs_per_target_wins":wins[1],
        "interpretation":"matched source-specific developer costs; BSGS generic, APS finite S3 pair MITM-equivalent; independent PCC/rho/CAS not assessed",
        "cases":results
    }))
}

fn main()->Result<(),String> {
    let args:Vec<String>=std::env::args().skip(1).collect();
    if args.len()==1 && args[0]=="--aps-vs-bsgs-cold" {
        let result=compare_factorized_aps_bsgs_cold()?;
        println!("{}",serde_json::to_string_pretty(&result).map_err(|e|e.to_string())?);
        return Ok(());
    }
    if args.len()==1 && args[0]=="--factorized-s3-fresh" {
        let result=factorized_s3_fresh_confirmation()?;
        println!("{}",serde_json::to_string_pretty(&result).map_err(|e|e.to_string())?);
        return Ok(());
    }
    if args.len()==1 && args[0]=="--factorized-s3-confirm" {
        let result=factorized_s3_confirmation_seed(0x4633_4348_4f52_4430u64)?;
        println!("{}",serde_json::to_string_pretty(&result).map_err(|e|e.to_string())?);
        return Ok(());
    }
    if args.len()==1 && args[0]=="--post-jac-profile" {
        let result=post_jacobian_partition_screen()?;
        println!("{}",serde_json::to_string_pretty(&result).map_err(|e|e.to_string())?);
        return Ok(());
    }
    if args.len()==1 && args[0]=="--jacbatch-confirm" {
        let result=jacobian_batch_confirmation()?;
        println!("{}",serde_json::to_string_pretty(&result).map_err(|e|e.to_string())?);
        // Incomplete or wrong recovery fails; slower valid results are
        // recorded as a negative benchmark, not silently discarded.
        return Ok(());
    }
    if args.len()==1 && args[0]=="--xfibre-confirm" {
        let result=s3_xfibre_confirmation()?;
        println!("{}",serde_json::to_string_pretty(&result).map_err(|e|e.to_string())?);
        return Ok(());
    }
    if args.len()==1 && args[0]=="--unequal-384-confirm" {
        let result=unequal_384_confirmation()?;
        println!("{}",serde_json::to_string_pretty(&result).map_err(|e|e.to_string())?);
        if result["passes_preregistered_development_gate"].as_bool()!=Some(true){
            return Err("fresh unequal-384 APS confirmation gate not met".into());
        }
        return Ok(());
    }
    if args.len()==1 && args[0]=="--unequal-confirm" {
        let result=unequal_profile_confirmation()?;
        println!("{}",serde_json::to_string_pretty(&result).map_err(|e| e.to_string())?);
        if result["all_complete"].as_bool()!=Some(true) {
            return Err("not every unequal-profile candidate recovered its target".into());
        }
        return Ok(());
    }
    // Reproducible complete optimized Rust APS one-relation solver,
    // with all public labelled-walk setup, algebraic work and replay paid.
    if args.len()==1 && args[0]=="--walk-mersenne-one" {
        let value=execute_sized_mode_mul(false,ROUNDS,PUBLIC_Q,T,true,true)?;
        println!("{}",serde_json::to_string_pretty(&value).map_err(|e|e.to_string())?);
        if value["complete_target_scalar_recovered"].as_bool()!=Some(true) {
            return Err("optimized prime31 APS walk did not recover the public log".into());
        }
        return Ok(());
    }
    if args.len()==1 && args[0]=="--mersenne-confirm" {
        let result=mersenne_field_confirmation()?;
        println!("{}",serde_json::to_string_pretty(&result).map_err(|e|e.to_string())?);
        if result["all_success_gate"].as_bool()!=Some(true) {
            return Err("fresh Mersenne field panel did not complete every scalar".into());
        }
        return Ok(());
    }
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
        _ => return Err("usage: prime31_aps_one_relation --one|--compare|--holdouts|--size-sweep|--size-confirm|--walk-confirm|--mersenne-confirm|--walk-mersenne-one".into())
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
    fn post_jac_profile_admission_and_rotation() {
        for profile in POST_JAC_PROFILES {
            assert!(profile.iter().all(|&n|n>=16 && n<=32_768));
            assert!(profile.iter().sum::<usize>()<=40_000);
        }
        assert_eq!(POST_JAC_PROFILES[2],POST_JAC_PROFILES[4]);
        for case in 0..5usize {
            let mut order:[usize;5]=std::array::from_fn(|j|(j+case%5)%5);
            order.sort_unstable();
            assert_eq!(order,[0,1,2,3,4]);
        }
    }
    #[test]
    fn jacobian_mixed_and_batch_normalizer_match_affine() {
        let mut ar=Arithmetic::default();
        check_curve(PRIME31,&mut ar).unwrap();
        let g=PRIME31.g;
        let neg=ar.neg(PRIME31,Some(g)).unwrap();
        assert_eq!(jac_mixed(jac_affine(g),neg).z,0);
        let doubled=jac_double(jac_affine(g));
        assert_eq!(normalize_jac_batch(&[(doubled,1,2)]).unwrap()[0].pt,
            ar.add(PRIME31,Some(g),Some(g)).unwrap());
        let mut j=jac_affine(g);
        let mut expected=Some(g);
        let mut block=Vec::new();
        for k in 2..211u64{
            let step=ar.scalar(PRIME31,k,Some(g)).unwrap();
            j=jac_mixed(j,step);
            expected=ar.add(PRIME31,expected,Some(step));
            if let Some(pt)=expected{
                assert_eq!(normalize_jac_batch(&[(j,7,11)]).unwrap()[0].pt,pt);
                block.push((j,k,k+1));
            }else{
                assert_eq!(j.z,0);
                block.clear();
            }
        }
        for (source,normalized) in block.iter().zip(normalize_jac_batch(&block).unwrap()){
            assert_eq!(normalize_jac_batch(&[*source]).unwrap()[0].pt,normalized.pt);
        }
    }
    #[test]
    fn jacobian_source_matches_exact_affine_walk_labels() {
        let mut ar_a=Arithmetic::default();
        let mut ar_b=Arithmetic::default();
        for sizes in [[32,48,64],[384,384,2048]]{
            let original=source_walk_profile(PUBLIC_Q,SEED,&mut ar_a,sizes).unwrap();
            let candidate=source_walk_profile_jac(PUBLIC_Q,SEED,&mut ar_b,sizes).unwrap();
            assert_eq!((original.1,original.2),(candidate.1,candidate.2));
            for (p,q) in original.0.iter().zip(candidate.0.iter()){
                assert_eq!(p.len(),q.len());
                for (x,y) in p.iter().zip(q.iter()){
                    assert_eq!((x.pt,x.a,x.b),(y.pt,y.a,y.b));
                }
            }
        }
    }
    #[test]
    fn precomputed_s3_x_fibre_matches_all_signed_roots() {
        let mut ar=Arithmetic::default();
        check_curve(PRIME31,&mut ar).unwrap();
        let mut points=Vec::new();
        for i in 2..55u64 {
            points.push(ar.scalar(PRIME31,i,Some(PRIME31.g)).unwrap());
        }
        for &p in &points {
            let x=p.0;
            let rhs=rhs_from_x::<true>(x);
            let root=sqrtp_mode::<true>(rhs).unwrap();
            assert_eq!(mulp_mode::<true>(root,root),rhs);
            // The recorded source ordinate is used in this TEST only.
            assert_eq!(rhs,mulp_mode::<true>(p.1,p.1));
            let (a,b,c)=s3_coeff_mode::<true>(x,x);
            assert_eq!(a,0);
            assert_ne!(b,0);
            let double_x=mulp_mode::<true>(subp(0,c),pownp_mode::<true>(b,P-2));
            assert_eq!(ar.add(PRIME31,Some(p),Some(p)).unwrap().0,double_x);
        }
        for &p in &points {
            for &q in &points {
                if p.0==q.0 {continue;}
                let (a,b,c)=s3_coeff_mode::<true>(p.0,q.0);
                let disc=subp(mulp_mode::<true>(b,b),
                    mulp_mode::<true>(mulp_mode::<true>(4,a),c));
                let want=mulp_mode::<true>(16,mulp_mode::<true>(
                    rhs_from_x::<true>(p.0),rhs_from_x::<true>(q.0)));
                assert_eq!(disc,want);
                let yp=sqrtp_mode::<true>(rhs_from_x::<true>(p.0)).unwrap();
                let yq=sqrtp_mode::<true>(rhs_from_x::<true>(q.0)).unwrap();
                let xr=mulp_mode::<true>(4,mulp_mode::<true>(yp,yq));
                assert_eq!(mulp_mode::<true>(xr,xr),disc);
                let old=sqrtp_mode::<true>(disc).unwrap();
                let inv=pownp_mode::<true>(mulp_mode::<true>(2,a),P-2);
                let mut roots_old=[
                    mulp_mode::<true>(subp(old,b),inv),
                    mulp_mode::<true>(subp(subp(0,old),b),inv)
                ];
                let mut roots_new=[
                    mulp_mode::<true>(subp(xr,b),inv),
                    mulp_mode::<true>(subp(subp(0,xr),b),inv)
                ];
                roots_old.sort_unstable();roots_new.sort_unstable();
                assert_eq!(roots_old,roots_new);
                for z in roots_new {
                    assert_eq!(s3_eval_mode::<true>((a,b,c),z),0);
                }
            }
        }
    }

    #[test]
    fn unequal_walk_profiles_preserve_labelled_point_semantics() {
        let q=PUBLIC_Q;
        let mut ar=Arithmetic::default();
        let (parts,identities,_duplicates)=
            source_walk_profile(q,SEED,&mut ar,[32,48,64]).unwrap();
        assert_eq!(parts.iter().map(Vec::len).collect::<Vec<_>>(),vec![32,48,64]);
        assert!(identities<10);
        let all=parts.iter().flat_map(|v|v.iter().map(|pt|pt.pt.0))
            .collect::<HashSet<_>>();
        assert_eq!(all.len(),144);
        for point in parts.iter().flat_map(|v|v.iter()){
            let a=ar.scalar(PRIME31,point.a,Some(PRIME31.g));
            let b=ar.scalar(PRIME31,point.b,Some(q));
            assert_eq!(ar.add(PRIME31,a,b),Some(point.pt));
        }
        let result=execute_unequal_profile(q,[32,48,64],1).unwrap();
        assert_eq!(result["profile"],json!([32,48,64]));
        assert!(result["attempts"].as_array().is_some());
    }
    #[test]
    fn mersenne_field_mul_canonical_and_s3_equivalent() {
        let vals=[0,1,2,3,4,31,P/2,P-2,P-1];
        for &x in &vals {
            for &y in &vals {
                assert_eq!(mulp_mersenne(x,y),mulp(x,y));
            }
        }
        let mut seed=0x4d45_5253_5f50_3331u64;
        for i in 0..100_000u64 {
            let x=prng(&mut seed)%P;
            let y=prng(&mut seed)%P;
            assert_eq!(mulp_mersenne(x,y),mulp(x,y),"pair {i}");
            if i%101==0 {
                assert_eq!(s3_coeff_mode::<true>(x,y),s3_coeff_mode::<false>(x,y));
                let f=s3_coeff_mode::<true>(x,y);
                let z=prng(&mut seed)%P;
                assert_eq!(s3_eval_mode::<true>(f,z),s3_eval_mode::<false>(f,z));
            }
        }
        for &v in &vals {
            assert_eq!(sqrtp_mode::<true>(v),sqrtp_mode::<false>(v));
        }
    }
    #[test]
    fn mersenne_s3_preserves_complete_public_search_trace() {
        let slow=execute_sized_mode_mul(false,1,PUBLIC_Q,128,true,false).unwrap();
        let fast=execute_sized_mode_mul(false,1,PUBLIC_Q,128,true,true).unwrap();
        assert_eq!(slow["complete_target_scalar_recovered"],fast["complete_target_scalar_recovered"]);
        assert_eq!(slow["attempts"][0]["algebraic_visited_pairs"],
            fast["attempts"][0]["algebraic_visited_pairs"]);
        assert_eq!(slow["attempts"][0]["solution"],
            fast["attempts"][0]["solution"]);
    }

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


#[cfg(test)]
mod factorized_coordinate_tests {
    use super::*;
    #[test]
    fn factorized_roots_match_original_s3_and_complete_labelled_solver(){
        let mut ar=Arithmetic::default();
        check_curve(PRIME31,&mut ar).unwrap();
        let (parts,_,_)=source_walk_profile_jac(
            PUBLIC_Q,SEED,&mut ar,[384,384,16_384]).unwrap();
        for l in parts[0].iter().take(48){
            for r in parts[1].iter().take(48){
                let x=l.pt.0;
                let z=r.pt.0;
                assert_ne!(x,z);
                let u=sqrtp_mode::<true>(rhs_from_x::<true>(x)).unwrap();
                let v=sqrtp_mode::<true>(rhs_from_x::<true>(z)).unwrap();
                let denom=mulp_mersenne(subp(x,z),subp(x,z));
                let inv=pownp_mode::<true>(denom,P-2);
                let sumx=addp(x,z);
                let plus=addp(u,v);
                let minus=subp(u,v);
                let mut got=[
                    subp(mulp_mersenne(mulp_mersenne(plus,plus),inv),sumx),
                    subp(mulp_mersenne(mulp_mersenne(minus,minus),inv),sumx)
                ];
                got.sort_unstable();
                let f=s3_coeff_mode::<true>(x,z);
                assert_eq!(f.0,denom);
                assert_eq!(s3_eval_mode::<true>(f,got[0]),0);
                assert_eq!(s3_eval_mode::<true>(f,got[1]),0);
                let disc=subp(mulp_mersenne(f.1,f.1),
                    mulp_mersenne(4,mulp_mersenne(f.2,f.0)));
                let droot=sqrtp_mode::<true>(disc).unwrap();
                let dinv=pownp_mode::<true>(mulp_mersenne(2,denom),P-2);
                let mut expected=[
                    mulp_mersenne(subp(droot,f.1),dinv),
                    mulp_mersenne(subp(subp(0,droot),f.1),dinv)
                ];
                expected.sort_unstable();
                assert_eq!(got,expected);
            }
        }
        let mut old_ar=Arithmetic::default();
        let mut new_ar=Arithmetic::default();
        let (old,old_stats)=
            algebraic_mode_precomp::<true,true>(&parts,PUBLIC_Q,&mut old_ar).unwrap();
        let (new,new_stats)=
            algebraic_mode_factorized(&parts,PUBLIC_Q,&mut new_ar).unwrap();
        assert_eq!(old.as_ref().map(|s|s.log),new.as_ref().map(|s|s.log));
        assert_eq!(old.as_ref().map(|s|s.indices),new.as_ref().map(|s|s.indices));
        assert_eq!(old.as_ref().map(|s|s.signs),new.as_ref().map(|s|s.signs));
        assert_eq!(old_stats.pair_s3,new_stats.pair_s3);
    }
}


#[cfg(test)]
mod bsgs_cold_comparator_tests{
    use super::*;
    #[test]
    fn complete_bsgs_recoveries_and_baby_zero_giant_zero(){
        let mut ar=Arithmetic::default();
        check_curve(PRIME31,&mut ar).unwrap();
        let mut m=1u64;
        while m*m<N{m+=1;}
        for &k in &[1u64,2u64,m,m+3,N-1]{
            let q=ar.scalar(PRIME31,k,Some(PRIME31.g)).unwrap();
            let res=cold_batched_bsgs(q).unwrap();
            assert_eq!(res["complete_target_scalar_recovered"].as_bool(),Some(true));
            assert_eq!(res["log"].as_u64(),Some(k));
        }
    }
}
