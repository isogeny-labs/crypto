from pathlib import Path
import hashlib,json
D=Path(__file__).resolve().parent;R=D.parent/'feature-native-projected-residual'
def once(s,a,b):
 assert s.count(a)==1,(a,s.count(a))
 return s.replace(a,b)
names=['lib.rs','source.rs','source-extra.rs','transport.rs','transport-extra.rs','tests.rs','subgroup.rs','subgroup-tests.rs','residual.rs','residual-tests.rs']
for n in names:
 s=(R/n).read_text()
 if n=='residual.rs':s=once(s,'../feature-native-low-product/polynomial.rs','polynomial.rs')
 (D/n).write_text(s)
p=D.parent/'feature-native-low-product/polynomial.rs';s=p.read_text()
s=once(s,' pub fn root_resultant(', (D/'product-method.rs').read_text()+' pub fn root_resultant(')
s=once(s,'let values=self.values(source.polynomial(),&destination)?;Ok(values.iter().fold(1,|v,&x|self.fm(v,x)))','self.value_product(source.polynomial(),&destination)')
s+='\n#[cfg(test)]#[path="../feature-native-low-product/polynomial.rs"]mod reference;\n#[cfg(test)]mod product_tests{use super::*;include!("product-tests.rs");}\n'
(D/'polynomial.rs').write_text(s)
h=lambda p:hashlib.sha256(p.read_bytes()).hexdigest()
(D/'generation.json').write_text(json.dumps({'inputs':{str(p):h(p)for p in [*(R/n for n in names),p,D/'product-method.rs',D/'product-tests.rs']},'outputs':{n:h(D/n)for n in names+['polynomial.rs']}},indent=2)+'\n')
