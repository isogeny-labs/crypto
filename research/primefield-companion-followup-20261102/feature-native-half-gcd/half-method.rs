 fn matrix_mul(&mut self,a:&Matrix,b:&Matrix)->Matrix{
  std::array::from_fn(|i|std::array::from_fn(|j|{let x=self.mul(&a[i][0],&b[0][j]);let y=self.mul(&a[i][1],&b[1][j]);self.add(&x,&y)}))
 }
 fn matrix_apply(&mut self,m:&Matrix,a:&[u64],b:&[u64])->(Vec<u64>,Vec<u64>){
  let x=self.mul(&m[0][0],a);let y=self.mul(&m[0][1],b);let c=self.add(&x,&y);
  let x=self.mul(&m[1][0],a);let y=self.mul(&m[1][1],b);let d=self.add(&x,&y);(c,d)
 }
 fn step_matrix(&self,q:&[u64])->Matrix{[[vec![0],vec![1]],[vec![1],self.sub(&[0],q)]]}
 fn lifted(&mut self,mut batch:Batch,a:&[u64],b:&[u64],shift:usize)->Result<Batch,Error>{
  let(c,d)=self.matrix_apply(&batch.matrix,a,b);let len=batch.steps.len();
  for(i,t)in batch.steps.iter_mut().enumerate(){t.m+=shift;t.n+=shift;if i+1<len{t.k=Some(t.k.ok_or(Error::Invariant)?+shift);}else{t.k=degree(&d);}}
  if let Some(last)=batch.steps.last(){if degree(&c)!=Some(last.n)||c.last()!=Some(&last.leading){return Err(Error::Invariant);}}
  if d!=[0]&&c.len()<=d.len(){return Err(Error::Invariant);}
  batch.a=c;batch.b=d;Ok(batch)
 }
 fn half_gcd(&mut self,a:&[u64],b:&[u64])->Result<Batch,Error>{
  let d=a.len()-1;let half=(d+1)/2;
  if b==[0]||b.len()-1<half{return Ok(Batch{matrix:identity(),a:a.to_vec(),b:b.to_vec(),steps:vec![]});}
  if a.len()<=b.len(){return Err(Error::Invariant);}
  if d<=8{
   let(mut a,mut b,mut matrix,mut steps)=(a.to_vec(),b.to_vec(),identity(),vec![]);
   while b!=[0]&&b.len()-1>=half{let(q,r)=self.divrem(&a,&b)?;steps.push(Transition{m:a.len()-1,n:b.len()-1,k:degree(&r),leading:*b.last().unwrap()});let step=self.step_matrix(&q);matrix=self.matrix_mul(&step,&matrix);a=b;b=r;}
   return Ok(Batch{matrix,a,b,steps});
  }
  #[cfg(test)]BATCH_CALLS.with(|n|n.set(n.get()+1));
  let small=self.half_gcd(&a[half..],&b[half..])?;let mut first=self.lifted(small,a,b,half)?;
  if first.b==[0]||first.b.len()-1<half{return Ok(first);}
  let(q,r)=self.divrem(&first.a,&first.b)?;
  first.steps.push(Transition{m:first.a.len()-1,n:first.b.len()-1,k:degree(&r),leading:*first.b.last().unwrap()});
  let step=self.step_matrix(&q);let prefix=self.matrix_mul(&step,&first.matrix);
  if r==[0]||r.len()-1<half{return Ok(Batch{matrix:prefix,a:first.b,b:r,steps:first.steps});}
  let shift=2*half-(first.b.len()-1);
  let small=self.half_gcd(&first.b[shift..],&r[shift..])?;let second=self.lifted(small,&first.b,&r,shift)?;
  let matrix=self.matrix_mul(&second.matrix,&prefix);first.steps.extend(second.steps);
  if second.b!=[0]&&second.b.len()-1>=half{return Err(Error::Invariant);}
  Ok(Batch{matrix,a:second.a,b:second.b,steps:first.steps})
 }
 pub fn resultant_class(&mut self,a:&[u64],b:&[u64])->Result<ResultantClass,Error>{
  let order=1u64<<(self.p-1).trailing_zeros();if order>4{return Err(Error::Domain);}
  let(mut a,mut b)=(self.trim(a),self.trim(b));if a==[0]||b==[0]{return self.image_class(0);}
  let(mut acc,mut negative)=(1,false);
  loop{
   let(m,n)=(a.len()-1,b.len()-1);
   if n==0{for _ in 0..(m as u64%order){acc=self.fm(acc,b[0]);}break;}
   if m<n{if m&1==1&&n&1==1{negative=!negative;}std::mem::swap(&mut a,&mut b);continue;}
   let(next_a,next_b,steps)=if m>8&&m>n&&n>=(m+1)/2{
    let batch=self.half_gcd(&a,&b)?;if batch.steps.is_empty(){return Err(Error::Invariant);}(batch.a,batch.b,batch.steps)
   }else{let(_,r)=self.divrem(&a,&b)?;let step=Transition{m,n,k:degree(&r),leading:*b.last().unwrap()};(b,r,vec![step])};
   for t in steps{let Some(k)=t.k else{return self.image_class(0);};if k>=t.n||t.n>t.m{return Err(Error::Invariant);}
    for _ in 0..((t.m-k)as u64%order){acc=self.fm(acc,t.leading);}if t.m&1==1&&t.n&1==1{negative=!negative;}
   }
   a=next_a;b=next_b;
  }
  let class=self.image_class(acc)?;Ok(if negative{class.negate()}else{class})
 }
