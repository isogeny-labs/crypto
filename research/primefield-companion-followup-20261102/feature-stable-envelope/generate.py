from pathlib import Path
import hashlib,json
p=Path(__file__).resolve().parent;b=p.parent/'feature-orbit-replay'
for f in ['modes.rs','reference.rs','adapter.rs']:(p/f).write_bytes((b/f).read_bytes())
s=(b/'orbit.rs').read_text().replace('fn main(){','fn orbit_checks(){')
(p/'orbit.rs').write_text(s+'\ninclude!("envelope.rs");\n')
a=(b/'adapter.rs').read_text().replace('recover_orbit(','recover_enveloped(').replace('orbit_history(','envelope_history(')
(p/'dynamic.rs').write_text(a)
(p/'generation.json').write_text(json.dumps({'sources':{str(b/f):hashlib.sha256((b/f).read_bytes()).hexdigest() for f in ['modes.rs','reference.rs','adapter.rs','orbit.rs']},'outputs':{f:hashlib.sha256((p/f).read_bytes()).hexdigest() for f in ['modes.rs','reference.rs','adapter.rs','orbit.rs','dynamic.rs']}},indent=2)+'\n')
