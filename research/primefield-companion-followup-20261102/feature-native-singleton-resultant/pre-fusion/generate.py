from pathlib import Path
import hashlib,json
p=Path(__file__).resolve().parent;src=p.parent/'feature-native-source-accounting';pins={};outs=[]
for f in sorted(src.iterdir()):
 if f.suffix!='.rs' and f.name not in ['validate.py','validate-recovery.py','generate-recovery.py']:continue
 t=f.read_text();pins[str(f)]=hashlib.sha256(f.read_bytes()).hexdigest()
 if f.name=='polynomial.rs':
  t=t.replace('impl Poly{','''#[cfg(resultant_audit)]std::thread_local!{static AUDIT:std::cell::Cell<[usize;3]>=const{std::cell::Cell::new([0;3])};}
#[cfg(resultant_audit)]pub fn reset_audit(){AUDIT.with(|x|x.set([0;3]));}
#[cfg(resultant_audit)]pub fn audit()->[usize;3]{AUDIT.with(|x|x.get())}
#[cfg(resultant_audit)]fn bump(i:usize){AUDIT.with(|x|{let mut a=x.get();a[i]+=1;x.set(a)});}
impl Poly{''',1)
  t=t.replace('fn fm(&self,a:u64,b:u64)->u64{','fn fm(&self,a:u64,b:u64)->u64{#[cfg(resultant_audit)]bump(0);',1)
  t=t.replace('fn inverse(&self,a:u64)->Result<u64,Error>{','fn inverse(&self,a:u64)->Result<u64,Error>{#[cfg(resultant_audit)]bump(1);',1)
  t=t.replace('pub fn root_product(&mut self,roots:&[u64])->Result<Vec<u64>,Error>{','pub fn root_product(&mut self,roots:&[u64])->Result<Vec<u64>,Error>{#[cfg(resultant_audit)]bump(2);',1)
  old='pub fn root_resultant_class(&mut self,roots:&[u64],points:&[u64])->Result<ResultantClass,Error>{let f=self.root_product(roots)?;let g=self.root_product(points)?;self.resultant_class(&g,&f)}'
  new='''/// Res(product(X-point), product(X-root)); stream singleton factors.
 pub fn root_resultant_class(&mut self,roots:&[u64],points:&[u64])->Result<ResultantClass,Error>{
  if roots.is_empty()||points.is_empty(){return Err(Error::EmptyTree);}
  if points.len()==1{
   let y=points[0]%self.p;let mut v=(y+self.p-roots[0]%self.p)%self.p;
   for &x in &roots[1..]{v=self.fm(v,(y+self.p-x%self.p)%self.p);}
   return self.image_class(v);
  }
  if roots.len()==1{
   let x=roots[0]%self.p;let mut v=(points[0]%self.p+self.p-x)%self.p;
   for &y in &points[1..]{v=self.fm(v,(y%self.p+self.p-x)%self.p);}
   return self.image_class(v);
  }
  let f=self.root_product(roots)?;let g=self.root_product(points)?;self.resultant_class(&g,&f)
 }'''
  assert old in t;t=t.replace(old,new)
  old='''  let f=self.root_product(roots)?;let g=self.root_product(points)?;
  // Existing source convention is product_j f(points[j]) = Res(g,f).
  self.projected_resultant(&g,&f)'''
  assert old in t;t=t.replace(old,'  Ok(self.root_resultant_class(roots,points)?.project())')
 if f.name=='check.rs':
  t=t.replace('fn main(){schedule_checks();','include!("singleton-checks.rs");\nfn main(){singleton_checks();schedule_checks();')
 if f.name=='validate.py':t=t.replace("'--cfg','source_audit'","'--cfg','source_audit','--cfg','resultant_audit'")
 (p/f.name).write_text(t);outs.append(f.name)
(p/'generation.json').write_text(json.dumps({'input_hashes':pins,'output_hashes':{n:hashlib.sha256((p/n).read_bytes()).hexdigest() for n in outs}},indent=2)+'\n')
