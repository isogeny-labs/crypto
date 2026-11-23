from pathlib import Path
import hashlib,json
D=Path(__file__).resolve().parent;R=D.parent/'feature-native-seed-cancellation'
def once(s,a,b):
 assert s.count(a)==1,(a,s.count(a))
 return s.replace(a,b)
for n in ['source.rs','source-extra.rs','tests.rs']:(D/n).write_bytes((R/n).read_bytes())
(D/'tests.rs').write_text((R/'tests.rs').read_text()+(D/'subgroup-tests.rs').read_text())
s=(R/'lib.rs').read_text();s=once(s,'mod source;','mod source;\nmod subgroup;');(D/'lib.rs').write_text(s)
s=(R/'transport.rs').read_text()
s=once(s,'#[path="../feature-native-division/lib.rs"]mod jets;\n','')
a='fn phase(&self,q:[u64;2])->Result<u64,Error>{#[cfg(test)]PHASE_CALLS.with(|n|n.set(n.get()+1));jets::Division::new(self.p,self.a,self.b,q[0],q[1]).map_err(|_|Error::Phase)?.phase(self.r).map_err(|_|Error::Phase)}'
b='fn validate(&self,q:[u64;2])->Result<(),Error>{#[cfg(test)]SUBGROUP_CHECKS.with(|n|n.set(n.get()+1));super::super::subgroup::validate(self.p,self.a,self.b,self.r,q).map_err(|_|Error::Phase)}'
s=once(s,a,b).replace('PHASE_CALLS','SUBGROUP_CHECKS')
s=once(s,'let c=out.phase(base)?;let first=out.values(base,c)?;','out.validate(base)?;let first=out.values(base,1)?;')
s=once(s,'let c=s.phase(point)?;out.direct_initializations+=1;(s.values(point,c)?,"direct")','s.validate(point)?;out.direct_initializations+=1;(s.values(point,1)?,"direct")')
s=once(s,'let c=s.phase(base).unwrap();','s.validate(base).unwrap();let c=1;')
(D/'transport.rs').write_text(s)
s=(R/'transport-extra.rs').read_text();s=once(s,'let c=s.phase(q)?;(s.values(q,c)?,"direct")','s.validate(q)?;(s.values(q,1)?,"direct")');(D/'transport-extra.rs').write_text(s)
h=lambda p:hashlib.sha256(p.read_bytes()).hexdigest()
(D/'generation.json').write_text(json.dumps({'inputs':{n:h(R/n)for n in ['lib.rs','source.rs','source-extra.rs','transport.rs','transport-extra.rs','tests.rs']},'outputs':{n:h(D/n)for n in ['lib.rs','source.rs','source-extra.rs','transport.rs','transport-extra.rs','tests.rs']}},indent=2)+'\n')
