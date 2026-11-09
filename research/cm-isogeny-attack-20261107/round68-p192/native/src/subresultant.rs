static NS:[AtomicU64;9]=[AtomicU64::new(0),AtomicU64::new(0),AtomicU64::new(0),AtomicU64::new(0),AtomicU64::new(0),AtomicU64::new(0),AtomicU64::new(0),AtomicU64::new(0),AtomicU64::new(0)];
fn normstats()->[u64;9]{std::array::from_fn(|i|NS[i].load(Ordering::Relaxed))}
fn pzero<const P:i64,const D:i64>(a:&[F<P,D>])->bool{a.iter().all(|&x|x==F::c(0))}
fn resultant<const P:i64,const D:i64>(f:&[F<P,D>],g:&[F<P,D>])->F<P,D>{
 let mut f=trim(f.to_vec());let mut g=trim(g.to_vec());let mut out=F::c(1);
 loop {let n=f.len()-1;let m=g.len()-1;if n==0{return out*f[0].pow(m as u64)}if pzero(&g){return F::c(0)}if m==0{return out*g[0].pow(n as u64)}let (_,r)=divide(&f,&g);if pzero(&r){return F::c(0)}if n*m%2==1{out=-out}out=out*g[m].pow((n-(r.len()-1))as u64);f=g;g=r}
}
fn egcd_inverse<const P:i64,const D:i64>(h:&[F<P,D>],e:&[F<P,D>])->(Vec<F<P,D>>,Option<Vec<F<P,D>>>){
 let mut r0=h.to_vec();let mut r1=trim(e.to_vec());let mut s0=vec![F::c(0)];let mut s1=vec![F::c(1)];
 while !pzero(&r1){if r1.len()==1 {let inv=r1[0].inv();return(vec![F::c(1)],Some(red(scaled(&s1,inv),h)))}let (q,r)=divide(&r0,&r1);let s=red(sum(&s0,&pm(&q,&s1),F::c(-1)),h);r0=r1;r1=r;s0=s1;s1=s}
 let lead=r0.last().unwrap().inv();(scaled(&r0,lead),None)
}
fn field_normjet<const P:i64,const D:i64>(h:&[F<P,D>],e0:&[F<P,D>],e1:&[F<P,D>])->([F<P,D>;2],usize){
 let n=h.len()-1;assert!(n>=1);let e0=red(e0.to_vec(),h);let e1=red(e1.to_vec(),h);
 if pzero(&e1){return([resultant(h,&e0),F::c(0)],8)}
 let (d,inv)=egcd_inverse(h,&e0);
 if d.len()==1 {let a=resultant(h,&e0);assert_ne!(a,F::c(0));let q=red(pm(&pm(&inv.unwrap(),&e1),&der(h)),h);let tr=q.get(n-1).copied().unwrap_or(F::c(0));return([a,a*tr],5)}
 if d.len()==2 {let t=-d[0]/d[1];let (bar,rem)=divide(h,&d);assert!(pzero(&rem));return([F::c(0),ev(&e1,t)*resultant(&bar,&e0)],6)}
 ([F::c(0),F::c(0)],7)
}
fn normjet<const P:i64,const D:i64>(al:&Alg<P,D>,e0:Vec<F<P,D>>,e1:Vec<F<P,D>>)->([F<P,D>;2],u64){
 let st=counts();let (v,branch)=field_normjet(&al.h,&e0,&e1);let fc=diff(st,counts());NS[0].fetch_add(fc[0],Ordering::Relaxed);NS[1].fetch_add(fc[1],Ordering::Relaxed);NS[4].fetch_add(1,Ordering::Relaxed);NS[branch].fetch_add(1,Ordering::Relaxed);
 let st=counts();let (w,piv)=normjet_dense(al,e0,e1);let dc=diff(st,counts());NS[2].fetch_add(dc[0],Ordering::Relaxed);NS[3].fetch_add(dc[1],Ordering::Relaxed);assert_eq!(v,w,"field norm vs dense determinant");
 if std::env::args().any(|s|s=="--dense-reference"){(w,piv)}else{(v,0)}
}
fn norm_controls<const P:i64,const D:i64>(g:Pt<P,D>,a:i64,b:i64,r:u64)->Value {
 let bg=base(g,a,r);let al=algebra(&bg,&bg,a,b);let roots:Vec<_>=bg.baby.iter().skip(1).map(|v|v.unwrap().0).collect();let t=roots[0];let mut cases=vec![("unit".to_string(),vec![F::c(1)],vec![F::c(1)]),("simple".to_string(),vec![-t,F::c(1)],vec![F::c(1)]),("zero".to_string(),vec![F::c(0)],vec![F::c(1)])];
 if roots.len()>=2{cases.push(("multiple".to_string(),pm(&[-t,F::c(1)],&[-roots[1],F::c(1)]),vec![F::c(1)]))}
 for c in 0..8{cases.push((format!("polynomial_{}",c),vec![F::c(c+1),F::c(c),F::c(1)],vec![F::c(1),F::c(c)]))}
 let mut rows=vec![];for(name,e0,e1)in cases{let st=counts();let (v,branch)=field_normjet(&al.h,&e0,&e1);let fc=diff(st,counts());let st=counts();let(w,_)=normjet_dense(&al,e0.clone(),e1.clone());let dc=diff(st,counts());let explicit=roots.iter().fold([F::c(1),F::c(0)],|v,&x|upd(v,[ev(&e0,x),ev(&e1,x)]));assert_eq!(v,w);assert_eq!(v,explicit);rows.push(json!({"name":name,"branch":branch,"jet":v.map(|x|x.code()),"field_counts":fc,"dense_counts":dc}))}
 json!({"p":P,"r":r,"b":roots.len(),"rows":rows})
}
