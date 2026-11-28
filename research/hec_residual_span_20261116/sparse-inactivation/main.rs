#[path="../setup/fast.rs"]mod fast;
#[path="../multi/algebra.rs"]mod algebra;
#[path="../batched/inverse.rs"]mod inverse;
#[path="../orbit-core/sparse.rs"]mod sparse;
use hec_native_ic::{counted,fp::Fp,jac::{Curve,Div},poly::Poly,rng::Rng,solve::{Row,Solver,SolveStats,solve_for_last}};
use std::collections::{BTreeMap,BTreeSet};
fn main(){counted::disable();println!("p\tsm\tpasses\tanchor_scans\tpoint_visits\tsingular\tbuckets\trows\tlabels\tk");
for(p,cc,ell)in[(101,2,7481),(251,18,67931),(601,7,411491)]{
 let c=Curve::new(Fp::new(p),Poly::from_coeffs(&[cc,0,0,0,0,1]));let(fb,z,a)=fast::prepare(&c);let(_,_,degree,lambda)=algebra::certificate(&a,ell);assert_eq!(degree,1);let lambda=lambda.unwrap();let f=Fp::new(ell);
 let mut map=vec![(usize::MAX,0);fb.len()];let mut reps=vec![];let mut members=vec![];
 for j in 0..fb.len(){if map[j].0!=usize::MAX{continue}let r=reps.len();reps.push(j);let mut orbit=vec![];let(mut x,y,mut w)=(fb[j].0,fb[j].1,1);for _ in 0..5{let q=fb.binary_search_by_key(&x,|p|p.0).unwrap();map[q]=(r,if fb[q].1==y{w}else{assert_eq!(fb[q].1,c.fp.neg(y));f.neg(w)});orbit.push(q);x=c.fp.mul(x,z);w=f.mul(w,lambda);}members.push(orbit);}
 let cols=reps.len();let base=c.place(fb[0].0,fb[0].1);let mut rng=Rng::new(2026112500^p);
 for sm in [1,17]{let d=c.mul(&base,sm);let secret=1+rng.below(ell-1);let target=c.mul(&d,secret);let seed=1;let ps=fb[reps[seed]];let seedpoint=c.place(ps.0,ps.1);
 let mut labels=vec![None;cols];labels[0]=Some([0,0,f.inv(sm)]);labels[seed]=Some([0,1,0]);let mut pool=vec![0,seed];let mut constraints=sparse::Sparse::new(2,ell);let mut reference=sparse::Sparse::new(cols+1,ell);let mut src=vec![0;cols+1];src[0]=1;reference.insert(&src,f.inv(sm));let pack=|v:&[u64]|->Row{v.iter().enumerate().filter_map(|(i,&x)|(x!=0).then_some((i as u32,x))).collect()};let mut rows=vec![(pack(&src),f.inv(sm))];
 let mut anchors=Vec::<Div>::new();let mut anchor=target;for _ in 1..=16{anchor=c.add(&anchor,&d);anchors.push(anchor);}let mut seen=BTreeSet::new();let(mut passes,mut scans,mut visits,mut singular,mut buckets,mut admitted)=(0,0,0,0,0,0);
 'closure: loop{passes+=1;let before=pool.len();for(ai,r)in anchors.iter().enumerate(){scans+=1;if r.u.deg()!=2{singular+=1;continue}let mut entries=vec![];let mut den=vec![];for &orb in &pool{for &j in &members[orb]{visits+=1;let x=fb[j].0;let u=r.u.eval(x,&c.fp);if u==0{singular+=1;continue}entries.push(j);den.push(u);}}
 let inv=inverse::invert(&den,&c.fp);let mut bs=BTreeMap::<u64,Vec<(usize,u64)>>::new();for(&j,&iv)in entries.iter().zip(&inv){let(x,y)=fb[j];let v=r.v.eval(x,&c.fp);for (sign,yy) in [y,c.fp.neg(y)].into_iter().enumerate(){if sign==1&&y==0{continue}let t=c.fp.mul(c.fp.sub(yy,v),iv);bs.entry(t).or_default().push((j,yy));}}
 for(t,bucket)in bs{if bucket.len()<2{continue}buckets+=1;if seen.contains(&(ai,t)){continue}let(j0,y0)=bucket[0];let Some(&(j1,y1))=bucket.iter().find(|(j,_)|fb[*j].0!=fb[j0].0)else{continue};let x0=fb[j0].0;let x1=fb[j1].0;let w=r.v.add(&r.u.scale(t,&c.fp),&c.fp);let h=c.f.sub(&w.mul(&w,&c.fp),&c.fp).div_exact(&r.u,&c.fp);assert_eq!(h.deg(),3);assert_eq!(h.lc(),1);let pair=Poly::x_minus(x0,&c.fp).mul(&Poly::x_minus(x1,&c.fp),&c.fp);let q=h.div_exact(&pair,&c.fp);assert_eq!(q.deg(),1);assert_eq!(q.lc(),1);let x2=c.fp.neg(q.c[0]);let y2=w.eval(x2,&c.fp);let j2=fb.binary_search_by_key(&x2,|p|p.0).unwrap();seen.insert((ai,t));
 let mut row=vec![0;cols+1];row[cols]=1;let mut group=*r;for(j,yy)in[(j0,y0),(j1,y1),(j2,y2)]{let(x,canonical)=fb[j];assert_eq!(w.eval(x,&c.fp),yy);let(orb,weight)=map[j];let weight=if yy==canonical{weight}else{assert_eq!(yy,c.fp.neg(canonical));f.neg(weight)};row[orb]=f.add(row[orb],weight);group=c.add_place(&group,x,yy);}assert!(c.is_zero(&group));
 let mut known=[1,0,(ai+1)as u64];let mut unknown=vec![];for j in 0..cols{if row[j]==0{continue}if let Some(label)=labels[j]{for k in 0..3{known[k]=f.add(known[k],f.mul(row[j],label[k]));}}else{unknown.push(j);}}assert!(unknown.len()<=1);
 if let Some(&j)=unknown.first(){let iv=f.inv(row[j]);let mut label=[0;3];for k in 0..3{label[k]=f.neg(f.mul(known[k],iv));}let pt=fb[reps[j]];assert_eq!(c.place(pt.0,pt.1),c.add(&c.add(&c.mul(&target,label[0]),&c.mul(&seedpoint,label[1])),&c.mul(&d,label[2])));labels[j]=Some(label);pool.push(j);}else{constraints.insert(&[known[1],known[0]],f.neg(known[2]));}
 let rhs=f.neg((ai+1)as u64);reference.insert(&row,rhs);rows.push((pack(&row),rhs));admitted+=1;let k=constraints.k();assert_eq!(reference.k(),k);if let Some(k)=k{assert_eq!(k,secret);assert_eq!(c.mul(&d,k),target);break 'closure;}
 }
 }if pool.len()==before{break}assert!(passes<=cols+1);}
 let mut st=SolveStats::default();assert_eq!(solve_for_last(&rows,cols,ell,Solver::Fast,&[],&mut st),constraints.k());println!("{p}\t{sm}\t{passes}\t{scans}\t{visits}\t{singular}\t{buckets}\t{admitted}\t{}\t{}",pool.len(),constraints.k().map_or("none".into(),|k|k.to_string()));
 }}
}
