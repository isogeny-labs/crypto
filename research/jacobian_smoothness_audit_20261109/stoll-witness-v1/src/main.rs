//! Candidate polynomial certificates, never a substitute for the Lean kernel.
//! Reuses the unchanged pinned MIT arithmetic and existing C++checked raw table.
#[path = "../../native-census-v1/src/vendor/mod.rs"]
pub mod vendor;
#[path = "../../../../src/hash/sha256.rs"]
pub mod sha256;
use num_bigint::BigUint;
use num_traits::ToPrimitive;
use serde::Serialize;
use serde_json::{json,Value};
use std::fs::{self,OpenOptions};
use std::path::Path;
use std::process::Command;
use vendor::curve::{HyperellipticCurveP,MumfordDivisorP};
use vendor::fp_poly::{FpPoly,fp_inv};

type Result<T> = std::result::Result<T,String>;
const BASE:&str="research/jacobian_smoothness_audit_20261109/";
const CRATE_DIR:&str="research/jacobian_smoothness_audit_20261109/stoll-witness-v1/";
// Involution: scalar n of P+T maps to this coefficient representative.
const PERM:[usize;10]=[0,6,2,8,4,5,1,9,3,7];
fn hash(b:&[u8])->String {sha256::sha256(b).iter().map(|x|format!("{x:02x}")).collect()}
fn file(path:&Path,b:&[u8])->Result<()> {
    use std::io::Write;
    OpenOptions::new().write(true).create_new(true).open(path)
        .and_then(|mut f|f.write_all(b)).map_err(|e|e.to_string())
}
fn save(path:&Path,v:&impl Serialize)->Result<()> {
    file(path,&serde_json::to_vec_pretty(v).map_err(|e|e.to_string())?)
}
fn poly(c:&[u64])->FpPoly {FpPoly::from_coeffs(c.iter().copied().map(BigUint::from).collect(),3u8.into())}
fn coeffs(p:&FpPoly)->Vec<u64> {p.coeffs.iter().map(|c|c.to_u64().expect("F3coefficient")).collect()}
fn key(d:&MumfordDivisorP)->(Vec<u64>,Vec<u64>){(coeffs(&d.u),coeffs(&d.v))}
fn exact(a:&FpPoly,b:&FpPoly)->Result<FpPoly>{
    if b.is_zero(){return Err("zero exact divisor".into());}
    let(q,r)=a.divrem(b);if !r.is_zero(){return Err("nonexact polynomial division".into());}Ok(q)
}
fn eq(a:&FpPoly,b:&FpPoly,what:&str)->Result<()> {
    if a.eq_poly(b){Ok(())}else{Err(format!("polynomial certificate mismatch: {what}"))}
}
fn hex(s:&str,n:usize)->bool{s.len()==n&&s.bytes().all(|c|c.is_ascii_hexdigit())}
fn check_sources(commit:&str)->Result<()> {
    let mut paths=vec![format!("{CRATE_DIR}Cargo.toml"),format!("{CRATE_DIR}Cargo.lock"),
        format!("{CRATE_DIR}src/main.rs"),"src/hash/sha256.rs".into(),
        format!("{BASE}lean/MumfordStollGeneral.lean")];
    for n in ["MumfordStoll.lean","MumfordStollCantor.lean","MumfordStollCycleGroup.lean"] {
        paths.push(format!("{BASE}lean/{n}"));
    }
    for n in ["mod.rs","curve.rs","fp_poly.rs","fp2.rs","LICENSE"] {
        paths.push(format!("{BASE}native-census-v1/src/vendor/{n}"));
    }
    for p in paths {
        let old=Command::new("git").args(["show",&format!("{commit}:{p}")])
            .output().map_err(|e|e.to_string())?;
        if !old.status.success()||old.stdout!=fs::read(&p).map_err(|e|e.to_string())? {
            return Err(format!("source not equal committed bytes: {p}"));
        }
    }Ok(())
}
fn representatives(curve:&HyperellipticCurveP)->Result<Vec<MumfordDivisorP>> {
    let data:[(&[u64],&[u64]);10]=[
        (&[1],&[]),(&[0,1],&[1]),(&[0,0,1],&[1]),(&[0,0,1],&[2]),(&[0,1],&[2]),
        (&[1,1],&[]),(&[0,1,1],&[1,1]),(&[0,1,1],&[2,2]),
        (&[2,1,1],&[0,1]),(&[2,1,1],&[0,2])];
    data.iter().map(|(u,v)|MumfordDivisorP::try_from_polynomials(curve,poly(u),poly(v))
        .map_err(|e|format!("fixed representative invalid: {e:?}"))).collect()
}
#[derive(Serialize)]
struct Witness {
    i:usize,j:usize,k:usize,
    d:Vec<u64>,a:Vec<u64>,b:Vec<u64>,c:Vec<u64>,u_mid:Vec<u64>,v_mid:Vec<u64>,q_comp:Vec<u64>,
    common_u_p:Vec<u64>,common_u_q:Vec<u64>,common_v_sum:Vec<u64>,f_quotient:Vec<u64>,
    high:bool,normalization_unit:u64,q_final:Vec<u64>,
}
fn witness(i:usize,j:usize,k:usize,reps:&[MumfordDivisorP],curve:&HyperellipticCurveP)->Result<Witness> {
    let p=&reps[i];let q=&reps[j];let r=&reps[k];
    let(d1,e1,e2)=p.u.ext_gcd(&q.u);let v_sum=p.v.add(&q.v);
    let(d,c1,c2)=d1.ext_gcd(&v_sum);let a=c1.mul(&e1);let b=c1.mul(&e2);let c=c2;
    let up=exact(&p.u.mul(&q.u),&d.mul(&d))?;
    if up.is_zero()||up.lead()!=1u8.into(){return Err("nonmonic composition".into());}
    let numerator=a.mul(&p.u).mul(&q.v).add(&b.mul(&q.u).mul(&p.v))
        .add(&c.mul(&p.v.mul(&q.v).add(&curve.f)));
    let divided=exact(&numerator,&d)?;let(q_comp,vp)=divided.divrem(&up);
    let common_u_p=exact(&p.u,&d)?;let common_u_q=exact(&q.u,&d)?;
    let common_v_sum=exact(&v_sum,&d)?;let f_quotient=exact(&curve.f.sub(&vp.mul(&vp)),&up)?;
    eq(&a.mul(&p.u).add(&b.mul(&q.u)).add(&c.mul(&v_sum)),&d,"Bezout")?;
    eq(&d.mul(&vp.add(&up.mul(&q_comp))),&numerator,"numerator division/remainder")?;
    let high=up.degree().expect("nonzero")>2;
    let (normalization_unit,q_final)=if high {
        if f_quotient.is_zero(){return Err("zero reduction quotient".into());}
        let lam=fp_inv(&f_quotient.lead(),&3u8.into()).ok_or("nonunit reduction lead")?;
        eq(&f_quotient.scalar_mul(&lam),&r.u,"normalized final u")?;
        eq(&vp.neg().rem(&r.u),&r.v,"final v remainder")?;
        let q_final=exact(&vp.neg().sub(&r.v),&r.u)?;
        (lam.to_u64().unwrap(),q_final)
    }else{
        eq(&up,&r.u,"unreduced final u")?;eq(&vp,&r.v,"unreduced final v")?;
        (1,FpPoly::zero(3u8.into()))
    };
    let actual=p.checked_add(q,curve).map_err(|e|format!("upstream checked add: {e:?}"))?;
    if key(&actual)!=key(r){return Err(format!("expected scalar table disagrees at {i},{j}"));}
    Ok(Witness{i,j,k,d:coeffs(&d),a:coeffs(&a),b:coeffs(&b),c:coeffs(&c),
        u_mid:coeffs(&up),v_mid:coeffs(&vp),q_comp:coeffs(&q_comp),
        common_u_p:coeffs(&common_u_p),common_u_q:coeffs(&common_u_q),common_v_sum:coeffs(&common_v_sum),
        f_quotient:coeffs(&f_quotient),high,normalization_unit,q_final:coeffs(&q_final)})
}
fn lean_poly(coeff:&[u64])->String {
    let mut terms=Vec::new();
    for(i,c)in coeff.iter().copied().enumerate(){if c==0{continue;}
        let x=if i==0{"1".to_string()}else if i==1{"X".to_string()}else{format!("X^{i}")};
        terms.push(if c==1{x}else if i==0{format!("Polynomial.C ({c}:K)")}
            else{format!("Polynomial.C ({c}:K)*{x}")});
    }if terms.is_empty(){"0".into()}else{format!("({})",terms.join("+"))}
}
fn choices(rows:&[Witness],which:char)->String {
    let mut s=format!("def bezout{} (i j : Fin 10) : K[X] :=\n  match i,j with\n",which.to_ascii_uppercase());
    for(n,w)in rows.iter().enumerate(){let coeff=match which{'a'=>&w.a,'b'=>&w.b,_=>&w.c};
        let pat=if rows.len()==100&&n==99{"_,_".into()}else{format!("{},{}",w.i,w.j)};
        s.push_str(&format!("  | {pat} => {}\n",lean_poly(coeff)));
    }if rows.len()!=100{s.push_str("  | _,_ => 0\n");}s.push('\n');s
}
fn fragments(rows:&[Witness])->String {
    let mut s=String::from("-- Candidate witness refinements only. No proof or kernel acceptance is asserted.\n");
    for w in rows {
        s.push_str(&format!("-- Inputs {},{}; expected representative {}\n",w.i,w.j,w.k));
        s.push_str(&format!("refine ⟨{},{},{},{},{},{},{},{},\n  ?_,?_,?_,?_,?_,?_,?_,?_,?_,?_,",lean_poly(&w.d),lean_poly(&w.u_mid),lean_poly(&w.v_mid),lean_poly(&w.q_comp),lean_poly(&w.common_u_p),lean_poly(&w.common_u_q),lean_poly(&w.common_v_sum),lean_poly(&w.f_quotient)));
        if w.high{s.push_str(&format!("Or.inr ⟨?_,({}:K),{},?_,?_,?_⟩⟩\n",w.normalization_unit,lean_poly(&w.q_final)));}
        else{s.push_str("Or.inl ⟨?_,?_,?_⟩⟩\n");}
    }s
}
fn run()->Result<()> {
    let args:Vec<_>=std::env::args().collect();if args.len()!=7{return Err("mode out seed source_commit protocol_sha protocol_path required".into());}
    let mode=&args[1];if mode!="cycle"&&mode!="full"{return Err("mode must be cycle or full".into());}
    let seed:u64=args[3].parse().map_err(|_|"invalid seed")?;let source=&args[4];let protocol_sha=&args[5];
    if !hex(source,40)||!hex(protocol_sha,64){return Err("malformed source/protocol identity".into());}
    check_sources(source)?;
    let protocol:Value=serde_json::from_slice(&fs::read(&args[6]).map_err(|e|e.to_string())?).map_err(|e|e.to_string())?;
    if protocol["freeze"]["status"]!="frozen"||protocol["freeze"]["protocol_sha256"]!=protocol_sha.as_str(){return Err("protocol not frozen or wrong hash".into());}
    let arms=protocol["arms"].as_array().ok_or("no arms")?;
    let arm_index=arms.iter().position(|a|a["id"]==mode.as_str()).ok_or("unknown frozen arm")?;
    let arm=&arms[arm_index];
    let binary=fs::read(std::env::current_exe().map_err(|e|e.to_string())?).map_err(|e|e.to_string())?;
    if arm["instrument"]["commit"]!=source.as_str()||arm["instrument"]["binary_sha256"]!=hash(&binary){return Err("instrument pin mismatch".into());}
    if protocol["randomization"]["seed_derivation"]["seeds"].as_array().ok_or("no frozen seeds")?.get(arm_index)!=Some(&json!(seed)){return Err("seed does not match frozen arm".into());}
    let input=protocol["links"]["raw_reference_path"].as_str().ok_or("no raw reference")?;
    let raw=fs::read(input).map_err(|e|e.to_string())?;
    if protocol["links"]["raw_reference_sha256"]!=hash(&raw){return Err("reference data hash mismatch".into());}
    let model=raw.split(|b|*b==b'\n').filter(|line|!line.is_empty()).map(serde_json::from_slice::<Value>)
        .collect::<std::result::Result<Vec<_>,_>>().map_err(|e|e.to_string())?.into_iter()
        .find(|r|r["model"]==1).ok_or("reference model1missing")?;
    if model["p"]!=3||model["f"]!=json!([1,0,0,0,0,1])||model["pairs"].as_array().map(Vec::len)!=Some(10)||model["smooth"]!=8{return Err("reference is not complete Stoll control".into());}
    let out=Path::new(&args[2]);fs::create_dir(out).map_err(|e|e.to_string())?;
    let now=std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map_err(|e|e.to_string())?;
    save(&out.join("start.json"),&json!({"schema":"stoll.witness.start/v1","mode":mode,"seed":seed,"source_commit":source,"protocol_sha256":protocol_sha,"binary_sha256":hash(&binary),"raw_reference_sha256":hash(&raw),"started_at_unix_seconds":now.as_secs(),"hostname":std::env::var("HOSTNAME").unwrap_or_default(),"wall_valid":false}))?;
    let curve=HyperellipticCurveP::try_new(3u8.into(),poly(&[1,0,0,0,0,1]),2).map_err(|e|format!("curve: {e:?}"))?;
    let reps=representatives(&curve)?;let raw_pairs=model["pairs"].as_array().unwrap();
    let mut raw_indices=Vec::new();for p in &reps{let(u,v)=key(p);let idx=raw_pairs.iter().position(|q|q["u"]==json!(u)&&q["v"]==json!(v)).ok_or("representative not in independently checked complete table")?;raw_indices.push(idx);}
    let n=if mode=="cycle"{5}else{10};let mut rows=Vec::new();
    for i in 0..n{for j in 0..n{let k=PERM[(PERM[i]+PERM[j])%10];let raw_k=model["table"][raw_indices[i]*10+raw_indices[j]].as_u64().ok_or("missing raw addition")?as usize;if raw_k!=raw_indices[k]{return Err(format!("independent C++checked table mismatch {i},{j}"));}rows.push(witness(i,j,k,&reps,&curve)?);}}
    save(&out.join("witnesses.json"),&rows)?;
    let mut choices_text=String::from("-- Candidate Bezout choices only. Their equations must be proved by Lean.\n");for ch in ['a','b','c']{choices_text.push_str(&choices(&rows,ch));}
    file(&out.join("bezout-choices.lean.part"),choices_text.as_bytes())?;
    file(&out.join("witness-refinements.txt"),fragments(&rows).as_bytes())?;
    let nontrivial_units=rows.iter().filter(|w|w.high&&w.normalization_unit!=1).count();
    save(&out.join("finish.json"),&json!({"schema":"stoll.witness.finish/v1","status":"succeeded","witness_rows":rows.len(),"independent_table_entries_matched":rows.len(),"high_reductions":rows.iter().filter(|w|w.high).count(),"nontrivial_normalization_units":nontrivial_units,"kernel_proof_status":"not_checked","scope":"Candidate exact polynomial witnesses only. Lean must establish every equation, actual gcd, determinism and final group/proportion statements. Same-host execution, no signed/independent-host standing.","wall_valid":false}))?;
    Ok(())
}
fn main(){if let Err(e)=run(){eprintln!("{e}");std::process::exit(1);}}
