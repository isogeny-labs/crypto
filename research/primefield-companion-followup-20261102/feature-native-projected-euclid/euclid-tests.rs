fn power(mut a:u64,mut n:u64,p:u64)->u64{let mut v=1;while n>0{if n&1!=0{v=v*a%p;}a=a*a%p;n>>=1;}v}
fn determinant(a:&[u64],b:&[u64],p:u64)->u64{
 let(m,n)=(a.len()-1,b.len()-1);let size=m+n;let mut mat=vec![vec![0u64;size];size];
 for i in 0..n{for j in 0..=m{mat[i][i+j]=a[m-j]%p;}}
 for i in 0..m{for j in 0..=n{mat[n+i][i+j]=b[n-j]%p;}}
 let mut result=1;
 for i in 0..size{let Some(k)=(i..size).find(|&k|mat[k][i]!=0)else{return 0;};if i!=k{mat.swap(i,k);result=(p-result)%p;}
  let v=mat[i][i];result=result*v%p;let inv=power(v,p-2,p);
  for k in i+1..size{let c=mat[k][i]*inv%p;for j in i..size{mat[k][j]=(mat[k][j]+p-c*mat[i][j]%p)%p;}}
 }result
}
#[test]fn projected_euclid_matches_determinants_and_roots(){
 let(mut polys,mut roots_count,mut zero)=(0,0,0);
 for p in [5u64,7,13,29,509]{let order=1u64<<(p-1).trailing_zeros();let odd=(p-1)/order;
  for m in 0..=5usize{for n in 0..=5usize{for seed in 0..6u64{
   let mut a:Vec<_>=(0..=m).map(|i|(seed*7+i as u64*11+3)%p).collect();let mut b:Vec<_>=(0..=n).map(|i|(seed*13+i as u64*i as u64+1)%p).collect();a[m]=seed%(p-1)+1;b[n]=(seed+1)%(p-1)+1;
   let det=determinant(&a,&b,p);let mut engine=Poly::new(p).unwrap();assert_eq!(engine.projected_resultant(&a,&b).unwrap(),power(det,odd,p),"p={p} m={m} n={n} seed={seed}");polys+=1;zero+=usize::from(det==0);
  }}}
  for n in [1usize,2,3,12,13,27,65]{for m in [1usize,2,3,12,13,27,65]{
   let roots:Vec<_>=(0..n).map(|i|(i*i+11)as u64%(2*p)).collect();let points:Vec<_>=(0..m).map(|i|(i*7+3)as u64%(2*p)).collect();let mut e=Poly::new(p).unwrap();let got=e.root_projected_resultant(&roots,&points).unwrap();
   let direct=points.iter().fold(1,|v,&x|roots.iter().fold(v,|w,&y|w*((x%p+p-y%p)%p)%p));assert_eq!(got,power(direct,odd,p));let old=e.root_resultant(&roots,&points).unwrap();assert_eq!(got,power(old,odd,p));roots_count+=1;
  }}
  let mut e=Poly::new(p).unwrap();assert_eq!(e.projected_resultant(&[0],&[1,1]),Ok(0));assert_eq!(e.projected_resultant(&[1,1],&[0]),Ok(0));assert_eq!(e.root_projected_resultant(&[],&[1]),Err(Error::EmptyTree));assert_eq!(e.root_projected_resultant(&[1],&[]),Err(Error::EmptyTree));
 }
 assert_eq!(Poly::new(17).unwrap().projected_resultant(&[1,1],&[2,1]),Err(Error::Domain));
 println!("polynomial_cases={polys} zero_determinants={zero} root_cases={roots_count}");
}
