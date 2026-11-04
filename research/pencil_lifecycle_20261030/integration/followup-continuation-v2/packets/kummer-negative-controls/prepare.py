from pathlib import Path
import shutil,json,hashlib
D=Path(__file__).parent;S=Path('/var/tmp/pencil-kummer-degree1-recovery-20261031')
shutil.copytree(S/'native',D/'native',dirs_exist_ok=True);shutil.copytree(S/'src',D/'src',dirs_exist_ok=True)
for n in ['Cargo.toml','Cargo.lock','fixtures.json']:shutil.copyfile(S/n,D/n)
wrapper=r'''
pub fn fixed_negative_controls(z:&Value)->Value{
 let input=&z["input"];let v=&input["target"];let t=&input["theta"]["selected"]["constants"];let p=v["p"].as_u64().unwrap();assert_eq!(p,739);let f=signed::curve_coeff(t,p);let j=Jacobian{f:Field::new(p,1,0),polynomial:f[..6].to_vec()};let c=v["cases"].as_array().unwrap();let g:Divisor=serde_json::from_value(c[1]["coordinate_witness"]["jacobian_expected"].clone()).unwrap();let q:Divisor=serde_json::from_value(c[2]["coordinate_witness"]["jacobian_expected"].clone()).unwrap();let d=j.add(q,q);assert_eq!(d.degree,1);
 let k=walk::project_any(d,&j,t,&[],p).unwrap();let kg=walk::project_any(g,&j,t,&[],p).unwrap();let plus=walk::project_any(j.add(d,g),&j,t,&[],p).unwrap();
 let mut badt=t.clone();badt["a"]=json!(0);let(singular,sing_status,_)=signed::boundary_lift(g,k,plus,&badt,&j,p);assert!(singular.is_none());assert_eq!(sing_status,"singular_coordinate_map");
 let l=t["lambda"].as_u64().unwrap();let mu=t["mu"].as_u64().unwrap();let nu=t["nu"].as_u64().unwrap();assert_eq!([l,mu,nu],[296,16,10]);let value=j.polynomial.iter().rev().fold(0,|z,&c|j.f.add(j.f.mul(z,2),c));assert_eq!(value,654);let mut e=369;let(mut b,mut pow)=(value,1);while e>0{if e&1==1{pow=pow*b%p;}b=b*b%p;e>>=1;}assert_eq!(pow,738);
 let mm=|a:u64,b:u64|a*b%p;let aa=|a:u64,b:u64|(a+b)%p;let rows=[[mm(mu,aa(l,nu)),p-mu,aa(mu,1),p-1],[mm(mm(nu,l),aa(1,mu)),p-mm(nu,l),aa(l,nu),p-1],[mm(nu,aa(l,mu)),p-nu,aa(nu,1),p-1],[mm(mm(mu,l),aa(1,nu)),p-mm(mu,l),aa(l,mu),p-1]];let scales=["a","b","c","d"].map(|z|t[z].as_u64().unwrap());let general=[0,1,2,4];let bad=std::array::from_fn(|i|mm(scales[i],(0..4).fold(0,|z,k|aa(z,mm(rows[i][k],general[k])))));let(ns,ns_status,_)=signed::boundary_lift(g,bad,plus,t,&j,p);assert!(ns.is_none());assert_eq!(ns_status,"degree1_nonsquare");
 assert_ne!(d,Divisor::O);let gg=j.add(g,g);assert_ne!(d,gg);assert_ne!(d,j.neg(gg));let(nomatch,nm_status,_)=signed::boundary_lift(g,k,kg,t,&j,p);assert!(nomatch.is_none());assert_eq!(nm_status,"degree1_no_signed_lift");
 let(positive,pos_status,_)=signed::boundary_lift(g,k,plus,t,&j,p);assert_eq!(positive,Some(d));
 json!({"schema":"kummer-boundary-negative-controls/v1","p":p,"source_exception_index":z["exception_index"],"singular":{"output":singular,"status":sing_status},"nonsquare":{"x":2,"rhs":value,"euler":pow,"output":ns,"status":ns_status},"no_match":{"output":nomatch,"status":nm_status,"supplied_neighbour":"project(G)"},"positive":{"output":positive,"status":pos_status,"supplied_neighbour":"project(expected+G)","oracle_independent_collector":false},"scientific_execution":false})
}
'''
p=D/'src/pruned.rs';p.write_text(p.read_text()+wrapper)
(D/'src/check.rs').write_text('mod pruned;fn main(){let f:serde_json::Value=serde_json::from_str(include_str!("../fixtures.json")).unwrap();println!("{}",pruned::fixed_negative_controls(&f[0]));}\n')
(D/'pre-execution-manifest.json').write_text(json.dumps({'unchanged_candidate_prefix_sha256':hashlib.sha256((S/'src/pruned.rs').read_bytes()).hexdigest(),'files':[{'path':str(p.relative_to(D)),'sha256':hashlib.sha256(p.read_bytes()).hexdigest()} for p in sorted(D.rglob('*')) if p.is_file()]},indent=2))
