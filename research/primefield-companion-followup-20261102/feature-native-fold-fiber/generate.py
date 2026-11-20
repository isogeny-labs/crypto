from pathlib import Path
import hashlib,json
D=Path(__file__).resolve().parent
source=D.parent/'feature-native-power-fold/lib.rs'
s=source.read_text();assert s.endswith('#[cfg(test)]mod tests;\n')
s=s.removesuffix('#[cfg(test)]mod tests;\n')+'include!("single.rs");\n#[cfg(test)]mod tests;\n'
(D/'lib.rs').write_text(s)
(D/'generation.json').write_text(json.dumps({'source':'../feature-native-power-fold/lib.rs','source_sha256':hashlib.sha256(source.read_bytes()).hexdigest(),'output_sha256':hashlib.sha256((D/'lib.rs').read_bytes()).hexdigest(),'change':'Append single.rs before local tests; original production prefix unchanged'},indent=2)+'\n')
