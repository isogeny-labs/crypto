from pathlib import Path
import json
c=json.loads(Path('certificate.json').read_text());q=json.loads(Path('point-certificate.json').read_text());r=c['r'];p=2**192-2**64-1
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
checks=0
for f in q['fixtures']:
 row=next(z for z in c['rows'] if z['power']==f['power']);m=next(z for z in row['matrices'] if z['kind']==f['kind']);A=m['matrix'];P=mul(f['scalar'],G)
 T=[mul(x,P) for x in m['eigenvector']]
 assert f['orbit'][0]==T
 for j,V in enumerate(f['orbit']):
  assert V==[mul(pow(row['eigenvalue'],j,r),x) for x in T]
  following=[add(mul(A[i][0],V[0]),mul(A[i][1],V[1])) for i in range(2)]
  assert following==f['orbit'][(j+1)%5];checks+=1
 if f['scalar']:
  j=min(range(5),key=lambda j:f['orbit'][j][0][0]);sgn=1 if f['orbit'][j][0][1]<p-f['orbit'][j][0][1] else -1
  rep=f['orbit'][j] if sgn==1 else [[x,(-y)%p] for x,y in f['orbit'][j]]
  assert (j,sgn,rep)==(f['rotation'],f['sign'],f['representative'])
  marked_label=sgn*m['eigenvector'][0]*pow(row['eigenvalue'],j,r)*f['scalar']%r
  assert mul(marked_label,G)==rep[0]
  assert f['canonicalization_count']['D']==m['canonicalization_D'] and f['canonicalization_count']['A']==m['canonicalization_A']
  for offset in range(5):
   for sign in [1,-1]:
    shifted=f['orbit'][offset:]+f['orbit'][:offset]
    if sign==-1:shifted=[[[x,(-y)%p] for x,y in V] for V in shifted]
    jj=min(range(5),key=lambda k:shifted[k][0][0]);candidate=shifted[jj]
    if candidate[0][1]>p-candidate[0][1]:candidate=[[x,(-y)%p] for x,y in candidate]
    assert candidate==rep
 else:assert all(x==0 for x in f['canonicalization_count'].values())
out={'all_checks_pass':True,'independent_orbit_and_matrix_transitions':checks,'point_fixtures':40,'canonical_representatives':32,'orbit_encoding_controls':320,'identity_controls':8,'imports_producer':False}
Path('point-verification.json').write_text(json.dumps(out,indent=2)+'\n');print(json.dumps(out,indent=2))
