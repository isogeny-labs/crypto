from pathlib import Path
import json,hashlib
p=Path(__file__).resolve().parent
sources=[p.parent/'feature-balanced-reciprocal-lift/polynomial.rs',p.parent/'feature-native-hgcd-roots/gcd.rs']
rows=[]
for f in sources:
 (p/f.name).write_bytes(f.read_bytes());rows.append({'source':str(f.relative_to(p.parent)),'local':f.name,'sha256':hashlib.sha256(f.read_bytes()).hexdigest()})
(p/'engine.rs').write_text((p/'polynomial.rs').read_text()+'\ninclude!("gcd.rs");\n')
(p/'generation.json').write_text(json.dumps({'source_commit':'13f693c7c969ff894c7c63b5c110295d0d5f97e5','copied':rows},indent=2)+'\n')
