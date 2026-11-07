type Matrix=[[Vec<u64>;2];2];
#[derive(Clone,Debug,PartialEq)]
struct Transition{m:usize,n:usize,k:Option<usize>,leading:u64}
struct Batch{matrix:Matrix,a:Vec<u64>,b:Vec<u64>,steps:Vec<Transition>}
fn degree(a:&[u64])->Option<usize>{if a==[0]{None}else{Some(a.len()-1)}}
fn identity()->Matrix{[[vec![1],vec![0]],[vec![0],vec![1]]]}
#[cfg(test)]std::thread_local!{static BATCH_CALLS:std::cell::Cell<usize>=const{std::cell::Cell::new(0)};}
#[cfg(test)]pub fn reset_batches(){BATCH_CALLS.with(|n|n.set(0));}
#[cfg(test)]pub fn batches()->usize{BATCH_CALLS.with(|n|n.get())}
