//! Target-conditioned balanced m8 using the exact oriented chord-tangent lift
//! of the final S3 pair. This is a bounded Rust F4 INPUT/RESOURCE gate, not an
//! index-calculus root extractor or complete ECDLP solver. [pcc:unverified]
//!
//! Uses existing prime31 Arithmetic, Base and the native f4_fp_tower F4 engine.
//! One known positive is used only for checking polynomial soundness;
//! the frozen natural public panel's private witness labels are not read.
//!
//! cargo +1.100.0 test --locked --release --manifest-path research/prime31_ic_20261115/native/Cargo.toml --example prime31_m8_oriented_f4
//! cargo +1.100.0 run --locked --release --manifest-path research/prime31_ic_20261115/native/Cargo.toml --example prime31_m8_oriented_f4 -- --panel
use prime31_algebraic_ic::{
    check_curve, Arithmetic, Base, Point, PRIME31,
    f4_fp_tower::{f4_tower, RPoly, SquareRule, TowerF4Options, TowerRing},
};
use serde::Deserialize;
use serde_json::{json, Value};
use std::collections::BTreeMap;
use std::time::{Duration, Instant};

const P: u64 = 2_147_483_647;
const A: u64 = 847_514_846;
const B: u64 = 2_087_444_724;
const DEPTH: usize = 6;
const BLOCKS: usize = 8;
const NT: usize = DEPTH * BLOCKS;
const NV: usize = 12; // x0..x7,u,v,y,t; original uses w in place of y.
const Q: Point = Some((1_547_953_785, 1_667_575_595));

fn add(a:u64,b:u64)->u64{let c=a+b;if c>=P{c-P}else{c}}
fn sub(a:u64,b:u64)->u64{if a>=b{a-b}else{a+P-b}}
fn mul(a:u64,b:u64)->u64{((a as u128*b as u128)%P as u128) as u64}
fn pow(mut b:u64,mut e:u64)->u64{
    let mut a=1;
    while e>0{if e&1==1{a=mul(a,b);}b=mul(b,b);e>>=1;}a
}
fn inv(v:u64)->u64{assert_ne!(v,0);pow(v,P-2)}

#[derive(Clone,Default)]
struct Poly(BTreeMap<[u8;NV],u64>);
impl Poly {
    fn constant(v:u64)->Self{
        if v%P==0{return Self::default();}
        Self(BTreeMap::from([([0;NV],v%P)]))
    }
    fn variable(i:usize)->Self{
        assert!(i<NV);
        let mut e=[0;NV];e[i]=1;
        Self(BTreeMap::from([(e,1)]))
    }
    fn plus(&self,rhs:&Self)->Self{
        let mut out=self.0.clone();
        for (m,&c) in &rhs.0{
            let v=add(*out.get(m).unwrap_or(&0),c);
            if v==0{out.remove(m);}else{out.insert(*m,v);}
        }
        Self(out)
    }
    fn minus(&self,rhs:&Self)->Self{
        self.plus(&rhs.scaled(P-1))
    }
    fn scaled(&self,c:u64)->Self{
        let mut out=BTreeMap::new();
        for (&m,&v) in &self.0{
            let z=mul(v,c%P);if z!=0{out.insert(m,z);}
        }
        Self(out)
    }
    fn times(&self,rhs:&Self)->Self{
        let mut out=BTreeMap::new();
        for (m,&x) in &self.0{
            for (n,&y) in &rhs.0{
                let mut ex=[0;NV];
                for k in 0..NV {
                    ex[k]=m[k].checked_add(n[k]).expect("symbolic exponent overflow");
                }
                let v=add(*out.get(&ex).unwrap_or(&0),mul(x,y));
                if v==0{out.remove(&ex);}else{out.insert(ex,v);}
            }
        }
        Self(out)
    }
    fn square(&self)->Self{self.times(self)}
    fn raw(&self,n_free:usize)->Result<Vec<(Vec<u32>,u64)>,String>{
        if n_free!=3&&n_free!=4 {return Err("only native free 3/4 supported".into());}
        if n_free==3&&self.0.keys().any(|e|e[11]!=0) {
            return Err("original x-only S3 input depends on slope t".into());
        }
        Ok(self.0.iter().map(|(m,&c)|{
            let mut ex=vec![0u32;NT+n_free];
            for i in 0..BLOCKS {ex[i*DEPTH]=u32::from(m[i]);}
            for j in 0..n_free {ex[NT+j]=u32::from(m[BLOCKS+j]);}
            (ex,c)
        }).collect())
    }
    fn eval(&self,variables:&[u64;NV])->u64{
        let mut sum=0;
        for (ex,&c) in &self.0{
            let mut v=c;
            for (i,&power) in ex.iter().enumerate(){
                if power>0{v=mul(v,pow(variables[i],u64::from(power)));}
            }
            sum=add(sum,v);
        }
        sum
    }
}
fn s3_coeff(x:&Poly,y:&Poly)->(Poly,Poly,Poly){
    let sum=x.plus(y);
    let prod=x.times(y);
    let aa=x.minus(y).square();
    let bb=sum.times(&prod.plus(&Poly::constant(A)))
        .plus(&Poly::constant(mul(2,B))).scaled(P-2);
    let cc=prod.minus(&Poly::constant(A)).square()
        .minus(&sum.scaled(mul(4,B)));
    (aa,bb,cc)
}
fn s3(x:&Poly,y:&Poly,z:&Poly)->Poly{
    let (a,b,c)=s3_coeff(x,y);
    a.times(&z.square()).plus(&b.times(z)).plus(&c)
}
fn s4(x:&Poly,y:&Poly,z:&Poly,w:&Poly)->Poly{
    let (a,b,c)=s3_coeff(x,y);
    let (d,e,f)=s3_coeff(z,w);
    a.times(&f).minus(&c.times(&d)).square().minus(
        &a.times(&e).minus(&b.times(&d))
            .times(&b.times(&f).minus(&c.times(&e))))
}
fn ring(terminal:u64,n_free:usize)->TowerRing{
    let mut rules=Vec::with_capacity(NT);
    for block in 0..BLOCKS {
        for level in 0..DEPTH {
            let last=level+1==DEPTH;
            rules.push(SquareRule{
                next:(!last).then_some((block*DEPTH+level+1) as u8),
                a:0,b:u64::from(!last),c:0,
                d:if last{(terminal+2)%P}else{2},
            });
        }
    }
    TowerRing{p:P,rules,n_free}
}
fn polys_original(leaf:&[Poly],u:&Poly,v:&Poly,w:&Poly,target_x:u64)->Vec<Poly>{
    vec![
        s4(&leaf[0],&leaf[1],&leaf[2],u),
        s4(&leaf[3],&leaf[4],&leaf[5],v),
        s3(&leaf[6],&leaf[7],w),
        s4(u,v,w,&Poly::constant(target_x)),
    ]
}
fn polys_oriented(leaf:&[Poly],u:&Poly,v:&Poly,y:&Poly,t:&Poly,target_x:u64)
    ->Vec<Poly>
{
    let x=&leaf[6];
    let z=&leaf[7];
    let w=t.square().minus(x).minus(z);
    let f=x.square().times(x).plus(&x.scaled(A)).plus(&Poly::constant(B));
    let tangent_rhs=x.square().plus(&x.times(z)).plus(&z.square())
        .plus(&Poly::constant(A));
    vec![
        s4(&leaf[0],&leaf[1],&leaf[2],u),
        s4(&leaf[3],&leaf[4],&leaf[5],v),
        y.square().minus(&f),
        y.times(t).scaled(2).plus(&z.minus(x).times(&t.square()))
            .minus(&tangent_rhs),
        s4(u,v,&w,&Poly::constant(target_x)),
    ]
}
fn witness_four(code:u64)->[(usize,i8);4]{
    std::array::from_fn(|j|(
        ((code>>(6*j))&63) as usize,
        if ((code>>(24+j))&1)==0{1}else{-1}
    ))
}
fn signed_points(base:&Base,left:u64,right:u64,ar:&Arithmetic)
    ->Result<Vec<(u64,u64)>,String>
{
    let mut out=Vec::new();
    for (i,sign) in witness_four(left).into_iter().chain(witness_four(right)){
        let point=base.points.get(i).ok_or("invalid planted index")?;
        let q=if sign>0{*point}else{ar.neg(PRIME31,Some(*point)).unwrap()};
        out.push(q);
    }
    if out.len()!=8{return Err("bad fixture length".into());}
    Ok(out)
}
fn sum_points(pts:&[(u64,u64)],ar:&mut Arithmetic)->Point{
    pts.iter().fold(None,|p,&q|ar.add(PRIME31,p,Some(q)))
}
#[derive(Deserialize)]
struct Witnesses{witnesses:Vec<[u64;4]>}
#[derive(Deserialize)]
struct Panel{rows:Vec<PublicRow>}
#[derive(Deserialize)]
struct PublicRow{index:usize,target:[u64;2]}

fn assert_positive_fixture(base:&Base,ar:&mut Arithmetic)->Result<Value,String>{
    let data:Witnesses=serde_json::from_str(include_str!("../M8_SPLIT_CONTROL.results.json"))
        .map_err(|e|e.to_string())?;
    let [a,b,left,right]=*data.witnesses.first().ok_or("missing planted witness")?;
    let pts=signed_points(base,left,right,ar)?;
    let u=sum_points(&pts[..3],ar).ok_or("left sum identity")?;
    let v=sum_points(&pts[3..6],ar).ok_or("right sum identity")?;
    let w=sum_points(&pts[6..],ar).ok_or("pair sum identity")?;
    let target=sum_points(&pts,ar).ok_or("full sum identity")?;
    let q1=ar.scalar(PRIME31,a,Some(PRIME31.g));
    let q2=ar.scalar(PRIME31,b,Q);
    if ar.add(PRIME31,q1,q2)!=Some(target) {
        return Err("planted public target mismatch".into());
    }
    let (x1,y)=pts[6];let (x2,y2)=pts[7];
    let slope=if x1!=x2{
        mul(sub(y2,y),inv(sub(x2,x1)))
    }else{
        if y!=y2{return Err("planted pair is vertical".into());}
        mul(add(mul(3,mul(x1,x1)),A),inv(mul(2,y)))
    };
    if sub(sub(mul(slope,slope),x1),x2)!=w.0{
        return Err("planted chord tangent abscissa mismatch".into());
    }
    let leaf:Vec<_>=(0..8).map(Poly::variable).collect();
    let (uvar,vvar,yvar,tvar)=(Poly::variable(8),Poly::variable(9),
                               Poly::variable(10),Poly::variable(11));
    let oriented=polys_oriented(&leaf,&uvar,&vvar,&yvar,&tvar,target.0);
    let wvar=Poly::variable(10);
    let standard=polys_original(&leaf,&uvar,&vvar,&wvar,target.0);
    let mut coords=[0u64;NV];
    for (i,pt) in pts.iter().enumerate(){coords[i]=pt.0;}
    coords[8]=u.0;coords[9]=v.0;coords[10]=y;coords[11]=slope;
    if oriented.iter().any(|p|p.eval(&coords)!=0){
        return Err("oriented system rejects full planted signed witness".into());
    }
    coords[10]=w.0;coords[11]=0;
    if standard.iter().any(|p|p.eval(&coords)!=0){
        return Err("original S4/S3 system rejects full planted witness".into());
    }
    Ok(json!({
        "public_target":target,
        "fixed_positive_witness_only":true,
        "standard_polynomials_vanish":4,
        "oriented_polynomials_vanish":5,
        "note":"generic EC arithmetic constructs validator only; no certificate enters public panel discovery"
    }))
}
fn f4_result(input:&[RPoly],r:&TowerRing)->Value{
    let opts=TowerF4Options::new(12).with_budget(Duration::from_secs(5))
        .with_max_nnz(15_000).with_max_dense(15_000);
    let started=Instant::now();
    let rep=f4_tower(input,r,&opts);
    let wall=started.elapsed().as_nanos();
    let censored=rep.oversize||rep.timed_out||rep.pairs_above_bound>0
        ||rep.staircase_at_stop.is_some();
    json!({
        "verdict":if censored{"undetermined"}else if rep.inconsistent{
            "complete_refutation"
        }else{"basis_only_no_root_extraction"},
        "oversize":rep.oversize,"timed_out":rep.timed_out,
        "degree_reached":rep.degree_reached,
        "solving_degree_max":rep.solving_degree_max,
        "max_cols":rep.max_cols,"max_nnz":rep.max_nnz,
        "max_dense_entries":rep.max_dense_entries,
        "muladds":rep.muladds,"steps":rep.steps,
        "basis_elements":rep.basis.len(),
        "whole_f4_wall_ns":wall,
        "f4_internal_ms":rep.ms,
        "root_extraction":"not_implemented",
        "caps":{"degree":12,"nnz":15_000,"dense":15_000,"seconds":5}
    })
}
fn trial()->Result<Value,String>{
    let mut ar=Arithmetic::default();
    check_curve(PRIME31,&mut ar)?;
    let base=Base::build(PRIME31,DEPTH,&mut ar)?;
    if base.points.len()!=31||base.terminal!=933_677_148{
        return Err("B31 fixture changed".into());
    }
    let positive=assert_positive_fixture(&base,&mut ar)?;
    let panel:Panel=serde_json::from_str(include_str!("../M8_NATURAL_F4_PANEL_20261115.json"))
        .map_err(|e|e.to_string())?;
    if panel.rows.len()!=16{return Err("16 natural target fixture changed".into());}
    let mut seen=std::collections::BTreeSet::new();
    let leaf:Vec<_>=(0..8).map(Poly::variable).collect();
    let (u,v,y,t)=(Poly::variable(8),Poly::variable(9),
        Poly::variable(10),Poly::variable(11));
    let xb=Poly::variable(10);
    let rb=ring(base.terminal,3);rb.check();
    let ro=ring(base.terminal,4);ro.check();
    let setup_started=Instant::now();
    let original_fixed=polys_original(&leaf,&u,&v,&xb,PRIME31.g.0);
    let ori_fixed=polys_oriented(&leaf,&u,&v,&y,&t,PRIME31.g.0);
    let orig_inputs_fixed:Vec<_>=original_fixed[..3].iter()
        .map(|p|rb.from_raw(&p.raw(3).unwrap())).collect();
    let ori_inputs_fixed:Vec<_>=ori_fixed[..4].iter()
        .map(|p|ro.from_raw(&p.raw(4).unwrap())).collect();
    let fixed_prep_ns=setup_started.elapsed().as_nanos();
    let orig_terms:Vec<_>=orig_inputs_fixed.iter().map(RPoly::len).collect();
    if orig_terms!=[526,526,16]{
        return Err(format!("original balanced S4/S3 fixed normal forms changed: {orig_terms:?}"));
    }
    let mut outcomes=Vec::new();
    for row in panel.rows{
        if !seen.insert(row.index){return Err("duplicate public panel index".into());}
        let rx=row.target[0];
        let target_poly_original=s4(&u,&v,&xb,&Poly::constant(rx));
        let target_poly_oriented=s4(&u,&v,&t.square().minus(&leaf[6]).minus(&leaf[7]),
                                    &Poly::constant(rx));
        let started=Instant::now();
        let mut original=orig_inputs_fixed.clone();
        original.push(rb.from_raw(&target_poly_original.raw(3)?));
        let original_construct_ns=started.elapsed().as_nanos();
        let started=Instant::now();
        let mut oriented=ori_inputs_fixed.clone();
        oriented.push(ro.from_raw(&target_poly_oriented.raw(4)?));
        let oriented_construct_ns=started.elapsed().as_nanos();
        // Crucially, no a,b nor validation labels from PublicRow are read here.
        let old=f4_result(&original,&rb);
        let new=f4_result(&oriented,&ro);
        outcomes.push(json!({
            "index":row.index,"public_R":row.target,
            "original":{"poly_count":original.len(),
                "target_normal_form_terms":original.last().unwrap().len(),
                "target_construct_ns":original_construct_ns,
                "f4":old},
            "oriented":{"poly_count":oriented.len(),
                "target_normal_form_terms":oriented.last().unwrap().len(),
                "target_construct_ns":oriented_construct_ns,
                "f4":new}
        }));
    }
    Ok(json!({
        "standing":"[pcc:unverified]",
        "scope":"target-conditioned m8 Rust F4 diagnostic only; no roots/logarithms",
        "curve_p":P,"curve_order":PRIME31.n,"B":base.points.len(),
        "base":"inherited depth-six Dickson64 x preimages (31 liftable canonical points)",
        "original_fixed_nf_terms":orig_terms,
        "oriented_fixed_nf_terms":ori_inputs_fixed.iter().map(RPoly::len).collect::<Vec<_>>(),
        "fixed_symbolic_preprocessing_ns":fixed_prep_ns,
        "planted_positive_validator":positive,
        "all_public_targets_from_frozen_panel":true,
        "target_known_scalars_used_by_algebraic_solver":false,
        "generic_pair_tables_used_by_algebraic_solver":false,
        "outcomes":outcomes
    }))
}
fn main()->Result<(),String>{
    let args:Vec<_>=std::env::args().skip(1).collect();
    if args.len()>1||args.first().is_some_and(|s|s!="--panel"&&s!="--verify"){
        return Err("usage: prime31_m8_oriented_f4 [--verify|--panel]".into());
    }
    if args.first().is_some_and(|s|s=="--panel"){
        println!("{}",serde_json::to_string_pretty(&trial()?).map_err(|x|x.to_string())?);
    }else{
        let mut ar=Arithmetic::default();
        check_curve(PRIME31,&mut ar)?;
        let base=Base::build(PRIME31,DEPTH,&mut ar)?;
        println!("{}",serde_json::to_string_pretty(
            &assert_positive_fixture(&base,&mut ar)?).map_err(|x|x.to_string())?);
    }
    Ok(())
}
#[cfg(test)]
mod tests{
    #[test]
    fn oriented_and_classic_m8_vanish_on_actual_public_positive(){
        let mut ar=prime31_algebraic_ic::Arithmetic::default();
        prime31_algebraic_ic::check_curve(super::PRIME31,&mut ar).unwrap();
        let base=prime31_algebraic_ic::Base::build(super::PRIME31,super::DEPTH,&mut ar).unwrap();
        super::assert_positive_fixture(&base,&mut ar).unwrap();
    }
}
