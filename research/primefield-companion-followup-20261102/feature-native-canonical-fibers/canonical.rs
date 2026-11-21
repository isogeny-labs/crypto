fn canonical_rem(p:&mut polynomial::Poly,a:&[u64],m:&[u64])->Result<Vec<u64>,Error>{Ok(p.divrem(a,m).map_err(|_|Error::Arithmetic)?.1)}
fn canonical_inverse(p:&mut polynomial::Poly,a:&[u64],m:&[u64],r:u64)->Result<Vec<u64>,Error>{
 if m.len()<2{return Err(Error::Domain)}let(mut x,mut y)=(m.to_vec(),canonical_rem(p,a,m)?);let(mut s,mut t)=(vec![0],vec![1]);
 while y!=[0]{let(q,z)=p.divrem(&x,&y).map_err(|_|Error::Arithmetic)?;let qt=p.mul(&q,&t);let next=p.sub(&s,&qt);let next=canonical_rem(p,&next,m)?;x=y;y=z;s=t;t=next;}
 if x.len()!=1||x[0]==0{return Err(Error::Arithmetic)}let inv=pow(x[0],r-2,r);let out:Vec<_>=s.iter().map(|&v|mul(v,inv,r)).collect();let check=p.mul(a,&out);if canonical_rem(p,&check,m)?!=[1]{return Err(Error::Arithmetic)}Ok(out)
}
struct CanonicalFiber{r:u64,both:Vec<u64>,one:Vec<u64>,root:Vec<u64>,inverses:usize,crt_merges:usize}
impl CanonicalFiber{
 fn new(r:u64,g:&[u64])->Result<Self,Error>{
  if r<7||r>u64::MAX/2||g.is_empty()||g.last()!=Some(&1)||g.iter().any(|&x|x>=r)||squarefree(r,g)?!=g{return Err(Error::Domain)}
  let mut p=polynomial::Poly::new(r).map_err(|_|Error::Domain)?;let one=p.gcd_monic(g,&[r-4,0,1],g.len()+2).map_err(|_|Error::Arithmetic)?.polynomial;let(both,rem)=p.divrem(g,&one).map_err(|_|Error::Arithmetic)?;if rem!=[0]{return Err(Error::Arithmetic)}let root=canonical_rem(&mut p,&[0,(r+1)/2],&one)?;
  Ok(Self{r,both,one,root,inverses:0,crt_merges:0})
 }
 fn support(&self,p:&mut polynomial::Poly)->Vec<u64>{p.mul(&self.both,&self.one)}
 fn keep(&mut self,p:&mut polynomial::Poly,pair:&Quad)->Result<(),Error>{
  let r=self.r;let(a,b)=(&pair.0[0],&pair.0[1]);if a.is_empty()||b.is_empty()||a.iter().chain(b.iter()).any(|&x|x>=r){return Err(Error::Domain)}let bound=self.both.len()+self.one.len()+a.len()+b.len()+2;
  // Existing selected roots only need substitution into the new equation.
  let product=p.mul(b,&self.root);let value=p.add(a,&product);let old_one=p.gcd_monic(&self.one,&value,bound).map_err(|_|Error::Arithmetic)?.polynomial;let old_root=canonical_rem(p,&self.root,&old_one)?;
  // On an unoriented fiber: B=0 retains both iff A=0; B!=0 selects -A/B.
  let zero_b=p.gcd_monic(&self.both,b,bound).map_err(|_|Error::Arithmetic)?.polynomial;let both=p.gcd_monic(&zero_b,a,bound).map_err(|_|Error::Arithmetic)?.polynomial;let(nonzero_b,rem)=p.divrem(&self.both,&zero_b).map_err(|_|Error::Arithmetic)?;if rem!=[0]{return Err(Error::Arithmetic)}
  let aa=p.mul(a,a);let bb=p.mul(b,b);let ab=p.mul(a,b);let mut wab=vec![0];wab.extend_from_slice(&ab);let norm=p.add(&p.add(&aa,&bb),&wab);let new_one=p.gcd_monic(&nonzero_b,&norm,bound*2).map_err(|_|Error::Arithmetic)?.polynomial;
  let new_root=if new_one.len()==1{vec![0]}else{self.inverses+=1;let inverse=canonical_inverse(p,b,&new_one,r)?;let negative:Vec<_>=a.iter().map(|&x|(r-x)%r).collect();let value=p.mul(&negative,&inverse);canonical_rem(p,&value,&new_one)?};
  let(root,one)=if old_one.len()==1{(new_root,new_one)}else if new_one.len()==1{(old_root,old_one)}else{
   self.inverses+=1;self.crt_merges+=1;let inverse=canonical_inverse(p,&old_one,&new_one,r)?;let delta=p.sub(&new_root,&old_root);let product=p.mul(&delta,&inverse);let correction=canonical_rem(p,&product,&new_one)?;let shifted=p.mul(&old_one,&correction);let root=p.add(&old_root,&shifted);let one=p.mul(&old_one,&new_one);(root,one)
  };
  if one.len()>1&&root.len()>=one.len(){return Err(Error::Arithmetic)}self.both=both;self.one=one;self.root=root;Ok(())
 }
 fn lift(&self,orientation:&ReciprocalMode,u:u64,cap:usize,ring:&mut QuadRing)->Result<Vec<u64>,Error>{
  let both=orientation.lift(&self.both,u,cap)?;let one=orientation.lift(&self.one,u,cap)?;let negative:Vec<_>=self.root.iter().map(|&x|(self.r-x)%self.r).collect();let one=ring.filter_lift(&Quad([negative,vec![1]]),&one)?;
  if both.len()-1+one.len()-1>cap{return Err(Error::Limit)}Ok(ring.p.mul(&both,&one))
 }
}
struct CanonicalBranch{u:u64,ring:QuadRing,canonical:CanonicalFiber}
impl CanonicalBranch{
 fn keep(&mut self,pair:Quad)->Result<(),Error>{self.canonical.keep(&mut self.ring.p,&pair)?;self.ring.g=self.canonical.support(&mut self.ring.p);Ok(())}
}
include!("canonical-image.rs");
include!("canonical-adapter.rs");
include!("canonical-checks.rs");
