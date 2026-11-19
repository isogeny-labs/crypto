//! CT226: native genus-4 composite-order transfer + complete toy IC cold-cost probe.
//! Uses the repository's FpPoly and genus-generic Mumford/Cantor arithmetic.
//! Fixed C/F101: y^2=u^9+u^3+46 -> E: y^2=x^3+x+46, E subgroup order 109.
//! Public known-answer target Q=(64,19); the solver never receives its scalar.
//! Diagnostic, not PCC or a growing-prime-field attack. [pcc:unverified]
use crypto_lib::prime_hyperelliptic::{FpPoly, HyperellipticCurveP, MumfordDivisorP};
use crypto_lib::prime_hyperelliptic::ops::{self, Kind};
use num_bigint::BigUint;
use std::time::Instant;

const P: i64 = 101;
const R: i64 = 109;
type Ec = Option<(i64,i64)>;
type Div = MumfordDivisorP;
fn b(x: i64) -> BigUint { BigUint::from(x as u64) }
fn m(x: i64, p: i64) -> i64 { x.rem_euclid(p) }
fn power(mut x:i64, mut n:i64, p:i64)->i64{
    let mut out=1;
    while n>0 {if n&1!=0 {out=m(out*x,p);} x=m(x*x,p);n>>=1;}
    out
}
fn ec_add(a:Ec, z:Ec)->Ec {
    match (a,z) {
        (None,p)|(p,None)=>p,
        (Some((x,y)),Some((u,v)))=>{
            if x==u && m(y+v,P)==0 {return None;}
            let (num,den)=if x==u {(3*x*x+1,2*y)} else {(v-y,u-x)};
            let slope=m(num*power(m(den,P),P-2,P),P);
            let xx=m(slope*slope-x-u,P);
            Some((xx,m(slope*(x-xx)-y,P)))
        }
    }
}
fn ec_mul(mut k:i64, point:Ec)->Ec {
    let mut result=None;
    let mut base=point;
    while k>0 {if k&1!=0 {result=ec_add(result,base);}
        base=ec_add(base,base); k>>=1;}
    result
}
fn direct_bsgs(g:Ec, target:Ec)->(i64,u64) {
    let n=11i64; // ceil(sqrt(109))
    let mut table=Vec::with_capacity(n as usize);
    let mut current=None;
    let mut ops=0u64;
    for k in 0..n {table.push((current,k)); current=ec_add(current,g);ops+=1;}
    let neg_g=g.map(|(x,y)|(x,m(-y,P)));
    let step=ec_mul(n,neg_g);
    let mut giant=target;
    for i in 0..=n {
        if let Some((_,j))=table.iter().find(|(p,_)|*p==giant) {
            let scalar=m(i*n+*j,R);
            assert_eq!(ec_mul(scalar,g),target);
            return (scalar,ops);
        }
        giant=ec_add(giant,step);ops+=1;
    }
    panic!("direct BSGS failed")
}
fn poly(xs:&[i64])->FpPoly{
    FpPoly::from_coeffs(xs.iter().copied().map(b).collect(),b(P))
}
fn curve()->HyperellipticCurveP {
    HyperellipticCurveP::try_new(b(P),poly(&[46,0,0,1,0,0,0,0,0,1]),4).unwrap()
}
fn point(curve:&HyperellipticCurveP, x:i64, y:i64)->Div {
    MumfordDivisorP::try_from_point(curve,&b(x),&b(y)).unwrap()
}
fn image(curve:&HyperellipticCurveP, e:Ec)->Div {
    match e {
        None=>Div::identity(b(P)),
        Some((x,y))=>Div::try_from_polynomials(curve,poly(&[m(-x,P),0,0,1]),poly(&[y])).unwrap()
    }
}
fn x_roster(curve:&HyperellipticCurveP)->(Vec<i64>,Vec<(i64,i64)>) {
    let mut xs=Vec::new();
    let mut first=Vec::new();
    for x in 0..P {
        let mut ys=Vec::new();
        let xx=b(x);
        let rhs=curve.f.eval(&xx).to_u64_digits().first().copied().unwrap_or(0) as i64;
        for y in 0..P {if m(y*y-rhs,P)==0 {ys.push(y);}}
        if !ys.is_empty() {
            assert_eq!(ys.len(),2,"unexpected ramified point in 54-variable model");
            xs.push(x);
            if first.len()<8 {first.push((x,ys[0]));}
        }
    }
    assert_eq!(xs.len(),54);
    (xs,first)
}
fn coord(x:i64,y:i64,xs:&[i64])->Vec<i64> {
    let mut out=vec![0;xs.len()];
    let i=xs.binary_search(&x).unwrap();
    assert!(y>0 && y<P && y!=P-y);
    out[i]=if y<P-y {1} else {-1};
    out
}
fn support(div:&Div,xs:&[i64])->Option<Vec<i64>> {
    if div.u.degree()!=Some(4) {return None;}
    let mut roots=Vec::new();
    for &x in xs {
        if div.u.eval(&b(x))==b(0) {roots.push(x);}
    }
    if roots.len()!=4 {return None;} // squarefree fully split only
    let mut row=vec![0i64;xs.len()];
    for x in roots {
        let yy=div.v.eval(&b(x)).to_u64_digits().first().copied().unwrap_or(0) as i64;
        let entry=coord(x,yy,xs);
        for (a,z) in row.iter_mut().zip(entry) {*a+=z;}
    }
    Some(row)
}
fn offsets(first:&[(i64,i64)])->Vec<Vec<(i64,i64)>> {
    let mut out=Vec::new();
    // Same lexicographic combination schedule as the frozen CT219 source.
    for a in 0..8 {for b in a+1..8 {out.push(vec![first[a],first[b]]);}}
    for a in 0..8 {for b in a+1..8 {for c in b+1..8 {
        out.push(vec![first[a],first[b],first[c]]);
    }}}
    for a in 0..8 {for b in a+1..8 {for c in b+1..8 {for d in c+1..8 {
        out.push(vec![first[a],first[b],first[c],first[d]]);
    }}}}
    out
}
fn inverse(x:i64)->i64{assert_ne!(m(x,R),0);power(m(x,R),R-2,R)}
fn reduce_row(pivots:&mut [Option<Vec<i64>>],row:&[i64],rhs:i64)->bool {
    let n=pivots.len();
    let mut a:Vec<i64>=row.iter().copied().map(|x|m(x,R)).collect();
    a.push(m(rhs,R));
    for i in 0..n {
        if a[i]==0 {continue;}
        if let Some(pivot)=&pivots[i] {
            let mul=a[i];
            for j in i..=n {a[j]=m(a[j]-mul*pivot[j],R);}
        } else {
            let inv=inverse(a[i]);
            for z in &mut a[i..=n] {*z=m(*z*inv,R);}
            pivots[i]=Some(a);
            return true;
        }
    }
    assert_eq!(a[n],0,"incorrect or inconsistent transfer relation");
    false
}
fn solve(pivots:&[Option<Vec<i64>>])->Vec<i64> {
    let n=pivots.len();
    assert!(pivots.iter().all(Option::is_some));
    let mut logs=vec![0;n];
    for i in (0..n).rev() {
        let row=pivots[i].as_ref().unwrap();
        let mut value=row[n];
        for j in i+1..n {value=m(value-row[j]*logs[j],R);}
        logs[i]=m(value,R);
    }
    logs
}
#[derive(Debug)]
struct Cost {
    setup:u128, relations:u128, linear_algebra:u128, recovery:u128, verification:u128,
    candidates:usize, smooth:usize, rank:usize, used_offsets:usize,
    cantor_adds:u64, target_adds:u64, fp_add:u64, fp_mul:u64, fp_sqr:u64, fp_inv:u64,
}
impl Cost {
    fn total(&self)->u128{
        self.setup+self.relations+self.linear_algebra+self.recovery+self.verification
    }
}
fn ic_cold()->Cost {
    let initial=Instant::now();
    let tally0=ops::snapshot().total();
    let c=curve();
    let g=Some((2,37));
    let q=Some((64,19));
    let gdiv=image(&c,g);
    let qdiv=image(&c,q);
    assert_eq!(gdiv.scalar_mul_fast(&b(R),&c),Div::identity(b(P)));
    assert_eq!(gdiv.scalar_mul_fast(&b(37),&c),qdiv);
    let (xs,first)=x_roster(&c);
    let roster=offsets(&first);
    let setup=initial.elapsed().as_nanos();
    let mut pivots=vec![None;xs.len()];
    let mut candidates=0usize;
    let mut smooth=0usize;
    let mut adds=0u64;
    let mut used_offsets=0usize;
    let mut relations=Vec::new();
    let t=Instant::now();
    for pts in &roster {
        used_offsets+=1;
        let mut cur=Div::identity(b(P));
        let mut off=vec![0i64;xs.len()];
        for &(x,y) in pts {
            cur=cur.add_fast(&point(&c,x,y),&c);adds+=1;
            let row=coord(x,y,&xs);
            for (o,z) in off.iter_mut().zip(row) {*o+=z;}
        }
        for k in 0..R {
            candidates+=1;
            if let Some(row)=support(&cur,&xs) {
                smooth+=1;
                let diff:Vec<i64>=row.iter().zip(&off).map(|(x,y)|x-y).collect();
                let rhs=m(3*k,R);
                relations.push((diff.clone(),rhs));
                reduce_row(&mut pivots,&diff,rhs);
                if pivots.iter().all(Option::is_some) {break;}
            }
            if k<R-1 {cur=cur.add_fast(&gdiv,&c);adds+=1;}
        }
        if pivots.iter().all(Option::is_some) {break;}
    }
    let relation_time=t.elapsed().as_nanos();
    let rank=pivots.iter().filter(|p|p.is_some()).count();
    assert_eq!(rank,54,"native full-rank finite IC not established");
    let t=Instant::now();
    let fb=solve(&pivots);
    for (row,rhs) in &relations {
        assert_eq!(m(row.iter().zip(&fb).map(|(x,y)|x*y).sum(),R),*rhs);
    }
    let linear_algebra=t.elapsed().as_nanos();
    let t=Instant::now();
    let pts=&roster[0];
    let mut off=vec![0i64;xs.len()];
    let mut cur=qdiv.clone();
    let mut target_adds=0u64;
    for &(x,y) in pts {
        cur=cur.add_fast(&point(&c,x,y),&c);target_adds+=1;
        let row=coord(x,y,&xs);
        for (o,z) in off.iter_mut().zip(row) {*o+=z;}
    }
    let mut recovered=None;
    for a in 0..R {
        if let Some(row)=support(&cur,&xs) {
            let scalar=m(
                (row.iter().zip(&off).zip(&fb).map(|((x,y),z)|(x-y)*z).sum::<i64>()
                   *inverse(3))-a,R);
            if ec_mul(scalar,g)==q {recovered=Some(scalar);break;}
            panic!("invalid norm IC target scalar");
        }
        cur=cur.add_fast(&gdiv,&c);target_adds+=1;
    }
    assert_eq!(recovered,Some(37));
    let recovery=t.elapsed().as_nanos();
    let t=Instant::now();
    for (i,&x) in xs.iter().enumerate() {
        let yy=(0..P).find(|&y|m(y*y-(m(power(x,9,P)+power(x,3,P)+46,P)),P)==0).unwrap();
        assert_eq!(ec_mul(fb[i],g),Some((power(x,3,P),yy)));
    }
    assert_eq!(gdiv.scalar_mul_fast(&b(recovered.unwrap()),&c),qdiv);
    let verification=t.elapsed().as_nanos();
    let tally=ops::snapshot().total().since(&tally0);
    Cost {setup,relations:relation_time,linear_algebra,recovery,verification,
        candidates,smooth,rank,used_offsets,cantor_adds:adds,target_adds,
        fp_add:tally.get(Kind::FieldAdd),fp_mul:tally.get(Kind::FieldMul),
        fp_sqr:tally.get(Kind::FieldSqr),fp_inv:tally.get(Kind::FieldInv)}
}
#[test]
fn paired_native_complete_composite_jacobian_ic_vs_direct_bsgs() {
    // Repeated cold solver construction. Test names indicate the comparator;
    // rho and preexisting prime-field IC are NOT silently counted as checked.
    for round in 0..3 {
        let public=Some((64,19));
        let baseline=|| {
            let start=Instant::now();
            let (scalar,adds)=direct_bsgs(Some((2,37)),public);
            assert_eq!(scalar,37);
            (start.elapsed().as_nanos(),adds)
        };
        let (ic,(bsgs_ns,bsgs_adds))=if round%2==0 {
            let ic=ic_cold();let bs=baseline();(ic,bs)
        } else {
            let bs=baseline();let ic=ic_cold();(ic,bs)
        };
        assert!(ic.total()>0 && bsgs_ns>0);
        assert_eq!((ic.rank,ic.smooth,ic.candidates),(54,62,1421));
        eprintln!(
            "CT226_NATIVE_PAIR_JSON {{\"round\":{},\"arm_order\":\"{}\",\"candidate_ns\":{},\"direct_bsgs_ns\":{},\"ratio_direct_over_candidate\":{:.10},\"ic_setup_ns\":{},\"ic_relation_ns\":{},\"ic_la_ns\":{},\"ic_recovery_ns\":{},\"ic_verify_ns\":{},\"ic_candidates\":{},\"ic_smooth\":{},\"ic_rank\":{},\"offsets\":{},\"cantor_adds\":{},\"target_adds\":{},\"direct_bsgs_adds\":{},\"field_add\":{},\"field_mul\":{},\"field_sqr\":{},\"field_inv\":{},\"source\":\"known_answer_F101\",\"memory\":\"unmeasured\",\"pcc\":\"unverified\"}}",
            round,if round%2==0 {"IC_BSGS"} else {"BSGS_IC"},
            ic.total(),bsgs_ns,bsgs_ns as f64/ic.total() as f64,
            ic.setup,ic.relations,ic.linear_algebra,ic.recovery,ic.verification,
            ic.candidates,ic.smooth,ic.rank,ic.used_offsets,ic.cantor_adds,
            ic.target_adds,bsgs_adds,ic.fp_add,ic.fp_mul,ic.fp_sqr,ic.fp_inv
        );
    }
}

// CT229 lives as a child module to reuse these exact validated native helpers.
#[path = "ct229_impl/probe.rs"]
mod ct229;

// CT231 stays in its own module while reusing all of CT226's native helpers.
#[path = "ct231_impl/probe.rs"]
mod ct231;
