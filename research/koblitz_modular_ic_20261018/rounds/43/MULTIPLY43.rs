//! Exact GF(32)[z]/z^615 products. No timings or attack claims.
use std::{env,fs,io::{BufWriter,Write}};
const N:usize=615; const W:usize=49;
type Series=[u8;N]; type Bits=[u64;W];
fn field(mut a:u8,mut b:u8)->u8{let mut c=0;for _ in 0..5{if b&1!=0{c^=a;}b>>=1;a<<=1;if a&32!=0{a^=0x25;}}c}
fn field_independent(a:u8,b:u8)->u8{let mut c=0u16;for i in 0..5{for j in 0..5{if (a>>i)&1!=0 && (b>>j)&1!=0{c^=1<<(i+j);}}}for k in (5..=8).rev(){if c&(1<<k)!=0{c^=0x25<<(k-5);}}c as u8}
fn mul(a:&Series,b:&Series,table:&[[u8;32];32])->Series{let mut c=[0;N];for i in 0..N{if a[i]!=0{for j in 0..N-i{c[i+j]^=table[a[i] as usize][b[j] as usize];}}}c}
fn plane(a:&Series)->[[u64;10];5]{let mut p=[[0;10];5];for i in 0..N{for k in 0..5{if (a[i]>>k)&1!=0{p[k][i/64]^=1<<(i%64);}}}p}
fn binary_conv(a:&[u64;10],b:&[u64;10])->[u64;10]{let mut c=[0;10];for i in 0..10{let mut v=a[i];while v!=0{let s=v.trailing_zeros() as usize;v&=v-1;for j in 0..10-i{c[i+j]^=b[j]<<s;if s!=0 && i+j+1<10{c[i+j+1]^=b[j]>>(64-s);}}}}c[9]&=(1u64<<39)-1;c}
fn mul_independent(a:&Series,b:&Series)->Series{let a=plane(a);let b=plane(b);let mut p=[[0u64;10];9];for i in 0..5{for j in 0..5{let c=binary_conv(&a[i],&b[j]);for k in 0..10{p[i+j][k]^=c[k];}}}for i in (5..=8).rev(){for k in 0..10{p[i-5][k]^=p[i][k];p[i-3][k]^=p[i][k];}}let mut c=[0;N];for i in 0..N{for k in 0..5{c[i]^=(((p[k][i/64]>>(i%64))&1) as u8)<<k;}}c}
fn pack(a:&Series)->Bits{let mut b=[0;W];for i in 0..N{for k in 0..5{if (a[i]>>k)&1!=0{let p=5*i+k;b[p/64]^=1<<(p%64);}}}b}
fn unpack(b:&Bits)->Series{let mut a=[0;N];for i in 0..N{for k in 0..5{let p=5*i+k;a[i]^=(((b[p/64]>>(p%64))&1) as u8)<<k;}}a}
fn xor(a:&mut Bits,b:&Bits){for i in 0..W{a[i]^=b[i];}}
fn top(a:&Bits)->Option<usize>{for i in (0..W).rev(){if a[i]!=0{return Some(i*64+63-a[i].leading_zeros() as usize);}}None}
fn bit(i:usize)->Bits{let mut a=[0;W];a[i/64]=1<<(i%64);a}
fn combine(co:&Bits,basis:&[Bits])->Bits{let mut a=[0;W];for i in 0..W{let mut v=co[i];while v!=0{let j=i*64+v.trailing_zeros() as usize;v&=v-1;assert!(j<basis.len());xor(&mut a,&basis[j]);}}a}
struct Space{piv:Vec<Option<(Bits,Bits)>>,basis:Vec<Bits>}
impl Space{
 fn new()->Self{Self{piv:vec![None;3075],basis:vec![]}}
 fn insert(&mut self,a:Bits)->(Bits,bool){let mut v=a;let mut co=[0;W];while let Some(p)=top(&v){if let Some((b,c))=&self.piv[p]{xor(&mut v,b);xor(&mut co,c);}else{let label=bit(self.basis.len());xor(&mut co,&label);self.piv[p]=Some((v,co));self.basis.push(a);return(label,true);}}(co,false)}
 fn check(&self){for (p,bc) in self.piv.iter().enumerate(){if let Some((b,c))=bc{assert_eq!(top(b),Some(p));assert_eq!(combine(c,&self.basis),*b);}}}
}
fn read(path:&str)->Vec<Vec<Series>>{let text=fs::read_to_string(path).unwrap();let mut spaces=vec![vec![],vec![],vec![]];for line in text.lines(){let mut parts=line.split_whitespace();let s:usize=parts.next().unwrap().parse().unwrap();let i:usize=parts.next().unwrap().parse().unwrap();assert!((1..=3).contains(&s));assert_eq!(i,spaces[s-1].len()+1);let values:Vec<u8>=parts.next().unwrap().split(',').map(|x|x.parse().unwrap()).collect();assert_eq!(values.len(),N);assert!(values.iter().all(|&x|x<32));assert!(parts.next().is_none());let a:Series=values.try_into().unwrap();assert_eq!(unpack(&pack(&a)),a);spaces[s-1].push(a);}assert_eq!(spaces.iter().map(Vec::len).collect::<Vec<_>>(),vec![68,196,324]);spaces}
fn write_bits(out:&mut impl Write,a:&Bits){for v in a{write!(out," {v:016x}").unwrap();}writeln!(out).unwrap();}
fn stage(input:&[Series],desired:usize,label:&str,root:&str,table:&[[u8;32];32])->Vec<Series>{
 let mut input_rank=Space::new();for x in input{assert!(input_rank.insert(pack(x)).1);}input_rank.check();
 let mut space=Space::new();let mut basis=vec![];let mut selected=BufWriter::new(fs::File::create(format!("{root}/{label}-basis.txt")).unwrap());let mut co_out=BufWriter::new(fs::File::create(format!("{root}/{label}-coordinates.bin")).unwrap());let mut count=0;
 'outer:for i in 0..input.len(){for j in i..input.len(){let val=mul(&input[i],&input[j],table);assert_eq!(val,mul_independent(&input[i],&input[j]),"{label} product {i}/{j}");let bits=pack(&val);let(co,new)=space.insert(bits);assert_eq!(combine(&co,&space.basis),bits);
  co_out.write_all(&(i as u32).to_le_bytes()).unwrap();co_out.write_all(&(j as u32).to_le_bytes()).unwrap();for x in co{co_out.write_all(&x.to_le_bytes()).unwrap();}
  if new{basis.push(val);write!(selected,"{} {} {} ",basis.len(),i+1,j+1).unwrap();for(k,x)in val.iter().enumerate(){if k>0{write!(selected,",").unwrap();}write!(selected,"{x}").unwrap();}writeln!(selected).unwrap();}
  count+=1;if basis.len()==desired{break 'outer;}
 }if i%32==0{println!("MULT43_STAGE label={label} outer={i} attempted={count} rank={}",basis.len());}}
 assert_eq!(basis.len(),desired,"finite product pool exhausted {label}");space.check();
 let mut out=BufWriter::new(fs::File::create(format!("{root}/{label}-pivots.txt")).unwrap());for(p,bc)in space.piv.iter().enumerate(){if let Some((b,c))=bc{write!(out,"pivot {p}").unwrap();write_bits(&mut out,b);write!(out,"coordinates {p}").unwrap();write_bits(&mut out,c);}}
 let val=mul(&input[0],&input[0],table);let mut bad=pack(&val);xor(&mut bad,&bit(3074));assert_ne!(bad,pack(&mul_independent(&input[0],&input[0])));let(mut co,_)=space.insert(pack(&val));xor(&mut co,&bit(0));assert_ne!(combine(&co,&space.basis),pack(&val));
 selected.flush().unwrap();co_out.flush().unwrap();out.flush().unwrap();
 fs::write(format!("{root}/{label}-result.txt"),format!("input_rank={}\noutput_rank={desired}\nattempted_products={count}\nindependent_products={count}\nall_recombinations=checked\nstop=expected_rank_reached\n",input.len())).unwrap();println!("MULT43_COMPLETE label={label} input_rank={} output_rank={desired} attempted_products={count} independent_products={count} all_recombinations=checked stop=expected_rank_reached",input.len());basis
}
fn main(){let a:Vec<_>=env::args().collect();assert_eq!(a.len(),3);let spaces=read(&a[1]);fs::create_dir(&a[2]).unwrap();let mut table=[[0u8;32];32];for i in 0..32{for j in 0..32{table[i][j]=field(i as u8,j as u8);assert_eq!(table[i][j],field_independent(i as u8,j as u8));}}
 let _four=stage(&spaces[1],452,"L4D0",&a[2],&table);let six=stage(&spaces[2],708,"L6D0",&a[2],&table);let twelve=stage(&six,1476,"L12D0",&a[2],&table);let _twentyfour=stage(&twelve,3012,"L24D0",&a[2],&table);
 println!("MULT43_RESULT precision=615 evaluation_dimension=3075 ranks=452,708,1476,3012 all_attempted_products_independent_checked=true all_original_recombinations=checked complete_RR_interpretation=conditional_on_retained_proper_pole_bridge geometric_transfer_evaluated=false");println!("MULT43_ALL_CHECKS_PASS");}
