from pathlib import Path
import hashlib,json
p=Path(__file__).resolve().parent;src=p.parent/'feature-native-quadratic-modes';pins={};outs=[]
for f in sorted(src.iterdir()):
 if f.suffix!='.rs' and f.name!='validate-recovery.py':continue
 t=f.read_text();pins[str(f)]=hashlib.sha256(f.read_bytes()).hexdigest()
 if f.name=='quadratic.rs':t+='\ninclude!("persistent.rs");\n'
 if f.name=='new-recovery.rs':t=t.replace('fn main(){','fn main(){persistent_checks();',1)
 (p/f.name).write_text(t);outs.append(f.name)
s=(src/'stream-adapter.rs').read_text();t=s[s.index('impl RecoveryModes{'):s.index('impl StreamModes{fn recover_traced')];t=t.replace('fn recover_traced(','fn recover_persistent(')
old='  let r=self.r;let s=source.sign'
new="""  if self.m<3||((self.r-1)/self.m as u64)%4!=0{return self.recover_traced(source,curve,base,target,radii,cap,extra,max_calls)}
  let mut fourth=None;for z in 2..self.r.min(66){let i=pow(z,(self.r-1)/4,self.r);if mul(i,i,self.r)==self.r-1{fourth=Some(i);break}}
  let Some(i)=fourth else{return self.recover_traced(source,curve,base,target,radii,cap,extra,max_calls)};
  let mut compressed:Option<PersistentImage>=None;
  let r=self.r;let s=source.sign"""
assert old in t;t=t.replace(old,new)
t=t.replace('let mut state=ImageDomain::new(r,self.m)?;','')
old='   let f=self.observed(&signs)?;self.intersect_modes(&mut p,&mut state,b,&f)?;'
new="""   let f=self.observed(&signs)?;
   if let Some(image)=&mut compressed{image.update(self,b,&f)?;}else{compressed=Some(PersistentImage::new(self,b,&f,i)?);}
   let dense=match compressed.as_mut().unwrap().materialize(cap,cap/self.m){Ok(g)=>g,Err(Error::Limit)=>{FIBER_COUNTS.with(|c|{let(a,b,d,e)=c.get();c.set((a,b,d+1,e))});continue},Err(e)=>return Err(e)};
   FIBER_COUNTS.with(|c|{let(a,b,d,e)=c.get();c.set((a,b,d,e+1))});
   let mut state=ImageDomain::new(r,self.m)?;state.intersect(&mut p,&dense)?;"""
assert old in t;t=t.replace(old,new)
t+="\nimpl StreamModes{fn recover_persistent(&self,source:&source::Source,curve:group::Curve,base:group::Point,target:Option<group::Point>,radii:&[u64],cap:usize,extra:&[u64],max_calls:usize)->Result<(Option<u64>,PullStats,&'static str),Error>{RecoveryModes::from_modes(self)?.recover_persistent(source,curve,base,target,radii,cap,extra,max_calls)}}\n"
(p/'persistent-adapter.rs').write_text(t);outs.append('persistent-adapter.rs')
(p/'generation.json').write_text(json.dumps({'input_hashes':pins,'output_hashes':{n:hashlib.sha256((p/n).read_bytes()).hexdigest() for n in outs}},indent=2)+'\n')
