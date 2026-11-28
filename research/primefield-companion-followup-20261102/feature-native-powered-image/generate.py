"""Source generation only; research arithmetic stays in Rust."""
from pathlib import Path
import hashlib,json
p=Path(__file__).resolve().parent;old=p.parent/'feature-native-signed-image';records=[]
for f in sorted(old.glob('*.rs')):
 if f.name=='driver.rs':continue
 (p/f.name).write_bytes(f.read_bytes());records.append({'source':f.name,'sha256':hashlib.sha256(f.read_bytes()).hexdigest()})
s=(old/'signed-image.rs').read_text().replace('SignedImage','PoweredImage').replace('SIGNED_IMAGE_COUNTS','POWERED_IMAGE_COUNTS')
s=s.replace('rounds:usize}', 'rounds:usize,slot_cap:usize}')
s=s.replace('f:&[u64],i:u64)->Result<Self,Error>', 'f:&[u64],i:u64,slot_cap:usize)->Result<Self,Error>')
s=s.replace('zero:true,rounds:0}', 'zero:true,rounds:0,slot_cap}')
a=s.index('  let phase_scale=');b=s.index('  }self.rounds+=1;Ok(())',a)
s=s[:a]+'''  self.zero=self.zero&&f[0]==a&&f[1]==0&&f[2]==0;
  for branch in &mut self.branches{if branch.ring.g.len()==1{continue}
   let variable=Quad([vec![0],vec![scale]]);
   let sections=match branch.ring.sections(self.m,&variable,self.slot_cap){Ok(v)=>Some(v),Err(Error::Limit)=>None,Err(e)=>return Err(e)};
   if let Some(values)=sections{
    if !first{let mut c0=branch.ring.scale(&values[0],a);c0.0[0][0]=(c0.0[0][0]+r-f[0])%r;branch.keep(c0)?;}
    if branch.ring.g.len()==1{continue}
    let product=branch.ring.multiply(&values[1],&values[self.m-1])?;
    let product=branch.ring.multiply(&product,&variable)?;
    let mut equation=branch.ring.scale(&product,mul(a,a,r));equation.0[0][0]=(equation.0[0][0]+r-mul(f[1],f[2],r))%r;
    branch.keep(equation)?;
   }else{
    POWERED_EVAL.with(|c|{let(a,b,d)=c.get();c.set((a,b,d+1))});
    if !first{let value=branch.ring.mode_scaled(modes,0,scale)?;let mut c0=branch.ring.scale(&value,a);c0.0[0][0]=(c0.0[0][0]+r-f[0])%r;branch.keep(c0)?;}
    if branch.ring.g.len()==1{continue}
    let epsilon=mul(mul(branch.u,branch.u,r),pow(scale,h/self.m as u64,r),r);
    let equation=branch.ring.signed_equation_scaled(modes,scale,[f[1],f[2]],epsilon)?;branch.keep(equation)?;
   }
'''+s[b:]
(p/'powered-image.rs').write_text(s)
s=(old/'signed-adapter.rs').read_text().replace('recover_signed_image','recover_powered_image').replace('SignedImage','PoweredImage').replace('SIGNED_IMAGE_COUNTS','POWERED_IMAGE_COUNTS')
s=s.replace('max_calls:usize)->','max_calls:usize,power_slots:usize)->').replace('PoweredImage::new(self,b,&f,i)?','PoweredImage::new(self,b,&f,i,power_slots)?')
s=s.replace('recover_powered_image(source,curve,base,target,radii,cap,extra,max_calls)}}','recover_powered_image(source,curve,base,target,radii,cap,extra,max_calls,power_slots)}}')
s=s.replace('self.phase(v,b,f[1])?', 'self.phase_powered(v,b,f[1],power_slots)?')
(p/'powered-adapter.rs').write_text(s)
s=(old/'driver.rs').read_text();a=s.index('fn main(){');s=s[:a]+'include!("powered.rs");\ninclude!("powered-image.rs");\ninclude!("powered-adapter.rs");\ninclude!("powered-checks.rs");\nfn main(){if std::env::args().any(|a|a=="--inherited"){inherited_checks()}else{powered_checks();}}\n'
(p/'driver.rs').write_text(s)
(p/'generation.json').write_text(json.dumps({'source_commit':'2c42cd6462bc667a16691e99b4681a14c5f878e2','copied':records,'generated':['powered-image.rs','powered-adapter.rs','driver.rs']},indent=2)+'\n')
