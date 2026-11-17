#![allow(dead_code)]
use hec_native_ic::{counted,fp::Fp,jac::{Curve,Div},poly::Poly,instance::factor_base};
use std::collections::{BTreeMap,HashSet};
#[path="../generic-kummer/terms.rs"]mod terms;
type P=BTreeMap<[u8;4],u64>;
const Q:u64=601;
fn add(a:u64,b:u64)->u64{(a+b)%Q}fn sub(a:u64,b:u64)->u64{(a+Q-b)%Q}fn mul(a:u64,b:u64)->u64{a*b%Q}
fn pow(mut a:u64,mut n:u64)->u64{let mut b=1;while n>0{if n&1==1{b=mul(a,b)}a=mul(a,a);n>>=1;}b}
fn term(p:&mut P,e:[u8;4],c:u64){let x=p.entry(e).or_default();*x=add(*x,c);}
fn product(a:&P,b:&P)->P{let mut c=P::new();for(e,x)in a{for(f,y)in b{let mut g=[0;4];for j in 0..4{g[j]=e[j]+f[j];}term(&mut c,g,mul(*x,*y));}}c}
fn monomials(d:u8)->Vec<[u8;4]>{let mut v=vec![];for a in 0..=d{for b in 0..=d-a{for c in 0..=d-a-b{v.push([a,b,c,d-a-b-c]);}}}v}
fn vector(p:&P,mons:&[[u8;4]])->Vec<u64>{mons.iter().map(|e|*p.get(e).unwrap_or(&0)).collect()}
fn rref(mut a:Vec<Vec<u64>>)->(Vec<Vec<u64>>,Vec<usize>){let cols=a[0].len();let mut r=0;let mut piv=vec![];for j in 0..cols{let Some(k)=(r..a.len()).find(|&i|a[i][j]!=0)else{continue};a.swap(k,r);let iv=pow(a[r][j],Q-2);for t in 0..cols{a[r][t]=mul(a[r][t],iv)}for i in 0..a.len(){if i==r{continue}let x=a[i][j];for t in 0..cols{a[i][t]=sub(a[i][t],mul(x,a[r][t]));}}piv.push(j);r+=1;if r==a.len(){break}}a.truncate(r);(a,piv)}
fn nullspace(a:Vec<Vec<u64>>)->Vec<Vec<u64>>{let n=a[0].len();let(a,p)=rref(a);let mut out=vec![];for j in 0..n{if p.contains(&j){continue}let mut x=vec![0;n];x[j]=1;for(i,&k)in p.iter().enumerate(){x[k]=sub(0,a[i][j]);}out.push(x);}out}
fn reduce(mut v:Vec<u64>,relations:&[Vec<u64>])->Vec<u64>{for r in relations{let j=r.iter().position(|&x|x!=0).unwrap();let x=v[j];for k in 0..v.len(){v[k]=sub(v[k],mul(x,r[k]));}}v}
fn project(c:&Curve,d:&Div)->[u64;4]{match d.u.deg(){0=>[0,0,0,1],1=>{let x=c.fp.neg(d.u.c[0]);[0,1,x,mul(x,x)]},2=>{let a=d.u.c[1];let b=d.u.c[0];[1,sub(0,a),b,add(mul(mul(d.v.c[1],d.v.c[1]),pow(7,Q-2)),mul(a,sub(mul(a,a),b)))]},_=>panic!()}}
fn biquadratic(k:[u64;4])->Vec<P>{terms::FORMS.iter().map(|ts|{let mut p=P::new();for &(v,n,e,f)in *ts{let mut c=mul(v.rem_euclid(Q as i64)as u64,pow(7,n as u64));for j in 0..4{c=mul(c,pow(k[j],f[j]as u64));}term(&mut p,e,c);}p}).collect()}
fn permutations(v:&mut[usize;5],i:usize,out:&mut Vec<[usize;5]>){if i==5{out.push(*v);return}for j in i..5{v.swap(i,j);permutations(v,i+1,out);v.swap(i,j);}}
fn space(c:&Curve,d:&Div,mons:&[[u8;4]],relations:&[Vec<u64>])->Vec<Vec<u64>>{let b=biquadratic(project(c,d));let b2=biquadratic(project(c,&c.mul(d,2)));let mut all=vec![];
 for e in mons{let mut indices=[0usize;5];let mut n=0;for i in 0..4{for _ in 0..e[i]{indices[n]=i;n+=1;}}let mut perms=vec![];permutations(&mut indices,0,&mut perms);let mut p=P::new();for v in perms{let mut unit=[0;4];unit[v[0]]=1;let x=P::from([(unit,1)]);let t=product(&product(&x,&b[4*v[1]+v[2]]),&b2[4*v[3]+v[4]]);for(e,c)in t{term(&mut p,e,c)}}all.push(reduce(vector(&p,mons),relations));}
 rref(all).0
}
fn evaluate(rows:&[Vec<u64>],mons:&[[u8;4]],k:[u64;4])->Vec<u64>{let vs:Vec<_>=mons.iter().map(|e|(0..4).fold(1,|s,j|mul(s,pow(k[j],e[j]as u64)))).collect();rows.iter().map(|r|r.iter().zip(&vs).fold(0,|s,(&x,&y)|add(s,mul(x,y)))).collect()}
fn same(a:&[u64],b:&[u64])->bool{let Some(j)=a.iter().position(|&x|x!=0)else{return false};b[j]!=0&&(0..a.len()).all(|i|mul(a[i],b[j])==mul(b[i],a[j]))}
fn prior_qualification(){counted::disable();let c=Curve::new(Fp::new(Q),Poly::from_coeffs(&[49,0,0,0,0,7]));let s=Div{u:Poly::from_coeffs(&[329,1]),v:Poly::from_coeffs(&[108])};let r=Div{u:Poly::from_coeffs(&[0,1]),v:Poly::from_coeffs(&[594])};for d in [&s,&r]{assert!(c.is_valid(d));assert!(c.is_zero(&c.mul(d,5)));}let h:HashSet<_>=(0..5).flat_map(|a|(0..5).map(move|b|(a,b))).map(|(a,b)|c.add(&c.mul(&r,a),&c.mul(&s,b))).collect();assert_eq!(h.len(),25);
 let mons=monomials(5);assert_eq!(mons.len(),56);
 // Source monic binomial Kummer quartic, detwisted projection.
 let quartic=P::from([([0,0,4,0],1),([0,2,0,2],1),([0,1,2,1],Q-2),([1,0,1,2],Q-4),([3,0,0,1],Q-28),([1,3,0,0],Q-28),([2,1,1,0],28)]);
 let rels:Vec<_>=(0..4).map(|j|{let mut e=[0;4];e[j]=1;vector(&product(&quartic,&P::from([(e,1)])),&mons)}).collect();let(relations,_)=rref(rels);assert_eq!(relations.len(),4);
 let a=space(&c,&r,&mons,&relations);let b=space(&c,&s,&mons,&relations);println!("space_R={} space_S={} quartic_relations=4",a.len(),b.len());assert_eq!(a.len(),12);assert_eq!(b.len(),12);
 let mut ab=a.clone();ab.extend(b.clone());let combined=rref(ab).0.len();assert_eq!(combined,20);let eqs=(0..56).map(|j|a.iter().map(|v|v[j]).chain(b.iter().map(|v|sub(0,v[j]))).collect()).collect();let ns=nullspace(eqs);assert_eq!(ns.len(),4);let map:Vec<Vec<u64>>=ns.iter().map(|x|(0..56).map(|j|(0..12).fold(0,|v,i|add(v,mul(x[i],a[i][j])))).collect()).collect();assert_eq!(rref(map.clone()).0.len(),4);
 for(i,row)in map.iter().enumerate(){println!("MAP\t{i}\t{}",row.iter().map(u64::to_string).collect::<Vec<_>>().join(","));}
 let fb=factor_base(&c);let mut samples=vec![c.zero(),r,s];for &(x,y)in fb.iter().take(32){let p=c.place(x,y);samples.push(p);samples.push(c.add(&p,&s));}let mut checks=0;for p in &samples{let k=project(&c,p);let qv=quartic.iter().fold(0,|v,(e,&a)|add(v,mul(a,(0..4).fold(1,|t,j|mul(t,pow(k[j],e[j]as u64))))));assert_eq!(qv,0);let v=evaluate(&map,&mons,k);assert!(v.iter().any(|&x|x!=0));for t in [r,s]{let p2=c.add(p,&t);assert!(same(&v,&evaluate(&map,&mons,project(&c,&p2))));checks+=1;}}
 let mut pairs=0;for i in 0..samples.len(){for j in 0..i{let x=evaluate(&map,&mons,project(&c,&samples[i]));let y=evaluate(&map,&mons,project(&c,&samples[j]));let expected=h.contains(&c.add(&samples[i],&c.neg(&samples[j])))||h.contains(&c.add(&samples[i],&samples[j]));assert_eq!(same(&x,&y),expected);pairs+=1;}}
 println!("intersection_dimension=4 samples={} translation_checks={checks} separation_pairs={pairs} independent_verification=false",samples.len());
}
fn normal(v:Vec<u64>)->Vec<u64>{let j=v.iter().position(|&x|x!=0).unwrap();let iv=pow(v[j],Q-2);v.into_iter().map(|x|mul(x,iv)).collect()}
fn polyreduce(p:P,quartic:&P)->P{let mut p=p;let mut out=P::new();while !p.is_empty(){let e=*p.keys().max_by_key(|e|(e[2],e[0],e[1],e[3])).unwrap();let c=p.remove(&e).unwrap();if c==0{continue}if e[2]<4{term(&mut out,e,c);continue}let mut lift=e;lift[2]-=4;for(f,&v)in quartic{if *f==[0,0,4,0]{continue}let mut g=[0;4];for j in 0..4{g[j]=lift[j]+f[j];}term(&mut p,g,sub(0,mul(c,v)));}}out.retain(|_,v|*v!=0);out}
fn main(){counted::disable();let c=Curve::new(Fp::new(Q),Poly::from_coeffs(&[49,0,0,0,0,7]));let mons=monomials(5);let rows:Vec<Vec<u64>>=include_str!("../quintic-map/CONTROL.txt").lines().filter(|l|l.starts_with("MAP\t")).map(|l|l.split('\t').nth(2).unwrap().split(',').map(|x|x.parse().unwrap()).collect()).collect();assert_eq!(rows.len(),4);for r in &rows{assert_eq!(r.len(),56);}
 let qm=monomials(4);assert_eq!(qm.len(),35);let fb=factor_base(&c);let mut seen=std::collections::BTreeSet::new();let mut evals=vec![];for &(x,y)in fb.iter().take(128){for &(x2,y2)in fb.iter().take(4){let d=c.add(&c.place(x,y),&c.place(x2,y2));let v=normal(evaluate(&rows,&mons,project(&c,&d)));if seen.insert(v.clone()){let v=[v[0],v[1],v[2],v[3]];evals.push(qm.iter().map(|e|(0..4).fold(1,|s,j|mul(s,pow(v[j],e[j]as u64)))).collect());}}}
 let samples=evals.len();assert!(samples>=70);let ns=nullspace(evals);println!("distinct_image_samples={samples} quartic_nullity={}",ns.len());assert_eq!(ns.len(),1);let coeff=normal(ns[0].clone());let target:P=qm.iter().copied().zip(coeff.iter().copied()).filter(|(_,v)|*v!=0).collect();
 println!("TARGET_QUARTIC\t{}",coeff.iter().map(u64::to_string).collect::<Vec<_>>().join(","));
 let quartic=P::from([([0,0,4,0],1),([0,2,0,2],1),([0,1,2,1],Q-2),([1,0,1,2],Q-4),([3,0,0,1],Q-28),([1,3,0,0],Q-28),([2,1,1,0],28)]);
 let ps:Vec<P>=rows.iter().map(|r|mons.iter().copied().zip(r.iter().copied()).filter(|(_,v)|*v!=0).collect()).collect();let powers:Vec<Vec<P>>=ps.iter().map(|p|{let mut v=vec![P::from([([0;4],1)])];for j in 1..=4{v.push(product(&v[j-1],p));}v}).collect();let mut composed=P::new();for(e,&a)in &target{let mut p=P::from([([0;4],a)]);for j in 0..4{p=product(&p,&powers[j][e[j]as usize]);}for(f,b)in p{term(&mut composed,f,b);}}
 let terms=composed.values().filter(|&&v|v!=0).count();let remainder=polyreduce(composed,&quartic);assert!(remainder.is_empty());
 let origin=normal(evaluate(&rows,&mons,[0,0,0,1]));let eval_target=|v:&[u64]|target.iter().fold(0,|s,(e,&a)|add(s,mul(a,(0..4).fold(1,|b,j|mul(b,pow(v[j],e[j]as u64))))));assert_eq!(eval_target(&origin),0);for j in 0..4{let derivative=target.iter().filter(|(e,_)|e[j]>0).fold(0,|s,(e,&a)|{let mut f=*e;f[j]-=1;add(s,mul(mul(a,e[j]as u64),(0..4).fold(1,|b,k|mul(b,pow(origin[k],f[k]as u64)))))});assert_eq!(derivative,0);}
 println!("composition_degree=20 composition_nonzero_terms_before_reduction={terms} exact_remainder_terms=0 origin_image={:?} origin_singular=true independent_verification=false",origin);
}
