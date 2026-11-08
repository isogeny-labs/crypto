#[path="lib.rs"]mod source;
#[path="../feature-orbit-local-terms/local.rs"]mod group;
mod polynomial;
#[derive(Debug,PartialEq)]enum Error{Domain,Limit,Arithmetic}
fn mul(a:u64,b:u64,r:u64)->u64{(a as u128*b as u128%r as u128)as u64}
fn pow(mut a:u64,mut n:u64,r:u64)->u64{let mut v=1;while n>0{if n&1!=0{v=mul(v,a,r)}n>>=1;if n>0{a=mul(a,a,r)}}v}
fn eval(a:&[u64],x:u64,r:u64)->u64{a.iter().rev().fold(0,|v,&c|(mul(v,x,r)+c)%r)}
fn multiple(c:group::Curve,p:group::Point,mut n:u64)->Option<group::Point>{let(mut out,mut a)=(None,Some(p));while n>0{if n&1!=0{out=c.plus(out,a)}n>>=1;if n>0{a=c.plus(a,a)}}out}
include!("circuit.rs");
include!("transcript.rs");
include!("public.rs");
include!("checks.rs");
fn main(){circuit_checks();public_checks();}
