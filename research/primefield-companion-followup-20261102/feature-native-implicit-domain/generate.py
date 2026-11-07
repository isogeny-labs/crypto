from pathlib import Path
import json,hashlib
p=Path(__file__).resolve().parent;src=p.parent/'feature-native-weighted-quotient';pins={};outs=[]
for f in sorted(src.iterdir()):
 if f.suffix!='.rs' and f.name!='validate-recovery.py':continue
 t=f.read_text();pins[str(f)]=hashlib.sha256(f.read_bytes()).hexdigest()
 if f.name=='trace-adapter.rs':
  old='let d=((r-1)/self.m as u64)as usize;let mut state=vec![0;d+2];state[1]=r-1;state[d+1]=1;'
  assert old in t;t=t.replace(old,'let mut state=ImageDomain::new(r,self.m)?;')
  old='state=p.gcd_monic(&state,&constraint,d+1).map_err(|_|Error::Arithmetic)?.polynomial;'
  assert old in t;t=t.replace(old,'state.intersect(&mut p,&constraint)?;')
  t=t.replace('if state.len()==1','if state.degree()==0').replace('if state.len()==2{let image=(r-state[0])%r;','if let Some(image)=state.linear_root(){')
  t=t.replace('pullback(&state,self.m,r,cap)','state.pullback(cap)').replace('stats.image_count=state.len()-1','stats.image_count=state.degree()')
  t='include!("implicit.rs");\n'+t
 if f.name=='new-recovery.rs':t=t.replace('fn main(){previous_main();','fn main(){implicit_checks();previous_main();')
 (p/f.name).write_text(t);outs.append(f.name)
(p/'generation.json').write_text(json.dumps({'input_hashes':pins,'output_hashes':{n:hashlib.sha256((p/n).read_bytes()).hexdigest() for n in outs}},indent=2)+'\n')
