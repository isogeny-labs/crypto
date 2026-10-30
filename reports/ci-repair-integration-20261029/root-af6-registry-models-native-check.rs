fn pow(mut a:u64,mut n:u64,p:u64)->u64 { let mut r=1;while n>0 {if n&1!=0 {r=((r as u128*a as u128)%p as u128)as u64;}a=((a as u128*a as u128)%p as u128)as u64;n>>=1;}r }
fn main(){
 for (p,a,b,expected,trace,j) in [(195709u64,0u64,3u64,194853u64,857i64,0u64),(493159,0,5,493329,-169,0),(903163,0,4,901317,1847,0),(1048583,3,12,1046742,1842,453488)] {
 let mut count=1u64;
 for x in 0..p {let rhs=(((x as u128*x as u128%p as u128)*x as u128+a as u128*x as u128+b as u128)%p as u128)as u64;count+=if rhs==0{1}else if pow(rhs,(p-1)/2,p)==1{2}else{0};}
 assert_eq!(count,expected);assert_eq!(p as i64+1-count as i64,trace);
 let a3=((a as u128*a as u128%p as u128)*a as u128%p as u128)as u64;
 let den=((4*a3 as u128+27*b as u128*b as u128)%p as u128)as u64;assert_ne!(den,0);
 let actual_j=((1728u128*4*a3 as u128%p as u128)*pow(den,p-2,p)as u128%p as u128)as u64;assert_eq!(actual_j,j);
 println!("PASS p={p} a={a} b={b}: exhaustive_order={count} trace={trace} j={actual_j}; discriminant_nonzero=true");
 }
}
