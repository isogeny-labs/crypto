from pathlib import Path
import hashlib,json
D=Path(__file__).resolve().parent;R=D.parent/'feature-native-torsion-cache'
def once(s,a,b):
 assert s.count(a)==1,(a,s.count(a))
 return s.replace(a,b)
names=['lib.rs','source.rs','source-extra.rs','transport.rs','transport-extra.rs','tests.rs','subgroup.rs','subgroup-tests.rs']
for n in names:
 s=(R/n).read_text()
 if n=='source.rs':
  s=once(s,'#[path="../feature-native-multiseed-factor/residual.rs"]mod residual;','#[path="residual.rs"]mod residual;')
  s=once(s,'  let odd=(self.p-1)/(1u64<<(self.p-1).trailing_zeros());\n  let geometry=geometry.map(|v|pow(v,odd,self.p));\n','')
  s+='\n#[cfg(test)]mod residual_tests{use super::*;include!("residual-tests.rs");}\n'
 (D/n).write_text(s)
h=lambda p:hashlib.sha256(p.read_bytes()).hexdigest()
(D/'generation.json').write_text(json.dumps({'inputs':{n:h(R/n)for n in names},'outputs':{n:h(D/n)for n in names},'local_inputs':{n:h(D/n)for n in ['residual.rs','residual-tests.rs']}},indent=2)+'\n')
