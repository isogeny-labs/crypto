 fn project_power(&self,mut a:u64,mut e:u64)->u64{let mut v=1;while e!=0{if e&1!=0{v=self.fm(v,a);}a=self.fm(a,a);e>>=1;}v}
 /// Standard Res(A,B), projected to the 2-primary image of order at most four.
 pub fn projected_resultant(&mut self,a:&[u64],b:&[u64])->Result<u64,Error>{
  let order=1u64<<(self.p-1).trailing_zeros();if order>4{return Err(Error::Domain);}
  let(mut a,mut b)=(self.trim(a),self.trim(b));if a==[0]||b==[0]{return Ok(0);}
  let(mut acc,mut negative)=(1,false);
  loop{
   let(m,n)=(a.len()-1,b.len()-1);
   if n==0{for _ in 0..(m as u64%order){acc=self.fm(acc,b[0]);}break;}
   if m<n{if m&1==1&&n&1==1{negative=!negative;}std::mem::swap(&mut a,&mut b);continue;}
   let(_,rem)=self.divrem(&a,&b)?;if rem==[0]{return Ok(0);}
   let k=rem.len()-1;let e=(m-k)as u64%order;
   for _ in 0..e{acc=self.fm(acc,*b.last().ok_or(Error::Invariant)?);}
   if m&1==1&&n&1==1{negative=!negative;}
   a=b;b=rem;
  }
  let v=self.project_power(acc,(self.p-1)/order);Ok(if negative{self.p-v}else{v})
 }
 /// Build only the product polynomial: no retained subproduct tree nodes.
 pub fn root_product(&mut self,roots:&[u64])->Result<Vec<u64>,Error>{
  if roots.is_empty(){return Err(Error::EmptyTree);}
  if roots.len()==1{return Ok(vec![(self.p-roots[0]%self.p)%self.p,1]);}
  let k=roots.len()/2;let a=self.root_product(&roots[..k])?;let b=self.root_product(&roots[k..])?;Ok(self.mul(&a,&b))
 }
 pub fn root_projected_resultant(&mut self,roots:&[u64],points:&[u64])->Result<u64,Error>{
  let f=self.root_product(roots)?;let g=self.root_product(points)?;
  // Existing source convention is product_j f(points[j]) = Res(g,f).
  self.projected_resultant(&g,&f)
 }
