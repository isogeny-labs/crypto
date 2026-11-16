include!("extracted.rs");
fn poly(a:&[u64],x:u64,p:u64)->u64 { a.iter().rev().fold(0,|r,&c|((r as u128*x as u128+c as u128)%p as u128) as u64) }
fn normalized(r:SieveResult)->(Vec<u64>,usize,usize,usize,usize,usize,usize,usize,Vec<Vec<u64>>) {(r.0.into_iter().collect(),r.1,r.2,r.3,r.4,r.5,r.6,r.7,r.8)}
fn fixture(d:Divisor,q:u64,p:u64)->Vec<u64>{let mut h=vec![0;5];for i in 0..d.degree {for j in 0..d.degree {h[i+j]=(h[i+j]+d.v[i]*d.v[j])%p;}}for i in 0..=d.degree {h[i]=(h[i]+p-q*d.u[i]%p)%p;}while h.last()==Some(&0){h.pop();}h}
fn pair(d:Divisor,q:u64,p:u64,id:usize,coverage:&mut [usize;5]) {
 let h=fixture(d,q,p);let ys=rational_y_by_x(&h,p);
 for x in 0..p {let u=poly(&d.u[..=d.degree],x,p);let v=poly(&d.v[..d.degree],x,p);let branch=if u==0 {if 2*v%p!=0 {2}else if q==0{3}else{4}}else if ys[x as usize].is_empty(){1}else{0};coverage[branch]+=1;}
 BASE_MODULUS.with(|v|v.set(p));PRICE_ACTIVE.with(|v|v.set(true));reset_arithmetic_counts();meter::reset();
 let old=point_sieve_candidates(d,&h,p,&ys);let oa=arithmetic_counts();let om=meter::snapshot();let old=normalized(old);
 reset_arithmetic_counts();meter::reset();let new=point_sieve_candidates_finite_difference(d,&h,p,&ys);let na=arithmetic_counts();let nm=meter::snapshot();let new=normalized(new);
 assert_eq!(old,new,"pair {id}");let k=(2*d.degree+1) as u64;
 assert_eq!(oa.base_adds+3*p+2,na.base_adds+k*p);assert_eq!(oa.base_muls,na.base_muls+k*p);
 let mut adj=na;adj.base_adds=oa.base_adds;adj.base_muls=oa.base_muls;assert_eq!(adj,oa);
 assert_eq!(om.collection_entry_visits,nm.collection_entry_visits+(k*(p-1)) as u128);let mut adj=nm;adj.collection_entry_visits=om.collection_entry_visits;assert_eq!(adj,om);
 println!("PAIR {id} p={p} degree={} q={q} old_ops={oa:?} new_ops={na:?} old_meter={om:?} new_meter={nm:?} result={old:?}",d.degree);
 PRICE_ACTIVE.with(|v|v.set(false));
}
fn main(){
 let mut pairs=0;let mut coverage=[0usize;5];
 for p in [3u64,5,7] {for degree in 0..=2 {for code in 0..p.pow((2*degree) as u32){let mut z=code;let mut d=Divisor{degree,u:[0;3],v:[0;2]};d.u[degree]=1;for i in 0..degree {d.u[i]=z%p;z/=p;d.v[i]=z%p;z/=p;}for q in 0..=1{pair(d,q,p,pairs,&mut coverage);pairs+=1;}}}}
 assert_eq!(pairs,6386);
 for p in [3u64,5,7] {for degree in 0..=1{let mut d=Divisor{degree,u:[p-1;3],v:[p-1;2]};d.u[degree]=1;for q in 0..=1{pair(d,q,p,pairs,&mut coverage);pairs+=1;}}}assert_eq!(pairs,6398);assert!(coverage.iter().all(|&n|n>0));
 let mut sequences=0;let mut samples=0;
 for p in [2u64,3,5,7]{for code in 0..p.pow(5){let mut z=code;let mut c=[0;5];for v in &mut c{*v=z%p;z/=p;}let mut u=c[0];let mut du=add(c[1],c[2],p);let ddu=add(c[2],c[2],p);let mut v=c[3];for x in 0..p {assert_eq!(u,poly(&c[..3],x,p));assert_eq!(v,poly(&c[3..],x,p));u=add(u,du,p);du=add(du,ddu,p);v=add(v,c[4],p);samples+=1;}sequences+=1;}}assert_eq!(sequences,20207);
 let p=1u64<<63;let mut boundaries=0;
 for c in [[0,0,0,0,0],[p-1,p-1,p-1,p-1,p-1],[1,p-1,1,0,p-1],[p-1,0,p-1,1,1]] {for x in [0,1,2,3,p/2,p-3,p-2,p-1] {let u=poly(&c[..3],x,p);let d=add(c[1],mul(c[2],add(add(x,x,p),1,p),p),p);let v=poly(&c[3..],x,p);let next=(x+1)%p;assert_eq!(add(u,d,p),poly(&c[..3],next,p));assert_eq!(add(v,c[4],p),poly(&c[3..],next,p));assert_eq!(add(d,add(c[2],c[2],p),p),add(c[1],mul(c[2],add(add(next,next,p),1,p),p),p));boundaries+=1;}}
 // Deliberately wrong recurrence controls must disagree.
 let p=7;let u=0;let wrong_du=0;assert_ne!(add(u,wrong_du,p),poly(&[0,0,1],1,p));
 let du=1;let ddu=2;assert_ne!(add(u,add(du,ddu,p),p),poly(&[0,0,1],1,p));
 let stuck_u=0;assert_ne!(stuck_u,poly(&[0,1,0],1,p));
 println!("SUMMARY pairs={pairs} sequences={sequences} samples={samples} boundary_transitions={boundaries} rejected_wrong_controls=3 branch_coverage={coverage:?}");
}
