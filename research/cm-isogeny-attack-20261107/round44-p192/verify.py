"""Independent point arithmetic, labels, canonicalization and extraction."""
from pathlib import Path
import json,hashlib
c=json.loads(Path('certificate.json').read_text());p=int(c['p']);r=int(c['r']);mu=int(c['mu']);G=c['G'];Q=c['Q']
def add(P,Q):
 if P is None:return Q
 if Q is None:return P
 x,y=P;u,v=Q
 if x==u and (y+v)%p==0:return None
 s=((3*x*x-3)*pow(2*y,-1,p) if P==Q else (v-y)*pow(u-x,-1,p))%p
 X=(s*s-x-u)%p;return [X,(s*(x-X)-y)%p]
def mul(k,P):
 out=None
 while k:
  if k&1:out=add(out,P)
  P=add(P,P);k//=2
 return out
def point(a,b):return add(mul(a,G),mul(b,Q))
def canon(T):
 if T[0] is None:return T,0,1
 j=min(range(len(T)),key=lambda i:T[i][0]);s=1 if T[j][1]<p-T[j][1] else -1
 out=T[j:]+T[:j]
 if s==-1:out=[[x,(-y)%p] for x,y in out]
 return out,j,s
def key(P):return b'identity' if P is None else P[0].to_bytes(24,'big')+P[1].to_bytes(24,'big')
assert pow(mu,5,r)==1 and mu!=1
for T,(a,b) in zip(c['table5'],c['table_coefficients']):
 assert T[0]==point(a,b)
 assert T==[mul(pow(mu,j,r),T[0]) for j in range(5)]
transitions=0;point_updates=0;fruitless=0
for row in c['walks']:
 m=row['arity'];B=row['walkers'];states=row['initial']['states'];labels=row['initial']['labels'];seen=[]
 for T,L in zip(states,labels):
  assert T[0]==point(*L) and canon(T)[0]==T
  assert T==[mul(pow(mu,j,r),T[0]) for j in range(m)]
  seen.append({key(T[0]):(0,L)})
 repeats=[]
 for step,rec in enumerate(row['steps'],1):
  nextstates=[];nextlabels=[];negatives=0
  for i,(T,L,idx) in enumerate(zip(states,labels,rec['indices'])):
   assert idx==int.from_bytes(hashlib.sha256(key(T[0])).digest()[:8],'big')%16
   U=c['table5'][idx][:m];summed=[add(P,R) for P,R in zip(T,U)]
   V,j,sign=canon(summed);z=sign*pow(mu,j,r)%r;a,b=L;d,e=c['table_coefficients'][idx];LL=[z*(a+d)%r,z*(b+e)%r]
   assert V==rec['states'][i] and LL==rec['labels'][i] and [j,sign]==rec['orientations'][i]
   if sign==-1:negatives+=1
   k=key(V[0])
   if k in seen[i]:
    prior,oldL=seen[i][k];repeats.append({'walker':i,'step':step,'previous':prior,'length':step-prior})
    assert LL==oldL;fruitless+=1
   else:seen[i][k]=(step,LL)
   nextstates.append(V);nextlabels.append(LL);transitions+=1;point_updates+=m
  assert set(rec['branches'])=={'generic'}
  assert rec['addition_cost']=={'M':5*m*B-3,'S':m*B,'I':1,'A':6*m*B,'K':0}
  expected=dict(rec['addition_cost']);expected['A']+=B+(m-1)*negatives;assert rec['total_cost']==expected
  states,labels=nextstates,nextlabels
 assert repeats==row['repeats']
for control in c['planted_collision_controls']:
 L=[];Ts=[]
 for a,b in control['input_labels']:
  R=point(a,b);T,j,sign=canon([mul(pow(mu,j,r),R) for j in range(5)]);z=sign*pow(mu,j,r)%r
  Ts.append(T);L.append([a*z%r,b*z%r])
 assert Ts[0]==Ts[1] and L==control['canonical_labels']
 (a,b),(d,e)=L;assert (e-b)%r
 x=(a-d)*pow((e-b)%r,-1,r)%r
 assert x==control['recovered'] and mul(x,G)==Q
for (P,R),V in zip(c['branch_control']['pairs'],c['branch_control']['outputs']):assert add(P,R)==V
out={'all_checks_pass':True,'labelled_transitions':transitions,'point_additions_replayed':point_updates,'planted_scalar_recoveries':10,'generic_batch_count_checks':72,'fruitless_repeat_records':fruitless,'zero_denominator':'identical labels imply zero denominator; extraction must reject','scope':'Finite deterministic mechanism verification; no random-walk performance assertion'}
Path('verification.json').write_text(json.dumps(out,indent=2)+'\n');print(json.dumps(out,indent=2))
