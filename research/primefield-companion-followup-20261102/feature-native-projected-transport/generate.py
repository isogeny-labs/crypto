"""Exact source transformations; preserve the full-value predecessor."""
from pathlib import Path
import hashlib,json
D=Path(__file__).resolve().parent; R=D.parent/'feature-native-monomial-recovery'
def once(s,a,b):
 assert s.count(a)==1,(a,s.count(a))
 return s.replace(a,b)
s=(R/'transport.rs').read_text()
s=once(s,'let mut out=Self{p,a,b,r,base,curve,partition,indices,lanes,base_x,squares:vec![],powers,anchors:vec![],max_window};', '''let order=1u64<<(p-1).trailing_zeros();if order>4{return Err(Error::Domain);}let odd=(p-1)/order;
  let powers=powers.into_iter().map(|v|((v as u128*odd as u128)%(p-1)as u128)as u64).collect();
  let mut out=Self{p,a,b,r,base,curve,partition,indices,lanes,base_x,squares:vec![],powers,anchors:vec![],max_window};''')
(D/'transport.rs').write_text(s)
s=(R/'source.rs').read_text()
s=once(s,'let geometry=residual::geometry_ratio(self.p,self.part,q,grid)?;', '''let geometry=residual::geometry_ratio(self.p,self.part,q,grid)?;
  let odd=(self.p-1)/(1u64<<(self.p-1).trailing_zeros());
  let geometry=geometry.map(|v|pow(v,odd,self.p));''')
(D/'source.rs').write_text(s)
for n in ['transport-extra.rs','source-extra.rs']:(D/n).write_bytes((R/n).read_bytes())
h=lambda p:hashlib.sha256(p.read_bytes()).hexdigest()
(D/'generation.json').write_text(json.dumps({'inputs':{n:h(R/n)for n in ['transport.rs','source.rs','transport-extra.rs','source-extra.rs']},'outputs':{n:h(D/n)for n in ['transport.rs','source.rs','transport-extra.rs','source-extra.rs']}},indent=2)+'\n')
