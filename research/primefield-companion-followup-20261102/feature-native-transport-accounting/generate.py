"""Instrumentation and path routing only; no cryptographic execution."""
from pathlib import Path
import hashlib,json,re,os
D=Path(__file__).resolve().parent;R=D.parent
paths={'recovery':'transport-recovery/lib.rs','collapse':'quartic-collapse/lib.rs','target':'quartic-collapse/target.rs','recurrence':'quartic-collapse/recurrence.rs','decoder':'centered-bitplanes/decoder.rs','source':'transport-recovery/source.rs','source_extra':'transport-recovery/source-extra.rs','transport':'transport-recovery/transport.rs','transport_extra':'transport-recovery/transport-extra.rs','residual':'multiseed-factor/residual.rs','geometry':'adaptive-extension/geometry.rs','jets':'division/lib.rs','value':'value-division/lib.rs','polynomial':'low-product/polynomial.rs','polyref':'polynomial/lib.rs'}
files={k:R/('feature-native-'+v)for k,v in paths.items()};reverse={p.resolve():k+'.rs'for k,p in files.items()};ledger=[]
def sha(s):return hashlib.sha256(s.encode()).hexdigest()
for name,path in files.items():
 original=path.read_text();s=original;edits=[]
 def change(old,new,first=False):
  global s
  n=s.count(old);assert n,(name,'missing',old)
  if first:s=s.replace(old,new,1);n=1
  else:s=s.replace(old,new)
  edits.append(dict(old=old,new=new,occurrences=n,first_only=first))
 def hit(c,o):return f'crate::meter::hit(crate::meter::Component::{c},crate::meter::Op::{o});'
 def scope(t):return f'let _accounting_scope=crate::meter::granular(crate::meter::Stage::{t});'
 # Route all explicitly imported modules and included fixtures.
 for old in dict.fromkeys(m.group(0)for m in re.finditer(r'#\[path\s*=\s*"([^"]+)"\]',s)):
  rel=re.search(r'"([^"]+)"',old).group(1);p=(path.parent/rel).resolve();change(old,'#[path="'+reverse.get(p,os.path.relpath(p,D/'generated'))+'"]')
 for old in dict.fromkeys(m.group(0)for m in re.finditer(r'(?<!pub )\bmod\s+(\w+)\s*;',s)):
  modname=re.search(r'mod\s+(\w+)',old).group(1);p=(path.parent/(modname+'.rs')).resolve()
  if p.exists() and not re.search(r'#\[path\s*=\s*"[^"]+"\]\s*$',s[:s.index(old)]):change(old,'#[path="'+reverse.get(p,os.path.relpath(p,D/'generated'))+'"]'+old)
 for old in dict.fromkeys(m.group(0)for m in re.finditer(r'include!\("([^"]+)"\)',s)):
  rel=re.search(r'"([^"]+)"',old).group(1);p=(path.parent/rel).resolve();change(old,'include!("'+reverse.get(p,os.path.relpath(p,D/'generated'))+'")')
 for old in dict.fromkeys(m.group(0)for m in re.finditer(r'include_str!\("([^"]+)"\)',s)):
  rel=re.search(r'"([^"]+)"',old).group(1);change(old,'include_str!("'+os.path.relpath((path.parent/rel).resolve(),D/'generated')+'")')
 if name in ('recovery','collapse','target','recurrence','source','transport'):
  c={'recovery':'Compiler','collapse':'Compiler','target':'Target','recurrence':'Recurrence','source':'Source','transport':'Transport'}[name]
  signature='fn mul(a:u64,b:u64,'+('p'if name in ('source','transport')else'r')+':u64)->u64{';change(signature,signature+hit(c,'Product'))
 if name in ('geometry','jets','value'):
  c={'geometry':'Geometry','jets':'Phase','value':'Value'}[name]
  sig='fn mul(self,b:Self)->Self{';change(sig,sig+hit(c,'Product'),True)
  sig='fn inv(self)->Option<Self>{';change(sig,sig+hit(c,'Inverse'),True)
  sig='while s!=0 {'if name=='jets'else'while s!=0{';change(sig,sig+hit(c,'EuclidStep'))
  if name=='jets':change('let exponent=inverse(rr,modulus)','let exponent={'+hit(c,'ScalarInverse')+'inverse(rr,modulus)}')
 if name in ('polynomial','polyref'):
  c='Source'if name=='source'else'Polynomial'
  if name!='source':
   sig='fn fm(&self,a:u64,b:u64)->u64{';change(sig,sig+hit(c,'Product'))
  sig='fn inverse(a:u64,p:u64)->Result<u64,Error>{'if name=='source'else'fn inverse(&self,a:u64)->Result<u64,Error>{';change(sig,sig+hit(c,'Inverse'))
  sig='while s!=0{';change(sig,sig+hit(c,'EuclidStep'))
 if name=='decoder':
  change('active.count_ones()', '{'+hit('Decoder','Popcount')+'active.count_ones()}')
  change('(active & negatives).count_ones()', '{'+hit('Decoder','Popcount')+'(active & negatives).count_ones()}')
 if name=='recovery':
  for sig,stage in [('pub fn construct(r:u64,q:usize,max_extra:usize,max_rows:usize,max_columns:usize)->Result<Self,Error>{','Compile'),('pub fn new(p:u64,a:u64,b:u64,r:u64,base:[u64;2],max_lanes:usize,max_window:usize)->Result<Self,Error>{','Calibration'),("pub fn prepare(&self)->Result<Prepared<'_>,Error>{",'Prepare'),('pub fn recover(&self,p:&Program,point:Option<[u64;2]>)->Result<Recovery,Error>{','Query'),('pub fn evaluate(&self,signs:&[i8],mut oracle:impl FnMut(usize)->Result<i8,Error>)->Result<Output,Error>{','Decode')]:change(sig,sig+scope(stage))
  change('.find(|c|c.signature==signs)','.find(|c|{'+hit('Decoder','RuleCompare')+'c.signature==signs})')
  change('p.evaluate(&initial.signs,|offset|{','p.evaluate(&initial.signs,|offset|{'+scope('Extension'))
 if name=='source':
  sig='fn ratio(&self,q:[u64;2],grid:&[[u64;2]],indices:&[u64],values:&[Pair])->Result<Pair,Error>{';change(sig,sig+hit('Source','Character'))
 if name=='transport':
  sig="pub fn begin<'b>(&'b self,q:Point,length:usize,max_extra:usize)->Result<(Window,Cursor<'b,'a>),Error>{";change(sig,sig+scope('InitialTransport'))
  old='let(joint,tail)=self.geometry.joint_window_tail(q,length).map_err(|_|Error::Geometry)?;';change(old,'let(joint,tail)={'+scope('InitialGeometry')+'self.geometry.joint_window_tail(q,length).map_err(|_|Error::Geometry)?};')
 if name=='transport_extra':
  sig='pub fn advance(self)->Result<(Extension,Self),Error>{';change(sig,sig+scope('ExtraTransport'))
  old='let(next,tail)=self.tail.advance().map_err(|_|Error::Geometry)?;';change(old,'let(next,tail)={'+scope('ExtraGeometry')+'self.tail.advance().map_err(|_|Error::Geometry)?};')
 if name=='source_extra':
  sig='fn consume(&self,w:transport::Window)->Result<Window,Error>{';change(sig,sig+scope('InitialCharacter'))
  old='let(ratio,sign)=if evaluate{';change(old,old+scope('ExtraCharacter'))

 reverted=s
 for e in reversed(edits):reverted=reverted.replace(e['new'],e['old'],1)if e['first_only']else reverted.replace(e['new'],e['old'])
 assert reverted==original,(name,'inverse transform')
 (D/'generated'/f'{name}.rs').write_text(s)
 ledger.append(dict(module=name,original=str(path.relative_to(R)),original_sha256=sha(original),generated_sha256=sha(s),edits=edits))
(D/'instrumentation.json').write_text(json.dumps(dict(schema='native-instrumentation-map/v1',modules=ledger),indent=2)+'\n')
print('generated',len(ledger),'modules; exact reverse transforms checked')
