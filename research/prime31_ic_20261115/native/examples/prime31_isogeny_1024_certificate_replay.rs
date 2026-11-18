//! Source-bound 1024-target prime31 DLP certificate transport E -> E'.
//! Uses original Curve/Arithmetic and frozen full-rank B31 relations. No generic
//! relation search is performed here: this is a certificate replay/transport,
//! not a new relation extractor, a new independent sample, or a cold solver.
//! A separate batched inversion study times the PUBLIC degree-three map.
//! [pcc:unverified]
use prime31_algebraic_ic::{check_curve, Arithmetic, Curve, Point, PRIME31};
use serde_json::{json, Value};
use std::collections::HashSet;
use std::time::Instant;

const P: u64 = 2_147_483_647;
const N: u64 = 2_147_446_831;
const E2: Curve = Curve {
    p: P, a: 323_891_735, b: 1_985_248_359,
    n: N, g: (1_767_020_261, 634_125_456),
};
const KERNEL_X: u64 = 533_591_203;
const TAU: u64 = 1_555_600_864;
const V: u64 = 259_447_350;

#[derive(Clone, Copy, Default)]
struct Fm { multiplications: u64, inversions: u64 }
impl Fm {
    fn mul(&mut self, x: u64, y: u64) -> u64 {
        self.multiplications += 1;
        ((x as u128 * y as u128) % P as u128) as u64
    }
    fn pow(&mut self, mut x: u64, mut k: u64) -> u64 {
        let mut r = 1;
        while k != 0 {
            if k & 1 != 0 { r = self.mul(r, x); }
            x = self.mul(x, x);
            k >>= 1;
        }
        r
    }
    fn inv(&mut self, x: u64) -> Result<u64, String> {
        if x == 0 { return Err("rational kernel pole".into()); }
        self.inversions += 1;
        Ok(self.pow(x, P-2))
    }
}
fn add(x: u64, y: u64) -> u64 { (x + y) % P }
fn sub(x: u64, y: u64) -> u64 { (x + P - y) % P }
fn from_h(q: Point, ih: u64, m: &mut Fm) -> Point {
    q.map(|(x,y)| {
        let ih2 = m.mul(ih, ih);
        let ih3 = m.mul(ih2, ih);
        let nx = add(x, add(m.mul(2*TAU, ih), m.mul(4*V, ih2)));
        let dy = sub(sub(1, m.mul(2*TAU, ih2)), m.mul(8*V, ih3));
        (nx, m.mul(y,dy))
    })
}
fn forward(q: Point, m: &mut Fm) -> Result<Point, String> {
    match q {
        None => Ok(None),
        Some((x,_)) => {
            let ih = m.inv(sub(x,KERNEL_X))?;
            Ok(from_h(q, ih, m))
        }
    }
}
fn forward_batch(qs: &[Point], m: &mut Fm) -> Result<Vec<Point>, String> {
    let mut h = Vec::new();
    let mut prefix = vec![1];
    for &q in qs {
        if let Some((x,_)) = q {
            let dx=sub(x,KERNEL_X);
            if dx==0 { return Err("batch kernel pole".into()); }
            h.push(dx);
            let next=m.mul(*prefix.last().unwrap(),dx);
            prefix.push(next);
        }
    }
    let mut total=m.inv(*prefix.last().unwrap())?;
    let mut inverses=vec![0;h.len()];
    for j in (0..h.len()).rev() {
        inverses[j]=m.mul(total,prefix[j]);
        total=m.mul(total,h[j]);
    }
    if total!=1 { return Err("batch inversion product mismatch".into()); }
    let mut out=Vec::with_capacity(qs.len());
    let mut j=0;
    for &q in qs {
        if q.is_some() {
            out.push(from_h(q,inverses[j],m));
            j+=1;
        } else {
            out.push(None);
        }
    }
    Ok(out)
}
fn scalar_mul(x:u64,y:u64)->u64 { ((x as u128 * y as u128)%N as u128) as u64 }
fn scalar_pow(mut a:u64, mut k:u64)->u64 {
    let mut out=1;
    while k>0 {
        if k&1!=0 {out=scalar_mul(out,a);}
        a=scalar_mul(a,a);
        k>>=1;
    }
    out
}
fn num(v:&Value)->Result<u64,String> {v.as_u64().ok_or_else(||"expected u64".into())}
fn pair(v:&Value)->Result<(u64,u64),String> {
    let a=v.as_array().ok_or("point must be [x,y]")?;
    if a.len()!=2 {return Err("wrong point coordinate count".into());}
    let p=(num(&a[0])?,num(&a[1])?);
    if p.0>=P || p.1>=P {return Err("point outside Fp".into());}
    Ok(p)
}
fn arr<'a>(v:&'a Value,key:&str)->Result<&'a Vec<Value>,String> {
    v.get(key).and_then(Value::as_array).ok_or_else(||format!("missing array {key}"))
}
fn solve_rows(rows:&[Value])->Result<Vec<u64>,String> {
    if rows.len()!=31 {return Err("not 31 basis rows".into());}
    let mut mat=Vec::new();
    for row in rows {
        let v=row.as_array().ok_or("basis row not array")?;
        if v.len()!=32 {return Err("basis row width != 32".into());}
        mat.push(v.iter().map(num).collect::<Result<Vec<_>,_>>()?);
    }
    for col in 0..31 {
        let pivot=(col..31).find(|&r|mat[r][col]!=0)
            .ok_or_else(||format!("rank deficiency at column {col}"))?;
        mat.swap(col,pivot);
        let ip=scalar_pow(mat[col][col],N-2);
        for j in col..32 {mat[col][j]=scalar_mul(mat[col][j],ip);}
        for r in 0..31 {
            if r==col {continue}
            let f=mat[r][col];
            for j in col..32 {
                let v=scalar_mul(f,mat[col][j]);
                mat[r][j]=(mat[r][j]+N-v)%N;
            }
        }
    }
    if (0..31).any(|i|mat[i][i]!=1) {return Err("RREF pivot mismatch".into());}
    Ok(mat.iter().map(|r|r[31]).collect())
}
fn first_x(ar:&mut Arithmetic,c:Curve)->Vec<(u64,u64)>{
    let mut out=Vec::with_capacity(31);
    for x in 0..c.p {
        let rhs=ar.rhs(c,x);
        if let Some(y)=ar.sqrt(rhs,c.p) {
            if y!=0 {out.push((x,y.min(c.p-y)));}
            if out.len()==31 {break;}
        }
    }
    out
}
fn compute()->Result<Value,String> {
    let start=Instant::now();
    let mut ar=Arithmetic::default();
    check_curve(PRIME31,&mut ar)?;
    check_curve(E2,&mut ar)?;
    if PRIME31.p!=P || PRIME31.n!=N {return Err("prime31 fixture drift".into());}
    let evidence:Value=serde_json::from_str(
        include_str!("../DICKSON31_COMPLETE_RANK_1024_RAW_20261117.json"))
        .map_err(|e|e.to_string())?;
    if evidence["schema"]!="prime31.dickson31.complete_rank1024.v1" {
        return Err("frozen complete-solver transcript changed".into());
    }
    let base:Vec<(u64,u64)>=arr(&evidence,"base_points")?
        .iter().map(pair).collect::<Result<_,_>>()?;
    let rows=arr(&evidence,"basis_rows_raw")?;
    let logs=solve_rows(rows)?;
    let recorded=arr(&evidence,"recovered_factor_base_logs")?;
    if base.len()!=31 || recorded.len()!=31 {return Err("wrong B31".into());}
    let mut images=Vec::new();
    let mut fbase=Fm::default();
    let mut xs=HashSet::new();
    for (i,&pt) in base.iter().enumerate() {
        let log=logs[i];
        if log!=num(&recorded[i])? {return Err("frozen factor log mismatch".into());}
        if ar.scalar(PRIME31,log,Some(PRIME31.g))!=Some(pt) {
            return Err(format!("source basis log failed at {i}"));
        }
        let image=forward(Some(pt),&mut fbase)?.ok_or("mapped infinity")?;
        if !xs.insert(image.0) {return Err("transport collided on rational x".into());}
        if !ar.on_curve(E2,Some(image))
            || ar.scalar(E2,log,Some(E2.g))!=Some(image) {
            return Err(format!("transport basis log failed at {i}"));
        }
        images.push(image);
    }
    let first_source=first_x(&mut ar,PRIME31);
    let first_target=first_x(&mut ar,E2);
    let first_e2:HashSet<u64>=first_target.iter().map(|p|p.0).collect();
    let first_trans:HashSet<u64>=first_source.iter()
        .map(|&pt| forward(Some(pt),&mut fbase).unwrap().unwrap().0).collect();
    let overlap=first_e2.intersection(&first_trans).count();
    let transcript=arr(&evidence,"target_replay_transcript")?;
    if transcript.len()!=1024 {return Err("not 1024 frozen Q".into());}
    let publics:Vec<Point>=transcript.iter().map(|r|
        r.get("public_target_q").ok_or("no public Q").and_then(|q| pair(q).map(Some)))
        .collect::<Result<_,_>>()?;
    let mut scalar_meter=Fm::default();
    let t_scalar=Instant::now();
    let single:Vec<Point>=publics.iter().map(|&q|forward(q,&mut scalar_meter))
        .collect::<Result<_,_>>()?;
    let scalar_ns=t_scalar.elapsed().as_nanos();
    let mut batch_meter=Fm::default();
    let t_batch=Instant::now();
    let batch=forward_batch(&publics,&mut batch_meter)?;
    let batch_ns=t_batch.elapsed().as_nanos();
    if batch!=single {return Err("batch/single public transfer disagreement".into());}
    if scalar_meter.inversions!=1024 || batch_meter.inversions!=1 {
        return Err("inversion accounting unexpected".into());
    }
    let mut histogram=[0u64;9];
    let mut zero_fallback=0usize;
    for (i,row) in transcript.iter().enumerate(){
        let q=publics[i];
        let k=num(&row["shift"])?;
        if k>8 {return Err("unregistered shift".into());}
        histogram[k as usize]+=1;
        if row["used_generic_bsgs_fallback"]==true {zero_fallback+=1;}
        let words=row["signed_eight_token_witness"].as_array()
            .ok_or("missing signed B31 witness")?;
        if words.len()!=8 {return Err("wrong eight witness size".into());}
        let mut src_sum=None;
        let mut dst_sum=None;
        let mut sum_log=0u64;
        for w in words {
            let token=num(w)? as usize;
            if token>=62 {return Err("witness token out of range".into());}
            let j=token/2;
            let neg=token%2==1;
            let pa=if neg {ar.neg(PRIME31,Some(base[j]))} else {Some(base[j])};
            let pb=if neg {ar.neg(E2,Some(images[j]))} else {Some(images[j])};
            src_sum=ar.add(PRIME31,src_sum,pa);
            dst_sum=ar.add(E2,dst_sum,pb);
            let lj=if neg {N-logs[j]} else {logs[j]};
            sum_log=(sum_log+lj)%N;
        }
        let orig_shifted=ar.add(PRIME31,q,ar.scalar(PRIME31,k,Some(PRIME31.g)));
        let image_shifted=ar.add(E2,single[i],ar.scalar(E2,k,Some(E2.g)));
        if src_sum!=orig_shifted || dst_sum!=image_shifted {
            return Err(format!("signed eight-leaf relation failed at {i}"));
        }
        let recovered=(sum_log+N-k)%N;
        if recovered!=num(&row["recovered_log"])? ||
            ar.scalar(PRIME31,recovered,Some(PRIME31.g))!=q ||
            ar.scalar(E2,recovered,Some(E2.g))!=single[i] ||
            !ar.on_curve(E2,single[i]) {
            return Err(format!("unknown-log recovery on public target {i} failed"));
        }
    }
    if histogram[0]!=951 || histogram[1]!=70 || histogram[2]!=3
        || histogram[3..].iter().any(|&x|x!=0) || zero_fallback!=0 {
        return Err("historical corpus histogram drift".into());
    }
    Ok(json!({
        "standing":"[pcc:unverified]",
        "kind":"1024-target source-bound certificate replay and batched public transfer",
        "not_a_new_extractor":true,"not_full_cost_solver":true,
        "curve_p":P,"prime_group_order":N,"source_cofactor":1,
        "source_curve_coefficients":[PRIME31.a,PRIME31.b],
        "target_curve_coefficients":[E2.a,E2.b],
        "source_factor_base_size":31,"transported_basis_distinct_x":xs.len(),
        "rank_from_original_31_equations":31,
        "original_and_transported_factor_log_checks":62,
        "frozen_public_unknown_log_recoveries_rederived_from_witnesses":1024,
        "signed_eight_witnesses_replayed_on_both_curves":2048,
        "target_first_hit_shift_histogram":histogram,
        "historical_generic_fallbacks":zero_fallback,
        "first_x31_on_source_transported_overlap_with_first_x31_on_target":overlap,
        "single_public_1024_map": {
            "inversions":scalar_meter.inversions,
            "fp_multiplications":scalar_meter.multiplications,
            "diagnostic_ns":scalar_ns.to_string()
        },
        "batch_public_1024_map": {
            "inversions":batch_meter.inversions,
            "fp_multiplications":batch_meter.multiplications,
            "diagnostic_ns":batch_ns.to_string()
        },
        "certificate_replay_total_diagnostic_ns":start.elapsed().as_nanos().to_string(),
        "independent_signed_pcc":false,
        "full_cost_1024_Eprime_ic_vs_rho_performed":false
    }))
}
fn main() {
    match compute() {
        Ok(v)=>println!("{}",serde_json::to_string_pretty(&v).unwrap()),
        Err(e)=>{eprintln!("prime31 isogeny replay failed: {e}");std::process::exit(1);}
    }
}
#[cfg(test)]
mod tests {
    #[test]
    fn exact_1024_transcript_and_batch_equivariance() {
        let v=super::compute().expect("original-source transcript/3-isogeny exact");
        assert_eq!(v["frozen_public_unknown_log_recoveries_rederived_from_witnesses"],1024);
        assert_eq!(v["batch_public_1024_map"]["inversions"],1);
        assert_eq!(v["full_cost_1024_Eprime_ic_vs_rho_performed"],false);
    }
}
