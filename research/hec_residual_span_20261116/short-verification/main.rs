#[path="../canonical-root/root.rs"]mod root;
#[path="../canonical-root/fast.rs"]mod fast;
mod reduce;
use hec_native_ic::{counted,fp::Fp,poly::Poly,jac::{Curve,Div},rng::Rng};
fn evaluate(c:&Curve,d:&Div,z:u64,a:&[i128])->Div{
 let n=a.len();let mut powers=vec![1;n];for j in 1..n{powers[j]=c.fp.mul(powers[j-1],z);}
 let degree=d.u.deg() as usize;let mut terms=vec![];
 for(j,&v)in a.iter().enumerate(){if v==0{continue;}let mut q=*d;
 if j>0{let mut u=vec![0;degree+1];let mut vv=vec![0;d.v.n];for i in 0..degree{u[i]=c.fp.mul(d.u.c[i],powers[(j*(degree-i))%n]);}u[degree]=1;for i in 0..d.v.n{vv[i]=if i==0{d.v.c[i]}else{c.fp.mul(d.v.c[i],powers[(n-(j*i)%n)%n])};}q=Div{u:Poly::from_coeffs(&u),v:Poly::from_coeffs(&vv)};}
 if v<0{q=c.neg(&q);}terms.push((q,u64::try_from(v.abs()).unwrap()));}
 let bits=terms.iter().map(|q|64-q.1.leading_zeros()).max().unwrap_or(0);let mut out=c.zero();for bit in(0..bits).rev(){if !c.is_zero(&out){out=c.add(&out,&out);}for(q,v)in &terms{if(v>>bit)&1==1{out=if q.u.deg()==1{c.add_place(&out,c.fp.neg(q.u.c[0]),q.v.c[0])}else{c.add(&out,q)}}}}out
}
fn main(){
 let old:Vec<_>=include_str!("../source-phases/CONTROL.tsv").lines().skip(1).map(|l|l.split('\t').collect::<Vec<_>>()).collect();let mut cases=0;
 for(p,cc,ell)in[(101u64,2u64,7481u64),(251,18,67931),(601,7,411491)]{
 counted::disable();let curve=Curve::new(Fp::new(p),Poly::from_coeffs(&[cc,0,0,0,0,1]));let(fb,z,pc)=fast::prepare(&curve);let base=curve.place(fb[0].0,fb[0].1);let mut relation:Vec<_>=pc.iter().map(|v|*v as i128).collect();relation[0]-=1;let mut rng=Rng::new(2026113000^p);
 for sm in[1,17]{let d=curve.mul(&base,sm);assert!(!curve.is_zero(&d));assert!(curve.is_zero(&curve.mul(&d,ell)));assert!(curve.is_zero(&evaluate(&curve,&d,z,&relation)));assert!(curve.is_zero(&evaluate(&curve,&d,z,&[1;5])));
 for case in 0..2{let k=1+rng.below(ell-1);let target=curve.mul(&d,k);let(a,witness,moves)=reduce::short(k,&relation);println!("CERTIFICATE p={p} sm={sm} case={case} k={k} rotation={z} R={relation:?} short={a:?} witness={witness:?} greedy_moves={moves}");
 assert_eq!(evaluate(&curve,&d,z,&a),target);let(wrong,_,_)=reduce::short((k+1)%ell,&relation);assert_ne!(evaluate(&curve,&d,z,&wrong),target);assert_ne!(evaluate(&curve,&d,z,&a),curve.neg(&target));assert_ne!(evaluate(&curve,&curve.neg(&d),z,&a),target);
 for arm in["scalar","polynomial"]{let mut prior=None;for rep in 0..2{counted::reset();let answer=if arm=="scalar"{curve.mul(&d,k)}else{let(coeff,_,_)=reduce::short(k,&relation);evaluate(&curve,&d,z,&coeff)};let counts=counted::snapshot();counted::disable();assert_eq!(answer,target);if let Some(ref old)=prior{assert_eq!(old,&counts);}else{prior=Some(counts.clone());}
 let v=counts.get(&p).copied().unwrap_or([0;16]);assert_eq!(v[8],2*v[7]+v[5]+v[6]);if arm=="scalar"{let row=old.iter().find(|x|x[0]==p.to_string()&&x[1]==sm.to_string()&&x[2]==case.to_string()&&x[3]==rep.to_string()&&x[4]=="verify"&&x[5]==p.to_string()).unwrap();let expected:Vec<u64>=row[6].split(',').map(|s|s.parse().unwrap()).collect();assert_eq!(v.as_slice(),expected.as_slice());}println!("COUNT p={p} sm={sm} case={case} arm={arm} rep={rep} vector={v:?}");
 }}cases+=1;
 }}}
 assert_eq!(cases,12);eprintln!("PASS fixed_original_targets=12 scalar_phase_replay_exact=true short_scalar_equivalence=true operator_premises_on_source_checked=true wrong_scalar_target_source_controls_rejected=true repeat_vectors_exact=true certified_operator_contract_conditional=true no_full_solver_speed_claim=true");
}
