#!/usr/bin/env python3
"""Source-exact isolated Rust A/B for the trace producer's division and linear-root paths.

The before source comes from the exact Round77 commit (stacked PR base).
This exercises production Rust functions, not a reimplementation of div/inv.
The Engine event plumbing is reduced for standalone rustc compilation.
"""
from pathlib import Path
import subprocess
import tempfile

root = Path(__file__).resolve().parents[2]
path = "src/cryptanalysis/prime_root_trace.rs"
before = subprocess.check_output([
    "git", "show",
    "5b1b404b4fe0632d4add0a1b20209430c0a8a810:" + path
], cwd=root, text=True)
after = (root / path).read_text()

def extract(source, signature):
    start = source.index(signature)
    brace = source.index("{", start)
    depth = 0
    for i in range(brace, len(source)):
        if source[i] == "{": depth += 1
        elif source[i] == "}":
            depth -= 1
            if depth == 0: return source[start:i + 1]
    raise ValueError("unclosed " + signature)

def linear_branch(source, suffix):
    start = source.index("                2 => {", source.index("    fn run("))
    end = source.index("self.event(RootEvent::LinearRoot", start)
    chunk = source[start + len("                2 => {"):end]
    return "fn linear_root_" + suffix + "(&mut self, h: &[u64]) -> Work<u64> {\n" + chunk + "\nOk(root)\n}"

div_sig = "fn div(&mut self, a: &[u64], b: &[u64])"
old_div = extract(before, div_sig).replace("fn div(", "fn div_old(", 1)
new_div = extract(after, div_sig).replace("fn div(", "fn div_new(", 1)
inv = extract(after, "fn inv(&mut self, value: u64)").replace("self.report.p", "self.p")
cached = extract(after, "fn field_value_cached(op: &FieldOp, a: u64, b: u64, p: u64, reciprocal32: u64)")
monomial = extract(after, "fn monic_x_shift(a: &[u64])")
linear_old = linear_branch(before, "old")
linear_new = linear_branch(after, "new")

harness = r'''
use std::hint::black_box;
use std::time::Instant;
const MAX_PRIME: u64 = (1u64<<61)-1;
#[derive(Clone,Debug,PartialEq,Eq)]
enum FieldOp{Add,Sub,Mul,Square}
#[allow(dead_code)]
enum RootEvent{
 Field{op:FieldOp,a:u64,b:u64,value:u64},
 InverseBegin{value:u64},
 InverseEnd{value:u64,inverse:u64},
 Division{a:Vec<u64>,b:Vec<u64>,quotient:Vec<u64>,remainder:Vec<u64>}
}
#[derive(Debug)]
enum RootStop{WorkLimit}
type Work<T> = Result<T,RootStop>;
fn trim(mut v:Vec<u64>)->Vec<u64>{while v.last()==Some(&0){v.pop();}v}
@@MONOMIAL@@
@@CACHED@@
struct Engine{p:u64,reciprocal32:u64,ops:u64,events:u64,hash:u64}
impl Engine{
 fn new(p:u64)->Self{Self{p,reciprocal32:if p<=u32::MAX as u64{((1u128<<64)/p as u128) as u64}else{0},ops:0,events:0,hash:0}}
 fn event(&mut self,event:RootEvent)->Work<()>{
   self.events+=1;
   let val=match event{
    RootEvent::Field{op,a,b,value}=>{
     let tag=match op{FieldOp::Add=>1,FieldOp::Sub=>2,FieldOp::Mul=>3,FieldOp::Square=>4};
     a^b^value^(tag<<48)
    },
    RootEvent::InverseBegin{value}=>value,
    RootEvent::InverseEnd{value,inverse}=>value^inverse,
    RootEvent::Division{a,b,quotient,remainder}=>{
     (a.len() as u64)^((b.len() as u64)<<9)^((quotient.len() as u64)<<18)^
     ((remainder.len() as u64)<<27)^quotient.first().copied().unwrap_or(0)^remainder.first().copied().unwrap_or(0)
    }
   };
   self.hash=self.hash.wrapping_mul(37).wrapping_add(val);Ok(())
 }
 fn field(&mut self,op:FieldOp,a:u64,b:u64)->Work<u64>{
   let value=field_value_cached(&op,a,b,self.p,self.reciprocal32);
   self.ops+=1;self.event(RootEvent::Field{op,a,b,value})?;Ok(value)
 }
 @@INV@@
 @@OLDDIV@@
 @@NEWDIV@@
 @@OLDROOT@@
 @@NEWROOT@@
}
fn next(s:&mut u64)->u64{*s=s.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);*s}
fn make_inputs(p:u64,case:&str)->Vec<(Vec<u64>,Vec<u64>)>{
 let mut seed=0x7820261111u64;
 let mut out=Vec::new();
 for _ in 0..96{
  let mut a:Vec<u64>=(0..9).map(|_|next(&mut seed)%p).collect();
  if *a.last().unwrap()==0 {*a.last_mut().unwrap()=1;}
  let b=match case{
    "x_shift_div"=>vec![0,0,0,0,1],
    "linear_div"=>vec![(next(&mut seed)% (p-1))+1,1],
    "short_div"=>{a.truncate(2);vec![1,2,3,4,1]},
    "dense_div"=>vec![next(&mut seed)%p,next(&mut seed)%p,next(&mut seed)%p,next(&mut seed)%p,1],
    "sparse_div"=>{a[2]=0;a[3]=0;a[5]=0;vec![next(&mut seed)%p,0,next(&mut seed)%p,0,1]},
    "linear_root"=>{a=vec![next(&mut seed)%p,1];vec![]},
    _=>unreachable!()
  };
  out.push((a,b));
 }out
}
fn calculate(e:&mut Engine,case:&str,a:&[u64],b:&[u64],fast:bool)->(Vec<u64>,Vec<u64>){
 if case=="linear_root"{
  (vec![if fast{e.linear_root_new(a).unwrap()}else{e.linear_root_old(a).unwrap()}],vec![])
 }else if fast{e.div_new(a,b).unwrap()}else{e.div_old(a,b).unwrap()}
}
fn sample(p:u64,case:&str,inputs:&[(Vec<u64>,Vec<u64>)],fast:bool)->(f64,u64,u64){
 let t=Instant::now();let mut hash=0u64;let mut ops=0;let mut events=0;
 for _ in 0..32{for (a,b) in inputs{
  let mut e=Engine::new(p);let (q,r)=calculate(&mut e,case,black_box(a),black_box(b),fast);
  hash=hash.wrapping_add(e.hash^q.last().copied().unwrap_or(0)^r.last().copied().unwrap_or(0));
  ops+=e.ops;events+=e.events;
 }}
 black_box(hash);
 (t.elapsed().as_nanos() as f64/(32*inputs.len()) as f64,ops,events)
}
fn main(){
 println!("prime,case,before_ns,after_ns,speedup,before_field_ops,after_field_ops,before_events,after_events");
 let mut checks=0;
 for p in [65521,1000000007,MAX_PRIME]{
  for case in ["x_shift_div","linear_div","short_div","dense_div","sparse_div","linear_root"]{
   let xs=make_inputs(p,case);
   for (a,b) in &xs{
    let mut old=Engine::new(p);let mut new=Engine::new(p);
    assert_eq!(calculate(&mut old,case,a,b,false),calculate(&mut new,case,a,b,true),"case={case}, p={p}, a={a:?}, b={b:?}");
    assert!(new.ops<=old.ops,"field operations regressed, case={case} p={p}");
    checks+=1;
   }
   let(mut old,mut new)=(Vec::new(),Vec::new());let mut counts=(0,0,0,0);
   for trial in 0..13{
    let (a,b)=if trial%2==0{
      (sample(p,case,&xs,false),sample(p,case,&xs,true))
    }else{
      let b=sample(p,case,&xs,true);(sample(p,case,&xs,false),b)
    };
    counts=(a.1,b.1,a.2,b.2);old.push(a.0);new.push(b.0);
   }
   old.sort_by(f64::total_cmp);new.sort_by(f64::total_cmp);
   println!("{p},{case},{:.3},{:.3},{:.3},{},{},{},{}",old[6],new[6],old[6]/new[6],counts.0,counts.1,counts.2,counts.3);
  }
 }
 println!("verified {checks} exact Rust result comparisons");
}
'''
repls = {
 "@@MONOMIAL@@":monomial, "@@CACHED@@":cached, "@@INV@@":inv,
 "@@OLDDIV@@":old_div, "@@NEWDIV@@":new_div,
 "@@OLDROOT@@":linear_old, "@@NEWROOT@@":linear_new
}
for marker, value in repls.items():
    assert marker in harness, marker
    harness = harness.replace(marker,value)
with tempfile.TemporaryDirectory() as directory:
    src = Path(directory)/"round78.rs"
    binary = Path(directory)/"round78"
    src.write_text(harness)
    subprocess.run(["rustc","--edition=2021","-O","-C","target-cpu=native","-o",str(binary),str(src)],check=True)
    subprocess.run([str(binary)],check=True)
