// Native systematic diagnostic for idea #73: RS(6,4) evaluation coding of
// six independently checked EC relation rows. Fixed erasures [1,4], or one
// corrupted evaluation at index 2. This repairs storage records, not relations.
const MOD:i64=1013;const P:i64=1009;
const ROWS:[[i64;4];6]=[
 [1,1,0,3],[0,1,1,5],[1,0,1,4],
 [1,1,0,3],[0,1,1,5],[1,0,-1,-2],
];
#[derive(Clone,Copy,Eq,PartialEq,Debug)]enum Pt{O,A(i64,i64)}
fn m(x:i64,n:i64)->i64{x.rem_euclid(n)}
fn inv(x:i64,n:i64)->i64{let(mut a,mut b,mut u,mut v)=(m(x,n),n,1,0);while b!=0{let q=a/b;(a,b)=(b,a-q*b);(u,v)=(v,u-q*v);}assert_eq!(a,1);m(u,n)}
fn neg(q:Pt)->Pt{match q{Pt::O=>q,Pt::A(x,y)=>Pt::A(x,m(-y,P))}}
fn add(q:Pt,t:Pt)->Pt{match(q,t){(Pt::O,v)|(v,Pt::O)=>v,(Pt::A(x,y),Pt::A(u,v))=>{if x==u&&m(y+v,P)==0{return Pt::O;}let s=if x==u{m((3*x*x+1)*inv(2*y,P),P)}else{m((v-y)*inv(u-x,P),P)};let xx=m(s*s-x-u,P);Pt::A(xx,m(s*(x-xx)-y,P))}}}
fn mul(mut k:i64,mut q:Pt)->Pt{if k<0{k=-k;q=neg(q);}let mut out=Pt::O;while k>0{if k&1==1{out=add(out,q);}q=add(q,q);k>>=1;}out}
fn normalize(row:[i64;4])->[i64;4]{row.map(|x|m(x,MOD))}
fn eval(c:[i64;4],x:i64)->i64{m(((c[3]*x+c[2])*x+c[1])*x+c[0],MOD)}
fn encode(c:[i64;4])->[i64;6]{std::array::from_fn(|j|eval(c,(j+1)as i64))}
fn interpolate(shards:&[(usize,i64);4])->[i64;4]{
 let mut a=[[0i64;5];4];
 for (i,&(j,y)) in shards.iter().enumerate(){let x=(j+1)as i64;let mut power=1;for c in 0..4{a[i][c]=power;power=m(power*x,MOD);}a[i][4]=y;}
 for c in 0..4{let pivot=(c..4).find(|&i|a[i][c]!=0).unwrap();a.swap(c,pivot);let z=inv(a[c][c],MOD);for j in c..5{a[c][j]=m(a[c][j]*z,MOD);}let row=a[c];for i in 0..4{if i!=c{let z=a[i][c];for j in c..5{a[i][j]=m(a[i][j]-z*row[j],MOD);}}}}
 std::array::from_fn(|i|a[i][4])
}
fn recover_one_error(shards:[i64;6])->([i64;4],usize,usize){
 let mut matches=Vec::new();
 for suspect in 0..6{let good:Vec<(usize,i64)>=(0..6).filter(|&i|i!=suspect).map(|i|(i,shards[i])).collect();
  let first:[(usize,i64);4]=good[..4].try_into().unwrap();let c=interpolate(&first);
  if good.iter().all(|&(i,y)|eval(c,(i+1)as i64)==y){matches.push((c,suspect));}
 }
 assert_eq!(matches.len(),1);(matches[0].0,matches[0].1,6)
}
fn checksum(c:[i64;4])->i64{m(c.iter().enumerate().map(|(i,&v)|(i as i64+1)*v).sum(),MOD)}
fn verify_ec(c:[i64;4],g:Pt)->bool{let points=[g,mul(2,g),mul(3,g)];let lhs=(0..3).fold(Pt::O,|a,j|add(a,mul(c[j],points[j])));lhs==mul(c[3],g)}
fn main(){let g=Pt::A(0,425);assert_eq!(mul(MOD,g),Pt::O);let mut repaired_erasure=0;let mut repaired_error=0;
 for (id,&orig) in ROWS.iter().enumerate(){let source=normalize(orig);assert!(verify_ec(source,g));let coded=encode(source);
  let kept=[(0,coded[0]),(2,coded[2]),(3,coded[3]),(5,coded[5])];let from_erasures=interpolate(&kept);
  assert_eq!(from_erasures,source);assert!(verify_ec(from_erasures,g));repaired_erasure+=1;
  let mut corrupt=coded;corrupt[2]=m(corrupt[2]+7,MOD);let (from_error,suspect,tries)=recover_one_error(corrupt);
  assert_eq!(suspect,2);assert_eq!(tries,6);assert_eq!(from_error,source);assert!(verify_ec(from_error,g));repaired_error+=1;
  let mut raw_error=source;raw_error[2]=m(raw_error[2]+7,MOD);assert_ne!(checksum(raw_error),checksum(source));
  println!("row={id} erasures=1,4 recovered=true one_error_index={suspect} recovered_error=true ec_identity=true");
 }
 println!("rows=6 erasure_recovered={repaired_erasure} one_error_recovered={repaired_error} checksum_detected_raw_error=6 checksum_recovered_erasures=0");
 println!("storage_symbols_per_row raw=4 raw_plus_checksum=5 rs6_4=6 rs_over_raw=1.5 rs_over_checksum=1.2");
 println!("encode_horner_per_row_mul=18 encode_horner_per_row_add=18 error_decode_interpolation_attempts_per_row=6 decoding_field_ops_unpriced=true");
}
