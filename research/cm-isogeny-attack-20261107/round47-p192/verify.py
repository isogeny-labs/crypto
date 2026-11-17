from pathlib import Path
import json
c=json.loads(Path('certificate.json').read_text());r=c['r'];p=2**192-2**64-1
G=[int('188DA80EB03090F67CBF20EB43A18800F4FF0AFD82FF1012',16),int('07192B95FFC8DA78631011ED6B24CDD573F977A11E794811',16)]
def add(P,Q):
 if P is None:return Q
 if Q is None:return P
 x,y=P;u,v=Q
 if x==u and (y+v)%p==0:return None
 s=((3*x*x-3)*pow(2*y,-1,p) if P==Q else (v-y)*pow(u-x,-1,p))%p
 z=(s*s-x-u)%p;return [z,(s*(x-z)-y)%p]
def mul(n,P):
 n%=r;out=None
 while n:
  if n&1:out=add(out,P)
  P=add(P,P);n//=2
 return out

lam=c['lambda'];Q=c['Q'];states=c['initial']['states'];labels=c['initial']['labels'];checks=0
for step in c['steps']:
 for i,(T,L) in enumerate(zip(step['states'],step['labels'])):
  P=add(mul(L[0],G),mul(L[1],Q));assert T==[mul(pow(lam,k,r),P) for k in range(3)]
  a,b=labels[i];aa,bb=c['coefficients'][step['indices'][i]];j,sg=step['orientations'][i];z=sg*pow(lam,j,r)%r
  assert L==[z*(a+aa)%r,z*(b+bb)%r]
  U=add(states[i][0],c['table'][step['indices'][i]][0]);orbit=[mul(pow(lam,k,r),U) for k in range(5)]
  jj=min(range(5),key=lambda k:orbit[k][0]);ss=1 if orbit[jj][1]<p-orbit[jj][1] else -1;assert [jj,ss]==[j,sg];checks+=1
 branches=step['layers'];assert len(branches)==103
 assert all(v in ['double','generic'] for layer in branches for v in layer)
 D=sum(v=='double' for layer in branches for v in layer);A=sum(v=='generic' for layer in branches for v in layer)
 assert (D,A)==(260,192)
 expected={'M':sum(5*len(layer)-3 for layer in branches),'S':2*D+A,'I':len(branches),'K':2*D,'A':5*D+6*A+4*19+4+3*sum(sg<0 for j,sg in step['orientations'])}
 assert expected==step['cost'],(expected,step['cost'])
 states,labels=step['states'],step['labels']
result={'all_checks_pass':True,'labelled_transitions':checks,'independent_field_count_reconstructions':len(c['steps']),'batch_layers_per_transition':103,'repeat_records':len(c['repeats']),'fruitless_repeats':sum(x['fruitless'] for x in c['repeats'])}
Path('verification.json').write_text(json.dumps(result,indent=2)+'\n');print(json.dumps(result,indent=2))
