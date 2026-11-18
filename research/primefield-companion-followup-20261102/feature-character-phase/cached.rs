struct CachedEvaluation{values:Vec<u64>,copied:usize,convolutions:usize}
impl ProgressionValues{
 fn evaluate_cached(&self,p:&mut polynomial::Poly,first:u64,count:usize)->Result<CachedEvaluation,Error>{
  if first>=self.r||count==0||count>self.d{return Err(Error::Domain)}
  let mut a=mul(first,self.inv_step,self.r);let(mut copied,mut convolutions)=(0,0);let mut values=Vec::with_capacity(count);
  while values.len()<count{
   let remaining=count-values.len();
   let taken=if a<=self.d as u64{
    let taken=remaining.min(self.d+1-a as usize);values.extend_from_slice(&self.samples[a as usize..a as usize+taken]);copied+=taken;taken
   }else{
    let taken=remaining.min((self.r-a)as usize);let outside=self.evaluate_normalized(p,a,taken)?.ok_or(Error::Arithmetic)?;assert_eq!(outside.len(),taken);values.extend(outside);convolutions+=1;taken
   };
   a=(a+taken as u64)%self.r;
  }
  assert!(convolutions<=1);Ok(CachedEvaluation{values,copied,convolutions})
 }
}
fn cached_shift_checks(){
 let(mut queries,mut values_checked,mut old_singular,mut copied,mut cache_only,mut mixed,mut wrapped)=(0,0,0,0,0,0,0);
 for r in [7u64,17,97,257]{for d in [1usize,2,3,7,16]{if d as u64>=r{continue}for seed in 0..3u64{for step in [1,2,r-1]{
  let f:Vec<_>=(0..=d).map(|i|seed*(i as u64+3)%r).collect();let mut p=polynomial::Poly::new(r).unwrap();let ctx=ProgressionValues::new(r,&mut p,&f,step).unwrap();
  for a in 0..r{for count in [1,(d+1)/2,d]{let first=mul(a,step,r);let expected:Vec<_>=(0..count).map(|i|eval(&f,mul((a+i as u64)%r,step,r),r)).collect();let old=ctx.evaluate(&mut p,first,count).unwrap();let result=ctx.evaluate_cached(&mut p,first,count).unwrap();assert_eq!(result.values,expected);if let Some(old)=old{assert_eq!(result.values,old)}else{old_singular+=1}
   assert!(result.convolutions<=1);let expected_copied=(0..count).filter(|&i|(a+i as u64)%r<=d as u64).count();assert_eq!(result.copied,expected_copied);assert_eq!(result.convolutions,usize::from(expected_copied<count));copied+=result.copied;if result.convolutions==0{cache_only+=1}else if result.copied>0{mixed+=1}if a+count as u64>r{wrapped+=1}queries+=1;values_checked+=count;
  }}
 }}}}
 let mut p=polynomial::Poly::new(17).unwrap();let ctx=ProgressionValues::new(17,&mut p,&[1,2,3],1).unwrap();assert!(ctx.evaluate_cached(&mut p,0,0).is_err());assert!(ctx.evaluate_cached(&mut p,0,3).is_err());assert!(ctx.evaluate_cached(&mut p,17,1).is_err());
 // Former singular progression uses cache plus at most one regular convolution, no query tree.
 let mut helper=IntervalProducts::new(257,56,7).unwrap();let requests=[(0,56),(100,33),(200,33)];let got=helper.products(&requests).unwrap();for ((a,l),v) in requests.into_iter().zip(got){assert_eq!(v,(0..l).fold(1,|x,j|mul(x,(a+j)%257,257)))}assert!(helper.cache_only_batches>0);assert!(helper.query_trees>0);assert_eq!(helper.progression_batches+helper.query_trees,helper.evaluation_batches);
 let mut stream=CoeffStream::new(1009,56,0,7).unwrap();while stream.left>0{stream.take_desc(stream.left.min(7)).unwrap();}assert_eq!(stream.products.progression_batches+stream.products.query_trees,stream.products.evaluation_batches);
 println!("cached_shift_queries={queries} exact_values={values_checked} previous_singular_queries={old_singular} copied_values={copied} cache_only_queries={cache_only} mixed_queries={mixed} wrapped_queries={wrapped} stream_progression_batches={} stream_convolutions={} stream_cached_values={} stream_query_trees={}",stream.products.progression_batches,stream.products.convolution_calls,stream.products.cached_copies,stream.products.query_trees);
}
