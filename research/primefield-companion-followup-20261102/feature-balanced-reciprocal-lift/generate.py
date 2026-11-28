from pathlib import Path
import hashlib,json
p=Path(__file__).resolve().parent;old=p.parent/'feature-character-phase-certificate';rows=[]
for f in sorted(old.glob('*.rs')):
 if f.name=='driver.rs':continue
 (p/f.name).write_bytes(f.read_bytes());rows.append({'source':f.name,'sha256':hashlib.sha256(f.read_bytes()).hexdigest()})
for name in ['initial','image','adapter','checks']:
 s=(old/f'certified-{name}.rs').read_text().replace('CertifiedCharacterImage','BalancedCharacterImage').replace('CERTIFIED_CHARACTER_COUNTS','BALANCED_CHARACTER_COUNTS').replace('recover_certified_character_image','recover_balanced_character_image').replace('certified_fiber_lift','balanced_fiber_lift').replace('fn certified_checks()','fn balanced_checks()').replace('certified_lift_checks();','balanced_lift_checks();')
 if name=='checks':s=s.replace('let old=modes.recover_character_image','let old=modes.recover_certified_character_image')
 (p/f'balanced-{name}.rs').write_text(s)
s=(old/'certified-lift.rs').read_text().replace('certified_reciprocal_lift','balanced_reciprocal_lift').replace('certified_fiber_lift','balanced_fiber_lift').replace('certified_lift_checks','balanced_lift_checks')
a=s.index('let mut lifted=vec![1];');b=s.index('\n // W=',a)
s=s[:a]+'let mut lifted=homogeneous_lift(r,g)?;'+s[b:]
s=s.replace('fn balanced_lift_checks(){','fn balanced_lift_checks(){homogeneous_checks();')
(p/'balanced-lift.rs').write_text(s)
s=(old/'driver.rs').read_text();s=s[:s.index('fn main(){')]+''.join('include!("'+n+'");\n' for n in ['homogeneous.rs','balanced-lift.rs','balanced-initial.rs','balanced-image.rs','balanced-adapter.rs','balanced-checks.rs'])+'fn main(){if std::env::args().any(|x|x=="--inherited"){inherited_checks()}else{balanced_checks()}}\n'
(p/'driver.rs').write_text(s)
(p/'generation.json').write_text(json.dumps({'source_commit':'0aa506f1c06fd8b5053a88ba1354a16e0ffb944f','copied':rows},indent=2)+'\n')
