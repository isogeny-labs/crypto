fn reference_pow(a:u64,n:u64,r:u64)->u64{let mut v=1u64;for bit in(0..64).rev(){v=(v as u128*v as u128%r as u128)as u64;if(n>>bit)&1!=0{v=(v as u128*a as u128%r as u128)as u64}}v}
fn terminal_checks(){
 let(mut powers,mut batches,mut inverses)=(0usize,0usize,0usize);
 for r in [7u64,11,17,97]{for a in 0..r{for n in(0..130).chain([1u64<<63,(1u64<<63)-1,u64::MAX]){
  let(expected,count)=(reference_pow(a,n,r),if n==0{0}else{63-n.leading_zeros()as u64+n.count_ones()as u64});
  let(got,s)=meter::capture(||{let _region=meter::Region::enter();pow(a,n,r)}).unwrap();assert_eq!(got,expected);assert_eq!(s.counts[0],count);assert!(!s.overflow);powers+=1;
 }}
 for len in [0usize,1,2,3,7,16]{for seed in 0..4{let original:Vec<_>=(0..len).map(|i|1+(i as u64*7+seed)%(r-1)).collect();let mut prefix=vec![1];for &v in &original{prefix.push(mul(*prefix.last().unwrap(),v,r));}let inv=reference_pow(*prefix.last().unwrap(),r-2,r);let mut values=original.clone();
  let(result,s)=meter::capture(||{let _region=meter::Region::enter();finish_batch_inverses(&mut values,&prefix,inv,r)}).unwrap();result.unwrap();assert_eq!(s.counts[0],2*len.saturating_sub(1)as u64);assert!(!s.overflow);
  for(&v,&w)in original.iter().zip(&values){assert_eq!(mul(v,w,r),1);inverses+=1;}batches+=1;
 }}
 }
 assert_eq!(finish_batch_inverses(&mut[1],&[1],1,7),Err(Error::Domain));assert_eq!(finish_batch_inverses(&mut[1],&[0,1],1,7),Err(Error::Domain));assert_eq!(finish_batch_inverses(&mut[],&[],1,7),Err(Error::Domain));
 println!("terminal_power_cases={powers} prefix_batches={batches} inverse_values={inverses} malformed_prefix_controls=3 exact_helper_counts=true");
}
fn terminal_predecessor_main(){if std::env::args().any(|a|a=="--inherited"){inherited_checks()}else{terminal_checks();progression_inverse_checks();meter_checks();}}

include!("triple.rs");
include!("triple-checks.rs");
