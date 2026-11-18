//! Target-reduced 8x8 elliptic RR9 determinant on the *original* Prime31 curve.
//! Supplies an exact algebraic relation predicate for GIVEN weighted signed points.
//! This does not extract unknown points and does not compute unknown discrete logs.
//! Protocol: ../rr9/W8_TARGET_REDUCED_DETERMINANT_PROTOCOL_20261118.md
use prime31_algebraic_ic::{check_curve, Arithmetic, Point, PRIME31};
use serde_json::{json, Value};
use std::collections::HashSet;
use std::time::Instant;

const P: u64 = 2_147_483_647;
const W: [u64; 8] = [1, 3, 5, 7, 11, 13, 17, 19];
const G: (u64,u64) = (625037715, 2100627251);

#[derive(Default,Clone,Copy)]
struct FpCost { multiplications: u64, inversions: u64 }
fn plus(a:u64,b:u64)->u64 { let s=a+b; if s>=P {s-P} else {s} }
fn minus(a:u64,b:u64)->u64 { if a>=b {a-b} else {a+P-b} }
fn times(a:u64,b:u64,c:&mut FpCost)->u64 {
    c.multiplications+=1; ((a as u128*b as u128)%P as u128) as u64
}
fn power(mut a:u64,mut e:u64,c:&mut FpCost)->u64 {
    let mut b=1u64;
    while e>0 {
        if (e&1)!=0 { b=times(b,a,c); }
        e>>=1;
        if e>0 { a=times(a,a,c); }
    }
    b
}
fn inverse(a:u64,c:&mut FpCost)->u64 {
    assert_ne!(a,0); c.inversions+=1; power(a,P-2,c)
}
fn det<const N:usize>(mut rows:[[u64;N];N], c:&mut FpCost)->u64 {
    let mut d=1u64;
    for j in 0..N {
        let Some(i)=(j..N).find(|&i| rows[i][j]!=0) else {return 0};
        if i!=j { rows.swap(i,j); d=minus(0,d); }
        let pj=rows[j][j];
        d=times(d,pj,c);
        let pi=inverse(pj,c);
        for i in j+1..N {
            if rows[i][j]==0 {continue}
            let k=times(rows[i][j],pi,c);
            for col in j+1..N {
                rows[i][col]=minus(rows[i][col],times(k,rows[j][col],c));
            }
            rows[i][j]=0;
        }
    }
    d
}
fn neg(q:(u64,u64))->(u64,u64) {(q.0,minus(0,q.1))}
// No elliptic group additions, scalar multiplication or discrete logarithms in either predicate.
fn determinant8(pts:&[(u64,u64);8],r:(u64,u64))->(u64,FpCost) {
    let mut c=FpCost::default();
    let mut rows=[[0u64;8];8];
    for (i,&(x,y)) in pts.iter().enumerate() {
        let mut xp=1u64;
        for j in 0..4 {
            rows[i][j]=times(minus(x,r.0),xp,&mut c);
            rows[i][4+j]=times(plus(y,r.1),xp,&mut c);
            xp=times(xp,x,&mut c);
        }
    }
    (det(rows,&mut c),c)
}
fn determinant9(pts:&[(u64,u64);8],r:(u64,u64))->(u64,FpCost) {
    let mut c=FpCost::default();
    let mut rows=[[0u64;9];9];
    for i in 0..9 {
        let (x,y)=if i<8 {pts[i]} else {neg(r)};
        let x2=times(x,x,&mut c); let x3=times(x2,x,&mut c);
        let x4=times(x3,x,&mut c);
        rows[i]=[1,x,y,x2,times(x,y,&mut c),x3,times(x2,y,&mut c),
                 x4,times(x3,y,&mut c)];
    }
    (det(rows,&mut c),c)
}
fn admissible(pts:&[(u64,u64);8],r:Point)->bool {
    let Some(r)=r else {return false};
    let mut seen=HashSet::new();
    for &q in pts.iter().chain(std::iter::once(&neg(r))) {
        if q.0>=P||q.1>=P||!seen.insert(q) {return false}
    }
    true
}
fn canonical_base(ar:&mut Arithmetic)->Vec<(u64,u64)> {
    let mut out=Vec::new();
    for x in 0..10_000 {
        if let Some(y)=ar.sqrt(ar.rhs(PRIME31,x),P) {
            if y!=0 {
                let q=(x,y.min(P-y));
                assert!(ar.on_curve(PRIME31,Some(q)));
                out.push(q);
            }
        }
        if out.len()==31 {break}
    }
    assert_eq!(out.len(),31);
    out
}
fn weighted_slots(ar:&mut Arithmetic,base:&[(u64,u64)]) ->Vec<Vec<(u64,u64)>> {
    let mut xs=HashSet::new();
    let out:Vec<_>=W.iter().map(|&w| {
        base.iter().map(|&q| {
            let p=ar.scalar(PRIME31,w,Some(q)).expect("nonidentity weighted point");
            assert!(xs.insert(p.0),"weighted slot abscissae must be disjoint");
            p
        }).collect()
    }).collect();
    assert_eq!(xs.len(),248);
    out
}
fn sum(ar:&mut Arithmetic,pts:&[(u64,u64);8])->Point {
    pts.iter().fold(None,|s,&q|ar.add(PRIME31,s,Some(q)))
}
fn fixture(t:usize,slots:&[Vec<(u64,u64)>])->[(u64,u64);8] {
    std::array::from_fn(|j| {
        let ix=(3*t+7*j+j*j)%31;
        let pt=slots[j][ix];
        if (t+3*j)%4<2 {pt} else {neg(pt)}
    })
}
fn run()->Value {
    let mut ar=Arithmetic::default();
    check_curve(PRIME31,&mut ar).expect("original prime-order curve");
    let base=canonical_base(&mut ar);
    assert_eq!(&base[..8].iter().map(|q|q.0).collect::<Vec<_>>(),&vec![0,1,2,3,5,6,8,9]);
    let slots=weighted_slots(&mut ar,&base);
    let mut records=Vec::new();
    let mut d8_ns=0u128;
    let mut d9_ns=0u128;
    let mut zeros=0u32;
    for t in 0..16usize {
        let pts=fixture(t,&slots);
        let r=sum(&mut ar,&pts).expect("registered target nonidentity");
        let changed={let mut a=pts;a[0]=neg(a[0]);a};
        let shifted=ar.add(PRIME31,Some(r),Some(G)).expect("shifted target nonidentity");
        for (name,q,rr,is_true) in [
            ("positive",pts,r,true),("first_sign_flipped",changed,r,false),
            ("residual_plus_g",pts,shifted,false)
        ] {
            assert!(admissible(&q,Some(rr)),"invalid repeated point or -R");
            // Check the expected group equation separately from the determinant predicates.
            assert_eq!(sum(&mut ar,&q)==Some(rr),is_true);
            let start=Instant::now();
            let (d8,c8)=determinant8(&q,rr);
            d8_ns+=start.elapsed().as_nanos();
            let start=Instant::now();
            let (d9,c9)=determinant9(&q,rr);
            d9_ns+=start.elapsed().as_nanos();
            assert_eq!(d8,d9,"exact target-reduced determinant identity");
            assert_eq!(d8==0,is_true,"genuine signed group relation");
            zeros+=u32::from(d8==0);
            records.push(json!({
                "fixture":t,"kind":name,"target":rr,"det8":d8,"det9":d9,
                "fp8_multiplications":c8.multiplications,"fp8_inversions":c8.inversions,
                "fp9_multiplications":c9.multiplications,"fp9_inversions":c9.inversions
            }));
        }
    }
    let pts=fixture(0,&slots);
    let r=sum(&mut ar,&pts).unwrap();
    let wrong=ar.add(PRIME31,Some(r),Some(G)).unwrap();
    let mut repeated=pts;
    repeated[1]=repeated[0];
    assert_ne!(sum(&mut ar,&repeated),Some(wrong));
    assert!(!admissible(&repeated,Some(wrong)));
    assert_eq!(determinant8(&repeated,wrong).0,0,"repeated row false zero");
    let mut at_negative_target=pts;
    at_negative_target[0]=neg(wrong);
    assert_ne!(sum(&mut ar,&at_negative_target),Some(wrong));
    assert!(!admissible(&at_negative_target,Some(wrong)));
    assert_eq!(determinant8(&at_negative_target,wrong).0,0,"-R zero row");
    json!({
        "schema":"prime31.w8.target-reduced-det8.v1","p":P,"a":PRIME31.a,
        "b":PRIME31.b,"n":PRIME31.n,"cofactor":1,
        "weights":W,"base_x":base.iter().map(|q|q.0).collect::<Vec<_>>(),
        "cases":records,"exact_identity_checks":48,"positive_zero_checks":zeros,
        "negative_nonzero_checks":32,
        "duplicate_false_zero_rejected":true,"target_point_false_zero_rejected":true,
        "total_det8_elapsed_ns":d8_ns.to_string(),"total_det9_elapsed_ns":d9_ns.to_string(),
        "unknown_chart_extractions":0,"complete_unknown_ecdlp_solves":0,
        "source_curve":"original-prime31","pcc":"unverified"
    })
}
fn main(){println!("{}",serde_json::to_string_pretty(&run()).unwrap());}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn target_reduced_exact_original_curve() {
        let j=run();
        assert_eq!(j["exact_identity_checks"],48);
        assert_eq!(j["positive_zero_checks"],16);
        assert_eq!(j["negative_nonzero_checks"],32);
        assert_eq!(j["duplicate_false_zero_rejected"],true);
        assert_eq!(j["target_point_false_zero_rejected"],true);
    }
    #[test]
    fn raw_zero_is_not_a_safe_relation_on_duplicated_rows() {
        let mut ar=Arithmetic::default();
        let b=canonical_base(&mut ar);
        let slots=weighted_slots(&mut ar,&b);
        let mut pts=fixture(0,&slots);
        let r=sum(&mut ar,&pts).unwrap();
        pts[1]=pts[0];
        assert!(!admissible(&pts,Some(r)));
        assert_eq!(determinant8(&pts,r).0,0);
    }
}
