from pathlib import Path
import json,hashlib,shutil,difflib
D=Path(__file__).parent;S=Path('/var/tmp/pencil-kummer-degree1-recovery-20261031')
shutil.copytree(S/'native',D/'native',dirs_exist_ok=True);shutil.copytree(S/'src',D/'src',dirs_exist_ok=True)
for n in ['Cargo.toml','Cargo.lock','fixtures.json']:shutil.copyfile(S/n,D/n)
s=(S/'src/pruned.rs').read_text()
helper=r'''
    pub fn grid_step(grid:[[u64;4];4],fixed:[[u64;4];4],constants:[u64;4],acts:&[(u64,[[u64;4];4])],p:u64)->(Option<[[u64;4];4]>,u64,[Option<u64>;4]){
      let choice=branch(grid[0],p);
      let(a,ra)=xadd_repaired(grid[0],grid[0],fixed[0],constants,acts,p);
      let(b,rb)=xadd_repaired(grid[0],grid[1],fixed[1],constants,acts,p);
      let(c,rc)=xadd_repaired(grid[0],grid[2],fixed[2],constants,acts,p);
      let(d,rd)=xadd_repaired(grid[0],grid[3],fixed[3],constants,acts,p);
      let next=a.zip(b).zip(c).zip(d).map(|(((a,b),c),d)|if choice==0{[a,b,c,d]}else{[b,a,d,c]});
      (next,choice,[ra,rb,rc,rd])
    }
'''
s=s.replace('    pub fn repair_probe(',helper+'    pub fn repair_probe(',1)
wrapper=r'''
pub fn grid_fixture_check(z:&Value)->Value{
 let input=&z["input"];let v=&input["target"];let th=&input["theta"]["selected"];let t=&th["constants"];let p=v["p"].as_u64().unwrap();let r=v["r"].as_u64().unwrap();assert!(r<=(1u64<<62));let n=2*r;
 let r0=th["r0"].as_u64().unwrap();let delta=th["delta"].as_u64().unwrap();let acts:Vec<_>=input["model"]["torsion_actions"].as_array().unwrap().iter().map(|z|(mul_mod(sub_mod(z["root"].as_u64().unwrap(),r0,p),inv_mod(delta,p),p),parse_matrix(&z["action"]["matrix"]))).collect();let f=signed::curve_coeff(t,p);let j=Jacobian{f:Field::new(p,1,0),polynomial:f[..6].to_vec()};let cases=v["cases"].as_array().unwrap();let g:Divisor=serde_json::from_value(cases[1]["coordinate_witness"]["jacobian_expected"].clone()).unwrap();let q:Divisor=serde_json::from_value(cases[2]["coordinate_witness"]["jacobian_expected"].clone()).unwrap();let torsion:Divisor=serde_json::from_value(z["torsion"].clone()).unwrap();assert!(j.valid(g)&&j.valid(q)&&j.valid(torsion));assert_ne!(torsion,Divisor::O);assert_eq!(j.add(torsion,torsion),Divisor::O);assert_ne!(j.add(g,g),Divisor::O);assert_eq!(j.mul(g,r),Divisor::O);assert_eq!(j.mul(q,r),Divisor::O);
 let setup_before=j.f.counts();walk::reset_ops();signed::reset_ops();
 let qt=j.add(q,torsion);let qg=j.add(q,g);let qgt=j.add(qg,torsion);let gt=j.add(g,torsion);
 let mut grid=[q,qt,qg,qgt].map(|x|walk::project_any(x,&j,t,&acts,p).unwrap());let fixed=[Divisor::O,torsion,g,gt].map(|x|walk::project_any(x,&j,t,&acts,p).unwrap());let setup=json!({"field":j.f.counts()-setup_before,"walk":walk::read_ops(),"signed":signed::read_ops(),"initial_grid":grid,"fixed":fixed});
 let constants=["A","B","C","D"].map(|key|t[key].as_u64().unwrap());let(mut a,mut b)=(0u64,1u64);let mut rows=vec![];
 for step in 0..3{
  walk::reset_ops();signed::reset_ops();let before=j.f.counts();let(next,choice,roots)=walk::grid_step(grid,fixed,constants,&acts,p);let step_counts=json!({"field":j.f.counts()-before,"walk":walk::read_ops(),"signed":signed::read_ops()});a=(2*a+choice)%n;b=2*b%n;
  let Some(next)=next else{rows.push(json!({"step":step,"status":"grid_operation_failed","choice":choice,"roots":roots,"counts":step_counts,"pass":false}));break;};
  // Recovery uses only propagated grid, fixed anchor and model. No reference has been computed for this step yet.
  walk::reset_ops();signed::reset_ops();let before=j.f.counts();let(recovered,recovery_status,tested)=signed::lift(g,next[0],next[2],fixed[2],t,&acts,&j,p);let recovery_counts=json!({"field":j.f.counts()-before,"walk":walk::read_ops(),"signed":signed::read_ops()});
  // Independent labelled reference is confined to the verification phase.
  walk::reset_ops();signed::reset_ops();let before=j.f.counts();let expected=j.add(j.mul(torsion,a),j.mul(q,b));let expected_t=j.add(expected,torsion);let expected_g=j.add(expected,g);let expected_gt=j.add(expected_g,torsion);let refs=[expected,expected_t,expected_g,expected_gt].map(|x|walk::project_any(x,&j,t,&acts,p).unwrap());let corners=std::array::from_fn::<_,4,_>(|i|walk::same_k(next[i],refs[i],p));let signed_ok=recovered==Some(expected);let verification=json!({"field":j.f.counts()-before,"walk":walk::read_ops(),"signed":signed::read_ops()});let pass=corners.iter().all(|&b|b)&&signed_ok;
  rows.push(json!({"step":step,"status":"checked","choice":choice,"labels":[a,b],"roots":roots,"evaluated":[true,true,true,true],"grid":next,"corner_matches":corners,"recovered":recovered,"expected":expected,"recovery_status":recovery_status,"recovery_tested":tested,"signed_ok":signed_ok,"counts":{"transition":step_counts,"recovery":recovery_counts,"verification":verification},"pass":pass}));grid=next;
 }
 json!({"p":p,"exception_index":z["exception_index"],"initialization":setup,"rows":rows})
}
'''
s+=wrapper;(D/'src/pruned.rs').write_text(s);(D/'src/check.rs').write_text('mod pruned;fn main(){let f:serde_json::Value=serde_json::from_str(include_str!("../fixtures.json")).unwrap();let cases:Vec<_>=f.as_array().unwrap().iter().map(pruned::grid_fixture_check).collect();let complete=cases.iter().all(|x|x["rows"].as_array().unwrap().len()==3&&x["rows"].as_array().unwrap().iter().all(|r|r["pass"]==true));println!("{}",serde_json::json!({"schema":"kummer-four-corner-check/v1","cases":cases,"complete":complete,"scientific_execution":false}));assert!(complete);}\n')
(D/'grid-helper.patch').write_text(''.join(difflib.unified_diff((S/'src/pruned.rs').read_text().splitlines(True),s.splitlines(True),fromfile='a/private-pruned.rs',tofile='b/private-grid.rs')))
(D/'pre-execution-manifest.json').write_text(json.dumps({'files':[{'path':str(p.relative_to(D)),'sha256':hashlib.sha256(p.read_bytes()).hexdigest()} for p in sorted(D.rglob('*')) if p.is_file()]},indent=2))
