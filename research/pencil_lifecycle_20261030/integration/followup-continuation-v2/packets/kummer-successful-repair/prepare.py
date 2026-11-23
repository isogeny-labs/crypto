from pathlib import Path
import json,hashlib,shutil,difflib
D=Path(__file__).parent;S=Path('/var/tmp/pencil-five-leads-kummer-check-20261030')
shutil.copytree(S/'native',D/'native',dirs_exist_ok=True);(D/'src').mkdir(exist_ok=True)
for n in ['walk_impl_uncached.rs','recover_impl_uncached.rs']:shutil.copyfile(S/'src'/n,D/'src'/n)
for n in ['Cargo.toml','Cargo.lock']:shutil.copyfile(S/n,D/n)
f=json.loads((S/'fixtures.json').read_text());fixtures=[]
for x in f:
 for i,c in enumerate(x['model']['exceptional_cases']):
  if c['kind']=='identity' and c['status']=='zero_output' and c['left_degree']==0 and c['right_degree']==1:
   d=c['coordinate_witness']['jacobian_expected'];assert d['v']==[0,0];fixtures.append({'input':x,'torsion':d,'exception_index':i})
assert fixtures
(D/'fixtures.json').write_text(json.dumps(fixtures))
orig=(S/'src/eager.rs').read_text().split('\npub fn fixture_check(')[0]
pruned=(S/'src/pruned.rs').read_text().split('\npub fn fixture_check(')[0]
def telemetry(s,ispruned):
 s=s.replace('type StepResult = (Option<Pair>, u64, [Option<u64>; 3]);','type StepResult = (Option<Pair>, u64, [Option<u64>; 3], [bool;3], [bool;3]);')
 if ispruned:
  s=s.replace('(kd.zip(km), choice, [r0, r1, None])','(kd.zip(km), choice, [r0, r1, None], [true,true,false], [true,true,false])').replace('(km.zip(kpp), choice, [None, r1, r2])','(km.zip(kpp), choice, [None, r1, r2], [false,true,true], [false,true,true])')
 else:s=s.replace('(pair, choice, [r0, r1, r2])','(pair, choice, [r0, r1, r2], [true,true,true], [choice==0,true,choice!=0])')
 s=s.replace('    roots: [Option<u64>; 3],','    roots: [Option<u64>; 3],\n    evaluated: [bool;3],\n    selected: [bool;3],')
 s=s.replace('let (next, choice, roots) = walk::step','let (next, choice, roots, evaluated, selected) = walk::step')
 # Constructor field insertions only, all indents.
 s=s.replace('                roots,\n','                roots, evaluated, selected,\n').replace('            roots,\n','            roots, evaluated, selected,\n').replace('        roots,\n','        roots, evaluated, selected,\n')
 # Expose base/repaired arithmetic for exact wrapper checks.
 s=s.replace('    pub fn project_divisor(','''    pub fn repair_probe(x:[u64;4],y:[u64;4],diff:[u64;4],t:[u64;4],acts:&[(u64,[[u64;4];4])],p:u64)->(Option<[u64;4]>,Option<[u64;4]>,Option<u64>){
       let plain=xadd(x,y,diff,t,p);let(repaired,root)=xadd_repaired(x,y,diff,t,acts,p);(plain,repaired,root)
    }
    pub fn project_divisor(''')
 return s
old=telemetry(orig,False);new=telemetry(pruned,True)
# Patch excludes diagnostic wrapper helper; shows full candidate change relative frozen original.
(D/'candidate-with-telemetry.patch').write_text(''.join(difflib.unified_diff(orig.splitlines(True),new.splitlines(True),fromfile='a/src/main_labelled.rs',tofile='b/src/main_labelled.rs')))
wrapper=r'''
pub fn fixture_check(z:&Value)->Value {
 let input=&z["input"];let v=&input["target"];let th=&input["theta"]["selected"];let t=&th["constants"];let p=v["p"].as_u64().unwrap();let r=v["r"].as_u64().unwrap();
 let r0=th["r0"].as_u64().unwrap();let delta=th["delta"].as_u64().unwrap();let acts:Vec<_>=input["model"]["torsion_actions"].as_array().unwrap().iter().map(|z|(mul_mod(sub_mod(z["root"].as_u64().unwrap(),r0,p),inv_mod(delta,p),p),parse_matrix(&z["action"]["matrix"]))).collect();
 let f=signed::curve_coeff(t,p);let j=Jacobian{f:Field::new(p,1,0),polynomial:f[..6].to_vec()};let c=v["cases"].as_array().unwrap();let g:Divisor=serde_json::from_value(c[1]["coordinate_witness"]["jacobian_expected"].clone()).unwrap();let q:Divisor=serde_json::from_value(c[2]["coordinate_witness"]["jacobian_expected"].clone()).unwrap();let torsion:Divisor=serde_json::from_value(z["torsion"].clone()).unwrap();
 assert!(j.valid(g)&&j.valid(q)&&j.valid(torsion));assert_eq!(j.add(torsion,torsion),Divisor::O);assert_ne!(j.add(g,g),Divisor::O);assert_ne!(j.add(q,q),Divisor::O);assert_eq!(j.mul(g,r),Divisor::O);assert_eq!(j.mul(q,r),Divisor::O);assert!(r<=(1u64<<62));
 let qt=j.add(q,torsion);let k=walk::project_any(q,&j,t,&acts,p).unwrap();let kp=walk::project_any(qt,&j,t,&acts,p).unwrap();let kt=walk::project_any(torsion,&j,t,&acts,p).unwrap();assert_eq!(j.add(qt,j.neg(q)),torsion);let k0=["a","b","c","d"].map(|key|t[key].as_u64().unwrap());let constants=["A","B","C","D"].map(|key|t[key].as_u64().unwrap());
 let expected_mixed=j.add(q,qt);let ke=walk::project_any(expected_mixed,&j,t,&acts,p).unwrap();let(plain,repaired,repairroot)=walk::repair_probe(k,kp,kt,constants,&acts,p);let repaired_correct=repaired.is_some_and(|a|walk::same_k(a,ke,p));
 walk::reset_ops();let(next,choice,roots,evaluated,selected)=walk::step(k,kp,kt,k0,constants,&acts,p);let counts=walk::read_ops();let expected=if choice==0{j.add(q,q)}else{expected_mixed};let expected_plus=j.add(expected,torsion);let pair_correct=next.is_some_and(|(a,b)|walk::same_k(a,walk::project_any(expected,&j,t,&acts,p).unwrap(),p)&&walk::same_k(b,walk::project_any(expected_plus,&j,t,&acts,p).unwrap(),p));
 let(mut recovered,mut recovery_status,mut signed_ok)=(None,"step_failed",false);
 if let Some((kn,_))=next{let external_plus=walk::project_any(j.add(expected,g),&j,t,&acts,p).unwrap();let kg=walk::project_any(g,&j,t,&acts,p).unwrap();let(d,status,_)=signed::lift(g,kn,external_plus,kg,t,&acts,&j,p);recovered=d;recovery_status=status;signed_ok=d==Some(expected);}
 json!({"p":p,"exception_index":z["exception_index"],"torsion":torsion,"q":q,"anchor":g,"raw_mixed":plain,"repaired_mixed":repaired,"repair_root":repairroot,"repaired_correct":repaired_correct,"next":next,"choice":choice,"roots":roots,"evaluated":evaluated,"selected":selected,"counts":counts,"pair_correct":pair_correct,"expected_signed":expected,"recovered":recovered,"recovery_status":recovery_status,"signed_ok":signed_ok,"sign_rejected":recovered!=Some(j.neg(expected))})
}
'''
(D/'src/eager.rs').write_text(old+wrapper);(D/'src/pruned.rs').write_text(new+wrapper)
(D/'src/check.rs').write_text(r'''
mod eager;mod pruned;use serde_json::{json,Value};
fn main(){let fixtures:Value=serde_json::from_str(include_str!("../fixtures.json")).unwrap();let mut rows=vec![];let mut failed=0;
 for f in fixtures.as_array().unwrap(){let e=eager::fixture_check(f);let p=pruned::fixture_check(f);let mut eq=true;for key in["next","choice","selected","repaired_mixed","expected_signed","recovered","signed_ok","sign_rejected"]{eq&=e[key]==p[key];}let masks=e["evaluated"]==json!([true,true,true])&&p["evaluated"]==p["selected"];let pass=eq&&masks&&p["raw_mixed"].is_null()&&!p["repair_root"].is_null()&&p["repaired_correct"]==true&&p["pair_correct"]==true&&p["signed_ok"]==true&&p["sign_rejected"]==true;if !pass{failed+=1;}rows.push(json!({"eager":e,"pruned":p,"semantic_equal":eq,"mask_valid":masks,"pass":pass}));}
 println!("{}",json!({"schema":"kummer-successful-repair-check/v1","rows":rows,"failed":failed,"scientific_execution":false}));assert_eq!(failed,0,"all fixed cases must meet prospective checks");}
''')
(D/'pre-execution-manifest.json').write_text(json.dumps({'files':[{'path':str(p.relative_to(D)),'sha256':hashlib.sha256(p.read_bytes()).hexdigest()} for p in sorted(D.rglob('*')) if p.is_file()]},indent=2))
print('fixtures',len(fixtures))
