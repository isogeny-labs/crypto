impl Ring{
 fn section_product(&mut self,a:&[Q],b:&[Q],v:&Q)->Vec<Q>{
  let m=a.len();assert_eq!(b.len(),m);let mut out=vec![vec![0];m];
  for i in 0..m{for j in 0..m{let mut c=self.qm(&a[i],&b[j]);let k=i+j;if k>=m{c=self.qm(&c,v);}out[k%m]=self.qa(&out[k%m],&c);}}out
 }
 // Caller certifies prime r. Slot cap is m*deg(g), not peak process memory.
 fn sections(&mut self,m:usize,v:&Q,slot_cap:usize)->Result<(Vec<Q>,usize),&'static str>{
  if self.r<7||self.r>u64::MAX/2||m<2||m as u64>=self.r||self.g.len()<2||self.g.last()!=Some(&1)||self.g.iter().chain(v.iter()).any(|&x|x>=self.r){return Err("domain")}
  if m.checked_mul(self.g.len()-1).ok_or("slots")?>slot_cap{return Err("slots")}
  let v=self.rem(v);let mut acc=vec![vec![0];m];acc[0]=vec![1];let mut base=vec![vec![0];m];base[0]=vec![self.r-1];base[1]=vec![1];
  let(mut exponent,mut products)=((self.r-1)/2,0);
  while exponent>0{if exponent&1!=0{acc=self.section_product(&acc,&base,&v);products+=1;}exponent>>=1;if exponent>0{base=self.section_product(&base,&base,&v);products+=1;}}
  Ok((acc,products))
 }
}
fn section_checks(){let(mut sections,mut products)=(0,0);
 for(r,m)in[(7u64,3usize),(11,2),(13,4),(17,6),(41,5),(97,3),(281,5),(281,35)]{
  let h=(r-1)/2;let mut dense=vec![1];for _ in 0..h{let mut next=vec![0;dense.len()+1];for(j,&c)in dense.iter().enumerate(){next[j]=(next[j]+r-c)%r;next[j+1]=(next[j+1]+c)%r;}dense=next;}
  for g in [vec![0,1],vec![1,0,1],vec![4,r-4,1],vec![0,0,0,1]]{for base in [vec![0],vec![0,1],vec![2,1],vec![1]]{
   let mut ring=Ring{r,g:g.clone(),p:polynomial::Poly::new(r).unwrap()};let v=ring.rem(&base);let(got,n)=ring.sections(m,&v,1024).unwrap();assert_eq!(n,(64-h.leading_zeros()as usize)-1+h.count_ones()as usize);products+=n;
   for e in 0..m{let cs:Q=dense.iter().skip(e).step_by(m).copied().collect();let mut expected=vec![0];for &c in cs.iter().rev(){expected=ring.qm(&expected,&v);expected=ring.qa(&expected,&vec![c]);}assert_eq!(got[e],expected);sections+=1;}
   assert!(ring.sections(m,&v,0).is_err());assert!(ring.sections(1,&v,1024).is_err());
  }}
 }
 let(r,m)=(998244353u64,7usize);let h=(r-1)/2;let zeta=pow(3,(r-1)/m as u64,r);assert_eq!(pow(zeta,m as u64,r),1);assert!((1..m).all(|j|pow(zeta,j as u64,r)!=1));let mut large=0;let mut large_products=0;
 for x in [0,1,2,3,7,101,1234567,r-1]{let v=pow(x,m as u64,r);let mut ring=Ring{r,g:vec![(r-v)%r,1],p:polynomial::Poly::new(r).unwrap()};let(got,n)=ring.sections(m,&vec![0,1],1024).unwrap();large_products=n;
  let mut z=1;let signs:Vec<_>=(0..m).map(|_|{let s=pow((x+r-z)%r,h,r);z=mul(z,zeta,r);s}).collect();
  for e in 0..m{let root=pow(zeta,(e as u64+m as u64-h%m as u64)%m as u64,r);let(mut weight,mut sum)=(1,0);for &s in &signs{sum=(sum+mul(weight,s,r))%r;weight=mul(weight,root,r);}assert_eq!(sum,mul(mul(m as u64,pow(x,e as u64,r),r),got[e][0],r));large+=1;}
 }
 println!("section_equalities={sections} small_algebra_products={products} large_prime={r} large_projection_equalities={large} large_algebra_products_each={large_products} mode1_length={} slot_guards=pass",h/m as u64);
}
