#[path="division.rs"]mod normalized;
fn division_checks(){
 let(mut comparisons,mut antipodes,mut cache_checks)=(0,0,0);
 for p in [5u64,7,17,41,97,193,257,509]{
  let(a,b,_,_)=if p==5{(2,1,7,[0,1])}else if p==7{(0,3,13,[1,2])}else if p==509{(2,10,479,[3,68])}else{fixture(p)};
  for x in 0..p{for y in 1..p{if mul(y,y,p)!=(mul(mul(x,x,p),x,p)+mul(a,x,p)+b)%p{continue}
   let mut got=normalized::ValueDivision::new(p,a,b,x,y).unwrap();let mut old=division::ValueDivision::new(p,a,b,x,y).unwrap();let mut negative=normalized::ValueDivision::new(p,a,b,x,p-y).unwrap();
   for n in (0..128).chain([u64::MAX-1,u64::MAX]){let v=got.value(n);assert_eq!(v,old.value(n),"p={p} x={x} y={y} n={n}");comparisons+=1;assert_eq!(negative.value(n),if n&1==0{(p-v)%p}else{v});antipodes+=1;
    let size=got.retained_indices();assert_eq!(got.value(n),v);assert_eq!(got.retained_indices(),size);cache_checks+=1;
   }
  }}
 }
 assert!(matches!(normalized::ValueDivision::new(3,0,0,0,0),Err(normalized::Error::Domain)));
 assert!(matches!(normalized::ValueDivision::new(u64::MAX,0,0,0,0),Err(normalized::Error::Domain)));
 assert!(matches!(normalized::ValueDivision::new(5,0,0,0,0),Err(normalized::Error::Singular)));
 assert!(matches!(normalized::ValueDivision::new(5,1,0,0,0),Err(normalized::Error::ZeroY)));
 assert!(matches!(normalized::ValueDivision::new(509,2,10,3,69),Err(normalized::Error::Point)));
 assert!(matches!(normalized::ValueDivision::new(509,2,10,512,68),Err(normalized::Error::Point)));
 println!("normalized_division_equalities={comparisons} antipodal_checks={antipodes} repeated_cache_checks={cache_checks} maximum_u64_indices=true");
}
