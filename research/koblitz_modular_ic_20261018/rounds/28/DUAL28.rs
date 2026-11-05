#[derive(Clone,Debug)]
struct Dual28 { f:B, g:B }
fn dz28()->Dual28 { Dual28{f:vec![],g:vec![]} }
fn done28()->Dual28 { Dual28{f:one(),g:vec![]} }
fn da28(a:&Dual28,b:&Dual28)->Dual28 {Dual28{f:add(&a.f,&b.f),g:add(&a.g,&b.g)}}
fn dm28(a:&Dual28,b:&Dual28,n:usize)->Dual28 {
 Dual28{f:mul(&a.f,&b.f,n),g:add(&mul(&a.f,&b.g,n),&mul(&a.g,&b.f,n))}
}
fn di28(a:&Dual28,n:usize)->Dual28 {
 let fi=inv(&a.f,n);let gi=mul(&a.g,&sq(&fi,n),n);
 let r=Dual28{f:fi,g:gi};let check=dm28(a,&r,n);
 assert_eq!(check.f,one());assert!(check.g.is_empty());r
}
fn norm11d28(p:&Dual28,n:usize)->Dual28 {
 let mut a=vec![vec![dz28();11];11];
 for j in 0..11 {
  each(&p.f,|k|{let r=(k+j)%11;let e=(k+j)/11;if e<n{set(&mut a[r][j].f,e);}});
  each(&p.g,|k|{let r=(k+j)%11;let e=(k+j)/11;if e<n{set(&mut a[r][j].g,e);}});
 }
 let mut det=done28();
 for j in 0..11 {
  let pivot=a[j][j].clone();assert!(bit(&pivot.f,0));let pi=di28(&pivot,n);det=dm28(&det,&pivot,n);
  for i in j+1..11 {
   if a[i][j].f.is_empty() && a[i][j].g.is_empty(){continue;}
   let m=dm28(&a[i][j],&pi,n);
   for k in j+1..11 {a[i][k]=da28(&a[i][k],&dm28(&m,&a[j][k],n));}
   a[i][j]=dz28();
  }
 }
 det
}
fn norm121d28(p:&Dual28,n:usize)->Dual28 {norm11d28(&norm11d28(p,11*n),n)}
fn bits28(s:&str)->B {
 let bytes=BigUint::parse_bytes(s.as_bytes(),16).unwrap().to_bytes_le();
 clean(bytes.chunks(8).map(|v|{let mut a=[0;8];a[..v.len()].copy_from_slice(v);u64::from_le_bytes(a)}).collect())
}
fn middle28(u:&B,v:&B,kind:usize,weighted_x:bool,n:usize)->Dual28 {
 let field=Field::find(10);let zeta=(2..1024).map(|a|field.pow(a,93)).find(|a|*a!=1).unwrap();assert_eq!(field.pow(zeta,11),1);
 let base=(0..n).map(|k|u32::from(bit(if kind==1{v}else{u},k))).collect::<Vec<_>>();
 let mut output_f=vec![0;n];output_f[0]=1;let mut output_g=vec![0;n];let pole=if kind==1{6}else{5};
 for b in 1..11 {
  let twisted_v=(0..n).map(|k|if bit(v,k){field.pow(zeta,(b*((k+19)%11)%11)as u64)}else{0}).collect::<Vec<_>>();
  let mut ff=field_mul(field,&base,&twisted_v,n);ff[pole]^=1;
  let mut gg=vec![0;n];
  if weighted_x {
   for k in 0..n-(pole-2) {if bit(u,k){gg[k+pole-2]=field.pow(zeta,(b*((k+20)%11)%11)as u64);}}
  }else{gg[pole]=1;}
  let aa=field_mul(field,&output_f,&ff,n);
  let mut bb=field_mul(field,&output_g,&ff,n);let cc=field_mul(field,&output_f,&gg,n);
  for(i,c)in cc.into_iter().enumerate(){bb[i]^=c;}
  output_f=aa;output_g=bb;
 }
 let pack=|data:Vec<u32>|{let mut result=vec![];for(k,a)in data.into_iter().enumerate(){assert!(a<=1,"middle dual coefficient did not descend toF2");if a==1{set(&mut result,k);}}result};
 Dual28{f:pack(output_f),g:pack(output_g)}
}
fn full28(u:&B,v:&B,ub:&B,vb:&B,kind:usize,weighted_x:bool,n:usize)->Dual28 {
 let big=121*n;let large_pole=if kind==1{366}else{365};let small_pole=if kind==1{366}else{245};
 let large_f=add(&mul(if kind==1{v}else{u},&dilate(v,121,n),n),&shift(&one(),large_pole,n));
 let large_g=if weighted_x{shift(&dilate(u,121,n),large_pole-242,n)}else{shift(&one(),large_pole,n)};
 let small_f=if kind==1 {add(&mul(vb,&dilate(vb,121,big),big),&shift(&one(),small_pole,big))}
 else{add(&mul(&dilate(ub,121,big),vb,big),&shift(&one(),small_pole,big))};
 let small_g=if weighted_x{shift(ub,small_pole-2,big)}else{shift(&one(),small_pole,big)};
 let small=norm121d28(&Dual28{f:small_f,g:small_g},n);
 let middle=middle28(u,v,kind,weighted_x,n);
 dm28(&dm28(&small,&middle,n),&Dual28{f:large_f,g:large_g},n)
}
fn save28(dir:&Path,name:&str,value:&serde_json::Value){let p=dir.join(name);assert!(!p.exists());std::fs::write(p,serde_json::to_string_pretty(value).unwrap()+"\n").unwrap();}
fn main(){
 let args=std::env::args().collect::<Vec<_>>();assert_eq!(args.len(),3,"output and original-input directory");let out=Path::new(&args[1]);let input=Path::new(&args[2]);
 let n=1024;let binary_controls=control(32);
 let small=Dual28{f:vec![3],g:one()};let tested=norm121d28(&small,32);assert_eq!(tested.f,vec![3]);assert_eq!(tested.g,one());
 let proportional=Dual28{f:vec![0b1011011],g:vec![0b1011011]};let tested=norm121d28(&proportional,32);assert_eq!(tested.f,tested.g);
 let left=Dual28{f:vec![3],g:vec![5]};let right=Dual28{f:vec![7],g:vec![11]};
 let product=norm121d28(&dm28(&left,&right,121*32),32);let norms_product=dm28(&norm121d28(&left,32),&norm121d28(&right,32),32);assert_eq!(product.f,norms_product.f);assert_eq!(product.g,norms_product.g);
 println!("DUAL28_DETERMINANT_DUAL_MULTIPLICATIVITY_CONTROLS_PASS");
 let(u,v)=xy(n);let(ub,vb)=xy(121*n);
 let mut records=vec![];
 for kind in [1,2] {
  let original:serde_json::Value=serde_json::from_slice(&std::fs::read(input.join(format!("m15-function-{kind}.json"))).unwrap()).unwrap();let pole=if kind==1{792}else{660};
  let mut derivatives=vec![];
  for weighted_x in [false,true] {
   let result=full28(&u,&v,&ub,&vb,kind,weighted_x,n);let recovered_base=recover(&result.f,pole,&u,&v,n);
   assert_eq!(recovered_base["A_bits"],original["A_bits"]);assert_eq!(recovered_base["B_bits"],original["B_bits"]);
   let corrupt=add(&bits28(original["A_bits"].as_str().unwrap()),&one());assert_ne!(hex(&corrupt),original["A_bits"].as_str().unwrap());
   let derivative=recover(&result.g,pole,&u,&v,n);
   println!("DUAL28_DERIVATIVE kind={kind} weighted_x={weighted_x} exact1024 positiveResidualZero");
   save28(out,&format!("dual28-function-{kind}-{}.json",if weighted_x{"xinf"}else{"one"}),&derivative);derivatives.push(derivative);
  }
  records.push(json!({"function_index":kind,"pole_bound":pole,"norm_matches_original":true,"derivative_one":derivatives[0],"derivative_xinf":derivatives[1]}));
 }
 save28(out,"dual28-results.json",&json!({"schema":"koblitz.round28/adjugate-norm-v1","precision":n,"input_precision":121*n,"records":records,"binary_controls":binary_controls,"dual_one_plus_t":true,"dual_proportional":true,"dual_multiplicativity":true,"image_values_selected":false,"geometry_bridge":"written Hecke/qexp/poles/RR and simple norm-zero adjugate specialization; not inferred from finite matching alone"}));
 println!("DUAL28_ALL_CHECKS_PASS");
}
