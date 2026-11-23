//! Exact non-table m=2 Semaev root extractor over Prime31.
//! Instead of F4, quotient H0,H1 by S3(s,t,z), MONIC in t;
//! compute two univariate eliminants D,N, gcd, factor Fp rational s-roots,
//! recover t and unordered x-pairs with complete linear-remainder branches.
//! This is an experimental B-polynomial algorithm, not an ECDLP speedup.
//! Exhaustive W-pair search is a separate POST-SOLVE check, never an input.
//! cargo test --locked --release --example prime31_monic_s3_pair_oracle
//! cargo run --locked --release --example prime31_monic_s3_pair_oracle
use crypto_lib::cryptanalysis::semaev_prime_poly::{Curve, Fp};
use std::collections::BTreeSet;
use std::time::Instant;

const P:u64 = 2_147_483_647;
const A:u64 = 847_514_846;
const B:u64 = 2_087_444_724;
const GX:u64 = 625_037_715;
const X2G:u64 = 1_923_055_775;
const X3G:u64 = 265_866_980;
const X10G:u64 = 1_344_580_913;
type Poly=Vec<u64>;
// Represents b(s)+a(s)t modulo the monic quadratic S3(s,t,z).
type Pair=(Poly,Poly);

fn trim(mut a:Poly)->Poly { while a.len()>1 && a.last()==Some(&0) {a.pop();} if a.is_empty(){a.push(0);} a }
fn zero(a:&Poly)->bool { a.len()==1 && a[0]==0 }
fn degree(a:&Poly)->usize {a.len()-1}
fn plus(f:Fp,a:&Poly,b:&Poly)->Poly {
    trim((0..a.len().max(b.len())).map(|i|f.add(*a.get(i).unwrap_or(&0),*b.get(i).unwrap_or(&0))).collect())
}
fn minus(f:Fp,a:&Poly,b:&Poly)->Poly {
    trim((0..a.len().max(b.len())).map(|i|f.sub(*a.get(i).unwrap_or(&0),*b.get(i).unwrap_or(&0))).collect())
}
fn scale(f:Fp,a:&Poly,c:u64)->Poly {trim(a.iter().map(|&x|f.mul(x,c)).collect())}
fn product(f:Fp,a:&Poly,b:&Poly)->Poly {
    let mut out=vec![0;a.len()+b.len()-1];
    for (i,&x) in a.iter().enumerate(){
        if x==0 {continue;}
        for (j,&y) in b.iter().enumerate(){out[i+j]=f.add(out[i+j],f.mul(x,y));}
    }
    trim(out)
}
fn eval(f:Fp,a:&Poly,s:u64)->u64 {
    a.iter().rev().fold(0,|acc,&c|f.add(f.mul(acc,s),c))
}
fn divrem(f:Fp,a:&Poly,b:&Poly)->(Poly,Poly) {
    assert!(!zero(b),"division by zero");
    let mut r=a.clone();
    let mut q=vec![0;a.len().saturating_sub(b.len())+1];
    let inv=f.inv(*b.last().unwrap());
    while !zero(&r) && r.len()>=b.len(){
        let shift=r.len()-b.len();
        let coefficient=f.mul(*r.last().unwrap(),inv);
        q[shift]=f.add(q[shift],coefficient);
        for (j,&bj) in b.iter().enumerate(){
            r[shift+j]=f.sub(r[shift+j],f.mul(coefficient,bj));
        }
        r=trim(r);
    }
    (trim(q),r)
}
fn gcd(f:Fp,mut a:Poly,mut b:Poly)->Poly {
    while !zero(&b){let (_,r)=divrem(f,&a,&b);a=b;b=r;}
    if zero(&a){a}else{scale(f,&a,f.inv(*a.last().unwrap()))}
}
fn powmod(f:Fp,mut base:Poly,mut exp:u64,modulus:&Poly)->Poly {
    assert!(degree(modulus)>0);
    base=divrem(f,&base,modulus).1;
    let mut out=vec![1];
    while exp>0 {
        if exp&1!=0 {out=divrem(f,&product(f,&out,&base),modulus).1;}
        exp >>=1;
        if exp>0 {base=divrem(f,&product(f,&base,&base),modulus).1;}
    }
    out
}
fn split_linear_roots(f:Fp,g:Poly,roots:&mut Vec<u64>)->Result<(),String> {
    if degree(&g)==0 {return Ok(());}
    if degree(&g)==1 {
        roots.push(f.mul(f.neg(g[0]),f.inv(g[1])));
        return Ok(());
    }
    for shift in 0..128u64 {
        let h=powmod(f,vec![shift,1],(f.p-1)/2,&g);
        let d=gcd(f,g.clone(),minus(f,&h,&vec![1]));
        if degree(&d)>0 && degree(&d)<degree(&g){
            let (other,rem)=divrem(f,&g,&d);
            if !zero(&rem){return Err("nonexact split division".into());}
            split_linear_roots(f,d,roots)?;
            split_linear_roots(f,other,roots)?;
            return Ok(());
        }
    }
    Err(format!("deterministic split cap at degree {}",degree(&g)))
}
fn fp_roots(f:Fp,g:&Poly)->Result<Vec<u64>,String> {
    if zero(g){return Err("zero univariate eliminant".into());}
    if degree(g)==0{return Ok(Vec::new());}
    // gcd(g,s^p-s) gives precisely the simple Fp-rational factors.
    let s=vec![0,1];
    let xp=powmod(f,s.clone(),f.p,g);
    let split=gcd(f,g.clone(),minus(f,&xp,&s));
    let mut roots=Vec::new();
    split_linear_roots(f,split,&mut roots)?;
    roots.sort_unstable(); roots.dedup();
    Ok(roots)
}
fn sqrt31(f:Fp,x:u64)->Option<u64> {
    // p = 2^31-1 = 3 mod 4
    let root=f.pow(x,(f.p+1)/4);
    if f.mul(root,root)==x {Some(root)} else {None}
}
fn pair_plus(f:Fp,x:&Pair,y:&Pair)->Pair { (plus(f,&x.0,&y.0),plus(f,&x.1,&y.1)) }
fn pair_minus(f:Fp,x:&Pair,y:&Pair)->Pair { (minus(f,&x.0,&y.0),minus(f,&x.1,&y.1)) }
fn pair_scale(f:Fp,x:&Pair,c:u64)->Pair {(scale(f,&x.0,c),scale(f,&x.1,c))}
fn pair_times_s(f:Fp,x:&Pair)->Pair {
    (product(f,&x.0,&vec![0,1]),product(f,&x.1,&vec![0,1]))
}
fn pair_times_t(f:Fp,x:&Pair,q:&Poly,r:&Poly)->Pair {
    (scale(f,&product(f,r,&x.1),f.neg(1)),
     minus(f,&x.0,&product(f,q,&x.1)))
}
fn coeff_s3(f:Fp,z:u64)->(Poly,Poly) {
    // S3=t^2+q(s)*t+r(s).
    let zz=f.mul(z,z);
    let q=vec![
        f.neg(f.add(f.mul(4,zz),f.mul(2,A))),
        f.neg(f.mul(2,z))
    ];
    let r=vec![
        f.sub(f.mul(A,A),f.mul(4,f.mul(B,z))),
        f.neg(f.add(f.mul(2,f.mul(A,z)),f.mul(4,B))),
        zz
    ];
    (q,r)
}
fn factorbase(f:Fp,n:usize)->Vec<u64>{
    let mut w=vec![GX,X2G];
    if n==2{return w;}
    for x in 0..10_000u64 {
        let v=f.add(f.add(f.mul(f.mul(x,x),x),f.mul(A,x)),B);
        if (v==0 || f.pow(v,(P-1)/2)==1) && !w.contains(&x) {w.push(x);}
        if w.len()==n{return w;}
    }
    panic!("too few rational x coordinates")
}
fn membership(f:Fp,w:&[u64])->Poly {
    w.iter().fold(vec![1],|h,&x| product(f,&h,&vec![f.neg(x),1]))
}
fn construct_from_membership(f:Fp,w:&[u64],z:u64,h:&Poly)->(Pair,Pair,Poly,Poly,Poly,Poly,Poly) {
    let (q,r)=coeff_s3(f,z);
    let b=w.len();
    let mut ps:Vec<Pair>=vec![(vec![2],vec![0]),(vec![0,1],vec![0])];
    for k in 2..=b+1 {
        let p=pair_minus(f,&pair_times_s(f,&ps[k-1]),&pair_times_t(f,&ps[k-2],&q,&r));
        ps.push(p);
    }
    let mut hs:Vec<Pair>=Vec::new();
    for j in 0..2 {
        let mut hj=(vec![0],vec![0]);
        for (k,&coef) in h.iter().enumerate(){
            hj=pair_plus(f,&hj,&pair_scale(f,&ps[k+j],coef));
        }
        hs.push(hj);
    }
    let (b0,a0)=&hs[0];let (b1,a1)=&hs[1];
    let d=minus(f,&product(f,a0,b1),&product(f,a1,b0));
    let n=plus(f,
        &minus(f,&product(f,b0,b0),&product(f,&q,&product(f,a0,b0))),
        &product(f,&r,&product(f,a0,a0)));
    let c=gcd(f,d.clone(),n.clone());
    (hs.remove(0),hs.remove(0),q,r,d,n,c)
}
fn construct(f:Fp,w:&[u64],z:u64)->(Pair,Pair,Poly,Poly,Poly,Poly,Poly) {
    let h=membership(f,w);
    construct_from_membership(f,w,z,&h)
}
// Rank-four algebra Fp[s,t,X]/(F_z(t),X^2-sX+t).
// This balanced factor-tree construction is source-independent of the
// Newton recurrence used by construct(), and is an exact coefficient oracle.
// Product of two degree-L nodes uses a constant number of poly products;
// fast univariate multiplication would make its asymptotic work O(M(B) log B).
fn product_pair_mod(f:Fp,a:&Pair,b:&Pair,q:&Poly,r:&Poly)->Pair {
    let bd=product(f,&a.1,&b.1);
    (
        minus(f,&product(f,&a.0,&b.0),&product(f,r,&bd)),
        minus(f,
              &plus(f,&product(f,&a.0,&b.1),&product(f,&a.1,&b.0)),
              &product(f,q,&bd)),
    )
}
fn balanced_h(f:Fp,w:&[u64],q:&Poly,r:&Poly)->(Pair,Pair) {
    if w.is_empty() {return ((vec![1],vec![0]),(vec![0],vec![0]));}
    if w.len()==1 {
        return ((vec![f.neg(w[0])],vec![0]),(vec![1],vec![0]));
    }
    let (u0,v0)=balanced_h(f,&w[..w.len()/2],q,r);
    let (u1,v1)=balanced_h(f,&w[w.len()/2..],q,r);
    let vv=product_pair_mod(f,&v0,&v1,q,r);
    let u=pair_minus(f,&product_pair_mod(f,&u0,&u1,q,r),&pair_times_t(f,&vv,q,r));
    let v=pair_plus(f,
        &pair_plus(f,&product_pair_mod(f,&u0,&v1,q,r),&product_pair_mod(f,&u1,&v0,q,r)),
        &pair_times_s(f,&vv));
    (u,v)
}
fn construct_balanced(f:Fp,w:&[u64],z:u64)->(Pair,Pair) {
    let (q,r)=coeff_s3(f,z);
    let (u,v)=balanced_h(f,w,&q,&r);
    let h0=pair_plus(f,&pair_scale(f,&u,2),&pair_times_s(f,&v));
    let h1=pair_minus(f,
        &pair_plus(f,&pair_times_s(f,&u),&pair_times_s(f,&pair_times_s(f,&v))),
        &pair_scale(f,&pair_times_t(f,&v,&q,&r),2));
    (h0,h1)
}
fn poly_eval_pair(f:Fp,x:&Pair,s:u64,t:u64)->u64 {
    f.add(eval(f,&x.0,s),f.mul(eval(f,&x.1,s),t))
}
fn valid_t(f:Fp,h0:&Pair,h1:&Pair,q:&Poly,r:&Poly,s:u64,t:u64)->bool {
    let eq=f.add(f.add(f.mul(t,t),f.mul(eval(f,q,s),t)),eval(f,r,s));
    eq==0 && poly_eval_pair(f,h0,s,t)==0 && poly_eval_pair(f,h1,s,t)==0
}
fn candidate_ts(f:Fp,h0:&Pair,h1:&Pair,q:&Poly,r:&Poly,s:u64)->Vec<u64> {
    let (b0,a0)=(eval(f,&h0.0,s),eval(f,&h0.1,s));
    let (b1,a1)=(eval(f,&h1.0,s),eval(f,&h1.1,s));
    if a0!=0 {return vec![f.mul(f.neg(b0),f.inv(a0))];}
    if b0!=0 {return Vec::new();}
    if a1!=0 {return vec![f.mul(f.neg(b1),f.inv(a1))];}
    if b1!=0 {return Vec::new();}
    // BOTH remainders vanish at this s: use the genuine quadratic chart.
    let qs=eval(f,q,s);let rs=eval(f,r,s);
    let disc=f.sub(f.mul(qs,qs),f.mul(4,rs));
    let Some(y)=sqrt31(f,disc) else{return Vec::new();};
    let inv2=f.inv(2);
    let mut ts=vec![f.mul(f.add(f.neg(qs),y),inv2),
                    f.mul(f.sub(f.neg(qs),y),inv2)];
    ts.sort_unstable();ts.dedup();ts
}
// Independent full elimination from the existing balanced rank-four factor
// tree.  Unlike the Newton version, h(X) is never explicitly expanded;
// both paths still perform exactly the SAME D/N/gcd, rational root finding,
// and post-substitution checks.  No generic signed-pair dictionary.
fn construct_tree_full(f:Fp,w:&[u64],z:u64)->(Pair,Pair,Poly,Poly,Poly,Poly,Poly) {
    let (h0,h1)=construct_balanced(f,w,z);
    let (q,r)=coeff_s3(f,z);
    let (b0,a0)=(&h0.0,&h0.1);
    let (b1,a1)=(&h1.0,&h1.1);
    let d=minus(f,&product(f,a0,b1),&product(f,a1,b0));
    let n=plus(f,
        &minus(f,&product(f,b0,b0),&product(f,&q,&product(f,a0,b0))),
        &product(f,&r,&product(f,a0,a0)));
    let c=gcd(f,d.clone(),n.clone());
    (h0,h1,q,r,d,n,c)
}
fn extract_with_cached_h(f:Fp,w:&[u64],z:u64,tree:bool,cached_h:Option<&Poly>)
    ->Result<(Vec<(u64,u64)>,usize,usize,usize),String>
{
    let (h0,h1,q,r,d,n,c)=if tree { construct_tree_full(f,w,z) }
                            else if let Some(h)=cached_h { construct_from_membership(f,w,z,h) }
                            else { construct(f,w,z) };
    if zero(&d) && zero(&n){return Err("D=N=0 identically; fallback needed".into());}
    let roots=fp_roots(f,&c)?;
    let mut pairs=BTreeSet::new();
    let inv2=f.inv(2);
    for s in roots.iter().copied(){
        for t in candidate_ts(f,&h0,&h1,&q,&r,s){
            if !valid_t(f,&h0,&h1,&q,&r,s,t){continue;}
            let discr=f.sub(f.mul(s,s),f.mul(4,t));
            let Some(y)=sqrt31(f,discr) else{continue;};
            let x0=f.mul(f.add(s,y),inv2);
            let x1=f.mul(f.sub(s,y),inv2);
            if !w.contains(&x0) || !w.contains(&x1){continue;}
            let pair=(x0.min(x1),x0.max(x1));
            if Curve::new(P,A,B).s3(pair.0,pair.1,z)==0 {pairs.insert(pair);}
        }
    }
    Ok((pairs.into_iter().collect(),degree(&d),degree(&n),degree(&c)))
}
fn extract_with_method(f:Fp,w:&[u64],z:u64,tree:bool)
    ->Result<(Vec<(u64,u64)>,usize,usize,usize),String>
{
    extract_with_cached_h(f,w,z,tree,None)
}
fn extract(f:Fp,w:&[u64],z:u64)->Result<(Vec<(u64,u64)>,usize,usize,usize),String>{
    extract_with_method(f,w,z,false)
}

// Independent exhaustive verifier: NEVER passed to the non-table oracle.
fn reference(f:Fp,w:&[u64],z:u64)->Vec<(u64,u64)> {
    let curve=Curve::new(P,A,B);
    let mut result=BTreeSet::new();
    for (i,&u) in w.iter().enumerate(){
        for &v in &w[i..] {
            if curve.s3(u,v,z)==0 {result.insert((u.min(v),u.max(v)));}
        }
    }
    let _=f;
    result.into_iter().collect()
}
fn check(f:Fp,size:usize,z:u64)->usize {
    let w=factorbase(f,size);
    let start=Instant::now();
    let (got,ddeg,ndeg,cdeg)=extract(f,&w,z).expect("non-table elimination / roots");
    let elapsed=start.elapsed().as_secs_f64()*1000.;
    let reference_start=Instant::now();
    let expected=reference(f,&w,z);
    let reference_ms=reference_start.elapsed().as_secs_f64()*1000.;
    assert_eq!(got,expected,"mismatched exact root set for B={size} target {z}");
    let start_tree=Instant::now();
    let tree=construct_balanced(f,&w,z);
    let tree_ms=start_tree.elapsed().as_secs_f64()*1000.;
    let (h0,h1,_,_,_,_,_)=construct(f,&w,z);
    assert_eq!(tree,(h0,h1),"balanced rank-four subproduct tree differs from Newton trace moments");
    println!("monic-t-oracle p={P} B={size} z={z} Ddeg={ddeg} Ndeg={ndeg} gcd_deg={cdeg} verified_pairs={} online_ms={elapsed:.3} reference_scan_ms={reference_ms:.3} tree_build_ms={tree_ms:.3} tree_newton_coeff_equality=pass exact_reference=pass no_generic_table=true",got.len());
    got.len()
}
// End-to-end *x-only oracle* ablation on the same public-input corpus.
// The only algorithmic difference is how H0,H1 are constructed. Timed stages
// include symbolic construction, D/N, gcd, rational roots, validation and W
// lookup. They exclude factor-base selection and the post-solve reference.
// Five paired repetitions, alternate arm order; this is NOT a complete DLP.
fn compare_construction(f:Fp,size:usize,targets:&[u64]) {
    let w=factorbase(f,size);
    let mut new_total=0.0f64;
    let mut old_total=0.0f64;
    let mut positives=0usize;
    let mut accepted=0usize;
    for &z in targets {
        let expected=reference(f,&w,z); // Independent oracle AFTER public targets frozen.
        let mut time_old=Vec::new();
        let mut time_new=Vec::new();
        for rep in 0..5usize {
            for &tree in if rep%2==0 { &[false,true][..] } else { &[true,false][..] } {
                let start=Instant::now();
                let (got,_,_,_)=extract_with_method(f,&w,z,tree).expect("complete x-only extraction");
                let ms=start.elapsed().as_secs_f64()*1000.0;
                assert_eq!(got,expected,"wrong public (B,z)=({size},{z}), tree={tree}");
                if tree {time_new.push(ms);}else{time_old.push(ms);}
            }
        }
        time_old.sort_by(f64::total_cmp);
        time_new.sort_by(f64::total_cmp);
        let old=time_old[2];
        let new=time_new[2];
        new_total+=new;
        old_total+=old;
        if !expected.is_empty(){positives+=1;}
        accepted+=expected.len();
        println!("monic-compare p={P} B={size} z={z} pairs={} new_tree_median_ms={new:.6} old_newton_median_ms={old:.6} speedup_newton_over_tree={:.3} exact=pass paired_reps=5 postsolve_reference=pass",expected.len(),old/new);
    }
    if targets.len()==8{
        assert_eq!(positives,3,"expected three x-only positive controls");
        assert_eq!(accepted,3,"expected exactly one x-only pair per positive control");
    }
    println!("monic-compare-total p={P} B={size} cells={} old_newton_sum_median_ms={old_total:.6} new_tree_sum_median_ms={new_total:.6} speedup={:.4} positives={positives} pairs={accepted} exact=pass",targets.len(),old_total/new_total);
}

/// Fixed public affine abscissae on the same prime-order ordinary curve.
/// No discrete-log indices are generated or passed to extract(); independent
/// pair enumeration happens only *after* the candidate returns.
fn public_curve_x_controls(f:Fp,count:usize)->Vec<u64>{
    let mut state=0xC31B_A7ED_2026_1122u64;
    let mut xs=Vec::new();
    while xs.len()<count {
        state=state.wrapping_mul(6_364_136_223_846_793_005)
                   .wrapping_add(1_442_695_040_888_963_407);
        let x=state%P;
        let y2=f.add(f.add(f.mul(f.mul(x,x),x),f.mul(A,x)),B);
        if y2!=0 && f.pow(y2,(P-1)/2)==1 && !xs.contains(&x) {
            xs.push(x);
        }
    }
    xs
}
/// Produce a known-valid x-only S3 target from public W elements for grading.
/// The extractor receives only z and W, never pair indices or any DLP scalar.
/// This is a PLANTED correctness control, not a blind random-target experiment.
fn planted_pair_target(f:Fp,u:u64,v:u64)->u64{
    assert_ne!(u,v,"nonexceptional S3 pair");
    let s=f.add(u,v);
    let t=f.mul(u,v);
    let delta=f.sub(u,v);
    let d=f.mul(delta,delta);
    let mid=f.add(f.mul(s,f.add(t,A)),f.mul(2,B));
    let c=f.sub(f.mul(f.sub(t,A),f.sub(t,A)),f.mul(f.mul(4,B),s));
    let disc=f.sub(f.mul(mid,mid),f.mul(d,c));
    let sqrt=sqrt31(f,disc).expect("rational planted S3 target");
    let z=f.mul(f.add(mid,sqrt),f.inv(d));
    assert_eq!(Curve::new(P,A,B).s3(u,v,z),0);
    z
}

// Run76: compare balanced tree, uncached Newton and precomputed-h Newton.
// This does not change the registered Run75 two-arm panel.
fn compare_cached_construction(f:Fp,size:usize,targets:&[u64]) {
    let w=factorbase(f,size);
    let start_setup=Instant::now();
    let cached=membership(f,&w);
    let setup_ms=start_setup.elapsed().as_secs_f64()*1000.;
    let mut totals=[0f64;3];
    let mut positive=0usize;
    let mut accepted=0usize;
    for &z in targets {
        let expected=reference(f,&w,z);
        let mut times=[Vec::new(),Vec::new(),Vec::new()];
        for rep in 0..5usize {
            for offset in 0..3usize {
                // Rotating order avoids systematically favoring a hot arm.
                let arm=(rep+offset)%3;
                let start=Instant::now();
                let (got,_,_,_)=match arm {
                    0=>extract_with_cached_h(f,&w,z,false,None),
                    1=>extract_with_cached_h(f,&w,z,true,None),
                    _=>extract_with_cached_h(f,&w,z,false,Some(&cached)),
                }.expect("run76 x-only extraction");
                let elapsed=start.elapsed().as_secs_f64()*1000.;
                assert_eq!(got,expected,"run76 mismatch B={size} z={z} arm={arm}");
                times[arm].push(elapsed);
            }
        }
        let mut medians=[0f64;3];
        for arm in 0..3usize {
            times[arm].sort_by(f64::total_cmp);
            medians[arm]=times[arm][2];
            totals[arm]+=medians[arm];
        }
        if !expected.is_empty(){positive+=1;}
        accepted+=expected.len();
        println!("run76-compare p={P} B={size} z={z} pairs={} newton_ms={:.6} tree_ms={:.6} cached_newton_ms={:.6} exact=pass reps=5",
            expected.len(),medians[0],medians[1],medians[2]);
    }
    if targets.len()==8 {
        assert_eq!(positive,3,"run76 wrong positive count");
        assert_eq!(accepted,3,"run76 wrong witness count");
    }
    println!("run76-total p={P} B={size} cells={} positives={positive} pairs={accepted} membership_setup_ms={setup_ms:.6} newton_sum_ms={:.6} tree_sum_ms={:.6} cached_newton_sum_ms={:.6} cached_amortized_ms={:.6} exact=pass",
        targets.len(),totals[0],totals[1],totals[2],setup_ms+totals[2]);
}

fn main(){
    let f=Fp::new(P);
    let compare=std::env::args().any(|a|a=="--compare");
    let compare_cached=std::env::args().any(|a|a=="--compare-cached");
    let scale=compare || compare_cached || std::env::args().any(|a|a=="--scale");
    let sizes:&[usize]=if scale { &[64,128,256,677] } else { &[3,4,8,16,32] };
    for &size in sizes {
        let mut targets:Vec<u64>=if scale {
            let mut x=vec![GX,X10G];
            x.extend(public_curve_x_controls(f,4));
            let base=factorbase(f,size);
            x.push(planted_pair_target(f,base[size/3],base[size/2]));
            x.push(planted_pair_target(f,base[size/4],base[size-3]));
            x
        } else { vec![GX,X2G,X3G,X10G] };
        targets.sort_unstable();
        targets.dedup();
        if scale {assert_eq!(targets.len(),8,"all scale controls must be distinct");}
        println!("native original-Rust Prime31 non-table S3 pair elimination scale={} compare={} B={} z={:?}; public-only oracle, two planted positives for grading",scale,compare,size,targets);
        if compare_cached { compare_cached_construction(f,size,&targets); continue; }
        if compare { compare_construction(f,size,&targets); continue; }
        let mut total_pairs=0usize;
        let mut positive_targets=0usize;
        for &z in &targets {
            let count=check(f,size,z);
            total_pairs+=count;
            if count>0 {positive_targets+=1;}
        }
        if scale {
            // Independent fixed S3 ground truth: GX and both nontrivial planted pairs
            // have one witness each; X10G and four deterministic curve-x controls do not.
            assert_eq!(positive_targets,3,"missing planted S3 positives at B={size}");
            assert_eq!(total_pairs,3,"unexpected S3 pair-set cardinality at B={size}");
        }
    }
    println!("S3 x-roots only: full signed group checks, authentic RHS rank, cold solver and asymptotic speedup NOT established.");
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn monic_s3_pair_oracle_exact_public_positive_and_negative_controls() {
        let f=Fp::new(P);
        for size in [2usize,3,4,6,8] {
            for z in [GX,X2G,X3G,X10G]{check(f,size,z);}
        }
    }
    #[test]
    fn quotient_pair_ghost_roots_are_filtered() {
        let f=Fp::new(P);
        let w=factorbase(f,4);
        for z in [GX,X2G,X3G,X10G]{
            let (h0,h1,q,r,_,_,_)=construct(f,&w,z);
            for &x in &w {
                for &y in &w {
                    let s=f.add(x,y);let t=f.mul(x,y);
                    // The H_j remainders agree with membership only ON F_z=0;
                    // off that hypersurface, H_j differs by a multiple of F_z.
                    let satisfies_s3=Curve::new(P,A,B).s3(x,y,z)==0;
                    if satisfies_s3 {
                        assert_eq!(poly_eval_pair(f,&h0,s,t),0);
                        assert_eq!(poly_eval_pair(f,&h1,s,t),0);
                    }
                    assert_eq!(valid_t(f,&h0,&h1,&q,&r,s,t),satisfies_s3);
                }
            }
        }
    }
}
