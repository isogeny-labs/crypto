from pathlib import Path
import hashlib,json
p=Path(__file__).resolve().parent;src=p.parent/'feature-native-canonical-fibers';pins={};outs=[]
for f in sorted(src.iterdir()):
 if f.suffix!='.rs' and f.name!='validate-recovery.py':continue
 t=f.read_text();pins[str(f)]=hashlib.sha256(f.read_bytes()).hexdigest()
 if f.name=='canonical.rs':t+='\ninclude!("split.rs");\n'
 if f.name=='new-recovery.rs':t=t.replace('fn main(){','fn main(){split_checks();',1)
 (p/f.name).write_text(t);outs.append(f.name)
t=(src/'canonical-image.rs').read_text().replace('CanonicalImage','SplitImage').replace('CANONICAL_COUNTS','SPLIT_COUNTS')
a=t.index('   if !first{');b=t.index('\n  }self.rounds',a)
t=t[:a]+"""   if !first{let mut ring=SplitRing::new(&branch.canonical)?;let value=ring.mode(modes,0,scale)?;let mut eq=ring.scale(&value,a);ring.subtract_constant(&mut eq,f[0]);branch.keep_split(eq)?;}
   if branch.ring.g.len()==1{continue}let mut ring=SplitRing::new(&branch.canonical)?;let f1=ring.mode(modes,1,scale)?;let f2=ring.mode(modes,2,scale)?;let sq=ring.multiply(&f1,&f1)?;let l=ring.scale(&f2,left);let rr=ring.scale(&sq,right);branch.keep_split(ring.sub(&l,&rr))?;"""+t[b:]
(p/'split-image.rs').write_text(t);outs.append('split-image.rs')
t=(src/'canonical-adapter.rs').read_text().replace('recover_canonical','recover_split').replace('CanonicalImage','SplitImage').replace('CANONICAL_COUNTS','SPLIT_COUNTS')
(p/'split-adapter.rs').write_text(t);outs.append('split-adapter.rs')
(p/'generation.json').write_text(json.dumps({'input_hashes':pins,'output_hashes':{n:hashlib.sha256((p/n).read_bytes()).hexdigest() for n in outs}},indent=2)+'\n')
