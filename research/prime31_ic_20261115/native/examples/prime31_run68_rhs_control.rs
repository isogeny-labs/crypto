//! Run68: genuine RHS-labelled full Prime31 log recovery, generic negative control.
//! Source-bound original Rust arithmetic and B677 factor base. The million-entry
//! pair table is GENERIC meet-in-the-middle; never represent it as blind Semaev
//! extraction, novel index calculus, or a competitive non-generic ECDLP solver.
use prime31_algebraic_ic::{check_curve, Arithmetic, Base, Point, PRIME31};
use std::collections::{HashMap, HashSet};
use std::time::Instant;

const EXTRA: &str = include_str!("../../../prime31_f4_admission_20261121/RUN66_Y7_EXTRA168.csv");
const BASIS: &str = include_str!("../../../prime31_f4_admission_20261121/run57_basis.json");
const N: u64 = 2_147_446_831;
const MAX_SHIFTS: u64 = 12_000_000;
fn add(a:u64,b:u64)->u64 {(a+b)%N}
fn neg(a:u64)->u64 {if a==0 {0} else {N-a}}
fn signed(a:u64,s:i8)->u64 {if s==1 {a} else {neg(a)}}
fn sub(a:u64,b:u64)->u64 {add(a,neg(b))}
fn mul(a:u64,b:u64)->u64 {((a as u128*b as u128)%N as u128) as u64}
fn pow(mut a:u64,mut k:u64)->u64 {let mut z=1; while k>0 {if k&1!=0 {z=mul(z,a)} a=mul(a,a); k>>=1;} z}
fn inv(a:u64)->u64 {assert_ne!(a,0);pow(a,N-2)}
fn from_i64(x:i64)->u64 {x.rem_euclid(N as i64) as u64}
fn scalar_signed(ar:&mut Arithmetic, p:Point, k:u64)->Point {ar.scalar(PRIME31,k,p)}
fn base(ar:&mut Arithmetic)->Result<Vec<Point>,String> {
    check_curve(PRIME31,ar)?;
    let canonical=|p:Point| p.map(|(x,y)| (x,y.min((PRIME31.p-y)%PRIME31.p)));
    let mut v=Base::build(PRIME31,10,ar)?.points.into_iter()
        .map(|p|canonical(Some(p))).collect::<Vec<_>>();
    if v.len()!=509 {return Err("original Dickson base changed".into())}
    for line in EXTRA.lines().filter(|l| !l.is_empty()) {
        let (a,b)=line.split_once(',').ok_or("bad extra point")?;
        v.push(canonical(Some((a.parse().map_err(|_|"bad x")?,b.parse().map_err(|_|"bad y")?))));
    }
    v.sort_unstable();v.dedup();
    if v.len()!=677 || v.iter().any(|&p|!ar.on_curve(PRIME31,p)) {
        return Err("B677 original hybrid fixture mismatch".into())
    }
    Ok(v)
}
fn free_rows(ar:&mut Arithmetic,pts:&[Point])->Result<Vec<Vec<i64>>,String> {
    let j:serde_json::Value=serde_json::from_str(BASIS).map_err(|e|e.to_string())?;
    let raw=j["rows"]["hybrid"].as_array().ok_or("hybrid homogeneous rows missing")?;
    if raw.len()!=88 {return Err("hybrid free-rank fixture changed".into())}
    let mut out=Vec::new();
    for row in raw {
        let a=row.as_array().ok_or("bad homogeneous relation")?;
        if a.len()%2!=0 {return Err("bad relation width".into())}
        let mut s=None;
        let mut v=Vec::new();
        for pair in a.chunks_exact(2) {
            let idx=pair[0].as_u64().ok_or("bad free index")? as usize;
            let c=pair[1].as_i64().ok_or("bad free coeff")?;
            if idx>=pts.len() || ![-2,-1,1,2].contains(&c) {return Err("bad free pair".into())}
            let p=if c>0 {pts[idx]} else {ar.neg(PRIME31,pts[idx])};
            for _ in 0..c.unsigned_abs() {s=ar.add(PRIME31,s,p);}
            v.push(idx as i64);v.push(c);
        }
        if s.is_some() {return Err("homogeneous identity fails original EC group law".into())}
        out.push(v);
    }
    Ok(out)
}
#[derive(Clone)]
struct AffineDsu {
    parent:Vec<usize>, sign:Vec<i8>, offset:Vec<u64>, value:Vec<Option<u64>>,
    independent:usize,
}
impl AffineDsu {
    fn new(b:usize)->Self {Self{parent:(0..b).collect(),sign:vec![1;b],offset:vec![0;b],value:vec![None;b],independent:0}}
    // log_i = sign_i*log_root + offset_i in F_n.
    fn find(&mut self,i:usize)->(usize,i8,u64) {
        let p=self.parent[i];if p==i{return (i,1,0)}
        let (r,s,b)=self.find(p);
        let a=self.sign[i]*s;
        let o=add(signed(b,self.sign[i]),self.offset[i]);
        self.parent[i]=r;self.sign[i]=a;self.offset[i]=o;
        (r,a,o)
    }
    fn equation(&mut self,i:usize,si:i8,j:usize,sj:i8,k:u64)->Result<bool,String>{
        let (ri,ai,bi)=self.find(i);let (rj,aj,bj)=self.find(j);
        let a=si*ai;let b=sj*aj;
        let rhs=sub(sub(k,signed(bi,si)),signed(bj,sj));
        if ri==rj {
            if a==-b {if rhs!=0 {return Err("inconsistent within-component RHS".into())}return Ok(false)}
            let v=mul(signed(rhs,a),inv(2)); // (a+b)=2a
            if let Some(old)=self.value[ri] {
                if old!=v {return Err("conflicting anchored scalar".into())}
                return Ok(false);
            }
            self.value[ri]=Some(v);self.independent+=1;return Ok(true)
        }
        // a*log_ri+b*log_rj=rhs; a,b are +/-1
        let c=-(a*b);
        let d=signed(rhs,a);
        let old_i=self.value[ri];
        let old_j=self.value[rj];
        self.parent[ri]=rj;self.sign[ri]=c;self.offset[ri]=d;
        if let Some(vi)=old_i {
            let vj=signed(sub(vi,d),c);
            if let Some(v)=old_j {if v!=vj {return Err("merged anchored roots conflict".into())}}
            else {self.value[rj]=Some(vj)}
        }
        self.independent+=1;
        Ok(true)
    }
    fn unknown_roots(&mut self)->Vec<usize> {
        let mut roots=HashSet::new();
        for i in 0..self.parent.len() {
            let (r,_,_)=self.find(i);
            if self.value[r].is_none(){roots.insert(r);}
        }
        let mut v=roots.into_iter().collect::<Vec<_>>();v.sort_unstable();v
    }
    fn log(&mut self,i:usize)->Option<u64> {
        let (r,s,b)=self.find(i);
        self.value[r].map(|v|add(signed(v,s),b))
    }
    // Specialize the 88 genuine homogeneous group laws through the affine
    // signed-pair DSU. Solve ONLY if the projected system has unique rank.
    fn close_free(&mut self,rows:&[Vec<i64>])->Result<bool,String> {
        let roots=self.unknown_roots();let u=roots.len();
        if u==0 {return Ok(true)}
        if u>rows.len(){return Ok(false)}
        let cols=roots.iter().enumerate().map(|(i,&r)|(r,i)).collect::<HashMap<_,_>>();
        let mut a=Vec::new();
        for row in rows {
            let mut eq=vec![0u64;u+1];
            for pc in row.chunks_exact(2) {
                let i=pc[0] as usize;let c=from_i64(pc[1]);
                let (r,s,b)=self.find(i);
                eq[u]=sub(eq[u],mul(c,b));
                if let Some(v)=self.value[r]{eq[u]=sub(eq[u],mul(c,signed(v,s)))}
                else {let col=cols[&r];eq[col]=add(eq[col],mul(c,signed(1,s)))}
            }
            a.push(eq);
        }
        let mut pivot_row=0;let mut pivots=Vec::new();
        for col in 0..u {
            let found=(pivot_row..a.len()).find(|&r|a[r][col]!=0);
            let Some(j)=found else {continue};
            a.swap(j,pivot_row);
            let den=inv(a[pivot_row][col]);
            for c in col..=u {a[pivot_row][c]=mul(a[pivot_row][c],den)}
            for r in 0..a.len() {
                if r==pivot_row || a[r][col]==0 {continue}
                let coef=a[r][col];
                for c in col..=u {a[r][c]=sub(a[r][c],mul(coef,a[pivot_row][c]))}
            }
            pivots.push(col);pivot_row+=1;
        }
        if a.iter().skip(pivot_row).any(|r|r[..u].iter().all(|&v|v==0)&&r[u]!=0) {
            return Err("free group relations inconsistent with true RHS".into())
        }
        if pivots.len()!=u {return Ok(false)}
        for (r,&col) in pivots.iter().enumerate() {
            self.value[roots[col]]=Some(a[r][u]);
        }
        Ok(true)
    }
}
fn pair_table(ar:&mut Arithmetic,pts:&[Point])->Result<(Vec<Point>,HashMap<Point,(u16,u16)>),String>{
    let mut signed_pts=Vec::with_capacity(2*pts.len());
    for &p in pts {signed_pts.push(p);signed_pts.push(ar.neg(PRIME31,p))}
    let mut tab=HashMap::with_capacity(920_000);
    for i in 0..signed_pts.len() {
        for j in i..signed_pts.len() {
            tab.entry(ar.add(PRIME31,signed_pts[i],signed_pts[j]))
               .or_insert((i as u16,j as u16));
        }
    }
    if tab.len()!=916_469 {return Err(format!("generic B677 pair table changed: {}",tab.len()))}
    Ok((signed_pts,tab))
}
fn sgn_index(i:usize)->(usize,i8) {(i/2,if i%2==0 {1} else {-1})}
fn collect(ar:&mut Arithmetic,pts:&[Point],tab:&HashMap<Point,(u16,u16)>,free:&[Vec<i64>])
    ->Result<(Vec<u64>,u64,u64,usize),String>{
    let mut ds=AffineDsu::new(pts.len());
    let t0=Instant::now();
    let mut r=None;let g=Some(PRIME31.g);
    let mut hits=0;let mut rank_hits=0;let mut shifts=0;
    for k in 1..=MAX_SHIFTS {
        r=ar.add(PRIME31,r,g);shifts=k;
        if let Some(&(i,j))=tab.get(&r) {
            let (a,sa)=sgn_index(i as usize);let (b,sb)=sgn_index(j as usize);
            let new=ds.equation(a,sa,b,sb,k%N)?;
            hits+=1;if new {rank_hits+=1}
            if hits%32==0 && ds.close_free(free)? {break}
        }
    }
    if !ds.close_free(free)? {
        return Err(format!("incomplete real RHS rank: hits={hits}, rank_hits={rank_hits}, unknown_roots={}, shifts={shifts}",ds.unknown_roots().len()))
    }
    let logs=(0..pts.len()).map(|i|ds.log(i).ok_or("unresolved base log"))
        .collect::<Result<Vec<_>,_>>()?;
    for (i,(&p,&k)) in pts.iter().zip(&logs).enumerate() {
        if scalar_signed(ar,Some(PRIME31.g),k)!=p {
            return Err(format!("real RHS base log verification failed at {i}"))
        }
    }
    println!("RUN68_RELATIONS shifts={shifts} successful_pairs={hits} independent_pair_rows={rank_hits} free_rows={} elapsed_ms={:.3} ALL_677_BASE_LOGS_VERIFIED=PASS GENERIC_PAIR_TABLE=TRUE",
        free.len(),t0.elapsed().as_secs_f64()*1000.0);
    Ok((logs,shifts,hits,rank_hits))
}
fn target_log(ar:&mut Arithmetic,q:Point,sp:&[Point],tab:&HashMap<Point,(u16,u16)>,logs:&[u64])
    ->Result<(u64,u64,u64),String> {
    let g=Some(PRIME31.g);
    let mut r=q;
    for shift in 0..1024 {
        for (t,&pt) in sp.iter().enumerate() {
            let complement=ar.add(PRIME31,r,ar.neg(PRIME31,pt));
            if let Some(&(i,j))=tab.get(&complement) {
                let (a,sa)=sgn_index(t);let (b,sb)=sgn_index(i as usize);let (c,sc)=sgn_index(j as usize);
                let x=sub(add(add(signed(logs[a],sa),signed(logs[b],sb)),signed(logs[c],sc)),shift%N);
                if scalar_signed(ar,g,x)!=q {return Err("target triple log independent scalar verification failed".into())}
                return Ok((x,shift,(t+1) as u64))
            }
        }
        r=ar.add(PRIME31,r,g);
    }
    Err("no generic triple witness found within 1024 public shifts".into())
}
fn bsgs(ar:&mut Arithmetic,qs:&[Point])->Result<(Vec<u64>,f64,f64),String> {
    let before=Instant::now();let m=46_342u64;
    let g=Some(PRIME31.g);
    let mut baby=HashMap::with_capacity(m as usize);let mut at=None;
    for i in 0..m {baby.entry(at).or_insert(i);at=ar.add(PRIME31,at,g)}
    let mg=ar.scalar(PRIME31,m,g);
    let delta=ar.neg(PRIME31,mg);
    let setup_ms=before.elapsed().as_secs_f64()*1e3;
    let mut found=Vec::new();let start=Instant::now();
    for &q in qs {
        let mut y=q;let mut out=None;
        for j in 0..=m {
            if let Some(&i)=baby.get(&y) {
                out=Some((i+j*m)%N);break
            }
            y=ar.add(PRIME31,y,delta);
        }
        let x=out.ok_or("BSGS no discrete log")?;
        if ar.scalar(PRIME31,x,g)!=q {return Err("BSGS failed verification".into())}
        found.push(x);
    }
    Ok((found,setup_ms,start.elapsed().as_secs_f64()*1e3))
}
fn main()->Result<(),String> {
    let mut ar=Arithmetic::default();
    let t0=Instant::now();
    let pts=base(&mut ar)?;let free=free_rows(&mut ar,&pts)?;
    let (sp,tab)=pair_table(&mut ar,&pts)?;
    let setup_ms=t0.elapsed().as_secs_f64()*1e3;
    // Only the benchmark harness has the scalars. The generic relation
    // collector and target_log receive points only, never these witnesses.
    let mut seed=0x68B677A1D1u64;
    let mut expected=Vec::new();let mut qs=Vec::new();
    for _ in 0..32 {
        seed^=seed<<13;seed^=seed>>7;seed^=seed<<17;
        let k=(seed%(N-1))+1;
        expected.push(k);qs.push(ar.scalar(PRIME31,k,Some(PRIME31.g)));
    }
    let (logs,shifts,hits,rank_hits)=collect(&mut ar,&pts,&tab,&free)?;
    let start=Instant::now();let mut results=Vec::new();let mut total_shifts=0;let mut trial_leaves=0;
    for &q in &qs {
        let (x,s,leaves)=target_log(&mut ar,q,&sp,&tab,&logs)?;
        results.push(x);total_shifts+=s+1;trial_leaves+=leaves;
    }
    let online_ms=start.elapsed().as_secs_f64()*1e3;
    if results!=expected {return Err("planted scalar crosscheck mismatch".into())}
    let mut baseline_ar=Arithmetic::default();
    let (baseline,bs_setup,bs_online)=bsgs(&mut baseline_ar,&qs)?;
    if baseline!=results {return Err("matched BSGS logs differ".into())}
    println!("RUN68_COMPLETE cohort=32 verified_logs=32 pair_table={} setup_ms={:.3} target_online_ms={:.3} target_shifts={} leaves_at_hit_sum={} collection_shifts={} collection_hits={} independent={} full_pipeline_ms_plus_harness={:.3} CLASS=GENERIC_REFERENCE_NOT_ADMISSIBLE_NON_GENERIC",
        tab.len(),setup_ms,online_ms,total_shifts,trial_leaves,shifts,hits,rank_hits,
        t0.elapsed().as_secs_f64()*1e3);
    println!("RUN68_BSGS same_process_reference_setup_ms={:.3} same_process_reference_online_32_ms={:.3} original_arithmetic=TRUE optimized_negation_rho_not_measured",
        bs_setup,bs_online);
    println!("RUN68 PASS genuine EC RHS labelled relations, 88 public free identities, verified 677 factor-base logs and 32 complete target logs; no blind polynomial oracle and no qualified non-generic advantage");
    Ok(())
}
