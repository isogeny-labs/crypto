//! Cubic character construction in F_r[X]/(X^r-X), with caller-certified prime r.
#[derive(Debug,PartialEq)]pub enum Error{Domain,Limit,OutsideSpan{rank:usize},Certificate}
#[derive(Debug)]pub struct Program{pub terms:Vec<(u64,Vec<usize>)>,pub rank:usize,pub columns:usize,pub matrix_entries:usize,pub elimination_updates:u64}
fn mul(a:u64,b:u64,r:u64)->u64{((a as u128*b as u128)%r as u128)as u64}
fn pow(mut a:u64,mut n:u64,r:u64)->u64{let mut out=1;while n>0{if n&1!=0{out=mul(out,a,r);}a=mul(a,a,r);n>>=1;}out}
fn product(a:&[u64],b:&[u64],r:u64)->Vec<u64>{
 let size=r as usize;let mut c=vec![0;size];
 for(i,&u)in a.iter().enumerate(){if u==0{continue;}for(j,&v)in b.iter().enumerate(){if v==0{continue;}let k=i+j;let k=if k>=size{k-(size-1)}else{k};c[k]=(c[k]+mul(u,v,r))%r;}}
 c
}
fn shifted_power(r:u64,a:usize)->Vec<u64>{
 let mut v=vec![1];for _ in 0..(r-1)/2{let mut w=vec![0;v.len()+1];for(i,&c)in v.iter().enumerate(){w[i]=(w[i]+mul(c,a as u64,r))%r;w[i+1]=(w[i+1]+c)%r;}v=w;}v.resize(r as usize,0);v
}
/// Exact dense reference backend. Limits apply before coefficient-space allocation.
/// No scalar evaluations or old program coefficients enter this constructor.
pub fn construct(r:u64,q:usize,max_rows:usize,max_columns:usize)->Result<Program,Error>{
 if r<3||r>u64::MAX/2||r&1==0||q==0||q>64||q as u64>=r{return Err(Error::Domain);}
 let rows=usize::try_from(r).map_err(|_|Error::Limit)?;if rows>max_rows{return Err(Error::Limit);}
 let mut labels=vec![vec![]];for a in 0..q{labels.push(vec![a]);}for a in 0..q{for b in a+1..q{labels.push(vec![a,b]);}}
 for a in 0..q{for b in a+1..q{for c in b+1..q{labels.push(vec![a,b,c]);}}}
 let m=labels.len();if m>max_columns{return Err(Error::Limit);}let entries=rows.checked_mul(m+1).ok_or(Error::Limit)?;
 let single:Vec<_>=(0..q).map(|a|shifted_power(r,a)).collect();
 let mut columns=Vec::with_capacity(m);for label in &labels{let mut v=vec![0;rows];v[0]=1;for &a in label{v=product(&v,&single[a],r);}columns.push(v);}
 let mut matrix=vec![vec![0;m+1];rows];for(j,col)in columns.iter().enumerate(){for(i,&v)in col.iter().enumerate(){matrix[i][j]=v;}}matrix[1][m]=1;
 let mut pivots=vec![];let mut updates=0u64;
 for col in 0..m{let pivot=pivots.len();if pivot==rows{break;}let Some(found)=(pivot..rows).find(|&i|matrix[i][col]!=0)else{continue;};matrix.swap(pivot,found);
  let inverse=pow(matrix[pivot][col],r-2,r);if mul(inverse,matrix[pivot][col],r)!=1{return Err(Error::Domain);}
  for j in col..=m{matrix[pivot][j]=mul(matrix[pivot][j],inverse,r);}
  let normalized=matrix[pivot].clone();for row in matrix.iter_mut().skip(pivot+1){let factor=row[col];if factor==0{continue;}for j in col..=m{row[j]=(row[j]+r-mul(factor,normalized[j],r))%r;updates+=1;}}
  pivots.push(col);
 }
 let rank=pivots.len();if matrix.iter().skip(rank).any(|row|row[m]!=0){return Err(Error::OutsideSpan{rank});}
 let mut coefficients=vec![0;m];for i in(0..rank).rev(){let col=pivots[i];let mut v=matrix[i][m];for j in col+1..m{v=(v+r-mul(matrix[i][j],coefficients[j],r))%r;}coefficients[col]=v;}
 let mut certificate=vec![0;rows];for(j,&c)in coefficients.iter().enumerate(){for(i,v)in certificate.iter_mut().enumerate(){*v=(*v+mul(c,columns[j][i],r))%r;}}
 for(i,&v)in certificate.iter().enumerate(){if v!=u64::from(i==1){return Err(Error::Certificate);}}
 let terms=coefficients.into_iter().zip(labels).filter(|(c,_)|*c!=0).collect();Ok(Program{terms,rank,columns:m,matrix_entries:entries,elimination_updates:updates})
}
#[cfg(test)]#[path="../feature-native-centered-bitplanes/decoder.rs"]mod decoder;
#[cfg(test)]#[path="../feature-native-seed-factor/routed/source.rs"]mod source;
#[cfg(test)]mod tests{
 use super::*;
 fn chi(x:u64,r:u64)->i8{if x==0{return 0;}if(1..r).any(|a|a*a%r==x){1}else{-1}}
 fn direct(r:u64,terms:&[(u64,Vec<usize>)],x:u64)->u64{let mut sum=0i128;for(c,label)in terms{let mut sign=1i128;for &a in label{sign*=chi((x+a as u64)%r,r)as i128;}sum+=*c as i128*sign;}sum.rem_euclid(r as i128)as u64}
 #[test]fn quotient_zero_and_wrap(){
  for r in [3u64,7,11,31]{let n=r as usize;for i in 0..n{for j in 0..n{let mut a=vec![0;n];a[i]=1;let mut b=vec![0;n];b[j]=1;let c=product(&a,&b,r);for x in 0..r{let value=c.iter().rev().fold(0,|v,&u|(mul(v,x,r)+u)%r);assert_eq!(value,mul(pow(x,i as u64,r),pow(x,j as u64,r),r));}}}}
 }
 #[test]fn limits_and_small_span(){
  assert!(matches!(construct(479,16,478,697),Err(Error::Limit)));assert!(matches!(construct(479,16,479,696),Err(Error::Limit)));
  assert!(matches!(construct(7,1,7,2),Err(Error::OutsideSpan{..})));assert!(matches!(construct(8,2,8,4),Err(Error::Domain)));
  let p=construct(7,3,7,8).unwrap();for x in 0..7{assert_eq!(direct(7,&p.terms,x),x);}
 }
 #[test]fn saved_modulus_constructor_and_control(){
  let control=construct(479,15,479,576);assert!(matches!(control,Err(Error::OutsideSpan{..})));println!("negative_control={control:?}");
  let p=construct(479,16,479,697).unwrap();println!("constructed rank={} columns={} matrix_entries={} nonzero_terms={} elimination_updates={}",p.rank,p.columns,p.matrix_entries,p.terms.len(),p.elimination_updates);
  for x in 0..479{assert_eq!(direct(479,&p.terms,x),x);}
  let compiled=decoder::Program::new(479,16,&p.terms).unwrap();let source=source::Source::new(509,2,10,479,[3,68]).unwrap();let source=source.prepare().unwrap();
  for line in include_str!("../feature-native-recovery/targets.tsv").lines(){let f:Vec<i64>=line.split('\t').map(|v|v.parse().unwrap()).collect();let q=if f[1]<0{None}else{Some([f[1]as u64,f[2]as u64])};let window=source.window(q,16).unwrap();assert_eq!(compiled.evaluate(|a|window.signs[a]),Ok(f[0]as u64));}
  if let Ok(path)=std::env::var("CONSTRUCTOR_PACKET_OUT"){let mut text=String::new();for(c,label)in &p.terms{text.push_str(&format!("{}\t{}\n",c,label.iter().map(|a|a.to_string()).collect::<Vec<_>>().join(",")));}std::fs::write(path,text).unwrap();}
 }
}
