use hec_native_ic::{fp::Fp,rng::Rng};
pub type Row=Vec<(usize,u64)>;
fn dot(a:&[u64],b:&[u64],f:&Fp)->u64 { a.iter().zip(b).fold(0,|z,(&a,&b)|f.add(z,f.mul(a,b))) }
fn mv(rows:&[Row],d:&[u64],x:&[u64],f:&Fp)->Vec<u64> {
 let mut out=vec![0;x.len()];
 for (r,&d) in rows.iter().zip(d) {
  let s=f.mul(d,r.iter().fold(0,|z,&(c,a)|f.add(z,f.mul(a,x[c]))));
  for &(c,a) in r {out[c]=f.add(out[c],f.mul(a,s));}
 } out
}
pub fn valid(rows:&[Row],x:&[u64],ell:u64)->bool {
 let f=Fp::new(ell);x.iter().any(|&a|a!=0)&&rows.iter().all(|r|r.iter().all(|&(c,a)|c<x.len()&&a<ell)&&r.iter().fold(0,|z,&(c,a)|f.add(z,f.mul(a,x[c])))==0)
}
pub fn kernel(rows:&[Row],n:usize,ell:u64,seed:u64,retries:usize)->Option<Vec<u64>> {
 let f=Fp::new(ell);let mut rng=Rng::new(seed);
 for attempt in 0..retries {
  let d:Vec<_>=(0..rows.len()).map(|_|1+rng.below(ell-1)).collect();
  let b:Vec<_>=(0..n).map(|_|rng.below(ell)).collect();
  let y=mv(rows,&d,&b,&f);let mut x=vec![0;n];let mut w=y.clone();let mut vp=vec![0;n];let mut wp=vec![0;n];let mut sp=0;
  let mut terminated=false;
  for iteration in 0..=n+5 {
   if w.iter().all(|&a|a==0) {terminated=true;break;}
   let v=mv(rows,&d,&w,&f);let s=dot(&w,&v,&f);if s==0 {break;}
   let si=f.inv(s);let cy=f.mul(dot(&w,&y,&f),si);
   for i in 0..n {x[i]=f.add(x[i],f.mul(cy,w[i]));}
   let a=f.mul(dot(&v,&v,&f),si);
   let mut wn:Vec<_>=(0..n).map(|i|f.sub(v[i],f.mul(a,w[i]))).collect();
   if iteration>0 {let b=f.mul(dot(&v,&vp,&f),f.inv(sp));for i in 0..n {wn[i]=f.sub(wn[i],f.mul(b,wp[i]));}}
   wp=w;w=wn;vp=v;sp=s;
  }
  if !terminated {println!("LANCZOS_RETRY attempt={attempt} reason=breakdown_or_cap");continue;}
  let mut z:Vec<_>=(0..n).map(|i|f.sub(x[i],b[i])).collect();
  if valid(rows,&z,ell) {let inv=f.inv(*z.iter().find(|&&a|a!=0).unwrap());for a in &mut z {*a=f.mul(*a,inv);}println!("SPARSE_KERNEL rows={} columns={n} entries={} attempt={attempt} exact_original_residual=true dense_candidate_allocation=false",rows.len(),rows.iter().map(Vec::len).sum::<usize>());return Some(z);}
  println!("LANCZOS_RETRY attempt={attempt} reason=zero_or_original_residual");
 }
 None
}
