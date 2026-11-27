// Exact controls on already verified public EC relation matrices A/B.
// A/B rows come from E/F_1009 in EC_GRAPH_RANK_GP_V2.gp; no timing claim.
const R:i64=1013;
const A:[[i64;3];3]=[[1,1,0],[0,1,1],[1,0,1]];
const B:[[i64;3];3]=[[1,1,0],[0,1,1],[1,0,-1]];
fn m(x:i64)->i64{x.rem_euclid(R)}
fn inv(x:i64)->i64{let(mut a,mut b,mut u,mut v)=(m(x),R,1,0);while b!=0{let q=a/b;(a,b)=(b,a-q*b);(u,v)=(v,u-q*v);}assert_eq!(a,1);m(u)}
fn rank(mut a:[[i64;3];3])->usize{let mut h=0;for c in 0..3{if let Some(k)=(h..3).find(|&k|m(a[k][c])!=0){a.swap(h,k);let q=inv(a[h][c]);for j in c..3{a[h][j]=m(a[h][j]*q);}let v=a[h];for i in 0..3{if i!=h{let q=a[i][c];for j in c..3{a[i][j]=m(a[i][j]-q*v[j]);}}}h+=1;}}h}
fn det(a:[[i64;3];3])->i64{a[0][0]*(a[1][1]*a[2][2]-a[1][2]*a[2][1])-a[0][1]*(a[1][0]*a[2][2]-a[1][2]*a[2][0])+a[0][2]*(a[1][0]*a[2][1]-a[1][1]*a[2][0])}
fn support(a:[[i64;3];3])->[[bool;3];3]{a.map(|row|row.map(|x|x!=0))}
fn cut(s:[[bool;3];3],mask:usize)->usize{
 let mut edges=[[false;3];3];for row in s{for i in 0..3{for j in i+1..3{if row[i]&&row[j]{edges[i][j]=true;}}}}
 (0..3).map(|i|(i+1..3).filter(|&j|edges[i][j]&&((mask>>i)&1)!=((mask>>j)&1)).count()).sum()
}
fn main(){
 let sa=support(A);let sb=support(B);assert_eq!(sa,sb);
 let sq_a=A.map(|row|row.map(|x|x*x));let sq_b=B.map(|row|row.map(|x|x*x));assert_eq!(sq_a,sq_b);
 let (ra,rb)=(rank(A),rank(B));assert_eq!((ra,rb),(3,2));
 let maxcut=(1..7).map(|mask|cut(sa,mask)).max().unwrap();assert_eq!(maxcut,2);
 let diagonal=[[1,0,0],[0,1,0],[0,0,R]];
 assert_eq!(det(diagonal),R);assert_eq!(rank(diagonal),2);
 // On the verified EC base logs (1,2,3), all three diagonal rows are
 // group-valid: [1]P, [2]P, [1013][3]P=O.
 let logs=[1,2,3];let rhs=[1,2,0];for i in 0..3{let lhs=(0..3).map(|j|diagonal[i][j]*logs[j]).sum::<i64>();assert_eq!(m(lhs),rhs[i]);}
 // Independent prior native control checked #E(F_17)=19, G=(5,1) order19.
 // Multiplicative F_17^x has order16. Compute first possible extension degree.
 let mut k=0;let mut z=1;loop{k+=1;z=z*17%19;if z==1{break;}assert!(k<19);}
 assert_eq!(k,9);assert_eq!(16%19,16);
 println!("ec_source=EC_GRAPH_RANK_GP_V2 curve_p=1009 group_order=1013");
 println!("A_B_same_support=true A_B_same_squared_coefficients=true max_cut_both={maxcut} rank_A={ra} rank_B={rb}");
 println!("diag_integer_det={} diag_real_rank=3 diag_mod_1013_rank={} diag_EC_rows_valid=3",det(diagonal),rank(diagonal));
 println!("E_F17_order=19 F17_multiplicative_order=16 first_extension_with_19_divides_17k_minus1={k}");
}
