from pathlib import Path
import json,hashlib
p=Path(__file__).resolve().parent;src=p.parent/'feature-native-shared-powers';pins={};outs=[]
for name in ['schedule.rs','division.rs','lib.rs','polynomial.rs','class-source.rs','class-checks.rs','check.rs','schedule-checks.rs','generate-recovery.py','validate.py','validate-recovery.py','new-recovery.rs']:
 f=src/name;t=f.read_text();pins[str(f)]=hashlib.sha256(f.read_bytes()).hexdigest()
 if name=='schedule.rs':
  t=t.replace('power_flags:Vec<u8>}', 'power_flags:Vec<u8>,pub(crate) slots:Vec<usize>,pub(crate) physical_steps:Vec<Step>,live_slots:usize}')
  t=t.replace('impl Schedule{','include!("allocation.rs");\nimpl Schedule{')
  t=t.replace('Ok(Self{steps,outputs,stats,power_flags})','let(slots,physical_steps,live_slots)=allocate(&steps,&outputs,ids.len());Ok(Self{steps,outputs,stats,power_flags,slots,physical_steps,live_slots})')
  t=t.replace(' pub fn power_counts(', ' pub fn live_slots(&self)->usize{self.live_slots}\n pub fn power_counts(')
 if name=='division.rs':
  t=t.replace('ZeroY,Nonunit}', 'ZeroY,Nonunit,Workspace}')
  sig=' pub fn evaluate(&self,p:u64,a:u64,b:u64,x:u64,y:u64,mode:Mode)->Result<Vec<u64>,Error>{'
  t=t.replace(sig,sig+'self.evaluate_bounded(p,a,b,x,y,mode,usize::MAX)}\n pub fn evaluate_bounded(&self,p:u64,a:u64,b:u64,x:u64,y:u64,mode:Mode,live_cap:usize)->Result<Vec<u64>,Error>{\n  if self.live_slots()>live_cap{return Err(Error::Workspace)}')
  t=t.replace('let mut values=Vec::with_capacity(self.stats().nodes);values.extend([c(0),c(1),w2,w3,w4]);','let mut values=vec![c(0);self.live_slots()];')
  t=t.replace('vec![c(0);self.stats().nodes]', 'vec![c(0);self.live_slots()]')
  t=t.replace('let flags=self.power_flags[i];','let flags=self.power_flags[i];let i=self.slots[i];')
  t=t.replace('for i in 0..5{prepare(i,values[i],&mut squares,&mut cubes)}','for(i,v)in [c(0),c(1),w2,w3,w4].into_iter().enumerate(){values[self.slots[i]]=v;prepare(i,v,&mut squares,&mut cubes)}')
  t=t.replace('for step in &self.steps{','for(i,step)in self.physical_steps.iter().enumerate(){')
  t=t.replace('prepare(values.len(),z,&mut squares,&mut cubes);values.push(z)', 'prepare(i+5,z,&mut squares,&mut cubes);values[self.slots[i+5]]=z')
  t=t.replace('(values[i]*two_y).v}else{values[i].v}', '(values[self.slots[i]]*two_y).v}else{values[self.slots[i]].v}')
 if name=='schedule-checks.rs':
  t=t.replace('fn schedule_checks(){','include!("liveness-checks.rs");\n#[path="../feature-native-shared-powers/division.rs"]mod unrecycled;\nfn schedule_checks(){liveness_checks();')
  t=t.replace('let reference=unshared::Schedule', 'let dense=unrecycled::Schedule::new(requests,4096).unwrap();let reference=unshared::Schedule')
  t=t.replace('assert_eq!(got.len(),requests.len());', 'assert_eq!(got.len(),requests.len());assert_eq!(got,dense.evaluate(p,a,b,x,y,unrecycled::Mode::Normalized).unwrap());assert_eq!(full,dense.evaluate(p,a,b,x,y,unrecycled::Mode::Full).unwrap());assert_eq!(schedule.evaluate_bounded(p,a,b,x,y,Mode::Normalized,schedule.live_slots()).unwrap(),got);assert_eq!(schedule.evaluate_bounded(p,a,b,x,y,Mode::Full,schedule.live_slots()-1),Err(compiled::Error::Workspace));')
 (p/name).write_text(t);outs.append(name)
(p/'generation.json').write_text(json.dumps({'input_hashes':pins,'output_hashes':{n:hashlib.sha256((p/n).read_bytes()).hexdigest() for n in outs}},indent=2)+'\n')
