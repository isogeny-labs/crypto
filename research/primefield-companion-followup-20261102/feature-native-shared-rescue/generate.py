from pathlib import Path
import hashlib,json
p=Path(__file__).resolve().parent;src=p.parent/'feature-native-doubled-rescue';pins={};outs=[]
for f in sorted(src.iterdir()):
 if f.suffix!='.rs' and f.name!='validate-recovery.py':continue
 t=f.read_text();pins[str(f)]=hashlib.sha256(f.read_bytes()).hexdigest()
 if f.name=='context.rs':
  t=t.replace('inner:std::cell::RefCell<IntervalProducts>', 'inner:std::rc::Rc<std::cell::RefCell<IntervalProducts>>')
  t=t.replace('inner:std::cell::RefCell::new(IntervalProducts::new(r,m as u64,cap)?)', 'inner:std::rc::Rc::new(std::cell::RefCell::new(IntervalProducts::new(r,m as u64,cap)?))')
 if f.name=='doubled.rs':t+='\ninclude!("shared-rescue.rs");\n'
 if f.name=='new-recovery.rs':t=t.replace('fn main(){','fn main(){shared_rescue_checks();',1)
 (p/f.name).write_text(t);outs.append(f.name)
t=(src/'doubled-adapter.rs').read_text().replace('recover_doubled','recover_shared_rescue').replace('self.doubled_phase(', 'self.shared_rescue_phase(')
(p/'shared-rescue-adapter.rs').write_text(t);outs.append('shared-rescue-adapter.rs')
t=(src/'doubled.rs').read_text();t=t[t.index('impl RecoveryModes{'):t.index('include!("doubled-adapter.rs")')]
t=t.replace('fn doubled_phase(', 'fn shared_rescue_phase(')
t=t.replace('let doubled=RecoveryModes::from_modes(&stream)?;', 'let doubled=RecoveryModes{r:stream.r,m:stream.m,zeta:stream.zeta,block:stream.block,products:self.products.for_doubled(&stream)?};')
(p/'shared-rescue-phase.rs').write_text(t);outs.append('shared-rescue-phase.rs')
(p/'generation.json').write_text(json.dumps({'input_hashes':pins,'output_hashes':{n:hashlib.sha256((p/n).read_bytes()).hexdigest() for n in outs}},indent=2)+'\n')
