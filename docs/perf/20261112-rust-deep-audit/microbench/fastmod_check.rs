include!("r2-fastmod.rs");
fn main(){
  let mut s=0x9E3779B97F4A7C15u64; let mut nx=||{s^=s<<13;s^=s>>7;s^=s<<17;s};
  let edges=[0u64,1,2,3,u64::MAX,u64::MAX-1,1<<63,(1<<63)-1,(1<<32)-1,1<<32];
  let mut ds:Vec<u64>=(1..=200).collect(); for _ in 0..200 { ds.push(nx()|1); ds.push(nx()>>(nx()%63)); } ds.extend([u64::MAX,u64::MAX-1,1<<63,(1<<63)+1]);
  for &d in &ds { if d==0 {continue} let f=FastMod::new(d); for &h in &edges { assert_eq!(f.rem(h), h%d, "d={d} h={h}"); } for _ in 0..20000 { let h=nx(); assert_eq!(f.rem(h), h%d, "d={d} h={h}"); } for k in 0..2000u64 { let h=d.wrapping_mul(k); assert_eq!(f.rem(h),h%d); assert_eq!(f.rem(h.wrapping_sub(1)),h.wrapping_sub(1)%d);} }
  println!("ok {} moduli", ds.len());
}
