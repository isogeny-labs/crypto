struct PhaseFold{r:u64,n:usize,phases:[u64;4],residues:[Vec<u64>;4],reads:usize}
impl PhaseFold{
 fn build(modes:&RecoveryModes,i:u64,max_degree:usize)->Result<Self,Error>{
  let r=modes.r;if((r-1)/modes.m as u64)%4!=0||i>=r||mul(i,i,r)!=r-1{return Err(Error::Domain)}let n=((r-1)/(4*modes.m as u64))as usize;if n==0||n>max_degree{return Err(Error::Limit)}let phases=[1,r-1,i,r-i];let mut residues: [Vec<u64>;4]=std::array::from_fn(|_|vec![0;n]);let mut reader=CoefficientReader{stream:modes.stream(0)?,buffer:Vec::new(),at:0,remaining:n+1};
  for j in(n..=2*n).rev(){let a=reader.next()?;for k in 0..4{let factor=pow(phases[k],(j/n)as u64,r);let index=j%n;residues[k][index]=(residues[k][index]+mul(a,factor,r))%r;if j!=n{let mirror=2*n-j;residues[k][mirror]=(residues[k][mirror]+a)%r;}}}
  Ok(Self{r,n,phases,residues,reads:n+1})
 }
 fn branch(&self,k:usize,c:u64)->Result<Vec<u64>,Error>{
  if k>=4||c>=self.r{return Err(Error::Domain)}let r=self.r;let u=self.phases[k];let mut equation=self.residues[k].clone();equation[0]=(equation[0]+r-c)%r;while equation.len()>1&&equation.last()==Some(&0){equation.pop();}
  if equation==[0]{let mut g=vec![0;self.n+1];g[0]=r-u;g[self.n]=1;return Ok(g)}if equation.len()==1{return Ok(vec![1])}
  let mut p=polynomial::Poly::new(r).map_err(|_|Error::Domain)?;let(mut value,mut base,mut exponent)=(vec![1],canonical_rem(&mut p,&[0,1],&equation)?,self.n);
  while exponent>0{if exponent&1!=0{let v=p.mul(&value,&base);value=canonical_rem(&mut p,&v,&equation)?;}exponent>>=1;if exponent>0{let v=p.mul(&base,&base);base=canonical_rem(&mut p,&v,&equation)?;}}
  value[0]=(value[0]+r-u)%r;Ok(p.gcd_monic(&equation,&value,self.n+1).map_err(|_|Error::Arithmetic)?.polynomial)
 }
}
// Generic scalar quotient in V. This constructor does not create a CanonicalFiber
// or assert V²-WV+1; it reuses only the scalar component of the evaluator.
impl ReusedRing{
 fn scalar_variable(r:u64,g:&[u64])->Result<Self,Error>{
  if g.len()<2||g.last()!=Some(&1)||g.iter().any(|&x|x>=r){return Err(Error::Domain)}let mut p=polynomial::Poly::new(r).map_err(|_|Error::Domain)?;let root=canonical_rem(&mut p,&[0,1],g)?;let inner=SplitRing{r,both:None,one:g.to_vec(),root,p,reads:0,pair_products:0,scalar_products:0};Ok(Self{inner,powers:std::collections::BTreeMap::new(),scale_key:None,power_products:0,value_products:0,resets:0})
 }
}
struct DirectBranch{u:u64,g:Vec<u64>}
struct DirectImage{r:u64,m:usize,t0:u64,inv_t0:u64,branches:Vec<DirectBranch>,zero:bool,rounds:usize}
std::thread_local!{static DIRECT_COUNTS:std::cell::Cell<(usize,usize,usize,usize)>=const{std::cell::Cell::new((0,0,0,0))};}
impl DirectImage{
 fn new(modes:&RecoveryModes,b:u64,f:&[u64],i:u64)->Result<Self,Error>{
  let r=modes.r;if modes.m<3||b==0||b>=r||f.len()!=3||f.iter().any(|&x|x>=r){return Err(Error::Domain)}let a=mul(modes.m as u64,pow(b,(r-1)/2,r),r);let c=mul(f[0],pow(a,r-2,r),r);let fold=PhaseFold::build(modes,i,((r-1)/(4*modes.m as u64))as usize)?;let mut branches=Vec::new();for k in 0..4{let g=fold.branch(k,c)?;if g.len()>1{branches.push(DirectBranch{u:fold.phases[k],g});}}
  let t0=pow(pow(b,modes.m as u64,r),r-2,r);let mut image=Self{r,m:modes.m,t0,inv_t0:pow(t0,r-2,r),branches,zero:true,rounds:0};image.absorb(modes,b,f,true)?;DIRECT_COUNTS.with(|c|{let(a,b,d,e)=c.get();c.set((a+1,b,d,e))});Ok(image)
 }
 fn update(&mut self,modes:&RecoveryModes,b:u64,f:&[u64])->Result<(),Error>{self.absorb(modes,b,f,false)?;DIRECT_COUNTS.with(|c|{let(a,b,d,e)=c.get();c.set((a,b+1,d,e))});Ok(())}
 fn keep(&mut self,j:usize,value:SplitValue)->Result<(),Error>{if value.both.is_some(){return Err(Error::Arithmetic)}let v=value.one.ok_or(Error::Arithmetic)?;let g=&self.branches[j].g;let mut p=polynomial::Poly::new(self.r).map_err(|_|Error::Domain)?;self.branches[j].g=p.gcd_monic(g,&v,g.len()+v.len()).map_err(|_|Error::Arithmetic)?.polynomial;Ok(())}
 fn contexts(&self)->Result<(Vec<usize>,Vec<ReusedRing>),Error>{let indices:Vec<_>=self.branches.iter().enumerate().filter(|(_,b)|b.g.len()>1).map(|(j,_)|j).collect();let rings=indices.iter().map(|&j|ReusedRing::scalar_variable(self.r,&self.branches[j].g)).collect::<Result<_,_>>()?;Ok((indices,rings))}
 fn absorb(&mut self,modes:&RecoveryModes,b:u64,f:&[u64],first:bool)->Result<(),Error>{
  let r=self.r;if modes.r!=r||modes.m!=self.m||b==0||b>=r||f.len()!=3||f.iter().any(|&x|x>=r){return Err(Error::Domain)}if self.rounds>=4{return Err(Error::Limit)}let h=(r-1)/2;let a=mul(self.m as u64,pow(b,h,r),r);let t=pow(pow(b,self.m as u64,r),r-2,r);let scale=mul(t,self.inv_t0,r);let left=mul(f[1],f[1],r);let right=mul(a,f[2],r);let c1=r-h;let c2=mul(mul(h,h-1,r),(r+1)/2,r);self.zero=self.zero&&f[0]==a&&mul(left,c2,r)==mul(right,mul(c1,c1,r),r);
  if !first{let(indices,mut rings)=self.contexts()?;let values=shared_mode(&mut rings,modes,0,scale)?;for((j,ring),value)in indices.into_iter().zip(&rings).zip(values){let mut eq=ring.scale(&value,a);ring.subtract_constant(&mut eq,f[0]);self.keep(j,eq)?;}}
  let(indices,mut rings)=self.contexts()?;let(f1,f2)=fused_modes(&mut rings,modes,scale)?;for(((j,ring),v1),v2)in indices.into_iter().zip(rings.iter_mut()).zip(f1).zip(f2){let sq=ring.multiply(&v1,&v1)?;let l=ring.scale(&v2,left);let rr=ring.scale(&sq,right);self.keep(j,ring.sub(&l,&rr))?;}self.rounds+=1;Ok(())
 }
 fn materialize(&mut self,lift_cap:usize,image_cap:usize)->Result<Vec<u64>,Error>{
  if self.zero&&image_cap==0{return Err(Error::Limit)}let mut out=if self.zero{vec![0,1]}else{vec![1]};let mut p=polynomial::Poly::new(self.r).map_err(|_|Error::Domain)?;for branch in &self.branches{if branch.g.len()-1>lift_cap||out.len()-1+branch.g.len()-1>image_cap{return Err(Error::Limit)}out=p.mul(&out,&branch.g);}
  let(mut scale,mut lead)=(1,1);for x in &mut out{*x=mul(*x,scale,self.r);lead=scale;scale=mul(scale,self.t0,self.r);}let inv=pow(lead,self.r-2,self.r);for x in &mut out{*x=mul(*x,inv,self.r)}Ok(out)
 }
}
include!("direct-adapter.rs");
include!("direct-checks.rs");

include!("opposite.rs");
