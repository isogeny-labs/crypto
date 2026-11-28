fn progression_inverse_checks(){
 let(mut setups,mut samples,mut evaluations,mut boundary)=(0usize,0usize,0usize,0usize);
 for r in [7u64,11,17,41,97]{let mut degrees=vec![1usize,2,3,7,16,(r-1)as usize];degrees.sort();degrees.dedup();for d in degrees{if d as u64>=r{continue}for step in [1,2,r-1]{for seed in [0,1,3]{
  let f:Vec<_>=(0..=d).map(|i|seed*(i as u64+1)%r).collect();let mut p=polynomial::Poly::new(r).unwrap();
  let(ctx,snapshot)=meter::capture(||{let _region=meter::Region::enter();ProgressionValues::new(r,&mut p,&f,step)}).unwrap();let ctx=ctx.unwrap();assert_eq!(snapshot.counts[3],1);assert!(!snapshot.overflow);assert_eq!(snapshot.counts[8],0);assert_eq!(mul(ctx.inv_step,step,r),1);
  for i in 0..=d{let x=mul(i as u64,step,r);let v=eval(&f,x,r);assert_eq!(ctx.samples[i],v);let denominator=(0..=d).filter(|&j|j!=i).fold(1,|a,j|mul(a,(i as u64+r-j as u64)%r,r));assert_eq!(mul(ctx.weights[i],denominator,r),v);samples+=1;}
  for first in 0..r{for count in [1,d]{let got=ctx.evaluate_cached(&mut p,first,count).unwrap();let expected:Vec<_>=(0..count).map(|j|eval(&f,(first+mul(j as u64,step,r))%r,r)).collect();assert_eq!(got.values,expected);evaluations+=count;}}
  setups+=1;boundary+=usize::from(d as u64==r-1);
 }}}}
 let mut p=polynomial::Poly::new(7).unwrap();for(f,step)in[(vec![1],1),(vec![1,2],0),(vec![1,2],7),(vec![1,7],1),(vec![1;8],1)]{assert_eq!(ProgressionValues::new(7,&mut p,&f,step).err(),Some(Error::Domain));}
 println!("progression_shared_inverse_setups={setups} sample_and_weight_checks={samples} evaluation_values={evaluations} degree_r_minus_one_setups={boundary} setup_fermat_calls_each=1 invalid_domains=5");
}
fn main(){if std::env::args().any(|x|x=="--inherited"){inherited_checks()}else{progression_inverse_checks();meter_checks();}}
