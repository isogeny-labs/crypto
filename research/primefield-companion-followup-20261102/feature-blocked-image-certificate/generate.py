from pathlib import Path
import json,hashlib
root=Path(__file__).resolve().parent;src=root.parent/'feature-certified-image-recurrence';pins={}
for name in ['modes.rs','reference.rs','adapter.rs','orbit.rs','dynamic.rs','envelope.rs','block-projection.rs','blocked-image.rs','trace-adapter.rs','history.rs','trace.rs','adaptive.rs']:
 s=(src/name).read_text();pins[name]=hashlib.sha256(s.encode()).hexdigest()
 if name=='history.rs':s=s.replace('adaptive_image(p,&state,m,r,cap,cap,cap.checked_mul(2).ok_or(Error::Limit)?)','adaptive_image_blocked(p,&state,m,r,cap,cap,cap.checked_mul(2).ok_or(Error::Limit)?,cap.checked_mul(cap).and_then(|v|v.checked_mul(2)).ok_or(Error::Limit)?)')
 if name=='trace.rs':
  s='include!("blocks.rs");\ninclude!("checks.rs");\n'+s
  s=s.replace('fn main(){adaptive_controls();','fn main(){block_checks();adaptive_controls();')
  s=s.replace('assert_eq!((&adaptive.images,&adaptive.envelope),(&got.images,&got.envelope));', '''assert_eq!((&adaptive.images,&adaptive.envelope),(&got.images,&got.envelope));let candidate=adaptive_image_blocked(&mut p,&g,m,r,r as usize,4096,2*r as usize,4096).unwrap();assert_eq!((&candidate.images,&candidate.envelope,&candidate.moments,candidate.rejected),(&adaptive.images,&adaptive.envelope,&adaptive.moments,adaptive.rejected));''')
 (root/name).write_text(s)
(root/'generation.json').write_text(json.dumps({'source':'../feature-certified-image-recurrence','input_hashes':pins,'output_hashes':{n:hashlib.sha256((root/n).read_bytes()).hexdigest() for n in pins}},indent=2)+'\n')
