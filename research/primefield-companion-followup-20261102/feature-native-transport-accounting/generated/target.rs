//! Reflection-odd solve with bidirectional nonsingular power columns.
#[path="recurrence.rs"] mod recurrence;
#[derive(Debug,PartialEq)]pub enum Error{Domain,Limit,OutsideSpan{rank:usize},Certificate}
#[derive(Debug)]pub struct Program{pub terms:Vec<(u64,Vec<usize>)>,pub rank:usize,pub columns:usize,pub matrix_entries:usize,pub elimination_updates:u64}
fn mul(a:u64,b:u64,r:u64)->u64{crate::meter::hit(crate::meter::Component::Target,crate::meter::Op::Product);((a as u128*b as u128)%r as u128)as u64}
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
pub fn construct_target(r:u64,q:usize,degree:usize,target:&[u64],max_rows:usize,max_columns:usize)->Result<Program,Error>{
 if !(3..=4).contains(&degree)||r<3||r>u64::MAX/2||r&1==0||q==0||q>64||q as u64>=r{return Err(Error::Domain);}
 let full_rows=usize::try_from(r).map_err(|_|Error::Limit)?;if full_rows>max_rows{return Err(Error::Limit);}let rows=(full_rows-1)/2;
 if target.len()!=full_rows||target.iter().any(|&v|v>=r)||target.iter().step_by(2).any(|&v|v!=0){return Err(Error::Domain);}
 let mut labels=vec![vec![]];for a in 0..q{labels.push(vec![a]);}for a in 0..q{for b in a+1..q{labels.push(vec![a,b]);}}
 for a in 0..q{for b in a+1..q{for c in b+1..q{labels.push(vec![a,b,c]);}}}
 if degree==4{for a in 0..q{for b in a+1..q{for c in b+1..q{for d in c+1..q{labels.push(vec![a,b,c,d]);}}}}}
 let half=(r+1)/2;let center=mul((q-1)as u64,half,r);let epsilon_negative=((r-1)/2)&1!=0;
 let mut orbits=Vec::new();for label in labels{let reflected:Vec<usize>=label.iter().rev().map(|&a|q-1-a).collect();let odd_sign=epsilon_negative&&label.len()%2!=0;if label>reflected||(label==reflected&&!odd_sign){continue;}orbits.push((label,reflected,odd_sign));}
 let m=orbits.len();if m>max_columns{return Err(Error::Limit);}let entries=rows.checked_mul(m+1).ok_or(Error::Limit)?;
 let context=recurrence::Context::new(r,max_rows).map_err(|_|Error::Domain)?;
 let mut columns=Vec::with_capacity(m);for(label,_,_)in &orbits{let mut centered:Vec<usize>=label.iter().map(|&a|((a as u64+r-center)%r)as usize).collect();centered.sort_unstable();let polynomial=context.column(&centered).map_err(|_|Error::Domain)?.0;columns.push((0..rows).map(|i|polynomial[2*i+1]).collect::<Vec<_>>());}
 let mut matrix=vec![vec![0;m+1];rows];for(j,col)in columns.iter().enumerate(){for(i,&v)in col.iter().enumerate(){matrix[i][j]=v;}}for i in 0..rows{matrix[i][m]=target[2*i+1];}
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
 for(i,&v)in certificate.iter().enumerate(){if v!=target[2*i+1]{return Err(Error::Certificate);}}
 let mut terms=Vec::new();if center!=0{terms.push((r-center,vec![]));}
 for(c,(label,reflected,odd_sign))in coefficients.into_iter().zip(orbits){if c==0{continue;}if label==reflected{terms.push((c,label));}else{let first=mul(c,half,r);terms.push((first,label));terms.push((if odd_sign{first}else{r-first},reflected));}}
 Ok(Program{terms,rank,columns:m,matrix_entries:entries,elimination_updates:updates})
}
