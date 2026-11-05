use std::{cell::Cell,collections::{BTreeMap,BTreeSet},env,fs};
use num_bigint::BigInt;
use num_integer::Integer;
use num_traits::{One,Zero,ToPrimitive};
use serde::{Deserialize,Serialize};
use serde_json::{json,Value};

#[derive(Deserialize)]struct Case{id:String,n:usize,conductor_prime:u64,target_trace:i64,target_modulus:String,retained_trace_zero_order:String,genus_historical:usize,small_control:bool}
#[derive(Clone,Serialize)]struct Cert{label:String,modulus:String,gap:String,inverse:String,bezout_k:String,gcd:String}
struct Modulus{r:BigInt,muls:Cell<u64>}
impl Modulus{
 fn val(&self,a:&BigInt)->BigInt{a.mod_floor(&self.r)}
 fn add(&self,a:&BigInt,b:&BigInt)->BigInt{self.val(&(a+b))}
 fn sub(&self,a:&BigInt,b:&BigInt)->BigInt{self.val(&(a-b))}
 fn neg(&self,a:&BigInt)->BigInt{self.val(&(-a))}
 fn mul(&self,a:&BigInt,b:&BigInt)->BigInt{
  let fast=self.val(&(a*b));let mut x=self.val(a);let mut y=self.val(b);let mut slow=BigInt::zero();
  while !y.is_zero(){if y.is_odd(){slow+=&x;if slow>=self.r{slow-=&self.r;}}y>>=1;if !y.is_zero(){x<<=1;if x>=self.r{x-=&self.r;}}}
  assert_eq!(fast,slow);self.muls.set(self.muls.get()+1);fast
 }
 fn pow(&self,a:&BigInt,mut n:usize)->BigInt{let mut r=BigInt::one();let mut b=self.val(a);while n>0{if n&1!=0{r=self.mul(&r,&b);}n>>=1;if n>0{b=self.mul(&b,&b);}}r}
 fn inv(&self,label:&str,a:&BigInt)->Option<Cert>{
  let eg=a.extended_gcd(&self.r);if eg.gcd!=BigInt::one(){return None;}
  let x=self.val(&eg.x);let (k,rem)=(BigInt::one()-a*&x).div_rem(&self.r);assert!(rem.is_zero());
  assert_eq!(a*&x+&self.r*&k,BigInt::one());assert_eq!(self.mul(a,&x),BigInt::one());
  Some(Cert{label:label.to_string(),modulus:self.r.to_string(),gap:a.to_string(),inverse:x.to_string(),bezout_k:k.to_string(),gcd:"1".to_string()})
 }
 fn unit(&self,label:&str,a:&BigInt,certs:&mut Vec<Cert>)->BigInt{let c=self.inv(label,a).expect("required exact unit");let x=c.inverse.parse().unwrap();certs.push(c);x}
}
fn bi(n:i64)->BigInt{n.into()}
type IntPoly=Vec<BigInt>;
fn itrim(mut a:IntPoly)->IntPoly{while a.last().is_some_and(Zero::is_zero){a.pop();}a}
fn iadd(a:&[BigInt],b:&[BigInt])->IntPoly{let mut c=vec![bi(0);a.len().max(b.len())];for(i,x)in a.iter().enumerate(){c[i]+=x;}for(i,x)in b.iter().enumerate(){c[i]+=x;}itrim(c)}
fn iscale(a:&[BigInt],s:&BigInt)->IntPoly{itrim(a.iter().map(|x|x*s).collect())}
fn isub(a:&[BigInt],b:&[BigInt])->IntPoly{iadd(a,&iscale(b,&bi(-1)))}
fn imul(a:&[BigInt],b:&[BigInt])->IntPoly{
 if a.is_empty()||b.is_empty(){return vec![];}let mut c=vec![bi(0);a.len()+b.len()-1];
 for(i,x)in a.iter().enumerate(){if x.is_zero(){continue;}for(j,y)in b.iter().enumerate(){if !y.is_zero(){c[i+j]+=x*y;}}}itrim(c)
}
fn mono(e:usize)->IntPoly{let mut a=vec![bi(0);e+1];a[e]=bi(1);a}
fn phi(n:usize)->IntPoly{(0..n).map(|i|bi(if i%2==0{1}else{-1})).collect()}
fn divide_phi(a:&[BigInt],n:usize)->(IntPoly,IntPoly){
 let p=phi(n);let mut r=itrim(a.to_vec());let mut q=vec![bi(0);a.len().saturating_sub(n-1)];
 while r.len()>=n{let e=r.len()-n;let leading=r.last().unwrap().clone();q[e]+=&leading;for i in 0..n{r[e+i]-=&leading*&p[i];}r=itrim(r);}
 let q=itrim(q);assert_eq!(iadd(&imul(&p,&q),&r),itrim(a.to_vec()));(q,r)
}
fn eval_poly(m:&Modulus,p:&[BigInt],t:&BigInt)->BigInt{p.iter().rev().fold(bi(0),|v,c|m.add(&m.mul(&v,t),c))}
fn field_phi(a:&[BigInt],m:&Modulus,n:usize)->IntPoly{let(_,r)=divide_phi(a,n);itrim(r.iter().map(|x|m.val(x)).collect())}
fn strings(a:&[BigInt])->Vec<String>{a.iter().map(ToString::to_string).collect()}
fn fmul(a:&[IntPoly],b:&[IntPoly])->Vec<IntPoly>{
 let mut c=vec![vec![];a.len()+b.len()-1];for(i,x)in a.iter().enumerate(){for(j,y)in b.iter().enumerate(){c[i+j]=iadd(&c[i+j],&imul(x,y));}}c
}
fn dot_phi_inverse(m:&Modulus,n:usize,rho:&BigInt,den:&[BigInt],label:&str,certs:&mut Vec<Cert>)->IntPoly{
 let order=2*n;let mut powers=vec![bi(1)];for k in 1..order{powers.push(m.mul(&powers[k-1],rho));}
 assert_eq!(powers[n],m.neg(&bi(1)));assert_eq!(m.mul(powers.last().unwrap(),rho),bi(1));
 let mut values=vec![];let mut nodes=vec![];
 for k in 0..n{
  let d=powers[(1+2*k)%order].clone();
  if d==m.neg(&bi(1)){values.push(bi(0));}else{let v=eval_poly(m,den,&d);values.push(m.unit(&format!("{label}/node-{k}"),&v,certs));}
  nodes.push(d);
 }
 let ni=m.unit(&format!("{label}/DFT-size"),&BigInt::from(n),certs);
 // Direct finite Fourier inverse on all n roots of X^n=-1; the -1 node is assigned 0.
 let mut coeffs=vec![];
 for e in 0..n{let mut sum=bi(0);for k in 0..n{
  let exponent=(order-(2*((k*e)%n))%order)%order;
  sum=m.add(&sum,&m.mul(&values[k],&powers[exponent]));
 }
 let scale=m.mul(&ni,&powers[(order-e%order)%order]);coeffs.push(m.mul(&scale,&sum));}
 // Verify interpolation itself at every node before reducing to Phi_2n.
 for(k,d)in nodes.iter().enumerate(){assert_eq!(eval_poly(m,&coeffs,d),values[k]);}
 let result=field_phi(&coeffs,m,n);
 for(k,d)in nodes.iter().enumerate(){if d!=&m.neg(&bi(1)){assert_eq!(eval_poly(m,&result,d),values[k]);}}
 let product=field_phi(&imul(den,&result),m,n);assert_eq!(product,vec![bi(1)]);result
}
fn main(){
 let args:Vec<String>=env::args().collect();assert_eq!(args.len(),3);
 let input:Value=serde_json::from_str(&fs::read_to_string(&args[1]).unwrap()).unwrap();let cases=input["cases"].as_array().unwrap();assert_eq!(cases.len(),6);
 let mut results=vec![];let mut certs=vec![];let mut certified=0;let mut obstructed=0;
 for c in cases{
  let id=c["id"].as_str().unwrap();let n=c["n"].as_u64().unwrap()as usize;
  let modulus:BigInt=c["target_modulus"].as_str().unwrap().parse().unwrap();let m=Modulus{r:modulus,muls:Cell::new(0)};
  let i=c["CM_exponents"][0].as_u64().unwrap()as usize;let j=c["CM_exponents"][1].as_u64().unwrap()as usize;
  let u=mono(i);let v=mono(j);let u2=imul(&u,&u);let v2=imul(&v,&v);
  let h=iadd(&iadd(&iscale(&u2,&bi(2)),&iscale(&imul(&u,&v),&bi(3))),&iscale(&v2,&bi(2)));
  let denominator=iscale(&imul(&isub(&v,&u),&h),&bi(2));
  let(reduction_quotient,reduced)=divide_phi(&denominator,n);
  let rho:BigInt=c["primitive_zeta"].as_str().unwrap().parse().unwrap();
  let mut norm=bi(1);let mut witnesses=vec![];
  for k in 1..2*n{if k.gcd(&(2*n))!=1{continue;}let d=m.pow(&rho,k);assert!(eval_poly(&m,&phi(n),&d).is_zero());let den=eval_poly(&m,&reduced,&d);norm=m.mul(&norm,&den);
   let g=den.gcd(&m.r);if g!=bi(1){witnesses.push(json!({"diamond":d.to_string(),"denominator":den.to_string(),"gcd":g.to_string()}));}
  }
  assert_eq!(norm.to_string(),c["global_denominator_norm_mod_target"].as_str().unwrap());
  let expected=c["global_denominator_norm_unit"].as_bool().unwrap();
  if !expected{
   assert!(!witnesses.is_empty());obstructed+=1;
   results.push(json!({"id":id,"n":n,"modulus":m.r.to_string(),"status":"obstructed","phi":strings(&phi(n)),"raw_denominator":strings(&denominator),"reduced_denominator":strings(&reduced),"reduction_quotient":strings(&reduction_quotient),"obstructions":witnesses,"checked_modular_products":m.muls.get()}));
   println!("GLOBAL42_CASE id={id} status=obstructed nonunit_nodes={} actual_modulus_preserved=true",witnesses.len());continue;
  }
  assert!(witnesses.is_empty());let inverse=dot_phi_inverse(&m,n,&rho,&reduced,id,&mut certs);
  let lhs=isub(&imul(&reduced,&inverse),&[bi(1)]);let(q,rem)=divide_phi(&lhs,n);
  let mut multiples=vec![];
  for x in &rem{let(v,r)=x.div_rem(&m.r);assert!(r.is_zero());multiples.push(v);}
  multiples=itrim(multiples);
  let rebuilt=iadd(&imul(&phi(n),&q),&iscale(&multiples,&m.r));assert_eq!(lhs,rebuilt);
  // Independent integer-lift identity using the unreduced genuine CM annihilators.
  let full_phi_quotient=iadd(&q,&imul(&reduction_quotient,&inverse));
  assert_eq!(isub(&imul(&denominator,&inverse),&[bi(1)]),iadd(&imul(&phi(n),&full_phi_quotient),&iscale(&multiples,&m.r)));
  let mut operators=vec![];
  for sign in [-1i64,1]{
   let q1=vec![iscale(&u2,&bi(2)),iscale(&u,&bi(sign)),vec![bi(1)]];
   let q2=vec![iscale(&v2,&bi(2)),iscale(&v,&bi(sign)),vec![bi(1)]];
   let b1=vec![imul(&iadd(&u,&iscale(&v,&bi(2))),&inverse),iscale(&inverse,&bi(-sign))];
   let b2=vec![iscale(&imul(&iadd(&v,&iscale(&u,&bi(2))),&inverse),&bi(-1)),iscale(&inverse,&bi(sign))];
   let e1=fmul(&b1,&q2);let e2=fmul(&b2,&q1);assert_eq!(e1.len(),4);assert_eq!(e2.len(),4);
   for f in 0..4{
    let total=iadd(&e1[f],&e2[f]);let expected=if f==0{imul(&denominator,&inverse)}else{vec![]};assert_eq!(total,expected);
    let desired=if f==0{vec![bi(1)]}else{vec![]};assert_eq!(field_phi(&total,&m,n),desired);
   }
   // Full coefficient certificates for reductions modulo each genuine quadratic.
   // e1+e2=1 and each e_i is literally a multiple of the opposite q_i.
   operators.push(json!({"quadratic_sign":sign,"q1":q1.iter().map(|p|strings(p)).collect::<Vec<_>>(),"q2":q2.iter().map(|p|strings(p)).collect::<Vec<_>>(),"B1":b1.iter().map(|p|strings(p)).collect::<Vec<_>>(),"B2":b2.iter().map(|p|strings(p)).collect::<Vec<_>>(),"operator1":e1.iter().map(|p|strings(p)).collect::<Vec<_>>(),"operator2":e2.iter().map(|p|strings(p)).collect::<Vec<_>>(),"sum_identity_mod_r_Phi":true,"opposite_annihilator_factor_preserved_over_Z":true}));
  }
  certified+=1;
  results.push(json!({"id":id,"n":n,"modulus":m.r.to_string(),"status":"certified","phi":strings(&phi(n)),"raw_denominator":strings(&denominator),"reduced_denominator":strings(&reduced),"reduction_quotient":strings(&reduction_quotient),"inverse_denominator":strings(&inverse),"inverse_phi_quotient":strings(&q),"inverse_r_multiple":strings(&multiples),"full_phi_quotient":strings(&full_phi_quotient),"operators":operators,"integer_coefficient_identities_checked":true,"all_DFT_nodes_checked":true,"actual_geometric_transfer_evaluated":false,"checked_modular_products":m.muls.get()}));
  println!("GLOBAL42_CASE id={id} status=certified n={n} modulus_bits={} inverse_degree={} full_integer_identity=true opposite_annihilators_preserved=true checked_modular_products={}",m.r.bits(),inverse.len().saturating_sub(1),m.muls.get());
 }
 assert_eq!(certified+obstructed,6);assert_eq!(certified,3);assert_eq!(obstructed,3);
 fs::create_dir(&args[2]).unwrap();fs::write(format!("{}/global-recovery.json",args[2]),serde_json::to_string_pretty(&json!({"schema":"koblitz.global-CM-recovery/v1","cases":results,"certified":certified,"obstructed":obstructed,"scope":"CM block polynomial model; not whole modular Jacobian or actual point transfer"})).unwrap()).unwrap();
 fs::write(format!("{}/DFT-unit-certificates.json",args[2]),serde_json::to_string_pretty(&certs).unwrap()).unwrap();
 println!("GLOBAL42_RESULT fixed_cases=6 certified={certified} obstruction_controls={obstructed} twist_signs_each=2 exact_integer_lift_certificates=true normalized_global_CM_block_recovery=true actual_larger_n_geometric_transfer_evaluated=false");println!("GLOBAL42_ALL_CHECKS_PASS");
}
