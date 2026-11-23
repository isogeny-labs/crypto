use crate::packet::Packet;use hec_native_ic::fp::Fp;use std::collections::BTreeMap;
pub type Index=BTreeMap<(u64,u64),Option<(usize,u64)>>;
fn insert(index:&mut Index,key:(u64,u64),value:(usize,u64)){use std::collections::btree_map::Entry;match index.entry(key){Entry::Vacant(e)=>{e.insert(Some(value));},Entry::Occupied(mut e)=>{e.insert(None);}}}
pub fn build(p:&Packet,fl:&Fp)->Index{let mut index=Index::new();for(j,&(x,y))in p.reps.iter().enumerate(){let(mut x,mut w)=(x,1);for _ in 0..5{insert(&mut index,(x,y),(j,w));let ny=p.curve.fp.neg(y);if ny!=y{insert(&mut index,(x,ny),(j,fl.neg(w)));}x=p.curve.fp.mul(x,p.z);w=fl.mul(w,p.lambda);}}index}
pub fn fold(p:&Packet,points:&[(u64,u64)],fl:&Fp,index:&Index)->Option<Vec<u64>>{let c=&p.curve;let mut out=vec![0;p.reps.len()+1];for &(x,y)in points{if x>=c.fp.p||y>=c.fp.p||c.fp.mul(y,y)!=c.f.eval(x,&c.fp){return None}let(j,w)=index.get(&(x,y)).copied().flatten()?;out[j]=fl.add(out[j],w);}Some(out)}
