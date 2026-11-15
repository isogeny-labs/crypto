from pathlib import Path
import json
c=json.loads(Path('certificate.json').read_text());p,r,t=c['p'],c['r'],c['trace']
assert p+1-t==r==c['alpha_norm']
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
for z in c['fixtures']:
 x=z['public_secret'];a,b=z['matrix'][0];cc,d=z['matrix'][1];u=a-d;assert z['u_b_c']==[u,b,cc]
 assert (b*x*x+u*x-cc)%r==0 and any(k%r for k in [b,u,cc])
 assert z['Q']==mul(x,G)
 image=[add(mul(a,G),mul(b,z['Q'])),add(mul(cc,G),mul(d,z['Q']))];assert image==z['image'] and image[1]==mul(x,image[0])
 for root in z['roots']:assert (b*root*root+u*root-cc)%r==0
 assert [v for v in z['roots'] if mul(v,G)==z['Q']]==[x]
 F=[[alpha,[0,0]],[[-x,0],[1,0]]];H=[[[1+x*x,0],om([x,0],alpha)],[om([x,0],bar(alpha)),[r,0]]]
 assert mm(mm(adj(F),H),F)==[[[r,0],[0,0]],[[0,0],[r,0]]]
 assert oa(om(H[0][0],H[1][1]),[-v for v in om(H[0][1],H[1][0])])==[r,0]
 # determinant of each lattice basis checks its exact index.
 A=z['reduced_basis'];det=sum(A[0][i]*(A[1][(i+1)%3]*A[2][(i+2)%3]-A[1][(i+2)%3]*A[2][(i+1)%3]) for i in range(3));assert abs(det)==r
 assert all((v*x+w*x*x-y)%r==0 for v,w,y in A)
q=c['toy_prime']
for z in c['toy_counts']:
 # Explicit conjugation of all upper triangular matrices, counted without testing the producer equation.
 x=z['x'];S=set()
 for aa in range(q):
  for bb in range(q):
   for dd in range(q):S.add(((aa-bb*x)%q,bb,(x*aa-x*x*bb-dd*x)%q,(x*bb+dd)%q))
 assert len(S)==z['stabilizers']==q**3
 assert sum((a*d-b*cc)%q!=0 for a,b,cc,d in S)==z['invertible']
out={'all_checks_pass':True,'independent_point_and_root_cases':len(c['fixtures']),'exact_CM_polarization_identities':len(c['fixtures']),'index_r_lattices':len(c['fixtures']),'independent_toy_line_stabilizers':len(c['toy_counts']),'scope':'algebraic identities and known-secret controls; no quotient construction for unknown secret'}
Path('verification.json').write_text(json.dumps(out,indent=2)+'\n');print(json.dumps(out,indent=2))
