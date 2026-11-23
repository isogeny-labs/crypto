fn progression_checks(){
 let(mut fast,mut singular,mut values_checked)=(0,0,0);
 for r in [7u64,17,97,257]{for d in [1usize,2,3,7,16]{if d as u64>=r{continue}for seed in 0..3u64{for step in [1,2,r-1]{
  let f:Vec<_>=(0..=d).map(|i|seed*(i as u64+3)%r).collect();let mut p=polynomial::Poly::new(r).unwrap();let ctx=ProgressionValues::new(r,&mut p,&f,step).unwrap();
  for a in 0..r{for count in [1,d]{let first=mul(a,step,r);let points:Vec<_>=(0..count).map(|i|mul((a+i as u64)%r,step,r)).collect();let expected:Vec<_>=points.iter().map(|&x|eval(&f,x,r)).collect();
   let should_fallback=(0..d+count).any(|j|(a+r-d as u64+j as u64)%r==0);
   match ctx.evaluate(&mut p,first,count).unwrap(){Some(got)=>{assert!(!should_fallback);assert_eq!(got,expected);fast+=1},None=>{assert!(should_fallback);let tree=p.product_tree(&points).unwrap();assert_eq!(p.values(&f,&tree).unwrap(),expected);singular+=1}}values_checked+=count;
  }}
 }}}}
 let mut p=polynomial::Poly::new(17).unwrap();assert!(ProgressionValues::new(17,&mut p,&[1],1).is_err());assert!(ProgressionValues::new(17,&mut p,&[1,1],0).is_err());let c=ProgressionValues::new(17,&mut p,&[1,1],1).unwrap();assert!(c.evaluate(&mut p,1,0).is_err());assert!(c.evaluate(&mut p,1,2).is_err());
 let(mut progression_batches,mut tree_batches,mut dispatch_products)=(0,0,0);
 for cap in [2usize,7,16]{let mut helper=IntervalProducts::new(65537,512,cap).unwrap();
  for start in [0,1,1000,65000]{let requests=[(start,512),((start+17)%65537,128),(0,0)];let got=helper.products(&requests).unwrap();for ((a,l),v) in requests.into_iter().zip(got){assert_eq!(v,(0..l).fold(1,|x,j|mul(x,(a+j)%65537,65537)));dispatch_products+=1;}}
  assert!(helper.progression_batches>0);assert_eq!(helper.evaluation_batches,helper.progression_batches+helper.query_trees);progression_batches+=helper.progression_batches;tree_batches+=helper.query_trees;
 }
 // A production-mode coefficient stream also takes the accelerated route.
 let mut s=CoeffStream::new(1009,56,0,7).unwrap();while s.left>0{s.take_desc(s.left.min(7)).unwrap();}assert!(s.products.progression_batches>0);assert_eq!(s.products.evaluation_batches,s.products.progression_batches+s.products.query_trees);
 println!("old_control_nonsingular={fast} old_control_singular={singular} exact_values={values_checked} dispatch_products={dispatch_products} dispatch_progression_batches={progression_batches} dispatch_query_trees={tree_batches} coefficient_stream_progression_batches={} coefficient_stream_query_trees={}",s.products.progression_batches,s.products.query_trees);
}
