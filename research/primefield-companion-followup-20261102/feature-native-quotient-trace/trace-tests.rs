#[path="polynomial.rs"]mod current;
#[path="../feature-native-weighted-quotient/polynomial.rs"]mod reference;
fn main(){
 use current::trace;
 assert!(trace::finish().is_err());assert!(trace::begin(100001).is_err());
 let mut cases=0;
 for p in [5u64,17,97,257]{for n in [1usize,2,3,8,32]{for lead in [1u64,2]{
  let d=n+2;let mut a=vec![1;d+n];a[d+n-1]=2;let mut b=vec![1;d+1];b[d]=lead;
  for mode in 0..3{
   for j in 1..=d.min(n-1){b[d-j]=if mode==0{0}else if mode==1{u64::from(j+2>=n)}else{1};}
   let expected=reference::Poly::new(p).unwrap().divrem(&a,&b).unwrap();
   let mut poly=current::Poly::new(p).unwrap();trace::begin(1).unwrap();assert!(trace::begin(1).is_err());let got=poly.divrem(&a,&b).unwrap();let t=trace::finish().unwrap();assert_eq!(got,expected);assert!(!t.invalid);assert_eq!(t.rows.len(),1);let e=&t.rows[0];assert_eq!((e.p,e.n,e.d,e.ok),(p,n,d,true));
   let js:Vec<_>=(1..=d.min(n-1)).filter(|&j|b[d-j]!=0).collect();let w:u128=js.iter().map(|&j|(n-j)as u128).sum();assert_eq!(e.support,js.len());assert_eq!(e.weight,w);assert_eq!(e.inversions,u64::from(lead!=1));
   let route=if n<=2{n as u8}else if w< n as u128{3}else{4};assert_eq!(e.route,route);
   if route==1{assert_eq!(e.products,u64::from(lead!=1));}
   if route==3{let q=&got.0;let scale=if lead==1{0}else{q.iter().filter(|&&v|v!=0).count()};let cross:usize=js.iter().map(|&j|q.iter().rev().take(n-j).filter(|&&v|v!=0).count()).sum();assert_eq!(e.products,(scale+cross)as u64);}
   // A/A transcript repeat, independently of global counter offsets and prior remainder work.
   trace::begin(1).unwrap();assert_eq!(poly.divrem(&a,&b).unwrap(),got);assert_eq!(trace::finish().unwrap().rows,t.rows);cases+=1;
  }
 }}}
 trace::begin(1).unwrap();current::Poly::new(17).unwrap().divrem(&[0],&[1]).unwrap();assert_eq!(trace::finish().unwrap().rows[0].route,0);
 trace::begin(0).unwrap();current::Poly::new(17).unwrap().divrem(&[1,2],&[1]).unwrap();let t=trace::finish().unwrap();assert!(t.invalid);assert_eq!(t.dropped,1);assert!(t.rows.is_empty());
 trace::begin(1).unwrap();trace::force_overflow();assert!(trace::finish().unwrap().invalid);
 trace::begin(1).unwrap();std::thread::spawn(||{trace::begin(1).unwrap();current::Poly::new(17).unwrap().divrem(&[1,2],&[1]).unwrap();assert_eq!(trace::finish().unwrap().rows.len(),1);}).join().unwrap();assert!(trace::finish().unwrap().rows.is_empty());
 trace::begin(1).unwrap();assert!(current::Poly::new(17).unwrap().divrem(&[1],&[0]).is_err());assert!(trace::finish().unwrap().rows.is_empty());
 println!("synthetic_trace_cases={cases} aa_transcripts=true routes=0,1,2,3,4 exact_metadata=true exact_sparse_products=true cap_drop_detected=true overflow_detected=true thread_isolation=true invalid_divisor_no_event=true");
}
