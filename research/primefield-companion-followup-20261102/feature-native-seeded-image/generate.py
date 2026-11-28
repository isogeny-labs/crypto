"""Source generation only; arithmetic stays in Rust."""
from pathlib import Path
import hashlib,json
p=Path(__file__).resolve().parent;old=p.parent/'feature-native-powered-image';records=[]
for f in sorted(old.glob('*.rs')):
 if f.name=='driver.rs':continue
 (p/f.name).write_bytes(f.read_bytes());records.append({'source':f.name,'sha256':hashlib.sha256(f.read_bytes()).hexdigest()})
s=(old/'powered-image.rs').read_text().replace('PoweredImage','SeededImage').replace('POWERED_IMAGE_COUNTS','SEEDED_IMAGE_COUNTS')
s=s.replace('slot_cap:usize)->Result<Self,Error>', 'slot_cap:usize,initial_character:u64)->Result<Self,Error>')
s=s.replace('let r=modes.r;if', 'let r=modes.r;if initial_character!=1&&initial_character!=r-1{return Err(Error::Domain)}if',1)
s=s.replace('  for u in [1,r-1,i,r-i]{let g=', '  let normalized=mul(initial_character,pow(b,(r-1)/2,r),r);\n  for u in [1,r-1,i,r-i]{if mul(u,u,r)!=normalized{continue}let g=')
s=s.replace('zero:true,rounds:0','zero:false,rounds:0')
(p/'seeded-image.rs').write_text(s)
s=(old/'powered-adapter.rs').read_text().replace('recover_powered_image','recover_seeded_image').replace('PoweredImage','SeededImage').replace('POWERED_IMAGE_COUNTS','SEEDED_IMAGE_COUNTS')
s=s.replace('let mut history=vec![(0,if s<0{r-1}else{s as u64})];','let initial_character=if s<0{r-1}else{s as u64};let mut history=vec![(0,initial_character)];')
s=s.replace('SeededImage::new(self,b,&f,i,power_slots)?','SeededImage::new(self,b,&f,i,power_slots,initial_character)?')
(p/'seeded-adapter.rs').write_text(s)
s=(old/'driver.rs').read_text();a=s.index('fn main(){');s=s[:a]+'include!("seeded-image.rs");\ninclude!("seeded-adapter.rs");\ninclude!("seeded-checks.rs");\nfn main(){if std::env::args().any(|a|a=="--inherited"){inherited_checks()}else{seeded_checks();}}\n'
(p/'driver.rs').write_text(s)
(p/'generation.json').write_text(json.dumps({'source_commit':'4862830f2295b861a2ddbef25542336a7c015cfe','copied':records,'generated':['seeded-image.rs','seeded-adapter.rs','driver.rs']},indent=2)+'\n')
