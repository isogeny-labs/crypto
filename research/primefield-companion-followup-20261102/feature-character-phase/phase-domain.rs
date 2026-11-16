// S_j(W)=U_j(W/2). Return (S_j,S_(j-1)), with S_-1=0.
fn second_pair(p:&mut polynomial::Poly,j:usize)->(Vec<u64>,Vec<u64>){
 if j==0{return(vec![1],vec![0])}
 let(a,b)=second_pair(p,j/2);let aa=p.mul(&a,&a);let bb=p.mul(&b,&b);let even=p.sub(&aa,&bb);
 if j%2==0{let twice=p.add(&a,&a);let wb=p.mul(&[0,1],&b);let factor=p.sub(&twice,&wb);(even,p.mul(&b,&factor))}
 else{let wa=p.mul(&[0,1],&a);let twice=p.add(&b,&b);let factor=p.sub(&wa,&twice);(p.mul(&a,&factor),even)}
}
std::thread_local!{static DIRECT_DOMAINS:std::cell::Cell<(usize,usize)>=const{std::cell::Cell::new((0,0))};}
fn direct_phase_domains(r:u64,n:usize,i:u64,normalized:u64)->Result<Vec<(u64,Vec<u64>)>,Error>{
 if r<7||r>u64::MAX/2||n==0||n as u64>=r||i>=r||mul(i,i,r)!=r-1||(normalized!=1&&normalized!=r-1){return Err(Error::Domain)}
 let mut p=polynomial::Poly::new(r).map_err(|_|Error::Domain)?;
 if normalized==1{
  let(a,b)=second_pair(&mut p,n/2);
  let(plus,minus)=if n%2==0{let plus=p.mul(&[r-4,0,1],&b);let twice=p.add(&a,&a);let wb=p.mul(&[0,1],&b);(plus,p.sub(&twice,&wb))}
   else{let sum=p.add(&a,&b);let difference=p.sub(&a,&b);(p.mul(&[r-2,1],&sum),p.mul(&[2,1],&difference))};
  DIRECT_DOMAINS.with(|c|{let(a,b)=c.get();c.set((a+1,b))});Ok(vec![(1,plus),(r-1,minus)])
 }else{
  // D_n has simple roots for 0<n<r in odd characteristic. Both orientations share W support.
  let dn=initial_dickson(&mut p,n);DIRECT_DOMAINS.with(|c|{let(a,b)=c.get();c.set((a,b+1))});Ok(vec![(i,dn.clone()),(r-i,dn)])
 }
}
