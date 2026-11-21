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
fn nfb(p:&Poly,b:&[Poly],bounds:&[i32],w:bool)->(Poly,i32){let mut r=p.clone();let mut out=Poly::new();let mut high=pd(p,w);while !r.is_empty(){let(k,v)=lead(&r,w);let mut reduced=false;for (i,q) in b.iter().enumerate(){let(j,s)=lead(q,w);if divs(j,k){high=high.max(bounds[i]+deg(k-j,w));r=sub(&r,&shift(q,k-j,m(v,iv(s))));reduced=true;break;}}if !reduced{r.remove(&k);out.insert(k,v);}}(out,high)}
fn lc(u:u32,v:u32)->u32{(0..5).map(|i|(ex(u,i).max(ex(v,i)) as u32)<<(6*i)).sum()}
fn poly(s:&str)->Poly{if s=="-"{return Poly::new()}let mut p=Poly::new();for term in s.split(','){let(k,v)=term.split_once(':').unwrap();let k=k.parse::<u32>().unwrap();let v=v.parse::<i32>().unwrap();assert!((1..49).contains(&v));assert!(p.insert(k,from(v)).is_none());}p}
fn membership(roots:&BTreeSet<i32>,x:&Poly)->Poly{let mut p=c(f(1,0));for &v in roots{p=pm(&p,&sub(x,&c(from(v))));}p}
fn system(base:&[BTreeSet<i32>],gamma:i32,beta:i32,left:i32,right:i32,q:P,source_mode:bool)->Vec<Poly>{let (s,t)=match q{P::V(x,y)=>(x.0,y.0),_=>panic!()};let u=var(0);let v=var(1);let a0=var(2);let z=var(3);let a1=var(4);let aa=from(1);let bb=from(8);
 if left<0||source_mode{let xy=pm(&u,&v);let sum=pa(&u,&v);let dif=sub(&u,&v);let mut p=ps(&pp(&dif,2),m(s,s));p=sub(&p,&ps(&pa(&pm(&sum,&pa(&xy,&c(aa))),&c(m(f(2,0),bb))),m(f(2,0),s)));p=pa(&p,&pp(&sub(&xy,&c(aa)),2));p=sub(&p,&ps(&sum,m(f(4,0),bb)));let roots: BTreeSet<i32>=base.iter().flat_map(|x|x.iter().copied()).collect();let lx=if left<0{roots.clone()}else{base[left as usize].clone()};let rx=if right<0{roots.clone()}else{base[right as usize].clone()};return vec![p,membership(&lx,&u),membership(&rx,&v)]}
 let mut inputs=vec![];let ls=[v,z];let av=[a0,a1];let bs=[left,right];let mut xs=vec![];
 for i in 0..2{let x=if bs[i]==0{ls[i].clone()}else{sub(&pp(&ls[i],2),&ps(&pa(&av[i],&c(from(gamma))),f(2,0)))};let y=sub(&pm(&u,&sub(&x,&c(s))),&c(t));let curve=sub(&sub(&sub(&pp(&y,2),&pp(&x,3)),&ps(&x,aa)),&c(bb));inputs.push(curve);
  if bs[i]==0{inputs.push(membership(&base[0],&x));inputs.push(av[i].clone());}else{let zz=m(from(beta),f(bs[i],0));let dd=m(from(8),m(zz,zz));let mut p=pp(&ls[i],4);p=sub(&p,&ps(&pm(&x,&pp(&ls[i],2)),f(6,0)));p=sub(&p,&ps(&pp(&x,2),f(3,0)));p=sub(&p,&ps(&pm(&y,&ls[i]),f(8,0)));p=sub(&p,&c(m(f(4,0),a(aa,dd))));inputs.push(p);inputs.push(sub(&pp(&av[i],7),&av[i]));}xs.push(x);
 }
 inputs.push(sub(&pa(&xs[0],&xs[1]),&sub(&pp(&u,2),&c(s))));let prod=pa(&pa(&ps(&pp(&u,2),s),&ps(&u,m(f(2,0),t))),&c(a(m(s,s),aa)));inputs.push(sub(&pm(&xs[0],&xs[1]),&prod));inputs
}
struct Reader{lines:Vec<String>,i:usize}
impl Reader{fn next(&mut self)->Vec<String>{let s=self.lines.get(self.i).unwrap().split_whitespace().map(str::to_string).collect();self.i+=1;s}fn exact(&mut self,s:&str){assert_eq!(self.next(),vec![s.to_string()]);}}
fn num(s:&str)->i32{s.parse().unwrap()}
fn point(x:i32,y:i32)->P{if x<0{assert_eq!((x,y),(-1,-1));P::O}else{assert!((0..49).contains(&x)&&(0..49).contains(&y));P::V(K(from(x),f(0,0)),K(from(y),f(0,0)))}}
fn coords(p:P)->(i32,i32){match p{P::O=>(-1,-1),P::V(x,y)=>(enc(x.0),enc(y.0))}}
#[derive(Default,Clone)]struct Count{calls:i32,terms:u64,pairs:u64,maximum:i32,accepted:i32,unknown:i32,rows:BTreeSet<[i32;4]>,generic:bool}
impl Count{fn new()->Self{Self{maximum:-1,..Self::default()}}fn charge(&mut self,v:&Self){self.calls+=v.calls;self.terms+=v.terms;self.pairs+=v.pairs;self.maximum=self.maximum.max(v.maximum);self.accepted+=v.accepted;self.unknown+=v.unknown;self.rows.extend(v.rows.iter().copied());}}
fn audit_case(rd:&mut Reader,base:&[BTreeSet<i32>],gamma:i32,beta:i32,mode:i32,left:i32,right:i32,q:P,qual:bool,source:&[P])->Count{
 let row=rd.next();assert_eq!(row.len(),9);assert_eq!(row[0],"CASE");assert_eq!((num(&row[1]),num(&row[2])),(left,right));let status=row[3].as_str();assert!(["complete","cap","resource"].contains(&status));let od=num(&row[4]);let wd=num(&row[5]);let work=row[6].parse::<u64>().unwrap();let ni=num(&row[7]) as usize;let nb=num(&row[8]) as usize;assert!(wd<=20&&od<=20&&nb<=1000);if status=="resource"{assert!(work>200000)}else{assert!(work<=200000)}
 let weighted=mode==2;let want=if qual{vec![sub(&var(0),&c(f(1,0))),sub(&var(1),&c(f(2,0)))]}else{system(base,gamma,beta,left,right,q,mode!=2)};assert_eq!(ni,want.len());let mut inputs=vec![];for p in want{let r=rd.next();assert_eq!(r.len(),2);assert_eq!(r[0],"I");assert_eq!(poly(&r[1]),p);inputs.push(p);}
 let mut bs=vec![];let mut bounds=vec![];for _ in 0..nb{let r=rd.next();assert_eq!(r.len(),ni+2);assert_eq!(r[0],"B");let p=poly(&r[1]);assert!(!p.is_empty());assert_eq!(lead(&p,weighted).1,f(1,0));assert!(pd(&p,false)<=od&&pd(&p,weighted)<=20);let mut exact=Poly::new();let mut wb=-1;for i in 0..ni{let w=poly(&r[i+2]);if !w.is_empty(){wb=wb.max(pd(&w,weighted)+pd(&inputs[i],weighted));}exact=pa(&exact,&pm(&w,&inputs[i]));}assert_eq!(exact,p);assert!(wb<=wd);bs.push(p);bounds.push(wb);}
 let mut verification=wd;if status=="complete"{assert!(!bs.is_empty());for p in &inputs{let(v,b)=nfb(p,&bs,&bounds,weighted);assert!(v.is_empty());verification=verification.max(b);}for i in 0..bs.len(){for j in 0..i{let(u,c0)=lead(&bs[i],weighted);let(v,c1)=lead(&bs[j],weighted);let k=lc(u,v);verification=verification.max(bounds[i]+deg(k-u,weighted)).max(bounds[j]+deg(k-v,weighted));let p=sub(&shift(&bs[i],k-u,iv(c0)),&shift(&bs[j],k-v,iv(c1)));let(rest,b)=nfb(&p,&bs,&bounds,weighted);assert!(rest.is_empty());verification=verification.max(b);}}assert!(verification<=20);}else{assert_eq!(nb,0);}
 let r=rd.next();assert_eq!(r.len(),3);assert_eq!(r[0],"ROOTS");let count=num(&r[1]) as usize;let pairs=r[2].parse::<u64>().unwrap();let mut want=BTreeSet::new();let mut tries=0;
 if status=="complete"&&!qual{let all:BTreeSet<_>=base.iter().flat_map(|b|b.iter().copied()).collect();let a=if left<0{&all}else{&base[left as usize]};let b=if right<0{&all}else{&base[right as usize]};for &p in source{for &r in source{if let(P::V(x,y),P::V(u,v))=(p,r){if !a.contains(&enc(x.0))||!b.contains(&enc(u.0)){continue}tries+=1;if add(p,r,from(1),from(8))==q{want.insert([enc(x.0),enc(y.0),enc(u.0),enc(v.0)]);}}}}}
 assert_eq!(tries,pairs);assert_eq!(count,want.len());for x in &want{let r=rd.next();assert_eq!(r[0],"R");assert_eq!(r.len(),5);assert_eq!(r[1..].iter().map(|x|num(x)).collect::<Vec<_>>(),x.to_vec());}rd.exact("ENDCASE");Count{calls:1,terms:work,pairs,maximum:wd,accepted:i32::from(status=="complete"),unknown:i32::from(status!="complete"),rows:want,generic:false}
}
fn z(x:i32)->i32{x.rem_euclid(47)}fn zi(x:i32)->i32{assert!(x!=0);let(mut a,mut n,mut r)=(x,45,1);while n>0{if n%2!=0{r=z(r*a)}a=z(a*a);n/=2}r}
struct Linear{n:usize,rows:BTreeMap<usize,Vec<i32>>}
impl Linear{fn new(n:usize)->Self{Self{n,rows:BTreeMap::new()}}fn add(&mut self,mut v:Vec<i32>,rhs:i32){v.push(rhs);for(&p,row)in &self.rows{let c=v[p];for i in 0..=self.n{v[i]=z(v[i]-c*row[i]);}}let pivot=(0..self.n).find(|&i|v[i]!=0);if let Some(p)=pivot{let c=zi(v[p]);for x in &mut v{*x=z(*x*c)}for row in self.rows.values_mut(){let c=row[p];for i in 0..=self.n{row[i]=z(row[i]-c*v[i]);}}self.rows.insert(p,v);}else{assert_eq!(v[self.n],0)}}fn logs(&self)->Vec<i32>{if self.rows.len()!=self.n{return vec![]}let mut x=vec![0;self.n];for(&p,row)in &self.rows{x[p]=row[self.n]}x}}
fn coefficients(p:[i32;4],b:&[P])->Vec<i32>{let mut v=vec![0;b.len()];for side in 0..2{let x=p[2*side];let y=from(p[2*side+1]);let i=b.iter().position(|&q|coords(q).0==x).unwrap();let yy=from(coords(b[i]).1);let sign=if yy==y{1}else{assert_eq!(n(yy),y);-1};v[i]=z(v[i]+sign);}v}
fn audit_query(rd:&mut Reader,g:usize,mode:i32,phase:i32,label:i32,q:P,serial:i32,bases:&[Vec<BTreeSet<i32>>],reps:&[P],lin:&mut Linear,source:&[P])->Count{
 let r=rd.next();let(x,y)=coords(q);let count=if q==P::O{0}else if mode==0{1}else{16};assert_eq!(r,vec!["CALL".to_string(),serial.to_string(),phase.to_string(),label.to_string(),x.to_string(),y.to_string(),count.to_string()]);let mut all=Count::new();
 if q==P::O{all.generic=true;for &p in reps{let(x,y)=coords(p);let ny=enc(n(from(y)));all.rows.insert([x,y,x,ny]);all.rows.insert([x,ny,x,y]);}let r=rd.next();assert_eq!(r,vec!["IDENTITY".to_string(),all.rows.len().to_string()]);for x in &all.rows{let r=rd.next();assert_eq!(r[0],"R");assert_eq!(r[1..].iter().map(|x|num(x)).collect::<Vec<_>>(),x.to_vec());}}
 else{let branches=if mode==0{vec![(-1,-1)]}else{(0..4).flat_map(|l|(0..4).map(move|r|(l,r))).collect()};for(l,r)in branches{let a=audit_case(rd,&bases[g],[7,28,35][g],[10,11,8][g],mode,l,r,q,false,source);all.charge(&a);}}
 if phase==0{for &r in &all.rows{lin.add(coefficients(r,reps),label);}}
 let r=rd.next();assert_eq!(r,vec!["ENDCALL".to_string(),lin.rows.len().to_string(),all.rows.len().to_string()]);all
}
fn run(){let args:Vec<_>=env::args().collect();assert_eq!(args.len(),3);let text=fs::read_to_string(&args[1]).unwrap();let mut rd=Reader{lines:text.lines().map(str::to_string).collect(),i:0};assert_eq!(rd.next().join(" "),"TC1 7 3 8 1 8 47 53 20 200000 9");let aa=from(1);let bb=from(8);let d=from(8);let zero=f(0,0);let generator=point(5,11);let target=point(22,12);assert!(on(target,aa,bb,d)&&target!=P::O&&on(generator,aa,bb,d));assert_eq!(mul(generator,47,aa,d),P::O);
 let mut source=vec![P::O];for x in 0..49{for y in 0..49{let p=point(x,y);if on(p,aa,bb,d){source.push(p)}}}assert_eq!(source.len(),47);
 let mut squares=BTreeMap::<i32,Vec<K>>::new();for x in 0..49{for y in 0..49{let v=K(from(x),from(y));let s=km(v,v,d);squares.entry(enc(s.0)*49+enc(s.1)).or_default().push(v);}}
 let mut bases=vec![vec![BTreeSet::new();4];3];let mut ext=1;for x0 in 0..49{for x1 in 0..49{let x=K(from(x0),from(x1));let rhs=ka(ka(km(km(x,x,d),x,d),km(K(aa,zero),x,d)),K(bb,zero));for &y in squares.get(&(enc(rhs.0)*49+enc(rhs.1))).unwrap_or(&vec![]){ext+=1;let p=P::V(x,y);let q=add(p,conj(p),aa,d);if let P::V(qx,qy)=q{assert_eq!(qx.1,zero);assert_eq!(qy.1,zero);for g in 0..3{let b=m(x.1,iv(from([10,11,8][g])));if a(x.0,n(from([7,28,35][g]))).1==0&&b.1==0{let br=b.0.min((-b.0).rem_euclid(7))as usize;bases[g][br].insert(enc(qx.0));}}}}}}assert_eq!(ext,2491);
 rd.exact("QUAL");let qualification=audit_case(&mut rd,&bases[0],7,10,0,-1,-1,generator,true,&source);assert_eq!(qualification.accepted,1);rd.exact("ENDQUAL");let mut reports=vec![];let mut algebraic=0;let mut generic=0;let mut blocked=0;
 for g in 0..3{for mode in 0..3{let id=g*3+mode;assert_eq!(rd.next(),vec!["WORK".to_string(),id.to_string(),g.to_string(),mode.to_string()]);for k in 0..4{let r=rd.next();assert_eq!(r.len(),3);assert_eq!(&r[..2],&["BASE".to_string(),k.to_string()]);let got:BTreeSet<i32>=r[2].split(',').map(|s|num(s)).collect();assert_eq!(got,bases[g][k]);}assert_eq!(rd.next(),vec!["SETUP_PAIRS".to_string(),"2491".to_string()]);let xs:BTreeSet<_>=bases[g].iter().flat_map(|b|b.iter().copied()).collect();let mut reps=vec![];for x in xs{let ys:Vec<_>=source.iter().copied().filter(|&q|coords(q).0==x).collect();assert_eq!(ys.len(),2);reps.push(*ys.iter().min_by_key(|&&q|coords(q).1).unwrap());}let r=rd.next();assert_eq!(r.len(),reps.len()+2);assert_eq!(&r[..2],&["COLUMNS".to_string(),reps.len().to_string()]);for(i,&p)in reps.iter().enumerate(){let(x,y)=coords(p);assert_eq!(r[i+2],format!("{}:{}",x,y));}
  let mut lin=Linear::new(reps.len());let mut serial=0;let mut total=Count::new();let mut setup_calls=0;for m in 1..47{let a=audit_query(&mut rd,g,mode as i32,0,m,mul(generator,m,aa,d),serial,&bases,&reps,&mut lin,&source);serial+=1;setup_calls+=1;total.charge(&a);if lin.rows.len()==reps.len(){break}}
  let logs=lin.logs();let r=rd.next();assert_eq!(r.len(),logs.len()+2);assert_eq!(&r[..2],&["LOGS".to_string(),logs.len().to_string()]);assert_eq!(r[2..].iter().map(|x|num(x)).collect::<Vec<_>>(),logs);let mut status="blocked";let(mut value,mut shift)=(-1,-1);
  if logs.len()==reps.len(){for(i,&p)in reps.iter().enumerate(){assert_eq!(mul(generator,logs[i],aa,d),p);}status="failed";for s in 0..47{let q=add(target,mul(generator,s,aa,d),aa,d);let a=audit_query(&mut rd,g,mode as i32,1,s,q,serial,&bases,&reps,&mut lin,&source);serial+=1;total.charge(&a);if a.rows.is_empty(){continue}let coeff=coefficients(*a.rows.first().unwrap(),&reps);let k=z(coeff.iter().zip(&logs).map(|(a,b)|a*b).sum::<i32>()-s);assert_eq!(mul(generator,k,aa,d),target);status=if a.generic{"generic"}else{"algebraic"};value=k;shift=s;break;}}
  let want=vec!["RESULT".to_string(),status.to_string(),value.to_string(),shift.to_string(),serial.to_string(),total.calls.to_string(),total.terms.to_string(),total.pairs.to_string(),total.maximum.to_string(),lin.rows.len().to_string()];assert_eq!(rd.next(),want);rd.exact("ENDWORK");match status{"algebraic"=>algebraic+=1,"generic"=>generic+=1,"blocked"=>blocked+=1,_=>{}}
  reports.push(format!("{{\"id\":{},\"grid\":{},\"mode\":{},\"status\":\"{}\",\"source_scalar\":{},\"shift\":{},\"base_columns\":{},\"setup_rank\":{},\"setup_queries\":{},\"all_queries\":{},\"branch_calls\":{},\"reported_polynomial_terms\":{},\"generic_extraction_pairs\":{},\"maximum_attempt_witness_degree\":{},\"accepted_bases\":{},\"unknown_bases\":{}}}",id,[12,37,42][g],mode,status,value,shift,reps.len(),lin.rows.len(),setup_calls,serial,total.calls,total.terms,total.pairs,total.maximum,total.accepted,total.unknown));
 }}assert_eq!(rd.i,rd.lines.len());fs::write(&args[2],format!("{{\"cold_workloads\":9,\"algebraic_recoveries\":{},\"generic_identity_recoveries\":{},\"blocked_setups\":{},\"target_x\":22,\"target_y\":12,\"records\":[{}]}}\n",algebraic,generic,blocked,reports.join(","))).unwrap();
}
fn main(){if std::panic::catch_unwind(run).is_err(){std::process::exit(101)}}
