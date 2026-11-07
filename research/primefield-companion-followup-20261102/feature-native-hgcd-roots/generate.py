from pathlib import Path
import hashlib,json
D=Path(__file__).resolve().parent;R=D.parent
paths=[R/'feature-native-half-gcd/polynomial.rs',R/'feature-native-fast-moment-roots/lib.rs',R/'feature-native-fast-moment-roots/roots.rs',R/'feature-native-fast-moment-roots/recovery.rs']
poly=paths[0].read_text()
for name in ['product-tests.rs','euclid-tests.rs','class-tests.rs','half-tests.rs']:
 poly=poly.replace('include!("'+name+'")','include!("../feature-native-half-gcd/'+name+'")')
poly+='\ninclude!("gcd.rs");\n'
lib=paths[1].read_text().split('\n#[cfg(test)]mod fast_tests',1)[0]
old='#[path="../feature-native-half-gcd/polynomial.rs"]mod polynomial;';assert lib.count(old)==1
lib=lib.replace(old,'mod polynomial;')+'\n#[cfg(test)]mod tests;\n'
roots=paths[2].read_text();lines=roots.splitlines();indices=[i for i,s in enumerate(lines)if s.startswith(' fn gcd(')];assert len(indices)==1
lines[indices[0]]=' fn gcd(&mut self,a:Vec<u64>,b:Vec<u64>)->Result<Vec<u64>,Error>{self.poly.gcd_monic(&a,&b,self.coefficients.len()-1).map(|v|v.polynomial).map_err(|_|Error::Domain)}'
outputs={'polynomial.rs':poly,'lib.rs':lib,'roots.rs':'\n'.join(lines)+'\n','recovery.rs':paths[3].read_text()}
for n,s in outputs.items():(D/n).write_text(s)
sha=lambda b:hashlib.sha256(b).hexdigest()
(D/'generation.json').write_text(json.dumps({'sources':{str(p.relative_to(R)):sha(p.read_bytes())for p in paths},'outputs':{n:sha(s.encode())for n,s in outputs.items()},'transformation':'Copy polynomial arithmetic unchanged, relocate four test includes and append gcd driver include; copy fast converter/recovery; replace only RootSet gcd dispatch; replace test module'},indent=2)+'\n')
