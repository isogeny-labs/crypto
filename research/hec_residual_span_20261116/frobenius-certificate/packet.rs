use hec_native_ic::{jac::{Curve,Div}};
use std::cell::RefCell;
#[derive(Clone,Debug)]pub enum Kind{Public,Source,Pencil{a:u64,t:u64,points:Vec<(u64,u64)>}}
#[derive(Clone,Debug)]pub struct Row{pub kind:Kind,pub coeff:Vec<u64>,pub rhs:u64}
#[derive(Clone,Debug)]pub struct Packet{pub curve:Curve,pub d:Div,pub target:Div,pub ell:u64,pub sm:u64,pub z:u64,pub lambda:u64,pub reps:Vec<(u64,u64)>,pub rows:Vec<Row>,pub witness:Vec<u64>,pub k:u64}
thread_local!{static ACTION:RefCell<(u64,u64)>=const{RefCell::new((0,0))};static CURRENT:RefCell<Option<Packet>>=const{RefCell::new(None)};}
pub fn action(z:u64,l:u64){ACTION.with(|x|*x.borrow_mut()=(z,l));}
pub fn begin(c:&Curve,d:&Div,t:&Div,ell:u64,sm:u64,fb:&[(u64,u64)],reps:&[usize]){let(z,lambda)=ACTION.with(|x|*x.borrow());CURRENT.with(|x|*x.borrow_mut()=Some(Packet{curve:c.clone(),d:*d,target:*t,ell,sm,z,lambda,reps:reps.iter().map(|&j|fb[j]).collect(),rows:vec![],witness:vec![],k:0}));}
pub fn row(kind:Kind,coeff:&[u64],rhs:u64){CURRENT.with(|x|x.borrow_mut().as_mut().unwrap().rows.push(Row{kind,coeff:coeff.to_vec(),rhs}));}
pub fn finish(w:Vec<u64>,k:u64){CURRENT.with(|x|{let mut x=x.borrow_mut();let p=x.as_mut().unwrap();p.witness=w;p.k=k;});}
pub fn take()->Packet{CURRENT.with(|x|x.borrow_mut().take().unwrap())}
