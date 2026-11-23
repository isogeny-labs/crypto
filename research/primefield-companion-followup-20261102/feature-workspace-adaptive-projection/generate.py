from pathlib import Path
import hashlib,json
root=Path(__file__).resolve().parent;src=root.parent/'feature-blocked-image-certificate';pins={}
names=['modes.rs','reference.rs','adapter.rs','orbit.rs','dynamic.rs','envelope.rs','block-projection.rs','blocked-image.rs','trace-adapter.rs','history.rs','trace.rs','adaptive.rs','blocks.rs','checks.rs']
for name in names:
 s=(src/name).read_text();pins[name]=hashlib.sha256(s.encode()).hexdigest()
 if name=='trace.rs':
  s='include!("planner.rs");\ninclude!("planned.rs");\ninclude!("workspace-checks.rs");\n'+s
  s=s.replace('fn main(){block_checks();','fn main(){workspace_checks();block_checks();')
  s=s.replace('let candidate=adaptive_image_blocked', 'let candidate=adaptive_image_workspace')
  # Repeat exact small/larger comparisons with a one-row table budget n.
  s=s.replace('(&adaptive.images,&adaptive.envelope,&adaptive.moments,adaptive.rejected));', '''(&adaptive.images,&adaptive.envelope,&adaptive.moments,adaptive.rejected));let tight=adaptive_image_workspace(&mut p,&g,m,r,r as usize,4096,2*r as usize,g.len()-1).unwrap();assert_eq!((&tight.images,&tight.envelope,&tight.moments,tight.rejected),(&adaptive.images,&adaptive.envelope,&adaptive.moments,adaptive.rejected));''')
 if name=='history.rs':
  s=s.replace('adaptive_image_blocked(', 'adaptive_image_workspace(')
  # cap coefficient words: exactly one row for maximum candidate degree.
  s=s.replace('cap.checked_mul(cap).and_then(|v|v.checked_mul(2)).ok_or(Error::Limit)?','cap')
 (root/name).write_text(s)
s=(src/'blocks.rs').read_text()
start=s.index(' let mut short=');end=s.index('\nfn blocked_certificate')
body=s[start:end]
head='''fn planned_blocks(p:&mut polynomial::Poly,g:&[u64],b:&[u64],length:usize,max_words:usize)->Result<PowerBlocks,Error>{
 if g.len()<2{return Err(Error::Domain)}let width=width_plan(g.len()-1,length,max_words)?.width;
'''
a=s[s.index('fn adaptive_image_blocked'):].replace('fn adaptive_image_blocked','fn adaptive_image_workspace').replace('let blocks=power_blocks(', 'let blocks=planned_blocks(')
(root/'planned.rs').write_text(head+body+'\n'+a)
outputs=names+['planned.rs']
(root/'generation.json').write_text(json.dumps({'source':'../feature-blocked-image-certificate','input_hashes':pins,'output_hashes':{n:hashlib.sha256((root/n).read_bytes()).hexdigest() for n in outputs}},indent=2)+'\n')
