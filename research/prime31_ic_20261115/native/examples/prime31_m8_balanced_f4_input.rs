//! Balanced S4/S3 m8 PKM polynomial input on the existing prime31 Dickson base.
//!
//! This is a SOURCE/SEMANTIC bridge to the existing Rust F4 tower, not an
//! F4 solve or a measured cheaper ECDLP algorithm.  The eight-fibre
//! chain needs only THREE ordinary free abscissae (u,v,w), not six:
//! S4(x0,x1,x2,u), S4(x3,x4,x5,v), S3(x6,x7,w), S4(u,v,w,x(R)).
//! All signed point lifts and the full public target are checked separately.
//! [pcc:unverified]
use prime31_algebraic_ic::{check_curve, f4_fp_tower::{f4_tower, RPoly, SquareRule, TowerF4Options, TowerRing}, Arithmetic, Base, Point, PRIME31};
use serde::Deserialize;
use serde_json::json;
use std::collections::{BTreeMap, HashSet};
use std::time::Duration;

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
// R31-F target-dependent development pilot: S3(x6,x7,w) was eliminated
// algebraically into a finite-root polynomial. With a fixed certified
// w-specialization, the other six leaves still use the inherited Dickson
// tower and the original S4 equations/engine. No generic group join is
// used to propose solutions; fixed positive fixtures validate soundness only.
fn six_ring(terminal:u64)->TowerRing {
    let mut rules=Vec::with_capacity(6*DEPTH);
    for block in 0..6 {
        for level in 0..DEPTH {
            let last=level+1==DEPTH;
            rules.push(SquareRule{
                next:(!last).then_some((block*DEPTH+level+1) as u8),
                a:0,b:u64::from(!last),c:0,
                d:if last {(terminal+2)%P} else {2},
            });
        }
    }
    TowerRing{p:P,rules,n_free:2}
}
fn six_input(poly:&Polynomial,ring:&TowerRing)->Result<RPoly,String> {
    let mut raw=Vec::new();
    for (monomial,&c) in &poly.0 {
        if monomial[6]!=0 || monomial[7]!=0 || monomial[10]!=0 {
            return Err("six-block polynomial retained eliminated x6/x7/w".into());
        }
        let mut powers=vec![0u32;6*DEPTH+2];
        for i in 0..6 {
            powers[i*DEPTH]=u32::from(monomial[i]);
        }
        powers[6*DEPTH]=u32::from(monomial[8]);
        powers[6*DEPTH+1]=u32::from(monomial[9]);
        raw.push((powers,c));
    }
    Ok(ring.from_raw(&raw))
}

// Same exact S4 resultant, but only its three active depth-six fibre blocks
// plus one free group-sum abscissa. Used solely for minimal-ideal ablation.
fn minimal_s4_ring(terminal:u64)->TowerRing {
    let mut rules=Vec::with_capacity(3*DEPTH);
    for block in 0..3 {
        for level in 0..DEPTH {
            let last=level+1==DEPTH;
            rules.push(SquareRule{
                next:(!last).then_some((block*DEPTH+level+1) as u8),
                a:0,b:u64::from(!last),c:0,
                d:if last {(terminal+2)%P} else {2},
            });
        }
    }
    TowerRing{p:P,rules,n_free:1}
}
fn minimal_s4_input(poly:&Polynomial,start:usize,free:usize,
    ring:&TowerRing)->Result<RPoly,String> {
    if start!=0 && start!=3 {return Err("invalid minimal S4 block".into());}
    if free!=8 && free!=9 {return Err("invalid minimal S4 free variable".into());}
    let mut raw=Vec::new();
    for (monomial,&coeff) in &poly.0 {
        if monomial.iter().enumerate().any(|(i,&pow)| pow!=0 &&
            !(i>=start && i<start+3) && i!=free) {
            return Err("minimal S4 still depends on eliminated coordinates".into());
        }
        let mut exps=vec![0u32;3*DEPTH+1];
        for j in 0..3 { exps[j*DEPTH]=u32::from(monomial[start+j]); }
        exps[3*DEPTH]=u32::from(monomial[free]);
        raw.push((exps,coeff));
    }
    Ok(ring.from_raw(&raw))
}


// EXP-R31-SPLIT: only three active Dickson fibres and TWO free x coordinates
// (outer sum u and internal pair sum h). This is a projected S4 presentation,
// not an extra target-dependent relation oracle.
fn split_s4_ring(terminal:u64)->TowerRing {
    let mut rules=Vec::with_capacity(3*DEPTH);
    for block in 0..3 {
        for level in 0..DEPTH {
            let last=level+1==DEPTH;
            rules.push(SquareRule{
                next:(!last).then_some((block*DEPTH+level+1) as u8),
                a:0,b:u64::from(!last),c:0,
                d:if last {(terminal+2)%P} else {2},
            });
        }
    }
    TowerRing{p:P,rules,n_free:2}
}
fn split_s4_input(poly:&Polynomial,ring:&TowerRing)->Result<RPoly,String>{
    let mut raw=Vec::new();
    for (m,&coeff) in &poly.0 {
        if m.iter().enumerate().any(|(i,&exp)| exp!=0 && i>=3 && i!=8 && i!=9) {
            return Err("split S4 constraint references a nonlocal leaf/free coordinate".into());
        }
        let mut exps=vec![0u32;3*DEPTH+2];
        for i in 0..3 {exps[i*DEPTH]=u32::from(m[i]);}
        exps[3*DEPTH]=u32::from(m[8]); // triple sum u
        exps[3*DEPTH+1]=u32::from(m[9]); // pair sum h
        raw.push((exps,coeff));
    }
    Ok(ring.from_raw(&raw))
}


 // EXP-R31-SPLIT: fixed-pair w is specialized, so the remaining six
 // Dickson leaves plus u,v and two internal S3 pair abscissae need only
 // 36 tower variables and four ordinary free variables (the engine limit).
fn six_chain_ring(terminal:u64)->TowerRing {
    let mut rules=Vec::with_capacity(6*DEPTH);
    for block in 0..6 {
        for level in 0..DEPTH {
            let last=level+1==DEPTH;
            rules.push(SquareRule{
                next:(!last).then_some((block*DEPTH+level+1) as u8),
                a:0,b:u64::from(!last),c:0,
                d:if last {(terminal+2)%P} else {2},
            });
        }
    }
    TowerRing{p:P,rules,n_free:4}
}
fn six_chain_input(poly:&Polynomial,ring:&TowerRing)->Result<RPoly,String>{
    let mut raw=Vec::new();
    for (m,&coeff) in &poly.0 {
        if m[10]!=0 {
            return Err("six-chain input retained eliminated fixed-pair w".into());
        }
        // Reuse former x6,x7 polynomial slots for auxiliary hL,hR;
        // they are NOT point leaves after certified fixed-pair projection.
        let mut exps=vec![0u32;6*DEPTH+4];
        for i in 0..6 {exps[i*DEPTH]=u32::from(m[i]);}
        exps[6*DEPTH]=u32::from(m[8]);   // u
        exps[6*DEPTH+1]=u32::from(m[9]); // v
        exps[6*DEPTH+2]=u32::from(m[6]); // hL
        exps[6*DEPTH+3]=u32::from(m[7]); // hR
        raw.push((exps,coeff));
    }
    Ok(ring.from_raw(&raw))
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
#[derive(Deserialize)]
struct Panel { rows:Vec<PanelRow> }
#[derive(Deserialize)]
struct PanelRow {
    index:usize,
    a:u64,
    b:u64,
    target:[u64;2],
    split_witness_count:usize,
    first_witness_codes:Option<[u64;2]>,
}
// Exact expected raw eight-summand witness count on a uniformly random
// prime-order public mask: each signed template maps to exactly one target.
// This counts templates WITH multiplicity, not distinct points or row rank.
fn choose(n:u128,k:u128)->u128 {
    if k>n {return 0;}
    let mut out=1_u128;
    for i in 1..=k.min(n-k) {out=out*(n+1-i)/i;}
    out
}

fn capped_f4(input:&[RPoly],ring:&TowerRing,known_witness:Option<&[u64]>)->Result<serde_json::Value,String>{
    // Deliberately tight pre-registered exploratory caps, not an EC solve.
    // Symbolic work and input formation are NOT included in muladds.
    let opts=TowerF4Options::new(12)
        .with_budget(Duration::from_secs(5))
        .with_max_nnz(15_000)
        .with_max_dense(15_000);
    let rep=f4_tower(input,ring,&opts);
    if let Some(xs)=known_witness {
        if rep.inconsistent {return Err("F4 refuted a witnessed positive prime31 ideal".into());}
        if rep.basis.iter().any(|g|ring.eval(g,xs)!=0) {
            return Err("F4 basis does not vanish at the known valid witness".into());
        }
    }
    let censored=rep.timed_out || rep.oversize || rep.pairs_above_bound>0
        || rep.staircase_at_stop.is_some();
    let verdict=if censored {"undetermined"} else if rep.inconsistent {
        "complete_refutation"
    } else {"basis_computed_without_root_extraction"};
    Ok(json!({"verdict":verdict,"timed_out":rep.timed_out,
        "oversize":rep.oversize,"pairs_above_bound":rep.pairs_above_bound,
        "staircase_at_stop":rep.staircase_at_stop,
        "inconsistent":rep.inconsistent,"degree_reached":rep.degree_reached,
        "solving_degree_max":rep.solving_degree_max,"steps":rep.steps,
        "max_cols":rep.max_cols,"max_nnz":rep.max_nnz,
        "max_dense_entries":rep.max_dense_entries,"muladds":rep.muladds,
        "reported_ms":rep.ms,"basis_elements":rep.basis.len(),
        "degree_cap":12,"nnz_cap":15_000,"dense_cap":15_000,
        "budget_seconds":5,"root_extraction":"not_implemented",
        "scope":"bounded F4 pilot only; no target logarithm recovered"}))
}
fn main()->Result<(),String>{
    let args:Vec<_>=std::env::args().skip(1).collect();
    if args.len()>1 || (!args.is_empty()
        && args[0]!="--probe-f4" && args[0]!="--panel-f4"
        && args[0]!="--ablate-f4" && args[0]!="--preeliminate-w-f4"
        && args[0]!="--split-s4-f4" && args[0]!="--m8-s3chain-f4"
        && args[0]!="--anchor2-m6-f4") {
        return Err("usage: prime31_m8_balanced_f4_input [--probe-f4|--panel-f4|--ablate-f4|--preeliminate-w-f4|--split-s4-f4|--m8-s3chain-f4]".into());
    }
    let probe_f4=args.first().is_some_and(|v|v=="--probe-f4");
    let panel_f4=args.first().is_some_and(|v|v=="--panel-f4");
    let ablate_f4=args.first().is_some_and(|v|v=="--ablate-f4");
    let preeliminate_w=args.first().is_some_and(|v|v=="--preeliminate-w-f4");
    let split_s4_f4=args.first().is_some_and(|v|v=="--split-s4-f4");
    let m8_s3chain_f4=args.first().is_some_and(|v|v=="--m8-s3chain-f4");
    let anchor2_m6_f4=args.first().is_some_and(|v|v=="--anchor2-m6-f4");
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


    // Two *public, charged* pinned factor-base points lower the remaining
    // eight-summand equation to an actual target-dependent six-summand ideal.
    // No pair-sum catalogue, scalar label, or known-root injection is used
    // on the frozen natural panel. This is a resource gate, not an ECDLP solve.
    if anchor2_m6_f4 {
        let r6=six_ring(fb.terminal);
        r6.check();
        let left=six_input(&fixed[0],&r6)?;
        let right=six_input(&fixed[1],&r6)?;
        let anchor_a=fb.points[0];
        let anchor_b=fb.points[1];
        let anchor_sum=ar.add(e,Some(anchor_a),Some(anchor_b))
            .ok_or("two fixed factor-base anchors cancel")?;
        let shifted_target=|r:(u64,u64),ar:&mut Arithmetic| -> Result<(u64,u64),String> {
            ar.add(e,Some(r),ar.neg(e,Some(anchor_sum)))
                .ok_or("anchored residual target is point at infinity".to_string())
        };
        let six:Vec<_>=fb.points.iter().copied().skip(2).take(6).collect();
        if six.len()!=6 {return Err("missing six positive-control leaves".into());}
        let residual=summands(&mut ar,&six,0,6)
            .ok_or("six-point known positive control is identity")?;
        let full=ar.add(e,Some(residual),Some(anchor_sum))
            .ok_or("known positive with anchors is identity")?;
        if shifted_target(full,&mut ar)?!=residual {
            return Err("anchor subtraction does not reconstruct positive target".into());
        }
        let pu=summands(&mut ar,&six,0,3).ok_or("known m6 u=O")?;
        let pv=summands(&mut ar,&six,3,6).ok_or("known m6 v=O")?;
        let mut known=vec![0u64;6*DEPTH+2];
        for (i,point) in six.iter().enumerate() {
            known[i*DEPTH]=point.0;
            for level in 1..DEPTH {
                let previous=known[i*DEPTH+level-1];
                known[i*DEPTH+level]=sub(mul(previous,previous),2);
            }
        }
        known[6*DEPTH]=pu.0;
        known[6*DEPTH+1]=pv.0;
        let positive_target=six_input(&s3(&u,&v,&Polynomial::constant(residual.0)),&r6)?;
        let positive_input=vec![left.clone(),right.clone(),positive_target];
        if positive_input.iter().any(|poly|r6.eval(poly,&known)!=0) {
            return Err("anchored m6 positive six-leaf witness lost in polynomial projection".into());
        }
        let positive=capped_f4(&positive_input,&r6,Some(&known))?;
        let panel:Panel=serde_json::from_str(
            include_str!("../M8_NATURAL_F4_PANEL_20261115.json"))
            .map_err(|err|err.to_string())?;
        if panel.rows.len()!=16
            || panel.rows.iter().filter(|row|row.split_witness_count>0).count()!=5 {
            return Err("anchored m6 frozen natural panel changed".into());
        }
        let mut outcomes=Vec::with_capacity(panel.rows.len());
        for (i,row) in panel.rows.iter().enumerate() {
            if row.index!=i || row.a>=e.n || row.b==0 || row.b>=e.n {
                return Err(format!("bad frozen mask at anchored row {i}"));
            }
            let ag=ar.scalar(e,row.a,Some(e.g));
            let bq=ar.scalar(e,row.b,Q);
            let r=ar.add(e,ag,bq)
                .ok_or("anchored natural public mask at infinity")?;
            if [r.0,r.1]!=row.target {
                return Err(format!("anchored natural public mask mismatch at row {i}"));
            }
            let shifted=shifted_target(r,&mut ar)?;
            let target_poly=s3(&u,&v,&Polynomial::constant(shifted.0));
            let input=vec![left.clone(),right.clone(),six_input(&target_poly,&r6)?];
            let f4=capped_f4(&input,&r6,None)?;
            outcomes.push(json!({
                "index":i,"public_R":r,"shifted_R":shifted,
                "registered_m8_restricted_split_positive":row.split_witness_count>0,
                "note":"old m8 split label is NOT a witness label for anchored m6",
                "f4":f4
            }));
        }
        println!("{}",json!({
            "standing":"[pcc:unverified]",
            "scope":"exact anchored six-summand target-dependent bounded Rust F4, no root extraction or ECDLP",
            "curve_prime":P,"curve_order":e.n,
            "base_size":fb.points.len(),
            "public_positive_anchors":[anchor_a,anchor_b],
            "anchor_sum":anchor_sum,
            "tower_variables_before":48,"sum_variables_before":3,
            "tower_variables_after":36,"sum_variables_after":2,
            "algebraic_equations":"S4(three leaves,u); S4(three leaves,v); S3(u,v,x(R-A-B))",
            "positive_six_leaf_control":{"public_target":full,
                "residual_target":residual,"f4":positive,
                "witness_used_only_for_f4_soundness":true},
            "natural_panel_count":outcomes.len(),"natural_outcomes":outcomes,
            "cost_note":"F4 ms and muladds omit input construction, curve operations, root extraction and full cold solve",
            "correctness_note":"No censored run is a negative relation proof; signed point-lift and exception handling required"
        }));
        return Ok(());
    }

    if split_s4_f4 {
        // Matched, bounded presentation ablation on an *already witnessed*
        // ordinary prime31 point triple. No known scalar enters either F4.
        // Projecting the chained S3 ideal eliminates the added h variable
        // and recovers the S4 condition only on the regular affine branch.
        // An oversize/censored result is NOT a refutation or a relation.
        let first=*transcript.witnesses.first().ok_or("missing fixed m8 witness")?;
        let pts=witness_points(first[2],first[3],&fb,&ar)?;
        let first_pair=ar.add(e,Some(pts[0]),Some(pts[1]));
        let triple=ar.add(e,first_pair,Some(pts[2]))
            .ok_or("left three-sum is identity")?;
        let base_r=minimal_s4_ring(fb.terminal);
        let split_r=split_s4_ring(fb.terminal);
        base_r.check();
        split_r.check();
        let old=minimal_s4_input(&fixed[0],0,8,&base_r)?;
        let mut old_assignment=vec![0u64;3*DEPTH+1];
        for i in 0..3 {
            old_assignment[i*DEPTH]=pts[i].0;
            for j in 1..DEPTH {
                let t=old_assignment[i*DEPTH+j-1];
                old_assignment[i*DEPTH+j]=sub(mul(t,t),2);
            }
        }
        old_assignment[3*DEPTH]=triple.0;
        if base_r.eval(&old,&old_assignment)!=0 {
            return Err("original S4 lost the inherited positive witness".into());
        }
        let baseline=capped_f4(&[old.clone()],&base_r,Some(&old_assignment))?;
        let h=Polynomial::variable(9);
        let choices=[(0usize,1usize,2usize),(0,2,1),(1,2,0)];
        let mut presentations=Vec::with_capacity(choices.len());
        for (i,j,k) in choices {
            let pair=ar.add(e,Some(pts[i]),Some(pts[j]))
                .ok_or("pair-sum identity: regular-branch experiment not admissible")?;
            let check=ar.add(e,Some(pair),Some(pts[k]))
                .ok_or("triple-sum identity")?;
            if check!=triple {return Err("S3 chaining changed the triple sum".into());}
            let first_s3=split_s4_input(&s3(&leaf[i],&leaf[j],&h),&split_r)?;
            let second_s3=split_s4_input(&s3(&h,&leaf[k],&u),&split_r)?;
            let mut assignment=vec![0u64;3*DEPTH+2];
            assignment[..3*DEPTH].copy_from_slice(&old_assignment[..3*DEPTH]);
            assignment[3*DEPTH]=triple.0;
            assignment[3*DEPTH+1]=pair.0;
            if split_r.eval(&first_s3,&assignment)!=0
                || split_r.eval(&second_s3,&assignment)!=0 {
                return Err("chained S3 does not retain regular-branch witness".into());
            }
            let term_counts=[first_s3.len(),second_s3.len()];
            let result=capped_f4(&[first_s3,second_s3],&split_r,Some(&assignment))?;
            presentations.push(json!({
                "pair_indices":[i,j],"remaining_index":k,
                "pair_sum_x_witness_validator_only":pair.0,
                "normal_form_terms":term_counts,
                "f4":result
            }));
        }
        println!("{}",json!({
            "standing":"[pcc:unverified]",
            "curve":"ordinary cofactor-one prime31",
            "p":P,"n":e.n,"B":fb.points.len(),
            "scope":"fixed three-point S4 presentation comparison, not m8 or any complete ECDLP",
            "original_s4":{"free_variables":1,"tower_blocks":3,
                "normal_form_terms":old.len(),"f4":baseline},
            "chained_s3":{"free_variables":2,"tower_blocks":3,
                "presentations":presentations},
            "all_regular_witnesses_checked":true,
            "root_extraction":"not implemented",
            "cold_solver_timing":"not measured",
            "frontier":"compare partial algebraic work only; censor all capped arms"
        }));
        return Ok(());
    }


    if m8_s3chain_f4 {
        // A genuinely target-dependent polynomial SYSTEM, not algebraic root
        // extraction or complete ECDLP: first choose a certified rational
        // fixed-pair w, then replace both triple S4s by chained S3 equations.
        // No known target logarithm or generic pair-sum table enters F4.
        let r6=six_chain_ring(fb.terminal);
        r6.check();
        let known=*transcript.witnesses.first().ok_or("missing m8 positive control")?;
        let pts=witness_points(known[2],known[3],&fb,&ar)?;
        let left_pair=summands(&mut ar,&pts,0,2).ok_or("left pair is identity")?;
        let left_triple=summands(&mut ar,&pts,0,3).ok_or("left triple is identity")?;
        let right_pair=summands(&mut ar,&pts,3,5).ok_or("right pair is identity")?;
        let right_triple=summands(&mut ar,&pts,3,6).ok_or("right triple is identity")?;
        let fixed_pair=summands(&mut ar,&pts,6,8).ok_or("fixed pair is identity")?;
        let known_left_right=ar.add(e,Some(left_triple),Some(right_triple));
        let known_sum=ar.add(e,known_left_right,Some(fixed_pair));
        let ag=ar.scalar(e,known[0],Some(e.g));
        let bq=ar.scalar(e,known[1],Q);
        let known_public=ar.add(e,ag,bq).ok_or("public positive target is identity")?;
        if known_sum!=Some(known_public) {
            return Err("test-only known natural positive is inconsistent".into());
        }
        let leaf_left_pair=Polynomial::variable(6);  // hL, NOT former x6
        let leaf_right_pair=Polynomial::variable(7); // hR, NOT former x7
        let constraints=[
            s3(&leaf[0],&leaf[1],&leaf_left_pair),
            s3(&leaf_left_pair,&leaf[2],&u),
            s3(&leaf[3],&leaf[4],&leaf_right_pair),
            s3(&leaf_right_pair,&leaf[5],&v),
        ];
        let fixed_input:Vec<_>=constraints.iter()
            .map(|p|six_chain_input(p,&r6)).collect::<Result<_,_>>()?;
        let terms:Vec<_>=fixed_input.iter().map(|q|q.len()).collect();
        let mut root=vec![0u64;6*DEPTH+4];
        for i in 0..6 {
            root[i*DEPTH]=pts[i].0;
            for j in 1..DEPTH {
                let prior=root[i*DEPTH+j-1];
                root[i*DEPTH+j]=sub(mul(prior,prior),2);
            }
        }
        root[6*DEPTH]=left_triple.0;
        root[6*DEPTH+1]=right_triple.0;
        root[6*DEPTH+2]=left_pair.0;
        root[6*DEPTH+3]=right_pair.0;
        if fixed_input.iter().any(|poly|r6.eval(poly,&root)!=0) {
            return Err("chained S3 lost the verified natural positive assignment".into());
        }
        let cases=[
            ("known_natural_positive_validator_only",fixed_pair.0,known_public.0,true),
            ("public_Q_branch_0",2_438_837,Q.unwrap().0,false),
            ("public_Q_branch_1",60_956_599,Q.unwrap().0,false),
            ("public_Q_branch_2",10_280_552,Q.unwrap().0,false),
            ("public_Q_branch_3",7_087_371,Q.unwrap().0,false),
            ("public_Q_branch_4",9_730_781,Q.unwrap().0,false),
            ("public_Q_branch_5",2_344_949,Q.unwrap().0,false),
            ("public_Q_diagonal",19_657_426,Q.unwrap().0,false),
        ];
        let mut records=Vec::new();
        for (name,wx,rx,positive) in cases {
            let final_eq=s4(&u,&v,&Polynomial::constant(wx),
                &Polynomial::constant(rx));
            let last=six_chain_input(&final_eq,&r6)?;
            let mut system=fixed_input.clone();
            system.push(last);
            let witness=positive.then_some(root.as_slice());
            if let Some(wit)=witness {
                if system.iter().any(|p|r6.eval(p,wit)!=0) {
                    return Err("positive target S4 not satisfied by chained witness".into());
                }
            }
            let report=capped_f4(&system,&r6,witness)?;
            records.push(json!({"arm":name,"w":wx,"target_x":rx,
                "known_witness_is_validator_only":positive,"f4":report}));
        }
        // Same-host old S4 control for the positive frozen branch.
        let old_r=six_ring(fb.terminal);
        old_r.check();
        let old_fixed=[six_input(&fixed[0],&old_r)?,
            six_input(&fixed[1],&old_r)?,
            six_input(&s4(&u,&v,&Polynomial::constant(fixed_pair.0),
                &Polynomial::constant(known_public.0)),&old_r)?];
        let mut old_root=vec![0u64;6*DEPTH+2];
        old_root[..6*DEPTH].copy_from_slice(&root[..6*DEPTH]);
        old_root[6*DEPTH]=left_triple.0;
        old_root[6*DEPTH+1]=right_triple.0;
        if old_fixed.iter().any(|poly|old_r.eval(poly,&old_root)!=0) {
            return Err("old six-block S4 witness lost".into());
        }
        let old_report=capped_f4(&old_fixed,&old_r,Some(&old_root))?;
        println!("{}",json!({
            "standing":"[pcc:unverified]",
            "curve":"ordinary prime31, full prime-order cofactor one",
            "p":P,"n":e.n,"factor_base":fb.points.len(),
            "scope":"bounded target-dependent m8 S3-chain F4 feasibility; not a complete oracle",
            "construction":"after fixed pair S3 is eliminated; 6 Dickson blocks, 4 free abscissae",
            "original_same_host_positive_control":{
                "three_S4_constraints":true,"tower_blocks":6,"free_variables":2,
                "f4":old_report},
            "chained_S3_constraints":4,
            "target_S4_constraints":1,
            "fixed_input_normal_form_terms":terms,
            "regular_positive_witness_checked":true,
            "certified_w_branch_catalogue_total":961,
            "w_branches_tested":8,
            "all_961_branches_tested":false,
            "actual_root_extraction":false,
            "outcomes":records,
            "classification":"All censored calls are undetermined; no full cold solver cost"
        }));
        return Ok(());
    }

    if preeliminate_w {
        // Frozen selection: one previously documented positive control and
        // seven deterministic first roots (one per first-collision arm).
        // All seven public-Q controls run, regardless of outcome. This is a
        // bounded *one-root-per-branch* F4 diagnostic, not a 961-root oracle.
        let r6=six_ring(fb.terminal);
        r6.check();
        let left=six_input(&fixed[0],&r6)?;
        let right=six_input(&fixed[1],&r6)?;
        let first=*transcript.witnesses.first().ok_or("no natural certificate")?;
        let ps=witness_points(first[2],first[3],&fb,&ar)?;
        let aux_w=summands(&mut ar,&ps,6,8).ok_or("known pair sum is identity")?;
        let aux_u=summands(&mut ar,&ps,0,3).ok_or("known left sum is identity")?;
        let aux_v=summands(&mut ar,&ps,3,6).ok_or("known right sum is identity")?;
        let first_ag=ar.scalar(e,first[0],Some(e.g));
        let first_bq=ar.scalar(e,first[1],Q);
        let r=ar.add(e,first_ag,first_bq).ok_or("known R is identity")?;
        if (aux_u.0,aux_v.0,aux_w.0,r.0)
            != (803_189_040,898_740_481,729_813_404,903_384_438) {
            return Err("known positive auxiliary witness changed".into());
        }
        let mut known=vec![0u64;6*DEPTH+2];
        for (i,point) in ps.iter().enumerate().take(6) {
            known[i*DEPTH]=point.0;
            for j in 1..DEPTH {
                let prev=known[i*DEPTH+j-1];
                known[i*DEPTH+j]=sub(mul(prev,prev),2);
            }
        }
        known[6*DEPTH]=aux_u.0;
        known[6*DEPTH+1]=aux_v.0;
        let cases=[
            ("known_positive_validator_only",aux_w.0,r.0,true),
            ("public_Q_branch_0",2_438_837,Q.unwrap().0,false),
            ("public_Q_branch_1",60_956_599,Q.unwrap().0,false),
            ("public_Q_branch_2",10_280_552,Q.unwrap().0,false),
            ("public_Q_branch_3",7_087_371,Q.unwrap().0,false),
            ("public_Q_branch_4",9_730_781,Q.unwrap().0,false),
            ("public_Q_branch_5",2_344_949,Q.unwrap().0,false),
            ("public_Q_diagonal",19_657_426,Q.unwrap().0,false),
        ];
        let mut outcomes=Vec::new();
        for (label,wx,rx,positive) in cases {
            let final_s4=s4(&u,&v,&Polynomial::constant(wx),
                &Polynomial::constant(rx));
            let final_input=six_input(&final_s4,&r6)?;
            let system=vec![left.clone(),right.clone(),final_input];
            let witness=if positive {
                if system.iter().any(|poly|r6.eval(poly,&known)!=0) {
                    return Err("six-block specialization lost known positive root".into());
                }
                Some(known.as_slice())
            } else { None };
            let result=capped_f4(&system,&r6,witness)?;
            outcomes.push(json!({
                "arm":label, "public_r_x":rx,"certified_pair_w":wx,
                "known_positive_validator_only":positive,
                "polynomial_count":system.len(),
                "f4":result
            }));
        }
        // Frozen diagnostic ablations of the *same* six-tower positive ideal.
        // These are strict subsets, never relation oracles. Their only purpose
        // is to identify the first reducer causing the remaining S4 stop.
        let f_known=six_input(&s4(&u,&v,&Polynomial::constant(aux_w.0),
            &Polynomial::constant(r.0)),&r6)?;
        let diagnostic_arms:Vec<(&str,Vec<RPoly>)>=vec![
            ("left_only",vec![left.clone()]),
            ("right_only",vec![right.clone()]),
            ("target_only",vec![f_known.clone()]),
            ("left_plus_right",vec![left.clone(),right.clone()]),
            ("left_plus_target",vec![left.clone(),f_known.clone()]),
            ("right_plus_target",vec![right.clone(),f_known.clone()]),
        ];
        let mut ablations=Vec::with_capacity(diagnostic_arms.len());
        for (label,inputs) in diagnostic_arms {
            if inputs.iter().any(|poly|r6.eval(poly,&known)!=0) {
                return Err(format!("invalid positive S4 ablation witness: {label}"));
            }
            let f4=capped_f4(&inputs,&r6,Some(known.as_slice()))?;
            ablations.push(json!({"arm":label,"polynomials":inputs.len(),
                "strict_subset_not_an_oracle":true,"f4":f4}));
        }
        // Minimal-arm F4: remove three unused six-level towers and the
        // unused auxiliary variable. This is a bounded diagnostic, never a
        // substitute for the full target-dependent S4 system.
        let r3=minimal_s4_ring(fb.terminal);
        r3.check();
        let mut minimal_arms=Vec::new();
        for (label,part,start,free) in [
            ("minimal_left_s4", &fixed[0], 0usize, 8usize),
            ("minimal_right_s4", &fixed[1], 3usize, 9usize)
        ] {
            let eq=minimal_s4_input(part,start,free,&r3)?;
            let mut short=vec![0u64;3*DEPTH+1];
            for j in 0..3 {
                for k in 0..DEPTH {
                    short[j*DEPTH+k]=known[(start+j)*DEPTH+k];
                }
            }
            short[3*DEPTH]=known[6*DEPTH+(free-8)];
            if r3.eval(&eq,&short)!=0 {
                return Err(format!("minimal S4 lost natural positive: {label}"));
            }
            let f4=capped_f4(&[eq],&r3,Some(&short))?;
            minimal_arms.push(json!({
                "arm":label, "reduced_ideal_only":true,
                "tower_blocks":3,"free_variables":1,"f4":f4
            }));
        }
        println!("{}",json!({
            "standing":"[pcc:unverified]",
            "scope":"eight finite fixed-w S4 F4 development calls; no full 961-way search, no root extraction or ECDLP",
            "reused":"existing Arithmetic, Base, and f4_fp_tower.rs",
            "towers_before":48,"towers_after":36,
            "free_variables_before":3,"free_variables_after":2,
            "fixed_pair_s3":"replaced by exact Fp polynomial root catalogue; selected w values only",
            "source":"PAIR_S3_ELIMINATION_POLYNOMIAL_20261115.md",
            "full_root_catalogue_size":961,
            "original_fixed_s3_bottleneck_dense_entries":26_618,
            "comparability":"only full 961-root/all-branch runs could be a complete oracle",
            "outcomes":outcomes,
            "positive_ideal_ablation_controls":ablations,
            "minimal_three_block_s4_controls":minimal_arms
        }));
        return Ok(());
    }
    if ablate_f4 {
        // Isolate which set of inherited fixed and target Semaev constraints
        // first exceeds the F4 dense-reducer B' limit. These separate ideals
        // are NOT interchangeable with the complete m8 oracle.
        let qx=Q.ok_or("no fixed public Q")?.0;
        let final_raw=s4(&u,&v,&w,&Polynomial::constant(qx));
        let final_poly=ring.from_raw(&final_raw.raw_tower());
        let mut full=polys.clone();
        full.push(final_poly.clone());
        let arms:Vec<(&str,Vec<RPoly>)>=vec![
            ("left_s4_only",vec![polys[0].clone()]),
            ("right_s4_only",vec![polys[1].clone()]),
            ("pair_s3_only",vec![polys[2].clone()]),
            ("target_s4_only",vec![final_poly]),
            ("three_fixed_constraints",polys.clone()),
            ("all_four_constraints",full),
        ];
        let mut outcomes=Vec::new();
        for (name,input) in arms {
            let result=capped_f4(&input,&ring,None)?;
            outcomes.push(json!({"arm":name,"input_polynomials":input.len(),
                "result":result}));
        }
        // The six arms above keep all eight 6-level Dickson blocks even
        // when an ideal references only the last two leaves. Check whether
        // this unused ring dimension, rather than the intrinsic S3 relation,
        // is the actual source of the early matrix oversize stop.
        let mut pair_rules=Vec::with_capacity(2*DEPTH);
        for block in 0..2 {
            for level in 0..DEPTH {
                let terminal=level+1==DEPTH;
                pair_rules.push(SquareRule{
                    next:(!terminal).then_some((block*DEPTH+level+1) as u8),
                    a:0,b:u64::from(!terminal),c:0,
                    d:if terminal {(fb.terminal+2)%P} else {2}
                });
            }
        }
        let small_ring=TowerRing{p:P,rules:pair_rules,n_free:1};
        small_ring.check();
        let mut short_raw=Vec::new();
        for (old,c) in s3(&leaf[6],&leaf[7],&w).raw_tower() {
            if old.iter().enumerate().any(|(i,&v)|v!=0
                && i!=6*DEPTH && i!=7*DEPTH && i!=TOWER+2) {
                return Err("pair-only S3 depends on unexpected variable".into());
            }
            let mut ex=vec![0_u32;2*DEPTH+1];
            ex[0]=old[6*DEPTH];
            ex[DEPTH]=old[7*DEPTH];
            ex[2*DEPTH]=old[TOWER+2];
            short_raw.push((ex,c));
        }
        let small_pair=small_ring.from_raw(&short_raw);
        let small_result=capped_f4(&[small_pair],&small_ring,None)?;
        outcomes.push(json!({"arm":"pair_s3_minimal_two_tower_blocks",
            "input_polynomials":1,
            "tower_blocks":2,"free_variables":1,
            "result":small_result}));
        println!("{}",json!({
            "standing":"[pcc:unverified]",
            "scope":"F4 matrix-stop ablation only; reduced ideals NOT an m8 relation oracle, no ECDLP",
            "public_Q":Q,"degree_cap":12,"dense_cap":15_000,
            "ablation_arms":outcomes
        }));
        return Ok(());
    }
    // Prospective naturally sampled F4 panel: source labels are checked only
    // AFTER the algebraic oracle has received the public R coordinates.
    // A "split miss" says NOTHING about the unrestricted m8 ideal: other
    // partitions can contain valid relations. Never infer UNSAT from a miss.
    if panel_f4 {
        let panel:Panel=serde_json::from_str(
            include_str!("../M8_NATURAL_F4_PANEL_20261115.json"))
            .map_err(|err|err.to_string())?;
        if panel.rows.len()!=16
            || panel.rows.iter().filter(|r|r.split_witness_count>0).count()!=5
            || panel.rows.iter().map(|r|r.split_witness_count).sum::<usize>()!=6 {
            return Err("natural panel size/support totals changed".into());
        }
        if !ar.on_curve(e,Q) {return Err("public Q fails the curve equation".into());}
        let mut probes=Vec::with_capacity(panel.rows.len());
        for (i,row) in panel.rows.iter().enumerate() {
            if row.index!=i || row.a>=e.n || row.b==0 || row.b>=e.n {
                return Err(format!("invalid frozen mask at row {i}"));
            }
            let ag=ar.scalar(e,row.a,Some(e.g));
            let bq=ar.scalar(e,row.b,Q);
            let r=ar.add(e,ag,bq).ok_or("frozen public mask is identity")?;
            if [r.0,r.1]!=row.target {return Err(format!("public R changed at {i}"));}
            let mut input=polys.clone();
            let term=s4(&u,&v,&w,&Polynomial::constant(r.0));
            input.push(ring.from_raw(&term.raw_tower()));
            // The independent generic diagnostic labels do not drive F4.
            // A known witness is used solely for post-F4 soundness checking.
            let known=if let Some([lc,rc])=row.first_witness_codes {
                if row.split_witness_count==0 {
                    return Err(format!("label/certificate disagreement at {i}"));
                }
                let ps=witness_points(lc,rc,&fb,&ar)?;
                let mut sum=None;
                for &pt in &ps {sum=ar.add(e,sum,Some(pt));}
                if sum!=Some(r) {return Err(format!("bad full-point certificate at {i}"));}
                let u_pt=summands(&mut ar,&ps,0,3).ok_or("panel u=O")?;
                let v_pt=summands(&mut ar,&ps,3,6).ok_or("panel v=O")?;
                let w_pt=summands(&mut ar,&ps,6,8).ok_or("panel w=O")?;
                let mut xs=[0u64;NV];
                for j in 0..8 {
                    xs[j*DEPTH]=ps[j].0;
                    for k in 1..DEPTH {
                        let t=xs[j*DEPTH+k-1];
                        xs[j*DEPTH+k]=sub(mul(t,t),2);
                    }
                }
                xs[TOWER]=u_pt.0;
                xs[TOWER+1]=v_pt.0;
                xs[TOWER+2]=w_pt.0;
                if input.iter().any(|poly|ring.eval(poly,&xs)!=0) {
                    return Err(format!("panel polynomial witness invalid at {i}"));
                }
                Some(xs)
            } else {
                if row.split_witness_count!=0 {
                    return Err(format!("missing positive certificate at {i}"));
                }
                None
            };
            let f4=capped_f4(&input,&ring,known.as_ref().map(|xs|xs.as_slice()))?;
            if known.is_some() && f4["verdict"]=="complete_refutation" {
                return Err(format!("false completed refutation on witnessed row {i}"));
            }
            probes.push(json!({
                "index":i,"public_a":row.a,"public_b":row.b,"public_R":row.target,
                "split_support_label":if known.is_some() {
                    "witnessed_positive"
                } else {
                    "outside_registered_split; unrestricted_m8_unknown"
                },
                "generic_split_witness_count_validation_only":row.split_witness_count,
                "f4":f4
            }));
        }
        println!("{}",json!({
            "standing":"[pcc:unverified]",
            "scope":"16 precommitted naturally sampled prime31 public masks; existing Rust S4/S3 F4; NO extraction or DLP recovery",
            "oracle_input":"four quotient-normal-form algebraic polynomials only; no generic group fallback",
            "positive_split_controls":5,
            "outside_split_not_proven_unsat":11,
            "per_query_f4_budget_seconds":5,
            "per_query_nnz_cap":15_000,
            "probes":probes
        }));
        return Ok(());
    }
    let mut checked=0;
    let mut mutated=0;
    let mut f4_probe_records=Vec::new();
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
        if probe_f4 && checked==0 {
            let mut input=polys.clone();
            input.push(final_poly.clone());
            let result=capped_f4(&input,&ring,Some(&xs))?;
            f4_probe_records.push(json!({"arm":"known_natural_positive_fixture",
                "selection":"first previously recorded natural m8 witness; NOT an oracle discovery",
                "result":result}));
            let unknown_final=s4(&u,&v,&w,&Polynomial::constant(Q.unwrap().0));
            input[3]=ring.from_raw(&unknown_final.raw_tower());
            let result=capped_f4(&input,&ring,None)?;
            f4_probe_records.push(json!({"arm":"public_Q_unclassified_target",
                "selection":"Q without any planted base-point witness",
                "result":result}));
        }
        let corrupted=s4(&u,&v,&w,&Polynomial::constant(add(r.0,1)));
        if ring.eval(&ring.from_raw(&corrupted.raw_tower()),&xs)!=0 {mutated+=1;}
        // The polynomial system uses x only. After a candidate x-tuple is
        // extracted, enumerate all 2^8 choices of y-sign and admit only the
        // one(s) whose signed EC sum equals the *full* public R, not just x(R).
        // This is bounded witness verification, never the relation oracle.
        let mut exact_lifts=0;
        let mut x_only_lifts=0;
        let mut distinct=HashSet::new();
        for mask in 0..256_u16 {
            let mut candidate=None;
            for (j,&pt) in ps.iter().enumerate() {
                let q=if ((mask>>j)&1)==1 {
                    ar.neg(e,Some(pt))
                } else {Some(pt)};
                candidate=ar.add(e,candidate,q);
            }
            if candidate==Some(r) {exact_lifts+=1;}
            if candidate.is_some_and(|q|q.0==r.0) {x_only_lifts+=1;}
            distinct.insert(candidate);
        }
        if exact_lifts!=1 || x_only_lifts!=2 || distinct.len()!=256 {
            return Err("signed lift / x-only ambiguity gate changed".into());
        }
        if first_aux.is_none(){first_aux=Some([u_pt.0,v_pt.0,w_pt.0,r.0]);}
        checked+=1;
    }
    if checked!=32 || mutated!=32
        || first_aux!=Some([803_189_040,898_740_481,729_813_404,903_384_438]) {
        return Err("frozen balanced m8 certificate controls changed".into());
    }
    let left=choose(15,4)*16;
    let right=choose(16,4)*16;
    let split=left*right;
    let full=choose(31,8)*256;
    let n=e.n as u128;
    if left!=21_840 || right!=29_120 || split!=635_980_800 || full!=2_019_513_600 {
        return Err("m8 template cardinality drift".into());
    }
    println!("{}",json!({"status":"[pcc:unverified]",
      "scope":"F4 tower-input construction and existing generic-witness semantic bridge; NOT an F4 relation oracle or cheaper ECDLP",
      "p":P,"n":e.n,"depth":DEPTH,"B":fb.points.len(),
      "tower_vars":TOWER,"free_vars":3,"normal_form_terms_fixed":nf_terms,
      "balanced_m8_natural_witnesses_checked":checked,
      "mutated_target_x_rejected":mutated,
      "sign_orientations_per_x_tuple":256,
      "unique_full_target_y_signs_per_tuple":1,
      "abscissa_only_sign_solutions_per_tuple":2,
      "first_auxiliary_x":first_aux,
      "uniform_mask_eight_sum_accounting":{
          "split_templates":split,"all_distinct_eight_templates":full,
          "subgroup_order":n,"split_expected_raw_witnesses_exact":"635980800/2147446831",
          "all_expected_raw_witnesses_exact":"2019513600/2147446831",
          "split_expected_calls_lower_bound_exact":format!("{}/{}",32*n,split),
          "all_expected_calls_lower_bound_exact":format!("{}/{}",32*n,full),
          "qualification":"uniform public masks, one raw witness per template, rank increment <= raw count; no F4 cost claim"},
      "f4_probe_requested":probe_f4,"f4_probe_records":f4_probe_records,
      "rust_compilation_status":"must be confirmed on exact GitHub head"}));
    Ok(())
}