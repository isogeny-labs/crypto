//! Exact small-field census. Arithmetic is reused unchanged from the pinned
//! MIT backend; every addition table is exported for a separate C++ check.
pub mod vendor;
#[path = "../../../../src/hash/sha256.rs"]
pub mod sha256;
use num_bigint::BigUint;
use num_traits::{ToPrimitive,Zero};
use serde::{Deserialize,Serialize};
use serde_json::json;
use std::collections::{BTreeMap,BTreeSet};
use std::fs::{self,File,OpenOptions};
use std::io::{BufRead,BufReader,BufWriter,Write};
use std::path::Path;
use std::process::Command;
use vendor::curve::{HyperellipticCurveP,MumfordDivisorP,PrimeHyperellipticError};
use vendor::fp_poly::FpPoly;

type Result<T> = std::result::Result<T,String>;
fn digest(bytes:&[u8])->String { sha256::sha256(bytes).iter().map(|b|format!("{b:02x}")).collect() }
fn new_file(p:&Path)->Result<File> { OpenOptions::new().write(true).create_new(true).open(p).map_err(|e|e.to_string()) }
fn json_file(p:&Path,v:&impl Serialize)->Result<()> {
    let f=new_file(p)?;serde_json::to_writer_pretty(f,v).map_err(|e|e.to_string())
}
fn small(v:&BigUint)->u64 { v.to_u64().expect("small-field coefficient exceeds u64") }
fn coeffs(p:&FpPoly)->Vec<u64> { p.coeffs.iter().map(small).collect() }
fn key(p:&MumfordDivisorP)->(Vec<u64>,Vec<u64>) { (coeffs(&p.u),coeffs(&p.v)) }
fn poly(c:&[u64],p:u64)->FpPoly { FpPoly::from_coeffs(c.iter().copied().map(BigUint::from).collect(),p.into()) }
fn split(u:&FpPoly,p:u64)->bool {
    match u.degree() { Some(0)|Some(1)=>true,Some(2)=>(0..p).any(|x|u.eval(&x.into()).is_zero()),_=>false }
}
fn prime(n:usize)->bool { n>=2 && (2..).take_while(|d|d*d<=n).all(|d|n%d!=0) }

#[derive(Serialize,Deserialize,Clone,Debug,PartialEq,Eq)]
struct Pair { u:Vec<u64>,v:Vec<u64>,smooth:bool }
#[derive(Serialize,Deserialize,Clone,Debug,PartialEq,Eq)]
struct Subgroup { members:Vec<usize>,generators:Vec<usize>,order:usize,smooth:usize,
    proper:bool,prime_order:bool,characteristic_order:bool }
#[derive(Serialize,Deserialize,Clone,Debug,PartialEq,Eq)]
struct CurveRow { schema:String,model:u64,p:u64,f:Vec<u64>,status:String,
    pairs:Vec<Pair>,table:Vec<usize>,subgroups:Vec<Subgroup>,smooth:usize,
    checked_additions:u64,associativity_checks:u64 }

fn enumerate(curve:&HyperellipticCurveP,p:u64)->Result<Vec<MumfordDivisorP>> {
    let mut out=vec![MumfordDivisorP::try_identity(curve).map_err(|e|format!("identity: {e:?}"))?];
    let mut add=|u:Vec<u64>,v:Vec<u64>|->Result<()> {
        match MumfordDivisorP::try_from_polynomials(curve,poly(&u,p),poly(&v,p)) {
            Ok(d)=>out.push(d),
            Err(PrimeHyperellipticError::DivisorEquationMismatch)=>{},
            Err(e)=>return Err(format!("unexpected candidate rejection: {e:?}")),
        } Ok(())
    };
    for a in 0..p {for b in 0..p {add(vec![a,1],vec![b])?;}}
    for a in 0..p {for b in 0..p {for c in 0..p {for d in 0..p {
        add(vec![a,b,1],vec![c,d])?;
    }}}}
    Ok(out)
}

fn group_check(table:&[usize],n:usize)->Result<u64> {
    if table.len()!=n*n || table.iter().any(|x|*x>=n) {return Err("bad table size/range".into());}
    for a in 0..n {
        if table[a]!=a || table[a*n]!=a {return Err("identity failure".into());}
        if !(0..n).any(|b|table[a*n+b]==0) {return Err("inverse failure".into());}
        for b in 0..n {
            if table[a*n+b]!=table[b*n+a] {return Err("commutativity failure".into());}
            for c in 0..n {
                if table[table[a*n+b]*n+c]!=table[a*n+table[b*n+c]] {return Err("associativity failure".into());}
            }
        }
    }
    Ok((n as u64).pow(3))
}

fn subgroups(table:&[usize],pairs:&[Pair],p:u64)->Result<Vec<Subgroup>> {
    let n=pairs.len();let mut distinct:BTreeMap<Vec<usize>,Vec<usize>>=BTreeMap::new();
    for g in 0..n {
        let mut seen=BTreeSet::new();let mut x=0;
        loop {
            if !seen.insert(x) {return Err("cycle repeats before returning to identity".into());}
            if seen.len()>n {return Err("cycle exceeds whole model".into());}
            x=table[x*n+g];if x==0 {break;}
        }
        let members:Vec<_>=seen.into_iter().collect();
        if !members.contains(&g) {return Err("generator missing from its cyclic subgroup".into());}
        distinct.entry(members).or_default().push(g);
    }
    Ok(distinct.into_iter().map(|(members,generators)|{
        let order=members.len();let smooth=members.iter().filter(|i|pairs[**i].smooth).count();
        Subgroup{members,generators,order,smooth,proper:order<n,prime_order:prime(order),
            characteristic_order:order==p as usize}
    }).collect())
}

fn model_row(p:u64,model:u64,f:Vec<u64>)->Result<CurveRow> {
    let mut row=CurveRow{schema:"mumford.census.curve/v1".into(),model,p,f:f.clone(),status:String::new(),
        pairs:vec![],table:vec![],subgroups:vec![],smooth:0,checked_additions:0,associativity_checks:0};
    let curve=match HyperellipticCurveP::try_new(p.into(),poly(&f,p),2) {
        Ok(c)=>c,
        Err(PrimeHyperellipticError::CurvePolynomialNotSquarefree)=>{row.status="nonsquarefree".into();return Ok(row);}
        Err(e)=>return Err(format!("unexpected curve rejection: {e:?}")),
    };
    let divisors=enumerate(&curve,p)?;
    let mut indices=BTreeMap::new();
    for (i,d) in divisors.iter().enumerate() {
        if indices.insert(key(d),i).is_some() {return Err("duplicate reduced representation".into());}
        row.pairs.push(Pair{u:coeffs(&d.u),v:coeffs(&d.v),smooth:split(&d.u,p)});
    }
    if !divisors[0].is_identity() {return Err("first representative is not identity".into());}
    for a in &divisors {for b in &divisors {
        let c=a.checked_add(b,&curve).map_err(|e|format!("checked Cantor addition: {e:?}"))?;
        row.table.push(*indices.get(&key(&c)).ok_or("Cantor output outside complete model")?);
        row.checked_additions+=1;
    }}
    row.associativity_checks=group_check(&row.table,divisors.len())?;
    row.smooth=row.pairs.iter().filter(|d|d.smooth).count();
    row.subgroups=subgroups(&row.table,&row.pairs,p)?;
    row.status="valid".into();
    if row.f==vec![0,p-1,0,0,0,1] {
        let expected=if p==3 {(8,7)}else{(16,16)};
        if (row.pairs.len(),row.smooth)!=expected {return Err("Lean-backed count control failed".into());}
    }
    if p==3&&row.f==vec![1,0,0,0,0,1] {
        let g=row.pairs.iter().position(|d|d.u==vec![0,1]&&d.v==vec![1]).ok_or("Stoll generator missing")?;
        let h=row.subgroups.iter().find(|h|h.generators.contains(&g)).ok_or("Stoll cycle missing")?;
        if row.pairs.len()!=10||row.smooth!=8||h.order!=5||h.smooth!=5||!h.proper {
            return Err("source-known Stoll order-five/smoothness control failed".into());
        }
    }
    Ok(row)
}

fn audit_row(w:&mut impl Write,r:&CurveRow)->Result<()> {
    write!(w,"C {} {}",r.model,r.p).map_err(|e|e.to_string())?;
    for x in &r.f {write!(w," {x}").map_err(|e|e.to_string())?;}
    writeln!(w,"\nV {}",u8::from(r.status=="valid")).map_err(|e|e.to_string())?;
    if r.status=="valid" {
        writeln!(w,"N {} {} {} {}",r.pairs.len(),r.smooth,r.checked_additions,
            r.associativity_checks).map_err(|e|e.to_string())?;
        for (i,d) in r.pairs.iter().enumerate() {
            write!(w,"D {i} {}",d.u.len()).map_err(|e|e.to_string())?;
            for x in &d.u {write!(w," {x}").map_err(|e|e.to_string())?;}
            write!(w," {}",d.v.len()).map_err(|e|e.to_string())?;
            for x in &d.v {write!(w," {x}").map_err(|e|e.to_string())?;}
            writeln!(w," {}",u8::from(d.smooth)).map_err(|e|e.to_string())?;
        }
        write!(w,"A").map_err(|e|e.to_string())?;
        for x in &r.table {write!(w," {x}").map_err(|e|e.to_string())?;}
        writeln!(w,"\nG {}",r.subgroups.len()).map_err(|e|e.to_string())?;
        for h in &r.subgroups {
            write!(w,"H {} {} {} {} {} {}",h.order,h.smooth,h.generators.len(),
                u8::from(h.proper),u8::from(h.prime_order),u8::from(h.characteristic_order)).map_err(|e|e.to_string())?;
            for x in &h.generators {write!(w," {x}").map_err(|e|e.to_string())?;}
            for x in &h.members {write!(w," {x}").map_err(|e|e.to_string())?;}
            writeln!(w).map_err(|e|e.to_string())?;
        }
    }
    writeln!(w,"E").map_err(|e|e.to_string())
}

fn verify_source_commit(commit:&str)->Result<()> {
    if commit.len()!=40 || !commit.bytes().all(|c|c.is_ascii_hexdigit()) {return Err("invalid source commit".into());}
    let prefix="research/jacobian_smoothness_audit_20261109/native-census-v1/";
    for file in ["Cargo.toml","Cargo.lock","src/main.rs","cpp_counts.cpp","src/vendor/mod.rs",
        "src/vendor/curve.rs","src/vendor/fp_poly.rs","src/vendor/fp2.rs","src/vendor/LICENSE"] {
        let path=format!("{prefix}{file}");
        let frozen=Command::new("git").args(["show",&format!("{commit}:{path}")]).output().map_err(|e|e.to_string())?;
        if !frozen.status.success() || frozen.stdout!=fs::read(&path).map_err(|e|e.to_string())? {
            return Err(format!("source differs from committed instrument: {path}"));
        }
    } Ok(())
}

fn census(a:&[String])->Result<()> {
    if a.len()!=7 {return Err("census P all|fixture OUTPUT_DIR SEED REPLICATE SOURCE_COMMIT PROTOCOL_SHA256".into());}
    let p:u64=a[0].parse().map_err(|_|"bad p")?;if ![3,5].contains(&p) {return Err("only p3/p5 are declared".into());}
    let mode=&a[1];if mode!="all"&&mode!="fixture" {return Err("bad enumeration mode".into());}
    let seed:u64=a[3].parse().map_err(|_|"bad seed")?;let replicate:u32=a[4].parse().map_err(|_|"bad replicate")?;
    verify_source_commit(&a[5])?;
    if a[6].len()!=64 || !a[6].bytes().all(|c|c.is_ascii_hexdigit()) {return Err("bad protocol hash".into());}
    let dir=Path::new(&a[2]);fs::create_dir(dir).map_err(|e|e.to_string())?;
    let exe=std::env::current_exe().map_err(|e|e.to_string())?;
    let read=|path:&str|fs::read_to_string(path).unwrap_or_else(|_|"unavailable".into());
    let cpu=read("/proc/cpuinfo").lines().find(|s|s.starts_with("model name")).unwrap_or("unavailable").to_string();
    let observed=Command::new("git").args(["rev-parse","HEAD"]).output().map_err(|e|e.to_string())?;
    json_file(&dir.join("start.json"),&json!({"schema":"mumford.census.start/v1","p":p,"mode":mode,
        "seed":seed,"replicate":replicate,"source_commit_checked":a[5],"protocol_sha256":a[6],
        "observed_worktree_commit":String::from_utf8_lossy(&observed.stdout).trim(),
        "binary_sha256":digest(&fs::read(exe).map_err(|e|e.to_string())?),"cpu":cpu,
        "host":read("/etc/hostname").trim(),"threads":1,"isolation":"L0","wall_valid":false}))?;
    let mut raw=BufWriter::new(new_file(&dir.join("raw.jsonl"))?);
    let mut audit=BufWriter::new(new_file(&dir.join("audit.tsv"))?);
    writeln!(audit,"RUN {p} {mode}").map_err(|e|e.to_string())?;
    let mut attempted=0u64;let mut valid=0u64;let mut additions=0u64;let mut associations=0u64;
    let bound=if mode=="fixture" {1}else{p.pow(5)};
    for code in 0..bound {
        let mut f=vec![0;6];f[5]=1;
        let model=if mode=="fixture" {f[1]=p-1;p*(p-1)}else{
            let mut n=code;for c in f.iter_mut().take(5) {*c=n%p;n/=p;}code
        };
        let row=model_row(p,model,f)?;attempted+=1;
        if row.status=="valid" {valid+=1;}
        additions+=row.checked_additions;associations+=row.associativity_checks;
        serde_json::to_writer(&mut raw,&row).map_err(|e|e.to_string())?;
        writeln!(raw).map_err(|e|e.to_string())?;audit_row(&mut audit,&row)?;
        raw.flush().map_err(|e|e.to_string())?;audit.flush().map_err(|e|e.to_string())?;
    }
    writeln!(audit,"DONE {attempted}").map_err(|e|e.to_string())?;
    if mode=="all"&&valid!=if p==3 {162}else{2500} {
        return Err("squarefree monic-quintic population count differs from declared control".into());
    }
    audit.flush().map_err(|e|e.to_string())?;raw.flush().map_err(|e|e.to_string())?;
    json_file(&dir.join("finish.json"),&json!({"schema":"mumford.census.finish/v1","status":"succeeded",
        "models_attempted":attempted,"valid_models":valid,"nonsquarefree_models":attempted-valid,
        "checked_additions":additions,"associativity_checks":associations,"wall_valid":false}))
}

fn analyze(a:&[String])->Result<()> {
    if a.len()!=2 {return Err("analyze RUN_DIR CPP_RESULT_JSON".into());}
    let dir=Path::new(&a[0]);let cpp:serde_json::Value=serde_json::from_reader(File::open(&a[1]).map_err(|e|e.to_string())?).map_err(|e|e.to_string())?;
    if cpp["status"]!="match" {return Err("C++ verifier did not return match".into());}
    let start:serde_json::Value=serde_json::from_reader(File::open(dir.join("start.json")).map_err(|e|e.to_string())?).map_err(|e|e.to_string())?;
    if cpp["p"]!=start["p"] {return Err("C++ and run-header fields differ".into());}
    let mut encoded_audit=Vec::<u8>::new();
    writeln!(encoded_audit,"RUN {} {}",start["p"].as_u64().ok_or("bad start p")?,
        start["mode"].as_str().ok_or("bad start mode")?).map_err(|e|e.to_string())?;
    let mut rows=0u64;let mut valid=0u64;let mut distinct_subgroups=0u64;
    let mut unequal=0u64;let mut odd_nonchar_unequal=0u64;let mut first_odd=None;
    let mut first_control=None;
    let mut total_additions=0u64;let mut total_associations=0u64;
    for line in BufReader::new(File::open(dir.join("raw.jsonl")).map_err(|e|e.to_string())?).lines() {
        let r:CurveRow=serde_json::from_str(&line.map_err(|e|e.to_string())?).map_err(|e|e.to_string())?;rows+=1;
        if r.schema!="mumford.census.curve/v1"||!["valid","nonsquarefree"].contains(&r.status.as_str()) {
            return Err("unknown raw schema/status".into());
        }
        audit_row(&mut encoded_audit,&r)?;
        total_additions+=r.checked_additions;total_associations+=r.associativity_checks;
        if r.status!="valid" {continue;}valid+=1;
        if r.f[0]==0&&r.f[1]==r.p-1&&r.f[2..5]==[0,0,0] {
            let expected=if r.p==3 {(8,7)}else{(16,16)};
            if (r.pairs.len(),r.smooth)!=expected {return Err("Lean-backed known count control failed".into());}
            first_control=Some(json!({"p":r.p,"model":r.model,"pairs":r.pairs.len(),"smooth":r.smooth}));
        }
        for h in &r.subgroups {
            distinct_subgroups+=1;let differs=h.smooth*r.pairs.len()!=r.smooth*h.order;
            if h.proper&&h.order>1&&differs {unequal+=1;}
            if h.proper&&h.prime_order&&h.order>=3&&!h.characteristic_order&&differs {
                odd_nonchar_unequal+=1;
                if first_odd.is_none() {first_odd=Some(json!({"p":r.p,"model":r.model,"f":r.f,
                    "ambient":r.pairs.len(),"ambient_smooth":r.smooth,"subgroup":h,
                    "generator_pair":r.pairs[h.generators[0]]}));}
            }
        }
    }
    if cpp["models_attempted"].as_u64()!=Some(rows)||cpp["valid_models"].as_u64()!=Some(valid) {
        return Err("C++/Rust flow counts differ".into());
    }
    if cpp["cantor_additions_checked"].as_u64()!=Some(total_additions)||
        cpp["associativity_checks"].as_u64()!=Some(total_associations)||
        cpp["distinct_subgroups_checked"].as_u64()!=Some(distinct_subgroups) {
        return Err("C++/Rust total table/subgroup counters differ".into());
    }
    if first_control.is_none() {return Err("known positive control missing".into());}
    if start["mode"]=="fixture"&&((start["p"]==3&&unequal==0)||(start["p"]==5&&unequal!=0)) {
        return Err("known unequal/equal smoothness detector control failed".into());
    }
    let count_control=first_control.as_ref().ok_or("missing count control")?.clone();
    writeln!(encoded_audit,"DONE {rows}").map_err(|e|e.to_string())?;
    if encoded_audit!=fs::read(dir.join("audit.tsv")).map_err(|e|e.to_string())? {
        return Err("JSON and independently checked TSV encode different records".into());
    }
    json_file(&dir.join("analysis.json"),&json!({"schema":"mumford.census.analysis/v1","status":"match",
        "models_attempted":rows,"valid_models":valid,"distinct_cyclic_subgroups":distinct_subgroups,
        "control_pairs":count_control["pairs"],"control_smooth":count_control["smooth"],
        "proper_nontrivial_unequal":unequal,"proper_odd_prime_noncharacteristic_unequal":odd_nonchar_unequal,
        "first_odd_prime_noncharacteristic_witness":first_odd,"lean_count_control":first_control,
        "cpp_result_sha256":digest(&fs::read(&a[1]).map_err(|e|e.to_string())?),
        "scope":"Exact finite model/curve and subgroup counts; same-host different-language arithmetic check. No geometric Lean bridge, asymptotic or independent-host conclusion."}))
}

fn main() {
    let mut a:Vec<String>=std::env::args().skip(1).collect();
    let result=if a.is_empty(){Err("expected census or analyze".into())}else{
        let mode=a.remove(0);match mode.as_str(){"census"=>census(&a),"analyze"=>analyze(&a),_=>Err("unknown mode".into())}
    };
    if let Err(e)=result {eprintln!("mumford census failure: {e}");std::process::exit(1);}
}
