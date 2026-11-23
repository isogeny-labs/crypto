from pathlib import Path
import hashlib,json
p=Path(__file__).resolve().parent;src=p.parent/'feature-native-persistent-fibers';pins={};outs=[]
for f in sorted(src.iterdir()):
 if f.suffix!='.rs' and f.name!='validate-recovery.py':continue
 t=f.read_text();pins[str(f)]=hashlib.sha256(f.read_bytes()).hexdigest()
 if f.name=='persistent.rs':t+='\ninclude!("canonical.rs");\n'
 if f.name=='new-recovery.rs':t=t.replace('fn main(){','fn main(){canonical_checks();',1)
 (p/f.name).write_text(t);outs.append(f.name)
s=(src/'persistent.rs').read_text();t=s[s.index('struct PersistentImage{'):s.index('include!("persistent-adapter.rs")')]
t=t.replace('PersistentImage','CanonicalImage').replace('OrientedBranch','CanonicalBranch').replace('FIBER_COUNTS','CANONICAL_COUNTS')
t=t.replace('constraints:Vec::new()','canonical:CanonicalFiber::new(r,&g)?')
old='let mut lift=self.orientation.lift(&branch.ring.g,branch.u,lift_cap)?;for pair in &branch.constraints{lift=branch.ring.filter_lift(pair,&lift)?;}'
new='let lift=branch.canonical.lift(&self.orientation,branch.u,lift_cap,&mut branch.ring)?;'
assert old in t;t=t.replace(old,new)
(p/'canonical-image.rs').write_text(t);outs.append('canonical-image.rs')
t=(src/'persistent-adapter.rs').read_text().replace('recover_persistent','recover_canonical').replace('PersistentImage','CanonicalImage').replace('FIBER_COUNTS','CANONICAL_COUNTS')
(p/'canonical-adapter.rs').write_text(t);outs.append('canonical-adapter.rs')
(p/'generation.json').write_text(json.dumps({'input_hashes':pins,'output_hashes':{n:hashlib.sha256((p/n).read_bytes()).hexdigest() for n in outs}},indent=2)+'\n')
