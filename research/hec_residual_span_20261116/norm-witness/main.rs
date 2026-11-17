use std::collections::BTreeSet;
type M=Vec<Vec<i128>>;
fn matrix(a:&[i128;7])->M {
 let mut m=vec![vec![0;6];6];
 for j in 0..6 {let mut c=[0i128;13]; for k in 0..7 {c[j+k]+=a[k];}
  for k in (6..13).rev(){let t=c[k];c[k]=0;for r in 0..6{c[k-6+r]-=t;}}
  for i in 0..6{m[i][j]=c[i];}}
 m
}
fn conjugate(a:&[i128;7])->[i128;7]{let mut b=[0;7];for j in 0..7{b[(7-j)%7]=a[j];}b}
fn gal(a:&[i128;7],e:usize)->[i128;7]{let mut b=[0;7];for j in 0..7{b[j*e%7]=a[j];}b}
fn pow(mut a:i128,mut e:u64,p:i128)->i128{let mut b=1;while e>0{if e&1==1{b=b*a%p;}a=a*a%p;e>>=1;}b}
fn rref(m:&M,p:i128)->(usize,M){let mut a=m.clone();for row in &mut a{for x in row{*x=x.rem_euclid(p);}}
 let mut r=0;for j in 0..6{let Some(i)=(r..6).find(|&i|a[i][j]!=0)else{continue};a.swap(i,r);
 let iv=pow(a[r][j],p as u64-2,p);for k in 0..6{a[r][k]=a[r][k]*iv%p;}
 for i in 0..6{if i==r{continue}let t=a[i][j];for k in 0..6{a[i][k]=(a[i][k]-t*a[r][k]).rem_euclid(p);}}
 r+=1;if r==6{break}}
 (r,a)
}
fn witnesses(l:i128)->Vec<[i128;7]>{let alpha=[0,1,1,0,1,0,0];let beta=[1,2,2,0,2,0,0];
 if l==2{return vec![alpha,conjugate(&alpha)]}if l==7{return vec![beta]}
 let(a,b)=match l{11=>(-2,1),23=>(4,1),29=>(1,2),37=>(3,2),43=>(6,1),_=>panic!()};
 let mut q=beta.map(|v|v*b);q[0]+=a;let mut out=vec![q,conjugate(&q)];
 if l==29||l==43{let eta=if l==29{[1,0,-2,0,0,-2,4]}else{[3,-2,-4,2,0,2,0]};for e in 1..7{out.push(gal(&eta,e));}}
 out
}
fn main(){
 let screen=include_str!("../frobenius-primes/CONTROL.tsv");let mut admissions=BTreeSet::new();let mut target_rows=0;
 for line in screen.lines().skip(1){let v:Vec<_>=line.split('\t').collect();if v[0]!="3"{continue}target_rows+=1;
  if v[9]=="false"{assert_eq!(v[6],"6");admissions.insert((v[1].parse::<u64>().unwrap(),v[3].parse::<i128>().unwrap()));}}
 assert_eq!(target_rows,30);assert_eq!(admissions.len(),14);
 println!("p\tl\twitnesses\tdistinct_kernels\trank\tnorm_identity");let mut total=0;
 for p in [113,421]{for(l,expected)in[(2,2),(7,1),(11,2),(23,2),(29,8),(37,2),(43,8)]{
  assert!(admissions.remove(&(p,l)));let ws=witnesses(l);let mut kernels=BTreeSet::new();
  for w in &ws{let a=matrix(w);let b=matrix(&conjugate(w));for i in 0..6{for j in 0..6{
   let n=(0..6).map(|k|a[i][k].checked_mul(b[k][j]).unwrap()).try_fold(0i128,|s,x|s.checked_add(x)).unwrap();
   assert_eq!(n,if i==j{l}else{0});}}
   let(rank,canonical)=rref(&a,l);assert_eq!(rank,3);assert!(kernels.insert(canonical));}
  assert_eq!(kernels.len(),expected);println!("{p}\t{l}\t{}\t{}\t3\ttrue",ws.len(),kernels.len());total+=kernels.len();
 }}assert!(admissions.is_empty());assert_eq!(total,50);eprintln!("model_kernel_combinations={total} exact_norm_rank_distinctness_checks=true conditional_exhaustion_see_derivation=true independent_verification=false");
}
