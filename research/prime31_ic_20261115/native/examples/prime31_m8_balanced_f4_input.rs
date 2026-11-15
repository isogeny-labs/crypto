//! Balanced S4/S3 m8 PKM polynomial input on the existing prime31 Dickson base.
//!
//! This is a SOURCE/SEMANTIC bridge to the existing Rust F4 tower, not an
//! F4 solve or a measured cheaper ECDLP algorithm.  The eight-fibre
//! chain needs only THREE ordinary free abscissae (u,v,w), not six:
//! S4(x0,x1,x2,u), S4(x3,x4,x5,v), S3(x6,x7,w), S4(u,v,w,x(R)).
//! All signed point lifts and the full public target are checked separately.
//! [pcc:unverified]
use prime31_algebraic_ic::{check_curve, f4_fp_tower::{SquareRule,TowerRing}, Arithmetic, Base, Point, PRIME31};
use serde::Deserialize;
use serde_json::json;
use std::collections::{BTreeMap, HashSet};

const Q: Point = Some((1_547_953_785, 1_667_575_595));
const NX: usize = 11;
const DEPTH: usize = 6;
const TOWER: usize = 8 * DEPTH;
const NV: usize = TOWER + 3;
const P: u64 = 2_147_483_647;
const A: u64 = 847_514_846;
const B: u64 = 2_087_444_724;

fn mul(x: u64, y: u64) -> u64 { ((x as u128 * y as u128) % P as u128) as u64 }
fn add(x: u64, y: u64) -> u64 { let z = x+y; if z>=P { z-P } else {z} }
fn sub(x: u64, y: u64) -> u64 { if x>=y {x-y} else {x+P-y} }

// Sparse polynomial over Fp in eight leaf abscissae and u,v,w. The raw
// polynomial is reduced by the EXISTING TowerRing::from_raw implementation;
// this module neither copies F4 arithmetic nor adds an alternate engine.
#[derive(Clone, Default)]
struct Polynomial(BTreeMap<[u8;NX],u64>);
impl Polynomial {
    fn constant(c: u64) -> Self {
        if c % P==0 {return Self::default();}
        Self(BTreeMap::from([([0;NX],c%P)]))
    }
    fn variable(i: usize) -> Self {
        assert!(i<NX);
        let mut key=[0u8;NX];key[i]=1;
        Self(BTreeMap::from([(key,1)]))
    }
    fn plus(&self, rhs:&Self) -> Self {
        let mut out=self.0.clone();
        for (m,&c) in &rhs.0 {
            let v=add(*out.get(m).unwrap_or(&0),c);
            if v==0 {out.remove(m);} else {out.insert(*m,v);}
        }
        Self(out)
    }
    fn negative(&self) -> Self {
        Self(self.0.iter().map(|(&m,&c)|(m,sub(0,c))).collect())
    }
    fn minus(&self,rhs:&Self)->Self{self.plus(&rhs.negative())}
    fn scaled(&self,c:u64)->Self {
        let mut out=BTreeMap::new();
        for (&m,&v) in &self.0 {let z=mul(v,c%P);if z!=0 {out.insert(m,z);}}
        Self(out)
    }
    fn times(&self,rhs:&Self)->Self {
        let mut out=BTreeMap::new();
        for (&m,&x) in &self.0 {
            for (&n,&y) in &rhs.0 {
                let mut ex=[0u8;NX];
                for k in 0..NX {ex[k]=m[k].checked_add(n[k]).expect("polynomial exponent overflow");}
                let v=add(*out.get(&ex).unwrap_or(&0),mul(x,y));
                if v==0 {out.remove(&ex);} else {out.insert(ex,v);}
            }
        }
        Self(out)
    }
    fn square(&self)->Self{self.times(self)}
    fn raw_tower(&self)->Vec<(Vec<u32>,u64)> {
        self.0.iter().map(|(m,&c)|{
            let mut ex=vec![0_u32;NV];
            for i in 0..8 {ex[i*DEPTH]=u32::from(m[i]);}
            for j in 0..3 {ex[TOWER+j]=u32::from(m[8+j]);}
            (ex,c)
        }).collect()
    }
}
fn s3_coeff(x:&Polynomial,y:&Polynomial)->(Polynomial,Polynomial,Polynomial) {
    let sum=x.plus(y);
    let pr=x.times(y);
    let a=x.minus(y).square();
    let b=sum.times(&pr.plus(&Polynomial::constant(A)))
        .plus(&Polynomial::constant(mul(2,B))).scaled(P-2);
    let c=pr.minus(&Polynomial::constant(A)).square()
        .minus(&sum.scaled(mul(4,B)));
    (a,b,c)
}
fn s3(x:&Polynomial,y:&Polynomial,z:&Polynomial)->Polynomial {
    let (a,b,c)=s3_coeff(x,y);
    a.times(&z.square()).plus(&b.times(z)).plus(&c)
}
// Resultant in t of S3(x,y,t)=a t^2+b t+c and S3(z,w,t)=d t^2+e t+f.
// This is the classical S4, no new polynomial identity is claimed.
fn s4(x:&Polynomial,y:&Polynomial,z:&Polynomial,w:&Polynomial)->Polynomial {
    let (a,b,c)=s3_coeff(x,y);
    let (d,e,f)=s3_coeff(z,w);
    a.times(&f).minus(&c.times(&d)).square().minus(
        &a.times(&e).minus(&b.times(&d))
        .times(&b.times(&f).minus(&c.times(&e))))
}
fn ring(terminal:u64)->TowerRing {
    let mut rules=Vec::with_capacity(TOWER);
    for block in 0..8 {
        for level in 0..DEPTH {
            let last=level+1==DEPTH;
            rules.push(SquareRule{
                next:(!last).then_some((block*DEPTH+level+1) as u8),
                a:0,b:u64::from(!last),c:0,
                d:if last {(terminal+2)%P} else {2},
            });
        }
    }
    TowerRing{p:P,rules,n_free:3}
}
fn four(code:u64)->[(usize,i8);4] {
    std::array::from_fn(|j| (
        ((code>>(6*j))&63) as usize,
        if ((code>>(24+j))&1)==0 {1} else {-1}
    ))
}
fn witness_points(lc:u64,rc:u64,base:&Base,ar:&Arithmetic)->Result<[(u64,u64);8],String> {
    let pairs:Vec<_>=four(lc).into_iter().chain(four(rc)).collect();
    if pairs.len()!=8 || pairs.iter().any(|(i,_)|*i>=base.points.len())
        || pairs.iter().map(|(i,_)|*i).collect::<HashSet<_>>().len()!=8
    {return Err("m8 witness indices invalid or repeated".into());}
    let pts:Vec<_>=pairs.into_iter().map(|(i,s)| {
        let v=Some(base.points[i]);
        if s<0 {ar.neg(PRIME31,v).unwrap()} else {v.unwrap()}
    }).collect();
    pts.try_into().map_err(|_|"wrong m8 point count".to_owned())
}
fn summands(ar:&mut Arithmetic,ps:&[(u64,u64)],start:usize,end:usize)->Point {
    let mut t=None;
    for &pt in &ps[start..end] {t=ar.add(PRIME31,t,Some(pt));}
    t
}
#[derive(Deserialize)]
struct Transcript { witnesses:Vec<[u64;4]> }
fn main()->Result<(),String>{
    let e=PRIME31;
    let mut ar=Arithmetic::default();
    check_curve(e,&mut ar)?;
    if e.p!=P || e.a!=A || e.b!=B {return Err("curve changed".into());}
    let fb=Base::build(e,DEPTH,&mut ar)?;
    if fb.terminal!=933_677_148 || fb.points.len()!=31 {return Err("factor base changed".into());}
    let transcript:Transcript=serde_json::from_str(include_str!("../M8_SPLIT_CONTROL.results.json"))
        .map_err(|x|x.to_string())?;
    if transcript.witnesses.len()!=32 {return Err("transcript count changed".into());}
    let ring=ring(fb.terminal);
    ring.check();
    let leaf:Vec<_>=(0..8).map(Polynomial::variable).collect();
    let (u,v,w)=(Polynomial::variable(8),Polynomial::variable(9),Polynomial::variable(10));
    let fixed=[s4(&leaf[0],&leaf[1],&leaf[2],&u),
               s4(&leaf[3],&leaf[4],&leaf[5],&v),
               s3(&leaf[6],&leaf[7],&w)];
    let polys:Vec<_>=fixed.iter().map(|p|ring.from_raw(&p.raw_tower())).collect();
    let nf_terms:Vec<_>=polys.iter().map(|p|p.len()).collect();
    if nf_terms!=[526,526,16] {return Err(format!("F4 tower input terms changed: {nf_terms:?}"));}
    let mut checked=0;
    let mut mutated=0;
    let mut first_aux=None;
    for [aa,bb,lc,rc] in transcript.witnesses {
        let ps=witness_points(lc,rc,&fb,&ar)?;
        let u_pt=summands(&mut ar,&ps,0,3).ok_or("u=O: require separate guard")?;
        let v_pt=summands(&mut ar,&ps,3,6).ok_or("v=O: require separate guard")?;
        let w_pt=summands(&mut ar,&ps,6,8).ok_or("w=O: require separate guard")?;
        let ag=ar.scalar(e,aa,Some(e.g));
        let bq=ar.scalar(e,bb,Q);
        let r=ar.add(e,ag,bq).ok_or("R=O: require separate guard")?;
        let uv=ar.add(e,Some(u_pt),Some(v_pt));
        if ar.add(e,uv,Some(w_pt))!=Some(r) {
            return Err("signed-y lift / actual public target mismatch".into());
        }
        let mut xs=[0_u64;NV];
        for i in 0..8 {
            xs[i*DEPTH]=ps[i].0;
            for j in 1..DEPTH {
                let prior=xs[i*DEPTH+j-1];
                xs[i*DEPTH+j]=sub(mul(prior,prior),2);
            }
            if mul(xs[i*DEPTH+DEPTH-1],xs[i*DEPTH+DEPTH-1]) != (fb.terminal+2)%P {
                return Err("witness not in Dickson fibre".into());
            }
        }
        xs[TOWER]=u_pt.0;xs[TOWER+1]=v_pt.0;xs[TOWER+2]=w_pt.0;
        for poly in &polys {if ring.eval(poly,&xs)!=0 {return Err("S3/S4 fixed polynomial residual !=0".into());}}
        let terminal=s4(&u,&v,&w,&Polynomial::constant(r.0));
        let final_poly=ring.from_raw(&terminal.raw_tower());
        if ring.eval(&final_poly,&xs)!=0 {return Err("S4 target polynomial residual !=0".into());}
        let corrupted=s4(&u,&v,&w,&Polynomial::constant(add(r.0,1)));
        if ring.eval(&ring.from_raw(&corrupted.raw_tower()),&xs)!=0 {mutated+=1;}
        if first_aux.is_none(){first_aux=Some([u_pt.0,v_pt.0,w_pt.0,r.0]);}
        checked+=1;
    }
    if checked!=32 || mutated!=32
        || first_aux!=Some([803_189_040,898_740_481,729_813_404,903_384_438]) {
        return Err("frozen balanced m8 certificate controls changed".into());
    }
    println!("{}",json!({"status":"[pcc:unverified]",
      "scope":"F4 tower-input construction and existing generic-witness semantic bridge; NOT an F4 relation oracle or cheaper ECDLP",
      "p":P,"n":e.n,"depth":DEPTH,"B":fb.points.len(),
      "tower_vars":TOWER,"free_vars":3,"normal_form_terms_fixed":nf_terms,
      "balanced_m8_natural_witnesses_checked":checked,
      "mutated_target_x_rejected":mutated,
      "first_auxiliary_x":first_aux,
      "rust_compilation_status":"must be confirmed on exact GitHub head"}));
    Ok(())
}