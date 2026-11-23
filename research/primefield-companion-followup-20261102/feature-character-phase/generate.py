from pathlib import Path
import hashlib,json
p=Path(__file__).resolve().parent;old=p.parent/'feature-split-negative-phase';rows=[]
for f in sorted(old.glob('*.rs')):
 if f.name=='driver.rs':continue
 (p/f.name).write_bytes(f.read_bytes());rows.append({'source':f.name,'sha256':hashlib.sha256(f.read_bytes()).hexdigest()})
s=(old/'seeded-image.rs').read_text().replace('SeededImage','CharacterImage').replace('CanonicalBranch','CharacterBranch').replace('SEEDED_IMAGE_COUNTS','CHARACTER_IMAGE_COUNTS')
a=s.index(' fn new(');b=s.index(' fn update(',a);s=s[:a]+s[b:]
s=s.replace('mul(mul(branch.u,branch.u,r),pow(scale,h/self.m as u64,r),r)','mul(branch.epsilon,pow(scale,h/self.m as u64,r),r)').replace('branch.u','branch.epsilon')
(p/'character-image.rs').write_text(s)
s=(old/'split-negative-adapter.rs').read_text().replace('recover_split_negative_image','recover_character_image').replace('SeededImage','CharacterImage').replace('SEEDED_IMAGE_COUNTS','CHARACTER_IMAGE_COUNTS')
s=s.replace('((self.r-1)/self.m as u64)%4!=0','((self.r-1)/2)%self.m as u64!=0')
a=s.index('  let mut fourth=');b=s.index('  let mut compressed:',a);s=s[:a]+s[b:]
s=s.replace('CharacterImage::new_split_negative(self,b,&f,i,power_slots,initial_character,trace_trials)','CharacterImage::new(self,b,&f,power_slots,initial_character,trace_trials)')
(p/'character-adapter.rs').write_text(s)
s=(old/'driver.rs').read_text();s=s[:s.index('fn main(){')]+'include!("character-domain.rs");\ninclude!("character-image.rs");\ninclude!("character-adapter.rs");\ninclude!("character-checks.rs");\nfn main(){if std::env::args().any(|x|x=="--inherited"){inherited_checks()}else{character_checks()}}\n'
(p/'driver.rs').write_text(s)
(p/'generation.json').write_text(json.dumps({'source_commit':'476e786fa633fc1fcb70f6110f8e5dd70e159cb6','copied':rows},indent=2)+'\n')
