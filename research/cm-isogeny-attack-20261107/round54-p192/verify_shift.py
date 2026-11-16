from pathlib import Path
from math import gcd
import json
root=Path(__file__).resolve().parent;c=json.loads((root/'P192-shift.json').read_text());p,r,a,b=c['p'],c['r'],c['a'],c['b'];nt=c['twist_order'];P=c['base_field_twist_point']
def add(P,Q):
 if P is None:return Q
 if Q is None:return P
 x,y=P;u,v=Q
 if x==u and (y+v)%p==0:return None
 slope=((3*x*x+a)*pow(2*y,-1,p) if P==Q else (v-y)*pow(u-x,-1,p))%p;X=(slope*slope-x-u)%p
 return [X,(slope*(x-X)-y)%p]
def mul(n,P):
 R=None
 for bit in bin(n)[2:]:
  R=add(R,R)
  if bit=='1':R=add(R,P)
 return R
def iso(P):return None if P is None else [[-P[0]%p,0],[0,P[1]]]
assert p==2**192-2**64-1 and (P[1]*P[1]-P[0]**3-a*P[0]+b)%p==0
assert gcd(r,nt)==gcd(6,nt)==1 and nt==2*p+2-r
assert mul(nt,P) is None and mul(r,P) is not None
assert iso(P)==c['S'] and iso(mul(6,P))==c['R_6S'] and iso(mul(r,P))==c['rS'] and iso(mul(6*r,P))==c['rR']
out={'all_checks_pass':True,'independent_base_field_twist_checks':4,'pole_free_shift_criterion':'rS and r(6S) nonzero','P192_trace_evaluated':False}
(root/'P192-shift-verification.json').write_text(json.dumps(out,indent=2)+'\n');print(json.dumps(out,indent=2))
