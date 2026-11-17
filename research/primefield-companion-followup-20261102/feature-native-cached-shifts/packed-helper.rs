/// Pack block values across bounded interval requests; keep each result owner.
struct IntervalProducts{r:u64,width:usize,request_cap:usize,polynomial:Option<Vec<u64>>,p:polynomial::Poly,blocked_calls:usize,max_points:usize,evaluation_batches:usize,progression:Option<ProgressionValues>,progression_batches:usize,query_trees:usize,convolution_calls:usize,cached_copies:usize,cache_only_batches:usize}
impl IntervalProducts{
 fn new(r:u64,max_length:u64,cap:usize)->Result<Self,Error>{
  if r<7||r>u64::MAX/2||max_length>=r||cap==0{return Err(Error::Domain)}
  let(mut low,mut high)=(1u64,max_length.max(1));while low<high{let mid=low+(high-low)/2;if mid as u128*mid as u128>=max_length as u128{high=mid}else{low=mid+1}}
  let width=(low as usize).min(cap);Ok(Self{r,width,request_cap:cap.saturating_mul(2),polynomial:None,p:polynomial::Poly::new(r).map_err(|_|Error::Domain)?,blocked_calls:0,max_points:0,evaluation_batches:0,progression:None,progression_batches:0,query_trees:0,convolution_calls:0,cached_copies:0,cache_only_batches:0})
 }
 fn flush(&mut self,points:&mut Vec<u64>,owners:&mut Vec<usize>,out:&mut[u64])->Result<(),Error>{
  if points.is_empty(){return Ok(())}assert_eq!(points.len(),owners.len());assert!(points.len()<=self.width);
  self.max_points=self.max_points.max(points.len());let step=self.width as u64;
  let eligible=points.len()>=2&&points.windows(2).all(|v|v[1]==(v[0]+step)%self.r);
  let accelerated=if eligible{if self.progression.is_none(){self.progression=Some(ProgressionValues::new(self.r,&mut self.p,self.polynomial.as_ref().ok_or(Error::Arithmetic)?,step)?);}{let result=self.progression.as_ref().unwrap().evaluate_cached(&mut self.p,points[0],points.len())?;self.convolution_calls+=result.convolutions;self.cached_copies+=result.copied;self.cache_only_batches+=usize::from(result.convolutions==0);Some(result.values)}}else{None};
  let values=if let Some(values)=accelerated{self.progression_batches+=1;values}else{let tree=self.p.product_tree(points).map_err(|_|Error::Arithmetic)?;self.query_trees+=1;self.p.values(self.polynomial.as_ref().ok_or(Error::Arithmetic)?,&tree).map_err(|_|Error::Arithmetic)?};
  assert_eq!(values.len(),owners.len());for (&owner,value) in owners.iter().zip(values){out[owner]=mul(out[owner],value,self.r)}
  self.evaluation_batches+=1;points.clear();owners.clear();Ok(())
 }
 fn product(&mut self,start:u64,length:u64)->Result<u64,Error>{Ok(self.products(&[(start,length)])?[0])}
 fn products(&mut self,requests:&[(u64,u64)])->Result<Vec<u64>,Error>{
  if requests.len()>self.request_cap{return Err(Error::Limit)}
  if requests.iter().any(|&(a,l)|a>=self.r||l>=self.r){return Err(Error::Domain)}
  let mut out=vec![1;requests.len()];let mut points=Vec::new();let mut owners=Vec::new();let w=self.width as u64;
  for (owner,&(start,length)) in requests.iter().enumerate(){
   if length<32||self.width<2||length<w{out[owner]=(0..length).fold(1,|a,j|mul(a,(start+j)%self.r,self.r));continue}
   if self.polynomial.is_none(){let roots:Vec<_>=(0..w).map(|i|(self.r-i)%self.r).collect();self.polynomial=Some(self.p.product_tree(&roots).map_err(|_|Error::Arithmetic)?.polynomial().to_vec());}
   self.blocked_calls+=1;let blocks=length/w;
   for j in 0..blocks{points.push((start+j*w)%self.r);owners.push(owner);if points.len()==self.width{self.flush(&mut points,&mut owners,&mut out)?;}}
   for j in blocks*w..length{out[owner]=mul(out[owner],(start+j)%self.r,self.r)}
  }
  self.flush(&mut points,&mut owners,&mut out)?;Ok(out)
 }
}
