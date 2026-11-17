from pathlib import Path
import hashlib,json
D=Path(__file__).resolve().parent;R=D.parent/'feature-native-resultant-product'
def once(s,a,b):
 assert s.count(a)==1,(a,s.count(a))
 return s.replace(a,b)
names=['lib.rs','source.rs','source-extra.rs','transport.rs','transport-extra.rs','tests.rs','subgroup.rs','subgroup-tests.rs','residual.rs','residual-tests.rs','polynomial.rs','product-tests.rs']
for n in names:
 s=(R/n).read_text()
 if n=='residual.rs':
  assert s.count('root_resultant(')==2;s=s.replace('root_resultant(','root_projected_resultant(')
  s=once(s,' let mut out=pow(resultant,odd,p);',' let mut out=resultant;')
  s=once(s,' let odd=(p-1)/order;let[w,k,s,e]=part;',' let[w,k,s,e]=part;')
 if n=='polynomial.rs':
  s=once(s,' pub fn value_product(', (D/'euclid-method.rs').read_text()+' pub fn value_product(')
  s+='\n#[cfg(test)]mod euclid_tests{use super::*;include!("euclid-tests.rs");}\n'
 (D/n).write_text(s)
h=lambda p:hashlib.sha256(p.read_bytes()).hexdigest()
(D/'generation.json').write_text(json.dumps({'inputs':{str(p):h(p)for p in [*(R/n for n in names),D/'euclid-method.rs',D/'euclid-tests.rs']},'outputs':{n:h(D/n)for n in names}},indent=2)+'\n')
