"""Independent constructive degree-five proof for two cubic state grids.

Every step emits an ordinary bounded-degree ideal derivation, replayable by
proof_trace.verify_refutation. This exercises the algebraic proof rather than
running a closure search. Variables are w,z,u; grid roots are 0,1,2.
"""
import json, random
from pathlib import Path
from instance import problem_digest
from proof_trace import verify_refutation

D=5
ZERO=(0,0,0)

def clean(a,p): return {e:c%p for e,c in a.items() if c%p}
def add(a,b,p,k=1):
 r=dict(a)
 for e,c in b.items():r[e]=(r.get(e,0)+k*c)%p
 return clean(r,p)
def mul(a,b,p):
 r={}
 for e,c in a.items():
  for f,d in b.items():
   g=tuple(x+y for x,y in zip(e,f));r[g]=(r.get(g,0)+c*d)%p
 return clean(r,p)
def deg(a):return max(map(sum,a),default=-1)
def terms(a):return [[c,list(e)] for e,c in sorted(a.items())]
def interp(v,p):
 a=(v[2]-2*v[1]+v[0])*pow(2,-1,p)%p
 return[v[0]%p,(v[1]-v[0]-a)%p,a]
def uni(c,k):
 r={}
 for i,a in enumerate(c):
  e=[0,0,0];e[k]=i
  if a:r[tuple(e)]=a
 return r

def evaluate(a,w,z,u,p):
 return sum(c*pow(w,e[0],p)*pow(z,e[1],p)*pow(u,e[2],p) for e,c in a.items())%p

class Builder:
 def __init__(self,problem):
  self.p=problem['p'];self.problem=problem;self.nodes=[];self.values=[];self.gs=[]
  for i,f in enumerate(problem['polynomials']):
   self.emit({'op':'input','index':i},{tuple(e):c%self.p for c,e in f if c%self.p})
  self.gs=[0,1]
 def emit(self,node,value):
  value=clean(value,self.p)
  assert deg(value)<=D,(node,deg(value))
  self.nodes.append(node);self.values.append(value);return len(self.nodes)-1
 def scale(self,j,c):return self.emit({'op':'scale','parent':j,'coefficient':c%self.p},{e:a*c%self.p for e,a in self.values[j].items()})
 def plus(self,j,k,c=1):return self.emit({'op':'add','left':j,'right':k,'coefficient':c%self.p},add(self.values[j],self.values[k],self.p,c))
 def var(self,j,k):
  a={}
  for e,c in self.values[j].items():
   f=list(e);f[k]+=1;a[tuple(f)]=c
  return self.emit({'op':'multiply','parent':j,'variable':k},a)
 def mon(self,j,e):
  for k,n in enumerate(e):
   for _ in range(n):j=self.var(j,k)
  return j
 def product(self,j,a):
  result=self.scale(j,0)
  for e,c in a.items():result=self.plus(result,self.mon(j,e),c)
  return result
 def nf(self,j):
  # Each grid input is t^3-3t^2+2t. Division preserves total degree.
  while True:
   e=next((e for e in self.values[j] if e[0]>=3 or e[1]>=3),None)
   if e is None:return j
   k=0 if e[0]>=3 else 1;mult=list(e);mult[k]-=3
   c=self.values[j][e]
   j=self.plus(j,self.mon(self.gs[k],mult),-c)
 def normalize(self,j,k):
  row0=self.nf(j);row1=self.nf(self.var(row0,k));row2=self.nf(self.var(row1,k))
  q=self.values[row0];alpha=[]
  for s in (0,1,2):
   v=0
   for e,c in q.items():
    if e[2]==2:v=(v+c*pow(s,e[k],self.p))%self.p
   assert v
   alpha.append(pow(v,-1,self.p))
  coeffs=interp(alpha,self.p)
  result=self.scale(row0,0)
  for row,c in zip((row0,row1,row2),coeffs):result=self.plus(result,row,c)
  result=self.nf(result)
  assert self.values[result].get((0,0,2))==1
  assert all(e[2]!=2 or e==(0,0,2) for e in self.values[result])
  assert deg(self.values[result])<=3
  return result
 def scalar_times_nf(self,j,a):
  # a may be arbitrary on the 3x3 grid; every scalar remainder has degree<=4.
  result=self.scale(j,0)
  for e,c in a.items():
   row=j
   for k,n in enumerate(e):
    assert k<2 or n==0
    for _ in range(n):row=self.nf(self.var(row,k))
   result=self.plus(result,row,c)
  return self.nf(result)
 def finish(self):
  h1=self.normalize(2,0);h2=self.normalize(3,1)
  H1,H2=self.values[h1],self.values[h2]
  A={e[:2]+(0,):c for e,c in H1.items() if e[2]==1}
  C={e[:2]+(0,):c for e,c in H2.items() if e[2]==1}
  E=add(A,C,self.p,-1)
  L=self.plus(h1,h2,-1)
  Hsum=self.plus(h1,h2)
  N=self.plus(self.scale(self.var(L,2),2),self.product(Hsum,E),-1)
  N=self.nf(N)
  assert deg(self.values[N])<=4
  F={e[:2]+(0,):c for e,c in self.values[N].items() if e[2]==1}
  assert all(not(e[0] and e[1]) for e in F)
  # EN is staged so intermediate scalar reductions return to degree <=4.
  EN=self.scalar_times_nf(N,E)
  FL=self.nf(self.product(L,F))
  res=self.nf(self.scale(self.plus(EN,FL,-1),-pow(2,-1,self.p)))
  assert all(e[2]==0 for e in self.values[res])
  assert deg(self.values[res])<=4
  inv_values=[[pow(evaluate(self.values[res],w,z,0,self.p),-1,self.p) for w in (0,1,2)] for z in (0,1,2)]
  # Interpolate first in w then in z.
  coeff_w=[interp(row,self.p) for row in inv_values]
  inv={}
  for i in range(3):
   for j,c in enumerate(interp([coeff_w[z][i] for z in range(3)],self.p)):
    if c:inv[(i,j,0)]=c
  goal=self.scalar_times_nf(res,inv)
  assert self.values[goal]=={ZERO:1}
  cert={'schema':'bounded-degree-refutation-v1','degree':5,'problem_sha256':problem_digest(self.problem),'nodes':self.nodes,'goal':goal}
  assert verify_refutation(self.problem,cert)
  return cert

def make_problem(p,rng):
 values=rng.sample(range(p),12)
 polys=[]
 for k in (0,1):polys.append(uni([0,2,-3,1],k))
 for k in (0,1):
  roots=[(values[6*k+2*i],values[6*k+2*i+1]) for i in range(3)]
  # Exercise repeated roots as well, without intersection between two families.
  if rng.randrange(3)==0:roots[0]=(roots[0][0],roots[0][0])
  alpha=[rng.randrange(1,p) for _ in range(3)]
  coeffs=[interp([a for a in alpha],p),interp([-a*(s+t) for a,(s,t) in zip(alpha,roots)],p),interp([a*s*t for a,(s,t) in zip(alpha,roots)],p)]
  q={}
  for degree_u,coefs in zip((2,1,0),coeffs):
   for i,c in enumerate(coefs):
    e=[0,0,degree_u];e[k]=i
    if c:q[tuple(e)]=c
  polys.append(q)
 return {'p':p,'n':3,'polynomials':[terms(a) for a in polys]}

def main():
 rng=random.Random(20261028);count=0;example=None
 for p in (17,31,101):
  for _ in range(20):
   problem=make_problem(p,rng);proof=Builder(problem).finish();count+=1
   if example is None:example=(problem,proof)
 root=Path(__file__).parent
 (root/'normalized_five_example_problem.json').write_text(json.dumps(example[0],indent=2)+'\n')
 (root/'normalized_five_example_trace.json').write_text(json.dumps(example[1],indent=2)+'\n')
 report={'cases':count,'all_constructive_traces_replayed':True,'degree':5,'seed':20261028,'scope':'general two-cubic-grid monic-normalization and resultant proof, disjoint rational quadratic root sets'}
 (root/'normalized_five_constructive_validation.json').write_text(json.dumps(report,indent=2)+'\n')
 print(json.dumps(report,indent=2))
if __name__=='__main__':main()
