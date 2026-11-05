// Independent packed-F2 rank/nullspace checker. No CAS or external crate.
use std::{collections::HashSet,env,fs};
#[derive(Clone)]
struct Matrix { rows:Vec<Vec<u64>>,cols:usize }
impl Matrix {
 fn zero(r:usize,c:usize)->Self {assert!(r>0&&r<=1_000_000&&c>0&&c<=324);Self{rows:vec![vec![0;(c+63)/64];r],cols:c}}
 fn bit(&self,r:usize,c:usize)->bool {(self.rows[r][c/64]>>(c%64))&1==1}
 fn flip(&mut self,r:usize,c:usize){self.rows[r][c/64]^=1u64<<(c%64);}
 fn rank(&self)->usize {
  let mut pivots:Vec<Option<Vec<u64>>>=vec![None;self.cols];let mut rank=0;
  for original in &self.rows {let mut row=original.clone();loop {
   let pivot=row.iter().enumerate().rev().find(|(_,w)|**w!=0).map(|(i,w)|64*i+63-w.leading_zeros() as usize);
   match pivot {None=>break,Some(p)=>{assert!(p<self.cols);if let Some(v)=&pivots[p]{for(a,b) in row.iter_mut().zip(v){*a^=*b;}}else{pivots[p]=Some(row);rank+=1;break;}}}
  }} rank
 }
}
fn read_matrix(text:&str,name:&str)->Matrix {
 let prefix=format!("matrix {name}[");let declaration=text.split(';').find(|s|s.trim().starts_with(&prefix)).expect("matrix declaration").trim();
 let sizes=declaration.strip_prefix(&prefix).unwrap().trim_end_matches(']').split("][").map(|s|s.parse::<usize>().unwrap()).collect::<Vec<_>>();assert_eq!(sizes.len(),2);
 let mut matrix=Matrix::zero(sizes[0],sizes[1]);let entry=format!("{name}[");let mut seen=HashSet::new();
 for clause in text.split(';').map(str::trim).filter(|s|s.starts_with(&entry)) {
  let (indices,value)=clause.strip_prefix(&entry).unwrap().split_once("]=").expect("entry syntax");
  let (r,c)=indices.split_once(',').unwrap();let r=r.parse::<usize>().unwrap();let c=c.parse::<usize>().unwrap();
  assert!(r>0&&r<=sizes[0]&&c>0&&c<=sizes[1]);assert!(seen.insert((r,c)),"duplicate entry");assert_eq!(value,"1","binary entry required");matrix.flip(r-1,c-1);
 } matrix
}
fn product_zero(m:&Matrix,k:&Matrix)->bool {
 assert_eq!(m.cols,k.rows.len());
 let mut columns=vec![vec![0;(m.cols+63)/64];k.cols];
 for i in 0..k.rows.len(){for j in 0..k.cols{if k.bit(i,j){columns[j][i/64]|=1u64<<(i%64);}}}
 m.rows.iter().all(|row|columns.iter().all(|col|row.iter().zip(col).fold(0u32,|p,(a,b)|p^((a&b).count_ones()&1))==0))
}
fn check(path:&str,family:&str,count:usize){
 let text=fs::read_to_string(path).expect("native coefficient certificate");
 let m=read_matrix(&text,"oddMatrix62");let k=read_matrix(&text,"kernelBasis62");
 assert_eq!(m.cols,count);assert_eq!(k.rows.len(),count);assert!(product_zero(&m,&k));
 let rank=m.rank();let kernel_rank=k.rank();assert_eq!(kernel_rank,k.cols);assert_eq!(rank+kernel_rank,count);
 let index=(0..count).find(|&c|m.rows.iter().enumerate().any(|(r,_)|m.bit(r,c))).expect("nonzero odd column");
 let mut changed=k.clone();changed.flip(index,0);assert!(!product_zero(&m,&changed),"changed kernel must fail");
 let (ki,kj)=(0..count).flat_map(|i|(0..k.cols).map(move|j|(i,j))).find(|&(i,j)|k.bit(i,j)).expect("nonzero kernel");
 let mut changed_m=m.clone();changed_m.flip(0,ki);assert!(!product_zero(&changed_m,&k),"changed row must fail");
 assert!(k.bit(ki,kj));
 println!("RUSTCHECK62_RESULT family={family} functions={count} binary_rows={} odd_rank={rank} square_dimension={kernel_rank} full_kernel_product=1 changed_kernel_rejected=1 changed_row_rejected=1",m.rows.len());
}
fn main(){let args=env::args().collect::<Vec<_>>();assert_eq!(args.len(),3);check(&args[1],"mixed",76);check(&args[2],"cubic",324);println!("RUSTCHECK62_ALL_CHECKS_PASS");}
