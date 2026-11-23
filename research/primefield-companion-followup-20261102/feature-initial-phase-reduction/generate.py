from pathlib import Path
import hashlib,json
p=Path(__file__).resolve().parent;old=p.parent/'feature-native-seeded-image';rows=[]
for f in sorted(old.glob('*.rs')):
 if f.name=='driver.rs':continue
 (p/f.name).write_bytes(f.read_bytes());rows.append({'source':f.name,'sha256':hashlib.sha256(f.read_bytes()).hexdigest()})
s=(old/'seeded-adapter.rs').read_text().replace('recover_seeded_image','recover_sectioned_image').replace('SeededImage::new(','SeededImage::new_sectioned(')
(p/'sectioned-adapter.rs').write_text(s)
s=(old/'driver.rs').read_text();s=s[:s.index('fn main(){')]+'include!("initial.rs");\ninclude!("sectioned-adapter.rs");\ninclude!("initial-checks.rs");\nfn main(){if std::env::args().any(|x|x=="--inherited"){inherited_checks()}else{initial_checks()}}\n'
(p/'driver.rs').write_text(s)
(p/'generation.json').write_text(json.dumps({'source_commit':'f56d959e467bac388651225c85c2ceeb228d2c98','copied':rows},indent=2)+'\n')
