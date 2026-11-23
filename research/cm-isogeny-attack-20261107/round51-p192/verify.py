from pathlib import Path
import json
c=json.loads(Path('certificate.json').read_text());p,r,t=c['p'],c['r'],c['trace']
G=[int('188DA80EB03090F67CBF20EB43A18800F4FF0AFD82FF1012',16),int('07192B95FFC8DA78631011ED6B24CDD573F977A11E794811',16)]
def add(P,Q):
 if P is None:return Q
 if Q is None:return P
 x,y=P;u,v=Q
 if x==u and (y+v)%p==0:return None
 m=((3*x*x-3)*pow(2*y,-1,p) if P==Q else (v-y)*pow(u-x,-1,p))%p;z=(m*m-x-u)%p
 return [z,(m*(x-z)-y)%p]
def mul(n,P):
 n%=r;R=None
 while n:
  if n&1:R=add(R,P)
  P=add(P,P);n//=2
 return R
# Ring O=Z[pi], pi^2=t*pi-p, with exact pair arithmetic.
def oa(a,b):return [a[0]+b[0],a[1]+b[1]]
def om(a,b):return [a[0]*b[0]-p*a[1]*b[1],a[0]*b[1]+a[1]*b[0]+t*a[1]*b[1]]
def bar(a):return [a[0]+t*a[1],-a[1]]
def mm(A,B):return [[oa(om(A[i][0],B[0][j]),om(A[i][1],B[1][j])) for j in range(2)] for i in range(2)]
def adj(A):return [[bar(A[j][i]) for j in range(2)] for i in range(2)]
alpha=[-1,1];assert om(alpha,bar(alpha))==[r,0]

def pairing(a,b,q):return sum(a[i]*b[i+2]-a[i+2]*b[i] for i in range(2))%q
def rank(rows,q):
 A=[[x%q for x in row] for row in rows];k=0
 for j in range(4):
  pivot=next((i for i in range(k,len(A)) if A[i][j]),None)
  if pivot is None:continue
  A[k],A[pivot]=A[pivot],A[k];v=pow(A[k][j],-1,q);A[k]=[v*x%q for x in A[k]]
  for i in range(len(A)):
   if i!=k:
    z=A[i][j];A[i]=[(x-z*y)%q for x,y in zip(A[i],A[k])]
  k+=1
 return k
for z in c['formal_controls']:
 x=z['x'];gamma=z['gamma']
 for key in ['rational_completion','mixed_completion']:
  W=z[key];assert rank(W,r)==2 and rank(W+[gamma],r)==2 and pairing(*W,r)==0
  assert rank(W+[[a,b,cc*p,dd*p] for a,b,cc,dd in W],r)==2
 assert z['anti_isometry_diagonal'][0]*z['anti_isometry_diagonal'][1]%r==r-1
for z in c['product_completions']:
 u,v=z['u'],z['v'];M=[[u,v],[[-w for w in bar(v)],bar(u)]]
 assert mm(adj(M),M)==[[[r,0],[0,0]],[[0,0],[r,0]]]
 x=z['x'];Q=mul(x,G);assert Q==z['Q']
 for row in M:
  aa=(row[0][0]+row[0][1])%r;bb=(row[1][0]+row[1][1])%r;assert add(mul(aa,G),mul(bb,Q)) is None
  assert ((row[0][0]+p*row[0][1])*(-x)+row[1][0]+p*row[1][1])%r==0
for n,ab in [(r,c['sum_squares_r']),(t-2,c['sum_squares_trace_minus_two'])]:assert sum(a*a for a in ab)==n
q=11;toy=[]
for x in range(q):
 g=[1,x,0,0];u=[0,1,0,0];w=[0,0,-x,1];stable=[]
 for s in list(range(q))+['infinity']:
  v=w if s=='infinity' else [(a+s*b)%q for a,b in zip(u,w)]
  W=[g,v];assert rank(W,q)==2 and pairing(g,v,q)==0
  if rank(W+[[v[0],v[1],2*v[2],2*v[3]]],q)==2:stable.append(s)
 assert stable==[0,'infinity'];toy.append({'x':x,'lines_checked':12,'stable_lines':stable})
assert all(pow(p,k,r)!=1 for k in range(1,1001));assert pow(p,1000,r)==c['extension_degree_exclusions']['last_residue']
out={'all_checks_pass':True,'formal_isotropic_and_Frobenius_cases':len(c['formal_controls'])*2,'toy_line_tests':132,'stable_lines_per_toy_slope':2,'exact_product_polarization_and_point_controls':len(c['product_completions']),'extension_degree_exclusions':1000,'toy':toy,'primality_dependency':'only exact sum-of-squares identities used; exploratory factor output is not a primality certificate'}
Path('verification.json').write_text(json.dumps(out,indent=2)+'\n');print(json.dumps({k:v for k,v in out.items() if k!='toy'},indent=2))
