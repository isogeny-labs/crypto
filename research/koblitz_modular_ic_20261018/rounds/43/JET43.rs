//! Exact finite-field collector on a fixed ordinary descent chart.
use std::{env,fs,collections::{BTreeMap,BTreeSet},io::{BufWriter,Write}};
type Point=[u8;4];type Quad=[u8;2];type Term=[usize;4];type Poly=Vec<Term>;
struct Field{m:usize,size:usize,table:Vec<Vec<u8>>,inverse:Vec<u8>}
fn product(mut a:u16,mut b:u16,m:usize,modulus:u16)->u8{let mut c=0;for _ in 0..m{if b&1!=0{c^=a;}b>>=1;a<<=1;if a&(1<<m)!=0{a^=modulus;}}c as u8}
fn independent(a:u8,b:u8,m:usize,modulus:u16)->u8{let mut c=0u16;for i in 0..m{for j in 0..m{if (a>>i)&1!=0&&(b>>j)&1!=0{c^=1<<(i+j);}}}for k in (m..=2*m-2).rev(){if c&(1<<k)!=0{c^=modulus<<(k-m);}}c as u8}
impl Field{
 fn new(m:usize,modulus:u16)->Self{let size=1<<m;let mut table=vec![vec![0;size];size];for a in 0..size{for b in 0..size{table[a][b]=product(a as u16,b as u16,m,modulus);assert_eq!(table[a][b],independent(a as u8,b as u8,m,modulus));if a!=0&&b!=0{assert_ne!(table[a][b],0);}}}let mut f=Self{m,size,table,inverse:vec![0;size]};for a in 1..size{f.inverse[a]=f.pow(a as u8,size-2);assert_eq!(f.mul(a as u8,f.inverse[a]),1);}for a in 0..size{for b in 0..size{for c in 0..size{assert_eq!(f.mul(f.mul(a as u8,b as u8),c as u8),f.mul(a as u8,f.mul(b as u8,c as u8)));assert_eq!(f.mul(a as u8,(b^c) as u8),f.mul(a as u8,b as u8)^f.mul(a as u8,c as u8));}}}for a in 0..size{assert_ne!(f.mul(a as u8,a as u8)^a as u8^1,0);}for a in 0..size{for b in 0..size{if a!=0||b!=0{let q=[a as u8,b as u8];assert_eq!(f.qmul(q,f.qinv(q)),[1,0]);}}}f}
 fn mul(&self,a:u8,b:u8)->u8{self.table[a as usize][b as usize]}
 fn pow(&self,mut a:u8,mut n:usize)->u8{let mut v=1;while n>0{if n&1!=0{v=self.mul(v,a);}n>>=1;if n>0{a=self.mul(a,a);}}v}
 fn div(&self,a:u8,b:u8)->u8{assert_ne!(b,0);self.mul(a,self.inverse[b as usize])}
 fn qmul(&self,a:Quad,b:Quad)->Quad{let t=self.mul(a[1],b[1]);[self.mul(a[0],b[0])^t,self.mul(a[0],b[1])^self.mul(a[1],b[0])^t]}
 fn qinv(&self,a:Quad)->Quad{assert_ne!(a,[0,0]);let norm=self.mul(a[0],a[0])^self.mul(a[0],a[1])^self.mul(a[1],a[1]);assert_ne!(norm,0);[self.div(a[0]^a[1],norm),self.div(a[1],norm)]}
 fn square_point(&self,p:Point)->Point{p.map(|x|self.mul(x,x))}
}
fn qadd(a:Quad,b:Quad)->Quad{[a[0]^b[0],a[1]^b[1]]}
fn load(path:&str)->BTreeMap<String,Vec<Poly>>{let text=fs::read_to_string(path).unwrap();let mut lines=text.lines();let mut polys:BTreeMap<String,Vec<Poly>>=BTreeMap::new();while let Some(header)=lines.next(){let p:Vec<_>=header.split_whitespace().collect();assert_eq!(p.len(),3);let idx:usize=p[1].parse().unwrap();let count:usize=p[2].parse().unwrap();let rows=polys.entry(p[0].to_string()).or_default();assert_eq!(idx,rows.len()+1);let mut poly=vec![];for _ in 0..count{let ex:Vec<usize>=lines.next().unwrap().split(',').map(|x|x.parse().unwrap()).collect();assert_eq!(ex.len(),4);poly.push(ex.try_into().unwrap());}rows.push(poly);}for (key,count) in [("S",5),("U",8),("N",1),("C",1),("L",5),("D",5)]{assert_eq!(polys[key].len(),count);}polys}
fn bounds(polys:&BTreeMap<String,Vec<Poly>>)->[usize;4]{let mut b=[0;4];for ps in polys.values(){for p in ps{for e in p{for i in 0..4{b[i]=b[i].max(e[i]);}}}}b}
fn powers(f:&Field,images:[Quad;4],b:[usize;4])->Vec<Vec<Quad>>{let mut rows=vec![];for k in 0..4{let mut v=vec![[1,0]];for i in 0..b[k]{v.push(f.qmul(v[i],images[k]));}rows.push(v);}rows}
fn eval(f:&Field,p:&Poly,powers:&[Vec<Quad>])->Quad{let mut ans=[0,0];for ex in p{let mut term=[1,0];for k in 0..4{term=f.qmul(term,powers[k][ex[k]]);}ans=qadd(ans,term);}ans}
fn images(p:Point)->[Quad;4]{let [b,c,l,n]=p;[[n,b],[l,1^c],[b,0],[c,0]]}
fn valid(f:&Field,polys:&BTreeMap<String,Vec<Poly>>,b:[usize;4],p:Point)->bool{let pw=powers(f,images(p),b);for eq in &polys["S"]{if eval(f,eq,&pw)!=[0,0]{return false;}}for eq in &polys["U"]{let v=eval(f,eq,&pw);if v==[0,0]{return false;}assert_eq!(f.qmul(v,f.qinv(v)),[1,0]);}true}
fn tate_add(f:&Field,p:Option<(u8,u8)>,q:Option<(u8,u8)>,b:u8,a:u8)->Option<(u8,u8)>{let Some((x,y))=p else{return q};let Some((z,w))=q else{return p};let slope=if x==z{if y!=w{assert_eq!(y^w,f.mul(a,x)^b);return None;}let den=f.mul(a,x)^b;if den==0{return None;}f.div(f.mul(x,x)^f.mul(a,y),den)}else{f.div(y^w,x^z)};let rx=f.mul(slope,slope)^f.mul(a,slope)^b^x^z;let ry=f.mul(slope^a,rx)^y^f.mul(slope,x)^b;assert_eq!(f.mul(ry,ry)^f.mul(a,f.mul(rx,ry))^f.mul(b,ry)^f.pow(rx,3)^f.mul(b,f.mul(rx,rx)),0);Some((rx,ry))}
fn torsion_check(f:&Field,p:Point){let [b,c,_,_]=p;let mut q=None;for k in 1..=11{q=tate_add(f,q,Some((0,0)),b,1^c);if k<11{assert!(q.is_some());}else{assert!(q.is_none());}}}
fn deck(f:&Field,p:Point)->Point{let [b,c,l,n]=p;let a=1^c;assert_ne!(b,0);assert_ne!(c,0);let s=f.div(b,c)^a;let v=f.mul(s,a^s);assert_ne!(v,0);let iu=f.div(v,f.mul(b,c));[f.mul(v,f.pow(iu,2)),1^f.mul(a,iu),f.mul(l^s,iu),f.mul(f.mul(l,b)^n^f.mul(b,c),f.pow(iu,3))]}
fn canonical(f:&Field,p:Point)->Point{let mut q=p;let mut least=p;for k in 1..=f.m{q=f.square_point(q);if k<f.m{assert_ne!(q,p);}least=least.min(q);}assert_eq!(q,p);least}
fn jacobian_rank_two(f:&Field,polys:&BTreeMap<String,Vec<Poly>>,b:[usize;4],p:Point)->bool{let pw=powers(f,images(p),b);let mut dl=vec![];let mut dn=vec![];for k in 0..5{dl.extend(eval(f,&polys["L"][k],&pw));dn.extend(eval(f,&polys["D"][k],&pw));}for i in 0..10{for j in i+1..10{if f.mul(dl[i],dn[j])^f.mul(dl[j],dn[i])!=0{return true;}}}false}
type Series=Vec<Quad>;
struct Jet<'a>{f:&'a Field,n:usize}
impl<'a> Jet<'a>{
 fn c(&self,x:Quad)->Series{let mut a=vec![[0,0];self.n];a[0]=x;a}
 fn add(&self,a:&Series,b:&Series)->Series{a.iter().zip(b).map(|(&x,&y)|qadd(x,y)).collect()}
 fn mul(&self,a:&Series,b:&Series)->Series{
  let mut c=vec![[0,0];self.n];
  let aa:Vec<_>=a.iter().enumerate().filter(|(_,x)|**x!=[0,0]).collect();
  let bb:Vec<_>=b.iter().enumerate().filter(|(_,x)|**x!=[0,0]).collect();
  for &(i,x) in &aa{for &(k,y) in &bb{if i+k>=self.n{break;}c[i+k]=qadd(c[i+k],self.f.qmul(*x,*y));}}c
 }
 fn dense(&self,a:&Series,b:&Series)->Series{let mut c=self.c([0,0]);for i in 0..self.n{for k in 0..self.n-i{c[i+k]=qadd(c[i+k],self.f.qmul(a[i],b[k]));}}c}
 fn square(&self,a:&Series)->Series{let mut c=self.c([0,0]);for i in 0..=(self.n-1)/2{c[2*i]=self.f.qmul(a[i],a[i]);}c}
 fn grouped(&self,polys:&[Poly],images:&[Series;4])->Vec<Series>{
  let mut max=[0;4];let mut keys=BTreeSet::new();
  for p in polys{for e in p{for k in 0..4{max[k]=max[k].max(e[k]);}keys.insert([e[0],e[1],e[2]]);}}
  let mut pw=vec![];for k in 0..4{let mut row=vec![self.c([1,0])];for i in 0..max[k]{row.push(self.mul(&row[i],&images[k]));}pw.push(row);}
  let mut prefix=BTreeMap::new();for e in keys{let mut t=self.c([1,0]);for k in 0..3{if e[k]>0{t=self.mul(&t,&pw[k][e[k]]);}}prefix.insert(e,t);}
  let ans:Vec<_>=polys.iter().map(|p|{let mut groups:BTreeMap<[usize;3],Series>=BTreeMap::new();for e in p{let g=groups.entry([e[0],e[1],e[2]]).or_insert_with(||self.c([0,0]));*g=self.add(g,&pw[3][e[3]]);}let mut sum=self.c([0,0]);for(key,v)in groups{sum=self.add(&sum,&self.mul(&prefix[&key],&v));}sum}).collect();
  let j=Jet{f:self.f,n:154};let small=images.clone().map(|a|j.truncate(&a));let check=j.eval(polys,&small);for(a,b)in ans.iter().zip(check){assert_eq!(j.truncate(a),b);}
  ans
 }
 fn pow(&self,a:&Series,mut e:usize)->Series{let mut b=a.clone();let mut c=self.c([1,0]);while e>0{if e&1!=0{c=self.mul(&c,&b);}e>>=1;if e>0{b=self.square(&b);}}c}
 fn inv(&self,a:&Series)->Series{
  assert_ne!(a[0],[0,0]);let mut b=self.c(self.f.qinv(a[0]));
  for k in 1..self.n{let mut r=[0,0];for i in 1..=k{r=qadd(r,self.f.qmul(a[i],b[k-i]));}b[k]=self.f.qmul(b[0],r);}
  assert_eq!(self.mul(a,&b),self.c([1,0]));b
 }
 fn eval(&self,polys:&[Poly],images:&[Series;4])->Vec<Series>{
  let mut max=[0;4];for p in polys{for e in p{for k in 0..4{max[k]=max[k].max(e[k]);}}}
  let mut powers=vec![];for k in 0..4{let mut row=vec![self.c([1,0])];for i in 0..max[k]{row.push(self.mul(&row[i],&images[k]));}powers.push(row);}
  polys.iter().map(|p|{let mut a=self.c([0,0]);for e in p{let mut t=self.c([1,0]);for k in 0..4{if e[k]>0{t=self.mul(&t,&powers[k][e[k]]);}}a=self.add(&a,&t);}a}).collect()
 }
 fn part(&self,a:&Series,k:usize)->Series{a.iter().map(|x|[x[k],0]).collect()}
 fn truncate(&self,a:&Series)->Series{a[..self.n].to_vec()}
}
fn load_sections(path:&str,key:&str)->Vec<Poly>{
 let s=fs::read_to_string(path).unwrap();let mut lines=s.lines();let mut out=vec![];
 while let Some(h)=lines.next(){let a:Vec<_>=h.split_whitespace().collect();assert_eq!(a[0],key);assert_eq!(a[1].parse::<usize>().unwrap(),out.len()+1);let mut p=vec![];
 for _ in 0..a[2].parse::<usize>().unwrap(){let e:Vec<usize>=lines.next().unwrap().split(',').map(|s|s.parse().unwrap()).collect();p.push(e.try_into().unwrap());}out.push(p);}
 assert_eq!(out.len(),68);out
}
fn pairs(path:&str,count:usize,a:usize,b:usize)->Vec<(usize,usize)>{
 let t=fs::read_to_string(path).unwrap();let v:Vec<_>=t.lines().enumerate().map(|(i,s)|{let v:Vec<usize>=s.split_whitespace().map(|s|s.parse().unwrap()).collect();assert_eq!(v.len(),3);assert_eq!(v[0],i+1);assert!(v[1]>0&&v[1]<=a&&v[2]>0&&v[2]<=b);(v[1]-1,v[2]-1)}).collect();assert_eq!(v.len(),count);v
}
fn physical(j:&Jet,b:&Series,c:&Series,l:&Series,n:&Series)->[Series;4]{
 let t=j.c([0,1]);let one=j.c([1,0]);
 [j.add(n,&j.mul(b,&t)),j.add(l,&j.mul(&j.add(&one,c),&t)),b.clone(),c.clone()]
}
fn scalar_component(j:&Jet,values:&[Series],row:usize,offset:usize)->Series{j.part(&values[offset+row/2],row%2)}
fn lifted(f:&Field,polys:&BTreeMap<String,Vec<Poly>>,p:Point)->[Series;6]{
 let full=Jet{f,n:615};let [b,c,l,n]=p;let u=f.div(b^c,f.mul(c,c));assert_ne!(u,0);assert_ne!(u,1);
 let v=f.div(f.mul(f.div(b,c),f.pow(u,3))^1,u^1);assert_eq!(f.mul(v,v)^v,f.pow(u,3)^u);
 let mut uj=full.c([u,0]);uj[1]=[1,0];
 let rhs=full.add(&full.add(&full.pow(&uj,3),&uj),&full.c([f.pow(u,3)^u,0]));
 let mut vj=full.c([v,0]);let mut term=rhs.clone();for _ in 0..10{vj=full.add(&vj,&term);term=full.square(&term);}assert_eq!(term,full.c([0,0]));
 assert_eq!(full.add(&full.add(&full.mul(&vj,&vj),&vj),&full.add(&full.pow(&uj,3),&uj)),full.c([0,0]));
 let rn=full.add(&full.mul(&full.add(&uj,&full.c([1,0])),&vj),&full.c([1,0]));
 let cn=full.add(&rn,&full.pow(&uj,3));
 let bj=full.mul(&full.mul(&rn,&cn),&full.inv(&full.pow(&uj,7)));
 let cj=full.mul(&cn,&full.inv(&full.pow(&uj,4)));
 assert_eq!(bj[0],[b,0]);assert_eq!(cj[0],[c,0]);
 let mut lj=full.c([l,0]);let mut nj=full.c([n,0]);
 let source:Vec<_>=polys["S"].iter().chain(&polys["L"]).chain(&polys["D"]).cloned().collect();
 let tiny=Jet{f,n:1};let initial=tiny.eval(&source,&physical(&tiny,&tiny.truncate(&bj),&tiny.truncate(&cj),&tiny.truncate(&lj),&tiny.truncate(&nj)));
 let mut pair=None;for a in 0..9{for b in a+1..10{
  let aa=scalar_component(&tiny,&initial,a,5)[0][0];let ab=scalar_component(&tiny,&initial,a,10)[0][0];
  let ba=scalar_component(&tiny,&initial,b,5)[0][0];let bb=scalar_component(&tiny,&initial,b,10)[0][0];
  if pair.is_none()&&f.mul(aa,bb)^f.mul(ab,ba)!=0{pair=Some((a,b));}
 }}
 let (a,b)=pair.expect("no invertible source pair");let mut precision=1;
 while precision<615{
  let next=(2*precision).min(615);let j=Jet{f,n:next};let bb=j.truncate(&bj);let cc=j.truncate(&cj);
  let ll=j.truncate(&lj);let nn=j.truncate(&nj);let values=j.eval(&source,&physical(&j,&bb,&cc,&ll,&nn));
  let ea=scalar_component(&j,&values,a,0);let eb=scalar_component(&j,&values,b,0);
  let aa=scalar_component(&j,&values,a,5);let ab=scalar_component(&j,&values,a,10);
  let ba=scalar_component(&j,&values,b,5);let bc=scalar_component(&j,&values,b,10);
  let det=j.add(&j.mul(&aa,&bc),&j.mul(&ab,&ba));let di=j.inv(&det);
  let dl=j.mul(&j.add(&j.mul(&ea,&bc),&j.mul(&eb,&ab)),&di);
  let dn=j.mul(&j.add(&j.mul(&aa,&eb),&j.mul(&ba,&ea)),&di);
  for k in 0..precision{assert_eq!(dl[k],[0,0]);assert_eq!(dn[k],[0,0]);}
  for k in 0..next{lj[k]=qadd(lj[k],dl[k]);nj[k]=qadd(nj[k],dn[k]);}
  for e in j.eval(&polys["S"],&physical(&j,&bb,&cc,&j.truncate(&lj),&j.truncate(&nj))){assert_eq!(e,j.c([0,0]));}
  precision=next;
 }
 let mut corrupt=nj.clone();corrupt[614][0]^=1;
 assert!(full.eval(&polys["S"],&physical(&full,&bj,&cj,&lj,&corrupt)).iter().any(|a|*a!=full.c([0,0])));
 [uj,vj,bj,cj,lj,nj]
}
fn conjugate(f:&Field,a:&Series)->Series{a.iter().map(|x|[f.mul(x[0],x[0]),f.mul(x[1],x[1])]).collect()}
fn function_spaces(f:&Field,coords:&[Series;6],old:&[Poly],new:&[Poly],p2:&[(usize,usize)],p3:&[(usize,usize)])->Vec<Vec<Series>>{
 let j=Jet{f,n:615};let [u,v,b,c,l,n]=coords;
 let phys=physical(&j,b,c,l,n);let old_images=[phys[0].clone(),phys[1].clone(),v.clone(),u.clone()];
 let conjugate_t=|a:&Series|a.iter().map(|x|[x[0]^x[1],x[1]]).collect::<Series>();
 let flipped=[conjugate_t(&phys[0]),conjugate_t(&phys[1]),v.clone(),u.clone()];
 let ov=j.grouped(old,&old_images);let oc=j.grouped(old,&flipped);let nv=j.grouped(new,&[n.clone(),l.clone(),v.clone(),u.clone()]);
 let up=j.add(u,&j.c([1,0]));
 let oi=j.inv(&j.mul(&j.pow(u,16),&j.pow(&up,16)));let ni=j.inv(&j.mul(&j.pow(u,49),&j.pow(&up,16)));
 let mut sections=vec![];
 for i in 0..68{let value=j.mul(&nv[i],&ni);let compare=j.mul(&j.add(&j.mul(&j.c([1,1]),&ov[i]),&j.mul(&j.c([0,1]),&oc[i])),&oi);
  assert_eq!(value,compare);assert!(value.iter().all(|x|x[1]==0));sections.push(value);
 }
 let quad:Vec<_>=p2.iter().map(|&(a,b)|j.mul(&sections[a],&sections[b])).collect();
 let cube:Vec<_>=p3.iter().map(|&(a,b)|j.mul(&sections[a],&quad[b])).collect();
 vec![sections,quad,cube]
}
fn full_checks(f:&Field,polys:&BTreeMap<String,Vec<Poly>>,coords:&[Series;6])->(Vec<Series>,Vec<Series>){
 let j=Jet{f,n:615};let mut c=coords.clone();let mut original_units=vec![];let mut original_inverses=vec![];
 for k in 0..=f.m{
  let images=physical(&j,&c[2],&c[3],&c[4],&c[5]);
  for a in j.eval(&polys["S"],&images){assert_eq!(a,j.c([0,0]));}
  let units=j.eval(&polys["U"],&images);let inverses:Vec<_>=units.iter().map(|a|j.inv(a)).collect();
  if k==0{original_units=units.clone();original_inverses=inverses.clone();}
  let mut expected=original_inverses.clone();for _ in 0..k{expected=expected.iter().map(|a|conjugate(f,a)).collect();}
  assert_eq!(expected,inverses);
  if k<f.m{c=c.map(|a|conjugate(f,&a));}
 }
 assert_eq!(&c,coords);(original_units,original_inverses)
}

fn xor(a:&mut [u64],b:&[u64]){for (x,y) in a.iter_mut().zip(b){*x^=*y;}}
fn highest(a:&[u64])->Option<usize>{a.iter().enumerate().rev().find(|(_,x)|**x!=0).map(|(i,x)|i*64+63-x.leading_zeros() as usize)}
fn rank_checked(columns:&[Vec<u64>],dimension:usize,path:&str)->usize{
 let mut piv:Vec<Option<(Vec<u64>,Vec<u64>)>>=vec![None;dimension];let mut rank=0;
 for (k,c) in columns.iter().enumerate(){let mut x=c.clone();let mut label=vec![0u64;(columns.len()+63)/64];label[k/64]|=1u64<<(k%64);
  loop{let Some(p)=highest(&x) else{break};assert!(p<dimension);
   if let Some((y,l))=&piv[p]{xor(&mut x,y);xor(&mut label,l);}else{piv[p]=Some((x,label));rank+=1;break;}
  }
 }
 let mut out=BufWriter::new(fs::File::create(path).unwrap());
 for (p,entry) in piv.into_iter().enumerate(){let Some((x,l))=entry else{continue};write!(out,"{p} ").unwrap();for v in &x{write!(out,"{v:016x},").unwrap();}write!(out," ").unwrap();for v in &l{write!(out,"{v:016x},").unwrap();}writeln!(out).unwrap();let mut rebuilt=vec![0u64;x.len()];for (k,c) in columns.iter().enumerate(){if (l[k/64]>>(k%64))&1!=0{xor(&mut rebuilt,c);}}assert_eq!(rebuilt,x);}
 out.flush().unwrap();rank
}
fn save(path:&str,tag:&str,rows:&[Series]){let mut out=BufWriter::new(fs::File::create(path).unwrap());for(i,row)in rows.iter().enumerate(){write!(out,"{tag} {} ",i+1).unwrap();for(k,x)in row.iter().enumerate(){if k>0{write!(out,",").unwrap();}assert_eq!(x[1],0);write!(out,"{}",x[0]).unwrap();}writeln!(out).unwrap();}out.flush().unwrap();}
fn reference(path:&str)->BTreeMap<(String,usize),Vec<u8>>{let text=fs::read_to_string(path).unwrap();let mut out=BTreeMap::new();for line in text.lines(){let w:Vec<_>=line.split_whitespace().collect();assert_eq!(w.len(),3);let values:Vec<u8>=w[2].split(',').map(|x|x.parse().unwrap()).collect();assert_eq!(values.len(),154);assert!(values.iter().all(|x|*x<32));assert!(out.insert((w[0].to_string(),w[1].parse().unwrap()),values).is_none());}out}
fn compare(tag:&str,rows:&[Series],old:&BTreeMap<(String,usize),Vec<u8>>){for(i,row)in rows.iter().enumerate(){let value=&old[&(tag.to_string(),i+1)];for k in 0..154{assert_eq!(row[k],[value[k],0],"regression {tag}/{} coefficient{k}",i+1);}}}
fn main(){
 let args:Vec<_>=env::args().collect();assert_eq!(args.len(),9);
 let polys=load(&args[1]);let old=load_sections(&args[2],"O");let new=load_sections(&args[3],"W");let p2=pairs(&args[4],196,68,68);let p3=pairs(&args[5],324,68,196);
 let f=Field::new(5,0x25);
 let test=Jet{f:&f,n:17};let samples:Vec<Series>=(0..8).map(|k|(0..17).map(|i|[((i*i+3*k*i+k)%32)as u8,((i+7*k)%32)as u8]).collect()).collect();
 for a in &samples{assert_eq!(test.square(a),test.dense(a,a));for b in &samples{assert_eq!(test.mul(a,b),test.dense(a,b));}}
 println!("JET43_ARITHMETIC independent_field_products=1024 sparse_vs_dense_convolutions=64 square_vs_dense=8");
 let coords=lifted(&f,&polys,[3,6,8,25]);println!("JET43_COORDINATES precision=615 all_source_stages=pass AS_terms=10 corrupt_top_nu=rejected");
 let(units,inverses)=full_checks(&f,&polys,&coords);println!("JET43_UNITS units=8 inverses=8 coefficient_conjugates=5 closure=pass");
 let j=Jet{f:&f,n:615};for a in coords.iter().chain(&units).chain(&inverses){assert_eq!(j.square(a),j.mul(a,a));}
 let spaces=function_spaces(&f,&coords,&old,&new,&p2,&p3);println!("JET43_FUNCTIONS original_orientations=136 literal_twist=68 every_descent_identity=pass grouped_vs_direct_precision154=204 quadratic_products=196 cubic_products=324");
 let reference_root=reference(&format!("{}/reference-coordinates-units154.txt",args[6]));assert_eq!(reference_root.len(),22);
 compare("C",&coords,&reference_root);compare("U",&units,&reference_root);compare("I",&inverses,&reference_root);
 let reference_spaces=reference(&format!("{}/reference-sections154.txt",args[6]));assert_eq!(reference_spaces.len(),588);
 for(g,space)in spaces.iter().enumerate(){compare(&(g+1).to_string(),space,&reference_spaces);}
 println!("JET43_REGRESSION coordinates=6 units=8 inverses=8 sections=68 quadratics=196 cubics=324 old_precision=154 every_coefficient=pass");
 fs::create_dir(&args[7]).unwrap();save(&format!("{}/coordinates615.txt",args[7]),"C",&coords);save(&format!("{}/units615.txt",args[7]),"U",&units);save(&format!("{}/inverses615.txt",args[7]),"I",&inverses);
 let mut ranks=vec![];
 for(g,space)in spaces.iter().enumerate(){save(&format!("{}/space{}.txt",args[7],g+1),&(g+1).to_string(),space);let columns:Vec<_>=space.iter().map(|a|{let mut bits=vec![0u64;(3075+63)/64];for(k,x)in a.iter().enumerate(){assert_eq!(x[1],0);for b in 0..5{if(x[0]>>b)&1!=0{let pos=5*k+b;bits[pos/64]^=1u64<<(pos%64);}}}bits}).collect();let rank=rank_checked(&columns,3075,&format!("{}/pivots{}.txt",args[7],g+1));assert_eq!(rank,[68,196,324][g]);ranks.push(rank);}
 fs::write(&args[8],format!("precision=615\nresidue_degree=5\nevaluation_dimension=3075\nrank1={}\nrank2={}\nrank3={}\nregression_precision=154\nregression_rows=610\n",ranks[0],ranks[1],ranks[2])).unwrap();
 println!("JET43_RESULT precision=615 residue_degree=5 evaluation_dimension=3075 ranks=68,196,324 all_pivot_recombinations=checked full_reference_rows=610 every_original_source_equation=pass all_unit_inverses=pass coefficient_descent_conjugates=5 complete_larger_RR_spaces_constructed=false geometric_transfer_evaluated=false");println!("JET43_ALL_CHECKS_PASS");
}
