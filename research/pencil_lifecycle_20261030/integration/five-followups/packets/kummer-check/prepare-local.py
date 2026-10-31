from pathlib import Path
import json,hashlib
D=Path(__file__).parent;H=Path('/home/ec2-user/crypto-pr2304-persist');S=H/'research/kummer_differential_collector_20261022/src'
for name in ['walk_impl_uncached.rs','recover_impl_uncached.rs']:(D/'src'/name).write_bytes((S/name).read_bytes())
(D/'native/Cargo.toml').write_text('[package]\nname="five-leads-study"\nversion="0.1.0"\nedition="2021"\n[dependencies]\nserde={version="1",features=["derive"]}\nserde_json="1"\n')
(D/'native/src/lib.rs').write_text('pub mod three_leads;\npub mod three_leads_followup;\n')
for n in ['three_leads.rs','three_leads_followup.rs']:(D/'native/src'/n).write_bytes((H/'src/cryptanalysis'/n).read_bytes())
(D/'Cargo.toml').write_text('[package]\nname="kummer-software-equivalence"\nversion="0.1.0"\nedition="2024"\n[workspace]\n[dependencies]\nfive-leads-study={path="native"}\nserde={version="1",features=["derive"]}\nserde_json="1"\n[[bin]]\nname="equivalence"\npath="src/check.rs"\n')
theta={v['p']:v for v in map(json.loads,(D/'theta-models.jsonl').read_text().splitlines())};models={v['p']:v for v in map(json.loads,(D/'generic-models.jsonl').read_text().splitlines())};fixtures=[];seen=set()
for bits in [16,20,24,28,32,36,40]:
 for line in (H/f'research/kummer_coordinates_20261022/results/target-gate-{bits}.jsonl').read_text().splitlines():
  v=json.loads(line);p=v['p']
  if p not in seen:fixtures.append({'target':v,'theta':theta[p],'model':models[p]});seen.add(p)
assert len(fixtures)==21
(D/'fixtures.json').write_text(json.dumps(fixtures))
# Apply exact proposal to source snapshot without touching historical producer.
orig=(S/'main_labelled.rs').read_text();patch=Path('/var/tmp/pencil-five-leads-kummer-20261030/branch-directed-proposal.patch').read_text().splitlines();old='\n'.join(x[1:] for x in patch if x.startswith('-') and not x.startswith('---'));new='\n'.join(x[1:] for x in patch if x.startswith('+') and not x.startswith('+++'));assert old in orig
wrapper=r'''
pub fn fixture_check(input:&Value, forced:bool)->Value {
 let v=&input["target"]; let th=&input["theta"]["selected"]; let t=&th["constants"]; let p=v["p"].as_u64().unwrap(); let r=v["r"].as_u64().unwrap();
 let r0=th["r0"].as_u64().unwrap(); let delta=th["delta"].as_u64().unwrap();
 let acts:Vec<_>=input["model"]["torsion_actions"].as_array().unwrap().iter().map(|z|(mul_mod(sub_mod(z["root"].as_u64().unwrap(),r0,p),inv_mod(delta,p),p),parse_matrix(&z["action"]["matrix"]))).collect();
 let f=signed::curve_coeff(t,p);let j=Jacobian{f:Field::new(p,1,0),polynomial:f[..6].to_vec()};
 let c=v["cases"].as_array().unwrap();let g:Divisor=serde_json::from_value(c[1]["coordinate_witness"]["jacobian_expected"].clone()).unwrap();let q:Divisor=serde_json::from_value(c[2]["coordinate_witness"]["jacobian_expected"].clone()).unwrap();
 let kg=parse_vec4(&c[1]["coordinate_witness"]["k_expected"]);let mut k=parse_vec4(&c[2]["coordinate_witness"]["k_expected"]);let mut kp=parse_vec4(&c[3]["coordinate_witness"]["k_expected"]);
 let k0=if forced{[0;4]}else{["a","b","c","d"].map(|z|t[z].as_u64().unwrap())};let constants=["A","B","C","D"].map(|z|t[z].as_u64().unwrap());
 let(mut a,mut b)=(0,1);let mut rows=vec![];
 for step_index in 0..if forced{1}else{3}{
 walk::reset_ops();signed::reset_ops();let x=advance(k,kp,kg,k0,constants,&acts,&j,t,p,r,g,q,a,b,true);let ops=walk::read_ops();
 let labels=((2*a+x.choice)%r,2*b%r);rows.push(json!({"p":p,"forced":forced,"step":step_index,"next":x.next,"choice":x.choice,"labels":labels,"fallback":x.used_fallback,"fallback_checks":x.fallback_validity_checks,"transition_checks":x.transition_checks,"validation_failed":x.validation_failed,"repairs":x.roots,"ops":ops}));
 assert!(!x.validation_failed);let(nk,nkp)=x.next.expect("fixture successor");k=nk;kp=nkp;(a,b)=labels;
 }
 json!(rows)
}
'''
(D/'src/eager.rs').write_text(orig+wrapper);(D/'src/pruned.rs').write_text(orig.replace(old,new)+wrapper)
(D/'src/check.rs').write_text(r'''
mod eager;mod pruned;
use serde_json::{Value,json};
fn main(){
 let f:Value=serde_json::from_str(include_str!("../fixtures.json")).unwrap();let mut out=vec![];
 for fixture in f.as_array().unwrap(){for forced in [false,true]{let e=eager::fixture_check(fixture,forced);let p=pruned::fixture_check(fixture,forced);for(x,y)in e.as_array().unwrap().iter().zip(p.as_array().unwrap()){
 for key in ["next","choice","labels","fallback","fallback_checks","transition_checks","validation_failed"]{assert_eq!(x[key],y[key],"semantic field {key}");}
 let mut deltas=serde_json::Map::new();for key in ["m","a","i"]{let a=x["ops"][key].as_u64().unwrap();let b=y["ops"][key].as_u64().unwrap();assert!(a>=b);deltas.insert(key,json!(a-b));}
 out.push(json!({"eager":x,"pruned":y,"counter_delta":deltas}));
 }}}
 // 2 choices * 3 results in each of 3 calls; None, Some without repair, Some with repair.
 let mut control_cases=0;for c in 0..2{for d0 in 0..3{for m in 0..3{for d1 in 0..3{let cv=|v|if v==0{None}else{Some(v)};let old=if c==0{cv(d0).zip(cv(m))}else{cv(m).zip(cv(d1))};let new=match c{0=>cv(d0).zip(cv(m)),_=>cv(m).zip(cv(d1))};assert_eq!(old,new);control_cases+=1;}}}}
 println!("{}",json!({"schema":"kummer-pruning-software-equivalence/v1","rows":out,"dispatcher_cases":control_cases,"scientific_execution":false}));
}
''')
(D/'pre-execution-manifest.json').write_text(json.dumps({'files':[{'path':str(p.relative_to(D)),'sha256':hashlib.sha256(p.read_bytes()).hexdigest()} for p in sorted(D.rglob('*')) if p.is_file()]},indent=2))
