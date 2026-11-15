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
        && args[0]!="--ablate-f4") {
        return Err("usage: prime31_m8_balanced_f4_input [--probe-f4|--panel-f4|--ablate-f4]".into());
    }
    let probe_f4=args.first().is_some_and(|v|v=="--probe-f4");
    let panel_f4=args.first().is_some_and(|v|v=="--panel-f4");
    let ablate_f4=args.first().is_some_and(|v|v=="--ablate-f4");
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