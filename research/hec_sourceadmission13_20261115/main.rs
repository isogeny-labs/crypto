use hec_native_ic::{fp::Fp,poly::Poly,jac::{Curve,Div},instance::{factor_base,exact_order_from_multiple,prime_factors}};
use std::{collections::HashMap,fs,path::Path};
fn sqrt_floor(n:u64)->u64{let(mut a,mut b)=(0,n+1);while b-a>1{let m=(a+b)/2;if m<=n/m{a=m}else{b=m}}a}
fn multiple(c:&Curve,d:&Div,lo:u64,hi:u64)->Option<u64>{
 let width=hi-lo;let step=sqrt_floor(width)+1;assert!(step<=50000,"baby_cap");
 let mut tab=HashMap::with_capacity(step as usize);let mut x=c.zero();let neg=c.neg(d);
 for j in 0..step{tab.entry(x).or_insert(j);x=c.add(&x,&neg);}
 let jump=c.mul(d,step);let mut x=c.mul(d,lo);
 for i in 0..=width/step+1{if let Some(&j)=tab.get(&x){let n=lo+i*step+j;if n<=hi&&!c.is_zero(d)&&c.is_zero(&c.mul(d,n)){return Some(n);}}x=c.add(&x,&jump);}
 None
}
fn serialize(p:u64,ell:u64,c:&Curve,d:&Div,b:&Div)->String{let mut v=vec![3,p,ell];for a in[c.f,d.u,d.v,b.u,b.v]{v.push(a.n as u64);v.extend_from_slice(&a.c[..a.n]);}format!("COLD_INPUT {}\n",v.iter().map(u64::to_string).collect::<Vec<_>>().join(" "))}
fn main(){let a:Vec<_>=std::env::args().collect();let root=Path::new(&a[1]);assert!(!root.exists());fs::create_dir(root).unwrap();let mut admitted=0;let mut rejected=0;
 for p in [127,1039,2011,4003]{assert!(p>3&&(2..).take_while(|d|d*d<=p).all(|d|p%d!=0));assert_eq!(p%12,7);
 for constant in 31..=34{
 let coeff=[constant,89,73,48,91,41,123,1];let fp=Fp::new(p);let f=Poly::from_coeffs(&coeff);
 if Poly::gcd(&f,&f.deriv(&fp),&fp).deg()!=0{println!("REJECT p={p} c={constant} reason=singular");rejected+=1;continue;}
 let c=Curve::new(fp,f);let n1=(p as i64+1)+(0..p).map(|x|{let a=f.eval(x,&fp);if a==0{0}else if fp.pow(a,(p-1)/2)==1{1}else{-1}}).sum::<i64>();
 let a1=n1-p as i64-1;let ceil=sqrt_floor(p)+1;
 let base=p as i128*p as i128*p as i128+1+(p as i128*p as i128+1)*a1 as i128;
 let radius=15*p as i128*(p+1)as i128+20*p as i128*ceil as i128;
 assert!(base>radius);let(lo,hi)=((base-radius)as u64,(base+radius)as u64);let width=hi-lo;
 let fb=factor_base(&c);let mut answer=None;
 for (attempt,&(x,y))in fb.iter().take(4).enumerate(){let e=c.place(x,y);let Some(m)=multiple(&c,&e,lo,hi)else{println!("WITNESS p={p} c={constant} attempt={attempt} bounded_unknown=no_multiple");continue;};let ord=exact_order_from_multiple(&c,&e,m);let ell=*prime_factors(ord).last().unwrap_or(&1);
 if ell<1000{println!("WITNESS p={p} c={constant} attempt={attempt} order={ord} largest_prime={ell} reject=small_prime");continue;}
 let d=c.mul(&e,ord/ell);assert!(!c.is_zero(&d)&&c.is_valid(&d)&&c.is_zero(&c.mul(&d,ell)));let first=lo.div_ceil(ell);let last=hi/ell;
 let dim2=(ceil+1).pow(4);
 if first!=last||ell<=dim2{println!("WITNESS p={p} c={constant} attempt={attempt} order={ord} largest_prime={ell} first_multiple={first} last_multiple={last} reject=ambiguous_or_factor_bound");continue;}
 let n=first*ell;assert!(lo<=n&&n<=hi&&n%ell==0);assert!((ell as u128)*(ell as u128)>n as u128);
 let b=fb.iter().skip(4).take(32).map(|&(x,y)|c.mul(&c.place(x,y),n/ell)).find(|b|!c.is_zero(b));let Some(b)=b else{println!("WITNESS p={p} c={constant} attempt={attempt} reject=no_independent_target_first32");continue;};assert!(c.is_valid(&b)&&c.is_zero(&c.mul(&b,ell)));
 let case=format!("p{p}-c{constant}");fs::write(root.join(format!("{case}-PUBLIC.txt")),serialize(p,ell,&c,&d,&b)).unwrap();
 // Independently verify the certificate on up to eight fresh degree-one classes.
 for &(x,y)in fb.iter().take(8){assert!(c.is_zero(&c.mul(&c.place(x,y),n)),"order_annihilation_control");}
 fs::write(root.join(format!("{case}-CERT.txt")),format!("p={p}\nf={coeff:?}\nN1={n1}\na1={a1}\nlo={lo}\nhi={hi}\nell={ell}\nunique_multiple={n}\nsource_order={ord}\nproper_factor_bound={dim2}\nknown_target_scalar_supplied=false\n")).unwrap();
 answer=Some((ell,n,ord,attempt));break;
 }
 if let Some((ell,n,ord,attempt))=answer{admitted+=1;println!("ADMIT p={p} c={constant} N1={n1} a1={a1} interval={lo},{hi} width={width} baby_steps={} ell={ell} N={n} witness_order={ord} attempt={attempt} Fp_simple_by_order=true target_generated_without_scalar=true",sqrt_floor(width)+1);}else{rejected+=1;println!("BOUNDED_UNKNOWN p={p} c={constant} N1={n1} interval={lo},{hi} attempted_places=4 no_admissible_certificate=true");}
 }}println!("COHORT_COMPLETE cases=16 admitted={admitted} rejected_or_bounded_unknown={rejected} no_DLP_or_timing_executed=true");}
