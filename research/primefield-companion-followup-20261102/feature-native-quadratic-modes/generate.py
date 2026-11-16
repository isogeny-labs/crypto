from pathlib import Path
import hashlib,json
p=Path(__file__).resolve().parent;src=p.parent/'feature-native-reciprocal-c0';pins={};outs=[]
for f in sorted(src.iterdir()):
 if f.suffix!='.rs' and f.name!='validate-recovery.py':continue
 t=f.read_text();pins[str(f)]=hashlib.sha256(f.read_bytes()).hexdigest()
 if f.name=='context.rs':
  old='  let c0=if let Some(g)=&state.dense'
  new='  if state.dense.is_none()&&self.m>=3{if let Some(g)=self.quadratic_initial(a,t,f,self.block)?{state.intersect(p,&g)?;return Ok(())}}\n'+old
  assert old in t;t=t.replace(old,new);t+='\ninclude!("quadratic.rs");\n'
 if f.name=='new-recovery.rs':t=t.replace('fn main(){','fn main(){quadratic_checks();',1)
 # The predecessor counter assertion belongs to its isolated path; retain that exact control.
 if f.name=='reciprocal-checks.rs':t=t.replace('let new=modes.recover_traced(', 'let new=modes.recover_reciprocal_control(')
 if f.name=='stream-adapter.rs':
  control=t[t.index('impl RecoveryModes{'):t.index('impl StreamModes{fn recover_traced')]
  control=control.replace('impl RecoveryModes{','impl ReciprocalControl{')
  t+=control+"\nimpl StreamModes{fn recover_reciprocal_control(&self,source:&source::Source,curve:group::Curve,base:group::Point,target:Option<group::Point>,radii:&[u64],cap:usize,extra:&[u64],max_calls:usize)->Result<(Option<u64>,PullStats,&'static str),Error>{ReciprocalControl::new(self)?.recover_traced(source,curve,base,target,radii,cap,extra,max_calls)}}\n"
 (p/f.name).write_text(t);outs.append(f.name)
# Preserve the predecessor mode body independently as the finite public control.
s=(src/'context.rs').read_text();start=s.index('struct RecoveryModes{');end=s.index('include!("context-checks.rs")');control=s[start:end].replace('RecoveryModes','ReciprocalControl').replace('fn from_modes(','fn new(')
# Delegate only the unchanged reciprocal helper; borrow the same context through a temporary view is unnecessary.
helper=(src/'reciprocal.rs').read_text();helper=helper[helper.index('impl RecoveryModes{'):helper.index('include!("reciprocal-checks.rs")')].replace('RecoveryModes','ReciprocalControl')
# Its model builder accepts RecoveryModes, so provide a generic-free explicit matching constructor.
helper=helper.replace('ReciprocalMode::build(self,','self.reciprocal_model(')
build=(src/'reciprocal.rs').read_text();build=build[build.index(' fn build('):build.index(' fn branch(')].replace('fn build(modes:&RecoveryModes,','fn reciprocal_model(&self,').replace('let r=modes.r;','let modes=self;let r=modes.r;').replace('Result<Option<Self>,Error>','Result<Option<ReciprocalMode>,Error>').replace('Some(Self{r,n,h,dn})','Some(ReciprocalMode{r,n,h,dn})')
(p/'reciprocal-control.rs').write_text(control+helper+'impl ReciprocalControl{'+build+'}\n');outs.append('reciprocal-control.rs')
(p/'generation.json').write_text(json.dumps({'input_hashes':pins,'output_hashes':{n:hashlib.sha256((p/n).read_bytes()).hexdigest() for n in outs}},indent=2)+'\n')
