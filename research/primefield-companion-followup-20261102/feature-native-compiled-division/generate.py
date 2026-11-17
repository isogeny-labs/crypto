from pathlib import Path
import json,hashlib
p=Path(__file__).resolve().parent;src=p.parent/'feature-native-normalized-division';base=src/'division.rs';s=base.read_text();start=s.index('pub struct ValueDivision');s=s[:start]
s=s.replace('use std::collections::BTreeMap;','include!("schedule.rs");')
s+='''impl Schedule{
 pub fn evaluate(&self,p:u64,a:u64,b:u64,x:u64,y:u64,mode:Mode)->Result<Vec<u64>,Error>{
'''
t=base.read_text();a=t.index('  if p<5');b=t.index('  let w4=',a);s+=t[a:b]
s+='''  let body=x6+c(5)*a*x4+c(20)*b*x3-c(5)*a2*x2-c(4)*a*b*x-c(8)*b.sq()-a2*a;
  let (w2,w4,z4,inv)=match mode{
   Mode::Normalized=>(c(1),c(2)*body,two_y.sq().sq(),c(1)),
   Mode::Full=>{let(mut r,mut s,mut u,mut v)=(p as i128,two_y.v as i128,0i128,1i128);while s!=0{let q=r/s;(r,s)=(s,r-q*s);(u,v)=(v,u-q*v);}if r!=1{return Err(Error::Nonunit)}(two_y,c(4)*y*body,c(1),c(u.rem_euclid(p as i128)as u64))}
  };
  let mut values=Vec::with_capacity(self.stats().nodes);values.extend([c(0),c(1),w2,w3,w4]);
  for step in &self.steps{let z=match *step{
   Step::Odd{a,b,c,d,half_even}=>{let left=values[a]*values[b].cube();let right=values[c]*values[d].cube();if mode==Mode::Full{left-right}else if half_even{z4*left-right}else{left-z4*right}},
   Step::Even{a,b,c,d,e}=>{let term=values[b]*values[c].sq()-values[d]*values[e].sq();if mode==Mode::Full{(values[a]*inv)*term}else{values[a]*term}}
  };values.push(z)}
  Ok(self.outputs.iter().map(|&(i,even)|if mode==Mode::Normalized&&even{(values[i]*two_y).v}else{values[i].v}).collect())
 }
}
'''
(p/'division.rs').write_text(s)
pins={str(base):hashlib.sha256(base.read_bytes()).hexdigest()};outs=['division.rs']
for name in ['lib.rs','polynomial.rs','class-source.rs','class-checks.rs','check.rs','generate-recovery.py','validate.py','validate-recovery.py','new-recovery.rs']:
 f=src/name;t=f.read_text();pins[str(f)]=hashlib.sha256(f.read_bytes()).hexdigest()
 if name=='lib.rs':
  t=t.replace('mod division;','pub mod division;').replace('calibration:u64}', 'calibration:u64,schedule:division::Schedule,mode:division::Mode}')
  sig=' pub fn new(p:u64,a:u64,b:u64,r:u64,base:[u64;2],max_lanes:usize)->Result<Self,Error>{'
  t=t.replace(sig,sig+'Self::build(p,a,b,r,base,max_lanes,None,4096)}\n pub fn new_with_costs(p:u64,a:u64,b:u64,r:u64,base:[u64;2],max_lanes:usize,costs:division::CostModel,node_cap:usize)->Result<Self,Error>{Self::build(p,a,b,r,base,max_lanes,Some(costs),node_cap)}\n fn build(p:u64,a:u64,b:u64,r:u64,base:[u64;2],max_lanes:usize,costs:Option<division::CostModel>,node_cap:usize)->Result<Self,Error>{')
  t=t.replace('  let mut out=Self{', '  let mut requests=vec![r,r+1,2,r+2,r/2+1,2*w];if s!=0{requests.push(s+partition[3])}let schedule=division::Schedule::new(&requests,node_cap).map_err(|_|Error::Domain)?;let mode=costs.map_or(division::Mode::Normalized,|c|schedule.stats().choose(c));\n  let mut out=Self{')
  t=t.replace('partition,calibration:0}', 'partition,calibration:0,schedule,mode}')
  t=t.replace('let mut values=division::ValueDivision::new(p,self.a,self.b,q[0],q[1])','let values=self.schedule.evaluate(p,self.a,self.b,q[0],q[1],self.mode)')
  for old,new in [('values.value(self.r)','values[0]'),('values.value(self.r+1)','values[1]'),('values.value(2)','values[2]'),('values.value(self.r+2)','values[3]')]:t=t.replace(old,new)
  t=t.replace('let mut scaled=|n:u64|','let scaled=|n:u64,v:u64|').replace('class(values.value(n))','class(v)').replace('scaled(self.r/2+1)','scaled(self.r/2+1,values[4])').replace('scaled(2*w)','scaled(2*w,values[5])').replace('scaled(s+e)','scaled(s+e,values[6])')
 if name=='check.rs':
  t=t.replace('include!("division-checks.rs");','include!("schedule-checks.rs");').replace('division_checks();','schedule_checks();')
  t=t.replace('let reference=full::Source', 'let selected=source::Source::new_with_costs(p,a,b,r,base,64,source::division::CostModel{multiplication:1,inversion:0},4096).unwrap();let reference=full::Source')
  t=t.replace('signs+=1;', 'signs+=1;assert_eq!(selected.sign(Some(xy)),Ok(expected));')
 (p/name).write_text(t);outs.append(name)
(p/'generation.json').write_text(json.dumps({'input_hashes':pins,'output_hashes':{n:hashlib.sha256((p/n).read_bytes()).hexdigest() for n in outs}},indent=2)+'\n')
