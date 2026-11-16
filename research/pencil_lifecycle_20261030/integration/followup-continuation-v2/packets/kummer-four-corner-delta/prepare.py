from pathlib import Path
import shutil,json,hashlib,difflib
D=Path(__file__).parent;S=Path('/var/tmp/pencil-kummer-four-corner-20261031')
shutil.copytree(S/'native',D/'native',dirs_exist_ok=True);shutil.copytree(S/'src',D/'src',dirs_exist_ok=True)
for n in ['Cargo.toml','Cargo.lock','fixtures.json']:shutil.copyfile(S/n,D/n)
s=(S/'src/pruned.rs').read_text();helper=r'''
    pub fn delta_lift(anchor:Divisor,kq:[u64;4],kplus:[u64;4],t:&Value,j:&Jacobian,p:u64)->(Option<Divisor>,&'static str,usize){
      if kq==[0;4]||kplus==[0;4]{return(None,"delta_zero_vector",0);}
      let Some(q)=general(kq,t,p)else{return(None,"delta_nonordinary_chart",0);};let f=curve(t,p);
      let(a0,a1)=(q[2],s(0,q[1],p));let z=s(m(a1,a1,p),a0,p);let b1sq=a(s(s(q[3],m(z,s(m(f[5],a1,p),m(f[6],z,p),p),p),p),m(a1,s(f[3],m(f[4],a1,p),p),p),p),f[2],p);
      let Some(root)=j.f.sqrt(b1sq)else{return(None,"delta_nonsquare",0);};
      let mut power=[1,0];let mut rem=[0,0];for &coeff in &f{rem[0]=a(rem[0],m(coeff,power[0],p),p);rem[1]=a(rem[1],m(coeff,power[1],p),p);power=[s(0,m(a0,power[1],p),p),s(power[0],m(a1,power[1],p),p)];}
      let mut candidates=Vec::new();for b1 in[root,j.f.neg(root)]{let b0s=if b1!=0{vec![m(a(rem[1],m(a1,b1sq,p),p),inv(m(2,b1,p),p),p)]}else{match j.f.sqrt(rem[0]){Some(b0)=>vec![b0,j.f.neg(b0)],None=>vec![]}};
        for b0 in b0s{let d=Divisor{degree:2,u:[a0,a1,1],v:[b0,b1]};if !candidates.contains(&d){candidates.push(d);}}}
      let tested=candidates.len();assert!(tested<=4);let mut matches=Vec::new();
      for d in candidates{if !j.valid(d){continue;}let Some(kd)=super::walk::project_any(d,j,t,&[],p)else{continue;};if !super::walk::same_k(kd,kq,p){continue;}let sum=j.add(anchor,d);let Some(ks)=super::walk::project_any(sum,j,t,&[],p)else{continue;};if super::walk::same_k(ks,kplus,p){matches.push(d);}}
      match matches.len(){1=>(Some(matches[0]),"delta_unique_signed_lift",tested),0=>(None,"delta_no_signed_lift",tested),_=>(None,"delta_ambiguous_signed_lift",tested)}
    }
'''
s=s.replace('    pub fn boundary_lift(',helper+'    pub fn boundary_lift(',1)
needle='''            (out, status, tested)''';assert s.count(needle)==1;s=s.replace(needle,'''            if out.is_none() && status == "Delta_zero" {
                return delta_lift(anchor,kq,kplus,t,j,p);
            }
            (out, status, tested)''')
# Negative controls execute only for exact retained failure at p739,index20,step1.
old='''let verification=json!({"field":j.f.counts()-before,"walk":walk::read_ops(),"signed":signed::read_ops()});let pass=corners.iter().all(|&b|b)&&signed_ok;'''
new='''let verification=json!({"field":j.f.counts()-before,"walk":walk::read_ops(),"signed":signed::read_ops()});let pass=corners.iter().all(|&b|b)&&signed_ok;
  let mut delta_controls=serde_json::Value::Null;
  if p==739 && z["exception_index"]==20 && step==1 {
    let(zero,zstatus,_)=signed::delta_lift(g,[0;4],next[2],t,&j,p);assert!(zero.is_none());assert_eq!(zstatus,"delta_zero_vector");
    let(zplus,pstatus,_)=signed::delta_lift(g,next[0],[0;4],t,&j,p);assert!(zplus.is_none());assert_eq!(pstatus,"delta_zero_vector");
    assert_ne!(expected,Divisor::O);assert_ne!(expected,j.add(g,g));assert_ne!(expected,j.neg(j.add(g,g)));
    let(nomatch,nstatus,_)=signed::delta_lift(g,next[0],fixed[2],t,&j,p);assert!(nomatch.is_none());assert_eq!(nstatus,"delta_no_signed_lift");
    delta_controls=json!({"zero_input":zstatus,"zero_neighbour":pstatus,"inconsistent_neighbour":nstatus});
  }'''
assert old in s;s=s.replace(old,new)
s=s.replace('"recovery_tested":tested,"signed_ok"','"recovery_tested":tested,"delta_controls":delta_controls,"signed_ok"')
(D/'src/pruned.rs').write_text(s);(D/'delta-lift.patch').write_text(''.join(difflib.unified_diff((S/'src/pruned.rs').read_text().splitlines(True),s.splitlines(True),fromfile='a/private-grid.rs',tofile='b/private-grid-delta.rs')))
(D/'pre-execution-manifest.json').write_text(json.dumps({'files':[{'path':str(p.relative_to(D)),'sha256':hashlib.sha256(p.read_bytes()).hexdigest()} for p in sorted(D.rglob('*')) if p.is_file()]},indent=2))
