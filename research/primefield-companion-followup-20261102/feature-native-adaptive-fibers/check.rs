//! Separate artifact checker: ordinary repeated-factor expansion and square-set
//! characters. Imports no producer, source, solver, or bitplane decoder.
use std::collections::BTreeSet;
fn times_linear(v:Vec<u64>,a:u64)->Vec<u64>{let mut next=vec![0;v.len()+1];for(i,u)in v.into_iter().enumerate(){next[i]=(next[i]+u*a)%479;next[i+1]=(next[i+1]+u)%479;}next}
fn main(){
 let path=std::env::args().nth(1).expect("program.tsv");let text=std::fs::read_to_string(path).unwrap();
 let mut terms=vec![];let mut labels=BTreeSet::new();let mut expanded=vec![0u64;718];
 for line in text.lines(){let(c,s)=line.split_once('\t').unwrap();let c:u64=c.parse().unwrap();assert!((1..479).contains(&c));let offsets:Vec<usize>=s.split(',').filter(|s|!s.is_empty()).map(|s|s.parse().unwrap()).collect();assert!(offsets.len()<=3);assert!(offsets.iter().all(|&a|a<15));assert!(offsets.windows(2).all(|w|w[0]<w[1]));assert!(labels.insert(offsets.clone()));
  let mut v=vec![1u64];for &a in &offsets{for _ in 0..239{v=times_linear(v,a as u64);}}
  for(i,&v)in v.iter().enumerate(){expanded[i]=(expanded[i]+c*v)%479;}terms.push((c,offsets));
 }
 assert!(!terms.is_empty());let mut reduced=vec![0u64;479];for(e,&c)in expanded.iter().enumerate(){let j=if e==0{0}else{1+(e-1)%478};reduced[j]=(reduced[j]+c)%479;}
 let mut expected=vec![0u64;479];expected[1]=1;
 for(t,c)in [(25,74),(173,405),(292,74),(440,405)]{
  let mut power=vec![1];for _ in 0..478{power=times_linear(power,479-t);}expected[0]=(expected[0]+c)%479;
  for(i,v)in power.into_iter().enumerate(){expected[i]=(expected[i]+479-c*v%479)%479;}
 }
 assert_eq!(reduced,expected,"pair-mean polynomial identity");
 let mut square=[false;479];for a in 1..479{square[a*a%479]=true;}
 let chi=|x:usize|if x==0{0i128}else if square[x]{1}else{-1};
 let sig=|x:usize|(0..15).map(|a|chi((x+a)%479)).collect::<Vec<_>>();let patterns=[sig(25),sig(292)];let mut triggered=vec![];
 for x in 0..479{
  let signs=sig(x);let mut sum=0i128;for(c,offsets)in &terms{let sign=offsets.iter().fold(1i128,|s,&a|s*signs[a]);sum+=*c as i128*sign;}
  let old=sum.rem_euclid(479);let mean=match x{25|173=>99,292|440=>366,_=>x};assert_eq!(old,mean as i128);
  if patterns.iter().any(|p|*p==signs){sum+=405*chi((x+15)%479);triggered.push(x);}
  assert_eq!(sum.rem_euclid(479),x as i128);
 }
 assert_eq!(triggered,vec![25,173,292,440]);
 println!("{{\"terms\":{},\"pair_mean_polynomial_identity\":true,\"adaptive_field_inputs\":479,\"extra_sign_inputs\":[25,173,292,440],\"producer_imports\":0}}",terms.len());
}
