//! Run69: authentic RHS target-specific quotient recovery, generic negative control.
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

#[derive(Clone,Copy)]
struct Witness {
    terms: [(usize,i8);3],
    shift: u64,
}
/// Find a first signed three-leaf certificate using the generic reference
/// dictionary. This is deliberately NOT a non-generic algebraic oracle.
fn target_witness(
    ar:&mut Arithmetic, q:Point, sp:&[Point], tab:&HashMap<Point,(u16,u16)>
)->Result<(Witness,u64),String>{
    let mut r=q;
    let mut tried=0;
    for shift in 0..1024 {
        for (t,&pt) in sp.iter().enumerate(){
            tried+=1;
            let negative=ar.neg(PRIME31,pt);
            let complement=ar.add(PRIME31,r,negative);
            if let Some(&(i,j))=tab.get(&complement) {
                let sum=ar.add(PRIME31,sp[i as usize],sp[j as usize]);
                if ar.add(PRIME31,pt,sum)!=r {
                    return Err("generic witness failed independent EC group equality".into())
                }
                return Ok((Witness {terms:[
                    sgn_index(t),sgn_index(i as usize),sgn_index(j as usize)
                ],shift},tried))
            }
        }
        r=ar.add(PRIME31,r,Some(PRIME31.g));
    }
    Err("no target witness within 1024 shifts; censored failure".into())
}
/// Full exact quotient test from REAL labelled DSU relations and the 88 real
/// homogeneous rows. Returns Some(k) only if the target linear form is
/// determined, even if many factor-base logs remain unknown.
fn target_values(
    ds:&mut AffineDsu, free:&[Vec<i64>], witnesses:&[Witness]
)->Result<Vec<Option<u64>>,String>{
    let roots=ds.unknown_roots();
    let u=roots.len();
    let cols=roots.iter().enumerate().map(|(i,&r)|(r,i)).collect::<HashMap<_,_>>();
    let mut mat=Vec::with_capacity(free.len());
    for row in free {
        let mut eq=vec![0u64;u+1];
        for pc in row.chunks_exact(2) {
            let i=pc[0] as usize;
            let c=from_i64(pc[1]);
            let (r,s,b)=ds.find(i);
            eq[u]=sub(eq[u],mul(c,b));
            if let Some(v)=ds.value[r] {
                eq[u]=sub(eq[u],mul(c,signed(v,s)))
            } else {
                eq[cols[&r]]=add(eq[cols[&r]],mul(c,signed(1,s)))
            }
        }
        mat.push(eq);
    }
    let mut p=0;let mut pivots=Vec::new();
    for col in 0..u {
        if p==mat.len(){break}
        let found=(p..mat.len()).find(|&r|mat[r][col]!=0);
        let Some(j)=found else {continue};
        mat.swap(j,p);
        let den=inv(mat[p][col]);
        for c in col..=u {mat[p][c]=mul(mat[p][c],den);}
        for r in 0..mat.len() {
            if r==p || mat[r][col]==0 {continue}
            let factor=mat[r][col];
            for c in col..=u {
                mat[r][c]=sub(mat[r][c],mul(factor,mat[p][c]));
            }
        }
        pivots.push(col);p+=1;
    }
    if mat.iter().skip(p).any(|r|r[..u].iter().all(|&v|v==0)&&r[u]!=0){
        return Err("real RHS/free system contradictory".into())
    }
    let mut out=Vec::with_capacity(witnesses.len());
    for w in witnesses {
        let mut c=vec![0u64;u];
        let mut constant=neg(w.shift%N);
        for &(i,coef) in &w.terms {
            let (r,s,b)=ds.find(i);
            constant=add(constant,signed(b,coef));
            let sg=coef*s;
            if let Some(v)=ds.value[r]{
                constant=add(constant,signed(v,sg));
            }else{
                let col=cols[&r];
                c[col]=add(c[col],signed(1,sg));
            }
        }
        for (r,&col) in pivots.iter().enumerate(){
            let factor=c[col];
            if factor==0 {continue}
            for j in col..u {
                c[j]=sub(c[j],mul(factor,mat[r][j]));
            }
            constant=add(constant,mul(factor,mat[r][u]));
        }
        if c.iter().all(|&v|v==0) {
            out.push(Some(constant));
        } else {
            out.push(None);
        }
    }
    Ok(out)
}
/// Not all factor-base logs need to be determined. Stop as soon as every
/// registered target form belongs to the actual labelled relation row-space.
fn collect_target_specific(
    ar:&mut Arithmetic,pts:&[Point],tab:&HashMap<Point,(u16,u16)>,
    free:&[Vec<i64>], witnesses:&[Witness], qs:&[Point]
)->Result<(Vec<u64>,u64,u64,usize,usize,bool,f64),String>{
    let mut ds=AffineDsu::new(pts.len());
    let t0=Instant::now();
    let mut r=None;let g=Some(PRIME31.g);
    let mut hits=0;let mut rank_hits=0;let mut shifts=0;
    let mut outputs=vec![None;qs.len()];
    for k in 1..=MAX_SHIFTS {
        r=ar.add(PRIME31,r,g);shifts=k;
        if let Some(&(i,j))=tab.get(&r) {
            let (a,sa)=sgn_index(i as usize);let (b,sb)=sgn_index(j as usize);
            if ds.equation(a,sa,b,sb,k%N)?{rank_hits+=1}
            hits+=1;
            if hits%32==0 {
                outputs=target_values(&mut ds,free,witnesses)?;
                let visible=outputs.iter().filter(|v|v.is_some()).count();
                if hits%256==0{
                    println!("RUN69_PROGRESS shifts={shifts} relations={hits} independent_pair_rows={rank_hits} target_visible={visible} of={} remaining_unanchored_roots={}",
                        qs.len(),ds.unknown_roots().len());
                }
                if visible==qs.len(){break;}
            }
        }
    }
    outputs=target_values(&mut ds,free,witnesses)?;
    if outputs.iter().any(Option::is_none){
        return Err(format!("partial-rank target recovery censored: recovered={} of={} shifts={shifts} hits={hits} independent={rank_hits}",
            outputs.iter().filter(|v|v.is_some()).count(),qs.len()))
    }
    let logs=outputs.into_iter().map(|v|v.ok_or("unresolved")).collect::<Result<Vec<_>,_>>()?;
    for (i,(&k,&q)) in logs.iter().zip(qs).enumerate(){
        if ar.scalar(PRIME31,k,g)!=q{
            return Err(format!("target-specific original EC verification fails at {i}"))
        }
    }
    let rank_only=ds.unknown_roots().len();
    let full_base=ds.close_free(free)?;
    let elapsed=t0.elapsed().as_secs_f64()*1000.0;
    println!("RUN69_RELATIONS shifts={shifts} hits={hits} independent_pair_rows={rank_hits} remaining_roots_before_free={rank_only} full_base_rank={full_base} actual_target_logs={} elapsed_ms={elapsed:.3} GENERIC_REFERENCE=TRUE",
        logs.len());
    Ok((logs,shifts,hits,rank_hits,rank_only,full_base,elapsed))
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

fn main()->Result<(),String>{
    let mut ar=Arithmetic::default();
    let t0=Instant::now();
    let pts=base(&mut ar)?;
    let free=free_rows(&mut ar,&pts)?;
    let (sp,tab)=pair_table(&mut ar,&pts)?;
    let setup_ms=t0.elapsed().as_secs_f64()*1e3;
    let mut seed=0x68B677A1D1u64;
    let mut expected=Vec::new();let mut qs=Vec::new();
    for _ in 0..32{
        seed^=seed<<13;seed^=seed>>7;seed^=seed<<17;
        let k=(seed%(N-1))+1;
        expected.push(k);
        qs.push(ar.scalar(PRIME31,k,Some(PRIME31.g)));
    }
    let witness_time=Instant::now();
    let mut witnesses=Vec::new();let mut leaf_probes=0;
    for &q in &qs {
        let (w,leaves)=target_witness(&mut ar,q,&sp,&tab)?;
        witnesses.push(w);leaf_probes+=leaves;
    }
    let witness_ms=witness_time.elapsed().as_secs_f64()*1e3;
    // No unknown scalar from expected is supplied to witness or row solver.
    let (logs,shifts,hits,rank_hits,unknown,full,relation_ms)=
        collect_target_specific(&mut ar,&pts,&tab,&free,&witnesses,&qs)?;
    if logs!=expected {return Err("post-solve harness secret mismatch".into())}
    let mut baseline_ar=Arithmetic::default();
    let (baseline,bs_setup,bs_online)=bsgs(&mut baseline_ar,&qs)?;
    if baseline!=logs {return Err("BSGS independent target logs differ".into())}
    println!("RUN69_COMPLETE verified=32 base_size={} generic_keys={} setup_ms={setup_ms:.3} witnessed_ms={witness_ms:.3} witnessed_leaves={leaf_probes} relation_ms={relation_ms:.3} shifts={shifts} hits={hits} independent={rank_hits} unanchored_before_free={unknown} full_base_rank={full} full_pipeline_ms={:.3} CLASS=GENERIC_NEGATIVE_CONTROL",
        pts.len(),tab.len(),t0.elapsed().as_secs_f64()*1e3);
    println!("RUN69_BSGS setup_ms={bs_setup:.3} online_32_ms={bs_online:.3} original_arithmetic=TRUE");
    println!("RUN69 PASS 32/32 genuine labelled target-specific row-space logs; 88 EC checked free rows; forbidden for non-generic performance claims");
    Ok(())
}
