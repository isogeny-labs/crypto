use std::collections::HashMap; use std::hash::{BuildHasherDefault,Hasher}; use std::time::Instant; use std::hint::black_box;
#[derive(Default)] struct Mix(u64);
impl Hasher for Mix { fn finish(&self)->u64{let mut h=self.0; h^=h>>32; h} fn write(&mut self,_:&[u8]){unreachable!()} fn write_u64(&mut self,v:u64){self.0=(self.0.rotate_left(5)^v).wrapping_mul(0x9E37_79B9_7F4A_7C15);} }
type FM<V>=HashMap<u64,V,BuildHasherDefault<Mix>>;
struct Set{slots:Vec<u64>,mask:usize,len:usize}
impl Set{ fn new(c:usize)->Self{let n=(c*2).next_power_of_two().max(16); Set{slots:vec![0;n],mask:n-1,len:0}}
 #[inline] fn home(&self,k:u64)->usize{(k.wrapping_mul(0x9E37_79B9_7F4A_7C15)>>20) as usize & self.mask}
 fn insert(&mut self,k:u64)->bool{ // k!=0 ; returns true if was present
   let mut s=self.home(k); loop{let h=self.slots[s]; if h==0{self.slots[s]=k; self.len+=1; if self.len*2>self.mask{self.grow()} return false} if h==k{return true} s=(s+1)&self.mask}}
 fn grow(&mut self){let old=std::mem::take(&mut self.slots); let n=old.len()*2; self.slots=vec![0;n]; self.mask=n-1; for k in old{ if k!=0{let mut s=self.home(k); while self.slots[s]!=0{s=(s+1)&self.mask} self.slots[s]=k;}}}}
fn main(){
 let mut x=99u64; let mut rnd=||{x^=x<<13;x^=x>>7;x^=x<<17;x};
 let keys:Vec<u64>=(0..123_000).map(|_| ((rnd()>>36)+1)<<1).collect();
 let tkeys:Vec<u64>=(0..131_000).map(|_| ((rnd()>>36)+1)).collect();
 let mut table:FM<(u32,u32)>=HashMap::with_capacity_and_hasher(131_000,Default::default()); for (i,&k) in tkeys.iter().enumerate(){table.entry(k).or_insert((i as u32,0));}
 for _ in 0..3{
 let t=Instant::now(); let mut seen:FM<()>=HashMap::with_capacity_and_hasher(4096,Default::default()); let mut c=0; for &k in &keys{ if seen.insert(black_box(k),()).is_some(){c+=1} } println!("seen std grow {:.1} ns {}",t.elapsed().as_nanos() as f64/keys.len() as f64,c);
 let t=Instant::now(); let mut seen=Set::new(4096); let mut c=0; for &k in &keys{ if seen.insert(black_box(k)){c+=1} } println!("seen oa grow {:.1} ns {}",t.elapsed().as_nanos() as f64/keys.len() as f64,c);
 let t=Instant::now(); let mut c=0u64; for &k in &keys{ if let Some(v)=table.get(&black_box(k>>1)){c+=v.0 as u64} } println!("table get {:.1} ns {}",t.elapsed().as_nanos() as f64/keys.len() as f64,c);
 }}
