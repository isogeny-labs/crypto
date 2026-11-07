fn packed_checks(){
 let(mut panels,mut requests_checked,mut fewer,mut equal)=(0,0,0,0);
 for r in [97u64,257,65537]{for m in [32u64,56,64,80,128,512]{if m>=r{continue}for cap in [1usize,2,7,16,64]{for seed in 0..5usize{
  let count=if seed==0{0}else{(seed*7).min(2*cap)};let requests:Vec<_>=(0..count).map(|j|{let start=match (j+seed)%4{0=>0,1=>1,2=>r-1,_=>(r-1)/2};let len=match (j+seed)%5{0=>0,1=>m.min(31),_=>m};(start,len)}).collect();
  let mut packed=IntervalProducts::new(r,m,cap).unwrap();let mut separate=IntervalProducts::new(r,m,cap).unwrap();let got=packed.products(&requests).unwrap();
  let old:Vec<_>=requests.iter().map(|&(a,l)|separate.product(a,l).unwrap()).collect();let direct:Vec<_>=requests.iter().map(|&(a,l)|(0..l).fold(1,|v,i|mul(v,(a+i)%r,r))).collect();assert_eq!(got,old);assert_eq!(got,direct);
  let w=packed.width as u64;let t:Vec<_>=requests.iter().map(|&(_,l)|if l>=32&&w>=2&&l>=w{l/w}else{0}).collect();let total:u64=t.iter().sum();let expected=(total+w-1)/w;let old_expected:u64=t.iter().map(|&n|(n+w-1)/w).sum();assert_eq!(packed.evaluation_batches,expected as usize);assert_eq!(separate.evaluation_batches,old_expected as usize);assert!(expected<=old_expected);assert!(packed.max_points<=packed.width);
  if expected<old_expected{fewer+=1}else{equal+=1}panels+=1;requests_checked+=count;
 }}}}
 let mut p=IntervalProducts::new(257,56,2).unwrap();assert!(p.products(&[(1,56);5]).is_err());assert!(p.products(&[(1,56),(257,0)]).is_err());assert!(p.polynomial.is_none());assert_eq!(p.evaluation_batches,0);assert_eq!(p.products(&[]).unwrap(),Vec::<u64>::new());
 // One real coefficient chunk: seven transitions, fourteen length-56 requests.
 let r=1009;let m=56;let z=(2..r).find(|&z|pow(z,m as u64,r)==1&&(1..m).all(|j|pow(z,j as u64,r)!=1)).unwrap();let dense=Modes::compile(r,m,z,m,256).unwrap();let mut stream=CoeffStream::new(r,m,0,7).unwrap();let before=stream.products.evaluation_batches;let got=stream.take_desc(7).unwrap();assert_eq!(got,dense.coeff[0].iter().rev().take(7).copied().collect::<Vec<_>>());let batches=stream.products.evaluation_batches-before;assert_eq!(batches,16);let separate_batches=28;assert!(batches<separate_batches);
 println!("packed_request_panels={panels} exact_requests={requests_checked} fewer_batch_panels={fewer} equal_batch_panels={equal} coefficient_chunk_batches={batches} separate_chunk_batches={separate_batches} request_cap_and_preflight=true");
}
