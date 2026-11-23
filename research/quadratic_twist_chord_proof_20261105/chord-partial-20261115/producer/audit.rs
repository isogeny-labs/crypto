// Independent finite-field, point-pair and polynomial replay. No candidate root table.
use serde_json::{json,Value};use sha2::{Digest,Sha256};use std::{collections::{BTreeSet,BTreeMap},fs,io::Write};
type Poly=Vec<i64>;type Row=[i64;3];type Pt=Option<(i64,i64)>;type Base=[(i64,i64);3];
fn md(x:i64,p:i64)->i64{x.rem_euclid(p)}fn f(x:i64)->i64{md(x,37)}
fn inv(x:i64,p:i64)->i64{let(mut a,mut b,mut u,mut v)=(md(x,p),p,1,0);assert_ne!(a,0);while b!=0{let q=a/b;(a,b)=(b,a-q*b);(u,v)=(v,u-q*v);}assert_eq!(a,1);md(u,p)}
fn norm(mut a:Poly)->Poly{while a.last()==Some(&0){a.pop();}a}
fn ps(a:&Poly,b:&Poly,c:i64)->Poly{let mut r=vec![0;a.len().max(b.len())];for(i,x)in a.iter().enumerate(){r[i]=*x;}for(i,x)in b.iter().enumerate(){r[i]=f(r[i]+c*x);}norm(r)}
fn sc(a:&Poly,c:i64)->Poly{norm(a.iter().map(|x|f(x*c)).collect())}
fn pm(a:&Poly,b:&Poly)->Poly{if a.is_empty()||b.is_empty(){return vec![]}let mut r=vec![0;a.len()+b.len()-1];for(i,x)in a.iter().enumerate(){for(j,y)in b.iter().enumerate(){r[i+j]=f(r[i+j]+x*y)}}norm(r)}
fn dg(a:&Poly)->i64{a.len()as i64-1}fn pd(a:&Poly,b:&Poly)->i64{if a.is_empty()||b.is_empty(){-1}else{dg(a)+dg(b)}}
fn dv(a:&Poly,b:&Poly)->(Poly,Poly){assert!(!b.is_empty());let mut r=a.clone();let mut q=vec![0;(dg(a)-dg(b)+1).max(0)as usize];while dg(&r)>=dg(b){let k=(dg(&r)-dg(b))as usize;let c=f(r[r.len()-1]*inv(b[b.len()-1],37));q[k]=c;let mut t=vec![0;k];t.extend(sc(b,c));r=ps(&r,&t,-1);} (norm(q),r)}
fn ev(a:&Poly,x:i64)->i64{a.iter().rev().fold(0,|r,c|f(r*x+c))}
fn certificate(a:&Poly,b:&Poly,cap:i64)->(Value,Option<(Poly,i64)>){
 if dg(a).max(dg(b))>cap{return(json!({"status":"unknown","denied_bound":dg(a).max(dg(b))}),None)}
 let(mut r,mut s,mut u,mut v,mut x,mut y)=(a.clone(),b.clone(),vec![1],vec![],vec![],vec![1]);
 while !s.is_empty(){let(q,t)=dv(&r,&s);let bound=[if q.is_empty()||x.is_empty()||a.is_empty(){-1}else{dg(&q)+dg(&x)+dg(a)},if q.is_empty()||y.is_empty()||b.is_empty(){-1}else{dg(&q)+dg(&y)+dg(b)},pd(&u,a),pd(&v,b)].into_iter().max().unwrap();if bound>cap{return(json!({"status":"unknown","denied_bound":bound}),None)}
  let nx=ps(&u,&pm(&q,&x),-1);let ny=ps(&v,&pm(&q,&y),-1);(r,s,u,v,x,y)=(s,t,x,y,nx,ny);
 }let z=inv(*r.last().unwrap(),37);let(h,u,v)=(sc(&r,z),sc(&u,z),sc(&v,z));let degree=pd(&u,a).max(pd(&v,b));assert!(degree<=cap);assert_eq!(ps(&pm(&u,a),&pm(&v,b),1),h);assert!(dv(a,&h).1.is_empty()&&dv(b,&h).1.is_empty());(json!({"status":"certified","gcd":h,"a":u,"b":v,"expanded_degree":degree}),Some((h,degree)))
}
fn neg(p:Pt)->Pt{p.map(|(x,y)|(x,f(-y)))}
fn plus(p:Pt,q:Pt,a:i64)->Pt{let((x,y),(u,v))=match(p,q){(None,q)=>return q,(p,None)=>return p,(Some(p),Some(q))=>(p,q)};if x==u&&f(y+v)==0{return None}let slope=if x==u{f((3*x*x+a)*inv(2*y,37))}else{f((y-v)*inv(x-u,37))};let z=f(slope*slope-x-u);let intercept=f(y-slope*x);Some((z,f(-slope*z-intercept)))}
fn times(p:Pt,n:i64,a:i64)->Pt{let mut r=None;for _ in 0..n{r=plus(r,p,a);}r}
fn phi(p:Pt)->Pt{p.and_then(|(x,y)|if x==4{None}else{let z=inv(x-4,37);Some((f(x+18*z),f(y*(1-18*z*z))))})}
fn back(p:Pt)->Pt{p.and_then(|(x,y)|if x==29{None}else{let z=inv(x-29,37);times(Some((f(28*(x+35*z)),f(14*y*(1-35*z*z)))),10,7)})}
fn group(a:i64)->Vec<Pt>{let p=if a==7{Some((1,8))}else{Some((32,29))};let b=if a==7{19}else{33};let mut r=BTreeSet::new();let mut q=None;for _ in 0..19{assert!(r.insert(q));q=plus(q,p,a);}assert_eq!(q,None);for p in &r{if let Some((x,y))=p{assert_eq!(f(y*y),f(x*x*x+a*x+b));}}r.into_iter().collect()}
fn policies(a:i64)->Vec<Base>{let half:Vec<_>=group(a).into_iter().flatten().filter(|&(x,y)|Some((x,y))<neg(Some((x,y)))).collect();assert_eq!(half.len(),9);let mut r=vec![];for i in 0..9{for j in i+1..9{for k in j+1..9{r.push([half[i],half[j],half[k]])}}}r}
fn rank(rows:&[Row])->i64{let mut piv=BTreeMap::new();for row in rows{let mut v=*row;for j in 0..3{if v[j]==0{continue}if let Some(p)=piv.get(&j){let p:&Row=p;let z=v[j];for k in j..3{v[k]=md(v[k]-z*p[k],19)}}else{let z=inv(v[j],19);for x in &mut v{*x=md(*x*z,19)}piv.insert(j,v);break}}}piv.len()as i64}
fn solve(eq:&[[i64;4]])->Option<Row>{let mut basis:BTreeMap<usize,[i64;4]>=BTreeMap::new();for row in eq{let mut r=*row;for col in 0..3{if r[col]==0{continue}if let Some(b)=basis.get(&col){let z=r[col];for j in col..4{r[j]=md(r[j]-z*b[j],19)}}else{let z=inv(r[col],19);for x in &mut r{*x=md(*x*z,19)}basis.insert(col,r);break}}if r[..3].iter().all(|&x|x==0){assert_eq!(r[3],0)}}if basis.len()!=3{return None}let mut x=[0;3];for j in(0..3).rev(){let r=basis[&j];x[j]=md(r[3]-(j+1..3).map(|k|r[k]*x[k]).sum::<i64>(),19)}Some(x)}
fn oracle(base:Base,a:i64,q:Pt,cap:i64)->(Value,Vec<Row>,i64){
 let mut distinct;let mut diagonal;let mut certified=[false;2];let mut degree=-1;let(mut le,mut xe)=(0,0);
 if let Some((aa,bb))=q{
  let c1=f(base.iter().map(|p|p.0).sum());let c2=f(base[0].0*base[1].0+base[0].0*base[2].0+base[1].0*base[2].0);let c3=f(base.iter().map(|p|p.0).product());let s=vec![f(-aa),0,1];let p=norm(vec![f(aa*aa+a),f(2*bb),aa]);
  let u=ps(&ps(&ps(&pm(&s,&s),&p,-1),&sc(&s,-c1),1),&vec![c2],1);let v=ps(&pm(&ps(&vec![c1],&s,-1),&p),&vec![f(-c3)],1);let w=ps(&v,&sc(&u,aa),1);let delta=ps(&pm(&s,&s),&sc(&p,-4),1);let mid=sc(&s,inv(2,37));let mut m=vec![1];for bp in base{m=pm(&m,&ps(&mid,&vec![bp.0],-1));}
  let (dc,dp)=certificate(&u,&w,cap);let(tc,tp)=certificate(&delta,&m,cap);distinct=dc;diagonal=tc;distinct["inputs"]=json!([u,w]);diagonal["inputs"]=json!([delta,m]);
  for (k,cert)in[dp,tp].into_iter().enumerate(){if let Some((h,d))=cert{certified[k]=true;degree=degree.max(d);let mut roots=vec![];for l in 0..37{le+=1;if ev(&h,l)!=0{continue}roots.push(l);let ds=f(ev(&s,l).pow(2)-4*ev(&p,l));if(k==0&&ds!=0)||(k==1&&ds==0){xe+=37;}}
   if k==0{distinct["roots"]=json!(roots)}else{diagonal["roots"]=json!(roots)}
  }}
 }else{distinct=json!({"status":"identity_generic"});diagonal=distinct.clone();certified=[true,true];}
 let mut witnesses=vec![];let mut rows=BTreeSet::new();for i in 0..3{for si in [1,-1]{let p=if si==1{Some(base[i])}else{neg(Some(base[i]))};for j in 0..3{for sj in[1,-1]{let r=if sj==1{Some(base[j])}else{neg(Some(base[j]))};if plus(p,r,a)!=q{continue}let branch=if p==r{1}else{0};if !certified[branch]{continue}let mut row=[0;3];row[i]+=si;row[j]+=sj;rows.insert(row);witnesses.push([si*(i as i64+1),sj*(j as i64+1)]);}}}}
 witnesses.sort();assert!(witnesses.windows(2).all(|w|w[0]!=w[1]));let rows:Vec<Row>=rows.into_iter().collect();let data=json!({"query":q,"distinct":distinct,"diagonal":diagonal,"witnesses":witnesses,"rows":rows,"rank":rank(&rows),"actual_expanded_degree":if degree<0{Value::Null}else{json!(degree)},"lambda_evaluations":le,"base_root_evaluations":xe});(data,rows,degree)
}
fn config(role:&str,id:usize,base:Base,cap:i64)->Value{
 let twist=role=="twist"||role=="matched";let a=if twist{28}else{7};let g=if twist{Some((32,29))}else{Some((1,8))};let source=group(7);let queries:Vec<_>=source.iter().map(|&q|if twist{phi(q)}else{q}).collect();let diag:Vec<_>=queries.iter().map(|&q|oracle(base,a,q,cap)).collect();let(mut calls,mut le,mut xe,mut maxd)=(0i64,0i64,0i64,-1i64);
 let mut fresh=|q|{let r=oracle(base,a,q,cap);let k=queries.iter().position(|&p|p==q).unwrap();assert_eq!(r.0,diag[k].0);calls+=1;le+=r.0["lambda_evaluations"].as_i64().unwrap();xe+=r.0["base_root_evaluations"].as_i64().unwrap();maxd=maxd.max(r.2);r};
 let mut recoveries=vec![];for &original in source.iter().skip(1){let mut q=None;let mut eq=vec![];let mut collected=vec![];let mut setup=vec![];let mut logs=None;
  for m in 1..19{q=plus(q,g,a);let r=fresh(q);for row in &r.1{collected.push(*row);eq.push([row[0],row[1],row[2],m]);}setup.push(json!({"m":m,"query":q,"rank":rank(&collected),"rows":r.1}));if rank(&collected)==3{logs=solve(&eq);break}}
  if logs.is_none(){recoveries.push(json!({"source_query":original,"setup":setup,"status":"blocked_setup","logs":null,"shifts":[],"scalar":null}));continue}let logs=logs.unwrap();for i in 0..3{assert_eq!(times(g,md(logs[i],19),a),Some(base[i]));}
  let mut r=if twist{phi(original)}else{original};let mut shifts=vec![];let mut scalar=None;for shift in 0..19{let got=fresh(r);shifts.push(json!({"shift":shift,"query":r,"rows":got.1}));if let Some(row)=got.1.first(){let k=md(row.iter().zip(logs).map(|(x,y)|x*y).sum::<i64>()-shift,19);assert_eq!(times(Some((1,8)),k,7),original);scalar=Some(k);break}r=plus(r,g,a);}assert!(scalar.is_some());recoveries.push(json!({"source_query":original,"setup":setup,"status":"recovered_original","logs":logs,"shifts":shifts,"scalar":scalar}));
 }json!({"role":role,"policy":id,"base":base,"cap":cap,"queries":diag.iter().map(|x|x.0.clone()).collect::<Vec<_>>(),"recoveries":recoveries,"cold_oracle_calls":calls,"cold_lambda_evaluations":le,"cold_base_root_evaluations":xe,"cold_max_expanded_degree":if maxd<0{Value::Null}else{json!(maxd)}})
}
fn sha(b:&[u8])->String{format!("{:x}",Sha256::digest(b))}
fn main(){let args:Vec<_>=std::env::args().collect();if args.len()==2&&args[1]=="--help"{println!("audit INPUT.json NEW_RECEIPT.json");return}if args.len()!=4||args[1]!="audit"{std::process::exit(2)}let result=std::panic::catch_unwind(||{
 let bytes=fs::read(&args[2]).unwrap();let data:Value=serde_json::from_slice(&bytes).unwrap();assert_eq!(data["schema"],"twist.chord-partial/v1");assert_eq!(data["fixture"],json!({"p":37,"E":[7,19],"T":[28,33],"r":19,"P":[1,8],"U":[32,29]}));
 assert_eq!(data.as_object().unwrap().len(),4);
 let ep=policies(7);let tp=policies(28);let ge=group(7);let gt=group(28);for &x in &ge{assert_eq!(back(phi(x)),x);for &y in &ge{assert_eq!(phi(plus(x,y,7)),plus(phi(x),phi(y),28));}}for x in gt{assert_eq!(phi(back(x)),x)}
 let route:Vec<_>=ge.iter().map(|&q|json!({"source":q,"image":phi(q),"back":back(phi(q))})).collect();assert_eq!(data["route"],json!(route));
 let cases=data["cases"].as_array().unwrap();assert_eq!(cases.len(),3024);let mut index=0;let mut recovered=0;let mut blocked=0;let mut unknown=0;let mut partial=0;
 for role in ["main","twist","matched","pullback"]{for id in 0..84{let mut base=if role=="main"||role=="matched"{ep[id]}else{tp[id]};if role=="matched"{for x in &mut base{*x=phi(Some(*x)).unwrap()}}if role=="pullback"{for x in &mut base{*x=back(Some(*x)).unwrap()}}
  for cap in 0..9{let expected=config(role,id,base,cap);assert_eq!(&expected,&cases[index],"configuration {}",index);for r in cases[index]["recoveries"].as_array().unwrap(){if r["status"]=="recovered_original"{recovered+=1}else{blocked+=1}}for q in cases[index]["queries"].as_array().unwrap(){let x=q["distinct"]["status"]=="unknown";let y=q["diagonal"]["status"]=="unknown";unknown+=x as usize+y as usize;partial+=(x!=y)as usize;}index+=1;
  }
 }}
 // Actual degree/rank frontier over every eligible main-curve policy and cap.
 let main:&[Value]=&cases[..756];let mut pairs=0u64;let mut extensions=0u64;let mut eligible=0u64;
 for c in &cases[756..]{if c["recoveries"][0]["status"]!="recovered_original"{continue}eligible+=18;let d=c["cold_max_expanded_degree"].as_i64().unwrap();for target in 0..18{let rank=c["queries"][target+1]["rank"].as_i64().unwrap();let mut dominated=false;let mut strict=0u64;for s in main{if s["recoveries"][target]["status"]!="recovered_original"{continue}let sd=s["cold_max_expanded_degree"].as_i64().unwrap();let sr=s["queries"][target+1]["rank"].as_i64().unwrap();if sd<=d&&sr>=rank{dominated=true}if sd>d&&sr<rank{strict+=1}}pairs+=strict;if !dominated&&strict>0{extensions+=1}}}
 let receipt=json!({"schema":"twist.chord-partial-audit/v1","dataset_sha256":sha(&bytes),"configurations":index,"diagnostic_cells":index*19,"original_target_attempts":recovered+blocked,"original_recoveries":recovered,"blocked_setups":blocked,"unknown_branch_records":unknown,"partial_branch_queries":partial,"eligible_candidate_target_records":eligible,"unmatched_actual_degree_rank_pairs":pairs,"strongest_source_frontier_extensions":extensions,"signed_verdict":false,"matrix_degree":null,"degree_metric":"expanded Bezout witness in stated chord generators; no intrinsic regularity claim"});
 let mut f=fs::OpenOptions::new().write(true).create_new(true).open(&args[3]).unwrap();f.write_all((serde_json::to_string_pretty(&receipt).unwrap()+"\n").as_bytes()).unwrap();
 });if result.is_err(){std::process::exit(101)} }
