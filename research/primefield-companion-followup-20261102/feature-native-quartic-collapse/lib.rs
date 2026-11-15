//! Compile a signature-fiber mean into a polynomial character decoder.
mod target;
#[path="../feature-native-centered-bitplanes/decoder.rs"]mod decoder;
use std::collections::BTreeMap;
fn mul(a:u64,b:u64,r:u64)->u64{((a as u128*b as u128)%r as u128)as u64}
fn pow(mut a:u64,mut n:u64,r:u64)->u64{let mut v=1;while n>0{if n&1!=0{v=mul(v,a,r);}a=mul(a,a,r);n>>=1;}v}
fn signature(x:u64,r:u64,q:usize)->Vec<i8>{(0..q).map(|a|{let z=(x+a as u64)%r;if z==0{0}else if pow(z,(r-1)/2,r)==1{1}else{-1}}).collect()}
#[derive(Debug)]pub struct Collapsed{pub terms:Vec<(u64,Vec<usize>)>,pub fibers:Vec<Vec<u64>>,pub rank:usize,pub columns:usize}
/// Caller certifies primality. The complete public coefficient field is bounded
/// and enumerated at compile time; no unknown target or curve point is input.
pub fn construct(r:u64,q:usize,degree:usize,max_rows:usize,max_columns:usize)->Result<Collapsed,String>{
 if r<3||r>u64::MAX/2||r&1==0||q==0||q>64||q as u64>=r||!(3..=4).contains(&degree){return Err("domain".into());}
 let n=usize::try_from(r).map_err(|_|"limit")?;if n>max_rows{return Err("limit".into());}
 let mut groups:BTreeMap<Vec<i8>,Vec<u64>>=BTreeMap::new();for x in 0..r{groups.entry(signature(x,r,q)).or_default().push(x);}
 let fibers:Vec<_>=groups.into_values().filter(|g|g.len()>1).collect();let center=mul((q-1)as u64,(r+1)/2,r);let mut rhs=vec![0;n];rhs[1]=1;
 for fiber in &fibers{let total=fiber.iter().fold(0,|s,&x|(s+x)%r);let inv=pow(fiber.len()as u64,r-2,r);if mul(inv,fiber.len()as u64,r)!=1{return Err("fiber size".into());}let mean=mul(total,inv,r);
  for &x in fiber{let c=(mean+r-x)%r;let t=(x+center)%r;let mut power=1;for k in(0..n).rev(){rhs[k]=(rhs[k]+r-mul(c,power,r))%r;power=mul(power,t,r);}rhs[0]=(rhs[0]+c)%r;}
 }
 if rhs.iter().step_by(2).any(|&x|x!=0){return Err("parity".into());}
 let p=target::construct_target(r,q,degree,&rhs,max_rows,max_columns).map_err(|e|format!("{e:?}"))?;
 Ok(Collapsed{terms:p.terms,fibers,rank:p.rank,columns:p.columns})
}
#[cfg(test)]mod tests{
 use super::*;
 fn check(r:u64,q:usize,p:&Collapsed){let compiled=decoder::Program::new(r,q,&p.terms).unwrap();for x in 0..r{let fiber=p.fibers.iter().find(|v|v.contains(&x));let expected=if let Some(f)=fiber{mul(f.iter().fold(0,|s,&v|(s+v)%r),pow(f.len()as u64,r-2,r),r)}else{x};let signs=signature(x,r,q);assert_eq!(compiled.evaluate(|a|signs[a]),Ok(expected));}}
 #[test]fn fixed_shorter_prefix(){
  let cubic=construct(479,14,3,479,2500);println!("q14_cubic={:?}",cubic.as_ref().map(|p|(p.rank,p.terms.len())));
  let p=construct(479,14,4,479,2500).unwrap();check(479,14,&p);
  println!("q14_quartic rank={} columns={} terms={} fibers={:?}",p.rank,p.columns,p.terms.len(),p.fibers);
  if let Ok(dir)=std::env::var("COLLAPSE_OUT"){let mut text=String::new();for(c,l)in &p.terms{text.push_str(&format!("{}\t{}\n",c,l.iter().map(|x|x.to_string()).collect::<Vec<_>>().join(",")));}std::fs::write(format!("{dir}/program.tsv"),text).unwrap();std::fs::write(format!("{dir}/fibers.tsv"),p.fibers.iter().map(|v|v.iter().map(|x|x.to_string()).collect::<Vec<_>>().join("\t")+"\n").collect::<String>()).unwrap();}
 }
 #[test]fn retained_cubic_and_small_fields(){let p=construct(479,15,3,479,2500).unwrap();check(479,15,&p);assert_eq!(p.fibers.len(),2);for(r,q)in[(5,3),(7,4),(11,5),(17,7),(31,8)]{let p=construct(r,q,4,31,2500).unwrap();check(r,q,&p);}}
 #[test]fn invalid_inputs(){assert!(construct(479,14,4,478,2500).is_err());assert!(construct(479,14,5,479,2500).is_err());assert!(construct(8,3,4,8,100).is_err());assert!(construct(479,14,4,479,1).is_err());}
}
