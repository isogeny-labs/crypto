from pathlib import Path
import json,hashlib
p=Path(__file__).resolve().parent
for n,h in json.loads((p/'EVIDENCE_SHA256.json').read_text()).items():assert hashlib.sha256(Path(n).read_bytes()).hexdigest()==h,n
r=json.loads(Path('research/torsion_cut_20261116_v74/raw.json').read_text());assert r['error']=="ModuleNotFoundError: No module named 'lab.research.odd_solver'"
cases=r['out']['chains']['cases'];ok=[v for v in cases if v['A']==0];assert len(ok)==8 and sum(v['original_roots']for v in ok)==1635 and all(v['rational_join_equal']for v in ok)
assert r['out']['chains']['control_nonrational_witness']['target_x']==19
def trim(a):
 while a and a[-1]==0:a.pop()
 return a
def mul(a,b,q):
 c=[0]*max(0,len(a)+len(b)-1)
 for i,x in enumerate(a):
  for j,y in enumerate(b):c[i+j]=(c[i+j]+x*y)%q
 return trim(c)
def rem(a,F,q):
 a=list(a)
 while len(a)>=len(F):
  v=a[-1];j=len(a)-len(F)
  for i,x in enumerate(F):a[i+j]=(a[i+j]-v*x)%q
  trim(a)
 return a
s=json.loads(Path('research/torsion_cut_nist_20261116_v74b/raw.json').read_text());assert s['error']is None and len(s['out']['fields'])==5
for v in s['out']['fields']:
 q=v['p'];F=v['F'];power=[1];base=[0,1];e=q
 while e:
  if e&1:power=rem(mul(power,base,q),F,q)
  e//=2
  if e:base=rem(mul(base,base,q),F,q)
 while len(power)<2:power.append(0)
 power[1]=(power[1]-1)%q;assert trim(power)==v['h']
 a=mul(v['U'],F,q);b=mul(v['V'],v['h'],q);z=[0]*max(len(a),len(b))
 for i,x in enumerate(a):z[i]=(z[i]+x)%q
 for i,x in enumerate(b):z[i]=(z[i]+x)%q
 assert trim(z)==[1]
print('C74 hashes, preserved failed dependency, binary joins and five independent Bezout identities verified')
