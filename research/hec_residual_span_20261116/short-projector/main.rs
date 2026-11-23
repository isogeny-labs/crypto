#[path="../canonical-root/root.rs"]mod root;
#[path="../canonical-root/fast.rs"]mod fast;
use hec_native_ic::{counted,fp::Fp,poly::Poly,jac::{Curve,Div}};
use std::collections::{HashMap,HashSet};
fn norm(a:&[i128])->i128{a.iter().map(|x|x*x).sum()}
fn nearest(a:i128,b:i128)->i128{if a>=0{(a+b/2)/b}else{-((-a+b/2)/b)}}
fn short(h:u64,r:&[i128])->(Vec<i128>,Vec<i128>,usize){
 let n=r.len();let mut basis=vec![];for j in 0..n{basis.push((0..n).map(|k|r[(k+n-j)%n]).collect::<Vec<_>>());}basis.push(vec![1;n]);
 let mut a=vec![0i128;n];a[0]=h as i128;let mut witness=vec![0;n+1];let mut moves=0;
 loop{let mut best=None;let old=norm(&a);for(j,b)in basis.iter().enumerate(){let nb=norm(b);if nb==0{continue;}let dot=a.iter().zip(b).map(|(x,y)|x*y).sum();let q=nearest(dot,nb);let decrease=2*q*dot-q*q*nb;if decrease>0&&best.as_ref().map_or(true,|&(_,_,v)|decrease>v){best=Some((j,q,decrease));}}
 let Some((j,q,_))=best else{break};assert!(moves<10000);for k in 0..n{a[k]-=q*basis[j][k];}witness[j]+=q;moves+=1;assert!(norm(&a)<old);}
 for k in 0..n{assert_eq!((if k==0{h as i128}else{0})-a[k],basis.iter().zip(&witness).map(|(b,w)|b[k]*w).sum::<i128>());}
 (a,witness,moves)
}
fn eval(c:&Curve,x:u64,y:u64,z:u64,a:&[i128])->Div{
 let mut terms=vec![];let mut xx=x;for &v in a{if v!=0{terms.push((xx,if v<0{c.fp.neg(y)}else{y},u64::try_from(v.abs()).unwrap()));}xx=c.fp.mul(xx,z);}
 let bits=terms.iter().map(|t|64-t.2.leading_zeros()).max().unwrap_or(0);let mut out=c.zero();
 for bit in (0..bits).rev(){if !c.is_zero(&out){out=c.add(&out,&out);}for &(x,y,v)in &terms{if(v>>bit)&1==1{out=c.add_place(&out,x,y);}}}out
}
fn main(){
 for (g,p,c,ell,h,expected) in [(2usize,251u64,3u64,101u64,55u64,24usize),(2,1021,4,631,1655,102),(3,211,2,301267,29,14),(4,181,2,48781,4445,10),(4,271,2,397,112347,9)]{
  counted::disable();let n=2*g+1;let mut cs=vec![0;n+1];cs[0]=c;cs[n]=1;let curve=Curve::new(Fp::new(p),Poly::from_coeffs(&cs));let(fb,z,pc)=fast::prepare(&curve);let mut r:Vec<i128>=pc.iter().map(|v|*v as i128).collect();r[0]-=1;let(a,witness,moves)=short(h,&r);
  println!("CERTIFICATE g={g} p={p} c={c} h={h} rotation={z} frobenius_minus_one={r:?} short={a:?} witness={witness:?} greedy_moves={moves}");
  let normpoly=vec![1i128;n];for &(x,y)in &fb{let raw=curve.place(x,y);assert!(curve.is_zero(&eval(&curve,x,y,z,&r)));assert!(curve.is_zero(&eval(&curve,x,y,z,&normpoly)));assert_eq!(eval(&curve,x,y,z,&a),curve.mul(&raw,h));}
  let byx:HashMap<_,_>=fb.iter().copied().collect();let mut seen=HashSet::new();let mut reps=vec![];for &(x,y)in &fb{if x==0||seen.contains(&x){continue;}reps.push((x,y));let mut xx=x;for _ in 0..n{assert_eq!(byx[&xx],y);assert!(seen.insert(xx));xx=curve.fp.mul(xx,z);}assert_eq!(xx,x);}assert_eq!(reps.len(),expected);
  for arm in ["scalar","polynomial"]{let mut prior=None;for rep in 0..2{let places:Vec<_>=reps.iter().map(|&(x,y)|curve.place(x,y)).collect();counted::reset();let outputs:Vec<_>=reps.iter().zip(&places).map(|(&(x,y),d)|if arm=="scalar"{curve.mul(d,h)}else{eval(&curve,x,y,z,&a)}).collect();let counts=counted::snapshot();counted::disable();for(d,&(x,y))in outputs.iter().zip(&reps){assert!(curve.is_valid(d));assert!(curve.is_zero(&curve.mul(d,ell)));assert_eq!(*d,curve.mul(&curve.place(x,y),h));}if let Some(ref old)=prior{assert_eq!(old,&(counts.clone(),outputs.clone()));}else{prior=Some((counts.clone(),outputs));}
   for(modulus,v)in counts{assert_eq!(v[8],2*v[7]+v[5]+v[6]);println!("COUNT g={g} p={p} arm={arm} rep={rep} modulus={modulus} vector={v:?}");}
  }}
 }
 eprintln!("PASS exact_integer_operator_witness=true all_canonical_places_R_norm_annihilation_and_scalar_equivalence=true counted_scalar_vs_joint_place_kernel=true repeated_vectors_exact=true free_preparation=true no_global_shortest_or_full_solver_claim=true");
}
