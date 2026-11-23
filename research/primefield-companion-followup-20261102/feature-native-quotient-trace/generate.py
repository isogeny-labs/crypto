from pathlib import Path
import hashlib,json
p=Path(__file__).resolve().parent;src=p.parent/'feature-native-weighted-quotient';pins={};outs=[]
for f in sorted(src.glob('*.rs')):
 t=f.read_text();pins[str(f)]=hashlib.sha256(f.read_bytes()).hexdigest()
 if f.name=='polynomial.rs':
  t=t.replace('#[cfg(resultant_audit)]bump(0);','#[cfg(resultant_audit)]bump(0);#[cfg(quotient_trace)]trace::bump(0);').replace('#[cfg(resultant_audit)]bump(1);','#[cfg(resultant_audit)]bump(1);#[cfg(quotient_trace)]trace::bump(1);')
  mark=' fn quotient_canonical(&mut self,a:&[u64],b:&[u64])->Result<Vec<u64>,Error>{'
  wrapper=''' fn quotient_canonical(&mut self,a:&[u64],b:&[u64])->Result<Vec<u64>,Error>{
  #[cfg(quotient_trace)]let before=trace::start(self.p,a,b);
  let result=self.quotient_impl(a,b);
  #[cfg(quotient_trace)]trace::end(before,result.is_ok());
  result
 }
 fn quotient_impl(&mut self,a:&[u64],b:&[u64])->Result<Vec<u64>,Error>{'''
  assert mark in t;t=t.replace(mark,wrapper,1)
  t=t.replace('if a!=[0] && n<=2{','if a!=[0] && n<=2{\n   #[cfg(quotient_trace)]trace::route(n as u8);',1)
  t=t.replace('   if sparse{','   if sparse{\n    #[cfg(quotient_trace)]trace::route(3);',1)
  t=t.replace('  let rev_a:Vec<_>=','  #[cfg(quotient_trace)]trace::route(if a==[0]{0}else{4});\n  let rev_a:Vec<_>=',1)
  t+='\n#[cfg(quotient_trace)]#[path="trace-meter.rs"]pub mod trace;\n'
 if f.name=='lib.rs':t+='\n#[cfg(quotient_trace)]pub use polynomial::trace as quotient_trace;\n'
 if f.name=='new-recovery.rs':
  t='''fn main(){
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
'''
 (p/f.name).write_text(t);outs.append(f.name)
(p/'generation.json').write_text(json.dumps({'input_hashes':pins,'output_hashes':{n:hashlib.sha256((p/n).read_bytes()).hexdigest() for n in outs}},indent=2)+'\n')
