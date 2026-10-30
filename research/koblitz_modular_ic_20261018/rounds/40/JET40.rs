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
  for i in 0..self.n{if a[i]==[0,0]{continue;}for j in 0..self.n-i{if b[j]!=[0,0]{c[i+j]=qadd(c[i+j],self.f.qmul(a[i],b[j]));}}}c
 }
 fn pow(&self,a:&Series,mut e:usize)->Series{let mut b=a.clone();let mut c=self.c([1,0]);while e>0{if e&1!=0{c=self.mul(&c,&b);}e>>=1;if e>0{b=self.mul(&b,&b);}}c}
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
 let full=Jet{f,n:17};let [b,c,l,n]=p;let u=f.div(b^c,f.mul(c,c));assert_ne!(u,0);assert_ne!(u,1);
 let v=f.div(f.mul(f.div(b,c),f.pow(u,3))^1,u^1);assert_eq!(f.mul(v,v)^v,f.pow(u,3)^u);
 let mut uj=full.c([u,0]);uj[1]=[1,0];
 let rhs=full.add(&full.add(&full.pow(&uj,3),&uj),&full.c([f.pow(u,3)^u,0]));
 let mut vj=full.c([v,0]);let mut term=rhs.clone();for _ in 0..5{vj=full.add(&vj,&term);term=full.mul(&term,&term);}assert_eq!(term,full.c([0,0]));
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
 while precision<17{
  let next=(2*precision).min(17);let j=Jet{f,n:next};let bb=j.truncate(&bj);let cc=j.truncate(&cj);
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
 let mut corrupt=nj.clone();corrupt[16][0]^=1;
 assert!(full.eval(&polys["S"],&physical(&full,&bj,&cj,&lj,&corrupt)).iter().any(|a|*a!=full.c([0,0])));
 [uj,vj,bj,cj,lj,nj]
}
fn conjugate(f:&Field,a:&Series)->Series{a.iter().map(|x|[f.mul(x[0],x[0]),f.mul(x[1],x[1])]).collect()}
fn function_spaces(f:&Field,coords:&[Series;6],old:&[Poly],new:&[Poly],p2:&[(usize,usize)],p3:&[(usize,usize)])->Vec<Vec<Series>>{
 let j=Jet{f,n:17};let [u,v,b,c,l,n]=coords;
 let phys=physical(&j,b,c,l,n);let old_images=[phys[0].clone(),phys[1].clone(),v.clone(),u.clone()];
 let conjugate_t=|a:&Series|a.iter().map(|x|[x[0]^x[1],x[1]]).collect::<Series>();
 let flipped=[conjugate_t(&phys[0]),conjugate_t(&phys[1]),v.clone(),u.clone()];
 let ov=j.eval(old,&old_images);let oc=j.eval(old,&flipped);let nv=j.eval(new,&[n.clone(),l.clone(),v.clone(),u.clone()]);
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
 let j=Jet{f,n:17};let mut c=coords.clone();let mut original_units=vec![];let mut original_inverses=vec![];
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
fn bits(spaces:&[Vec<Vec<Series>>],group:usize,column:usize)->Vec<u64>{
 let mut out=vec![0u64;(spaces.len()*17*7+63)/64];
 for (point,space) in spaces.iter().enumerate(){for k in 0..17{let x=space[group][column][k];assert_eq!(x[1],0);for b in 0..7{if (x[0]>>b)&1!=0{let pos=(point*17+k)*7+b;out[pos/64]^=1u64<<(pos%64);}}}}out
}
fn xor(a:&mut [u64],b:&[u64]){for (x,y) in a.iter_mut().zip(b){*x^=*y;}}
fn highest(a:&[u64])->Option<usize>{a.iter().enumerate().rev().find(|(_,x)|**x!=0).map(|(i,x)|i*64+63-x.leading_zeros() as usize)}
fn rank_checked(columns:&[Vec<u64>],dimension:usize)->usize{
 let mut piv:Vec<Option<(Vec<u64>,Vec<u64>)>>=vec![None;dimension];let mut rank=0;
 for (k,c) in columns.iter().enumerate(){let mut x=c.clone();let mut label=vec![0u64;(columns.len()+63)/64];label[k/64]|=1u64<<(k%64);
  loop{let Some(p)=highest(&x) else{break};assert!(p<dimension);
   if let Some((y,l))=&piv[p]{xor(&mut x,y);xor(&mut label,l);}else{piv[p]=Some((x,label));rank+=1;break;}
  }
 }
 for entry in piv.into_iter().flatten(){let (x,l)=entry;let mut rebuilt=vec![0u64;x.len()];for (k,c) in columns.iter().enumerate(){if (l[k/64]>>(k%64))&1!=0{xor(&mut rebuilt,c);}}assert_eq!(rebuilt,x);}
 rank
}
fn main(){
 let a:Vec<_>=env::args().collect();assert_eq!(a.len(),10);
 let polys=load(&a[1]);let old=load_sections(&a[2],"O");let new=load_sections(&a[3],"W");
 let p2=pairs(&a[4],196,68,68);let p3=pairs(&a[5],324,68,196);
 let f5=Field::new(5,0x25);let control=lifted(&f5,&polys,[3,6,8,25]);full_checks(&f5,&polys,&control);
 let control_spaces=function_spaces(&f5,&control,&old,&new,&p2,&p3);
 let golden=fs::read_to_string(&a[7]).unwrap();let mut controls=0;
 for line in golden.lines(){let s:Vec<_>=line.split_whitespace().collect();assert_eq!(s.len(),3);let g:usize=s[0].parse().unwrap();let i:usize=s[1].parse().unwrap();let words:Vec<u8>=s[2].split(',').map(|x|x.parse().unwrap()).collect();assert_eq!(words.len(),154);
  for k in 0..17{assert_eq!(control_spaces[g-1][i-1][k],[words[k],0]);}controls+=1;
 }assert_eq!(controls,588);println!("JET40_CONTROL F32_reference_sections=588 precision=17 coordinates_from_independent_solver=true corrupt_top_nu=rejected");
 let f=Field::new(7,0x83);let text=fs::read_to_string(&a[6]).unwrap();fs::create_dir(&a[8]).unwrap();
 let mut spaces=vec![];let mut seen=BTreeSet::new();
 for line in text.lines(){let w:Vec<usize>=line.split_whitespace().map(|s|s.parse().unwrap()).collect();assert_eq!(w.len(),6);let p=[w[2] as u8,w[3] as u8,w[4] as u8,w[5] as u8];assert!(seen.insert(p));assert_eq!(canonical(&f,p),p);
  let c=lifted(&f,&polys,p);let (units,inverses)=full_checks(&f,&polys,&c);
  let sp=function_spaces(&f,&c,&old,&new,&p2,&p3);
  let mut out=BufWriter::new(fs::File::create(format!("{}/o{}-d{}.txt",a[8],w[0],w[1])).unwrap());
  let blocks=vec![c.to_vec(),units,inverses,sp[0].clone(),sp[1].clone(),sp[2].clone()];
  for (g,block) in blocks.iter().enumerate(){for (i,s) in block.iter().enumerate(){write!(out,"{} {}",g,i+1).unwrap();for x in s{write!(out," {},{}",x[0],x[1]).unwrap();}writeln!(out).unwrap();}}out.flush().unwrap();
  println!("JET40_POINT orbit={} deck={} precision=17 original_source=pass unit_inverses=8 coefficient_conjugates=7 descent_sections=68 products=196,324 corrupt_top_nu=rejected",w[0],w[1]);spaces.push(sp);
 }
 assert_eq!(spaces.len(),60);let mut ranks=vec![];let mut out=BufWriter::new(fs::File::create(&a[9]).unwrap());
 for (g,count) in [68,196,324].into_iter().enumerate(){let cols:Vec<_>=(0..count).map(|i|bits(&spaces,g,i)).collect();let r=rank_checked(&cols,7140);assert_eq!(r,count);ranks.push(r);writeln!(out,"{} {} {}",g+1,count,r).unwrap();}
 out.flush().unwrap();println!("JET40_RESULT closed_points=60 precision=17 residue_degree=7 evaluation_divisor_degree=7140 all_source_equations=5 unit_inverses_per_point=8 coefficient_conjugates=7 section_ranks={},{},{} all_pivot_recombinations=checked F32_precision17_reference_rows=588 complete_degree460_divisor_evaluated=false new_Picard_classes_evaluated=false",ranks[0],ranks[1],ranks[2]);println!("JET40_ALL_CHECKS_PASS");
}
