//! Exact GF(32)[z]/z^154 products. No timings or attack claims.
use std::{env,fs,io::{BufWriter,Write}};
const N:usize=154; const W:usize=13;
type Series=[u8;N]; type Bits=[u64;W];
fn field(mut a:u8,mut b:u8)->u8{let mut c=0;for _ in 0..5{if b&1!=0{c^=a;}b>>=1;a<<=1;if a&32!=0{a^=0x25;}}c}
fn field_independent(a:u8,b:u8)->u8{let mut c=0u16;for i in 0..5{for j in 0..5{if (a>>i)&1!=0 && (b>>j)&1!=0{c^=1<<(i+j);}}}for k in (5..=8).rev(){if c&(1<<k)!=0{c^=0x25<<(k-5);}}c as u8}
fn mul(a:&Series,b:&Series,table:&[[u8;32];32])->Series{let mut c=[0;N];for i in 0..N{if a[i]!=0{for j in 0..N-i{c[i+j]^=table[a[i] as usize][b[j] as usize];}}}c}
fn plane(a:&Series)->[[u64;3];5]{let mut p=[[0;3];5];for i in 0..N{for k in 0..5{if (a[i]>>k)&1!=0{p[k][i/64]^=1<<(i%64);}}}p}
fn binary_conv(a:&[u64;3],b:&[u64;3])->[u64;3]{let mut c=[0;3];for i in 0..3{let mut v=a[i];while v!=0{let s=v.trailing_zeros() as usize;v&=v-1;for j in 0..3-i{c[i+j]^=b[j]<<s;if s!=0 && i+j+1<3{c[i+j+1]^=b[j]>>(64-s);}}}}c[2]&=(1<<26)-1;c}
fn mul_independent(a:&Series,b:&Series)->Series{let a=plane(a);let b=plane(b);let mut p=[[0u64;3];9];for i in 0..5{for j in 0..5{let c=binary_conv(&a[i],&b[j]);for k in 0..3{p[i+j][k]^=c[k];}}}for i in (5..=8).rev(){for k in 0..3{p[i-5][k]^=p[i][k];p[i-3][k]^=p[i][k];}}let mut c=[0;N];for i in 0..N{for k in 0..5{c[i]^=(((p[k][i/64]>>(i%64))&1) as u8)<<k;}}c}
fn pack(a:&Series)->Bits{let mut b=[0;W];for i in 0..N{for k in 0..5{if (a[i]>>k)&1!=0{let p=5*i+k;b[p/64]^=1<<(p%64);}}}b}
fn unpack(b:&Bits)->Series{let mut a=[0;N];for i in 0..N{for k in 0..5{let p=5*i+k;a[i]^=(((b[p/64]>>(p%64))&1) as u8)<<k;}}a}
fn xor(a:&mut Bits,b:&Bits){for i in 0..W{a[i]^=b[i];}}
fn top(a:&Bits)->Option<usize>{for i in (0..W).rev(){if a[i]!=0{return Some(i*64+63-a[i].leading_zeros() as usize);}}None}
fn bit(i:usize)->Bits{let mut a=[0;W];a[i/64]=1<<(i%64);a}
fn combine(co:&Bits,basis:&[Bits])->Bits{let mut a=[0;W];for i in 0..W{let mut v=co[i];while v!=0{let j=i*64+v.trailing_zeros() as usize;v&=v-1;assert!(j<basis.len());xor(&mut a,&basis[j]);}}a}
struct Space{piv:Vec<Option<(Bits,Bits)>>,basis:Vec<Bits>}
impl Space{
 fn new()->Self{Self{piv:vec![None;770],basis:vec![]}}
 fn insert(&mut self,a:Bits)->(Bits,bool){let mut v=a;let mut co=[0;W];while let Some(p)=top(&v){if let Some((b,c))=&self.piv[p]{xor(&mut v,b);xor(&mut co,c);}else{let label=bit(self.basis.len());xor(&mut co,&label);self.piv[p]=Some((v,co));self.basis.push(a);return(label,true);}}(co,false)}
 fn check(&self){for (p,bc) in self.piv.iter().enumerate(){if let Some((b,c))=bc{assert_eq!(top(b),Some(p));assert_eq!(combine(c,&self.basis),*b);}}}
}
fn read(path:&str)->Vec<Vec<Series>>{let text=fs::read_to_string(path).unwrap();let mut spaces=vec![vec![],vec![],vec![]];for line in text.lines(){let mut parts=line.split_whitespace();let s:usize=parts.next().unwrap().parse().unwrap();let i:usize=parts.next().unwrap().parse().unwrap();assert!((1..=3).contains(&s));assert_eq!(i,spaces[s-1].len()+1);let values:Vec<u8>=parts.next().unwrap().split(',').map(|x|x.parse().unwrap()).collect();assert_eq!(values.len(),N);assert!(values.iter().all(|&x|x<32));assert!(parts.next().is_none());let a:Series=values.try_into().unwrap();assert_eq!(unpack(&pack(&a)),a);spaces[s-1].push(a);}assert_eq!(spaces.iter().map(Vec::len).collect::<Vec<_>>(),vec![68,196,324]);spaces}
fn write_bits(out:&mut impl Write,a:&Bits){for v in a{write!(out," {v:016x}").unwrap();}writeln!(out).unwrap();}
fn residual36(mut v:Bits,space:&Space)->Bits{while let Some(p)=top(&v){if let Some((b,_))=&space.piv[p]{xor(&mut v,b);}else{let mut ans=v;for q in (0..p).rev(){if (ans[q/64]>>(q%64))&1!=0{if let Some((b,_))=&space.piv[q]{xor(&mut ans,b);}}}return ans;}}v}
fn series_basis36(values:&[Series])->Space{let mut space=Space::new();for v in values{assert!(space.insert(pack(v)).1);}space.check();space}
fn kernel36(cols:&[Bits])->Vec<Bits>{let mut piv:Vec<Option<(Bits,Bits)>>=vec![None;770];let mut ker=vec![];let mut rank=0;for (i,col) in cols.iter().enumerate(){let mut v=*col;let mut co=bit(i);while let Some(p)=top(&v){if let Some((b,c))=&piv[p]{xor(&mut v,b);xor(&mut co,c);}else{piv[p]=Some((v,co));rank+=1;break;}}if top(&v).is_none(){assert_eq!(combine(&co,cols),[0;W]);ker.push(co);}}assert_eq!(rank+ker.len(),cols.len());let mut independent=Space::new();for k in &ker{assert!(independent.insert(*k).1);}independent.check();ker}
fn read_collector47(path:&str)->Vec<Vec<Vec<u16>>>{
 let text=fs::read_to_string(path).unwrap();let mut out=vec![vec![],vec![],vec![]];
 for line in text.lines(){let p:Vec<_>=line.split_whitespace().collect();assert_eq!(p.len(),19);let g:usize=p[0].parse().unwrap();if g<3{continue;}assert!(g<=5);let i:usize=p[1].parse().unwrap();assert_eq!(i,out[g-3].len()+1);let mut words=vec![];
 for pair in &p[2..]{let v:Vec<u16>=pair.split(',').map(|x|x.parse().unwrap()).collect();assert_eq!(v.len(),2);assert!(v[0]<8192);assert_eq!(v[1],0);words.push(v[0]);}assert_eq!(words.len(),17);out[g-3].push(words);
 }assert_eq!(out.iter().map(Vec::len).collect::<Vec<_>>(),vec![68,196,324]);out
}
fn constraint47(q:&Series,p:&[u16])->Bits{
 assert_eq!(p.len(),17);let mut bits=[0;W];
 for k in 0..23{for b in 0..5{if(q[k]>>b)&1!=0{let pos=k*5+b;bits[pos/64]^=1<<(pos%64);}}}
 for b in 0..13{if(p[0]>>b)&1!=0{let pos=115+b;bits[pos/64]^=1<<(pos%64);}}
 assert!(top(&bits).is_none_or(|p|p<128));bits
}
fn save40(path:&str,values:&[Series]){
 let mut out=BufWriter::new(fs::File::create(path).unwrap());for(i,v)in values.iter().enumerate(){write!(out,"{}",i+1).unwrap();for x in v{write!(out," {x}").unwrap();}writeln!(out).unwrap();}out.flush().unwrap();
}
fn main(){
 let a:Vec<String>=env::args().collect();assert_eq!(a.len(),5);let global=read(&a[1]);let point_text=fs::read_to_string(&a[2]).unwrap();let points:Vec<_>=point_text.lines().map(|l|{let v:Vec<usize>=l.split_whitespace().map(|x|x.parse().unwrap()).collect();assert_eq!(v.len(),6);(v[0],v[1])}).collect();assert_eq!(points.len(),10);
 fs::create_dir(&a[4]).unwrap();
 let mut certificates=BufWriter::new(fs::File::create(format!("{}/coupling-certificates.txt",a[4])).unwrap());let mut cases=0;
 for &(orbit,deck) in &points{
  let jets=read_collector47(&format!("{}/o{orbit}-d{deck}.txt",a[3]));
  for (g,count,expected,label) in [(1,196,68,"small"),(2,324,196,"large")]{
   let columns:Vec<_>=(0..count).map(|i|constraint47(&global[g][i],&jets[g][i])).collect();
   writeln!(certificates,"case {orbit} {deck} {label} columns={count} rows=128").unwrap();for c in &columns{write!(certificates,"evaluation").unwrap();write_bits(&mut certificates,c);}
   assert!(columns.iter().any(|c|top(c).is_some()),"corrupt nonvanishing column control failed");
   let kernel=kernel36(&columns);assert_eq!(kernel.len(),expected);
   let source:Vec<_>=global[g].iter().map(pack).collect();let values:Vec<_>=kernel.iter().map(|k|{
    assert_eq!(combine(k,&columns),[0;W]);write!(certificates,"kernel").unwrap();write_bits(&mut certificates,k);unpack(&combine(k,&source))
   }).collect();
   let independent=series_basis36(&values);assert_eq!(independent.basis.len(),expected);
   save40(&format!("{}/{label}-o{orbit}-d{deck}-basis.txt",a[4]),&values);
  }
  cases+=1;println!("COUPLE47_POINT orbit={orbit} deck={deck} divisor_degree=128 padding=23Q small_rank=68 large_rank=196 evaluation_rank=128 every_kernel_column_recombined=true");
 }
 assert_eq!(cases,10);certificates.flush().unwrap();
 println!("COUPLE47_RESULT complete_effective_divisors=10 divisor_degree=128 padding_Q_multiplicity=23 point_degree=13 small_spaces=10 large_spaces=10 ranks=68,196 evaluation_rank_each=128 all_kernel_recombinations=checked vanishing_nonmember_control=rejected proper_RR_bridge=retained Picard_group_operations_performed=false factor_images_evaluated=false");
 println!("COUPLE47_ALL_CHECKS_PASS");
}
