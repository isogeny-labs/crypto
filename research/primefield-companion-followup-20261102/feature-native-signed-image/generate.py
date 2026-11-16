"""Source transformation only; all research arithmetic runs in Rust."""
from pathlib import Path
import hashlib,json
p=Path(__file__).resolve().parent;old=p.parent/'feature-native-signed-reciprocal'
records=[]
for f in sorted(old.glob('*.rs')):
 if f.name=='driver.rs':continue
 (p/f.name).write_bytes(f.read_bytes());records.append({'source':f.name,'sha256':hashlib.sha256(f.read_bytes()).hexdigest()})
s=(old/'canonical-image.rs').read_text().replace('CanonicalImage','SignedImage').replace('CANONICAL_COUNTS','SIGNED_IMAGE_COUNTS')
a=s.index('  let h=(r-1)/2;',s.index(' fn absorb('));b=s.index('  }self.rounds+=1;Ok(())',a)
s=s[:a]+'''  let h=(r-1)/2;let a=mul(self.m as u64,pow(b,h,r),r);let t=pow(pow(b,self.m as u64,r),r-2,r);let scale=mul(t,self.inv_t0,r);
  let phase_scale=pow(scale,h/self.m as u64,r);
  self.zero=self.zero&&f[0]==a&&f[1]==0&&f[2]==0;
  for branch in &mut self.branches{if branch.ring.g.len()==1{continue}
   if !first{let value=branch.ring.mode_scaled(modes,0,scale)?;let mut c0=branch.ring.scale(&value,a);c0.0[0][0]=(c0.0[0][0]+r-f[0])%r;branch.keep(c0)?;}
   if branch.ring.g.len()==1{continue}
   let epsilon=mul(mul(branch.u,branch.u,r),phase_scale,r);
   let equation=branch.ring.signed_equation_scaled(modes,scale,[f[1],f[2]],epsilon)?;
   branch.keep(equation)?;
'''+s[b:]
(p/'signed-image.rs').write_text(s)
s=(old/'canonical-adapter.rs').read_text().replace('recover_canonical','recover_signed_image').replace('CanonicalImage','SignedImage').replace('CANONICAL_COUNTS','SIGNED_IMAGE_COUNTS')
s=s.replace('let f=self.observed(&signs)?;','let pair=self.signed_observed(&signs)?;let f=[signs.iter().fold(0,|a,&s|(a+s)%r),pair[0],pair[1]];')
(p/'signed-adapter.rs').write_text(s)
s=(p/'modes.rs').read_text()+'\ninclude!("signed.rs");\ninclude!("transport.rs");\ninclude!("signed-image.rs");\ninclude!("signed-adapter.rs");\ninclude!("image-checks.rs");\nfn main(){if std::env::args().any(|a|a=="--inherited"){inherited_checks()}else{signed_image_checks();}}\n'
(p/'driver.rs').write_text(s)
(p/'generation.json').write_text(json.dumps({'source_commit':'2062489b4b4f763abe488bf5ddc156e934f55942','copied':records,'generated':['signed-image.rs','signed-adapter.rs','driver.rs']},indent=2)+'\n')
