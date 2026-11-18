use hec_native_ic::{fp::Fp,instance::thread_curve,poly::Poly,jac::Curve};
fn gcd(mut a:u64,mut b:u64)->u64{while b!=0{(a,b)=(b,a%b);}a}
fn preserves(f:&Poly,z:u64,fp:&Fp)->bool{f.c[..f.n].iter().enumerate().all(|(i,&v)|fp.mul(v,fp.pow(z,i as u64))==v)}
fn main(){let input=include_str!("NATIVE-INSTANCES.tsv");let mut models=0;let mut primitive_available=0;println!("g\tp\tc\tnative_ell\tnative_coefficients\tnative_scalings\tbinomial_scalings\tprimitive_order_n_available\trejected_rotation_coefficient_witness");
 for line in input.lines().skip(1){let r:Vec<_>=line.split('\t').collect();let g:usize=r[0].parse().unwrap();let p:u64=r[1].parse().unwrap();let cc:u64=r[2].parse().unwrap();let ell:u64=r[3].parse().unwrap();let expected:Vec<u64>=r[4].split(',').map(|s|s.parse().unwrap()).collect();let curve=thread_curve(p,cc,g).unwrap();assert_eq!(&curve.f.c[..curve.f.n],expected.as_slice());assert!(curve.f.c[1..2*g+1].iter().any(|v|*v!=0));let n=(2*g+1) as u64;
 let native:Vec<_>=(1..p).filter(|&z|preserves(&curve.f,z,&curve.fp)).collect();assert_eq!(native,vec![1]);let mut bc=vec![0;n as usize+1];bc[0]=cc;bc[n as usize]=1;let binomial=Curve::new(Fp::new(p),Poly::from_coeffs(&bc));assert_ne!(curve.f,binomial.f);let bin:Vec<_>=(1..p).filter(|&z|preserves(&binomial.f,z,&binomial.fp)).collect();assert_eq!(bin.len() as u64,gcd(n,p-1));
 let primitive=bin.iter().copied().find(|&z|(1..n).all(|k|curve.fp.pow(z,k)!=1));if primitive.is_some(){primitive_available+=1;}
 let witness=if let Some(z)=bin.iter().copied().find(|z|*z!=1){let i=(0..curve.f.n).find(|&i|curve.fp.mul(curve.f.c[i],curve.fp.pow(z,i as u64))!=curve.f.c[i]).unwrap();format!("z={z},coefficient_degree={i},old={},scaled={}",curve.f.c[i],curve.fp.mul(curve.f.c[i],curve.fp.pow(z,i as u64)))}else{"none_in_field".to_owned()};
 println!("{g}\t{p}\t{cc}\t{ell}\t{}\t{}\t{}\t{}\t{witness}",r[4],native.len(),bin.len(),primitive.is_some());models+=1;
 }
 assert_eq!(models,11);eprintln!("PASS initial_native_models=11 all_initial_scaling_groups_trivial=true same_g_p_c_binomial_polynomials_distinct=true binomial_count_gcd_exact=true fields_with_primitive_binomial_rotation={primitive_available} no_claim_about_other_automorphisms=true no_dlp_or_speed_measurement=true");
}
