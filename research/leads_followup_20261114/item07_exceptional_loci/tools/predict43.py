"""Prediction for F_43 from the integer cubic Q alone (no closure computed). Writes ../data/pred43.json"""
import sys,json
sys.path.insert(0,'/home/user/crypto/research/leads_chase_20261112/tools')
from unit_test import s3_coeffs
p=43
def Q(al,be):
    A=(al+3)%p; B=(be-2)%p
    return (10384*A*A*B+5377*A**3+6548*A*B*B+1352*B**3-84*(2*B*B+A*B-A*A))%p
def valid(al,be):
    a,b,t=al,be,1
    return (4*a**3+27*b*b)%p!=0 and len(s3_coeffs(p,a,b,t))==9 and (t**3+a*t+b)%p!=0
pred={}
inv=lambda x:pow(x,p-2,p)
for al in range(p):
    for be in range(p):
        if not valid(al,be): continue
        tan0=(al+3)%p==0; q0=Q(al,be)==0
        if tan0 and q0: d=600
        elif tan0: d=603
        elif q0:
            m=(be-2)*inv(al+3)%p
            d=599 if m==(-2)%p else (607 if m==(-5*inv(4))%p else 602)
        else: continue
        pred[f'{al},{be}']=d
json.dump(pred,open('../data/pred43.json','w'),sort_keys=True,indent=0)
import collections
print(len(pred),collections.Counter(pred.values()))
print('Q points valid not tan0:',sum(1 for k,v in pred.items() if v in (599,602,607)))
print('nodal point (alpha,beta)=(-3,2) mod 43 =',(40,2),'valid?',valid(40,2))
