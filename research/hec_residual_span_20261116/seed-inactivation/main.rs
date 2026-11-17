#[path="../setup/fast.rs"]mod fast;
#[path="../multi/algebra.rs"]mod algebra;
#[path="../orbit-core/sparse.rs"]mod sparse;
use hec_native_ic::{counted,fp::Fp,jac::Curve,poly::Poly,rng::Rng,solve::{Row,SolveStats,Solver,solve_for_last}};
use std::fs;
fn main(){counted::disable();let raw=fs::read_to_string("research/hec_residual_span_20261116/incidence-oracle/ROOTS.tsv").unwrap();println!("p\tsm\tseed\tpasses\tknown\tadmitted\tconstraint_rank\tk");let mut checks=0;
for(p,cc,ell)in[(601,7,411491)]{
 let c=Curve::new(Fp::new(p),Poly::from_coeffs(&[cc,0,0,0,0,1]));let(fb,z,a)=fast::prepare(&c);let(_,_,degree,lambda)=algebra::certificate(&a,ell);assert_eq!(degree,1);let lambda=lambda.unwrap();let f=Fp::new(ell);
 let mut map=vec![(usize::MAX,0);fb.len()];let mut reps=vec![];
 for j in 0..fb.len(){if map[j].0!=usize::MAX{continue}let r=reps.len();reps.push(j);let(mut x,y,mut w)=(fb[j].0,fb[j].1,1);for _ in 0..5{let q=fb.binary_search_by_key(&x,|p|p.0).unwrap();map[q]=(r,if fb[q].1==y{w}else{assert_eq!(fb[q].1,c.fp.neg(y));f.neg(w)});x=c.fp.mul(x,z);w=f.mul(w,lambda);}}
 let cols=reps.len();let base=c.place(fb[0].0,fb[0].1);let mut rng=Rng::new(2026112500^p);
 for sm in [1]{let d=c.mul(&base,sm);let secret=1+rng.below(ell-1);let t=c.mul(&d,secret);let mut input=vec![];
 for l in raw.lines().skip(1){let v:Vec<_>=l.split('\t').collect();if v[0].parse::<u64>().unwrap()!=p||v[1].parse::<u64>().unwrap()!=sm{continue}let mut row=vec![0;cols+1];row[cols]=1;for s in v[5].split(','){let a:Vec<u64>=s.split(':').map(|x|x.parse().unwrap()).collect();let j=fb.binary_search_by_key(&a[1],|p|p.0).unwrap();assert_eq!(map[j].0,a[0]as usize);let w=if a[2]==fb[j].1{map[j].1}else{assert_eq!(a[2],c.fp.neg(fb[j].1));f.neg(map[j].1)};row[map[j].0]=f.add(row[map[j].0],w);}input.push((row,f.neg(v[3].parse::<u64>().unwrap())));}
 for seed in 1..cols {let ps=fb[reps[seed]];let seedpoint=c.place(ps.0,ps.1);let mut constraints=sparse::Sparse::new(2,ell);let mut labels=vec![None;cols];labels[0]=Some([0,0,f.inv(sm)]);labels[seed]=Some([0,1,0]);let mut src=vec![0;cols+1];src[0]=1;
 let mut solver=sparse::Sparse::new(cols+1,ell);solver.insert(&src,f.inv(sm));let to_row=|v:&[u64]|->Row{v.iter().enumerate().filter_map(|(i,&x)|(x!=0).then_some((i as u32,x))).collect()};let mut rows=vec![(to_row(&src),f.inv(sm))];let mut used=vec![false;input.len()];let(mut passes,mut admitted,mut first)=(0,0,None);
 loop{passes+=1;let mut grew=false;for(index,(row,rhs))in input.iter().enumerate(){if used[index]{continue}let mut unknown=vec![];let(mut alpha,mut beta,mut gamma)=(row[cols],0,f.neg(*rhs));for j in 0..cols{if row[j]==0{continue}if let Some(h)=labels[j]{alpha=f.add(alpha,f.mul(row[j],h[0]));beta=f.add(beta,f.mul(row[j],h[1]));gamma=f.add(gamma,f.mul(row[j],h[2]));}else{unknown.push(j);}}if unknown.len()>1{continue}
 used[index]=true;admitted+=1;let action=if let Some(&j)=unknown.first(){let inv=f.inv(row[j]);let h=[f.neg(f.mul(alpha,inv)),f.neg(f.mul(beta,inv)),f.neg(f.mul(gamma,inv))];let pj=fb[reps[j]];assert_eq!(c.place(pj.0,pj.1),c.add(&c.add(&c.mul(&t,h[0]),&c.mul(&seedpoint,h[1])),&c.mul(&d,h[2])));labels[j]=Some(h);grew=true;"label"}else{constraints.insert(&[beta,alpha],f.neg(gamma));if let Some(k)=constraints.k(){assert_eq!(c.mul(&d,k),t);if first.is_none(){first=Some((passes,index+1,k));}}"constraint"};
 solver.insert(row,*rhs);rows.push((to_row(row),*rhs));let k=constraints.k();assert_eq!(solver.k(),k);assert_eq!(first.map(|x|x.2),k);if let Some(k)=k{assert_eq!(k,secret);assert_eq!(c.mul(&d,k),t);}checks+=1;
 let _=action;
 }if !grew{break}assert!(passes<=cols+1);}
 let mut st=SolveStats::default();let terminal=solve_for_last(&rows,cols,ell,Solver::Fast,&[],&mut st);assert_eq!(terminal,constraints.k());println!("{p}\t{sm}\t{seed}\t{passes}\t{}\t{admitted}\t{}\t{}",labels.iter().filter(|x|x.is_some()).count(),constraints.rank,terminal.map_or("none".into(),|k|k.to_string()));
 }
 }}eprintln!("prefix_checks={checks} all_symbolic_seed_labels_original_group_and_constraint_sparse_prefix_checks=true terminal_nativeFast_checks=67 free_support_oracle=true");}
