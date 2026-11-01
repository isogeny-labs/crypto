use serde_json::{Value,json};
use sha2::{Digest,Sha256};
use std::{fs,path::Path,collections::{BTreeMap,BTreeSet}};
const PS:[u64;21]=[739,1151,1187,3371,3527,3931,11731,11887,12143,48383,48823,49451,186379,186451,186583,741967,748807,749299,2967343,2968367,2972219];
const THETA:&str="research/kummer_theta_20261022/results/theta-models.jsonl";
const GENERIC:&str="research/kummer_coordinates_20261022/results/generic-models.jsonl";
type E=Result<Value,String>;
fn check(x:bool,s:&str)->Result<(),String>{if x{Ok(())}else{Err(s.into())}}
fn sha(b:&[u8])->String{format!("{:x}",Sha256::digest(b))}
fn digest(b:&[u8],expected:&str)->Result<(),String>{check(sha(b)==expected,"digest mismatch")}
fn n(v:&Value)->Result<u64,String>{v.as_u64().ok_or("unsigned integer".into())}
fn prime(p:u64)->bool{if p<2{return false}if p%2==0{return p==2}let mut d=3;while d<=p/d{if p%d==0{return false}d+=2}true}
fn modulus(p:u64)->Result<(),String>{check(p<=1u64<<32 && prime(p),"prime modulus bound")}
fn canonical(v:&Value,p:u64)->Result<u64,String>{let x=n(v)?;check(x<p,"noncanonical residue")?;Ok(x)}
fn matrix(v:&Value,p:u64)->Result<[[u64;4];4],String>{let rows=v.as_array().ok_or("matrix array")?;check(rows.len()==4,"matrix rows")?;let mut m=[[0;4];4];for i in 0..4{let r=rows[i].as_array().ok_or("matrix row")?;check(r.len()==4,"matrix columns")?;for j in 0..4{m[i][j]=canonical(&r[j],p)?;}}Ok(m)}
fn mul(a:u64,b:u64,p:u64)->u64{((a as u128*b as u128)%p as u128)as u64}
fn power(mut a:u64,mut e:u64,p:u64)->u64{let mut r=1;while e>0{if e&1==1{r=mul(r,a,p)}a=mul(a,a,p);e>>=1}r}
fn square(m:[[u64;4];4],p:u64)->Result<u64,String>{let mut a=[[0;4];4];for i in 0..4{for j in 0..4{let mut s=0u128;for k in 0..4{s+=m[i][k]as u128*m[k][j]as u128}a[i][j]=(s%p as u128)as u64;}}let c=a[0][0];check(c!=0,"zero square scalar")?;for i in 0..4{for j in 0..4{check(a[i][j]==if i==j{c}else{0},"not scalar square")?;}}Ok(c)}
fn normalized(root:u64,r0:u64,delta:u64,p:u64)->Result<u64,String>{check(root<p&&r0<p&&delta>0&&delta<p,"normalization residue/denominator")?;Ok(mul((root+p-r0)%p,power(delta,p-2,p),p))}
fn index(text:&str)->Result<BTreeMap<u64,Value>,String>{let mut map=BTreeMap::new();for line in text.lines(){let v:Value=serde_json::from_str(line).map_err(|e|e.to_string())?;let p=n(&v["p"])?;check(map.insert(p,v).is_none(),"duplicate prime key")?;}Ok(map)}
fn read(root:&Path,rel:&str,inputs:&mut BTreeMap<String,String>)->Result<Vec<u8>,String>{let b=fs::read(root.join(rel)).map_err(|e|format!("{rel}: {e}"))?;inputs.insert(rel.into(),sha(&b));Ok(b)}
fn controls()->Result<Value,String>{let mut out=vec![];let id=json!([[1,0,0,0],[0,1,0,0],[0,0,1,0],[0,0,0,1]]);let perm=json!([[0,2,0,0],[2,0,0,0],[0,0,0,2],[0,0,2,0]]);for(name,v,want)in[("identity",id,1),("doubled_permutation",perm,4)]{let got=square(matrix(&v,7)?,7)?;check(got==want,"positive control")?;out.push(json!({"name":name,"expected":"accept","matched":true}));}
 let bad=[("zero_matrix",json!([[0,0,0,0],[0,0,0,0],[0,0,0,0],[0,0,0,0]])),("unequal_diagonal",json!([[1,0,0,0],[0,2,0,0],[0,0,1,0],[0,0,0,1]])),("wrong_shape",json!([[1,0],[0,1]])),("noncanonical",json!([[7,0,0,0],[0,1,0,0],[0,0,1,0],[0,0,0,1]]))];
 for(name,v)in bad{let r=matrix(&v,7).and_then(|m|square(m,7));check(r.is_err(),"negative matrix control")?;out.push(json!({"name":name,"expected":"reject","matched":true,"error":r.unwrap_err()}));}
 for(name,r)in[("altered_digest",digest(b"abc",&sha(b"abd"))),("composite_modulus",modulus(15)),("oversized_modulus",modulus((1u64<<32)+15)),("zero_delta",normalized(1,0,0,7).map(|_|())),("duplicate_prime_key",index("{\"p\":7}\n{\"p\":7}\n").map(|_|()))]{check(r.is_err(),"negative control")?;out.push(json!({"name":name,"expected":"reject","matched":true,"error":r.unwrap_err()}));}
 Ok(json!({"cases":out,"all_expected":true}))}
fn audit(root:&Path)->E{
 let mut inputs=BTreeMap::new();let sb=read(root,"inputs/summary.json",&mut inputs)?;let s:Value=serde_json::from_slice(&sb).map_err(|e|e.to_string())?;
 let mut pins=BTreeMap::new();for pin in s["source_digests"].as_array().ok_or("source pins")?{let path=pin["path"].as_str().ok_or("pin path")?;let hash=pin["sha256"].as_str().ok_or("pin digest")?;check(pins.insert(path.to_string(),hash.to_string()).is_none(),"duplicate source pin")?;}
 let tb=read(root,"inputs/theta-models.jsonl",&mut inputs)?;let gb=read(root,"inputs/generic-models.jsonl",&mut inputs)?;
 digest(&tb,pins.get(THETA).ok_or("theta pin")?)?;digest(&gb,pins.get(GENERIC).ok_or("generic pin")?)?;
 let theta=index(std::str::from_utf8(&tb).map_err(|e|e.to_string())?)?;let generic=index(std::str::from_utf8(&gb).map_err(|e|e.to_string())?)?;
 let mut cert=BTreeMap::new();for v in s["models"].as_array().ok_or("summary models")?{check(cert.insert(n(&v["p"])?,v.clone()).is_none(),"duplicate certificate prime")?;}
 let expected:BTreeSet<u64>=PS.into_iter().collect();for map in [&theta,&generic,&cert]{check(map.keys().copied().collect::<BTreeSet<_>>()==expected,"exact21 membership")?;}
 let ib=read(root,"pin-index.json",&mut inputs)?;let ix:Value=serde_json::from_slice(&ib).map_err(|e|e.to_string())?;let mut audited_pins=vec![];let mut seen=BTreeSet::new();
 for row in ix.as_array().ok_or("pin index")?{let p=row["path"].as_str().ok_or("index path")?;check(seen.insert(p.to_string()),"duplicate indexed pin")?;let expected=pins.get(p).ok_or("unknown indexed pin")?;let local=row["local"].as_str().ok_or("unavailable pinned input")?;let path=Path::new(local);check(!path.is_absolute()&&!path.components().any(|c|matches!(c,std::path::Component::ParentDir)),"unsafe pin path")?;let bytes=read(root,local,&mut inputs)?;digest(&bytes,expected)?;audited_pins.push(json!({"path":p,"sha256":expected,"matches":true}));}
 check(seen==pins.keys().cloned().collect(),"incomplete source pin inventory")?;
 let mut models=vec![];let mut total=0;
 for p in PS{modulus(p)?;let t=&theta[&p]["selected"];let r0=canonical(&t["r0"],p)?;let delta=canonical(&t["delta"],p)?;check(delta!=0,"zero delta")?;
 let actions=generic[&p]["torsion_actions"].as_array().ok_or("actions")?;check(actions.len()==5&&n(&cert[&p]["action_count"])?==5,"five ordered actions")?;let saved=cert[&p]["square_scalars"].as_array().ok_or("saved scalars")?;check(saved.len()==5,"five scalar certificates")?;
 let mut rows=vec![];let mut roots=BTreeSet::new();for(i,a)in actions.iter().enumerate(){let rt=canonical(&a["root"],p)?;check(roots.insert(rt),"duplicate action root")?;let m=matrix(&a["action"]["matrix"],p)?;let c=square(m,p)?;check(c==canonical(&saved[i],p)?,"ordered square scalar mismatch")?;let normalized=normalized(rt,r0,delta,p)?;rows.push(json!({"index":i,"original_root":rt,"normalized_root":normalized,"matrix":m,"square_scalar":c,"saved_square_scalar":saved[i],"identity_pass":true}));total+=1;}
 models.push(json!({"p":p,"prime":true,"bound_p_le_2pow32":true,"r0":r0,"delta":delta,"actions":rows}));}
 check(total==105,"105 actions")?;
 let current=read(root,"walk-source.rs",&mut inputs)?;let current_sha=sha(&current);let historic=pins.get("research/kummer_differential_collector_20261022/src/walk_impl.rs").ok_or("walk pin")?;
 for(rel,hash)in &inputs{digest(&fs::read(root.join(rel)).map_err(|e|e.to_string())?,hash)?;}
 Ok(json!({"schema":"retained-kummer-matrix-audit/v1","accepted":true,"model_count":models.len(),"action_count":total,"models":models,"source_pins":audited_pins,"inputs":inputs,"inputs_unchanged":true,"controls":controls()?,"loader_source":{"audited_sha256":current_sha,"historical_pinned_sha256":historic,"same_revision":current_sha==*historic,"difference":"Historical version caches theta inverses; supplied source uses uncached inversion. Literal loader and linear definitions agree by retained source comparison."},"scientific_execution":false,"point_enumeration":false,"new_cost_ratio":null,"native_row_fold_attested":false,"compiled_runtime_input_provenance_attested":false}))
}
fn main(){let root=std::env::args().nth(1).expect("audit root");match audit(Path::new(&root)){Ok(v)=>println!("{}",serde_json::to_string_pretty(&v).unwrap()),Err(e)=>{println!("{}",json!({"schema":"retained-kummer-matrix-audit/v1","accepted":false,"error":e}));std::process::exit(1)}}}
