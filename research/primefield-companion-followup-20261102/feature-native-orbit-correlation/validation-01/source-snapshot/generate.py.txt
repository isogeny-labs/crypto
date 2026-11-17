from pathlib import Path
import hashlib,json
D=Path(__file__).resolve().parent;S=D.parent/'feature-native-fold-fiber'
a=(S/'lib.rs').read_text();assert a.endswith('#[cfg(test)]mod tests;\n')
a=a.removesuffix('#[cfg(test)]mod tests;\n').replace('include!("single.rs");','include!("../feature-native-fold-fiber/single.rs");')
b=(S/'single.rs').read_text();method=b[b.index(' pub fn recover(&self,target:'):b.rfind('\n}')]
method=method.replace('pub fn recover(&self,target:Option<[u64;2]>)','fn recover_correlated(&self,target:Option<[u64;2]>,correlations:&[Correlation])')
assert 'let mut product=1;for offset' in method
method=method.replace('let mut product=1;for offset','let mut orbit=Vec::with_capacity(self.inner.fold.m);let mut product=1;for offset').replace('product*=sign;','orbit.push(sign);product*=sign;')
needle='self.inner.fold.observe(&mut image,b as u64,product)?;'
assert needle in method
method=method.replace(needle,needle+'for c in correlations{c.observe(&mut image,b as u64,&orbit)?;}')
(D/'lib.rs').write_text(a+'include!("correlation.rs");\nimpl SingleRecovery{\n'+method+'\n}\n#[cfg(test)]mod tests;\n')
(D/'generation.json').write_text(json.dumps({'source_hashes':{str(f.relative_to(D.parent)):hashlib.sha256(f.read_bytes()).hexdigest()for f in [S/'lib.rs',S/'single.rs']},'output_sha256':hashlib.sha256((D/'lib.rs').read_bytes()).hexdigest(),'changes':'Relocate single include; derive second method adding orbit-sign collection and correlation observations before singleton test. Old method preserved.'},indent=2)+'\n')
