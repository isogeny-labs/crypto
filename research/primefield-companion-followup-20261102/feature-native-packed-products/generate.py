from pathlib import Path
import json,hashlib
p=Path(__file__).resolve().parent;src=p.parent/'feature-native-interval-products';pins={};outs=[]
for f in sorted(src.iterdir()):
 if f.suffix!='.rs' and f.name!='validate-recovery.py':continue
 t=f.read_text();pins[str(f)]=hashlib.sha256(f.read_bytes()).hexdigest()
 if f.name=='interval.rs':
  t=(p/'packed-helper.rs').read_text()+'\n'+t[t.index('fn interval_checks(){'):]
  t+='\ninclude!("packed-checks.rs");\n'
 if f.name=='streamed.rs':
  old='let num=products.product(h-q+1,q)?;let den=products.product(1,q)?;'
  assert old in t;t=t.replace(old,'let initial=products.products(&[(h-q+1,q),(1,q)])?;let num=initial[0];let den=initial[1];')
  old='for _ in 0..steps{let mut num=self.products.product(index-self.m+1,self.m)?;let den=self.products.product(self.h-index+1,self.m)?;'
  new='let mut requests=Vec::with_capacity(2*steps);for _ in 0..steps{requests.push((index-self.m+1,self.m));requests.push((self.h-index+1,self.m));index-=self.m;}let values=self.products.products(&requests)?;\n  for pair in values.chunks_exact(2){let mut num=pair[0];let den=pair[1];'
  assert old in t;t=t.replace(old,new).replace('prefix.push(mul(*prefix.last().unwrap(),den,self.r));index-=self.m;','prefix.push(mul(*prefix.last().unwrap(),den,self.r));')
 if f.name=='new-recovery.rs':t=t.replace('fn main(){','fn main(){packed_checks();',1)
 (p/f.name).write_text(t);outs.append(f.name)
pins[str(p/'packed-helper.rs')]=hashlib.sha256((p/'packed-helper.rs').read_bytes()).hexdigest()
(p/'generation.json').write_text(json.dumps({'input_hashes':pins,'output_hashes':{n:hashlib.sha256((p/n).read_bytes()).hexdigest() for n in outs}},indent=2)+'\n')
