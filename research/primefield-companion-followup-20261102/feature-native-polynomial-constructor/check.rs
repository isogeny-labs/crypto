//! Independent artifact checker: unreduced repeated linear-factor expansion.
//! No producer, source, solver, or decoder imports.
use std::collections::BTreeSet;
fn main(){
 let path=std::env::args().nth(1).expect("program.tsv");let text=std::fs::read_to_string(path).unwrap();let r=479usize;let h=239usize;
 let mut terms=vec![];let mut labels=BTreeSet::new();let mut expanded=vec![0u64;3*h+1];
 for line in text.lines(){let(c,s)=line.split_once('\t').unwrap();let c:u64=c.parse().unwrap();assert!((1..479).contains(&c));let offsets:Vec<usize>=s.split(',').filter(|s|!s.is_empty()).map(|s|s.parse().unwrap()).collect();assert!(offsets.len()<=3);assert!(offsets.iter().all(|&a|a<16));assert!(offsets.windows(2).all(|w|w[0]<w[1]));assert!(labels.insert(offsets.clone()));
  let mut v=vec![1u64];for &a in &offsets{for _ in 0..h{let mut next=vec![0u64;v.len()+1];for(i,&u)in v.iter().enumerate(){next[i]=(next[i]+u*a as u64)%479;next[i+1]=(next[i+1]+u)%479;}v=next;}}
  for(i,&v)in v.iter().enumerate(){expanded[i]=(expanded[i]+c*v)%479;}terms.push((c,offsets));
 }
 assert!(!terms.is_empty());let mut reduced=vec![0u64;r];for(e,&c)in expanded.iter().enumerate(){let target=if e==0{0}else{1+(e-1)%(r-1)};reduced[target]=(reduced[target]+c)%479;}
 for(i,&c)in reduced.iter().enumerate(){assert_eq!(c,u64::from(i==1));}
 let mut square=[false;479];for a in 1..479{square[a*a%479]=true;}
 for x in 0..479{let mut sum=0i128;for(c,offsets)in &terms{let mut sign=1i128;for &a in offsets{let z=(x+a)%479;sign*=if z==0{0}else if square[z]{1}else{-1};}sum+=*c as i128*sign;}assert_eq!(sum.rem_euclid(479),x as i128);}
 println!("{{\"terms\":{},\"polynomial_identity\":true,\"field_inputs_verified\":479,\"producer_imports\":0}}",terms.len());
}
