//! Complete finite character check, no constructor/recurrence/decoder imports.
use std::collections::{BTreeMap,BTreeSet};
fn main(){let path=std::env::args().nth(1).unwrap();let text=std::fs::read_to_string(path).unwrap();let mut terms=vec![];let mut seen=BTreeSet::new();
 for line in text.lines(){let(c,l)=line.split_once('\t').unwrap();let c:i64=c.parse().unwrap();assert!(c>0&&c<479);let l:Vec<usize>=l.split(',').filter(|s|!s.is_empty()).map(|s|s.parse().unwrap()).collect();assert!(l.len()<=4&&l.iter().all(|&a|a<14)&&l.windows(2).all(|w|w[0]<w[1]));assert!(seen.insert(l.clone()));terms.push((c,l));}
 let mut sq=[false;479];for a in 1..479{sq[a*a%479]=true;}let chi=|x:usize|if x==0{0i64}else if sq[x]{1}else{-1};
 let mut groups:BTreeMap<Vec<i64>,Vec<usize>>=BTreeMap::new();for x in 0..479{groups.entry((0..14).map(|a|chi((x+a)%479)).collect()).or_default().push(x);}
 let mut ambiguous=0;let mut fibers=0;let mut quartic=0;for(_,l)in &terms{quartic+=usize::from(l.len()==4);}
 for(signs,g)in &groups{let inv=(1..479).find(|i|i*g.len()%479==1).unwrap();let mean=(g.iter().sum::<usize>()%479)*inv%479;for &x in g{let out=terms.iter().map(|(c,l)|c*l.iter().fold(1,|s,&a|s*signs[a])).sum::<i64>().rem_euclid(479);assert_eq!(out,mean as i64,"x={x}");}if g.len()>1{fibers+=1;ambiguous+=g.len();}}
 assert_eq!((fibers,ambiguous),(8,16));println!("{{\"field_inputs\":479,\"fibers\":8,\"ambiguous_inputs\":16,\"terms\":{},\"quartic_terms\":{},\"producer_imports\":0}}",terms.len(),quartic);
}
