from pathlib import Path
import json
c=json.loads(Path('certificate.json').read_text());r=c['r'];checks=0
for p,x,rows in [(r,None,c['fixtures']),(11,3,c['toy'])]:
 for z in rows:
  xx=z.get('public_secret',x);e=z['exponent'];L=(z['a']+xx*z['b'])%p;v=pow(L,e,p);da=e*pow(L,e-1,p)%p;db=xx*da%p
  assert z['N']==[(v+2)%p,da,db] and z['D']==[(v+1)%p,da,db]
  assert z['Wa']==-da%p and z['Wb']==-db%p
  assert z['recovered']==(xx if da else None)
  m=e.bit_length()-1+bin(e).count('1');expected={'M':5*m+4+int(da!=0),'A':2*m+11,'I':int(da!=0)}
  assert z['cost']==expected,(z['cost'],expected);checks+=1
f=c['frobenius'];assert f['N'][1:]==f['D'][1:]==[0,0] and f['recovered'] is None
n=c['negative'];assert n['Wa']!=0 and n['recovered']==pow(3,4,r)*pow(pow(2,4,r),-1,r)%r and n['recovered']!=17
# Exhaustive finite-grid descent, considering only pairs where D!=0.
fibers={}
for z in c['toy']:
 if z['D'][0]:fibers.setdefault((z['a']+3*z['b'])%11,set()).add(z['N'][0]*pow(z['D'][0],-1,11)%11)
assert all(len(v)==1 for v in fibers.values())
out={'all_checks_pass':True,'independent_derivative_and_cost_checks':checks,'toy_fibers_checked':len(fibers),'toy_pole_extractions':sum(z['D'][0]==0 and z['recovered']==3 for z in c['toy']),'negative_descent_control':True,'frobenius_degree_control':True,'universal_theorem':'written proof, not machine-checked'}
Path('verification.json').write_text(json.dumps(out,indent=2)+'\n');print(json.dumps(out,indent=2))
