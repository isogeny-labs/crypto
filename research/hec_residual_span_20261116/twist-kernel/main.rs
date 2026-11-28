#[allow(dead_code)]#[path="../canonical-root/root.rs"]mod root;
use hec_native_ic::{counted,fp::Fp,jac::{Curve,Div},poly::Poly,instance::factor_base};
use std::collections::HashSet;
fn sigma(c:&Curve,a:&Div,z:u64)->Div{let f=&c.fp;let iz=f.inv(z);let d=a.u.deg()as usize;let mut u=a.u;let mut v=a.v;for j in 0..u.n{u.c[j]=f.mul(u.c[j],f.pow(z,(d-j)as u64));}for j in 0..v.n{v.c[j]=f.mul(v.c[j],f.pow(iz,j as u64));}let b=Div{u,v};assert!(c.is_valid(&b));b}
fn delta(c:&Curve,a:&Div,z:u64)->Div{c.add(&sigma(c,a,z),&c.neg(a))}
fn span(c:&Curve,a:&Div,b:&Div)->HashSet<Div>{(0..5).flat_map(|i|(0..5).map(move|j|c.add(&c.mul(a,i),&c.mul(b,j)))).collect()}
fn main(){counted::disable();let f=Fp::new(601);let d=(2..601).find(|&x|f.legendre(x)==-1).unwrap();let c=Curve::new(f,Poly::from_coeffs(&[f.mul(d,7),0,0,0,0,d]));let fb=factor_base(&c);let points=2*fb.len()+1;assert_eq!(points,523);let ntwist=1-79i64+2731-47479+361201;assert_eq!(ntwist,316375);assert_eq!(ntwist/125,2531);let(z,_)=root::power_map(&f,5);assert_eq!(f.pow(z,5),1);
 let mut chosen=None;for(i,&(x,y))in fb.iter().take(32).enumerate(){let s=c.mul(&c.place(x,y),2531);assert!(c.is_zero(&c.mul(&s,5)));let t=delta(&c,&s,z);let r=delta(&c,&t,z);if !c.is_zero(&r){chosen=Some((i,s,t,r));break}}
 let(i,s,t,r)=chosen.expect("bounded first32 catalogue points did not supply length3 chain");assert!(c.is_zero(&delta(&c,&r,z)));assert!(!c.is_zero(&r));assert!(!c.is_zero(&t));for a in [s,t,r]{assert!(c.is_valid(&a));assert!(c.is_zero(&c.mul(&a,5)));}
 let mut full=HashSet::new();for a in 0..5{for b in 0..5{for k in 0..5{full.insert(c.add(&c.add(&c.mul(&s,a),&c.mul(&t,b)),&c.mul(&r,k)));}}}assert_eq!(full.len(),125);
 let mut hs=vec![];for a in 0..5{hs.push(span(&c,&r,&c.add(&s,&c.mul(&t,a))));}hs.push(span(&c,&r,&t));for h in &hs{assert_eq!(h.len(),25);assert!(h.is_subset(&full));}
 for a in 0..6{let image:HashSet<_>=hs[a].iter().map(|x|sigma(&c,x,z)).collect();assert_eq!(image,hs[if a==5{5}else{(a+1)%5}]);for b in 0..a{assert_ne!(hs[a],hs[b]);}}
 println!("p=601 original_c=7 twist_nonresidue={d} twist_points={points} twist_order={ntwist} cofactor=2531 zeta5={z} catalogue_index={i}");for(name,a)in[("S",s),("deltaS",t),("R",r)]{println!("{name}\tu={:?}\tv={:?}",&a.u.c[..a.u.n],&a.v.c[..a.v.n]);}
 println!("rational_twist_torsion_generated=125 kernel_size=25 kernel_orbits=1,5 source_kernel_frobenius=minus_identity conditional_isotropy=true independent_verification=false");
}
