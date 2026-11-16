//! Exact algebraic interface. Sparse classical products/direct evaluation;
//! the KU multipoint backend in the written complexity reduction is NOT implemented.
use std::collections::BTreeMap;
pub trait Ops{type El:Clone+PartialEq;fn c(&self,n:i64)->Self::El;fn add(&self,a:&Self::El,b:&Self::El)->Self::El;fn mul(&self,a:&Self::El,b:&Self::El)->Self::El;}
pub type Poly<E>=BTreeMap<(usize,usize),E>;
#[derive(Clone)]
pub struct Four<E:Clone>{pub p:[Poly<E>;4]}
fn term<O:Ops>(o:&O,c:O::El,x:usize,y:usize)->Poly<O::El>{let mut p=BTreeMap::new();if c!=o.c(0){p.insert((x,y),c);}p}
fn add<O:Ops>(o:&O,a:&Poly<O::El>,b:&Poly<O::El>)->Poly<O::El>{let mut p=a.clone();for(k,v)in b{let n=o.add(p.get(k).unwrap_or(&o.c(0)),v);if n==o.c(0){p.remove(k);}else{p.insert(*k,n);}}p}
fn scale<O:Ops>(o:&O,a:&Poly<O::El>,b:&O::El)->Poly<O::El>{if *b==o.c(0){return BTreeMap::new()}a.iter().filter_map(|(k,v)|{let c=o.mul(v,b);if c==o.c(0){None}else{Some((*k,c))}}).collect()}
fn pmul<O:Ops>(o:&O,a:&Poly<O::El>,b:&Poly<O::El>)->Poly<O::El>{let mut p=BTreeMap::new();for(&(i,j),v)in a{for(&(k,l),w)in b{let key=(i+k,j+l);let c=o.add(p.get(&key).unwrap_or(&o.c(0)),&o.mul(v,w));if c==o.c(0){p.remove(&key);}else{p.insert(key,c);}}}p}
fn ssum<O:Ops>(o:&O,parts:&[Poly<O::El>])->Poly<O::El>{parts.iter().fold(BTreeMap::new(),|a,b|add(o,&a,b))}
pub fn branch_polynomial<O:Ops>(o:&O,a:&O::El,b:&O::El)->Poly<O::El>{let ft=ssum(o,&[term(o,o.c(1),3,0),term(o,a.clone(),1,0),term(o,b.clone(),0,0)]);let fv=ssum(o,&[term(o,o.c(1),0,3),term(o,a.clone(),0,1),term(o,b.clone(),0,0)]);pmul(o,&ft,&fv)}
pub fn one<O:Ops>(o:&O)->Four<O::El>{Four{p:[term(o,o.c(1),0,0),BTreeMap::new(),BTreeMap::new(),BTreeMap::new()]}}
// Channel order: 1,W,eps,eps*W. In particular the odd W channel retains the signed mark.
pub fn times<O:Ops>(o:&O,h:&Poly<O::El>,a:&Four<O::El>,b:&Four<O::El>)->Four<O::El>{
 let mut p: [Poly<O::El>;4]=std::array::from_fn(|_|BTreeMap::new());
 for i in 0..4{for j in 0..4{let eps=(i/2)+(j/2);if eps>1{continue}let ww=(i%2)+(j%2);let mut m=pmul(o,&a.p[i],&b.p[j]);if ww==2{m=pmul(o,&m,h)}let channel=2*eps+ww%2;p[channel]=add(o,&p[channel],&m);}}
 Four{p}
}
pub fn balanced<O:Ops>(o:&O,h:&Poly<O::El>,v:&[Four<O::El>])->Four<O::El>{if v.is_empty(){return one(o)}if v.len()==1{return v[0].clone()}let m=v.len()/2;times(o,h,&balanced(o,h,&v[..m]),&balanced(o,h,&v[m..]))}
fn ev<O:Ops>(o:&O,p:&Poly<O::El>,t:&O::El,v:&O::El)->O::El{let dx=p.keys().map(|a|a.0).max().unwrap_or(0);let dy=p.keys().map(|a|a.1).max().unwrap_or(0);let mut tx=vec![o.c(1)];let mut vy=tx.clone();for i in 1..=dx{tx.push(o.mul(&tx[i-1],t))}for i in 1..=dy{vy.push(o.mul(&vy[i-1],v))}p.iter().fold(o.c(0),|z,(&(i,j),c)|o.add(&z,&o.mul(c,&o.mul(&tx[i],&vy[j]))))}
pub fn evaluate<O:Ops>(o:&O,p:&Four<O::El>,t:&O::El,v:&O::El,w:&O::El)->[O::El;2]{let e:Vec<_>=p.p.iter().map(|c|ev(o,c,t,v)).collect();[o.add(&e[0],&o.mul(w,&e[1])),o.add(&e[2],&o.mul(w,&e[3]))]}
pub fn jetmul<O:Ops>(o:&O,a:&[O::El;2],b:&[O::El;2])->[O::El;2]{[o.mul(&a[0],&b[0]),o.add(&o.mul(&a[0],&b[1]),&o.mul(&a[1],&b[0]))]}
pub fn factor<O:Ops>(o:&O,a:&O::El,b:&O::El,x:&O::El,y:&O::El,z:&O::El,w:&O::El)->Four<O::El>{
 let triples=|c:&O::El,axis:bool|{let mon=|cc:O::El,k|if axis{term(o,cc,0,k)}else{term(o,cc,k,0)};
 let c2=o.mul(c,c);let aa=o.mul(a,a);
 let d=ssum(o,&[mon(o.c(1),2),mon(o.mul(&o.c(-2),c),1),mon(c2.clone(),0)]);
 let n=ssum(o,&[mon(c.clone(),2),mon(o.add(&c2,a),1),mon(o.add(&o.mul(a,c),&o.mul(&o.c(2),b)),0)]);
 let f=ssum(o,&[mon(c2,2),mon(o.add(&o.mul(&o.c(-2),&o.mul(a,c)),&o.mul(&o.c(-4),b)),1),mon(o.add(&aa,&o.mul(&o.c(-4),&o.mul(b,c))),0)]);(d,n,f)};
 let(dg,ng,fg)=triples(x,false);let(dq,nq,fq)=triples(z,true);
 let m0=ssum(o,&[pmul(o,&fg,&dq),pmul(o,&dg,&fq),scale(o,&pmul(o,&ng,&nq),&o.c(-2))]);
 let m1=scale(o,&ssum(o,&[pmul(o,&ng,&dq),scale(o,&pmul(o,&dg,&nq),&o.c(-1))]),&o.c(2));
 Four{p:[m0,term(o,o.mul(&o.c(8),&o.mul(y,w)),0,0),m1,BTreeMap::new()]}
}
pub fn contract<O:Ops>(o:&O,a:&O::El,b:&O::El,babies:&[(O::El,O::El,O::El)],giants:&[(O::El,O::El,O::El,O::El)],chunk:usize)->([O::El;2],serde_json::Value){
 let h=branch_polynomial(o,a,b);let mut out=[o.c(1),o.c(0)];let mut maxdegree=0;let mut slots=0;let mut blocks=0;
 for c in giants.chunks(chunk.max(1)){let inputs:Vec<_>=c.iter().map(|(x,y,z,w)|factor(o,a,b,x,y,z,w)).collect();let prod=balanced(o,&h,&inputs);let degree=prod.p.iter().flat_map(|p|p.keys()).map(|&(i,j)|i.max(j)).max().unwrap_or(0);assert!(degree<=2*c.len());maxdegree=maxdegree.max(degree);slots=slots.max(prod.p.iter().map(|p|p.len()).sum::<usize>());
 for(t,v,w)in babies{out=jetmul(o,&out,&evaluate(o,&prod,t,v,w));}blocks+=1;}
 (out,serde_json::json!({"giants":giants.len(),"baby_points":babies.len(),"chunks":blocks,"chunk_size":chunk,"max_individual_degree":maxdegree,"max_nonzero_coefficient_slots":slots,"four_polynomial_dense_slot_bound":4*(2*chunk+1)*(2*chunk+1),"evaluation_backend":"direct_sparse; KU not implemented","product_backend":"classical_sparse"}))
}
