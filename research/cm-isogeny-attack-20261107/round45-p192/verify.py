from pathlib import Path
from math import prod
from fractions import Fraction
import itertools,json
c=json.loads(Path('certificate.json').read_text());r=c['r'];mu=c['mu']
def det(u,v):return u[0]*v[1]-u[1]*v[0]
def ceil(x):return -((-x.numerator)//x.denominator)
def region_check(z,lam):
 scale=z['scale'];B=z['B'];u,v=z['basis'];w=z['offset'];d=det(u,v)
 assert d==scale*r and w==[0,(-lam*lam)%r]
 for x,y in [u,v]:assert x%scale==0 and (y-lam*(x//scale))%r==0
 # Change basis, then derive exact coordinate bounds from the square corners.
 u=[u[i]+v[i] for i in range(2)];bounds=[]
 corners=[(x-w[0],y-w[1]) for x in [-B,B] for y in [-B,B]]
 coords=[[Fraction(det(X,v),d) for X in corners],[Fraction(det(u,X),d) for X in corners]]
 for vals in coords:bounds.append((ceil(min(vals)),max(vals).__floor__()))
 pts=set()
 for i in range(bounds[0][0],bounds[0][1]+1):
  for j in range(bounds[1][0],bounds[1][1]+1):
   x=w[0]+i*u[0]+j*v[0];y=w[1]+i*u[1]+j*v[1]
   if abs(x)<=B and abs(y)<=B:pts.add((x,y))
 assert pts=={tuple(v[:2]) for v in z['points']}
 return pts
def mm(A,B):return [[sum(A[i][k]*B[k][j] for k in range(2)) for j in range(2)] for i in range(2)]
def det4(A):
 out=0
 for perm in itertools.permutations(range(4)):
  sign=(-1)**sum(perm[i]>perm[j] for i in range(4) for j in range(i+1,4))
  out+=sign*prod(A[i][perm[i]] for i in range(4))
 return out
C=[[0,1,0,0],[0,0,1,0],[0,0,0,1],[-1,-1,-1,-1]]
C2=[[sum(C[i][k]*C[k][j] for k in range(4)) for j in range(4)] for i in range(4)]
norms=[]
for row in c['rows']:
 lam=row['eigenvalue'];assert lam==pow(mu,row['power'],r)
 for key in ['companion_minimax','entry_rectangle_bound']:
  opt=row[key];assert not region_check(opt['infeasible'],lam) and region_check(opt['feasible'],lam)
  assert opt['infeasible']['H']==opt['minimum']-1 and opt['feasible']['H']==opt['minimum']
  for name in ['infeasible','feasible']:
   z=opt[name];H=z['H'];assert z['scale']==(H if key=='entry_rectangle_bound' else 1) and z['B']==(2*H*H if key=='entry_rectangle_bound' else H)
 for m in row['matrices']:
  A=m['matrix'];a,b=A[0];cc,d=A[1];tr=a+d;de=a*d-b*cc
  assert tr==m['trace'] and de==m['determinant'] and (lam*lam-tr*lam+de)%r==0
  assert max(abs(x) for R in A for x in R)==m['height']>=row['entry_rectangle_bound']['minimum']
  v=m['eigenvector'];assert v==[b%r,(lam-a)%r] and all((sum(A[i][j]*v[j] for j in range(2))-lam*v[i])%r==0 for i in range(2))
  power=[[1,0],[0,1]]
  for _ in range(5):power=mm(power,A)
  assert power!=[[1,0],[0,1]]
  N=det4([[C2[i][j]-tr*C[i][j]+de*int(i==j) for j in range(4)] for i in range(4)])
  assert 0<N<=(1+abs(tr)+abs(de))**4 and N%r==0
  norms.append({'power':row['power'],'kind':m['kind'],'cyclotomic_resultant':N})
  for s in m['schedules']:
   w=s['width'];assert all(sum(d*2**i for i,d in enumerate(ds))==A[x][y] for x,R in enumerate(s['digits']) for y,ds in enumerate(R))
   D=2*int(w>2)+sum(max(0,max(map(len,R))-1) for R in s['digits'])
   adds=2*(2**(w-2)-1)+sum(max(0,sum(d!=0 for ds in R for d in ds)-1) for R in s['digits'])
   assert (D,adds)==(s['D'],s['A'])
   cd=2*D-int(w>2) if m['kind']=='companion' else 3*D
   ca=2*adds+3-(2**(w-2)-1) if m['kind']=='companion' else 3*adds+6
   assert (cd,ca)==(s['canonical_D'],s['canonical_A'])
  assert m['selected']==min(m['schedules'],key=lambda z:(z['canonical_D']+z['canonical_A'],z['canonical_D'],z['canonical_A'],z['width']))
out={'all_checks_pass':True,'global_minimax_certificates':4,'global_entry_rectangle_certificates':4,'matrix_eigenvectors_and_non_global_period_checks':8,'signed_digit_schedules':40,'resultant_checks':norms,'proof_scope':'Trace/determinant minimax and rectangular entry lower bounds are global; balanced realization is a finite construction.'}
Path('verification.json').write_text(json.dumps(out,indent=2)+'\n');print('INDEPENDENT_LATTICE_MATRIX_AND_COST_PASS',4,4,8,40)
