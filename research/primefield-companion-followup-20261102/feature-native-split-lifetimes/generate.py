from pathlib import Path
import json,hashlib
p=Path(__file__).resolve().parent;src=p.parent/'feature-native-recycled-powers';pins={};outs=[]
for name in ['schedule.rs','division.rs','lib.rs','polynomial.rs','class-source.rs','class-checks.rs','check.rs','schedule-checks.rs','generate-recovery.py','validate.py','validate-recovery.py','new-recovery.rs']:
 f=src/name;t=f.read_text();pins[str(f)]=hashlib.sha256(f.read_bytes()).hexdigest()
 if name=='schedule.rs':
  t=t.replace('slots:Vec<usize>', 'slots:Vec<[usize;3]>').replace('allocate(&steps,&outputs,ids.len())','allocate(&steps,&outputs,&power_flags)').replace('live_slots','field_slots')
 if name=='division.rs':
  t=t.replace('live_slots','field_slots').replace('live_cap','field_cap')
  t=t.replace('  let mut squares=vec![c(0);self.field_slots()];let mut cubes=vec![c(0);self.field_slots()];\n','')
  a=t.index('  let prepare=');b=t.index('\n  for(i,v)',a)
  t=t[:a]+'''  let prepare=|i:usize,v:F,values:&mut Vec<F>|{let flags=self.power_flags[i];let slots=self.slots[i];if flags&1!=0{values[slots[1]]=v.sq()}if flags&2!=0{values[slots[2]]=values[slots[1]]*v}};'''+t[b:]
  t=t.replace('values[self.slots[i]]=v;prepare(i,v,&mut squares,&mut cubes)', 'values[self.slots[i][0]]=v;prepare(i,v,&mut values)')
  t=t.replace('cubes[b]','values[b]').replace('cubes[d]','values[d]').replace('squares[c]','values[c]').replace('squares[e]','values[e]')
  t=t.replace('prepare(i+5,z,&mut squares,&mut cubes);values[self.slots[i+5]]=z','prepare(i+5,z,&mut values);values[self.slots[i+5][0]]=z')
  t=t.replace('values[self.slots[i]]','values[self.slots[i][0]]')
 if name=='schedule-checks.rs':
  t=t.replace('schedule.live_slots()','schedule.field_slots()')
  t=t.replace('let dense=unrecycled::Schedule', 'let bundle=bundled::Schedule::new(requests,4096).unwrap();let dense=unrecycled::Schedule')
  t=t.replace('assert_eq!(got.len(),requests.len());','assert_eq!(got.len(),requests.len());assert_eq!(got,bundle.evaluate(p,a,b,x,y,bundled::Mode::Normalized).unwrap());assert_eq!(full,bundle.evaluate(p,a,b,x,y,bundled::Mode::Full).unwrap());')
 (p/name).write_text(t);outs.append(name)
(p/'generation.json').write_text(json.dumps({'input_hashes':pins,'output_hashes':{n:hashlib.sha256((p/n).read_bytes()).hexdigest() for n in outs}},indent=2)+'\n')
