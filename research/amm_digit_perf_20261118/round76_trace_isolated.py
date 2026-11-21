#!/usr/bin/env python3
"""Source-exact Rust A/B for round76 producer polynomial loops.

Extracts Engine::mul/div and field_value_cached from repository source. The
baseline is the pre-round76 loop reconstructed by undoing only identity
shortcuts. Includes exact polynomial equality tests, 13 alternating trials,
and field-operation counts. This isolates kernels, not the full solve+verify.
"""
from pathlib import Path
import subprocess
import tempfile

root=Path(__file__).resolve().parents[2]
source=(root/"src/cryptanalysis/prime_root_trace.rs").read_text()
def extract(signature):
    start=source.index(signature)
    brace=source.index("{",start)
    nesting=0
    for i in range(brace,len(source)):
        if source[i]=="{":nesting+=1
        elif source[i]=="}":
            nesting-=1
            if nesting==0:return source[start:i+1]
    raise ValueError("unclosed "+signature)

cached=extract("fn field_value_cached(op: &FieldOp, a: u64, b: u64, p: u64, reciprocal32: u64)")
mul=extract("fn mul(&mut self, a: &[u64], b: &[u64])")
div=extract("fn div(&mut self, a: &[u64], b: &[u64])")
old_add="                product[i + j] = self.field(FieldOp::Add, product[i + j], z)?;"
new_add="""                // A zero accumulator has no previous contribution. Avoid
                // an Add field event (and its sink/serialization overhead).
                // The product certificate remains checked independently.
                let previous = product[i + j];
                product[i + j] = if previous == 0 {
                    z
                } else {
                    self.field(FieldOp::Add, previous, z)?
                };"""
assert mul.count(new_add)==1
mul_old=mul.replace("fn mul(","fn mul_baseline(",1).replace(new_add,old_add)
mul_new=mul.replace("fn mul(","fn mul_new(",1)
old_div="""            let inv = self.inv(*b.last().unwrap())?;
            while r.len() >= b.len() {
                let shift = r.len() - b.len();
                let c = self.field(FieldOp::Mul, *r.last().unwrap(), inv)?;
                q[shift] = c;
                for (k, &v) in b.iter().enumerate() {
                    // Zero divisor terms leave the remainder unchanged.
                    // This also avoids two trace events for each zero term.
                    if v == 0 { continue; }
                    let product = self.field(FieldOp::Mul, c, v)?;
                    r[shift + k] = self.field(FieldOp::Sub, r[shift + k], product)?;
                }
                r = trim(r);
            }"""
start=div.index("            // For monic divisors the quotient")
end=div.index("                r = trim(r);",start)+len("                r = trim(r);\n            }")
div_old=div[:start]+old_div+div[end:]
div_old=div_old.replace("fn div(","fn div_baseline(",1)
div_new=div.replace("fn div(","fn div_new(",1)

harness=r"""
use std::hint::black_box;
use std::time::Instant;
const MAX_PRIME:u64=(1u64<<61)-1;
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
enum FieldOp{Add,Sub,Mul,Square}
#[derive(Debug)]
enum RootEvent {
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
fn new(p:u64)->Self{Self{p,reciprocal32:if p<=u32::MAX as u64 {((1u128<<64)/p as u128) as u64}else{0},ops:0,sink:0}}
fn field(&mut self,op:FieldOp,a:u64,b:u64)->Work<u64>{
    let v=field_value_cached(&op,a,b,self.p,self.reciprocal32);
    self.ops+=1;
    self.sink=self.sink.wrapping_mul(33).wrapping_add(v);
    Ok(v)
}
fn event(&mut self,event:RootEvent)->Work<()>{
    self.sink^=match event{
        RootEvent::Product{a,b,product}=>a.len() as u64 ^ ((b.len() as u64)<<8)^product.last().copied().unwrap_or(0),
        RootEvent::Division{a,b,quotient,remainder}=>a.len() as u64 ^ ((b.len() as u64)<<8)^quotient.first().copied().unwrap_or(0)^remainder.first().copied().unwrap_or(0)
    };
    Ok(())
}
fn inv(&mut self,value:u64)->Work<u64>{
    if value==1{return Ok(1);}
    let mut exponent=self.p-2;let (mut acc,mut base)=(1,value);
    while exponent>0{
        if exponent&1==1{acc=self.field(FieldOp::Mul,acc,base)?;}
        exponent>>=1;
        if exponent!=0{base=self.field(FieldOp::Square,base,base)?;}
    }
    Ok(acc)
}
@@OLDMUL@@
@@NEWMUL@@
@@OLDDIV@@
@@NEWDIV@@
}
fn next(seed:&mut u64)->u64{
    *seed=seed.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);*seed
}
fn inputs(p:u64,degree:usize,shape:&str,div:bool,monic:bool)->Vec<(Vec<u64>,Vec<u64>)>{
    let mut seed=2026112076u64;let mut out=Vec::new();
    for _ in 0..96 {
        let mut a=Vec::new();let mut b=Vec::new();
        for i in 0..degree+1 {
            let x=next(&mut seed)%p;
            a.push(if shape=="sparse" && i%3!=0 {0}else{x});
        }
        let bsize=if div{degree/2+1}else{degree+1};
        for i in 0..bsize{
            let x=next(&mut seed)%p;
            b.push(if shape=="sparse" && i%3!=0 {0}else{x});
        }
        *a.last_mut().unwrap()=2;
        *b.last_mut().unwrap()=if monic{1}else{2};
        out.push((a,b));
    }
    out
}
fn measure(p:u64,inputs:&[(Vec<u64>,Vec<u64>)],div:bool,fast:bool)->(f64,u64){
    let start=Instant::now();let mut h=0u64;let mut ops=0;
    for _ in 0..100 {for (a,b) in inputs {
        let mut e=Engine::new(p);
        if div {
            let (q,r)=if fast{e.div_new(black_box(a),black_box(b))}
                        else{e.div_baseline(black_box(a),black_box(b))}.unwrap();
            h^=q.first().copied().unwrap_or(0)^r.first().copied().unwrap_or(0);
        } else {
            let c=if fast{e.mul_new(black_box(a),black_box(b))}
                        else{e.mul_baseline(black_box(a),black_box(b))}.unwrap();
            h^=c.first().copied().unwrap_or(0);
        }
        ops+=e.ops;h^=e.sink;
    }}
    black_box(h);
    (start.elapsed().as_nanos() as f64/(100*inputs.len()) as f64,ops)
}
fn main(){
    println!("prime,case,before_ns,after_ns,speedup,before_ops,after_ops");
    for p in [65521,1_000_000_007,MAX_PRIME]{
        for degree in [4,8]{
          for shape in ["dense","sparse"]{
            for is_div in [false,true]{
              for monic in [true,false]{
                if !is_div && !monic {continue}
                let xs=inputs(p,degree,shape,is_div,monic);
                for (a,b) in &xs {
                    let mut old=Engine::new(p);let mut new=Engine::new(p);
                    if is_div{assert_eq!(old.div_baseline(a,b).unwrap(),new.div_new(a,b).unwrap());}
                    else{assert_eq!(old.mul_baseline(a,b).unwrap(),new.mul_new(a,b).unwrap());}
                    assert!(new.ops<=old.ops);
                }
                let (mut old,mut new)=(Vec::new(),Vec::new());
                let mut counts=(0,0);
                for trial in 0..13{
                    let (a,b)=if trial%2==0{
                        (measure(p,&xs,is_div,false),measure(p,&xs,is_div,true))
                    }else{
                        let b=measure(p,&xs,is_div,true);
                        (measure(p,&xs,is_div,false),b)
                    };
                    counts=(a.1,b.1);
                    old.push(a.0);new.push(b.0);
                }
                old.sort_by(f64::total_cmp);new.sort_by(f64::total_cmp);
                println!("{p},{}_d{}_{}_{} ,{:.3},{:.3},{:.3},{},{}",
                    if is_div{"div"}else{"mul"},degree,shape,
                    if monic{"monic"}else{"nonmonic"},
                    old[6],new[6],old[6]/new[6],counts.0,counts.1);
              }
            }
          }
        }
    }
    println!("native Rust source-exact polynomial equality tests passed");
}
"""
rust=(harness.replace("@@CACHED@@",cached).replace("@@OLDMUL@@",mul_old)
       .replace("@@NEWMUL@@",mul_new).replace("@@OLDDIV@@",div_old)
       .replace("@@NEWDIV@@",div_new))
with tempfile.TemporaryDirectory() as directory:
    sourcefile=Path(directory)/"round76.rs"
    binary=Path(directory)/"round76"
    sourcefile.write_text(rust)
    subprocess.run(["rustc","--edition=2021","-O","-C","target-cpu=native",
                    "-o",str(binary),str(sourcefile)],check=True)
    subprocess.run([str(binary)],check=True)
