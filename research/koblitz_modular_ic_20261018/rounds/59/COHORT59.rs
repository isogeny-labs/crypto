//! Exact GF(2) quotient and complete stable-support cohort; no random search.
use std::{collections::BTreeMap,env,fs,io::{BufWriter,Write}};
#[derive(Clone,PartialEq,Eq,Debug)]struct V{n:usize,w:Vec<u64>}
impl V{
 fn z(n:usize)->Self{Self{n,w:vec![0;(n+63)/64]}}
 fn e(n:usize,i:usize)->Self{let mut v=Self::z(n);v.set(i);v}
 fn get(&self,i:usize)->bool{assert!(i<self.n);self.w[i/64]>>(i%64)&1!=0}
 fn set(&mut self,i:usize){assert!(i<self.n);self.w[i/64]|=1<<(i%64);}
 fn xor(&mut self,b:&Self){assert!(self.n==b.n);for(x,y)in self.w.iter_mut().zip(&b.w){*x^=*y;}}
 fn zero(&self)->bool{self.w.iter().all(|x|*x==0)}
 fn top(&self)->Option<usize>{for(i,x)in self.w.iter().enumerate().rev(){if *x!=0{return Some(64*i+63-x.leading_zeros()as usize);}}None}
 fn ones(&self)->Vec<usize>{let mut out=vec![];for(i,&word)in self.w.iter().enumerate(){let mut x=word;while x!=0{let j=x.trailing_zeros()as usize;out.push(64*i+j);x&=x-1;}}out}
 fn slice(&self,start:usize,n:usize)->Self{let mut x=Self::z(n);for i in 0..n{if self.get(start+i){x.set(i);}}x}
 fn hex(&self)->String{self.w.iter().map(|x|format!("{x:016x}")).collect::<Vec<_>>().join(",")}
 fn fromhex(n:usize,s:&str)->Self{let w=s.split(',').filter(|x|!x.is_empty()).map(|x|u64::from_str_radix(x,16).unwrap()).collect::<Vec<_>>();assert!(w.len()==(n+63)/64);if n%64!=0{assert!(w.last().unwrap()>>(n%64)==0);}Self{n,w}}
}
#[derive(Clone,PartialEq,Eq,Debug)]struct M{r:usize,c:usize,v:Vec<V>}
impl M{
 fn z(r:usize,c:usize)->Self{Self{r,c,v:vec![V::z(r);c]}}
 fn cols(r:usize,v:Vec<V>)->Self{assert!(v.iter().all(|x|x.n==r));Self{r,c:v.len(),v}}
 fn id(n:usize)->Self{Self::cols(n,(0..n).map(|i|V::e(n,i)).collect())}
 fn mv(&self,x:&V)->V{assert!(x.n==self.c);let mut y=V::z(self.r);for j in x.ones(){y.xor(&self.v[j]);}y}
 fn mul(&self,b:&Self)->Self{assert!(self.c==b.r);Self::cols(self.r,b.v.iter().map(|x|self.mv(x)).collect())}
 fn add(&self,b:&Self)->Self{assert!(self.r==b.r&&self.c==b.c);let mut x=self.clone();for(i,y)in b.v.iter().enumerate(){x.v[i].xor(y);}x}
 fn zero(&self)->bool{self.v.iter().all(V::zero)}
 fn rows(&self)->Vec<V>{let mut rows=vec![V::z(self.c);self.r];for(j,x)in self.v.iter().enumerate(){for i in x.ones(){rows[i].set(j);}}rows}
 fn rowpart(&self,start:usize,n:usize)->Self{Self::cols(n,self.v.iter().map(|x|x.slice(start,n)).collect())}
 fn inverse(&self)->Self{assert!(self.r==self.c);let n=self.r;let left=self.rows();let mut rows=vec![V::z(2*n);n];for i in 0..n{for j in left[i].ones(){rows[i].set(j);}rows[i].set(n+i);}
  for k in 0..n{let p=(k..n).find(|&i|rows[i].get(k)).expect("singular inverse");rows.swap(k,p);let pivot=rows[k].clone();for i in 0..n{if i!=k&&rows[i].get(k){rows[i].xor(&pivot);}}}
  let mut out=Self::z(n,n);for i in 0..n{assert!((0..n).all(|j|rows[i].get(j)==(i==j)));for j in 0..n{if rows[i].get(n+j){out.v[j].set(i);}}}out
 }
}
// Independent product checker: row-dot-column parity, not producer column XOR.
fn dotmul(a:&M,b:&M)->M{assert!(a.c==b.r);let rows=a.rows();let mut out=M::z(a.r,b.c);for(j,col)in b.v.iter().enumerate(){for(i,row)in rows.iter().enumerate(){let p=row.w.iter().zip(&col.w).fold(0u32,|s,(x,y)|s^((x&y).count_ones()&1));if p!=0{out.v[j].set(i);}}}out}
fn product(a:&M,b:&M,c:&M){assert!(dotmul(a,b)==*c,"independent product mismatch");}
struct Elim{image:M,pre:M,kernel:M}
fn eliminate(m:&M)->Elim{let mut piv:Vec<Option<(V,V)>>=vec![None;m.r];let mut kernels=vec![];
 for j in 0..m.c{let mut v=m.v[j].clone();let mut label=V::e(m.c,j);loop{match v.top(){None=>{kernels.push(label);break;},Some(p)=>{if let Some((b,l))=&piv[p]{v.xor(b);label.xor(l);}else{piv[p]=Some((v,label));break;}}}}}
 let(mut image,mut pre)=(vec![],vec![]);for entry in piv.into_iter().rev(){if let Some((v,l))=entry{image.push(v);pre.push(l);}}
 Elim{image:M::cols(m.r,image),pre:M::cols(m.c,pre),kernel:M::cols(m.c,kernels)}
}
fn decoder(image:&M)->M{let piv=image.v.iter().map(|v|v.top().unwrap()).collect::<Vec<_>>();let mut d=M::z(image.c,image.r);
 for j in 0..image.r{let mut v=V::e(image.r,j);for(k,b)in image.v.iter().enumerate(){if v.get(piv[k]){v.xor(b);d.v[j].set(k);}}}d}
fn left_inverse(m:&M)->M{let e=eliminate(m);assert!(e.image.c==m.c);let l=e.pre.mul(&decoder(&e.image));assert!(l.mul(m)==M::id(m.c));l}
struct Split{k:M,r:M,h:M,rank:usize}
fn split_kernel(c:&M)->Split{let e=eliminate(c);let h=e.pre.mul(&decoder(&e.image));let p=M::id(c.c).add(&h.mul(c));let k=e.kernel;let r=if k.c==0{M::z(0,c.c)}else{left_inverse(&k).mul(&p)};
 assert!(c.mul(&k).zero()&&r.mul(&k)==M::id(k.c)&&k.mul(&r).add(&h.mul(c))==M::id(c.c));Split{rank:e.image.c,k,r,h}}
struct Quot{e:M,g:M,h:M,rank:usize}
fn quotient(a:&M)->Quot{assert!(a.r==a.c);let n=a.r;let el=eliminate(a);let rank=el.image.c;let mut span:Vec<Option<V>>=vec![None;n];for v in &el.image.v{span[v.top().unwrap()]=Some(v.clone());}let mut g=vec![];
 for j in 0..n{let original=V::e(n,j);let mut v=original.clone();while let Some(p)=v.top(){if let Some(b)=&span[p]{v.xor(b);}else{span[p]=Some(v);g.push(original);break;}}}
 let mut cols=el.image.v.clone();cols.extend(g.clone());let j=M::cols(n,cols);assert!(j.c==n);let inv=j.inverse();assert!(j.mul(&inv)==M::id(n)&&inv.mul(&j)==M::id(n));
 let e=inv.rowpart(rank,n-rank);let g=M::cols(n,g);let h=el.pre.mul(&inv.rowpart(0,rank));assert!(e.mul(a).zero()&&e.mul(&g)==M::id(n-rank)&&a.mul(&h).add(&g.mul(&e))==M::id(n));Quot{e,g,h,rank}}
fn poly(s:&str,n:usize)->V{let mut out=V::z(n);if s=="0"{return out;}for term in s.split('+'){let i=if term=="1"{0}else{let rest=term.strip_prefix('u').or_else(||term.strip_prefix('w')).expect("polynomial symbol");if rest.is_empty(){1}else{rest.strip_prefix('^').unwrap_or(rest).parse().unwrap()}};assert!(!out.get(i));out.set(i);}out}
fn data(path:&str)->BTreeMap<String,M>{let mut groups:BTreeMap<String,Vec<(usize,V)>>=BTreeMap::new();for line in fs::read_to_string(path).unwrap().lines(){let t=line.split_whitespace().collect::<Vec<_>>();if t[0]=="FORMAT"{assert!(t==["FORMAT","DATA59v1"]);continue;}assert!(t.len()==3);let n=match t[0]{"SIG"|"CTRL"=>100,"EOLD"=>80,"CRT"|"NORM"|"NORMALL"|"RHO"=>320,_=>640};groups.entry(t[0].into()).or_default().push((t[1].parse().unwrap(),poly(t[2],n)));}
 let mut out=BTreeMap::new();for(name,mut rows)in groups{rows.sort_by_key(|x|x.0);assert!(rows.iter().enumerate().all(|(i,x)|x.0==i+1));let n=rows[0].1.n;out.insert(name,M::cols(n,rows.into_iter().map(|x|x.1).collect()));}out}
fn store(out:&mut BTreeMap<String,M>,name:&str,m:M){assert!(out.insert(name.into(),m).is_none());}
fn save(path:&str,mats:&BTreeMap<String,M>){let mut out=BufWriter::new(fs::File::create(path).unwrap());writeln!(out,"CERT59v1").unwrap();for(name,m)in mats{writeln!(out,"M {name} {} {}",m.r,m.c).unwrap();for v in &m.v{writeln!(out,"{}",v.hex()).unwrap();}}out.flush().unwrap();}
fn read_cert(path:&str)->BTreeMap<String,M>{let text=fs::read_to_string(path).unwrap();let mut it=text.lines();assert!(it.next()==Some("CERT59v1"));let mut out=BTreeMap::new();while let Some(head)=it.next(){let t=head.split_whitespace().collect::<Vec<_>>();assert!(t.len()==4&&t[0]=="M");let r=t[2].parse().unwrap();let c=t[3].parse().unwrap();let v=(0..c).map(|_|V::fromhex(r,it.next().unwrap())).collect();store(&mut out,t[1],M::cols(r,v));}out}
fn polys(m:&M,ps:&[V])->Vec<M>{let max=ps.iter().filter_map(V::top).max().unwrap_or(0);let mut p=M::id(m.r);let mut out=vec![M::z(m.r,m.r);ps.len()];for k in 0..=max{for(i,q)in ps.iter().enumerate(){if q.get(k){out[i]=out[i].add(&p);}}if k<max{p=m.mul(&p);}}out}
fn polys_dot(m:&M,ps:&[V])->Vec<M>{let max=ps.iter().filter_map(V::top).max().unwrap_or(0);let mut p=M::id(m.r);let mut out=vec![M::z(m.r,m.r);ps.len()];for k in 0..=max{for(i,q)in ps.iter().enumerate(){if q.get(k){out[i]=out[i].add(&p);}}if k<max{p=dotmul(m,&p);}}out}
fn constraints(f:&M,crt:&[M],e2:&M,o:usize,t:usize)->M{let mut blocks=vec![];for i in 0..4{if t>>i&1!=0{blocks.push(crt[i].mul(f));}}for i in 0..4{if(o|t)>>i&1==0{blocks.push(e2.mul(&crt[i]).mul(f));}}stack(&blocks)}
fn stack(blocks:&[M])->M{let r=blocks.iter().map(|m|m.r).sum();let c=blocks[0].c;assert!(blocks.iter().all(|m|m.c==c));let mut out=M::z(r,c);let mut off=0;for m in blocks{for j in 0..c{for i in m.v[j].ones(){out.v[j].set(off+i);}}off+=m.r;}out}
fn masks()->Vec<(usize,usize,usize)>{let mut out=vec![];for o in 0..4{for t in 0..4{if o!=t{out.push((1<<o,1<<t,20));}}}for o in 1usize..16{if o.count_ones()==2{out.push((o,15^o,40));}}assert!(out.len()==18);out}
fn write_split(out:&mut BTreeMap<String,M>,name:&str,c:M,s:Split){store(out,&format!("{name}_C"),c);store(out,&format!("{name}_K"),s.k);store(out,&format!("{name}_R"),s.r);store(out,&format!("{name}_H"),s.h);}
fn produce(d:&BTreeMap<String,M>,cert:&str,cohort:&str){assert!(d["MOD"].v[0]==poly("u^20+u^8+1",640));let mut out=BTreeMap::new();let a=&d["A"];let a2=a.mul(a);let a4=a2.mul(&a2);assert!(a4==d["A4"]&&a.r==640&&a.c==640);let q=quotient(&a4);assert!(q.rank==320&&q.e.r==320);
 for(name,m)in[("E",q.e.clone()),("G",q.g.clone()),("H",q.h.clone())]{store(&mut out,name,m);}
 let mut ops=BTreeMap::new();for tag in["A","NU","B","Z","L","V"]{let m=q.e.mul(&d[tag]).mul(&q.g);assert!(q.e.mul(&d[tag])==m.mul(&q.e));store(&mut out,&format!("q{tag}"),m.clone());ops.insert(tag,m);}
 let qa=&ops["A"];let qa2=qa.mul(qa);let qa3=qa2.mul(qa);assert!(qa3.mul(qa).zero());let b_inv=ops["B"].inverse();let w=ops["NU"].mul(&b_inv);assert!(w.mul(&ops["B"])==ops["NU"]&&ops["B"].mul(&w)==ops["NU"]);store(&mut out,"BINV",b_inv);store(&mut out,"W",w.clone());
 let mut ps=d["CRT"].v.clone();ps.push(d["NORMALL"].v[0].clone());ps.push(d["RHO"].v[0].clone());let vals=polys(&w,&ps);let crt=vals[..4].to_vec();let normsq=vals[4].mul(&vals[4]);assert!(normsq.mul(&normsq).zero());store(&mut out,"RHOOP",vals[5].clone());
 let mut sum=M::z(320,320);for i in 0..4{sum=sum.add(&crt[i]);store(&mut out,&format!("P{i}"),crt[i].clone());for j in 0..4{assert!(crt[i].mul(&crt[j])==if i==j{crt[i].clone()}else{M::z(320,320)});}}assert!(sum==M::id(320));
 // Point-labelled length-four Jordan chains give both CRT dimensions and nilpotent ranks.
 let mut chains=vec![];for i in 0..4{let e=eliminate(&qa3.mul(&crt[i]));assert!(e.image.c==20);for v in &e.pre.v{let x=crt[i].mv(v);chains.push(qa3.mv(&x));chains.push(qa2.mv(&x));chains.push(qa.mv(&x));chains.push(x);}}
 let jordan=M::cols(320,chains);let jinv=jordan.inverse();let mut j4=M::z(320,320);for b in 0..80{for k in 1..4{j4.v[4*b+k].set(4*b+k-1);}}assert!(qa.mul(&jordan)==jordan.mul(&j4));store(&mut out,"J",jordan.clone());store(&mut out,"JI",jinv);store(&mut out,"J4",j4);
 for i in 0..4{let mut select=M::z(320,320);for k in 80*i..80*(i+1){select.v[k].set(k);}assert!(crt[i].mul(&jordan)==jordan.mul(&select));}
 let q0=quotient(qa);assert!(q0.e.r==80);let w0=q0.e.mul(&w).mul(&q0.g);let one=q.e.mv(&d["ONE"].v[0]);let mut v=q0.e.mv(&one);let mut krylov=vec![];for _ in 0..80{krylov.push(v.clone());v=w0.mv(&v);}let krylov=M::cols(80,krylov);let kinv=krylov.inverse();let old=kinv.mul(&q0.e).mul(&q.e);assert!(M::cols(80,old.v[..160].to_vec())==d["EOLD"]);assert!(q0.e.mul(&ops["Z"].add(&vals[5])).zero());store(&mut out,"E0",q0.e);store(&mut out,"G0",q0.g);store(&mut out,"H0",q0.h);store(&mut out,"KRYLOV",krylov);store(&mut out,"KRYLOVI",kinv);store(&mut out,"OLD",old);
 let q2=quotient(&qa2);assert!(q2.e.r==160);store(&mut out,"E2",q2.e.clone());store(&mut out,"G2",q2.g);store(&mut out,"H2",q2.h);let f=q.e.mul(&d["F"]);assert!(f.c==100);let finv=left_inverse(&f);assert!(f.mv(&d["CTRL"].v[0])==one&&f.mv(&d["CTRL"].v[1])==qa2.mv(&one));store(&mut out,"F",f.clone());store(&mut out,"FI",finv);
 let reg=stack(&(0..4).map(|i|q2.e.mul(&crt[i]).mul(&f)).collect::<Vec<_>>());let rs=split_kernel(&reg);assert!(rs.rank==99&&rs.k.c==1&&reg.mv(&d["CTRL"].v[1]).zero()&&!reg.mv(&d["CTRL"].v[0]).zero());write_split(&mut out,"regular",reg,rs);
 let mut rows=String::from("id\tdegree\tO_mask\tT_mask\trank\tkernel_dimension\tstatus\n");let mut survivors=0;let mut geometry_bad=false;
 for(id,(o,t,degree))in masks().into_iter().enumerate(){let c=constraints(&f,&crt,&q2.e,o,t);assert!(c.r==640&&c.c==100);let s=split_kernel(&c);if s.k.c>1{geometry_bad=true;}if s.k.c==1{assert!(d["SIG"].mv(&s.k.v[0])==s.k.v[0]);survivors+=1;}
  rows+=&format!("{}\t{degree}\t{o}\t{t}\t{}\t{}\t{}\n",id+1,s.rank,s.k.c,if s.k.c==0{"excluded"}else if s.k.c==1{"principal"}else{"geometry_audit"});println!("COHORT59_PATTERN id={} degree={degree} O={o} T={t} rank={} dimension={}",id+1,s.rank,s.k.c);write_split(&mut out,&format!("case{}",id+1),c,s);}
 fs::write(cohort,rows).unwrap();save(cert,&out);assert!(!geometry_bad,"pattern dimension exceeds proper degree-zero bound; audit");
 println!("COHORT59_RESULT presentation=640 quotient=320 point_blocks=4 point_block_dimension=80 nilpotent_ranks=320,240,160,80,0 old_evaluation_columns=160 rr_evaluation_rank=100 positive_regular_rank=99 patterns=18 principal_patterns={survivors} excluded_patterns={} complete_kernel_certificates=18",18-survivors);println!("COHORT59_ALL_CHECKS_PASS");}
fn check(d:&BTreeMap<String,M>,c:&BTreeMap<String,M>){let e=&c["E"];let g=&c["G"];let h=&c["H"];let a2=dotmul(&d["A"],&d["A"]);product(&a2,&a2,&d["A4"]);product(e,&d["A4"],&M::z(320,640));product(e,g,&M::id(320));let mh=dotmul(&d["A4"],h);assert!(mh.add(&dotmul(g,e))==M::id(640));let mut bad=e.clone();let i=g.v[0].ones()[0];bad.v[i].w[0]^=1;assert!(dotmul(&bad,g)!=M::id(320));
 for tag in["A","NU","B","Z","L","V"]{product(e,&d[tag],&dotmul(&c[&format!("q{tag}")],e));product(&dotmul(e,&d[tag]),g,&c[&format!("q{tag}")]);}
 product(&c["qB"],&c["BINV"],&M::id(320));product(&c["BINV"],&c["qB"],&M::id(320));product(&c["W"],&c["qB"],&c["qNU"]);product(&c["J"],&c["JI"],&M::id(320));product(&c["JI"],&c["J"],&M::id(320));product(&c["qA"],&c["J"],&dotmul(&c["J"],&c["J4"]));
 for i in 0..4{let mut select=M::z(320,320);for k in 80*i..80*(i+1){select.v[k].set(k);}product(&c[&format!("P{i}")],&c["J"],&dotmul(&c["J"],&select));}
 product(&c["E0"],&c["qA"],&M::z(80,320));product(&c["E0"],&c["G0"],&M::id(80));assert!(dotmul(&c["qA"],&c["H0"]).add(&dotmul(&c["G0"],&c["E0"]))==M::id(320));product(&c["KRYLOV"],&c["KRYLOVI"],&M::id(80));product(&c["KRYLOVI"],&dotmul(&c["E0"],e),&c["OLD"]);assert!(M::cols(80,c["OLD"].v[..160].to_vec())==d["EOLD"]);
 let qa2=dotmul(&c["qA"],&c["qA"]);product(&c["E2"],&qa2,&M::z(160,320));product(&c["E2"],&c["G2"],&M::id(160));assert!(dotmul(&qa2,&c["H2"]).add(&dotmul(&c["G2"],&c["E2"]))==M::id(320));product(e,&d["F"],&c["F"]);product(&c["FI"],&c["F"],&M::id(100));
 let mut source_polys=d["CRT"].v.clone();source_polys.push(d["NORMALL"].v[0].clone());source_polys.push(d["RHO"].v[0].clone());let checked_polys=polys_dot(&c["W"],&source_polys);
 for i in 0..4{assert!(checked_polys[i]==c[&format!("P{i}")]);}assert!(checked_polys[5]==c["RHOOP"]);let nsq=dotmul(&checked_polys[4],&checked_polys[4]);assert!(dotmul(&nsq,&nsq).zero());
 let z2=dotmul(&c["qZ"],&c["qZ"]);let z3=dotmul(&z2,&c["qZ"]);let base=dotmul(&c["qV"],&c["qV"]).add(&dotmul(&z2,&c["qV"])).add(&c["qZ"]).add(&z3);assert!(base.zero());
 let rr=dotmul(&M::id(320).add(&c["qZ"]),&c["qV"]).add(&z3);assert!(M::id(320).add(&dotmul(&c["qZ"],&rr.add(&M::id(320))))==c["qA"]);product(&dotmul(&c["qZ"],&rr),&rr.add(&M::id(320)),&c["qB"]);
 let crt=(0..4).map(|i|c[&format!("P{i}")].clone()).collect::<Vec<_>>();let reg=stack(&(0..4).map(|i|dotmul(&dotmul(&c["E2"],&crt[i]),&c["F"])).collect::<Vec<_>>());assert!(reg==c["regular_C"]);
 let mut cases=vec![("regular".to_string(),reg)];for(i,(o,t,_))in masks().into_iter().enumerate(){cases.push((format!("case{}",i+1),constraints(&c["F"],&crt,&c["E2"],o,t)));}
 for(name,m)in cases{assert!(m==c[&format!("{name}_C")]);let k=&c[&format!("{name}_K")];let r=&c[&format!("{name}_R")];let h=&c[&format!("{name}_H")];product(&m,k,&M::z(m.r,k.c));product(r,k,&M::id(k.c));assert!(dotmul(k,r).add(&dotmul(h,&m))==M::id(100));}
 println!("CHECKCOHORT59_RESULT presentation=640 quotient=320 complete_patterns=18 source_columns=100 old_columns=160 independent_row_dot_products=1 changed_cokernel_rejected=1");println!("CHECKCOHORT59_ALL_CHECKS_PASS");}
fn main(){let args=env::args().collect::<Vec<_>>();if args.get(1).map(String::as_str)==Some("--check"){assert!(args.len()==4);check(&data(&args[2]),&read_cert(&args[3]));}else{assert!(args.len()==4);produce(&data(&args[1]),&args[2],&args[3]);}}
