from pathlib import Path
import hashlib,json
D=Path(__file__).resolve().parent;S=D.parent/'feature-native-orbit-correlation'
s=(S/'lib.rs').read_text();assert s.endswith('#[cfg(test)]mod tests;\n');s=s.removesuffix('#[cfg(test)]mod tests;\n').replace('include!("correlation.rs");','include!("../feature-native-orbit-correlation/correlation.rs");')
start=s.index(' fn recover_correlated(');end=s.rfind('\n}\n');method=s[start:end]
method=method.replace('fn recover_correlated','fn recover_higher').replace('correlations:&[Correlation])','correlations:&[Correlation],triples:&[TripleCorrelation])')
needle='for c in correlations{c.observe(&mut image,b as u64,&orbit)?;}'
assert needle in method;method=method.replace(needle,needle+'for t in triples{t.observe(&mut image,b as u64,&orbit)?;}')
(D/'lib.rs').write_text(s+'include!("triple.rs");\nimpl SingleRecovery{\n'+method+'\n}\n#[cfg(test)]mod tests;\n')
(D/'generation.json').write_text(json.dumps({'source':'../feature-native-orbit-correlation/lib.rs','source_sha256':hashlib.sha256((S/'lib.rs').read_bytes()).hexdigest(),'output_sha256':hashlib.sha256((D/'lib.rs').read_bytes()).hexdigest(),'changes':'Relocate pair include; derive new method adding triple constraints after pair constraints; historical methods preserved.'},indent=2)+'\n')
