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
fn quotient36(label:&str,v:&[Series],target:&Space,gens:&[Series],expected:usize,table:&[[u8;32];32],out:&mut impl Write)->Vec<Series>{let mut candidate=v.to_vec();let mut products=0;for (g,gen) in gens.iter().enumerate(){let mut residuals=vec![];for c in &candidate{let value=mul(c,gen,table);assert_eq!(value,mul_independent(c,gen));products+=1;residuals.push(residual36(pack(&value),target));}if residuals.iter().any(|r|top(r).is_some()){let previous:Vec<Bits>=candidate.iter().map(pack).collect();let ker=kernel36(&residuals);writeln!(out,"step {label} {} {} {}",g+1,candidate.len(),ker.len()).unwrap();for r in &residuals{write!(out,"residual").unwrap();write_bits(out,r);}let mut next=vec![];for co in ker{let value=combine(&co,&previous);write!(out,"kernel").unwrap();write_bits(out,&co);write!(out,"candidate").unwrap();write_bits(out,&value);next.push(unpack(&value));}candidate=next;series_basis36(&candidate);println!("QUOT36_STEP label={label} generator={} dimension={}",g+1,candidate.len());}if expected!=usize::MAX{assert!(candidate.len()>=expected,"quotient below expected dimension");}}
 if expected!=usize::MAX{assert_eq!(candidate.len(),expected);}else{assert!(candidate.len()<=1);}series_basis36(&candidate);let mut allchecks=0;for gen in gens{for c in &candidate{let value=mul(c,gen,table);assert_eq!(value,mul_independent(c,gen));assert_eq!(residual36(pack(&value),target),[0;W]);allchecks+=1;}}println!("QUOT36_QUOTIENT label={label} generators={} dimension={} construction_products={products} final_memberships={allchecks} independent_products=all",gens.len(),candidate.len());candidate}
fn read_basis36(path:&str)->Vec<Series>{let text=fs::read_to_string(path).unwrap();let mut out=vec![];for line in text.lines(){if line.starts_with("cubic "){continue;}let p:Vec<_>=line.split_whitespace().collect();assert_eq!(p.len(),157);assert_eq!(p[0].parse::<usize>().unwrap(),out.len()+1);let mut v=[0;N];for i in 0..N{v[i]=p[i+3].parse().unwrap();assert!(v[i]<32);}out.push(v);}assert_eq!(out.len(),196);out}
fn product_space36(s:&Series,values:&[Series],table:&[[u8;32];32])->Space{let products:Vec<_>=values.iter().map(|v|{let p=mul(s,v,table);assert_eq!(p,mul_independent(s,v));p}).collect();series_basis36(&products)}
fn flip36(label:&str,v:&[Series],d:&[Series],table:&[[u8;32];32],out:&mut impl Write,expected:usize)->Vec<Series>{let s=d[0];let target=product_space36(&s,v,table);assert_eq!(target.basis.len(),324);quotient36(label,v,&target,d,expected,table,out)}
fn equality36(label:&str,v:&[Series],d:&[Series],e:&[Series],expected:usize,table:&[[u8;32];32],out:&mut impl Write){let target=product_space36(&d[0],e,table);assert_eq!(target.basis.len(),196);let w=quotient36(label,v,&target,d,expected,table,out);assert_eq!(w.len(),expected);println!("GROUP36_EQUALITY label={label} equivalent={} witness_dimension={expected}",expected==1);}
fn addflip36(label:&str,v:&[Series],d:&[Series],e:&[Series],table:&[[u8;32];32],out:&mut impl Write)->Vec<Series>{let complement=flip36(&format!("{label}-first-flip"),v,d,table,out,68);let target=product_space36(&d[0],e,table);assert_eq!(target.basis.len(),196);let sum=quotient36(&format!("{label}-sum-divisor"),v,&target,&complement,68,table,out);flip36(&format!("{label}-last-flip"),v,&sum,table,out,196)}
fn negate37(label:&str,v:&[Series],base:&[Series],d:&[Series],table:&[[u8;32];32],out:&mut impl Write)->Vec<Series>{let ds=series_basis36(d);let middle=quotient36(&format!("{label}-middle"),v,&ds,base,68,table,out);flip36(&format!("{label}-flip"),v,&middle,table,out,196)}
fn add37(label:&str,v:&[Series],base:&[Series],d:&[Series],e:&[Series],table:&[[u8;32];32],out:&mut impl Write)->Vec<Series>{let af=addflip36(&format!("{label}-addflip"),v,d,e,table,out);negate37(&format!("{label}-negate"),v,base,&af,table,out)}
fn same37(label:&str,v:&[Series],d:&[Series],e:&[Series],table:&[[u8;32];32],out:&mut impl Write)->bool{let target=product_space36(&d[0],e,table);assert_eq!(target.basis.len(),196);let w=quotient36(label,v,&target,d,usize::MAX,table,out);assert!(w.len()<=1);println!("SIGNED37_EQUALITY label={label} witness_dimension={} equivalent={}",w.len(),!w.is_empty());!w.is_empty()}
fn save37(dir:&str,label:&str,values:&[Series]){let mut file=BufWriter::new(fs::File::create(format!("{dir}/{label}-basis.txt")).unwrap());for (i,val) in values.iter().enumerate(){write!(file,"{}",i+1).unwrap();for x in val{write!(file," {x}").unwrap();}writeln!(file).unwrap();}file.flush().unwrap();}
fn read_class37(path:&str)->Vec<Series>{let text=fs::read_to_string(path).unwrap();let mut out=vec![];for line in text.lines(){let p:Vec<_>=line.split_whitespace().collect();assert_eq!(p.len(),155);assert_eq!(p[0].parse::<usize>().unwrap(),out.len()+1);let mut v=[0;N];for i in 0..N{v[i]=p[i+1].parse().unwrap();assert!(v[i]<32);}out.push(v);}assert_eq!(out.len(),196);out}
fn multiply37(label:&str,v:&[Series],base:&[Series],d:&[Series],n:u64,table:&[[u8;32];32],out:&mut impl Write)->Vec<Series>{assert!(n>0);let top=63-n.leading_zeros();let mut value=d.to_vec();for k in (0..top).rev(){value=add37(&format!("{label}-double-{k}"),v,base,&value,&value,table,out);if(n>>k)&1!=0{value=add37(&format!("{label}-add-{k}"),v,base,&value,d,table,out);}}value}
type Residual41=[i64;2];
fn ra41(a:Residual41,b:Residual41)->Residual41{[(a[0]+b[0]).rem_euclid(11),(a[1]+b[1]).rem_euclid(11)]}
fn rs41(a:Residual41,c:i64)->Residual41{[(a[0]*c).rem_euclid(11),(a[1]*c).rem_euclid(11)]}
fn rm41(a:Residual41,b:Residual41)->Residual41{[(a[0]*b[0]-a[1]*b[1]).rem_euclid(11),(a[0]*b[1]+a[1]*b[0]+4*a[1]*b[1]).rem_euclid(11)]}
fn rp41(mut a:Residual41,mut n:usize)->Residual41{let mut r=[1,0];while n>0{if n&1!=0{r=rm41(r,a);}n>>=1;if n>0{a=rm41(a,a);}}r}
fn rq41(k:usize)->Residual41{ra41(ra41([1,0],rp41([0,1],k)),rs41(rp41([0,1],2*k),2))}
fn evalr41(a:Residual41,t:i64)->i64{(a[0]+a[1]*t).rem_euclid(11)}
fn operators41()->(Residual41,Residual41){
 assert_eq!(rp41([0,1],10),[1,0]);let mut sum=[0,0];for j in 0..5{sum=ra41(sum,rp41([0,1],2*j));}
 let e=rm41(ra41([1,0],rs41(rp41([0,1],5),-1)),ra41([5,0],rs41(sum,-1)));assert_eq!(e,[10,0]);
 let b=3721%11;assert_eq!(b,3);
 let p7=rs41(rm41(rm41(e,rq41(7)),rq41(6)),b);let p8=rs41(rm41(rm41(e,rq41(4)),rq41(3)),b);
 assert_eq!(p7,[6,2]);assert_eq!(p8,[3,9]);assert_eq!(rs41(ra41(p7,p8),5),[1,0]);
 assert_eq!([evalr41(p7,7),evalr41(p7,8),evalr41(p8,7),evalr41(p8,8)],[9,0,0,9]);
 assert_ne!(evalr41(ra41(p7,[1,0]),7),9);
 println!("IMAGES41_POLYNOMIAL quotient=t^2-4t+1 Pi7=6+2t Pi8=3+9t recovery_scalar=5 corrupt_operator=rejected");(p7,p8)
}
fn linear41(label:&str,c:Residual41,v:&[Series],base:&[Series],zero:&[Series],f:&[Series],g:&[Series],table:&[[u8;32];32],out:&mut impl Write)->Vec<Series>{
 let scalar=|s:&[Series],n:i64,out:&mut _|{if n==0{zero.to_vec()}else if n==1{s.to_vec()}else{multiply37(label,v,base,s,n as u64,table,out)}};
 let first=scalar(f,c[0],out);let second=scalar(g,c[1],out);add37(label,v,base,&first,&second,table,out)
}
fn main(){
 let a:Vec<String>=env::args().collect();assert_eq!(a.len(),7);let spaces=read(&a[1]);let v=&spaces[2];let base=&spaces[0];let zero=&spaces[1];
 let f=read_class37(&format!("{}/F-basis.txt",a[2]));let g=read_class37(&format!("{}/G-basis.txt",a[2]));
 let p7=read_class37(&format!("{}/P7-basis.txt",a[2]));let p8=read_class37(&format!("{}/P8-basis.txt",a[2]));let old=read_class37(&a[3]);
 let metadata=fs::read_to_string(&a[4]).unwrap();let norm:u64=metadata.split("number normMinus37=").nth(1).unwrap().split(';').next().unwrap().parse().unwrap();assert_eq!(norm,3721);
 for s in [&f,&g,&p7,&p8,&old]{series_basis36(s);}
 let mut table=[[0u8;32];32];for i in 0..32{for j in 0..32{table[i][j]=field(i as u8,j as u8);assert_eq!(table[i][j],field_independent(i as u8,j as u8));}}
 fs::create_dir(&a[5]).unwrap();let mut out=BufWriter::new(fs::File::create(format!("{}/image-certificates.txt",a[5])).unwrap());
 let (c7,c8)=operators41();let pi7=linear41("full-operator-seven",c7,v,base,zero,&f,&g,&table,&mut out);let pi8=linear41("full-operator-eight",c8,v,base,zero,&f,&g,&table,&mut out);
 let nine7=multiply37("nine-component-seven",v,base,&p7,9,&table,&mut out);let nine8=multiply37("nine-component-eight",v,base,&p8,9,&table,&mut out);
 assert!(same37("operator-seven-versus-nine-component",v,&pi7,&nine7,&table,&mut out));assert!(same37("operator-eight-versus-nine-component",v,&pi8,&nine8,&table,&mut out));
 let sum=add37("operator-image-sum",v,base,&pi7,&pi8,&table,&mut out);let recovered=multiply37("operator-recovery-five",v,base,&sum,5,&table,&mut out);assert!(same37("operator-recovery-versus-F",v,&recovered,&f,&table,&mut out));
 let pi7zero=same37("operator-seven-zero",v,&pi7,zero,&table,&mut out);let pi8zero=same37("operator-eight-zero",v,&pi8,zero,&table,&mut out);
 assert!(!same37("old-R-nonzero-control",v,&old,zero,&table,&mut out));let old11=multiply37("old-R-eleven-control",v,base,&old,11,&table,&mut out);assert!(same37("old-R-eleven-zero",v,&old11,zero,&table,&mut out));
 let mut seven_matches=vec![];let mut eight_matches=vec![];let mut current=zero.to_vec();
 for k in 0..11{
  if k==1{current=old.clone();}else if k>1{current=add37(&format!("old-R-multiple-{k}"),v,base,&current,&old,&table,&mut out);}
  let eq7=same37(&format!("component-seven-old-multiple-{k}"),v,&p7,&current,&table,&mut out);let eq8=same37(&format!("component-eight-old-multiple-{k}"),v,&p8,&current,&table,&mut out);
  if eq7{seven_matches.push(k);}if eq8{eight_matches.push(k);}save37(&a[5],&format!("old-R-multiple-{k}"),&current);
 }assert!(seven_matches.len()<=1);assert!(eight_matches.len()<=1);
 let m7=seven_matches.first().map(|k|k.to_string()).unwrap_or_else(||"none".to_string());let m8=eight_matches.first().map(|k|k.to_string()).unwrap_or_else(||"none".to_string());
 save37(&a[5],"operator-seven",&pi7);save37(&a[5],"operator-eight",&pi8);save37(&a[5],"operator-recovered",&recovered);out.flush().unwrap();
 fs::write(&a[6],format!("operator7_principal={pi7zero}\noperator8_principal={pi8zero}\nrecovery=true\ncomponent7_old_scalar={m7}\ncomponent8_old_scalar={m8}\nscalar_candidates_tested=11\n")).unwrap();
 println!("IMAGES41_RESULT operator7_principal={pi7zero} operator8_principal={pi8zero} actual_operator_linear_forms=6F+2TF,3F+9TF independent_component_forms=9P7,9P8 recovery_scalar=5 exact_recovery=true component7_old_scalar={m7} component8_old_scalar={m8} all_old_scalar_candidates=11 old_eleven_control=pass all_generator_quotients=checked independent_products=all actual_source_operator_images_evaluated=true chosen_Koblitz_model_isogeny_evaluated=false");
 println!("IMAGES41_ALL_CHECKS_PASS");
}
