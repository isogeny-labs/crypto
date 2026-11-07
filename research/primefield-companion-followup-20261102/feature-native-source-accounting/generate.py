from pathlib import Path
import json,hashlib
p=Path(__file__).resolve().parent;src=p.parent/'feature-native-split-lifetimes';geom=p.parent/'feature-native-joint-window/lib.rs';pins={str(geom):hashlib.sha256(geom.read_bytes()).hexdigest()};outs=[]
g=geom.read_text().split('#[cfg(test)]mod joint_tests{',1)[0]
g=g.replace('pub struct Curve{','#[derive(Clone)]\npub struct Curve{').replace("pub struct Prepared<'a>{curve:&'a Curve",'pub struct Prepared{curve:Curve').replace("Result<Prepared<'_>,Error>",'Result<Prepared,Error>').replace('curve:self,r,','curve:self.clone(),r,').replace("impl Prepared<'_>{",'impl Prepared{').replace('let c=self.curve;','let c=&self.curve;')
(p/'geometry.rs').write_text(g);outs.append('geometry.rs')
for name in ['schedule.rs','allocation.rs','division.rs','lib.rs','polynomial.rs','class-source.rs','class-checks.rs','check.rs','schedule-checks.rs','liveness-checks.rs','generate-recovery.py','validate.py','validate-recovery.py','new-recovery.rs']:
 f=src/name;t=f.read_text();pins[str(f)]=hashlib.sha256(f.read_bytes()).hexdigest()
 if name=='lib.rs':
  t=t.replace('#[path="../feature-native-joint-window/lib.rs"] mod geometry;','mod geometry;')
  t=t.replace('mode:division::Mode}', '''mode:division::Mode,exponent:u64,prepared:std::sync::OnceLock<Result<geometry::Prepared,geometry::Error>>,#[cfg(source_audit)] preparation_builds:std::sync::atomic::AtomicUsize}''')
  t=t.replace('  let mut out=Self{', '  let order=1u64<<(p-1).trailing_zeros();let exponent=inverse(mul(r%order,r%order,order),order)?;\n  let mut out=Self{')
  t=t.replace('schedule,mode};','schedule,mode,exponent,prepared:std::sync::OnceLock::new(),#[cfg(source_audit)] preparation_builds:std::sync::atomic::AtomicUsize::new(0)};')
  t=t.replace('let exponent=inverse(mul(self.r%order,self.r%order,order),order)?;', 'let exponent=self.exponent;')
  t=t.replace('let prepared=self.curve.prepare(self.r,self.base,w,k,s,e).map_err(|_|Error::Geometry)?;', '''let prepared=self.prepared.get_or_init(||{#[cfg(source_audit)] self.preparation_builds.fetch_add(1,std::sync::atomic::Ordering::Relaxed);self.curve.prepare(self.r,self.base,w,k,s,e)}).as_ref().map_err(|_|Error::Geometry)?;''')
  t=t.replace(' pub fn raw(', ' #[cfg(source_audit)] pub fn preparation_builds(&self)->usize{self.preparation_builds.load(std::sync::atomic::Ordering::Relaxed)}\n pub fn raw(')
 if name=='check.rs':
  t=t.replace('for(start,k0)in', 'assert_eq!(s.preparation_builds(),0);assert_eq!(s.window(None,0),Ok(vec![]));assert_eq!(s.preparation_builds(),0);\n  for(start,k0)in')
  t=t.replace('  assert!(source::Source::new(p,a,b,r,base,0)', '''  assert_eq!(s.preparation_builds(),1);assert_eq!(selected.preparation_builds(),0);std::thread::scope(|scope|{let hs:Vec<_>=(0..4).map(|_|scope.spawn(||selected.window(Some(base),3).unwrap())).collect();for h in hs{assert_eq!(h.join().unwrap(),s.window(Some(base),3).unwrap());}});assert_eq!(selected.preparation_builds(),1);std::thread::scope(|scope|{let handles:Vec<_>=(0..4).map(|_|scope.spawn(||s.window(Some(base),3).unwrap())).collect();for h in handles{assert_eq!(h.join().unwrap(),s.window(Some(base),3).unwrap());}});assert_eq!(s.preparation_builds(),1);
  assert!(source::Source::new(p,a,b,r,base,0)''')
  t=t.replace('let selected=source::Source', 'let selected=source::Source')
  t=t.replace(' }assert!(wrong_exponent>0);',' }println!("prepared_sources=8 lazy_zero_window_controls=8 repeated_and_concurrent_reuse=true");assert!(wrong_exponent>0);')
 if name=='validate.py':t=t.replace("'--edition','2021'", "'--edition','2021','--cfg','source_audit'")
 (p/name).write_text(t);outs.append(name)
(p/'generation.json').write_text(json.dumps({'input_hashes':pins,'output_hashes':{n:hashlib.sha256((p/n).read_bytes()).hexdigest() for n in outs}},indent=2)+'\n')
