use std::{env,fs,collections::BTreeMap};
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
fn parse(s:&str)->Vec<i32>{if s=="-"{return vec![]}let v:Vec<_>=s.split(',').map(|x|x.parse::<i32>().unwrap()).collect();assert!(v.windows(2).all(|w|w[0]<w[1]));v}
fn run(){let arg:Vec<_>=env::args().collect();assert_eq!(arg.len(),3);let text=fs::read_to_string(&arg[1]).unwrap();let mut lines=text.lines();assert_eq!(lines.next(),Some("TQ1 7 3 8 1 8 47"));let aa=from(1);let bb=from(8);let delta=from(8);let mut targets=vec![P::O];for x in 0..49{for y in 0..49{let p=P::V(K(from(x),f(0,0)),K(from(y),f(0,0)));if on(p,aa,bb,delta){targets.push(p)}}}assert_eq!(targets.len(),47);
 // Reference is point enumeration, not a candidate quartic root list.
 let mut by_target=BTreeMap::<(i32,i32),(Vec<i32>,Vec<i32>)>::new();for a0 in 0..49{for c0 in 0..49{for d0 in 0..49{let p=P::V(K(from(a0),f(1,0)),K(from(c0),from(d0)));if !on(p,aa,bb,delta){continue}let q=add(p,conj(p),aa,delta);let key=match q{P::O=>panic!("off-axis lift has zero trace"),P::V(x,y)=>{assert_eq!(x.1,f(0,0));assert_eq!(y.1,f(0,0));(enc(x.0),enc(y.0))}};let z=by_target.entry(key).or_default();z.0.push(d0);if a0<7{z.1.push(d0)}}}}for z in by_target.values_mut(){z.0.sort();z.1.sort();assert!(z.0.windows(2).all(|w|w[0]<w[1]));}
 let mut total=0;let mut restricted=0;let mut positive=0;for (id,q) in targets.iter().enumerate(){let row=lines.next().unwrap();let v:Vec<_>=row.split_whitespace().collect();assert_eq!(v.len(),7);assert_eq!(v[0].parse::<usize>().unwrap(),id);let key=match q{P::O=>(-1,-1),P::V(x,y)=>(enc(x.0),enc(y.0))};assert_eq!(v[1].parse::<i32>().unwrap(),key.0);assert_eq!(v[2].parse::<i32>().unwrap(),key.1);let r=parse(v[4]);let s=parse(v[6]);assert_eq!(v[3].parse::<usize>().unwrap(),r.len());assert_eq!(v[5].parse::<usize>().unwrap(),s.len());let want=by_target.get(&key).cloned().unwrap_or_default();assert_eq!((r.clone(),s.clone()),want);
 if let P::V(x,y)=q{for &code in &r{let l=from(code);let ll=m(l,l);let g=a(a(a(a(m(ll,ll),n(m(f(6,0),m(x.0,ll)))),n(m(f(3,0),m(x.0,x.0)))),n(m(f(8,0),m(y.0,l)))),n(m(f(4,0),a(aa,delta))));assert_eq!(g,f(0,0));}}
 total+=r.len();restricted+=s.len();positive+=usize::from(!r.is_empty());}assert!(lines.next().is_none());fs::write(&arg[2],format!("{{\"queries\":47,\"chart_lifts\":{},\"subfield_lifts\":{},\"positive_queries\":{},\"matrix_degree\":null,\"quartic_degree\":4,\"expanded_curve_equation_degree_bound\":6}}\n",total,restricted,positive)).unwrap();}
fn main(){if std::panic::catch_unwind(run).is_err(){std::process::exit(101)}}
