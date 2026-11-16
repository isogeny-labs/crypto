from fractions import Fraction as Q
from pathlib import Path
import json
# Gaussian-rational polynomial coefficients, low degree first.
def c(a,b=0): return Q(a),Q(b)
def add(a,b): return a[0]+b[0],a[1]+b[1]
def mul(a,b): return a[0]*b[0]-a[1]*b[1],a[0]*b[1]+a[1]*b[0]
def padd(a,b):
 out=[c(0)]*max(len(a),len(b))
 for j,x in enumerate(a):out[j]=add(out[j],x)
 for j,x in enumerate(b):out[j]=add(out[j],x)
 return out
def pmul(a,b):
 out=[c(0)]*(len(a)+len(b)-1)
 for j,x in enumerate(a):
  for k,y in enumerate(b):out[j+k]=add(out[j+k],mul(x,y))
 return out
def power(a,n):
 out=[c(1)]
 for _ in range(n):out=pmul(out,a)
 return out
def scale(a,s):return [mul(x,s) for x in a]
A=[c(0,1),c(1,1)];B=[c(0,-1),c(1,-1)]
F=[c(0)]
for r,an,bn in [(c(33,56),6,0),(c(159,56),4,2),(c(159,-56),2,4),(c(33,-56),0,6)]:F=padd(F,scale(pmul(power(A,an),power(B,bn)),r))
F=scale(F,c(Q(-1,384)));assert all(b==0 for a,b in F)
while F and F[-1]==c(0):F.pop()
alpha=F[5][0];beta=-F[4][0]/(5*alpha)
N=[c(0)]
for j,coef in enumerate(F):N=padd(N,scale(power([c(beta),c(alpha)],j),mul(coef,c(1/alpha**6))))
assert N[5]==c(1) and N[4]==c(0)
out={'schema':'research.exact-symbolic-expansion/v1','source_coefficients_low_first':[str(a) for a,b in F],'alpha':str(alpha),'beta':str(beta),'ordinate_scale':str(alpha**3),'normalized_coefficients_low_first':[str(a) for a,b in N],'finite_fields_enumerated':0,'scientific_executions':0}
Path(__file__).with_name('expansion.json').write_text(json.dumps(out,indent=2)+'\n');print(json.dumps(out,indent=2))
