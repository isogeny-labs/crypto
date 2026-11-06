#[derive(Clone,Debug)]
struct Jet29 { a:[B;3] }
fn jz29()->Jet29 {Jet29{a:std::array::from_fn(|_|vec![])}}
fn jo29()->Jet29 {let mut j=jz29();j.a[0]=one();j}
fn ja29(a:&Jet29,b:&Jet29)->Jet29 {Jet29{a:std::array::from_fn(|i|add(&a.a[i],&b.a[i]))}}
fn jm29(a:&Jet29,b:&Jet29,n:usize)->Jet29 {
 let mut result=jz29();for k in 0..3{for i in 0..=k{result.a[k]=add(&result.a[k],&mul(&a.a[i],&b.a[k-i],n));}}result
}
fn ji29(a:&Jet29,n:usize)->Jet29 {
 let fi=inv(&a.a[0],n);let fi2=sq(&fi,n);let fi3=mul(&fi2,&fi,n);
 let r=Jet29{a:[fi,mul(&a.a[1],&fi2,n),add(&mul(&a.a[2],&fi2,n),&mul(&sq(&a.a[1],n),&fi3,n))]};
 let p=jm29(a,&r,n);assert_eq!(p.a[0],one());assert!(p.a[1].is_empty()&&p.a[2].is_empty());r
}
fn jnorm11_29(p:&Jet29,n:usize)->Jet29 {
 let mut a=vec![vec![jz29();11];11];
 for j in 0..11{for component in 0..3{each(&p.a[component],|k|{let r=(k+j)%11;let e=(k+j)/11;if e<n{set(&mut a[r][j].a[component],e);}});}}
 let mut det=jo29();
 for j in 0..11{let pivot=a[j][j].clone();assert!(bit(&pivot.a[0],0));let pi=ji29(&pivot,n);det=jm29(&det,&pivot,n);
  for i in j+1..11 {if a[i][j].a.iter().all(Vec::is_empty){continue;}let m=jm29(&a[i][j],&pi,n);
   for k in j+1..11{a[i][k]=ja29(&a[i][k],&jm29(&m,&a[j][k],n));}a[i][j]=jz29();}
 }det
}
fn jnorm121_29(p:&Jet29,n:usize)->Jet29 {jnorm11_29(&jnorm11_29(p,11*n),n)}
fn mid29(u:&B,v:&B,weight:usize,n:usize)->Jet29 {
 let field=Field::find(10);let zeta=(2..1024).map(|a|field.pow(a,93)).find(|a|*a!=1).unwrap();assert_eq!(field.pow(zeta,11),1);
 let base=(0..n).map(|k|u32::from(bit(v,k))).collect::<Vec<_>>();let mut output=vec![vec![0;n];3];output[0][0]=1;
 for b in 1..11{
  let tv=(0..n).map(|k|if bit(v,k){field.pow(zeta,(b*((k+19)%11)%11)as u64)}else{0}).collect::<Vec<_>>();
  let mut ff=field_mul(field,&base,&tv,n);ff[6]^=1;let mut gg=vec![0;n];
  if weight!=1{gg[6]^=1;}
  if weight!=0{for k in 0..n-4{if bit(u,k){gg[k+4]^=field.pow(zeta,(b*((k+20)%11)%11)as u64);}}}
  let parts=[ff,gg,vec![0;n]];let mut next=vec![vec![0;n];3];
  for k in 0..3{for i in 0..=k{let product=field_mul(field,&output[i],&parts[k-i],n);for(j,c)in product.into_iter().enumerate(){next[k][j]^=c;}}}output=next;
 }
 Jet29{a:std::array::from_fn(|component|{let mut bits=vec![];for(k,c)in output[component].iter().enumerate(){assert!(*c<=1);if *c==1{set(&mut bits,k);}}bits})}
}
fn full29(u:&B,v:&B,ub:&B,vb:&B,weight:usize,n:usize)->Jet29 {
 let big=121*n;
 let lf=add(&mul(v,&dilate(v,121,n),n),&shift(&one(),366,n));
 let sf=add(&mul(vb,&dilate(vb,121,big),big),&shift(&one(),366,big));
 let mut lg=vec![];let mut sg=vec![];
 if weight!=1{lg=shift(&one(),366,n);sg=shift(&one(),366,big);}
 if weight!=0{lg=add(&lg,&shift(&dilate(u,121,n),124,n));sg=add(&sg,&shift(ub,364,big));}
 let small=jnorm121_29(&Jet29{a:[sf,sg,vec![]]},n);let middle=mid29(u,v,weight,n);
 jm29(&jm29(&small,&middle,n),&Jet29{a:[lf,lg,vec![]]},n)
}
fn main(){
 let args=std::env::args().collect::<Vec<_>>();assert_eq!(args.len(),4,"output,M15inputs,R28derivatives");let out=Path::new(&args[1]);let original_dir=Path::new(&args[2]);let derivative_dir=Path::new(&args[3]);let n=1024;
 let a=Jet29{a:[vec![3],one(),vec![]]};let p=jnorm121_29(&a,32);assert_eq!(p.a[0],vec![3]);assert_eq!(p.a[1],one());assert!(p.a[2].is_empty());
 let a=Jet29{a:[vec![3],vec![],one()]};let p=jnorm121_29(&a,32);assert_eq!(p.a[0],vec![3]);assert!(p.a[1].is_empty());assert_eq!(p.a[2],one());
 let a=Jet29{a:[vec![3],vec![5],vec![7]]};let b=Jet29{a:[vec![7],vec![11],vec![13]]};
 let p=jnorm121_29(&jm29(&a,&b,121*32),32);let q=jm29(&jnorm121_29(&a,32),&jnorm121_29(&b,32),32);for i in 0..3{assert_eq!(p.a[i],q.a[i]);}
 println!("JET29_SECOND_ORDER_NORM_CONTROLS_PASS");
 let original:serde_json::Value=serde_json::from_slice(&std::fs::read(original_dir.join("m15-function-1.json")).unwrap()).unwrap();
 let d1:serde_json::Value=serde_json::from_slice(&std::fs::read(derivative_dir.join("dual28-function-1-one.json")).unwrap()).unwrap();
 let dx:serde_json::Value=serde_json::from_slice(&std::fs::read(derivative_dir.join("dual28-function-1-xinf.json")).unwrap()).unwrap();
 let(u,v)=xy(n);let(ub,vb)=xy(121*n);let mut records=vec![];
 for weight in 0..3{
  let result=full29(&u,&v,&ub,&vb,weight,n);let base=recover(&result.a[0],792,&u,&v,n);let first=recover(&result.a[1],792,&u,&v,n);let second=recover(&result.a[2],792,&u,&v,n);
  assert_eq!(base["A_bits"],original["A_bits"]);assert_eq!(base["B_bits"],original["B_bits"]);
  for key in ["A_bits","B_bits"]{
   let expected=if weight==0{d1[key].as_str().unwrap().to_owned()}else if weight==1{dx[key].as_str().unwrap().to_owned()}else{hex(&add(&bits28(d1[key].as_str().unwrap()),&bits28(dx[key].as_str().unwrap())))};
   assert_eq!(first[key].as_str().unwrap(),expected);
  }
  save28(out,&format!("jet29-weight-{weight}.json"),&second);records.push(second);
  println!("JET29_RECOVERED_SECOND_COEFFICIENT weight={weight} all1024coefficients");
 }
 save28(out,"jet29-results.json",&json!({"schema":"koblitz.round29/second-adjugate-v1","records":records,"precision":n,"pole_bound":792,"original_norm_exactly_matched":true,"all_first_derivatives_exactly_match_R28":true,"pair_values_selected":false,"geometry_bridge":"written modularity/poles/RR and epsilon2 factor specialization"}));
 println!("JET29_ALL_CHECKS_PASS");
}
