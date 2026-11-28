// Frozen support-only controls for ideas 82, 98, 100 on the six verified
// E(F_1009) relation rows of EC_GRAPH_RANK_GP_V2.gp. No timings.
const R:i64=1013;
const A:[[i64;3];3]=[[1,1,0],[0,1,1],[1,0,1]];
const B:[[i64;3];3]=[[1,1,0],[0,1,1],[1,0,-1]];
const AR:[i64;3]=[3,5,4];
const BR:[i64;3]=[3,5,-2];
const LOGS:[i64;3]=[1,2,3];
fn m(x:i64)->i64{x.rem_euclid(R)}
fn inv(x:i64)->i64{let(mut a,mut b,mut u,mut v)=(m(x),R,1,0);while b!=0{let q=a/b;(a,b)=(b,a-q*b);(u,v)=(v,u-q*v);}assert_eq!(a,1);m(u)}
fn rank(mut a:[[i64;3];3])->usize{
 let mut h=0;
 for c in 0..3{if let Some(k)=(h..3).find(|&k|m(a[k][c])!=0){a.swap(h,k);let q=inv(a[h][c]);for j in c..3{a[h][j]=m(a[h][j]*q);}let pivot=a[h];for i in 0..3{if i!=h{let q=a[i][c];for j in c..3{a[i][j]=m(a[i][j]-q*pivot[j]);}}}h+=1;}}
 h
}
fn dot(row:[i64;3])->i64{m((0..3).map(|j|row[j]*LOGS[j]).sum())}
fn matching_size(s:[[bool;3];3])->usize{
 let mut best=0;
 for i in 0..3{for j in 0..3{for k in 0..3{
  if i!=j&&i!=k&&j!=k&&s[0][i]&&s[1][j]&&s[2][k]{best=3;}
 }}}
 best
}
fn main(){
 let sa=A.map(|r|r.map(|x|x!=0));let sb=B.map(|r|r.map(|x|x!=0));assert_eq!(sa,sb);
 let counts:[usize;3]=std::array::from_fn(|j|(0..3).filter(|&i|sa[i][j]).count());
 assert_eq!(counts,[2,2,2]);
 for i in 0..3{assert_eq!(dot(A[i]),m(AR[i]));assert_eq!(dot(B[i]),m(BR[i]));}
 let (ra,rb)=(rank(A),rank(B));assert_eq!((ra,rb),(3,2));
 let matching=matching_size(sa);assert_eq!(matching,3);
 let mut invalid_after_drop=0;
 for i in 0..3{let mut row=A[i];row[0]=0;if dot(row)!=m(AR[i]){invalid_after_drop+=1;}}
 assert_eq!(invalid_after_drop,2);
 println!("source=EC_GRAPH_RANK_GP_V2 curve_p=1009 subgroup_order={R} rows=6");
 println!("support_equal=true column_nonzero_counts={counts:?} column_support_denominator=3");
 println!("bipartite_matching_A={matching} bipartite_matching_B={matching} rank_A={ra} rank_B={rb}");
 println!("drop_column_0_from_A_invalid_EC_rows={invalid_after_drop}");
}
