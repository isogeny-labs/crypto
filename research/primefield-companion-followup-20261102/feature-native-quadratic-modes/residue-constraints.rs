/// Block Horner for F(inv*Z) modulo a borrowed monic nonconstant G.
struct ScaledResidue<'a>{r:u64,g:&'a[u64],powers:Vec<u64>,step:Vec<u64>}
impl<'a> ScaledResidue<'a>{
 fn new(r:u64,g:&'a[u64],inv:u64)->Result<Self,Error>{
  if g.len()<2||g.last()!=Some(&1)||inv==0||inv>=r||g.iter().any(|&x|x>=r){return Err(Error::Domain)}
  let n=g.len()-1;let mut powers=Vec::with_capacity(n);let mut v=1;
  for _ in 0..n{powers.push(v);v=mul(v,inv,r)}
  let step=g[..n].iter().map(|&x|mul((r-x)%r,v,r)).collect();Ok(Self{r,g,powers,step})
 }
 fn reduce(&self,p:&mut polynomial::Poly,f:&[u64])->Result<Vec<u64>,Error>{
  if f.is_empty()||f.iter().any(|&x|x>=self.r){return Err(Error::Domain)}
  let n=self.powers.len();let mut acc=vec![0];let mut first=true;
  for chunk in f.chunks(n).rev(){
   if !first{let product=p.mul(&acc,&self.step);assert!(product.len()<=2*n-1);acc=p.divrem(&product,self.g).map_err(|_|Error::Arithmetic)?.1;}
   first=false;acc.resize(n,0);
   for (i,&x) in chunk.iter().enumerate(){acc[i]=(acc[i]+mul(x,self.powers[i],self.r))%self.r;}
   while acc.len()>1&&acc.last()==Some(&0){acc.pop();}
  }Ok(acc)
 }
}
impl Modes{
 fn intersect_modes(&self,p:&mut polynomial::Poly,state:&mut ImageDomain,b:u64,f:&[u64])->Result<(),Error>{
  let r=self.r;
  if state.r!=r||state.m!=self.m||b==0||b>=r||f.len()!=self.coeff.len()||f.iter().any(|&x|x>=r){return Err(Error::Domain)}
  if state.degree()==0{return Ok(())}
  let a=mul(self.m as u64,pow(b,(r-1)/2,r),r);let inv=pow(pow(b,self.m as u64,r),r-2,r);
  let mut c0=if let Some(g)=&state.dense{ScaledResidue::new(r,g,inv)?.reduce(p,&self.coeff[0])?}
   else{let mut scale=1;self.coeff[0].iter().map(|&x|{let v=mul(x,scale,r);scale=mul(scale,inv,r);v}).collect()};
  for x in &mut c0{*x=mul(a,*x,r)}c0[0]=(c0[0]+r-f[0])%r;
  state.intersect(p,&c0)?;
  if self.m<3||state.degree()==0{return Ok(())}
  let c2=if let Some(g)=&state.dense{
   let ctx=ScaledResidue::new(r,g,inv)?;let u=ctx.reduce(p,&self.coeff[1])?;let mut v=ctx.reduce(p,&self.coeff[2])?;
   let square=p.mul(&u,&u);assert!(square.len()<=2*(g.len()-1)-1);
   let mut square=p.divrem(&square,g).map_err(|_|Error::Arithmetic)?.1;
   let left=mul(f[1],f[1],r);let right=mul(a,f[2],r);
   for x in &mut v{*x=mul(left,*x,r)}for x in &mut square{*x=mul(right,*x,r)}p.sub(&v,&square)
  }else{
   // For compiled modes, C0 can vanish identically only when all modes are constant.
   if self.coeff.iter().any(|c|c.len()!=1){return Err(Error::Arithmetic)}
   vec![(mul(mul(f[1],f[1],r),self.coeff[2][0],r)+r-mul(mul(a,f[2],r),mul(self.coeff[1][0],self.coeff[1][0],r),r))%r]
  };
  state.intersect(p,&c2)
 }
}
fn residue_constraint_checks(){
 let mut reductions=0;let mut intersections=0;let mut constant_controls=0;
 for r in [7u64,17,97,257]{for n in [1usize,2,3,8,17]{for len in [1,n,n+1,2*n-1,3*n+2,257]{for seed in 0..4u64{for inv in [1,2,r-1]{
  let mut g:Vec<_>=(0..n).map(|i|(seed*(i as u64+3))%r).collect();g.push(1);
  let f:Vec<_>=(0..len).map(|i|(seed*(i as u64+7))%r).collect();let mut p=polynomial::Poly::new(r).unwrap();
  let ctx=ScaledResidue::new(r,&g,inv).unwrap();let got=ctx.reduce(&mut p,&f).unwrap();let mut scale=1;
  let full:Vec<_>=f.iter().map(|&x|{let v=mul(x,scale,r);scale=mul(scale,inv,r);v}).collect();
  assert_eq!(got,p.divrem(&full,&g).unwrap().1);assert!(got.len()<=n);reductions+=1;
 }}}}}
 for r in [7u64,11,13,17,31,97]{for m in 2..r as usize{if (r-1)%m as u64!=0{continue}
  let zeta=(2..r).find(|&z|pow(z,m as u64,r)==1&&(1..m).all(|j|pow(z,j as u64,r)!=1)).unwrap();let modes=Modes::compile(r,m,zeta,m,256).unwrap();
  for k in 0..r{let mut old=ImageDomain::new(r,m).unwrap();let mut new=ImageDomain::new(r,m).unwrap();let mut p=polynomial::Poly::new(r).unwrap();
   for b in [1,2,r-1]{let mut z=1;let signs:Vec<_>=(0..m).map(|_|{let value=pow((k+r-mul(b,z,r))%r,(r-1)/2,r);z=mul(z,zeta,r);value}).collect();let f=modes.observed(&signs).unwrap();
    for c in modes.constraints(b,&f){old.intersect(&mut p,&c).unwrap();}
    modes.intersect_modes(&mut p,&mut new,b,&f).unwrap();assert_eq!(new.dense,old.dense);assert_eq!(new.degree(),old.degree());intersections+=1;
   }
  }
  if m==r as usize-1{let b=2;let a=mul(m as u64,pow(b,(r-1)/2,r),r);let f=vec![mul(a,modes.coeff[0][0],r),1,0];let mut p=polynomial::Poly::new(r).unwrap();let mut s=ImageDomain::new(r,m).unwrap();modes.intersect_modes(&mut p,&mut s,b,&f).unwrap();assert_eq!(s.dense,Some(vec![1]));constant_controls+=1;}
 }}
 println!("scaled_residue_comparisons={reductions} sequential_mode_intersections={intersections} implicit_constant_controls={constant_controls} bounded_product_assertions=true");
}
