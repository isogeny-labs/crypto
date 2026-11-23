use std::{env,fs,collections::{BTreeMap,BTreeSet}};
#[derive(Clone,Copy,PartialEq,Eq,Debug)]struct F(i32,i32);
fn f(a:i32,b:i32)->F{F(a.rem_euclid(7),b.rem_euclid(7))}
fn from(n:i32)->F{f(n%7,n/7)}fn enc(x:F)->i32{x.0+7*x.1}
fn a(x:F,y:F)->F{f(x.0+y.0,x.1+y.1)}fn n(x:F)->F{f(-x.0,-x.1)}fn m(x:F,y:F)->F{let c=[x.0*y.0,x.0*y.1+x.1*y.0,x.1*y.1];f(c[0]+3*c[2],c[1])}
fn pw(mut x:F,mut k:i32)->F{let mut r=f(1,0);while k>0{if k%2==1{r=m(r,x)}x=m(x,x);k/=2}r}
fn iv(x:F)->F{assert!(x!=f(0,0));pw(x,47)}fn prime(k:i32)->bool{k>=2&&(2..k).all(|d|k%d!=0)}
fn count(aa:F,bb:F)->i32{let mut out=1;for x in 0..49{let z=from(x);let rhs=a(a(m(m(z,z),z),m(aa,z)),bb);for y in 0..49{if m(from(y),from(y))==rhs{out+=1}}}out}
#[derive(Clone,Copy,PartialEq,Eq,Debug)]struct K(F,F);
fn ka(x:K,y:K)->K{K(a(x.0,y.0),a(x.1,y.1))}fn kn(x:K)->K{K(n(x.0),n(x.1))}
fn km(x:K,y:K,d:F)->K{K(a(m(x.0,y.0),m(d,m(x.1,y.1))),a(m(x.0,y.1),m(x.1,y.0)))}
fn ki(x:K,d:F)->K{let den=iv(a(m(x.0,x.0),n(m(d,m(x.1,x.1)))));K(m(x.0,den),n(m(x.1,den)))}fn ci(x:i32)->K{K(f(x,0),f(0,0))}
#[derive(Clone,Copy,PartialEq,Eq,Debug)]enum P{O,V(K,K)}
fn add(p:P,q:P,aa:F,d:F)->P{match(p,q){(P::O,_)=>q,(_,P::O)=>p,(P::V(x,y),P::V(u,v))=>{if x==u&&ka(y,v)==ci(0){return P::O}let l=if x==u{km(ka(km(ci(3),km(x,x,d),d),K(aa,f(0,0))),ki(km(ci(2),y,d),d),d)}else{km(ka(v,kn(y)),ki(ka(u,kn(x)),d),d)};let z=ka(ka(km(l,l,d),kn(x)),kn(u));P::V(z,ka(km(l,ka(x,kn(z)),d),kn(y)))}}}
fn mul(mut p:P,mut k:i32,aa:F,d:F)->P{let mut o=P::O;while k>0{if k%2==1{o=add(o,p,aa,d)}p=add(p,p,aa,d);k/=2}o}
fn conj(p:P)->P{match p{P::O=>p,P::V(x,y)=>P::V(K(x.0,n(x.1)),K(y.0,n(y.1)))}}
fn on(p:P,aa:F,bb:F,d:F)->bool{match p{P::O=>true,P::V(x,y)=>km(y,y,d)==ka(ka(km(km(x,x,d),x,d),km(K(aa,f(0,0)),x,d)),K(bb,f(0,0)))}}
type Poly=BTreeMap<u32,F>;
fn ex(k:u32,i:u32)->i32{((k>>(6*i))&63) as i32}
fn deg(k:u32,w:bool)->i32{(0..5).map(|i|ex(k,i)*if w&&(i==2||i==4){2}else{1}).sum()}
fn pd(p:&Poly,w:bool)->i32{p.keys().map(|&k|deg(k,w)).max().unwrap_or(-1)}
fn c(v:F)->Poly{if v==f(0,0){Poly::new()}else{[(0,v)].iter().copied().collect()}}
fn var(i:u32)->Poly{[(1<<(6*i),f(1,0))].iter().copied().collect()}
fn put(p:&mut Poly,k:u32,v:F){let z=a(*p.get(&k).unwrap_or(&f(0,0)),v);if z==f(0,0){p.remove(&k);}else{p.insert(k,z);}}
fn pa(p:&Poly,q:&Poly)->Poly{let mut r=p.clone();for (&k,&v) in q{put(&mut r,k,v)}r}
fn ps(p:&Poly,s:F)->Poly{p.iter().filter_map(|(&k,&v)|{let z=m(v,s);if z==f(0,0){None}else{Some((k,z))}}).collect()}
fn pm(p:&Poly,q:&Poly)->Poly{let mut r=Poly::new();for(&k,&v)in p{for(&j,&u)in q{assert!((0..5).all(|i|ex(k,i)+ex(j,i)<64));put(&mut r,k+j,m(v,u));}}r}
fn pp(p:&Poly,k:i32)->Poly{let mut r=c(f(1,0));for _ in 0..k{r=pm(&r,p)}r}
fn sub(p:&Poly,q:&Poly)->Poly{pa(p,&ps(q,f(-1,0)))}
fn lead(p:&Poly,w:bool)->(u32,F){p.iter().map(|(&k,&v)|(k,v)).max_by_key(|(k,_)|(deg(*k,w),*k)).unwrap()}
fn divs(u:u32,v:u32)->bool{(0..5).all(|i|ex(u,i)<=ex(v,i))}
fn shift(p:&Poly,k:u32,v:F)->Poly{p.iter().filter_map(|(&u,&s)|{let z=m(s,v);if z==f(0,0){None}else{Some((u+k,z))}}).collect()}
fn nf(p:&Poly,b:&[Poly],w:bool)->Poly{let mut r=p.clone();let mut out=Poly::new();while !r.is_empty(){let(k,v)=lead(&r,w);let mut reduced=false;for q in b{let(j,s)=lead(q,w);if divs(j,k){r=sub(&r,&shift(q,k-j,m(v,iv(s))));reduced=true;break;}}if !reduced{r.remove(&k);out.insert(k,v);}}out}
fn lc(u:u32,v:u32)->u32{(0..5).map(|i|(ex(u,i).max(ex(v,i)) as u32)<<(6*i)).sum()}
fn poly(s:&str)->Poly{if s=="-"{return Poly::new()}let mut p=Poly::new();for term in s.split(','){let(k,v)=term.split_once(':').unwrap();let k=k.parse::<u32>().unwrap();let v=v.parse::<i32>().unwrap();assert!((1..49).contains(&v));assert!(p.insert(k,from(v)).is_none());}p}
fn membership(roots:&BTreeSet<i32>,x:&Poly)->Poly{let mut p=c(f(1,0));for &v in roots{p=pm(&p,&sub(x,&c(from(v))));}p}
fn system(base:&[BTreeSet<i32>],gamma:i32,beta:i32,left:i32,right:i32,q:P)->Vec<Poly>{let (s,t)=match q{P::V(x,y)=>(x.0,y.0),_=>panic!()};let u=var(0);let v=var(1);let a0=var(2);let z=var(3);let a1=var(4);let aa=from(1);let bb=from(8);
 if left<0{let xy=pm(&u,&v);let sum=pa(&u,&v);let dif=sub(&u,&v);let mut p=ps(&pp(&dif,2),m(s,s));p=sub(&p,&ps(&pa(&pm(&sum,&pa(&xy,&c(aa))),&c(m(f(2,0),bb))),m(f(2,0),s)));p=pa(&p,&pp(&sub(&xy,&c(aa)),2));p=sub(&p,&ps(&sum,m(f(4,0),bb)));let roots: BTreeSet<i32>=base.iter().flat_map(|x|x.iter().copied()).collect();return vec![p,membership(&roots,&u),membership(&roots,&v)]}
 let mut inputs=vec![];let ls=[v,z];let av=[a0,a1];let bs=[left,right];let mut xs=vec![];
 for i in 0..2{let x=if bs[i]==0{ls[i].clone()}else{sub(&pp(&ls[i],2),&ps(&pa(&av[i],&c(from(gamma))),f(2,0)))};let y=sub(&pm(&u,&sub(&x,&c(s))),&c(t));let curve=sub(&sub(&sub(&pp(&y,2),&pp(&x,3)),&ps(&x,aa)),&c(bb));inputs.push(curve);
  if bs[i]==0{inputs.push(membership(&base[0],&x));inputs.push(av[i].clone());}else{let zz=m(from(beta),f(bs[i],0));let dd=m(from(8),m(zz,zz));let mut p=pp(&ls[i],4);p=sub(&p,&ps(&pm(&x,&pp(&ls[i],2)),f(6,0)));p=sub(&p,&ps(&pp(&x,2),f(3,0)));p=sub(&p,&ps(&pm(&y,&ls[i]),f(8,0)));p=sub(&p,&c(m(f(4,0),a(aa,dd))));inputs.push(p);inputs.push(sub(&pp(&av[i],7),&av[i]));}xs.push(x);
 }
 inputs.push(sub(&pa(&xs[0],&xs[1]),&sub(&pp(&u,2),&c(s))));let prod=pa(&pa(&ps(&pp(&u,2),s),&ps(&u,m(f(2,0),t))),&c(a(m(s,s),aa)));inputs.push(sub(&pm(&xs[0],&xs[1]),&prod));inputs
}
fn run(){let args:Vec<_>=env::args().collect();assert_eq!(args.len(),3);let text=fs::read_to_string(&args[1]).unwrap();let mut lines=text.lines();assert_eq!(lines.next(),Some("TD1 7 3 8 1 8 47 153"));let aa=from(1);let bb=from(8);let d=from(8);let zero=f(0,0);
 let mut targets=vec![P::O];for x in 0..49{for y in 0..49{let p=P::V(K(from(x),zero),K(from(y),zero));if on(p,aa,bb,d){targets.push(p)}}}assert_eq!(targets.len(),47);let target=targets[1];let gammas=[7,28,35];let betas=[10,11,8];let mut bases=vec![vec![BTreeSet::new();4];3];
 // Independent complete point reference creates only PUBLIC branch membership.
 let mut squares=BTreeMap::<i32,Vec<K>>::new();for y0 in 0..49{for y1 in 0..49{let y=K(from(y0),from(y1));let z=km(y,y,d);squares.entry(enc(z.0)*49+enc(z.1)).or_default().push(y);}}
 for x0 in 0..49{for x1 in 0..49{let x=K(from(x0),from(x1));let rhs=ka(ka(km(km(x,x,d),x,d),km(K(aa,zero),x,d)),K(bb,zero));for &y in squares.get(&(enc(rhs.0)*49+enc(rhs.1))).unwrap_or(&vec![]){let p=P::V(x,y);let q=add(p,conj(p),aa,d);if let P::V(qx,qy)=q{assert_eq!(qx.1,zero);assert_eq!(qy.1,zero);for g in 0..3{let b=m(x.1,iv(from(betas[g])));if a(x.0,n(from(gammas[g]))).1==0&&b.1==0{let br=b.0.min((-b.0).rem_euclid(7)) as usize;bases[g][br].insert(enc(qx.0));}}}}}}
 for g in 0..3{for k in 0..4{let row:Vec<_>=lines.next().unwrap().split_whitespace().collect();assert_eq!(&row[..3],&["BASE",&g.to_string(),&k.to_string()]);let got:BTreeSet<i32>=row[3].split(',').map(|s|s.parse().unwrap()).collect();assert_eq!(got,bases[g][k]);}}
 let tq:Vec<_>=lines.next().unwrap().split_whitespace().collect();if let P::V(x,y)=target{assert_eq!(tq,vec!["TARGET".to_string(),enc(x.0).to_string(),enc(y.0).to_string()]);}
 let mut plans=vec![(-1,-1,-1,2)];for g in 0..3{for cap in [14,16,20]{plans.push((g,-1,-1,cap));for l in 0..4{for r in 0..4{plans.push((g,l,r,cap));}}}}
 let mut observations=vec![];let mut complete=0;let mut unknown=0;let mut useful=0;
 for (serial,&(g,l,r,cap))in plans.iter().enumerate(){let row:Vec<_>=lines.next().unwrap().split_whitespace().collect();assert_eq!(row.len(),12);assert_eq!(row[0],"CASE");let nums:Vec<i32>=row[1..6].iter().map(|x|x.parse().unwrap()).collect();assert_eq!(nums,vec![serial as i32,g,l,r,cap]);let status=row[6];assert!(["complete","cap","resource"].contains(&status));let od=row[7].parse::<i32>().unwrap();let wd=row[8].parse::<i32>().unwrap();let _work=row[9].parse::<u64>().unwrap();let ni=row[10].parse::<usize>().unwrap();let nb=row[11].parse::<usize>().unwrap();let control=serial==0;let weighted=l>=0&&!control;
  let want=if control{vec![sub(&var(0),&c(f(1,0))),sub(&var(1),&c(f(2,0)))]}else{system(&bases[g as usize],gammas[g as usize],betas[g as usize],l,r,target)};assert_eq!(ni,want.len());let mut ins=vec![];for p in want{let row:Vec<_>=lines.next().unwrap().split_whitespace().collect();assert_eq!(row.len(),2);assert_eq!(row[0],"I");let got=poly(row[1]);assert_eq!(got,p);ins.push(p);}
  let mut basis=vec![];for _ in 0..nb{let row:Vec<_>=lines.next().unwrap().split_whitespace().collect();assert_eq!(row.len(),ni+2);assert_eq!(row[0],"B");let p=poly(row[1]);assert!(!p.is_empty());assert_eq!(lead(&p,weighted).1,f(1,0));assert!(pd(&p,false)<=od&&pd(&p,weighted)<=cap);let mut exact=Poly::new();for i in 0..ni{let w=poly(row[i+2]);if !w.is_empty(){assert!(pd(&w,weighted)+pd(&ins[i],weighted)<=wd&&wd<=cap);}exact=pa(&exact,&pm(&w,&ins[i]));}assert_eq!(exact,p);basis.push(p);}
  if status=="complete"{assert!(wd<=cap&&od<=cap);assert!(!basis.is_empty());for p in &ins{assert!(nf(p,&basis,weighted).is_empty());}for i in 0..basis.len(){for j in 0..i{let(u,c0)=lead(&basis[i],weighted);let(v,c1)=lead(&basis[j],weighted);let k=lc(u,v);let s=sub(&shift(&basis[i],k-u,iv(c0)),&shift(&basis[j],k-v,iv(c1)));assert!(nf(&s,&basis,weighted).is_empty());}}if !control{complete+=1}}else{assert_eq!(nb,0);unknown+=1;}
  let rr:Vec<_>=lines.next().unwrap().split_whitespace().collect();assert_eq!(rr.len(),3);assert_eq!(rr[0],"ROOTS");let nr=rr[1].parse::<usize>().unwrap();let attempts=rr[2].parse::<usize>().unwrap();let mut expected=vec![];let mut checks=0;
  if status=="complete"&&!control{let contains=|branch:i32,x:i32|if branch<0{bases[g as usize].iter().any(|b|b.contains(&x))}else{bases[g as usize][branch as usize].contains(&x)};for &p in &targets{for &q in &targets{if let(P::V(x,y),P::V(u,v))=(p,q){if !contains(l,enc(x.0))||!contains(r,enc(u.0)){continue}checks+=1;if add(p,q,aa,d)==target{expected.push([enc(x.0),enc(y.0),enc(u.0),enc(v.0)]);}}}}}
  assert_eq!(nr,expected.len());assert_eq!(attempts,checks);for p in &expected{let row:Vec<_>=lines.next().unwrap().split_whitespace().collect();assert_eq!(row[0],"R");let got:Vec<i32>=row[1..].iter().map(|x|x.parse().unwrap()).collect();assert_eq!(&got,p);}assert_eq!(lines.next(),Some("END"));if !control&&nr>0{useful+=1;}
  observations.push(format!("{{\"serial\":{},\"grid_index\":{},\"left\":{},\"right\":{},\"cap\":{},\"status\":\"{}\",\"ordinary_degree\":{},\"expanded_weighted_witness_degree\":{},\"basis_rows\":{},\"ordered_root_rows\":{},\"generic_extraction_pairs\":{}}}",serial,g,l,r,cap,status,od,wd,nb,nr,attempts));
 }assert!(lines.next().is_none());assert_eq!(plans.len(),154);
 fs::write(&args[2],format!("{{\"scientific_configurations\":153,\"positive_controls\":1,\"complete_scientific_bases\":{},\"unknown_scientific_bases\":{},\"nonempty_certified_cases\":{},\"records\":[{}]}}\n",complete,unknown,useful,observations.join(","))).unwrap();
}
fn main(){if std::panic::catch_unwind(run).is_err(){std::process::exit(101)}}
