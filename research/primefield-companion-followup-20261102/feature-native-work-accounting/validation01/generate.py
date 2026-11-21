"""Copy the fixed predecessor and instrument only enumerated entry points."""
from pathlib import Path
import hashlib,json
D=Path(__file__).resolve().parent;S=D.parent/'feature-native-planned-width'
sha=lambda b:hashlib.sha256(b).hexdigest()
records=[]
for src in sorted(S.glob('*.rs'))+[D.parent/'feature-native-hgcd-roots/polynomial.rs']:
 name='metered-polynomial.rs' if src.parent!=S else src.name
 original=src.read_text();t=original;edits=[]
 def edit(old,new):
  global t
  assert t.count(old)==1,(name,old,t.count(old))
  t=t.replace(old,new);edits.append(dict(old=old,new=new,occurrences=1))
 if name=='modes.rs':
  edit('#[path="../feature-native-hgcd-roots/polynomial.rs"] mod polynomial;','#[path="metered-polynomial.rs"] mod polynomial;\nmod meter;\ninclude!("meter-checks.rs");')
  edit('fn mul(a:u64,b:u64,r:u64)->u64{','fn mul(a:u64,b:u64,r:u64)->u64{meter::hit(0);')
  edit('fn pow(mut a:u64,mut n:u64,r:u64)->u64{','fn pow(mut a:u64,mut n:u64,r:u64)->u64{if n==r-2{meter::hit(3);}')
 if name=='metered-polynomial.rs':
  edit('fn fm(&self,a:u64,b:u64)->u64{','fn fm(&self,a:u64,b:u64)->u64{crate::meter::hit(1);')
  edit('fn inverse(&self,a:u64)->Result<u64,Error>{','fn inverse(&self,a:u64)->Result<u64,Error>{crate::meter::hit(2);')
 if name=='interval.rs':
  edit('fn products(&mut self,requests:&[(u64,u64)])->Result<Vec<u64>,Error>{','fn products(&mut self,requests:&[(u64,u64)])->Result<Vec<u64>,Error>{\n  let _region=meter::Region::enter();')
 if name=='new-recovery.rs':
  edit('fn main(){','fn inherited_checks(){')
 (D/name).write_text(t)
 records.append(dict(source=str(src.relative_to(D.parent)),output=name,source_sha256=sha(src.read_bytes()),generated_sha256=sha(t.encode()),edits=edits))
(D/'generation.json').write_text(json.dumps(records,indent=2)+'\n')
