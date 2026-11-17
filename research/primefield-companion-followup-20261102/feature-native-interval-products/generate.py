from pathlib import Path
import json,hashlib
p=Path(__file__).resolve().parent;src=p.parent/'feature-native-streamed-modes';pins={};outs=[]
for f in sorted(src.iterdir()):
 if f.suffix!='.rs' and f.name!='validate-recovery.py':continue
 t=f.read_text();pins[str(f)]=hashlib.sha256(f.read_bytes()).hexdigest()
 if f.name=='streamed.rs':
  t='include!("interval.rs");\n'+t
  t=t.replace('left:usize,cap:usize}','left:usize,cap:usize,products:IntervalProducts}')
  t=t.replace('let q=h-index;','let q=(h-index).min(index);')
  old='let(mut num,mut den)=(1,1);for i in 0..q{num=mul(num,h-i,r);den=mul(den,i+1,r)}'
  assert old in t;t=t.replace(old,'let mut products=IntervalProducts::new(r,m as u64,cap)?;let num=products.product(h-q+1,q)?;let den=products.product(1,q)?;')
  t=t.replace('if q%2!=0','if (h-index)%2!=0').replace('index,value,left,cap})','index,value,left,cap,products})')
  old='let(mut num,mut den)=(1,1);for i in 0..self.m{num=mul(num,index-i,self.r);den=mul(den,self.h-index+i+1,self.r)}'
  assert old in t;t=t.replace(old,'let mut num=self.products.product(index-self.m+1,self.m)?;let den=self.products.product(self.h-index+1,self.m)?;')
 if f.name=='new-recovery.rs':t=t.replace('fn main(){','fn main(){interval_checks();interval_public_checks();',1)
 (p/f.name).write_text(t);outs.append(f.name)
(p/'generation.json').write_text(json.dumps({'input_hashes':pins,'output_hashes':{n:hashlib.sha256((p/n).read_bytes()).hexdigest() for n in outs}},indent=2)+'\n')
