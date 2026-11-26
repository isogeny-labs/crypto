// Frozen exact component control for #98. Six source relation rows are from
// EC_GRAPH_RANK_GP_V2.gp; known log(P0)=1 is supplied as an explicit input.
// This checks substitution and witnesses, not an exhausted-prime selection policy.
const FIELD:i64=1009;
const ORDER:i64=1013;
const A:[[i64;3];3]=[[1,1,0],[0,1,1],[1,0,1]];
const B:[[i64;3];3]=[[1,1,0],[0,1,1],[1,0,-1]];
const AR:[i64;3]=[3,5,4];
const BR:[i64;3]=[3,5,-2];
#[derive(Clone,Copy,Debug,Eq,PartialEq)] enum Pt { O, A(i64,i64) }
fn m(x:i64,n:i64)->i64{x.rem_euclid(n)}
fn inv(x:i64,n:i64)->i64{let(mut a,mut b,mut u,mut v)=(m(x,n),n,1,0);while b!=0{let q=a/b;(a,b)=(b,a-q*b);(u,v)=(v,u-q*v)}assert_eq!(a,1);m(u,n)}
fn plus(a:Pt,b:Pt)->Pt{match(a,b){(Pt::O,q)|(q,Pt::O)=>q,(Pt::A(x,y),Pt::A(u,v))=>{if x==u&&m(y+v,FIELD)==0{return Pt::O}let s=if x==u{m((3*x*x+1)*inv(2*y,FIELD),FIELD)}else{m((v-y)*inv(u-x,FIELD),FIELD)};let xx=m(s*s-x-u,FIELD);Pt::A(xx,m(s*(x-xx)-y,FIELD))}}}
fn mul(mut n:i64,mut q:Pt)->Pt{if n<0{n=-n;q=match q{Pt::O=>Pt::O,Pt::A(x,y)=>Pt::A(x,m(-y,FIELD))}}let mut out=Pt::O;while n>0{if n&1!=0{out=plus(out,q)}q=plus(q,q);n>>=1}out}
fn rank2(rows:[[i64;2];3])->usize{let mut z=rows;let mut h=0;for col in 0..2{if let Some(k)=(h..3).find(|&i|m(z[i][col],ORDER)!=0){z.swap(h,k);let p=inv(z[h][col],ORDER);for j in col..2{z[h][j]=m(z[h][j]*p,ORDER)}let pivot=z[h];for i in 0..3{if i!=h{let q=z[i][col];for j in col..2{z[i][j]=m(z[i][j]-q*pivot[j],ORDER)}}}h+=1}}h}
fn solve(rows:[[i64;2];3],rhs:[i64;3])->[i64;2]{let mut z=[[0;3];3];for i in 0..3{z[i]=[m(rows[i][0],ORDER),m(rows[i][1],ORDER),m(rhs[i],ORDER)]}let mut h=0;for col in 0..2{let k=(h..3).find(|&i|z[i][col]!=0).expect("rank 2");z.swap(h,k);let p=inv(z[h][col],ORDER);for j in col..3{z[h][j]=m(z[h][j]*p,ORDER)}let pivot=z[h];for i in 0..3{if i!=h{let q=z[i][col];for j in col..3{z[i][j]=m(z[i][j]-q*pivot[j],ORDER)}}}h+=1}assert_eq!(z[2],[0,0,0]);[z[0][2],z[1][2]]}
fn main(){
 let g=Pt::A(0,425);assert_eq!(mul(ORDER,g),Pt::O);let points=[g,mul(2,g),mul(3,g)];
 let mut original_rows=0;let mut substituted_rows=0;let mut blind_invalid=0;
 for (label,rows,rhs) in [("A",A,AR),("B",B,BR)]{
  let mut reduced=[[0;2];3];let mut shifted=[0;3];let mut nnz_old=0;let mut nnz_new=0;
  for i in 0..3{
   let original=(0..3).fold(Pt::O,|s,j|plus(s,mul(rows[i][j],points[j])));
   assert_eq!(original,mul(rhs[i],g));original_rows+=1;
   reduced[i]=[rows[i][1],rows[i][2]];shifted[i]=m(rhs[i]-rows[i][0],ORDER);
   let after=plus(mul(reduced[i][0],points[1]),mul(reduced[i][1],points[2]));
   assert_eq!(after,mul(shifted[i],g));substituted_rows+=1;
   if after!=mul(rhs[i],g){blind_invalid+=1}
   nnz_old+=rows[i].iter().filter(|&&v|v!=0).count();
   nnz_new+=reduced[i].iter().filter(|&&v|v!=0).count();
  }
  assert_eq!(rank2(reduced),2);let logs=solve(reduced,shifted);assert_eq!(logs,[2,3]);
  for i in 0..3{assert_eq!(m(rows[i][0]+rows[i][1]*logs[0]+rows[i][2]*logs[1],ORDER),m(rhs[i],ORDER))}
  assert_eq!(mul(logs[1],g),points[2]);
  println!("matrix={label} rows=3 original_nnz={nnz_old} reduced_nnz={nnz_new} reduced_rank=2 recovered_logs={logs:?} target_P2_covered=true");
 }
 assert_eq!((original_rows,substituted_rows,blind_invalid),(6,6,4));
 println!("original_rows_checked={original_rows} substituted_rows_checked={substituted_rows} blind_drop_invalid={blind_invalid} known_log_P0=1 group_order={ORDER}");
 println!("cost_scope=reused_rows_only known_log_acquisition_unpriced=true collector_cost_unpriced=true elimination_cost_unpriced=true");
}
