include!("../../hec_calibration631_20261115/common_fit.rs");
use hec_native_ic::{jac::Div,poly::Poly};
use std::{fs,path::Path};
fn transform(m:&[u64],x:[u64;8])->[u64;8]{std::array::from_fn(|i|(0..8).fold(0,|z,k|(z+m[8*i+k]*x[k])%631))}
fn store(p:&Path,x:&[u64]){fs::write(p,x.iter().map(u64::to_string).collect::<Vec<_>>().join(" ")+"\n").unwrap();}
fn main(){
 let a:Vec<_>=std::env::args().collect(); assert_eq!(a.len(),4);
 let calibration=Path::new(&a[1]);let matrix=Path::new(&a[2]);let out=Path::new(&a[3]);assert!(!out.exists());
 let ns:Vec<u64>=fs::read_to_string(calibration).unwrap().split_whitespace().map(|s|s.parse().unwrap()).collect();let mut at=0;let mut ps=vec![];for _ in 0..2{let n=ns[at]as usize;at+=1;ps.push(Poly::from_coeffs(&ns[at..at+n]));at+=n;}assert_eq!(ns.len(),at+6);let base=Div{u:ps[0],v:ps[1]};
 let m:Vec<u64>=fs::read_to_string(matrix).unwrap().split_whitespace().map(|s|s.parse().unwrap()).collect();assert_eq!(m.len(),64);assert!(m.iter().all(|&x|x<631));
 std::env::set_var("HEC_ANCHOR_I","0");std::env::set_var("HEC_ANCHOR_J","1");let src=source::Source::new();assert_eq!(src.mul_base(&base,59296481).u,Poly::one());fs::create_dir(out).unwrap();
 for(label,c)in[("D1",src.n(1)),("B",src.b())]{let mut chosen=None;for offset in 1..=4{let d=src.add(&c,&src.mul_base(&base,offset));let next=src.add(&d,&base);let Some(x)=src.eval(&d)else{println!("EXCLUDE {label} offset={offset} primary");continue;};let Some(y)=src.eval(&next)else{println!("EXCLUDE {label} offset={offset} adjacent");continue;};let x=transform(&m,x);let y=transform(&m,y);if x[0]==0{println!("EXCLUDE {label} offset={offset} h0_zero");continue;}assert!(y.iter().any(|&z|z!=0));chosen=Some((offset,x,y));break;}let(offset,x,y)=chosen.expect("fixed_translation_cap4");store(&out.join(format!("{label}-KUMMER.txt")),&x);store(&out.join(format!("{label}-NEXT-KUMMER.txt")),&y);store(&out.join(format!("{label}-OFFSET.txt")),&[offset]);println!("INPUT {label} offset={offset} primary={x:?} adjacent={y:?}");}
 println!("PASS public_input_coordinates=2 primary_and_adjacent=true actual_inverse_classes_or_global_map_verified=false");
}
