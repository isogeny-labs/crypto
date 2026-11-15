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
fn run(){let argv:Vec<_>=env::args().collect();assert_eq!(argv.len(),3);let raw=fs::read_to_string(&argv[1]).unwrap();let body=raw.trim().strip_prefix('{').unwrap().strip_suffix('}').unwrap();let mut map=BTreeMap::new();for p in body.split(','){let(k,v)=p.split_once(':').unwrap();assert!(map.insert(k.trim().trim_matches('"').to_string(),v.trim().to_string()).is_none());}assert_eq!(map.len(),13);assert_eq!(map["schema"],"\"twist.trace-fixture/v1\"");let get=|s:&str|map[s].parse::<i32>().unwrap();assert_eq!(get("p"),7);assert_eq!(get("base_modulus"),3);
 let d=(1..49).map(from).find(|&x|pw(x,24)==f(6,0)).unwrap();assert_eq!(get("delta"),enc(d));let mut first=None;let mut eligible=0;for x in 0..49{for y in 0..49{let aa=from(x);let bb=from(y);let dis=a(m(f(4,0),m(m(aa,aa),aa)),m(f(27,0),m(bb,bb)));if dis==f(0,0){continue}let j=m(m(f(1728,0),m(f(4,0),m(m(aa,aa),aa))),iv(dis));if j.1==0{continue}let r=count(aa,bb);let t=100-r;if r<11||t<11||r==t||!prime(r)||!prime(t){continue}eligible+=1;if first.is_none(){first=Some((aa,bb,r,t))}}}let(aa,bb,r,t)=first.unwrap();assert_eq!(get("A"),enc(aa));assert_eq!(get("B"),enc(bb));assert_eq!(get("source_order"),r);assert_eq!(get("twist_order"),t);assert_eq!(get("eligible_curves"),eligible);let at=m(m(d,d),aa);let bt=m(m(m(d,d),d),bb);assert_eq!(get("twist_A"),enc(at));assert_eq!(get("twist_B"),enc(bt));assert_eq!(count(at,bt),t);let h=(r+1)/2;assert_eq!(get("half"),h);
 let mut e=vec![P::O];let mut tw=vec![P::O];for x in 0..49{for y in 0..49{let p=P::V(K(from(x),f(0,0)),K(from(y),f(0,0)));if on(p,aa,bb,d){e.push(p)}if on(p,at,bt,d){tw.push(p)}}}let mut checks=0;for &p in &e{assert_eq!(mul(p,r,aa,d),P::O);for &u in &tw{let c=match u{P::O=>u,P::V(x,y)=>P::V(K(m(x.0,iv(d)),f(0,0)),K(f(0,0),m(y.0,iv(m(d,d)))))};assert!(on(c,aa,bb,d));assert_eq!(add(c,conj(c),aa,d),P::O);let l=add(mul(p,h,aa,d),c,aa,d);assert!(on(l,aa,bb,d));assert_eq!(add(l,conj(l),aa,d),p);checks+=1}}assert_eq!(get("trace_pairs"),checks);
 // Separate exhaustive extension-field point count; no producer point list supplied.
 let mut roots=BTreeMap::new();for k in 0..2401{let z=K(from(k%49),from(k/49));let q=km(z,z,d);*roots.entry((enc(q.0),enc(q.1))).or_insert(0)+=1;}let mut ext=1;for k in 0..2401{let x=K(from(k%49),from(k/49));let q=ka(ka(km(km(x,x,d),x,d),km(K(aa,f(0,0)),x,d)),K(bb,f(0,0)));ext+=roots.get(&(enc(q.0),enc(q.1))).copied().unwrap_or(0);}assert_eq!(ext,r*t);fs::write(&argv[2],format!("{{\"accepted\":true,\"trace_pairs\":{},\"extension_points\":{},\"eligible_curves\":{}}}\n",checks,ext,eligible)).unwrap();}
fn main(){if std::panic::catch_unwind(run).is_err(){std::process::exit(101)}}
