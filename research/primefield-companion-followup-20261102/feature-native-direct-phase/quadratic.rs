#[derive(Clone)]struct Quad([Vec<u64>;2]);
struct QuadRing{r:u64,g:Vec<u64>,p:polynomial::Poly}
impl QuadRing{
 fn new(r:u64,g:&[u64])->Result<Self,Error>{if g.len()<2||g.last()!=Some(&1)||g.iter().any(|&x|x>=r){return Err(Error::Domain)}Ok(Self{r,g:g.to_vec(),p:polynomial::Poly::new(r).map_err(|_|Error::Domain)?})}
 fn rem(&mut self,a:&[u64])->Result<Vec<u64>,Error>{Ok(self.p.divrem(a,&self.g).map_err(|_|Error::Arithmetic)?.1)}
 fn add(&mut self,a:&Quad,b:&Quad)->Quad{Quad([self.p.add(&a.0[0],&b.0[0]),self.p.add(&a.0[1],&b.0[1])])}
 fn sub(&mut self,a:&Quad,b:&Quad)->Quad{Quad([self.p.sub(&a.0[0],&b.0[0]),self.p.sub(&a.0[1],&b.0[1])])}
 fn scale(&self,a:&Quad,c:u64)->Quad{Quad(std::array::from_fn(|i|a.0[i].iter().map(|&x|mul(x,c,self.r)).collect()))}
 fn multiply(&mut self,a:&Quad,b:&Quad)->Result<Quad,Error>{
  let ac=self.p.mul(&a.0[0],&b.0[0]);let bd=self.p.mul(&a.0[1],&b.0[1]);let sa=self.p.add(&a.0[0],&a.0[1]);let sb=self.p.add(&b.0[0],&b.0[1]);let cross=self.p.mul(&sa,&sb);let cross=self.p.sub(&self.p.sub(&cross,&ac),&bd);
  let mut wbd=vec![0];wbd.extend_from_slice(&bd);let x=self.p.sub(&ac,&bd);let y=self.p.add(&cross,&wbd);Ok(Quad([self.rem(&x)?,self.rem(&y)?]))
 }
 fn power_v(&mut self,mut exponent:usize)->Result<Quad,Error>{let mut a=Quad([vec![1],vec![0]]);let mut b=Quad([vec![0],vec![1]]);while exponent>0{if exponent&1!=0{a=self.multiply(&a,&b)?;}exponent>>=1;if exponent>0{b=self.multiply(&b,&b)?;}}Ok(a)}
 fn block(&mut self,reader:&mut CoefficientReader<'_>,length:usize)->Result<(Quad,Quad),Error>{
  if length==0{return Err(Error::Domain)}if length==1{return Ok((Quad([vec![reader.next()?],vec![0]]),Quad([vec![0],vec![1]])))}
  let(a,ap)=self.block(reader,length/2)?;let(b,bp)=self.block(reader,length-length/2)?;let ab=self.multiply(&a,&bp)?;let value=self.add(&ab,&b);let power=self.multiply(&ap,&bp)?;Ok((value,power))
 }
 fn mode(&mut self,modes:&RecoveryModes,e:usize)->Result<Quad,Error>{let stream=modes.stream(e)?;let length=stream.left;let mut reader=CoefficientReader{stream,buffer:Vec::new(),at:0,remaining:length};Ok(self.block(&mut reader,length)?.0)}
 fn norm(&mut self,a:&Quad)->Result<Vec<u64>,Error>{let aa=self.p.mul(&a.0[0],&a.0[0]);let bb=self.p.mul(&a.0[1],&a.0[1]);let ab=self.p.mul(&a.0[0],&a.0[1]);let mut wab=vec![0];wab.extend_from_slice(&ab);let out=self.p.add(&self.p.add(&aa,&bb),&wab);self.rem(&out)}
 fn common(&mut self,a:&Quad,b:&Quad)->Result<Vec<u64>,Error>{
  let na=self.norm(a)?;let nb=self.norm(b)?;let ad=self.p.mul(&a.0[0],&b.0[1]);let bc=self.p.mul(&a.0[1],&b.0[0]);let det=self.p.sub(&ad,&bc);let mut g=self.g.clone();
  for c in [na,nb,det]{g=self.p.gcd_monic(&g,&c,self.g.len()*2).map_err(|_|Error::Arithmetic)?.polynomial;}Ok(g)
 }
 fn filter_lift(&mut self,pair:&Quad,lift:&[u64])->Result<Vec<u64>,Error>{
  if lift.len()==1{return Ok(vec![1])}let r=self.r;let inv=pow(lift[0],r-2,r);if mul(inv,lift[0],r)!=1{return Err(Error::Arithmetic)}
  let iv:Vec<_>=lift[1..].iter().map(|&x|mul((r-x)%r,inv,r)).collect();let w=self.p.add(&iv,&[0,1]);let w=self.p.divrem(&w,lift).map_err(|_|Error::Arithmetic)?.1;
  let mut values=[vec![0],vec![0]];for j in 0..2{for &c in pair.0[j].iter().rev(){let product=self.p.mul(&values[j],&w);values[j]=self.p.divrem(&product,lift).map_err(|_|Error::Arithmetic)?.1;values[j][0]=(values[j][0]+c)%r;}}
  let bv=self.p.mul(&values[1],&[0,1]);let equation=self.p.add(&values[0],&bv);Ok(self.p.gcd_monic(lift,&equation,lift.len()*2).map_err(|_|Error::Arithmetic)?.polynomial)
 }
}
fn squarefree(r:u64,g:&[u64])->Result<Vec<u64>,Error>{if g.len()<2{return Ok(g.to_vec())}let mut p=polynomial::Poly::new(r).map_err(|_|Error::Domain)?;let derivative:Vec<_>=(1..g.len()).map(|j|mul(j as u64,g[j],r)).collect();let common=p.gcd_monic(g,&derivative,g.len()).map_err(|_|Error::Arithmetic)?.polynomial;let(q,rem)=p.divrem(g,&common).map_err(|_|Error::Arithmetic)?;if rem!=[0]{return Err(Error::Arithmetic)}Ok(q)}
std::thread_local!{static QUADRATIC_COUNTS:std::cell::Cell<(usize,usize,usize)>=const{std::cell::Cell::new((0,0,0))};static QUADRATIC_DEGREES:std::cell::Cell<(usize,usize,usize,usize)>=const{std::cell::Cell::new((0,0,0,0))};}
impl ReciprocalMode{
 fn quadratic_constraint(&self,modes:&RecoveryModes,a:u64,t:u64,f:&[u64],i:u64,cap:usize)->Result<Vec<u64>,Error>{
  let r=self.r;if modes.r!=r||modes.m<3||((r-1)/modes.m as u64)!=4*self.n as u64||f.len()!=3||f.iter().any(|&x|x>=r)||a==0||a>=r||t==0||t>=r||i>=r||mul(i,i,r)!=r-1||pow(t,4*self.n as u64,r)!=1{return Err(Error::Domain)}
  let c=mul(f[0],pow(a,r-2,r),r);let h=(r-1)/2;let c1=r-h;let c2=mul(mul(h,h-1,r),(r+1)/2,r);let left=mul(f[1],f[1],r);let right=mul(a,f[2],r);
  let zero=f[0]==a&&mul(left,c2,r)==mul(right,mul(c1,c1,r),r);let mut out=if zero{if cap==0{return Err(Error::Limit)}vec![0,1]}else{vec![1]};let mut p=polynomial::Poly::new(r).map_err(|_|Error::Domain)?;
  for u in [1,r-1,i,r-i]{
   let g=squarefree(r,&self.branch(u,c)?)?;if g.len()==1{continue}let mut ring=QuadRing::new(r,&g)?;
   let mut phase=ring.power_v(self.n)?;phase.0[0][0]=(phase.0[0][0]+r-u)%r;
   let f1=ring.mode(modes,1)?;let f2=ring.mode(modes,2)?;let sq=ring.multiply(&f1,&f1)?;let l=ring.scale(&f2,left);let rr=ring.scale(&sq,right);let equation=ring.sub(&l,&rr);
   let kept=ring.common(&phase,&equation)?;QUADRATIC_DEGREES.with(|c|{let(b,x,y,s)=c.get();c.set((b+1,x+g.len()-1,y+kept.len()-1,s+usize::from(kept.len()<g.len())))});
   let lift=self.lift(&kept,u,cap)?;let filtered=ring.filter_lift(&equation,&lift)?;if out.len()-1+filtered.len()-1>cap{return Err(Error::Limit)}out=p.mul(&out,&filtered);
  }
  let(mut scale,mut lead)=(1,1);for x in &mut out{*x=mul(*x,scale,r);lead=scale;scale=mul(scale,t,r);}let inv=pow(lead,r-2,r);for x in &mut out{*x=mul(*x,inv,r)}Ok(out)
 }
}
impl RecoveryModes{
 fn quadratic_initial(&self,a:u64,t:u64,f:&[u64],cap:usize)->Result<Option<Vec<u64>>,Error>{
  if self.m<3||((self.r-1)/self.m as u64)%4!=0{return Ok(None)}let mut fourth=None;for z in 2..self.r.min(66){let i=pow(z,(self.r-1)/4,self.r);if mul(i,i,self.r)==self.r-1{fourth=Some(i);break}}let Some(i)=fourth else{return Ok(None)};
  let model=ReciprocalMode::build(self,((self.r-1)/(4*self.m as u64))as usize)?.ok_or(Error::Arithmetic)?;QUADRATIC_COUNTS.with(|c|{let(a,b,d)=c.get();c.set((a+1,b,d))});
  match model.quadratic_constraint(self,a,t,f,i,cap){Ok(g)=>{QUADRATIC_COUNTS.with(|c|{let(a,b,d)=c.get();c.set((a,b+1,d))});Ok(Some(g))},Err(Error::Limit)=>{QUADRATIC_COUNTS.with(|c|{let(a,b,d)=c.get();c.set((a,b,d+1))});Ok(None)},Err(e)=>Err(e)}
 }
}
include!("reciprocal-control.rs");
include!("quadratic-checks.rs");

include!("persistent.rs");
