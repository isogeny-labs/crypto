#![allow(dead_code)]
mod meter;
#[path="generated/scalar.rs"]mod scalar;
#[path="generated/euclid.rs"]mod euclid;
#[path="generated/half.rs"]mod half;
fn evaluate(arm:&str,p:u64,roots:&[u64],points:&[u64])->Result<(u64,meter::Snapshot),&'static str>{
 if p<3||p>u64::MAX/2||(1u64<<(p-1).trailing_zeros())>4{return Err("unsupported modulus; primality must be certified by input contract");}
 match arm{
  "scalar"=>meter::measure(||{let mut c=scalar::Poly::new(p).map_err(|_|"domain")?;let v=c.root_resultant(roots,points).map_err(|_|"resultant")?;Ok(meter::project(v,p))}),
  "euclid"=>meter::measure(||{let mut c=euclid::Poly::new(p).map_err(|_|"domain")?;c.root_projected_resultant(roots,points).map_err(|_|"resultant")}),
  "half"=>meter::measure(||{let mut c=half::Poly::new(p).map_err(|_|"domain")?;c.root_projected_resultant(roots,points).map_err(|_|"resultant")}),
  _=>Err("unknown arm")
 }
}
fn self_test(){
 meter::self_test();
 for arm in ["scalar","euclid","half"]{let(v,s)=evaluate(arm,13,&[1,2],&[3,4]).unwrap();assert_eq!(v,12);assert_eq!(s.ops[7],1);assert_eq!(evaluate(arm,13,&[1,2],&[3,4]).unwrap(),(v,s));assert!(evaluate(arm,13,&[],&[3]).is_err());}
 let(_,s)=meter::measure(||{let mut c=half::Poly::new(13).unwrap();assert_eq!(c.mul(&[1,2],&[3,4]),[3,10,8]);Ok(0)}).unwrap();
 assert_eq!(&s.ops[..8],&[4,4,0,0,7,0,0,0]);
 let(_,s)=meter::measure(||{let c=half::Poly::new(13).unwrap();assert_eq!(c.add(&[1,2],&[3,4]),[4,6]);Ok(0)}).unwrap();
 assert_eq!(&s.ops[..8],&[0,2,0,0,6,0,0,0]);
 let(_,s)=meter::measure(||{let c=half::Poly::new(13).unwrap();assert_eq!(c.sub(&[1,2],&[3,4]),[11,11]);Ok(0)}).unwrap();
 assert_eq!(&s.ops[..8],&[0,2,0,2,8,0,0,0]);
 let(_,s)=meter::measure(||{let mut c=half::Poly::new(13).unwrap();assert_eq!(c.inv_series(&[2],1).unwrap(),[7]);Ok(0)}).unwrap();
 assert_eq!(&s.ops[..8],&[0,0,0,0,3,1,2,0]);
 // Class arithmetic bypasses fm; verify its direct sites are charged.
 let(_,s)=meter::measure(||{let c=half::Poly::new(13).unwrap();let v=c.image_class(2).unwrap().combine(c.image_class(3).unwrap()).unwrap().square().negate().project();assert_eq!(v,1);Ok(0)}).unwrap();
 assert_eq!(&s.ops[..8],&[6,0,0,1,2,0,0,1]);
 assert!(evaluate("half",17,&[1],&[2]).is_err());
 println!("RESULTANT_ACCOUNTING_SELF_TESTS_PASS");
}
fn list(s:&str)->Result<Vec<u64>,&'static str>{s.split(',').map(|x|x.parse().map_err(|_|"invalid coefficient")).collect()}
fn main(){
 let args:Vec<_>=std::env::args().collect();
 if args.len()==2&&args[1]=="--self-test"{self_test();return;}
 if args.len()==6&&args[1]=="--case"{
  let p=args[3].parse().expect("modulus");let roots=list(&args[4]).unwrap();let points=list(&args[5]).unwrap();
  let(v,s)=evaluate(&args[2],p,&roots,&points).expect("valid scalar-only accounting interval");
  print!("{{\"arm\":\"{}\",\"value\":{},\"metrics\":{{",args[2],v);
  for(i,(name,value))in meter::NAMES.iter().zip(s.ops).chain(meter::HEAP_NAMES.iter().zip(s.heap)).enumerate(){if i>0{print!(",");}print!("\"{}\":{}",name,value);}println!("}}}}");return;
 }
 eprintln!("usage: --self-test | --case ARM P ROOTS_CSV POINTS_CSV (panel execution requires a separately frozen protocol)");std::process::exit(2);
}
