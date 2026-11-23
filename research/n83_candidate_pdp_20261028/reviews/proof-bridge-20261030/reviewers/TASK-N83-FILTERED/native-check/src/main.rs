use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{collections::BTreeMap, fs, path::{Path, PathBuf}};
type Poly = BTreeMap<u32, u128>;
const ROOT: &str = "/home/ec2-user/n83-candidate-recovery/proof-bridge-review-inputs/filtered/supplement";
const MANIFEST: &str = "/home/ec2-user/crypto-n83-degree-sweep-20261027/research/n83_candidate_pdp_20261028/reviews/proof-bridge-20261030/filtered-supplement.json";
const P: u128 = 4835703278761981908090881;
const KEY: &str = "original-prime-m3-natural";
fn raw(path: &Path) -> Vec<u8> { fs::read(path).unwrap() }
fn read(path: &Path) -> Value { serde_json::from_slice(&raw(path)).unwrap() }
fn sha(bytes: &[u8]) -> String { format!("{:x}", Sha256::digest(bytes)) }
fn number(v: &Value) -> u128 { v.as_str().unwrap().parse().unwrap() }
fn add(a: u128, b: u128) -> u128 { assert!(a<P && b<P); (a+b)%P }
fn neg(a: u128) -> u128 { assert!(a<P); if a==0 {0} else {P-a} }
fn mul(mut a: u128, mut b: u128) -> u128 {
    assert!(a<P && b<P);
    let mut result=0;
    while b!=0 { if b&1!=0 {result=add(result,a);} a=add(a,a); b>>=1; }
    result
}
fn pow(mut a:u128, mut e:u128)->u128 {
    let mut result=1;
    while e!=0 { if e&1!=0 { result=mul(result,a); } a=mul(a,a); e>>=1; }
    result
}
fn insert(poly:&mut Poly, mask:u32, coefficient:u128) {
    let value=add(*poly.get(&mask).unwrap_or(&0),coefficient);
    if value==0 {poly.remove(&mask);} else {poly.insert(mask,value);}
}
fn degree(poly:&Poly)->u32 {poly.keys().map(|m|m.count_ones()).max().unwrap_or(0)}
fn times_x(poly:&Poly)->Poly {
    let mut result=Poly::new();
    for (&mask,&coefficient) in poly {
        for j in 0..27 {
            let bit=1u32<<j;
            // Here coefficient*2^j fits u128, independently of the modular-mul routine.
            let c=if mask&bit==0 {coefficient} else {(coefficient*(1u128<<j))%P};
            insert(&mut result,mask|bit,c);
        }
    }
    result
}
fn encode(poly:&Poly)->Vec<u8> {
    let mut result=b"N83PFF1\n".to_vec();
    result.extend(27u32.to_le_bytes()); result.extend(degree(poly).to_le_bytes());
    result.extend((poly.len() as u64).to_le_bytes()); result.extend(P.to_le_bytes());
    for (&mask,&coefficient) in poly {result.extend((mask as u64).to_le_bytes());result.extend(coefficient.to_le_bytes());}
    result
}
fn parse(path:&Path)->Poly {
    let bytes=raw(path); assert!(bytes.len()>=40);
    assert_eq!(&bytes[..8],b"N83PFF1\n");
    assert_eq!(u32::from_le_bytes(bytes[8..12].try_into().unwrap()),27);
    let stored_degree=u32::from_le_bytes(bytes[12..16].try_into().unwrap());
    let count=u64::from_le_bytes(bytes[16..24].try_into().unwrap()) as usize;
    assert_eq!(u128::from_le_bytes(bytes[24..40].try_into().unwrap()),P);
    assert_eq!(bytes.len(),40+count*24);
    let mut result=Poly::new(); let mut prior=None;
    for row in bytes[40..].chunks_exact(24) {
        let mask=u64::from_le_bytes(row[..8].try_into().unwrap());
        let coefficient=u128::from_le_bytes(row[8..].try_into().unwrap());
        assert!(mask<1<<27 && coefficient>0 && coefficient<P);
        assert!(prior.map_or(true,|p|mask>p)); prior=Some(mask);
        result.insert(mask as u32,coefficient);
    }
    assert_eq!(degree(&result),stored_degree);
    assert_eq!(encode(&result),bytes);
    result
}
fn s3(x:&str,y:&str,z:&str)->String {
    format!("(({x})-({y}))^2*({z})^2-2*((({x})+({y}))*(({x})*({y})+2)+2*3)*({z})+(({x})*({y})-2)^2-4*3*(({x})+({y}))")
}
fn main() {
    let root=Path::new(ROOT);
    let meta=read(Path::new(MANIFEST));
    let raw_manifest_text=fs::read_to_string(root.join("manifest.sha256")).unwrap();
    let raw_manifest:BTreeMap<_,_>=raw_manifest_text.lines().map(|line|{
        let mut fields=line.split_whitespace();
        let hash=fields.next().unwrap(); let name=fields.next().unwrap().trim_start_matches("./");
        (name.to_owned(),hash.to_owned())
    }).collect();
    for row in meta["files"].as_array().unwrap() {
        let bytes=raw(Path::new(row["path"].as_str().unwrap()));
        assert_eq!(bytes.len() as u64,row["bytes"].as_u64().unwrap());
        assert_eq!(sha(&bytes),row["sha256"].as_str().unwrap());
        let rel=row["archive_relative_path"].as_str().unwrap();
        if rel!="manifest.sha256" {assert_eq!(raw_manifest[rel],sha(&bytes));}
    }
    let input_path=root.join("inputs").join(KEY).join("input.json");
    let source_path=root.join("inputs").join(KEY).join("system.sing");
    let cert_path=root.join("terminal").join(KEY).join("certificate.json");
    let top_path=root.join("top").join(format!("{KEY}.json"));
    let result_path=root.join("results").join(format!("{KEY}.json"));
    let input=read(&input_path); let cert=read(&cert_path); let top=read(&top_path); let result=read(&result_path);
    let protocol=read(&root.join("protocol.json"));
    let case=protocol["extensions"]["cases"].as_array().unwrap().iter().find(|c|c["key"]==KEY).unwrap();
    assert_eq!(cert["input"],input);
    for (name,path) in [("input",&input_path),("source",&source_path),("terminal_certificate",&cert_path),("top_certificate",&top_path)] {
        let field=format!("{name}_sha256");
        assert_eq!(sha(&raw(path)),case[&field].as_str().unwrap()); assert_eq!(case[&field],result[&field]);
    }
    for name in ["input_sha256","source_sha256","factor_base_manifest_sha256"] {assert_eq!(top[name],case[name]);}
    assert_eq!(input["factor_base_manifest_sha256"],case["factor_base_manifest_sha256"]);
    let mut source_hashes=0;
    for row in protocol["extensions"]["source_files"].as_array().unwrap() {
        let path=root.join("source").join(row["path"].as_str().unwrap());
        if path.exists() {assert_eq!(sha(&raw(&path)),row["sha256"].as_str().unwrap());source_hashes+=1;}
    }
    assert_eq!(number(&input["field_modulus"]),P);
    let t=number(&input["target_x"]); assert!(t<P);
    assert_eq!(input["family"],"prime");assert_eq!(input["m"],3);assert_eq!(input["ell"],27);
    assert_eq!(input["n_vars"],82);assert_eq!(input["input_equations"],83);
    assert_eq!(input["target_kind"],"natural");assert!(input["witness"].is_null());
    assert_eq!(input["curve_a"],"2");assert_eq!(input["curve_b"],"3");
    let mut remaining=P-1; let mut factors=BTreeMap::new();
    for q in [2u128,5,677,13999,23203] {
        let mut d=2; while d*d<=q {assert_ne!(q%d,0);d+=1;}
        let mut e=0; while remaining%q==0 {remaining/=q;e+=1;} assert!(e>0); factors.insert(q,e);
    }
    assert_eq!(remaining,1);
    // Fixed Lucas witness, independently checked against every prime divisor of P-1.
    let witness=3;
    assert_eq!(pow(witness,P-1),1);
    for &q in factors.keys() {assert_ne!(pow(witness,(P-1)/q),1);}
    let weights:Vec<u128>=(0..27).map(|j|1<<j).collect();
    let stored_weights:Vec<u128>=cert["source"]["weights"].as_array().unwrap().iter().map(number).collect();
    assert_eq!(weights,stored_weights);
    assert_eq!(top["normalization_inverses"].as_array().unwrap().len(),27);
    for (&w,v) in weights.iter().zip(top["normalization_inverses"].as_array().unwrap()) {assert_eq!(mul(w,number(v)),1);}
    let mut source=format!("ring r=(integer,{P}),(v(1..82)),dp;\nideal I;\n");
    for block in 0..3 {
        source+=&format!("poly x{block}={};\n",weights.iter().enumerate().map(|(j,w)|format!("{w}*v({})",27*block+j+1)).collect::<Vec<_>>().join("+"));
    }
    source+=&format!("I[1]={};\nI[2]={};\n",s3("x0","x1","v(82)"),s3("v(82)","x2",&t.to_string()));
    for j in 0..81 {source+=&format!("I[{}]=v({})^2-v({});\n",j+3,j+1,j+1);}
    assert_eq!(source.as_bytes(),raw(&source_path));
    assert_eq!(sha(source.as_bytes()),input["source_sha256"].as_str().unwrap());
    let mut h=Poly::new();
    for bits in 0..(1u32<<13) {
        let mask=(0..13).map(|j|1u32<<(2*j+((bits>>j)&1))).sum();
        h.insert(mask,if bits.count_ones()%2==0 {1} else {P-1});
    }
    let a1=times_x(&h); let a2=times_x(&a1); let factors_a=[h,a1,a2];
    for (i,poly) in factors_a.iter().enumerate() {
        let path=root.join("terminal").join(KEY).join(format!("A{i}.poly"));
        assert_eq!(parse(&path),*poly); assert_eq!(encode(poly),raw(&path));
        let record=&cert["factor_files"][i];
        assert_eq!(record["sha256"].as_str().unwrap(),sha(&encode(poly)));
        assert_eq!(record["sha256"],result["factor_sha256"][i]);
        assert_eq!(record["terms"].as_u64().unwrap(),poly.len() as u64);
        assert_eq!(record["degree"].as_u64().unwrap(),degree(poly) as u64);
    }
    let tt=mul(t,t);let low=neg(add(mul(4,t),12));let minus2t=neg(mul(2,t));
    let b=[[add(4,neg(mul(12,t))),low,tt],[low,neg(add(mul(2,tt),4)),minus2t],[tt,minus2t,1]];
    let wc=&cert["witness_circuit"];
    for i in 0..3 {for u in 0..3 {assert_eq!(number(&wc["B_coefficients_low_to_high"][i][u]),b[i][u]);}}
    assert_eq!(wc["U_variable_zero_based"],81);assert_eq!(wc["U_is_unbounded_indeterminate"],true);
    assert_eq!(wc["relation"],"g=NF(H*S3(U,X,t))");
    assert_eq!(cert["algebra"]["affine_derivation_degree_upper_bound"],17);
    let mut c=Poly::new();
    for (poly,scalar) in factors_a.iter().zip([tt,minus2t,1]) {
        for (&mask,&coefficient) in poly {insert(&mut c,mask,mul(coefficient,scalar));}
    }
    let coefficient_path=root.join("results").join(format!("{KEY}.poly"));
    assert_eq!(parse(&coefficient_path),c);assert_eq!(encode(&c),raw(&coefficient_path));
    assert_eq!(sha(&encode(&c)),result["coefficient_sha256"].as_str().unwrap());
    let pairing=factors_a[0].iter().fold(0,|v,(&mask,&coefficient)|add(v,mul(coefficient,*c.get(&(((1<<27)-1)^mask)).unwrap_or(&0))));
    let delta=add(weights.iter().copied().sum(),minus2t);
    assert_eq!(pairing,mul(pow(P-2,13),delta));assert_eq!(pairing,number(&result["pairing"]));assert_ne!(pairing,0);
    assert_eq!(delta,number(&result["delta"]));
    let mut g:BTreeMap<(u32,usize),u128>=BTreeMap::new();
    for (poly,row) in factors_a.iter().zip(b) {
        for (&mask,&coefficient) in poly {
            for (u,scalar) in row.into_iter().enumerate() {
                let key=(mask,u);let value=add(*g.get(&key).unwrap_or(&0),mul(coefficient,scalar));
                if value==0 {g.remove(&key);} else {g.insert(key,value);}
            }
        }
    }
    let g_degree=g.keys().map(|(m,u)|m.count_ones()+*u as u32).max().unwrap();
    assert_eq!(g_degree,16);assert_eq!(degree(&c),14);
    let mut points:Vec<(u128,u128)>=input["points"].as_array().unwrap().iter().map(|v|(number(&v[0]),number(&v[1]))).collect();
    points.push((t,number(&input["target_y"])));
    for &(x,y) in &points {assert_eq!(mul(y,y),add(add(mul(mul(x,x),x),mul(2,x)),3));}
    println!("{}",serde_json::to_string_pretty(&json!({
        "status":"PASS","language":"Rust","case":KEY,
        "supplement_files_hash_checked":meta["files"].as_array().unwrap().len(),
        "protocol_source_files_present_and_hash_checked":source_hashes,
        "field_modulus":P.to_string(),
        "lucas_n_minus_1_factors":factors.iter().map(|(q,e)|json!({"prime":q.to_string(),"exponent":e})).collect::<Vec<_>>(),
        "lucas_witness":witness,
        "source_sha256":sha(source.as_bytes()),
        "A_terms":factors_a.iter().map(|p|p.len()).collect::<Vec<_>>(),
        "A_degrees":factors_a.iter().map(degree).collect::<Vec<_>>(),
        "coefficient_terms":c.len(),"coefficient_degree":degree(&c),
        "coefficient_sha256":sha(&encode(&c)),
        "g_terms":g.len(),"g_degree":g_degree,
        "delta":delta.to_string(),"pairing":pairing.to_string(),
        "provided_point_and_target_curve_lifts_checked":points.len(),
        "full_other_41_case_replay":false,"producer_arithmetic_imported":false
    })).unwrap());
}

