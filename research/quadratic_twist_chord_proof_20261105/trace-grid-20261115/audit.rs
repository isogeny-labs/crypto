use std::{env,fs,collections::{BTreeMap,BTreeSet}};
#[derive(Clone,Copy,PartialEq,Eq,Debug)]struct F(i32,i32);
fn f(a:i32,b:i32)->F{F(a.rem_euclid(7),b.rem_euclid(7))}
fn from(n:i32)->F{f(n%7,n/7)}fn enc(x:F)->i32{x.0+7*x.1}
fn a(x:F,y:F)->F{f(x.0+y.0,x.1+y.1)}fn n(x:F)->F{f(-x.0,-x.1)}fn m(x:F,y:F)->F{let c=[x.0*y.0,x.0*y.1+x.1*y.0,x.1*y.1];f(c[0]+3*c[2],c[1])}
fn pw(mut x:F,mut k:i32)->F{let mut r=f(1,0);while k>0{if k%2==1{r=m(r,x)}x=m(x,x);k/=2}r}
fn iv(x:F)->F{assert!(x!=f(0,0));pw(x,47)}fn prime(k:i32)->bool{k>=2&&(2..k).all(|d|k%d!=0)}
fn count(aa:F,bb:F)->i32{let mut out=1;for x in 0..49{let z=from(x);let rhs=a(a(m(m(z,z),z),m(aa,z)),bb);for y in 0..49{if m(from(y),from(y))==rhs{out+=1}}}out}
#[derive(Clone,Copy,PartialEq,Eq,Debug)]struct K(F,F);
fn ka(x:K,y:K)->K{K(a(x.0,y.0),a(x.1,y.1))}fn kn(x:K)->K{K(n(x.0),n(x.1))}
fn km(x:K,y:K,d:F)->K{K(a(m(x.0,y.0),m(d,m(x.1,y.1))),a(m(x.0,y.1),m(x.1,y.0)))}
fn ki(x:K,d:F)->K{let den=iv(a(m(x.0,x.0),n(m(d,m(x.1,x.1)))));K(m(x.0,den),n(m(x.1,den)))}fn ci(x:i32)->K{K(f(x,0),f(0,0))}
#[derive(Clone,Copy,PartialEq,Eq,Debug)]enum P{O,V(K,K)}
fn add(p:P,q:P,aa:F,d:F)->P{match(p,q){(P::O,_)=>q,(_,P::O)=>p,(P::V(x,y),P::V(u,v))=>{if x==u&&ka(y,v)==ci(0){return P::O}let l=if x==u{km(ka(km(ci(3),km(x,x,d),d),K(aa,f(0,0))),ki(km(ci(2),y,d),d),d)}else{km(ka(v,kn(y)),ki(ka(u,kn(x)),d),d)};let z=ka(ka(km(l,l,d),kn(x)),kn(u));P::V(z,ka(km(l,ka(x,kn(z)),d),kn(y)))}}}
fn mul(mut p:P,mut k:i32,aa:F,d:F)->P{let mut o=P::O;while k>0{if k%2==1{o=add(o,p,aa,d)}p=add(p,p,aa,d);k/=2}o}
fn conj(p:P)->P{match p{P::O=>p,P::V(x,y)=>P::V(K(x.0,n(x.1)),K(y.0,n(y.1)))}}
fn on(p:P,aa:F,bb:F,d:F)->bool{match p{P::O=>true,P::V(x,y)=>km(y,y,d)==ka(ka(km(km(x,x,d),x,d),km(K(aa,f(0,0)),x,d)),K(bb,f(0,0)))}}
fn pc(p:P)->i32{match p{P::O=>-1,P::V(x,y)=>((enc(x.0)*49+enc(x.1))*49+enc(y.0))*49+enc(y.1)}}
fn key(p:P)->(i32,i32){match p{P::O=>(-1,-1),P::V(x,y)=>{assert_eq!(x.1,f(0,0));assert_eq!(y.1,f(0,0));(enc(x.0),enc(y.0))}}}
fn parse(s:&str)->BTreeSet<i32>{if s=="-"{return BTreeSet::new()}let v:Vec<i32>=s.split(',').map(|x|x.parse().unwrap()).collect();assert!(v.windows(2).all(|w|w[0]<w[1]));v.into_iter().collect()}
fn run(){
 let args:Vec<_>=env::args().collect();assert_eq!(args.len(),3);let text=fs::read_to_string(&args[1]).unwrap();let mut lines=text.lines();assert_eq!(lines.next(),Some("TG1 7 3 8 1 8 47 53 56 2632"));
 let aa=from(1);let bb=from(8);let d=from(8);let zero=f(0,0);
 let mut source=vec![P::O];for x in 0..49{for y in 0..49{let p=P::V(K(from(x),zero),K(from(y),zero));if on(p,aa,bb,d){source.push(p)}}}assert_eq!(source.len(),47);
 // Reference enumerates ALL K-rational points, never candidate quartic roots.
 let mut square_roots=BTreeMap::<i32,Vec<K>>::new();for y0 in 0..49{for y1 in 0..49{let y=K(from(y0),from(y1));let z=km(y,y,d);square_roots.entry(enc(z.0)*49+enc(z.1)).or_default().push(y);}}
 let mut all=vec![(P::O,P::O)];for x0 in 0..49{for x1 in 0..49{let x=K(from(x0),from(x1));let rhs=ka(ka(km(km(x,x,d),x,d),km(K(aa,zero),x,d)),K(bb,zero));for &y in square_roots.get(&(enc(rhs.0)*49+enc(rhs.1))).unwrap_or(&vec![]){let p=P::V(x,y);let q=add(p,conj(p),aa,d);key(q);all.push((p,q));}}}assert_eq!(all.len(),2491);
 let betas=[1,7,8,9,10,11,12,13];let mut summaries=vec![];let mut gid=0;let mut total=0;let mut max_pairs=0;let mut eligible=0;
 for gamma in (0..49).step_by(7){for beta in betas{
  let mut by_query=BTreeMap::<(i32,i32),BTreeSet<i32>>::new();let mut axis=BTreeMap::<(i32,i32),BTreeSet<i32>>::new();
  for &(p,q) in &all{let included=match p{P::O=>true,P::V(x,_)=>a(x.0,n(from(gamma))).1==0&&m(x.1,iv(from(beta))).1==0};if !included{continue}let k=key(q);by_query.entry(k).or_default().insert(pc(p));if matches!(p,P::O)||matches!(p,P::V(x,_) if x.1==zero){axis.entry(k).or_default().insert(pc(p));}}
  let mut xs=BTreeSet::new();let mut affine=0;let mut fibers=0;let mut zero_fiber=0;let mut axis_count=0;
  for (id,&q) in source.iter().enumerate(){let cols:Vec<_>=lines.next().unwrap().split_whitespace().collect();assert_eq!(cols.len(),8);let nums:Vec<i32>=cols[..7].iter().map(|x|x.parse().unwrap()).collect();let k=key(q);assert_eq!(&nums[..6],&[gid,gamma,beta,id as i32,k.0,k.1]);let got=parse(cols[7]);assert_eq!(nums[6] as usize,got.len());let want=by_query.get(&k).cloned().unwrap_or_default();assert_eq!(got,want,"grid {gid} query {id}");
   let axes=axis.get(&k).cloned().unwrap_or_default();assert!(axes.is_subset(&got));axis_count+=axes.len();fibers+=got.len();
   if !got.is_empty(){match q{P::O=>zero_fiber=got.len(),P::V(x,_)=>{affine+=1;xs.insert(enc(x.0));}}}
  }
  assert_eq!(affine,2*xs.len());let pairs=xs.len();max_pairs=max_pairs.max(pairs);eligible+=usize::from(pairs>14);total+=fibers;
  summaries.push(format!("{{\"id\":{},\"gamma\":{},\"beta\":{},\"projected_nonidentity_points\":{},\"signed_pairs\":{},\"lifts\":{},\"axis_lifts\":{},\"zero_trace_lifts\":{},\"source_membership_degree\":{},\"direct_subfield_pullback_degree\":14}}",gid,gamma,beta,affine,pairs,fibers,axis_count,zero_fiber,pairs));gid+=1;
 }}assert_eq!(gid,56);assert!(lines.next().is_none());
 fs::write(&args[2],format!("{{\"grids\":56,\"queries_per_grid\":47,\"cells\":2632,\"extension_points\":2491,\"grid_lift_incidence\":{},\"maximum_signed_pairs\":{},\"grids_over_degree14_threshold\":{},\"matrix_degree\":null,\"grids_detail\":[{}]}}\n",total,max_pairs,eligible,summaries.join(","))).unwrap();
}
fn main(){if std::panic::catch_unwind(run).is_err(){std::process::exit(101)}}
