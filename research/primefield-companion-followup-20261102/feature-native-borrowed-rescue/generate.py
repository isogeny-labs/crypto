from pathlib import Path
import json,hashlib
p=Path(__file__).resolve().parent;src=p.parent/'feature-native-shared-rescue';pins={};outs=[]
for f in sorted(src.iterdir()):
 if f.suffix!='.rs' and f.name!='validate-recovery.py':continue
 t=f.read_text();pins[str(f)]=hashlib.sha256(f.read_bytes()).hexdigest()
 if f.suffix=='.rs':
  t=t.replace('impl RecoveryModes{', "impl RecoveryModes<'_>{").replace('impl RecoveryProducts{', "impl RecoveryProducts<'_>{")
 if f.name=='context.rs':
  t='include!("engine-storage.rs");\n'+t
  t=t.replace("&'a RecoveryProducts", "&'a RecoveryProducts<'a>")
  t=t.replace('struct RecoveryProducts{', "struct RecoveryProducts<'a>{")
  t=t.replace('inner:std::rc::Rc<std::cell::RefCell<IntervalProducts>>', "inner:EngineStorage<'a,IntervalProducts>")
  t=t.replace('inner:std::rc::Rc::new(std::cell::RefCell::new(IntervalProducts::new(r,m as u64,cap)?))', 'inner:EngineStorage::Owned(std::cell::RefCell::new(IntervalProducts::new(r,m as u64,cap)?))')
  t=t.replace('struct RecoveryModes{', "struct RecoveryModes<'a>{").replace('products:RecoveryProducts}', "products:RecoveryProducts<'a>}")
 if f.name=='reciprocal-control.rs':
  t=t.replace('products:RecoveryProducts}', "products:RecoveryProducts<'static>}")
 if f.name=='shared-rescue.rs':
  t=t.replace('fn for_doubled(&self,stream:&StreamModes)->Result<Self,Error>', "fn for_doubled(&self,stream:&StreamModes)->Result<RecoveryProducts<'_>,Error>")
  t=t.replace('Ok(Self{r:self.r,m:stream.m,cap:self.cap,inner:std::rc::Rc::clone(&self.inner)})', 'Ok(RecoveryProducts{r:self.r,m:stream.m,cap:self.cap,inner:EngineStorage::Borrowed(&self.inner)})')
  t+='\ninclude!("borrowed-checks.rs");\n'
 if f.name=='shared-rescue-checks.rs':
  t=t.replace('std::rc::Rc::ptr_eq(&base.inner,&shared.inner)', 'std::ptr::eq(&*base.inner,&*shared.inner)')
  t=t.replace('assert_eq!(std::rc::Rc::strong_count(&base.inner),2);', 'assert!(matches!(base.inner,EngineStorage::Owned(_)));assert!(matches!(shared.inner,EngineStorage::Borrowed(_)));')
  t=t.replace('assert_eq!(std::rc::Rc::strong_count(&base.inner),1);', 'assert!(base.inner.try_borrow_mut().is_ok());')
 if f.name=='new-recovery.rs':t=t.replace('fn main(){','fn main(){borrowed_checks();',1)
 (p/f.name).write_text(t);outs.append(f.name)
(p/'generation.json').write_text(json.dumps({'input_hashes':pins,'output_hashes':{n:hashlib.sha256((p/n).read_bytes()).hexdigest() for n in outs}},indent=2)+'\n')
