fn main(){
 if std::env::var("PF_QUOTIENT_TRACE_ADMITTED").as_deref()!=Ok("1"){eprintln!("collector requires frozen protocol and admitted execution; build is not execution");std::process::exit(2);}
 for(p,a,b,r,base,m)in[(17,2,2,19,[0,6],2),(41,1,5,47,[0,13],2),(97,0,5,79,[1,43],3),(193,2,7,191,[0,59],2),(257,1,7,281,[1,3],5)]{
  let c=group::Curve::new(p,a,b);let bp=group::Point{x:base[0],y:base[1]};let z=(2..r).find(|&z|pow(z,m as u64,r)==1&&(1..m).all(|j|pow(z,j as u64,r)!=1)).unwrap();let modes=Modes::compile(r,m,z,16,256).unwrap();let extra:Vec<_>=(0..17.min(r)).collect();
  for k in 0..r{
   let target=multiple(c,bp,k);let source=source::Source::new(p,a,b,r,base,64).unwrap();
   source::quotient_trace::begin(100000).unwrap();
   let recovered=modes.recover_traced(&source,c,bp,target,&[1,2,3,4],64,&extra,128);
   let capture=source::quotient_trace::finish().unwrap();assert!(!capture.invalid,"incomplete trace");
   let(new,stats,_)=recovered.unwrap();assert_eq!(new,Some(k));
   println!("target,{p},{r},{k},{},{},{}",stats.calls,capture.rows.len(),capture.dropped);
   for (index,e) in capture.rows.iter().enumerate(){println!("quotient,{p},{r},{k},{index},{},{},{},{},{},{},{},{},{}",e.n,e.d,e.support,e.weight,e.route,e.products,e.inversions,e.ok,e.p);}
  }
 }
}
