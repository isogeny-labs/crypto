from pathlib import Path
import shutil,json,hashlib,difflib
D=Path(__file__).parent;S=Path('/var/tmp/pencil-kummer-successful-repair-20261031');shutil.copytree(S/'native',D/'native',dirs_exist_ok=True);shutil.copytree(S/'src',D/'src',dirs_exist_ok=True)
for n in ['Cargo.toml','Cargo.lock','fixtures.json']:shutil.copyfile(S/n,D/n)
# Eager keeps original helper file; candidate includes additive homogeneous helper.
r=(S/'src/recover_impl_uncached.rs').read_text();start=r.index('fn general(');end=r.index('\nfn curve(',start);old=r[start:end];new=old.replace('fn general(', 'fn general_homogeneous(');cut=new.index('    let out = solve4(system, p)?;');new=new[:cut]+'    solve4(system,p)\n}\n'+'''fn general(k:[u64;4],theta:&Value,p:u64)->Option<[u64;4]>{
 let out=general_homogeneous(k,theta,p)?;
 if out[0]==0{return None;}let unit=inv(out[0],p);Some(out.map(|z|m(z,unit,p)))
}
''';(D/'src/recover_boundary_impl.rs').write_text(r[:start]+new+r[end:])
s=(D/'src/pruned.rs').read_text().replace('/src/recover_impl_uncached.rs','/src/recover_boundary_impl.rs')
needle='''            (None, "general_conversion_inapplicable", 0)''';assert s.count(needle)==1;s=s.replace(needle,'''            boundary_lift(anchor,kq,kplus,t,j,p)''')
helper=r'''
    pub fn boundary_lift(anchor:Divisor,kq:[u64;4],kplus:[u64;4],t:&Value,j:&Jacobian,p:u64)->(Option<Divisor>,&'static str,usize){
      if kq==[0;4]||kplus==[0;4]{return(None,"zero_projective_vector",0);}
      let Some(q)=general_homogeneous(kq,t,p)else{return(None,"singular_coordinate_map",0);};
      if q[0]!=0{return(None,"not_boundary_chart",0);}
      if q[1]==0{
        if q[2]!=0||q[3]==0{return(None,"invalid_boundary_chart",0);}
        let Some(ka)=super::walk::project_any(anchor,j,t,&[],p)else{return(None,"anchor_projection_failure",0);};
        return if super::walk::same_k(ka,kplus,p){(Some(Divisor::O),"identity_recovered",1)}else{(None,"identity_neighbour_mismatch",1)};
      }
      if m(q[1],q[3],p)!=m(q[2],q[2],p){return(None,"invalid_degree1_conic",0);}
      let x=m(q[2],inv(q[1],p),p);let value=j.polynomial.iter().rev().fold(0,|z,&c|j.f.add(j.f.mul(z,x),c));
      let Some(y)=j.f.sqrt(value)else{return(None,"degree1_nonsquare",0);};
      let mut matches=Vec::new();let mut tested=0;
      for yy in [y,j.f.neg(y)]{
        let candidate=Divisor{degree:1,u:[j.f.neg(x),1,0],v:[yy,0]};if matches.contains(&candidate){continue;}tested+=1;
        if !j.valid(candidate){continue;}let Some(kc)=super::walk::project_any(candidate,j,t,&[],p)else{continue;};
        if !super::walk::same_k(kc,kq,p){continue;}let sum=j.add(anchor,candidate);let Some(ks)=super::walk::project_any(sum,j,t,&[],p)else{continue;};
        if super::walk::same_k(ks,kplus,p){matches.push(candidate);}
      }
      match matches.len(){1=>(Some(matches[0]),"degree1_unique_signed_lift",tested),0=>(None,"degree1_no_signed_lift",tested),_=>(None,"degree1_ambiguous_signed_lift",tested)}
    }
'''
pos=s.index('    pub fn lift(');# Insert before allow attribute to avoid decorating boundary by existing attr? valid attribute on helper okay.
s=s[:pos]+helper+s[pos:]
# Add controls using exact expected output only in assertion harness, never passed to recovery.
control=r'''
pub fn boundary_controls(z:&Value)->Value{
 let input=&z["input"];let v=&input["target"];let t=&input["theta"]["selected"]["constants"];let p=v["p"].as_u64().unwrap();let f=signed::curve_coeff(t,p);let j=Jacobian{f:Field::new(p,1,0),polynomial:f[..6].to_vec()};let c=v["cases"].as_array().unwrap();let g:Divisor=serde_json::from_value(c[1]["coordinate_witness"]["jacobian_expected"].clone()).unwrap();let q:Divisor=serde_json::from_value(c[2]["coordinate_witness"]["jacobian_expected"].clone()).unwrap();let d=j.add(q,q);
 if d.degree!=1{return json!({"applicable":false});}
 let torsion:Divisor=serde_json::from_value(z["torsion"].clone()).unwrap();let k=walk::project_any(d,&j,t,&[],p).unwrap();let kg=walk::project_any(g,&j,t,&[],p).unwrap();let plus=walk::project_any(j.add(g,d),&j,t,&[],p).unwrap();
 let scale=|v:[u64;4]|v.map(|x|x*7%p);let(scaled,ss,_)=signed::boundary_lift(g,scale(k),scale(plus),t,&j,p);assert_eq!(scaled,Some(d));
 let(zero,zs,_)=signed::boundary_lift(g,[0;4],plus,t,&j,p);assert!(zero.is_none());let(wrong,ws,_)=signed::boundary_lift(g,k,[0;4],t,&j,p);assert!(wrong.is_none());
 let ki=walk::project_any(Divisor::O,&j,t,&[],p).unwrap();let(identity,is,_)=signed::boundary_lift(g,ki,kg,t,&j,p);assert_eq!(identity,Some(Divisor::O));let(wi,wis,_)=signed::boundary_lift(g,ki,ki,t,&j,p);assert!(wi.is_none());
 let tplus=walk::project_any(j.add(torsion,d),&j,t,&[],p).unwrap();let(amb,ams,_)=signed::boundary_lift(torsion,k,tplus,t,&j,p);assert!(amb.is_none());assert_eq!(ams,"degree1_ambiguous_signed_lift");
 let x=j.f.neg(d.u[0]);let general=[0,1,x,j.f.add(j.f.sq(x),1)];let l=t["lambda"].as_u64().unwrap();let mu=t["mu"].as_u64().unwrap();let nu=t["nu"].as_u64().unwrap();let mm=|a:u64,b:u64|a*b%p;let aa=|a:u64,b:u64|(a+b)%p;
 let rows=[[mm(mu,aa(l,nu)),p-mu,aa(mu,1),p-1],[mm(mm(nu,l),aa(1,mu)),p-mm(nu,l),aa(l,nu),p-1],[mm(nu,aa(l,mu)),p-nu,aa(nu,1),p-1],[mm(mm(mu,l),aa(1,nu)),p-mm(mu,l),aa(l,mu),p-1]];let scales=["a","b","c","d"].map(|z|t[z].as_u64().unwrap());let bad=std::array::from_fn(|i|mm(scales[i],(0..4).fold(0,|z,k|aa(z,mm(rows[i][k],general[k])))));let(badout,bs,_)=signed::boundary_lift(g,bad,plus,t,&j,p);assert!(badout.is_none());assert_eq!(bs,"invalid_degree1_conic");
 json!({"applicable":true,"p":p,"exception_index":z["exception_index"],"scaled":ss,"zero":zs,"wrong_plus":ws,"identity":is,"wrong_identity":wis,"torsion_anchor":ams,"invalid_conic":bs})
}
'''
s+=control;(D/'src/pruned.rs').write_text(s)
check=(D/'src/check.rs').read_text();check=check.replace(',"recovered","signed_ok","sign_rejected"','')
check=check.replace('"pass":pass}', '"pass":pass,"boundary_controls":pruned::boundary_controls(f)}')
(D/'src/check.rs').write_text(check)
(D/'degree1-recovery.patch').write_text(''.join(difflib.unified_diff((S/'src/pruned.rs').read_text().splitlines(True),s.splitlines(True),fromfile='a/private-pruned.rs',tofile='b/private-pruned.rs')))
(D/'pre-execution-manifest.json').write_text(json.dumps({'files':[{'path':str(p.relative_to(D)),'sha256':hashlib.sha256(p.read_bytes()).hexdigest()} for p in sorted(D.rglob('*')) if p.is_file()]},indent=2))
