use std::{cell::Cell,collections::{BTreeMap,BTreeSet},env,fs};
use num_bigint::BigInt;
use num_integer::Integer;
use num_traits::{One,Zero,ToPrimitive,Signed};
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
fn lucas42(n:usize)->(BigInt,BigInt){
 let(mut s0,mut s1)=(bi(2),bi(1));let(mut u0,mut u1)=(bi(0),bi(1));
 for _ in 1..n{let s=&s1-bi(2)*&s0;let u=&u1-bi(2)*&u0;s0=s1;s1=s;u0=u1;u1=u;}(s1,u1)
}
fn lattice42(n:usize,i:usize,j:usize)->Value{
 let u=mono(i);let v=mono(j);let u2=imul(&u,&u);let v2=imul(&v,&v);
 let h=iadd(&iadd(&iscale(&u2,&bi(2)),&iscale(&imul(&u,&v),&bi(3))),&iscale(&v2,&bi(2)));
 let raw=iscale(&imul(&isub(&v,&u),&h),&bi(2));let(reduction_q,den)=divide_phi(&raw,n);let size=n-1;
 let mut matrix=vec![vec![bi(0);size];size];
 for col in 0..size{let(_,value)=divide_phi(&imul(&den,&mono(col)),n);for(row,x)in value.iter().enumerate(){matrix[row][col]=x.clone();}}
 let mut work=matrix.clone();for row in 0..size{work[row].push(bi(if row==0{1}else{0}));}
 let mut previous=bi(1);let mut swaps=0;let mut divisions=0u64;let mut pivots=vec![];
 for k in 0..size-1{
  let row=(k..size).find(|&row|!work[row][k].is_zero()).expect("singular denominator lattice");
  if row!=k{work.swap(row,k);swaps+=1;}
  let pivot=work[k][k].clone();assert!(!pivot.is_zero());
  for a in k+1..size{let entry=work[a][k].clone();for b in k+1..=size{
   let numerator=&pivot*&work[a][b]-&entry*&work[k][b];let(q,r)=numerator.div_rem(&previous);assert!(r.is_zero());work[a][b]=q;divisions+=1;
  }work[a][k]=bi(0);}
  pivots.push(json!({"step":k,"swapped_row":row,"pivot":pivot.to_string(),"previous":previous.to_string()}));previous=pivot;
 }
 let determinant=if swaps%2==0{work[size-1][size-1].clone()}else{-work[size-1][size-1].clone()};assert!(determinant>bi(0));
 let(s,ul)=lucas42(n);let difference=i.abs_diff(j);let gcd_difference=difference.gcd(&(2*n));
 assert!(gcd_difference==1||gcd_difference==2);
 let expected=(BigInt::one()<<(n-1))*if gcd_difference==2{BigInt::from(n)*&ul*&ul}else{&s*&s};
 assert_eq!(determinant,expected);
 let mut adj=vec![bi(0);size];
 for row in (0..size).rev(){let mut rhs=&determinant*&work[row][size];for col in row+1..size{rhs-=&work[row][col]*&adj[col];}let(q,r)=rhs.div_rem(&work[row][row]);assert!(r.is_zero());adj[row]=q;}
 for row in 0..size{let value:BigInt=matrix[row].iter().zip(&adj).map(|(a,b)|a*b).sum();assert_eq!(value,if row==0{determinant.clone()}else{bi(0)});}
 let common=adj.iter().fold(determinant.clone(),|g,a|g.gcd(a));let(d,rem)=determinant.div_rem(&common);assert!(rem.is_zero());
 let mut a=vec![];for x in &adj{let(v,r)=x.div_rem(&common);assert!(r.is_zero());a.push(v);}
 let mut g=d.clone();let mut weights=vec![bi(1)];
 for value in &a{
  if g==bi(1){weights.push(bi(0));continue;}
  let eg=g.extended_gcd(value);for w in &mut weights{*w*=&eg.x;}weights.push(eg.y);g=eg.gcd;
 }assert_eq!(g,bi(1));assert_eq!(weights.len(),size+1);
 let primitive=&weights[0]*&d+weights[1..].iter().zip(&a).map(|(w,a)|w*a).sum::<BigInt>();assert_eq!(primitive,bi(1));
 let lhs=isub(&imul(&den,&a),&[d.clone()]);let(q,r)=divide_phi(&lhs,n);assert!(r.is_empty());assert_eq!(lhs,imul(&phi(n),&q));
 let full_q=iadd(&q,&imul(&reduction_q,&a));assert_eq!(isub(&imul(&raw,&a),&[d.clone()]),imul(&phi(n),&full_q));
 let predicted=bi(2)*if gcd_difference==2{BigInt::from(n)*ul.abs()}else{s.abs()};
 println!("INTEGER42_LATTICE n={n} dimension={size} determinant_bits={} least_scalar={} predicted_formula_matches={} exact_divisions={divisions} primitive_witness=checked",determinant.bits(),d,d==predicted);
 json!({"n":n,"exponents":[i,j],"dimension":size,"matrix":matrix.iter().map(|r|strings(r)).collect::<Vec<_>>(),"bareiss_pivots":pivots,"exact_divisions":divisions,"determinant":determinant.to_string(),"closed_norm_formula":expected.to_string(),"trace_sequence":s.to_string(),"lucas_sequence":ul.to_string(),"difference_gcd":gcd_difference,"adjugate_column":strings(&adj),"common_gcd":common.to_string(),"least_scalar":d.to_string(),"primitive_inverse_polynomial":strings(&a),"primitive_weights":strings(&weights),"primitive_weighted_sum":"1","phi":strings(&phi(n)),"raw_denominator":strings(&raw),"reduced_denominator":strings(&den),"reduction_quotient":strings(&reduction_q),"scaled_inverse_phi_quotient":strings(&q),"full_phi_quotient":strings(&full_q),"predicted_least_scalar":predicted.to_string(),"predicted_formula_matches":d==predicted,"least_scalar_scope":"order of e0 in the denominator multiplication cokernel over the stated integral character basis"})
}
fn main(){
 let args:Vec<String>=env::args().collect();assert_eq!(args.len(),3);let input:Value=serde_json::from_str(&fs::read_to_string(&args[1]).unwrap()).unwrap();
 let cases=input["cases"].as_array().unwrap();assert_eq!(cases.len(),6);
 let mut cached:BTreeMap<(usize,usize,usize),Value>=BTreeMap::new();let mut result=vec![];let mut certs=vec![];
 for c in cases{
  let id=c["id"].as_str().unwrap();let n=c["n"].as_u64().unwrap()as usize;let i=c["CM_exponents"][0].as_u64().unwrap()as usize;let j=c["CM_exponents"][1].as_u64().unwrap()as usize;
  let data=cached.entry((n,i,j)).or_insert_with(||lattice42(n,i,j));
  let d:BigInt=data["least_scalar"].as_str().unwrap().parse().unwrap();let r:BigInt=c["target_modulus"].as_str().unwrap().parse().unwrap();let m=Modulus{r,muls:Cell::new(0)};
  let gcd=d.gcd(&m.r);let certificate=m.inv(&format!("{id}/least-scalar-unit"),&d);if let Some(c)=&certificate{certs.push(c.clone());}
  assert_eq!(certificate.is_some(),c["global_denominator_norm_unit"].as_bool().unwrap());
  result.push(json!({"id":id,"n":n,"target_modulus":m.r.to_string(),"least_scalar":d.to_string(),"target_gcd":gcd.to_string(),"invertible_at_target":certificate.is_some(),"lattice_key":format!("{n}/{i}/{j}")}));
 }
 assert_eq!(cached.len(),4);fs::create_dir(&args[2]).unwrap();
 fs::write(format!("{}/integer-recovery.json",args[2]),serde_json::to_string_pretty(&json!({"schema":"koblitz.integer-CM-denominator/v1","lattices":cached.values().collect::<Vec<_>>(),"cases":result,"scope":"exact denominator multiplication lattice; not full modular Hom lattice"})).unwrap()).unwrap();
 fs::write(format!("{}/least-scalar-unit-certificates.json",args[2]),serde_json::to_string_pretty(&certs).unwrap()).unwrap();
 println!("INTEGER42_RESULT distinct_integer_lattices=4 fixed_target_cases=6 exact_least_scalar_certificates=true original_matrix_columns=all Bareiss_divisions=exact independent_norm_formula=checked target_unit_cases={} full_modular_Hom_lattice_computed=false",certs.len());println!("INTEGER42_ALL_CHECKS_PASS");
}
