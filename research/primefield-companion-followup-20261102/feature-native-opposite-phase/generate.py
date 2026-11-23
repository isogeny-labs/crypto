from pathlib import Path
import hashlib,json
p=Path(__file__).resolve().parent;src=p.parent/'feature-native-direct-phase';pins={};outs=[]
for f in sorted(src.iterdir()):
 if f.suffix!='.rs' and f.name!='validate-recovery.py':continue
 t=f.read_text();pins[str(f)]=hashlib.sha256(f.read_bytes()).hexdigest()
 if f.name=='direct.rs':t+='\ninclude!("opposite.rs");\n'
 if f.name=='new-recovery.rs':t=t.replace('fn main(){','fn main(){opposite_checks();',1)
 (p/f.name).write_text(t);outs.append(f.name)
t=(src/'direct-adapter.rs').read_text().replace('recover_direct','recover_opposite').replace('DIRECT_COUNTS','OPPOSITE_COUNTS')
t=t.replace('  let mut compressed:Option<DirectImage>=None;',"  if self.m%2==0 || ((self.r-1)/(4*self.m as u64))%2!=0 || radii.len()<2 || radii[0]+radii[1]!=self.r {return self.recover_direct(source,curve,base,target,radii,cap,extra,max_calls)}\n  let mut first:Option<(u64,Vec<u64>)>=None;\n  let mut compressed:Option<DirectImage>=None;")
t=t.replace('else{compressed=Some(DirectImage::new(self,b,&f,i)?);}', 'else if let Some((b0,f0))=first.take(){compressed=Some(DirectImage::opposite(self,b0,&f0,&f,i)?);}else{first=Some((b,f));continue;}')
t=t.replace('image.update(self,b,&f)?;', 'image.update(self,b,&f)?;OPPOSITE_COUNTS.with(|c|{let(a,b,d,e)=c.get();c.set((a,b+1,d,e))});')
(p/'opposite-adapter.rs').write_text(t);outs.append('opposite-adapter.rs')
(p/'generation.json').write_text(json.dumps({'input_hashes':pins,'output_hashes':{n:hashlib.sha256((p/n).read_bytes()).hexdigest() for n in outs}},indent=2)+'\n')
