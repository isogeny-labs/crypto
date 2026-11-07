from pathlib import Path
import hashlib,json
p=Path(__file__).resolve().parent;b=p.parent/'feature-mode-set-pullback'
s=(b/'modes.rs').read_text().replace('include!("pullback.rs");','include!("reference.rs");\ninclude!("orbit.rs");')
ref=(b/'pullback.rs').read_text().replace('fn main(){','fn pullback_checks(){')
start=ref.index(' fn recover_pulled(');end=ref.index('\n}\nfn exact_sets',start)
method=ref[start:end].replace('fn recover_pulled(','fn recover_orbit(')
old='for &(offset,sign)in &history{fiber=replay(&mut p,&fiber,offset,sign,r,cap)?;stats.replayed+=1;}'
new='let (next,powers)=orbit_history(&mut p,&fiber,&history,self.m,self.zeta,r,cap)?;fiber=next;stats.replayed=history.len(); ORBIT_POWERS.with(|n|n.set(n.get()+powers));'
assert old in method;method=method.replace(old,new)
(p/'modes.rs').write_text(s);(p/'reference.rs').write_text(ref);(p/'adapter.rs').write_text('impl Modes{\n'+method+'\n}\n')
(p/'generation.json').write_text(json.dumps({'sources':{str(b/f):hashlib.sha256((b/f).read_bytes()).hexdigest() for f in ['modes.rs','pullback.rs']},'outputs':{f:hashlib.sha256((p/f).read_bytes()).hexdigest() for f in ['modes.rs','reference.rs','adapter.rs']}},indent=2)+'\n')
