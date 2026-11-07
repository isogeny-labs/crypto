from pathlib import Path
import json,hashlib
p=Path(__file__).resolve().parent;src=p.parent/'feature-native-opposite-phase';pins={};outs=[]
for f in sorted(src.iterdir()):
 if (f.suffix!='.rs' and f.name!='validate-recovery.py') or f.name.startswith('validation01-'):continue
 t=f.read_text();pins[str(f)]=hashlib.sha256(f.read_bytes()).hexdigest()
 if f.name=='opposite.rs':t+='\ninclude!("doubled.rs");\n'
 if f.name=='new-recovery.rs':t=t.replace('fn main(){','fn main(){doubled_checks();',1)
 (p/f.name).write_text(t);outs.append(f.name)
t=(src/'direct-adapter.rs').read_text().replace('recover_direct','recover_doubled')
t=t.replace('  let mut compressed:Option<DirectImage>=None;', '  let mut first:Option<(u64,Vec<u64>)>=None;\n  let mut compressed:Option<DirectImage>=None;')
t=t.replace('   if let Some(image)=&mut compressed{image.update(self,b,&f)?;}', '   if let Some((b0,ref f0))=first {if b+b0==r && self.m%2==1 && (r-1)%(8*self.m as u64)==0 {if let Some(k)=self.doubled_phase(b0,f0,&f,i,cap)? {return finish(k,stats,"doubled-phase")}}}\n   first=None;\n   if let Some(image)=&mut compressed{image.update(self,b,&f)?;}')
t=t.replace('else{compressed=Some(DirectImage::new(self,b,&f,i)?);}', 'else{compressed=Some(DirectImage::new(self,b,&f,i)?);first=Some((b,f.clone()));}')
(p/'doubled-adapter.rs').write_text(t);outs.append('doubled-adapter.rs')
(p/'generation.json').write_text(json.dumps({'input_hashes':pins,'output_hashes':{n:hashlib.sha256((p/n).read_bytes()).hexdigest() for n in outs}},indent=2)+'\n')
