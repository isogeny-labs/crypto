from pathlib import Path
import json
from action import *
from cheon import solve
root=Path(__file__).resolve().parent;c=json.loads((root.parent/'round53-p192/certificate.json').read_text());d=json.loads((root.parent/'round54-p192/certificate.json').read_text())
G=read(d['curve']['G']);T=read(d['curve']['T']);R0=mul(6,read(d['shift_generator']));Er=[add(mul(i,G),mul(j,T)) for i in range(7) for j in range(7)];psi,reps=division_from_torsion(Er);rows=[]
for target in c['targets']:
 Q=tuple(K(F(v)) for v in target['Q']);H=[]
 for j in range(7):
  A,B=mul(j,G),mul(j,Q)
  for U,V in target['dual_pairs']:
   H.append((add(A,None if U is None else tuple(K(F(v)) for v in U)),add(B,None if V is None else tuple(K(F(v)) for v in V))))
 A,B=numerator(H,R0,psi,reps);actions=[]
 for j in range(1,7):
  U=mul(j,G);P,receipt=extract(U,A,B,psi,R0);assert P==mul(j,Q)
  actions.append({'input':code(U),'output':code(P),'receipt':receipt})
 solves=[];rejections=0
 for divisor in [2,3,6]:
  Pd=Q
  for _ in range(divisor-1):Pd=extract(Pd,A,B,psi,R0)[0]
  scalar,receipt=solve(G,Q,Pd,divisor);assert mul(scalar,G)==Q
  try:solve(G,Q,add(Pd,G),divisor)
  except ValueError:rejections+=1
  else:raise AssertionError('bad auxiliary input accepted')
  solves.append({'divisor':divisor,'auxiliary':code(Pd),'action_extractions':divisor-1,'recovered':scalar,'receipt':receipt})
 rows.append({'Q':code(Q),'A':[v.code() for v in A],'B':[v.code() for v in B],'A_degree':max(i for i,v in enumerate(A) if v),'B_degree':max(i for i,v in enumerate(B) if v),'actions':actions,'scalar_recoveries':solves,'bad_auxiliary_rejections':rejections})
 print('TARGET',code(Q),'degrees',rows[-1]['A_degree'],rows[-1]['B_degree'],'recoveries',[v['recovered'] for v in solves],flush=True)
out={'all_checks_pass':True,'r':7,'G':code(G),'R0':code(R0),'psi':[v.code() for v in psi],'targets':rows,'scope':'dense numerator and supplied checked kernel; auxiliary-power extraction and Cheon stages counted, not an end-to-end speed comparison'}
(root/'certificate.json').write_text(json.dumps(out,separators=(',',':'))+'\n')
