from pathlib import Path
import hashlib,json
p=Path(__file__).resolve().parent
source=p.parent/'feature-weighted-mode-derivation/modes.rs'
s=source.read_text();assert s.count('fn main(){')==1
s=s.replace('fn main(){','fn previous_checks(){',1)+'\ninclude!("pullback.rs");\n'
(p/'modes.rs').write_text(s)
(p/'generation.json').write_text(json.dumps({'source':str(source),'source_sha256':hashlib.sha256(source.read_bytes()).hexdigest(),'output_sha256':hashlib.sha256(s.encode()).hexdigest()},indent=2)+'\n')
