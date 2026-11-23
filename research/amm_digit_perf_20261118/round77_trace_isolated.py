#!/usr/bin/env python3
"""Production-source-exact isolated Rust A/B for round77 trace producer.

Extracts the actual Rust polynomial methods. Reconstructs the previous methods
by deleting only the new unit-specialized branches. Compares every result and
counts field operations. This is not a complete solve + certificate timing.
"""
from pathlib import Path
import subprocess
import tempfile

root=Path(__file__).resolve().parents[2]
# Historical microbenchmarks compare their original production snapshot.
# Round78's separate harness tests current code against the Round77 base.
source=subprocess.check_output(["git", "show", "5b1b404b4fe0632d4add0a1b20209430c0a8a810:src/cryptanalysis/prime_root_trace.rs"], cwd=root, text=True)
def extract(signature):
    start=source.index(signature)
    brace=source.index("{",start)
    count=0
    for i in range(brace,len(source)):
        if source[i]=="{":count+=1
        if source[i]=="}":
            count-=1
            if count==0:return source[start:i+1]
    raise ValueError("unclosed "+signature)

cached=extract("fn field_value_cached(op: &FieldOp, a: u64, b: u64, p: u64, reciprocal32: u64)")
mul=extract("fn mul(&mut self, a: &[u64], b: &[u64])")
div=extract("fn div(&mut self, a: &[u64], b: &[u64])")
square=extract("fn square(&mut self, a: &[u64])")
mul_begin=mul.index("        // Multiplying by the constant polynomial one")
mul_end=mul.index("        let mut product = if a.is_empty()",mul_begin)
old_mul=(mul[:mul_begin]+mul[mul_end:]).replace("fn mul(","fn mul_old(",1)
new_mul=mul.replace("fn mul(","fn mul_new(",1)
div_begin=div.index("        // Euclidean GCD often ends with a unit divisor")
div_end=div.index("        let mut r = a.to_vec();",div_begin)
old_div=(div[:div_begin]+div[div_end:]).replace("fn div(","fn div_old(",1)
new_div=div.replace("fn div(","fn div_new(",1)
new_square=square.replace("fn square(","fn square_new(",1)

harness=r"""
use std::hint::black_box;
use std::time::Instant;
const MAX_PRIME:u64=(1u64<<61)-1;
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
enum FieldOp{Add,Sub,Mul,Square}
enum RootEvent{
 Product{a:Vec<u64>,b:Vec<u64>,product:Vec<u64>},
 Division{a:Vec<u64>,b:Vec<u64>,quotient:Vec<u64>,remainder:Vec<u64>}
}
#[derive(Debug)]
enum RootStop{WorkLimit}
type Work<T>=Result<T,RootStop>;
fn trim(mut v:Vec<u64>)->Vec<u64>{while v.last()==Some(&0){v.pop();}v}
@@CACHED@@
struct Engine{p:u64,reciprocal32:u64,ops:u64,sink:u64}
impl Engine{
fn new(p:u64)->Self{Self{p,reciprocal32:if p<=u32::MAX as u64{((1u128<<64)/p as u128) as u64}else{0},ops:0,sink:0}}
fn field(&mut self,op:FieldOp,a:u64,b:u64)->Work<u64>{
 let v=field_value_cached(&op,a,b,self.p,self.reciprocal32);
 self.ops+=1;self.sink=self.sink.wrapping_mul(33).wrapping_add(v);Ok(v)
}
fn event(&mut self,event:RootEvent)->Work<()>{
 self.sink^=match event{
 RootEvent::Product{a,b,product}=>a.len() as u64^((b.len() as u64)<<8)^product.last().copied().unwrap_or(0),
 RootEvent::Division{a,b,quotient,remainder}=>a.len() as u64^((b.len() as u64)<<8)^quotient.first().copied().unwrap_or(0)^remainder.first().copied().unwrap_or(0)
 };Ok(())
}
fn inv(&mut self,value:u64)->Work<u64>{
 if value==1{return Ok(1)}
 let mut e=self.p-2;let (mut acc,mut base)=(1,value);
 while e>0{if e&1==1{acc=self.field(FieldOp::Mul,acc,base)?;}
 e>>=1;if e!=0{base=self.field(FieldOp::Square,base,base)?;}}
 Ok(acc)
}
@@OLDMUL@@
@@NEWMUL@@
@@OLDDIV@@
@@NEWDIV@@
@@NEWSQUARE@@
fn square_old(&mut self,a:&[u64])->Work<Vec<u64>>{self.mul_old(a,a)}
}
fn next(seed:&mut u64)->u64{
 *seed=seed.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);*seed
}
fn inputs(p:u64,case:&str)->Vec<(Vec<u64>,Vec<u64>)>{
 let mut seed=0x77112026u64;let mut xs=Vec::new();
 for _ in 0..96{
  let mut a=Vec::new();
  for i in 0..9{
   let value=next(&mut seed)%p;
   a.push(if case=="sparse_square" && i%3!=0{0}else{value});
  }
  *a.last_mut().unwrap()=next(&mut seed)%p;
  if case=="unit_div" {xs.push((a,vec![1]));}
  else if case=="unit_mul" {xs.push((vec![1],a));}
  else {xs.push((a.clone(),a));}
 }xs
}
fn calculate(e:&mut Engine,case:&str,a:&[u64],b:&[u64],fast:bool)->Vec<u64>{
 match case{
 "dense_square"|"sparse_square"=>if fast{e.square_new(a).unwrap()}else{e.square_old(a).unwrap()},
 "unit_div"=>if fast{e.div_new(a,b).unwrap().0}else{e.div_old(a,b).unwrap().0},
 "unit_mul"=>if fast{e.mul_new(a,b).unwrap()}else{e.mul_old(a,b).unwrap()},
 _=>unreachable!()
 }
}
fn sample(p:u64,case:&str,xs:&[(Vec<u64>,Vec<u64>)],fast:bool)->(f64,u64){
 let start=Instant::now();let mut hash=0u64;let mut count=0u64;
 for _ in 0..80{for (a,b) in xs {
 let mut e=Engine::new(p);
 let c=calculate(&mut e,case,black_box(a),black_box(b),fast);
 hash=hash.wrapping_add(e.sink^c.last().copied().unwrap_or(0));
 count+=e.ops;
 }}
 black_box(hash);
 (start.elapsed().as_nanos() as f64/(80*xs.len()) as f64,count)
}
fn main(){
 println!("prime,case,before_ns,after_ns,speedup,before_ops,after_ops");
 let mut checks=0;
 for p in [65521,1000000007,MAX_PRIME] {
  for case in ["dense_square","sparse_square","unit_div","unit_mul"]{
   let xs=inputs(p,case);
   for (a,b) in &xs {
    let mut old=Engine::new(p);let mut new=Engine::new(p);
    assert_eq!(calculate(&mut old,case,a,b,false),calculate(&mut new,case,a,b,true));
    assert!(new.ops<=old.ops);
    checks+=1;
   }
   let (mut old,mut new)=(Vec::new(),Vec::new());let mut counts=(0,0);
   for trial in 0..13{
    let (a,b)=if trial%2==0{(sample(p,case,&xs,false),sample(p,case,&xs,true))}
    else{let b=sample(p,case,&xs,true);(sample(p,case,&xs,false),b)};
    counts=(a.1,b.1);old.push(a.0);new.push(b.0);
   }
   old.sort_by(f64::total_cmp);new.sort_by(f64::total_cmp);
   println!("{p},{case},{:.3},{:.3},{:.3},{},{}",old[6],new[6],old[6]/new[6],counts.0,counts.1);
  }
 }
 println!("verified {} Rust polynomial output comparisons",checks);
}
"""
rust=(harness.replace("@@CACHED@@",cached).replace("@@OLDMUL@@",old_mul)
 .replace("@@NEWMUL@@",new_mul).replace("@@OLDDIV@@",old_div)
 .replace("@@NEWDIV@@",new_div).replace("@@NEWSQUARE@@",new_square))
with tempfile.TemporaryDirectory() as directory:
    path=Path(directory)/"round77.rs"
    binary=Path(directory)/"round77"
    path.write_text(rust)
    subprocess.run(["rustc","--edition=2021","-O","-C","target-cpu=native","-o",str(binary),str(path)],check=True)
    subprocess.run([str(binary)],check=True)
