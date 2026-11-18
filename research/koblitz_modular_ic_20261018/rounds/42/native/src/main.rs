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
fn prime(n:u64)->bool{n>=2&&(2..).take_while(|d|d*d<=n).all(|d|n%d!=0)}
fn psmall(mut a:u64,mut e:u64,p:u64)->u64{let mut r=1;while e>0{if e&1!=0{r=r*a%p;}e>>=1;if e>0{a=a*a%p;}}r}
fn genus_trace(n:usize,t:i64)->BigInt{let(mut a,mut b)=(bi(2),bi(t));for _ in 1..n{let c=bi(t)*&b-bi(2)*&a;a=b;b=c;}b}
fn quadratic_power(m:&Modulus,n:usize,trace:i64)->[BigInt;2]{
 fn mul(m:&Modulus,a:&[BigInt;2],b:&[BigInt;2],trace:i64)->[BigInt;2]{
  let ab=m.mul(&a[1],&b[1]);
  [m.sub(&m.mul(&a[0],&b[0]),&m.mul(&bi(2),&ab)),
   m.add(&m.add(&m.mul(&a[0],&b[1]),&m.mul(&a[1],&b[0])),&m.mul(&bi(trace),&ab))]
 }
 let mut out=[bi(1),bi(0)];let mut base=[bi(0),bi(1)];let mut e=n;
 while e>0{if e&1!=0{out=mul(m,&out,&base,trace);}e>>=1;if e>0{base=mul(m,&base,&base,trace);}}out
}
fn root_equation(m:&Modulus,a:&BigInt,t:i64)->BigInt{m.add(&m.sub(&m.mul(a,a),&m.mul(&bi(t),a)),&bi(2))}
#[derive(Clone,Serialize)]struct Row{k:usize,cm_embedding:usize,alpha:String,generator_diamond:String,inverse_generator_diamond:String,diamond_two:String,frob_branch1:String,frob_branch2:String}
fn affine(m:&Modulus,label:&str,d1:&BigInt,d2:&BigInt,certs:&mut Vec<Cert>)->Value{
 let gap=d1-d2;
 let Some(c)=m.inv(label,&gap)else{return json!({"status":"nonunit_gap","gap":gap.to_string(),"gcd":gap.gcd(&m.r).to_string()})};
 let inverse:BigInt=c.inverse.parse().unwrap();certs.push(c);
 let p1=[m.neg(&m.mul(d2,&inverse)),inverse.clone()];
 let p2=[m.mul(d1,&inverse),m.neg(&inverse)];
 let eval=|p:&[BigInt;2],x:&BigInt|m.add(&p[0],&m.mul(&p[1],x));
 assert_eq!(eval(&p1,d1),bi(1));assert_eq!(eval(&p1,d2),bi(0));assert_eq!(eval(&p2,d1),bi(0));assert_eq!(eval(&p2,d2),bi(1));
 assert_eq!(m.add(&p1[0],&p2[0]),bi(1));assert_eq!(m.add(&p1[1],&p2[1]),bi(0));
 assert_ne!(m.mul(&gap,&m.add(&inverse,&bi(1))),bi(1));
 json!({"status":"unit_separation","lambda1":d1.to_string(),"lambda2":d2.to_string(),"gap":gap.to_string(),"inverse":inverse.to_string(),"branch1_projector_coefficients":[p1[0].to_string(),p1[1].to_string()],"branch2_projector_coefficients":[p2[0].to_string(),p2[1].to_string()],"sum_identity":true,"all_node_evaluations":true,"corrupt_inverse_rejected":true})
}
fn set_rows(rows:&[Row],branch:usize,diamond:usize)->BTreeSet<(BigInt,BigInt)>{
 rows.iter().map(|r|{
  let d=match diamond{0=>&r.generator_diamond,1=>&r.inverse_generator_diamond,_=>&r.diamond_two};
  let f=if branch==1{&r.frob_branch1}else{&r.frob_branch2};
  (d.parse().unwrap(),f.parse().unwrap())
 }).collect()
}
fn describe_set(v:&BTreeSet<(BigInt,BigInt)>)->Vec<Value>{v.iter().map(|(d,f)|json!({"diamond":d.to_string(),"frobenius":f.to_string()})).collect()}
fn verify_quadratic_bezout(m:&Modulus,u:&BigInt,v:&BigInt,h:&BigInt,den:&BigInt,sign:i64){
 let u2=m.mul(u,u);let v2=m.mul(v,v);
 let q1=[m.mul(&bi(2),&u2),m.mul(&bi(sign),u),bi(1)];
 let q2=[m.mul(&bi(2),&v2),m.mul(&bi(sign),v),bi(1)];
 let l1=[m.add(u,&m.mul(&bi(2),v)),bi(-sign)];
 let l2=[m.add(v,&m.mul(&bi(2),u)),bi(-sign)];
 let mut coefficients=vec![bi(0);4];
 for i in 0..2{for j in 0..3{coefficients[i+j]=m.add(&coefficients[i+j],&m.sub(&m.mul(&l1[i],&q2[j]),&m.mul(&l2[i],&q1[j])));}}
 assert_eq!(coefficients,vec![den.clone(),bi(0),bi(0),bi(0)]);
 let matrix=[[bi(1),q1[1].clone(),q1[0].clone(),bi(0)],[bi(0),bi(1),q1[1].clone(),q1[0].clone()],[bi(1),q2[1].clone(),q2[0].clone(),bi(0)],[bi(0),bi(1),q2[1].clone(),q2[0].clone()]];
 let mut determinant=bi(0);
 for a in 0..4{for b in 0..4{for c in 0..4{for d in 0..4{let permutation=[a,b,c,d];if permutation.iter().copied().collect::<BTreeSet<_>>().len()!=4{continue;}
  let mut inversions=0;for i in 0..4{for j in i+1..4{if permutation[i]>permutation[j]{inversions+=1;}}}
  let mut product=bi(1);for row in 0..4{product=m.mul(&product,&matrix[row][permutation[row]]);}
  determinant=if inversions%2==0{m.add(&determinant,&product)}else{m.sub(&determinant,&product)};
 }}}}
 let expected=m.mul(&bi(2),&m.mul(&m.pow(&m.sub(u,v),2),h));
 assert_eq!(determinant,expected);assert_eq!(determinant,m.mul(&m.sub(v,u),den));
}
fn case_run(case:&Case,certs:&mut Vec<Cert>)->Value{
 let r:BigInt=case.target_modulus.parse().unwrap();assert!(r>bi(3)&&r.is_odd());
 assert!(prime(case.n as u64));assert!(prime(case.conductor_prime));assert_ne!(case.n,7);
 let m=Modulus{r,muls:Cell::new(0)};let q=BigInt::one()<<case.n;
 let trace=genus_trace(case.n,case.target_trace);
 let (card,rem)=(q+bi(1)-&trace).div_rem(&bi(3-case.target_trace));assert!(rem.is_zero());
 assert_eq!(card.to_string(),case.retained_trace_zero_order);
 let (index,rem)=card.div_rem(&m.r);assert!(rem.is_zero());assert!(index>bi(0));
 let p=case.conductor_prime;let order=2*case.n;
 assert_eq!((p-1)%(order as u64),0);let conductor_index=(p-1)/(order as u64);assert_eq!(conductor_index%2,1);
 let g=(2..p).find(|&g|{let mut powers=BTreeSet::new();let mut x=1;for _ in 0..p-1{powers.insert(x);x=x*g%p;}x==1&&powers.len()==(p-1)as usize}).unwrap();
 let mut logs=BTreeMap::new();let mut x=1;for k in 0..p-1{assert!(logs.insert(x,k).is_none());x=x*g%p;}assert_eq!(x,1);
 for a in 1..p{assert_eq!(a*psmall(a,p-2,p)%p,1);}
 let finite_roots:Vec<_>=(0..p).filter(|&a|(a*a+p-a+2)%p==0).collect();assert_eq!(finite_roots.len(),2);
 let fa=finite_roots[0];let fb=(p+1-fa)%p;assert!(finite_roots.contains(&fb));assert_ne!(fa,fb);assert_eq!(fa*fb%p,2);
 let i=logs[&fa]as usize%order;let j=logs[&fb]as usize%order;let e2=logs[&2]as usize%order;
 assert_eq!((i+j)%order,e2);assert_eq!(((p-1)/2)as usize%order,case.n);
 let diamond_two_order=order/e2.gcd(&order);
 let generator=(0..7).map(|t|g+t*p).find(|a|a%7==1).unwrap();
 let delta=psmall(generator,case.n as u64,7*p);assert_eq!(delta%7,1);assert_eq!(logs[&(delta%p)]as usize%order,case.n);
 let wrong_delta=(e2*case.n)%order;let two_surrogate_matches=wrong_delta==case.n;
 if diamond_two_order==case.n{assert_eq!(wrong_delta,0);assert!(!two_surrogate_matches);}
 let power=quadratic_power(&m,case.n,case.target_trace);
 let linear_gcd=power[1].gcd(&m.r);
 let (unit_root,root_method)=if let Some(c)=m.inv(&format!("{}/unit-root-linear",case.id),&power[1]){
  let inverse:BigInt=c.inverse.parse().unwrap();certs.push(c);
  (m.mul(&m.sub(&bi(1),&power[0]),&inverse),"exact_linear_coefficient_inverse")
 }else{
  assert!(case.small_control&&m.r<=bi(10000),"large-target root extraction is not a unit");
  let end=m.r.to_u64().unwrap();
  let root=(0..end).map(|x|BigInt::from(x)).find(|a|root_equation(&m,a,case.target_trace).is_zero()&&m.pow(a,case.n)==bi(1)&&a!=&bi(1)).expect("fixed small-control root");
  (root,"fixed_small_control_exhaustion")
 };
 assert!(root_equation(&m,&unit_root,case.target_trace).is_zero());assert_eq!(m.pow(&unit_root,case.n),bi(1));assert_ne!(unit_root,bi(1));
 assert!(!root_equation(&m,&m.add(&unit_root,&bi(1)),case.target_trace).is_zero());
 let alpha=if case.target_trace==1{unit_root.clone()}else{m.neg(&unit_root)};
 let beta=m.sub(&bi(1),&alpha);assert!(root_equation(&m,&alpha,1).is_zero());assert!(root_equation(&m,&beta,1).is_zero());assert_ne!(alpha,beta);assert_eq!(m.mul(&alpha,&beta),bi(2));
 m.unit(&format!("{}/CM-discriminant",case.id),&(&alpha-&beta),certs);
 let zeta=m.neg(&unit_root);assert_eq!(m.pow(&zeta,case.n),m.neg(&bi(1)));assert_eq!(m.pow(&zeta,order),bi(1));assert_ne!(m.pow(&zeta,2),bi(1));
 let mut powers=vec![bi(1)];for k in 1..order{powers.push(m.mul(&powers[k-1],&zeta));}
 assert_eq!(powers.iter().cloned().collect::<BTreeSet<_>>().len(),order);
 assert_eq!(m.mul(powers.last().unwrap(),&zeta),bi(1));
 let alphas=[alpha,beta];let mut modes=vec![];
 for twisted in [false,true]{
  let mut rows=vec![];
  for k in 1..order{if k.gcd(&order)!=1{continue;}for (embedding,a) in alphas.iter().enumerate(){
   let d=&powers[k];let di=&powers[(order-k)%order];let d2=&powers[(k*e2)%order];
   assert_eq!(m.mul(d,di),bi(1));
   let b=m.sub(&bi(1),a);let mut f1=m.mul(a,&powers[(k*i)%order]);let mut f2=m.mul(&b,&powers[(k*j)%order]);
   if twisted{f1=m.neg(&f1);f2=m.neg(&f2);}
   assert_eq!(m.mul(&f1,&f2),m.mul(&bi(2),d2));
   rows.push(Row{k,cm_embedding:embedding,alpha:a.to_string(),generator_diamond:d.to_string(),inverse_generator_diamond:di.to_string(),diamond_two:d2.to_string(),frob_branch1:f1.to_string(),frob_branch2:f2.to_string()});
  }}
  assert_eq!(rows.len(),2*(case.n-1));
  let a=set_rows(&rows,1,0);let b=set_rows(&rows,2,0);let shared:BTreeSet<_>=a.intersection(&b).cloned().collect();
  let ai=set_rows(&rows,1,1);let b_inv=set_rows(&rows,2,1);let shared_inverse:BTreeSet<_>=ai.intersection(&b_inv).cloned().collect();assert_eq!(shared.len(),shared_inverse.len());
  let a2=set_rows(&rows,1,2);let b2=set_rows(&rows,2,2);let shared2:BTreeSet<_>=a2.intersection(&b2).cloned().collect();
  let fixed1:BTreeSet<BigInt>=a.iter().filter(|(_,f)|f==&bi(1)).map(|(d,_)|d.clone()).collect();
  let fixed2:BTreeSet<BigInt>=b.iter().filter(|(_,f)|f==&bi(1)).map(|(d,_)|d.clone()).collect();
  let rational_shared:BTreeSet<BigInt>=fixed1.intersection(&fixed2).cloned().collect();
  let separation=if !rational_shared.is_empty(){json!({"status":"shared_rational_system","witness_generator_values":rational_shared.iter().map(ToString::to_string).collect::<Vec<_>>(),"polynomial_separator_obstructed":true})}
   else if fixed1.len()==1&&fixed2.len()==1{
    let d1=fixed1.first().unwrap();let d2=fixed2.first().unwrap();
    let direct=affine(&m,&format!("{}/{twisted}/generator-gap",case.id),d1,d2,certs);
    let inv1=m.pow(d1,order-1);let inv2=m.pow(d2,order-1);assert_eq!(m.mul(d1,&inv1),bi(1));assert_eq!(m.mul(d2,&inv2),bi(1));
    let inverse=affine(&m,&format!("{}/{twisted}/inverse-generator-gap",case.id),&inv1,&inv2,certs);
    json!({"status":"two_disjoint_rational_lines","direct":direct,"inverse":inverse})
   }else{json!({"status":"single_empty_or_multiline","branch1_lines":fixed1.len(),"branch2_lines":fixed2.len()})};
  if case.n==5{assert_eq!(shared.len(),2);if twisted{assert_eq!(fixed1, BTreeSet::from([bi(7)]));assert_eq!(fixed2,BTreeSet::from([bi(8)]));assert!(rational_shared.is_empty());}else{assert_eq!(rational_shared,BTreeSet::from([bi(6)]));}}
  if case.id=="n11-conductor-control"&&!twisted{assert!(!rational_shared.is_empty());}
  modes.push(json!({"twisted":twisted,"rows":rows,"shared_generator_systems":describe_set(&shared),"shared_inverse_generator_systems":describe_set(&shared_inverse),"shared_diamond_two_systems":describe_set(&shared2),"fixed_branch1_generator_values":fixed1.iter().map(ToString::to_string).collect::<Vec<_>>(),"fixed_branch2_generator_values":fixed2.iter().map(ToString::to_string).collect::<Vec<_>>(),"rational_shared_generator_values":rational_shared.iter().map(ToString::to_string).collect::<Vec<_>>(),"separation":separation}));
 }
 // Equivalent denominator for Bezout splitting of both full quadratic branches.
 let mut global_denominators=vec![];let mut norm=bi(1);let mut obstructions=vec![];
 for k in 1..order{if k.gcd(&order)!=1{continue;}
  let d=&powers[k];let u=&powers[(k*i)%order];let v=&powers[(k*j)%order];
  let u2=m.mul(u,u);let v2=m.mul(v,v);let uv=m.mul(u,v);
  let h=m.add(&m.add(&m.mul(&bi(2),&u2),&m.mul(&bi(3),&uv)),&m.mul(&bi(2),&v2));
  let denominator=m.mul(&m.sub(v,u),&m.mul(&bi(2),&h));let gcd=denominator.gcd(&m.r);
  for sign in [-1i64,1]{verify_quadratic_bezout(&m,u,v,&h,&denominator,sign);}
  // Independent point controls in addition to the full coefficient identity.
  for sign in [-1i64,1]{for f in 0..4i64{
   let f=bi(f);let q1=m.add(&m.add(&m.mul(&f,&f),&m.mul(&bi(sign),&m.mul(u,&f))),&m.mul(&bi(2),&u2));
   let q2=m.add(&m.add(&m.mul(&f,&f),&m.mul(&bi(sign),&m.mul(v,&f))),&m.mul(&bi(2),&v2));
   let l1=m.sub(&m.add(u,&m.mul(&bi(2),v)),&m.mul(&bi(sign),&f));
   let l2=m.sub(&m.add(v,&m.mul(&bi(2),u)),&m.mul(&bi(sign),&f));
   assert_eq!(m.sub(&m.mul(&l1,&q2),&m.mul(&l2,&q1)),denominator);
  }}
  if gcd!=bi(1){obstructions.push(json!({"generator_diamond":d.to_string(),"denominator":denominator.to_string(),"gcd":gcd.to_string()}));}
  norm=m.mul(&norm,&denominator);
  global_denominators.push(json!({"k":k,"generator_diamond":d.to_string(),"denominator":denominator.to_string(),"gcd":gcd.to_string()}));
 }
 let norm_certificate=m.inv(&format!("{}/global-denominator-norm",case.id),&norm);if let Some(c)=&norm_certificate{certs.push(c.clone());assert!(obstructions.is_empty());}else{assert!(!obstructions.is_empty());}
 if case.n==5{assert!(!obstructions.is_empty());}
 println!("RESIDUAL42_CASE id={} n={} modulus_bits={} coefficient_rows_per_twist={} global_unit={} checked_modular_products={}",case.id,case.n,m.r.bits(),2*(case.n-1),norm_certificate.is_some(),m.muls.get());
 json!({"id":case.id,"n":case.n,"conductor_prime":p,"target_trace":case.target_trace,"target_modulus":case.target_modulus,"target_modulus_bits":m.r.bits(),"retained_target_primality_boundary":true,"trace_zero_order":card.to_string(),"target_index_in_order":index.to_string(),"integer_trace":trace.to_string(),"genus_historical":case.genus_historical,"finite_generator":g,"CRT_generator_diamond":generator,"finite_CM_roots":[fa,fb],"CM_exponents":[i,j],"diamond_two_exponent":e2,"diamond_two_order":diamond_two_order,"delta_CRT_representative":delta,"diamond_two_n_matches_delta":two_surrogate_matches,"unit_root":unit_root.to_string(),"unit_root_method":root_method,"root_linear_coefficient_gcd":linear_gcd.to_string(),"primitive_zeta":zeta.to_string(),"modes":modes,"global_denominators":global_denominators,"global_denominator_norm_mod_target":norm.to_string(),"global_denominator_norm_unit":norm_certificate.is_some(),"global_denominator_obstructions":obstructions,"checked_modular_products":m.muls.get()})
}
fn main(){
 let a:Vec<String>=env::args().collect();assert_eq!(a.len(),3);let cases:Vec<Case>=serde_json::from_str(&fs::read_to_string(&a[1]).unwrap()).unwrap();assert_eq!(cases.len(),6);
 let mut certs=vec![];let results:Vec<_>=cases.iter().map(|c|case_run(c,&mut certs)).collect();
 fs::create_dir(&a[2]).unwrap();
 fs::write(format!("{}/residual-systems.json",a[2]),serde_json::to_string_pretty(&json!({"schema":"koblitz.larger-residual/v1","cases":results,"primality_scope":"retained target assumptions; no new large-prime proof","actual_larger_n_geometric_transfer_evaluated":false})).unwrap()).unwrap();
 fs::write(format!("{}/unit-certificates.json",a[2]),serde_json::to_string_pretty(&certs).unwrap()).unwrap();
 println!("RESIDUAL42_RESULT fixed_cases=6 twist_modes=12 all_CM_and_primitive_character_embeddings=checked arbitrary_precision=true all_modular_products_independently_checked=true unit_certificates={} actual_larger_n_geometric_transfer_evaluated=false IC_relation_rank=unknown",certs.len());
 println!("RESIDUAL42_ALL_CHECKS_PASS");
}
