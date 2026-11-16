from pathlib import Path
import hashlib,json
p=Path(__file__).resolve().parent;src=p.parent/'feature-native-neighbor-modes';pins={};outs=[]
for f in sorted(src.iterdir()):
 if f.suffix!='.rs' and f.name!='validate-recovery.py':continue
 t=f.read_text();pins[str(f)]=hashlib.sha256(f.read_bytes()).hexdigest()
 if f.name=='neighbor.rs':t+='\ninclude!("fused.rs");\n'
 if f.name=='new-recovery.rs':t=t.replace('fn main(){','fn main(){fused_checks();',1)
 (p/f.name).write_text(t);outs.append(f.name)
for name in ['image','adapter']:
 t=(src/('neighbor-'+name+'.rs')).read_text().replace('NeighborImage','FusedImage').replace('NEIGHBOR_COUNTS','FUSED_COUNTS').replace('recover_neighbor','recover_fused').replace('neighbor_modes','fused_modes')
 dest='fused-'+name+'.rs';(p/dest).write_text(t);outs.append(dest)
s=(src/'neighbor.rs').read_text();a=s.index('fn neighbor_block(');b=s.index('fn neighbor_modes(',a);t=s[a:b].replace('neighbor_block','fused_block').replace('NeighborReader','FusedReader');(p/'fused-block.rs').write_text(t);outs.append('fused-block.rs')
(p/'generation.json').write_text(json.dumps({'input_hashes':pins,'output_hashes':{n:hashlib.sha256((p/n).read_bytes()).hexdigest() for n in outs}},indent=2)+'\n')
