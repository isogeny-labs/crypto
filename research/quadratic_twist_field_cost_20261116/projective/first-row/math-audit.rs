// Independent coefficient-pair field arithmetic, linear Miller chain and linear field DLP reference.
use std::{collections::BTreeSet,fmt::Write as _,fs,io::Read as _};
use std::path::Path;
#[link(name="crypto")]extern "C"{fn SHA256(p:*const u8,n:usize,out:*mut u8)->*mut u8;}
fn digest(s:&[u8])->String{let mut out=[0u8;32];unsafe{SHA256(s.as_ptr(),s.len(),out.as_mut_ptr());}out.iter().map(|x|format!("{:02x}",x)).collect()}
type F=(i64,i64);type P=Option<(F,F)>;fn md(x:i64)->i64{x.rem_euclid(61)}fn addf(x:F,y:F)->F{(md(x.0+y.0),md(x.1+y.1))}fn nf(x:F)->F{(md(-x.0),md(-x.1))}fn subf(x:F,y:F)->F{addf(x,nf(y))}static BASE_POLY:std::sync::atomic::AtomicBool=std::sync::atomic::AtomicBool::new(false);
struct BaseScope(bool);impl BaseScope{fn new(b:bool)->Self{Self(BASE_POLY.swap(b,std::sync::atomic::Ordering::Relaxed))}}impl Drop for BaseScope{fn drop(&mut self){BASE_POLY.store(self.0,std::sync::atomic::Ordering::Relaxed);}}
fn mf(x:F,y:F)->F{if BASE_POLY.load(std::sync::atomic::Ordering::Relaxed){assert_eq!(x.1,0);assert_eq!(y.1,0);return(md(x.0*y.0),0)}(md(x.0*y.0+59*x.1*y.1),md(x.0*y.1+x.1*y.0))}fn pow(mut x:F,mut n:i64)->F{let mut r=(1,0);while n>0{if n%2==1{r=mf(r,x)}x=mf(x,x);n/=2;}r}fn div(x:F,y:F)->F{assert_ne!(y,(0,0));if BASE_POLY.load(std::sync::atomic::Ordering::Relaxed){assert_eq!(x.1,0);assert_eq!(y.1,0);return mf(x,pow(y,59))}mf(x,pow(y,3719))}fn enc(x:F)->i64{x.0+61*x.1}
fn neg(p:P)->P{p.map(|(x,y)|(x,nf(y)))}fn on(p:P,a:i64,b:i64)->bool{p.is_none_or(|(x,y)|mf(y,y)==addf(addf(mf(mf(x,x),x),mf((a,0),x)),(b,0)))}
fn plus(p:P,q:P,a:i64)->P{let((x,y),(u,v))=match(p,q){(None,q)=>return q,(p,None)=>return p,(Some(p),Some(q))=>(p,q)};if x==u&&addf(y,v)==(0,0){return None}let m=if x==u{div(addf(mf((3,0),mf(x,x)),(a,0)),mf((2,0),y))}else{div(subf(v,y),subf(u,x))};let z=subf(subf(mf(m,m),x),u);let c=subf(y,mf(m,x));Some((z,nf(addf(mf(m,z),c))))}
fn mul(p:P,n:i64,a:i64)->P{let mut r=None;for _ in 0..n{r=plus(r,p,a)}r}fn chi(p:P)->P{p.map(|(x,y)|(div(x,(59,0)),div(y,(0,59))))}
fn line(p:P,q:P,at:P)->Option<F>{let((x,y),(u,v),(xx,yy))=(p?,q?,at?);if x==u&&addf(y,v)==(0,0){let z=subf(xx,x);return(z!=(0,0)).then_some(z)}let den=if x==u{mf((2,0),y)}else{subf(u,x)};if den==(0,0){return None}let m=if x==u{div(addf(mf((3,0),mf(x,x)),(21,0)),den)}else{div(subf(v,y),den)};let num=subf(subf(yy,y),mf(m,subf(xx,x)));let vert=plus(p,q,21).map_or((1,0),|(z,_)|subf(xx,z));(num!=(0,0)&&vert!=(0,0)).then(||div(num,vert))}
fn miller(p:P,at:P)->Option<F>{p?;at?;let mut r=p;let mut f=(1,0);for _ in 1..31{f=mf(f,line(r,p,at)?);r=plus(r,p,21)}assert_eq!(r,None);Some(f)}fn weil(p:P,q:P)->Option<F>{if p.is_none()||q.is_none(){return Some((1,0))}Some(nf(div(miller(p,q)?,miller(q,p)?)))}fn shifted(p:P,q:P,s:P)->Option<F>{if p.is_none()||q.is_none(){return Some((1,0))}Some(mf(div(miller(p,plus(q,s,21))?,miller(p,s)?),div(miller(q,neg(s))?,miller(q,plus(p,neg(s),21))?)))}
fn group(g:P,a:i64,b:i64)->Vec<P>{let mut set=BTreeSet::new();let mut p=None;for _ in 0..31{assert!(set.insert(p));assert!(on(p,a,b));p=plus(p,g,a);}assert_eq!(p,None);set.into_iter().collect()}
fn point(p:P)->String{match p{None=>"-1 -1".to_string(),Some((x,y))=>format!("{} {}",enc(x),enc(y))}}
fn solve(g:F,value:F)->i64{assert_ne!(g,(1,0));assert_eq!(pow(g,31),(1,0));let mut p=(1,0);for n in 0..31{if p==value{return n}p=mf(p,g);}panic!("finite field log absent")}
type Poly=Vec<F>;fn trim(a:&mut Poly){while a.last()==Some(&(0,0)){a.pop();}}fn degree(a:&Poly)->i32{a.len()as i32-1}fn scale(mut a:Poly,b:F)->Poly{for x in &mut a{*x=mf(*x,b)}trim(&mut a);a}fn sub(mut a:Poly,b:&Poly)->Poly{a.resize(a.len().max(b.len()),(0,0));for(i,&x)in b.iter().enumerate(){a[i]=subf(a[i],x)}trim(&mut a);a}
fn bound(n:i32,cap:i32,peak:&mut i32)->Result<(),()>{if n>cap{return Err(())}*peak=(*peak).max(n);Ok(())}
fn product(a:&Poly,b:&Poly,cap:i32,weight:i32,peak:&mut i32)->Result<Poly,()>{if a.is_empty()||b.is_empty(){return Ok(vec![])}bound(degree(a)+degree(b)+weight,cap,peak)?;let mut c=vec![(0,0);a.len()+b.len()-1];for(i,&x)in a.iter().enumerate(){for(j,&y)in b.iter().enumerate(){c[i+j]=addf(c[i+j],mf(x,y))}}trim(&mut c);Ok(c)}
fn divide(mut a:Poly,b:&Poly,cap:i32,peak:&mut i32)->Result<(Poly,Poly),()>{assert!(!b.is_empty());let mut q=vec![(0,0);(degree(&a)-degree(b)+1).max(0)as usize];while !a.is_empty()&&degree(&a)>=degree(b){let k=(degree(&a)-degree(b))as usize;let v=div(*a.last().unwrap(),*b.last().unwrap());q[k]=addf(q[k],v);let mut mono=vec![(0,0);k+1];mono[k]=v;a=sub(a,&product(&mono,b,cap,0,peak)?);}trim(&mut q);Ok((q,a))}
static TERMINAL_STOP:std::sync::atomic::AtomicBool=std::sync::atomic::AtomicBool::new(false);
fn certificate(a:&[F],b:&[F],z:F,cap:i32,peak:&mut i32)->Result<(Poly,Poly,Poly,Poly,Poly),()>{
 assert_eq!(pow(z,31),(1,0));assert_ne!(z.0,60);let route=ROUTE.load(std::sync::atomic::Ordering::Relaxed);let _mode=BaseScope::new(route==4);bound(3,cap,peak)?;let mut f=vec![(1,0)];let mut h=f.clone();
 if route==4{for &(aa,bb) in a{assert_ne!(aa,60);assert_eq!(md(aa*aa-59*bb*bb),1);f=product(&f,&vec![(md(-bb),0),(md(aa+1),0)],cap,0,peak)?;}for &(aa,bb) in b{assert_ne!(aa,60);assert_eq!(md(aa*aa-59*bb*bb),1);h=product(&h,&vec![(md((aa+1)*z.1-bb*(z.0+1)),0),(md(-(aa+1)*(z.0+1)+59*bb*z.1),0)],cap,0,peak)?;}}
 else{for &x in a{f=product(&f,&vec![nf(x),(1,0)],cap,0,peak)?;}for &x in b{h=product(&h,&vec![z,nf(x)],cap,0,peak)?;}}
 assert_eq!(degree(&f),3);assert_eq!(degree(&h),3);let u=vec![*h.last().unwrap()];let v=vec![nf(*f.last().unwrap())];let r=sub(scale(f.clone(),u[0]),&scale(h.clone(),nf(v[0])));assert!(degree(&r)<=2);
 if r.is_empty(){let u=vec![div((1,0),*f.last().unwrap())];let d=scale(f.clone(),u[0]);assert_eq!(product(&u,&f,cap,0,peak)?,d);return Ok((f,h,d,u,vec![]))}
 assert!(sub(sub(product(&u,&f,cap,0,peak)?,&scale(product(&v,&h,cap,0,peak)?,(-1,0))),&r).is_empty());
 Ok((f,h,r,u,v))
}
// Independent square-root enumeration: choose the real coordinate in F61,
// derive the imaginary coordinate from 2uv=b, and verify the square directly.
// This shares no Tonelli--Shanks implementation with the producer.
fn independent_square_roots(n:F)->Vec<F>{
 let base=BASE_POLY.load(std::sync::atomic::Ordering::Relaxed);let mut out=vec![];
 if base{assert_eq!(n.1,0);for u in 0..61{if md(u*u)==n.0{out.push((u,0));}}}
 else{for u in 0..61{if u==0{if n.1==0{for v in 0..61{let x=(0,v);if mf(x,x)==n{out.push(x);}}}}else{let v=md(n.1*pow((md(2*u),0),59).0);let x=(u,v);if mf(x,x)==n{out.push(x);}}}}
 out.sort();out.dedup();out
}
fn unrestricted_cover_roots(r:&Poly)->Vec<F>{
 assert!(!r.is_empty()&&degree(r)<=3);let mut out=vec![];
 if degree(r)==3{
  let base=BASE_POLY.load(std::sync::atomic::Ordering::Relaxed);
  for real in 0..61{for imag in 0..if base{1}else{61}{let x=(real,imag);if eval(r,x)==(0,0){out.push(x)}}}
  out.sort();out.dedup();assert_eq!(out.len(),3,"actual distinct split cubic complete reference");return out;
 }
 if degree(r)==1{out.push(div(nf(r[0]),r[1]));}
 if degree(r)==2{let disc=subf(mf(r[1],r[1]),mf((4,0),mf(r[2],r[0])));for delta in independent_square_roots(disc){out.push(div(addf(nf(r[1]),delta),mf((2,0),r[2])));}}
 out.sort();out.dedup();for &x in &out{assert_eq!(eval(r,x),(0,0));}out
}

// Independent reference computes original roots first, then the exact norm domain.
// It deliberately shares neither the producer shortcut nor Tonelli--Shanks.
fn cover_roots(r:&Poly)->Vec<F>{
 let mut roots=unrestricted_cover_roots(r);
 if !BASE_POLY.load(std::sync::atomic::Ordering::Relaxed){roots.retain(|&x|x!=(0,0)&&mf(x,(x.0,md(-x.1)))==(1,0));}
 roots
}

fn root(a:&Poly,x:F)->bool{let _mode=BaseScope::new(true);let(mut value,mut den)=((0,0),(1,0));for &c in a.iter().rev(){value=addf(mf(value,(x.1,0)),mf(c,den));den=mf(den,(md(x.0+1),0));}value==(0,0)}
fn eval(a:&Poly,x:F)->F{a.iter().rev().fold((0,0),|z,&c|addf(mf(z,x),c))}fn poly(a:&Poly)->String{let mut z=a.len().to_string();for &x in a{write!(z,",{}",enc(x)).unwrap()}z}
fn rel(x:i64)->i64{x.rem_euclid(31)}fn rinv(x:i64)->i64{(1..31).find(|&y|rel(x*y)==1).unwrap()}
fn insert(matrix:&mut std::collections::BTreeMap<usize,[i64;4]>,row:[i64;3],m:i64){let mut r=[rel(row[0]),rel(row[1]),rel(row[2]),rel(m)];for(&p,b)in matrix.iter(){let c=r[p];for j in 0..4{r[j]=rel(r[j]-c*b[j]);}}let p=(0..3).find(|&j|r[j]!=0);let Some(p)=p else{assert_eq!(r[3],0);return};let k=rinv(r[p]);for x in &mut r{*x=rel(*x*k)}for b in matrix.values_mut(){let c=b[p];for j in 0..4{b[j]=rel(b[j]-c*r[j]);}}matrix.insert(p,r);}
fn query_cold(q:P,s:P,base:&[P],images:&[F],cap:i32,count:&mut[i64;11],trace:&mut String,supplied:Option<F>,first_branch_only:bool)->Vec<[i64;3]>{
 let z=if cap>=3{if supplied.is_none(){count[1]+=1;}supplied.unwrap_or_else(||pair(q,s).unwrap())}else{(0,0)};writeln!(trace,"QUERY {} {}",point(q),enc(z)).unwrap();let mut rows=vec![];
 for sign in [-1,1]{for sign2 in [-1,1]{count[2]+=1;let mut peak=0;write!(trace,"BR {} {} ",sign,sign2).unwrap();
  let result=if cap<3{Err(())}else{let a:Vec<_>=images.iter().map(|&x|if sign==1{x}else{if ROUTE.load(std::sync::atomic::Ordering::Relaxed)>=4{(x.0,md(-x.1))}else{div((1,0),x)}}).collect();let b:Vec<_>=images.iter().map(|&x|if sign2==1{x}else{if ROUTE.load(std::sync::atomic::Ordering::Relaxed)>=4{(x.0,md(-x.1))}else{div((1,0),x)}}).collect();certificate(&a,&b,z,cap,&mut peak).map(|c|(a,b,c))};
  match result{Ok((a,b,(f,h,d,u,v)))=>{
   let extracted={let _mode=BaseScope::new(ROUTE.load(std::sync::atomic::Ordering::Relaxed)==4);cover_roots(&d)};let mut found=vec![];
   for &x in &extracted{count[5]+=1;let both={let _mode=BaseScope::new(ROUTE.load(std::sync::atomic::Ordering::Relaxed)==4);eval(&f,x)==(0,0)&&eval(&h,x)==(0,0)};if !both{continue;}
    for i in 0..3{let belongs=if ROUTE.load(std::sync::atomic::Ordering::Relaxed)==4{(a[i].1,0)==mf(x,(md(a[i].0+1),0))}else{a[i]==x};if belongs{for j in 0..3{if mf(a[i],b[j])==z{assert_eq!(plus(if sign==1{base[i]}else{neg(base[i])},if sign2==1{base[j]}else{neg(base[j])},21),q);let mut row=[0;3];row[i]+=sign;row[j]+=sign2;rows.push(row);found.push((i,j));}}}}
   }
   found.sort();let mut reference=vec![];for i in 0..3{for j in 0..3{if plus(if sign==1{base[i]}else{neg(base[i])},if sign2==1{base[j]}else{neg(base[j])},21)==q{reference.push((i,j));}}}assert_eq!(found,reference,"complete actual signed source-row coverage AFTER root extraction");
   write!(trace,"C {} {} {} {} {} {} {}",peak,poly(&f),poly(&h),poly(&d),poly(&u),poly(&v),extracted.len()).unwrap();for &x in &extracted{write!(trace," {}",enc(x)).unwrap();}write!(trace," {}",found.len()).unwrap();for(i,j)in &found{write!(trace," {},{}",i,j).unwrap()}trace.push('\n');count[4]+=1;count[6]+=found.len()as i64;
  },Err(())=>{writeln!(trace,"U {}",peak).unwrap();count[3]+=1;}}
  count[9]=count[9].max(peak as i64);if first_branch_only&&!rows.is_empty(){return rows}
 }}rows
}

struct Recovery{scalar:i64,kind:i64,rank:usize,setup_m:i64,shift:i64,logs:[i64;3],count:[i64;11],trace:String}

static ROUTE:std::sync::atomic::AtomicUsize=std::sync::atomic::AtomicUsize::new(0);
fn pair(p:P,q:P)->Option<F>{if ROUTE.load(std::sync::atomic::Ordering::Relaxed)<2{return weil(p,q)}if p.is_none()||q.is_none(){return Some((1,0))}Some(pow(miller(p,q)?,120))}
fn recover_cold(unknown:P,g:P,s:P,base:&[P],cap:i32)->Recovery{let route=ROUTE.load(std::sync::atomic::Ordering::Relaxed);let mut r=Recovery{scalar:-1,kind:0,rank:0,setup_m:0,shift:-1,logs:[-1;3],count:[0;11],trace:String::new()};let mut images=vec![];for &p in base{let v=pair(p,s).unwrap();assert_ne!(v,(1,0));images.push(v);r.count[0]+=1;}let zeta=if route>=3{r.count[1]+=1;let z=pair(g,s).unwrap();assert_ne!(z,(1,0));assert_eq!(pow(z,31),(1,0));z}else{(1,0)};let mut public_image=(1,0);let mut matrix=std::collections::BTreeMap::new();let mut setup=None;
for m in 1..=30{if matrix.len()==3{break}setup=plus(setup,g,21);r.count[7]+=1;writeln!(r.trace,"PUBLIC {}",m).unwrap();let supplied=if route>=3{public_image=mf(public_image,zeta);r.count[10]+=1;Some(public_image)}else{None};let rows=query_cold(setup,s,base,&images,cap,&mut r.count,&mut r.trace,supplied,false);for row in rows{insert(&mut matrix,row,m)}r.setup_m=m;}
r.rank=matrix.len();if r.rank!=3{return r}r.logs=[matrix[&0][3],matrix[&1][3],matrix[&2][3]];for i in 0..3{assert_eq!(mul(g,r.logs[i],21),base[i]);}let mut image=if route>=3{r.count[1]+=1;pair(unknown,s).unwrap()}else{(1,0)};let mut shifted=unknown;
for shift in 0..=30{r.count[8]+=1;writeln!(r.trace,"ONLINE {}",shift).unwrap();if shifted.is_none(){r.scalar=rel(-shift);r.kind=2;r.shift=shift;r.trace.push_str("IDENTITY\n");break}let rows=query_cold(shifted,s,base,&images,cap,&mut r.count,&mut r.trace,if route>=3{Some(image)}else{None},true);if !rows.is_empty(){r.scalar=rel((0..3).map(|i|rows[0][i]*r.logs[i]).sum::<i64>()-shift);r.kind=1;r.shift=shift;break}shifted=plus(shifted,g,21);if route>=3{image=mf(image,zeta);r.count[10]+=1;}}
assert!(r.scalar>=0);assert_eq!(mul(g,r.scalar,21),unknown);r}

struct Profile{route:usize,policy:usize,ranks:Vec<usize>,works:Vec<Recovery>}
fn numbers<T:std::fmt::Display>(v:&[T])->String{v.iter().map(|x|x.to_string()).collect::<Vec<_>>().join(",")}
fn profile(route:usize,policy:usize,g:P,s:P,source:&[P],base:&[P])->(String,Profile){
 ROUTE.store(route,std::sync::atomic::Ordering::Relaxed);let mut out=format!("CARDANO {} {} 3 61 59 31\nBASE",route,policy);for &p in base{write!(out," {}",point(p)).unwrap()}out.push('\n');let mut count=[0i64;11];let images:Vec<_>=base.iter().map(|&p|pair(p,s).unwrap()).collect();let mut ranks=vec![];
 for(qi,&q)in source.iter().enumerate(){writeln!(out,"DIAG {}",qi).unwrap();let rows=query_cold(q,s,base,&images,3,&mut count,&mut out,None,false);let mut m=std::collections::BTreeMap::new();for row in rows{insert(&mut m,row,0)}ranks.push(m.len());writeln!(out,"RANK {}",m.len()).unwrap();}
 let mut works=vec![];for(qi,&q)in source.iter().enumerate().skip(1){let r=recover_cold(q,g,s,base,3);write!(out,"WORK {} {} {} {} {} {}",qi,r.scalar,r.kind,r.rank,r.setup_m,r.shift).unwrap();for x in r.logs{write!(out," {}",x).unwrap()}for x in r.count{write!(out," {}",x).unwrap()}out.push_str("\nEND\n");works.push(r);}
 (out,Profile{route,policy,ranks,works})
}
fn profile_json(p:&Profile)->String{
 let mut out=format!("{{\"route\":{},\"policy\":{},\"cap\":3,\"diagnostic_ranks\":[{}],\"rank_numerator\":{},\"works\":[",p.route,p.policy,numbers(&p.ranks),p.ranks.iter().skip(1).sum::<usize>());
 for(i,r)in p.works.iter().enumerate(){if i>0{out.push(',')}write!(out,"{{\"target\":{},\"result\":{{\"scalar\":{},\"kind\":{},\"rank\":{},\"setup_m\":{},\"shift\":{},\"logs\":[{}],\"counts\":[{}]}}}}",i+1,r.scalar,r.kind,r.rank,r.setup_m,r.shift,numbers(&r.logs),numbers(&r.count)).unwrap();}
 out.push_str(&format!("],\"matrix_degree\":null,\"polynomial_field\":\"{}\",\"original_input_pullback_witness_degree\":null}}",if p.route==4{"F61"}else{"F61^2"}));out
}
fn next_line<'a>(text:&'a str,pos:&mut usize)->&'a str{let end=text[*pos..].find('\n').expect("complete line")+*pos;let row=&text[*pos..end];*pos=end+1;row}
fn audit(head:&str,a:&Path,b:&Path){
 let mut c=String::new();std::io::stdin().read_to_string(&mut c).unwrap();let aa=fs::read_to_string(a).unwrap();let bb=fs::read_to_string(b).unwrap();let texts=[c.as_str(),aa.as_str(),bb.as_str()];let mut pos=[0usize;3];
 for(i,tag)in["C","A","B"].iter().enumerate(){assert_eq!(next_line(texts[i],&mut pos[i]),format!("TWIST-FIRST-ROW 1 {} {} 61 59 31 21 39 23 54 0 10 37 14",head,tag));}
 assert!((0..61).all(|x|md(x*x)!=59));let g=Some(((0,0),(10,0)));let partner=chi(Some(((37,0),(14,0))));assert!(on(g,21,39)&&on(partner,21,39)&&mul(g,31,21).is_none()&&mul(partner,31,21).is_none());assert_eq!(partner.map(|(x,y)|(pow(x,61),pow(y,61))),neg(partner));let source=group(g,21,39);let reps:Vec<_>=source.iter().copied().filter(|p|p.is_some()&&p<&neg(*p)).collect();assert_eq!(reps.len(),15);let mut profiles=vec![];
 for route in 3..6{ROUTE.store(route,std::sync::atomic::Ordering::Relaxed);let z=pair(g,partner).unwrap();assert_ne!(z,(1,0));assert_eq!(pow(z,31),(1,0));for &x in &source{for &y in &source{assert_eq!(pair(plus(x,y,21),partner).unwrap(),mf(pair(x,partner).unwrap(),pair(y,partner).unwrap()));}}
  let mut policy=0;for i in 0..15{for j in i+1..15{for k in j+1..15{
   let(body,p)=profile(route,policy,g,partner,&source,&[reps[i],reps[j],reps[k]]);let hash=digest(body.as_bytes());let frame=format!("CASE {} {} 3 {} {}",route,policy,body.len(),hash);
   for index in 0..3{assert_eq!(next_line(texts[index],&mut pos[index]),frame,"actual full case membership/hash");if index==0{let end=pos[0]+body.len();assert!(end<=c.len());assert_eq!(&c[pos[0]..end],body,"complete independent roots/witnesses/rows/public logs/source scalars");pos[0]=end;}assert_eq!(next_line(texts[index],&mut pos[index]),"ENDCASE");}
   profiles.push(p);policy+=1;
  }}}assert_eq!(policy,455);
 }assert_eq!(profiles.len(),1365);for i in 0..3{assert_eq!(next_line(texts[i],&mut pos[i]),"DONE 1365");assert_eq!(pos[i],texts[i].len(),"no extra records");}
 let mut stats=vec![];for route in 3..6{let(mut algebraic,mut identity,mut blocked,mut queries)=(0,0,0,0);for p in profiles.iter().filter(|p|p.route==route){for r in &p.works{match r.kind{0=>blocked+=1,1=>algebraic+=1,2=>identity+=1,_=>panic!()};queries+=r.count[0]+r.count[1];}}stats.push(format!("{{\"route\":{},\"cap\":3,\"profiles\":455,\"algebraic\":{},\"identity\":{},\"blocked\":{},\"map_calls\":{}}}",route,algebraic,identity,blocked,queries));}
 let data=profiles.iter().map(profile_json).collect::<Vec<_>>().join(",");println!("{{\"schema\":\"twist.first-row.audit/v1\",\"head\":\"{}\",\"profiles\":1365,\"target_workloads\":40950,\"diagnostic_cells\":42315,\"raw_C_sha256\":\"{}\",\"raw_A_sha256\":\"{}\",\"raw_B_sha256\":\"{}\",\"stats\":[{}],\"profiles_data\":[{}],\"matrix_degree\":null,\"signed_verdict\":false}}",head,digest(c.as_bytes()),digest(aa.as_bytes()),digest(bb.as_bytes()),stats.join(","),data);
}
fn selftest(head:&str){let mut text=String::new();std::io::stdin().read_to_string(&mut text).unwrap();let mut at=0;assert_eq!(next_line(&text,&mut at),format!("CUBIC-SELFTEST {}",head));let mut total=0;
 for domain in 0..2{ROUTE.store(3,std::sync::atomic::Ordering::Relaxed);let mut alphabet=vec![];if domain==0{for a in 0..61{alphabet.push((a,0));}}else{alphabet.push((0,0));let g=Some(((0,0),(10,0)));let s=chi(Some(((37,0),(14,0))));let z=pair(g,s).unwrap();assert_ne!(z,(1,0));assert_eq!(pow(z,31),(1,0));let mut x=(1,0);for _ in 0..31{alphabet.push(x);x=mf(x,z);}}alphabet.sort();assert_eq!(alphabet.iter().collect::<BTreeSet<_>>().len(),alphabet.len());let _scope=BaseScope::new(domain==0);let mut count=0;
  for i in 0..alphabet.len(){for j in i..alphabet.len(){for k in j..alphabet.len(){let mut peak=0;let mut f=vec![(1,0)];for x in[alphabet[i],alphabet[j],alphabet[k]]{f=product(&f,&vec![nf(x),(1,0)],3,0,&mut peak).unwrap();}let mut expected=vec![alphabet[i],alphabet[j],alphabet[k]];expected.dedup();let mut row=format!("S {} {} {} {} {} {}",domain,i,j,k,poly(&f),expected.len());for x in expected{write!(row," {}",enc(x)).unwrap();}assert_eq!(next_line(&text,&mut at),row,"complete coefficient-only root selftest result");count+=1;}}}assert_eq!(next_line(&text,&mut at),format!("DOMAIN {} {}",domain,count));total+=count;
 }assert_eq!(next_line(&text,&mut at),"SELFTEST-DONE");assert_eq!(at,text.len());println!("{{\"schema\":\"twist.cubic-root-selftest.audit/v1\",\"head\":\"{}\",\"split_cubic_instances\":{},\"all_repeated_roots_retained\":true,\"raw_sha256\":\"{}\",\"signed_verdict\":false}}",head,total,digest(text.as_bytes()));}
fn main(){let args:Vec<_>=std::env::args().collect();let result=std::panic::catch_unwind(||{if args.len()==5&&args[1]=="audit"{audit(&args[2],Path::new(&args[3]),Path::new(&args[4]));}else if args.len()==3&&args[1]=="selftest"{selftest(&args[2]);}else{panic!("audit HEAD A B or selftest HEAD")}});if result.is_err(){std::process::exit(101)}}
