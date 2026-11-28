mod arith;
mod orders;
mod builder {
 use crate::arith::{Curve,Div,E,O,P,Pol};
 use hec_native_ic::{fp::Fp,poly::Poly};
 include!("kernels.rs");
}
use arith::{Curve,Field,Pol,P,Z,O,base};
use std::{fs,path::Path,collections::HashSet};
fn main(){let a:Vec<_>=std::env::args().collect();let public=fs::read_to_string(&a[1]).unwrap();let mut it=public.split_whitespace();assert_eq!(it.next(),Some("COLD_INPUT"));let ns:Vec<u64>=it.map(|s|s.parse().unwrap()).collect();assert_eq!(&ns[..2],&[3,P]);let ell=ns[2];assert_eq!(ns[3],8);let coeff=&ns[4..12];let fp=hec_native_ic::fp::Fp::new(P);let cp=hec_native_ic::poly::Poly::from_coeffs(coeff);assert_eq!(hec_native_ic::poly::Poly::gcd(&cp,&cp.deriv(&fp),&fp).deg(),0);
 let l:[i64;7]=fs::read_to_string(&a[2]).unwrap().split_whitespace().map(|s|s.parse().unwrap()).collect::<Vec<_>>().try_into().unwrap();assert_eq!(l[6],(P*P*P)as i64);
 let out=Path::new(&a[3]);assert!(!out.exists());fs::create_dir(out).unwrap();let field=Field::new();assert!(field.irreducible());assert_eq!(field.q%4,3);
 let mut rng=hec_native_ic::rng::Rng::new(202611151300);for _ in 0..32{let x=std::array::from_fn(|_|rng.below(P));let y=std::array::from_fn(|_|rng.below(P));let z=std::array::from_fn(|_|rng.below(P));assert_eq!(field.mul(field.mul(x,y),z),field.mul(x,field.mul(y,z)));assert_eq!(field.mul(x,field.add(y,z)),field.add(field.mul(x,y),field.mul(x,z)));if x!=Z{assert_eq!(field.mul(x,field.inv(x)),O);}let mut f=x;for _ in 0..13{f=field.frob(f);}assert_eq!(f,x);assert_eq!(field.frob(field.mul(x,y)),field.mul(field.frob(x),field.frob(y)));}
 println!("FIELD_CONTROL p={P} degree=13 modulus_constant={} irreducible=true seeded_cases=32 laws_inverse_frobenius=true",field.c);
 let curve=Curve{field,f:Pol::embed(&cp)};let ks=builder::cold_build_kernels(&curve,l,ell);let h0:HashSet<_>=ks[0].iter().cloned().collect();let h1:HashSet<_>=ks[1].iter().cloned().collect();assert_eq!(h0.intersection(&h1).count(),1);
 for (i,set)in ks.iter().enumerate(){let lookup:HashSet<_>=set.iter().cloned().collect();for x in set{assert!(curve.valid(x)&&curve.is_zero(&curve.small(x,3)));assert!(lookup.contains(&curve.frob(x)));for y in set{assert!(lookup.contains(&curve.add(x,y)),"kernel_closure");}}println!("KERNEL_CONTROL index={i} points=27 closure_checks=729 order3=true frobenius_stable=true");}
 let mut full=HashSet::new();for x in &ks[0]{for y in &ks[1]{let z=curve.add(x,y);assert!(curve.valid(&z)&&curve.is_zero(&curve.small(&z,3)));assert!(full.insert(z));}}assert_eq!(full.len(),729);
 let nonzero=ks[0].iter().find(|x|!curve.is_zero(x)).unwrap();let mut corrupt=nonzero.clone();*corrupt.u.0.last_mut().unwrap()=base(2);assert!(!curve.valid(&corrupt));
 let mut dump=format!("FIELD {P} 13 {}\n",field.c);for(i,set)in ks.iter().enumerate(){for(j,d)in set.iter().enumerate(){dump+=&format!("DIV {i} {j}");for p in [&d.u,&d.v]{dump+=&format!(" {}",p.0.len());for e in &p.0{for v in e{dump+=&format!(" {v}");}}}dump.push('\n');}}fs::write(out.join("KERNELS.txt"),dump).unwrap();
 println!("PASS constructed_F631_kernels=true primary_groups=2 each_points=27 full_3torsion_points=729 intersection_only_zero=true corrupted_divisor_rejected=true pairing_values_computed=false quotient_map_constructed=false DLP_or_timing_claim=false");}
