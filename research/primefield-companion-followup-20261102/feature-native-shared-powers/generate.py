from pathlib import Path
import json,hashlib
p=Path(__file__).resolve().parent;src=p.parent/'feature-native-compiled-division';pins={};outs=[]
for name in ['schedule.rs','division.rs','lib.rs','polynomial.rs','class-source.rs','class-checks.rs','check.rs','schedule-checks.rs','generate-recovery.py','validate.py','validate-recovery.py','new-recovery.rs']:
 f=src/name;t=f.read_text();pins[str(f)]=hashlib.sha256(f.read_bytes()).hexdigest()
 if name=='schedule.rs':
  t=t.replace('stats:Stats}', 'stats:Stats,pub(crate) power_flags:Vec<u8>}')
  t=t.replace('Ok(Self{steps,outputs,stats})','''let mut power_flags=vec![0u8;ids.len()];
  for step in &steps{match *step{Step::Odd{b,d,..}=>{power_flags[b]|=3;power_flags[d]|=3},Step::Even{c,e,..}=>{power_flags[c]|=1;power_flags[e]|=1}}}
  Ok(Self{steps,outputs,stats,power_flags})''')
  t=t.replace(' pub fn stats(&self)', ''' pub fn power_counts(&self)->(u64,u64){(self.power_flags.iter().filter(|f|**f&1!=0).count()as u64,self.power_flags.iter().filter(|f|**f&2!=0).count()as u64)}
 pub fn saved_multiplications(&self)->u128{let(s,c)=self.power_counts();2*self.stats.even as u128+4*self.stats.odd as u128-s as u128-c as u128}
 pub fn stats(&self)''')
 if name=='division.rs':
  t=t.replace('  for step in &self.steps{', '''  let mut squares=vec![c(0);self.stats().nodes];let mut cubes=vec![c(0);self.stats().nodes];
  let prepare=|i:usize,v:F,squares:&mut Vec<F>,cubes:&mut Vec<F>|{let flags=self.power_flags[i];if flags&1!=0{squares[i]=v.sq()}if flags&2!=0{cubes[i]=squares[i]*v}};
  for i in 0..5{prepare(i,values[i],&mut squares,&mut cubes)}
  for step in &self.steps{''')
  t=t.replace('values[b].cube()','cubes[b]').replace('values[d].cube()','cubes[d]').replace('values[c].sq()','squares[c]').replace('values[e].sq()','squares[e]')
  t=t.replace('};values.push(z)}', '};prepare(values.len(),z,&mut squares,&mut cubes);values.push(z)}')
 if name=='schedule-checks.rs':
  t=t.replace('fn schedule_checks(){', '#[path="../feature-native-compiled-division/division.rs"]mod unshared;\nfn schedule_checks(){')
  t=t.replace('let st=schedule.stats();', '''let st=schedule.stats();let (sq,cu)=schedule.power_counts();assert!(cu<=sq);assert_eq!(schedule.saved_multiplications(),2*st.even as u128+4*st.odd as u128-sq as u128-cu as u128);
  let mut expected=vec![0u8;st.nodes];for step in &schedule.steps{match *step{Step::Odd{b,d,..}=>{expected[b]|=3;expected[d]|=3},Step::Even{c,e,..}=>{expected[c]|=1;expected[e]|=1}}}assert_eq!(schedule.power_flags,expected);
  let reference=unshared::Schedule::new(requests,4096).unwrap();''')
  t=t.replace('assert_eq!(got.len(),requests.len());','assert_eq!(got.len(),requests.len());assert_eq!(got,reference.evaluate(p,a,b,x,y,unshared::Mode::Normalized).unwrap());assert_eq!(full,reference.evaluate(p,a,b,x,y,unshared::Mode::Full).unwrap());')
  t=t.replace(' // Cost comparison', ''' let mut bound_checks=0;let mut strict=0;
 for n in 0..4096{let s=Schedule::new(&[n],4096).unwrap();let st=s.stats();let(sq,cu)=s.power_counts();assert!(sq+cu<=2*st.even+4*st.odd);if s.saved_multiplications()>0{strict+=1}bound_checks+=1;}
 assert!(strict>0);println!("shared_power_bound_checks={bound_checks} strict_saving_exists=true");
 // Cost comparison''')
 (p/name).write_text(t);outs.append(name)
(p/'generation.json').write_text(json.dumps({'input_hashes':pins,'output_hashes':{n:hashlib.sha256((p/n).read_bytes()).hexdigest() for n in outs}},indent=2)+'\n')
