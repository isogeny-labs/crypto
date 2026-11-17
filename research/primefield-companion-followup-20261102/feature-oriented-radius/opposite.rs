// Opposite-radius C0 supports; public observations only, no root enumeration.
fn parity_lift(g:&[u64])->Vec<u64>{let mut v=vec![0;2*(g.len()-1)+1];for(j,&x)in g.iter().enumerate(){v[2*j]=x;}v}
impl PhaseFold{
 fn opposite_branch(&self,k:usize,plus:u64,minus:u64)->Result<Vec<u64>,Error>{
  let r=self.r;if self.n%2!=0||k>=4||plus>=r||minus>=r{return Err(Error::Domain)}
  let half=self.n/2;let inv2=(r+1)/2;let s=mul((plus+minus)%r,inv2,r);let d=mul((plus+r-minus)%r,inv2,r);
  let mut p=polynomial::Poly::new(r).map_err(|_|Error::Domain)?;
  let even:Vec<_>=self.residues[k].iter().step_by(2).copied().collect();
  let odd:Vec<_>=self.residues[k].iter().skip(1).step_by(2).copied().collect();
  // Reuse sparse-binomial gcd; this temporary PhaseFold is in Y, not V.
  let small=PhaseFold{r,n:half,phases:self.phases,residues:std::array::from_fn(|_|even.clone()),reads:0};
  let mut g=small.branch(k,s)?;if g.len()==1{return Ok(g)}
  let o=canonical_rem(&mut p,&odd,&g)?;
  if d==0{g=p.gcd_monic(&g,&o,self.n+1).map_err(|_|Error::Arithmetic)?.polynomial;return Ok(parity_lift(&g))}
  let osq=p.mul(&o,&o);let mut norm=vec![0];norm.extend(osq);norm[0]=(r-mul(d,d,r))%r;
  let norm=canonical_rem(&mut p,&norm,&g)?;g=p.gcd_monic(&g,&norm,self.n+2).map_err(|_|Error::Arithmetic)?.polynomial;
  if g.len()==1{return Ok(g)}
  let o=canonical_rem(&mut p,&o,&g)?;let lifted=parity_lift(&g);let mut oriented=vec![r-d];oriented.extend(parity_lift(&o));
  Ok(p.gcd_monic(&lifted,&oriented,self.n+2).map_err(|_|Error::Arithmetic)?.polynomial)
 }
}
std::thread_local!{static OPPOSITE_COUNTS:std::cell::Cell<(usize,usize,usize,usize)>=const{std::cell::Cell::new((0,0,0,0))};}
impl DirectImage{
 fn opposite(modes:&RecoveryModes,b:u64,plus:&[u64],minus:&[u64],i:u64)->Result<Self,Error>{
  let r=modes.r;let m=modes.m;let n=((r-1)/(4*m as u64))as usize;
  if m<3||m%2==0||n%2!=0||b==0||b>=r||plus.len()!=3||minus.len()!=3||plus.iter().chain(minus).any(|&v|v>=r){return Err(Error::Domain)}
  let a=mul(m as u64,pow(b,(r-1)/2,r),r);let ia=pow(a,r-2,r);let fold=PhaseFold::build(modes,i,n)?;
  let mut branches=Vec::new();for k in 0..4{let g=fold.opposite_branch(k,mul(plus[0],ia,r),mul(minus[0],ia,r))?;if g.len()>1{branches.push(DirectBranch{u:fold.phases[k],g})}}
  let t0=pow(pow(b,m as u64,r),r-2,r);let mut image=Self{r,m,t0,inv_t0:pow(t0,r-2,r),branches,zero:true,rounds:0};
  image.absorb(modes,b,plus,true)?;image.absorb(modes,r-b,minus,true)?;
  OPPOSITE_COUNTS.with(|c|{let(a,b,d,e)=c.get();c.set((a+1,b,d,e))});Ok(image)
 }
}
include!("opposite-adapter.rs");
include!("opposite-checks.rs");

include!("doubled.rs");
