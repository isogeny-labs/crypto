mod base0{
include!("/workspace/crypto-twist-field-cost/research/quadratic_twist_field_cost_20261116/projective/first-row/math-audit.rs");
pub fn probe(route:usize)->String{ROUTE.store(route,std::sync::atomic::Ordering::Relaxed);let g=Some(((0,0),(10,0)));let s=chi(Some(((37,0),(14,0))));let points=group(g,21,39);let reps:Vec<_>=points.iter().copied().filter(|p|p.is_some()&&p<&neg(*p)).collect();let r=recover_cold(points[1],g,s,&[reps[0],reps[1],reps[2]],2);assert!(r.rank==0&&r.kind==0&&r.scalar==-1);let mut out=format!("D 0 {} {} {} {} {} {}",route,r.scalar,r.kind,r.rank,r.setup_m,r.shift);for x in r.logs{out+=&format!(" {}",x)}for x in r.count{out+=&format!(" {}",x)}out}
}
mod base1{
include!("/workspace/crypto-twist-field-cost/research/quadratic_twist_field_cost_20261116/projective/setup-rank/math-audit.rs");
pub fn probe(route:usize)->String{ROUTE.store(route,std::sync::atomic::Ordering::Relaxed);let g=Some(((0,0),(10,0)));let s=chi(Some(((37,0),(14,0))));let points=group(g,21,39);let reps:Vec<_>=points.iter().copied().filter(|p|p.is_some()&&p<&neg(*p)).collect();let r=recover_cold(points[1],g,s,&[reps[0],reps[1],reps[2]],2);assert!(r.rank==0&&r.kind==0&&r.scalar==-1);let mut out=format!("D 1 {} {} {} {} {} {}",route,r.scalar,r.kind,r.rank,r.setup_m,r.shift);for x in r.logs{out+=&format!(" {}",x)}for x in r.count{out+=&format!(" {}",x)}out}
}
mod base2{
include!("/workspace/crypto-twist-field-cost/research/quadratic_twist_field_cost_20261116/projective/two-base/math-audit.rs");
pub fn probe(route:usize)->String{ROUTE.store(route,std::sync::atomic::Ordering::Relaxed);let g=Some(((0,0),(10,0)));let s=chi(Some(((37,0),(14,0))));let points=group(g,21,39);let reps:Vec<_>=points.iter().copied().filter(|p|p.is_some()&&p<&neg(*p)).collect();let r=recover_cold(points[1],g,s,&[reps[0],reps[1]],1);assert!(r.rank==0&&r.kind==0&&r.scalar==-1);let mut out=format!("D 2 {} {} {} {} {} {}",route,r.scalar,r.kind,r.rank,r.setup_m,r.shift);for x in r.logs{out+=&format!(" {}",x)}out+=" 0";for x in r.count{out+=&format!(" {}",x)}out}
}
fn main(){for route in 3..6{println!("{}",base0::probe(route));println!("{}",base1::probe(route));println!("{}",base2::probe(route));}}
