/// Capped arithmetic product blocks, unrelated to group collision search.
struct IntervalProducts{r:u64,width:usize,polynomial:Option<Vec<u64>>,p:polynomial::Poly,blocked_calls:usize,max_points:usize}
impl IntervalProducts{
 fn new(r:u64,max_length:u64,cap:usize)->Result<Self,Error>{
  if r<7||r>u64::MAX/2||max_length>=r||cap==0{return Err(Error::Domain)}
  let(mut low,mut high)=(1u64,max_length.max(1));while low<high{let mid=low+(high-low)/2;if mid as u128*mid as u128>=max_length as u128{high=mid}else{low=mid+1}}
  let width=(low as usize).min(cap);Ok(Self{r,width,polynomial:None,p:polynomial::Poly::new(r).map_err(|_|Error::Domain)?,blocked_calls:0,max_points:0})
 }
 fn product(&mut self,start:u64,length:u64)->Result<u64,Error>{
  if start>=self.r||length>=self.r{return Err(Error::Domain)}
  if length<32||self.width<2||length<self.width as u64{return Ok((0..length).fold(1,|a,j|mul(a,(start+j)%self.r,self.r)))}
  let w=self.width as u64;
  if self.polynomial.is_none(){let roots:Vec<_>=(0..w).map(|i|(self.r-i)%self.r).collect();self.polynomial=Some(self.p.product_tree(&roots).map_err(|_|Error::Arithmetic)?.polynomial().to_vec());}
  self.blocked_calls+=1;let blocks=length/w;let mut done=0;let mut out=1;
  while done<blocks{let count=(blocks-done).min(w);self.max_points=self.max_points.max(count as usize);
   let points:Vec<_>=(0..count).map(|j|(start+(done+j)*w)%self.r).collect();let tree=self.p.product_tree(&points).map_err(|_|Error::Arithmetic)?;
   for v in self.p.values(self.polynomial.as_ref().unwrap(),&tree).map_err(|_|Error::Arithmetic)?{out=mul(out,v,self.r)}done+=count;
  }
  for j in blocks*w..length{out=mul(out,(start+j)%self.r,self.r)}Ok(out)
 }
}
fn interval_checks(){
 let(mut cases,mut blocked,mut zero,mut wrapped)=(0,0,0,0);
 for r in [7u64,17,97,257,65537]{for cap in [1usize,2,7,16,64]{let mut lengths:Vec<_>=(0..r.min(70)).collect();for n in [127,256,511,1024,4096]{if n<r{lengths.push(n)}}
  let max=*lengths.iter().max().unwrap();let mut products=IntervalProducts::new(r,max,cap).unwrap();
  for length in lengths{for start in [0,1,r-1,(r-1)/2]{let expected=(0..length).fold(1,|v,i|mul(v,(start+i)%r,r));assert_eq!(products.product(start,length).unwrap(),expected);cases+=1;if expected==0{zero+=1}if start+length>r{wrapped+=1}}}
  assert!(products.width<=cap);assert!(products.max_points<=products.width);assert!(products.polynomial.as_ref().map_or(true,|p|p.len()==products.width+1));blocked+=products.blocked_calls;
 }}
 assert!(IntervalProducts::new(13,13,2).is_err());assert!(IntervalProducts::new(13,3,0).is_err());let mut q=IntervalProducts::new(13,3,2).unwrap();assert!(q.product(13,1).is_err());assert!(q.product(0,13).is_err());
 // Constant modes initialize from the smaller symmetric binomial index.
 let r=65537;for e in 0..3{let mut s=CoeffStream::new(r,r as usize-1,e,7).unwrap();assert_eq!(s.left,1);let expected=if e==0{1}else if e==1{(r-(r-1)/2)%r}else{mul(mul((r-1)/2,(r-1)/2-1,r),pow(2,r-2,r),r)};assert_eq!(s.take_desc(1).unwrap(),vec![expected]);assert_eq!(s.products.blocked_calls,0);assert!(s.products.polynomial.is_none());}
 println!("interval_exact_comparisons={cases} blocked_product_calls={blocked} zero_products={zero} wrapped_intervals={wrapped} constant_mode_initializers=3 bounded_width_and_batches=true");
}
fn interval_public_checks(){
 let(p,a,b,r,base,m)=(257,1,7,281,[1,3],56usize);let c=group::Curve::new(p,a,b);let bp=group::Point{x:base[0],y:base[1]};let source=source::Source::new(p,a,b,r,base,64).unwrap();let z=(2..r).find(|&z|pow(z,m as u64,r)==1&&(1..m).all(|j|pow(z,j as u64,r)!=1)).unwrap();let old=Modes::compile(r,m,z,m,256).unwrap();let new=StreamModes::new(r,m,z,m,256,7).unwrap();let extra:Vec<_>=(0..17).collect();let(mut calls,mut unresolved)=(0,0);
 let mut probe=new.stream(0).unwrap();while probe.left>0{probe.take_desc(probe.left.min(7)).unwrap();}assert!(probe.products.blocked_calls>0);
 for k in 0..r{let target=multiple(c,bp,k);let(expected,os,route)=old.recover_traced(&source,c,bp,target,&[1,2,3,4],64,&extra,256).unwrap();let(got,ns,nroute)=new.recover_traced(&source,c,bp,target,&[1,2,3,4],64,&extra,256).unwrap();assert_eq!(got,expected);assert_eq!(nroute,route);assert_eq!((ns.calls,ns.extra,ns.replayed,ns.lifted,ns.image_count),(os.calls,os.extra,os.replayed,os.lifted,os.image_count));if let Some(v)=got{assert_eq!(v,k)}else{unresolved+=1}calls+=ns.calls;}
 println!("interval_public_cases={r} mode_m={m} source_calls={calls} unresolved={unresolved} blocked_stream_exercised=true");
}
