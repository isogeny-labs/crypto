from pathlib import Path
import hashlib,json
D=Path(__file__).resolve().parent;S=D.parent/'feature-native-progression-inverse';rows=[]
for f in sorted(S.glob('*.rs')):
 old=f.read_text();t=old;edits=[]
 def sub(a,b):
  global t
  assert t.count(a)==1,(f.name,a,t.count(a));t=t.replace(a,b);edits.append([a,b])
 if f.name=='modes.rs':
  sub('a=mul(a,a,r);n>>=1}v}','n>>=1;if n!=0{a=mul(a,a,r)}}v}')
  sub('let mut ids=vec![0;steps];\n    for j in(0..steps).rev(){ids[j]=mul(inv,prefix[j],r);inv=mul(inv,ds[j],r)}','let mut ids=ds;\n    finish_batch_inverses(&mut ids,&prefix,inv,r)?;')
  sub('mod meter;','mod meter;\ninclude!("inverse-tail.rs");')
 if f.name in ['context.rs','streamed.rs']:
  sub('for i in(0..steps).rev(){let den=ds[i];ds[i]=mul(inv,prefix[i],self.r);inv=mul(inv,den,self.r)}','finish_batch_inverses(&mut ds,&prefix,inv,self.r)?;')
 if f.name=='progression.rs':
  sub('for i in(0..n).rev(){let value=denominators[i];denominators[i]=mul(inv,prefix[i],r);inv=mul(inv,value,r)}','finish_batch_inverses(&mut denominators,&prefix,inv,r)?;')
 if f.name=='meter-checks.rs':
  sub('assert_eq!(s.counts[0],13);','assert_eq!(s.counts[0],12);')
  sub('fermat_products=13','fermat_products=12')
 if f.name=='inverse-checks.rs':
  sub('fn main(){','fn progression_predecessor_main(){')
  t+='\ninclude!("terminal-checks.rs");\n';edits.append(['','\ninclude!("terminal-checks.rs");\n'])
 (D/f.name).write_text(t);rows.append({'source':'../feature-native-progression-inverse/'+f.name,'output':f.name,'source_sha256':hashlib.sha256(old.encode()).hexdigest(),'sha256':hashlib.sha256(t.encode()).hexdigest(),'edits':edits})
(D/'generation.json').write_text(json.dumps(rows,indent=2)+'\n')
