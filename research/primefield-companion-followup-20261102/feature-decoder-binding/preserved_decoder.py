"""Coefficient-only synthesis; all modular arithmetic has explicit counters."""
from itertools import combinations

def labels(q):return [s for d in range(4) for s in combinations(range(q),d)]
class Meter:
 def __init__(self,p):self.p=p;self.stage='feature';self.counts={}
 def op(self,k):key=self.stage+'_'+k;self.counts[key]=self.counts.get(key,0)+1
 def mul(self,a,b):self.op('products');return a*b%self.p
 def add(self,a,b):self.op('additions');return (a+b)%self.p
 def sub(self,a,b):self.op('additions');return (a-b)%self.p
 def inv(self,a):self.op('inversions');return pow(a,-1,self.p)
class Features:
 def __init__(self,r,q,m):
  self.r=r;self.m=m;self.pairs={};h=(r-1)//2;binom=[1]
  for n in range(1,h+1):binom=[1]+[m.add(binom[i-1],binom[i]) for i in range(1,n)]+[1]
  self.single=[]
  for a in range(q):
   powers=[1]
   for j in range(h):powers.append(m.mul(powers[-1],a))
   self.single.append([m.mul(binom[i],powers[h-i]) for i in range(h+1)])
 def mul(self,a,b):
  m=self.m;r=self.r;out=[0]*(len(a)+len(b)-1)
  for i,x in enumerate(a):
   if x:
    for j,y in enumerate(b):
     if y:out[i+j]=m.add(out[i+j],m.mul(x,y))
  for j in range(len(out)-1,r-1,-1):out[j-r+1]=m.add(out[j-r+1],out[j])
  return out[:r]+[0]*max(0,r-len(out))
 def feature(self,s):
  if not s:return [1]+[0]*(self.r-1)
  if len(s)==1:return self.single[s[0]]+[0]*(self.r-len(self.single[0]))
  pair=s[:2]
  if pair not in self.pairs:self.pairs[pair]=self.mul(self.single[pair[0]],self.single[pair[1]])
  return self.pairs[pair][:] if len(s)==2 else self.mul(self.pairs[pair],self.single[s[2]])

def compile(r,q,early):
 m=Meter(r);features=Features(r,q,m);basis=[];attempts=[];residual=[0,1]+[0]*(r-2);weights=[];probes=[];available=labels(q)
 for label in available:
  m.stage='feature';v=features.feature(label);history=[];m.stage='elimination'
  for j in sorted(range(len(basis)),key=lambda j:basis[j]['pivot']):
   factor=v[basis[j]['pivot']]
   if factor:
    v=[m.sub(x,m.mul(factor,y)) for x,y in zip(v,basis[j]['v'])];history.append([j,factor])
  pivot=next((i for i,x in enumerate(v) if x),None)
  attempts.append(dict(label=list(label),history=history,pivot=pivot))
  if pivot is None:continue
  inv=m.inv(v[pivot]);v=[m.mul(x,inv) for x in v]
  basis.append(dict(label=list(label),history=history,pivot=pivot,inv=inv,v=v))
  m.stage='target';factor=residual[pivot];weights.append(factor)
  if factor:residual=[m.sub(x,m.mul(factor,y)) for x,y in zip(residual,v)]
  probes.append(dict(rank=len(basis),factor=factor,residual=residual[:]))
  if (early and not any(residual)) or len(basis)==r:break
 m.stage='reconstruction';terms=[];w=weights[:]
 if not any(residual):
  for j in reversed(range(len(basis))):
   b=basis[j];factor=m.mul(w[j],b['inv'])
   if factor:
    terms.append(dict(offsets=b['label'],coefficient=factor))
    for i,c in b['history']:w[i]=m.sub(w[i],m.mul(factor,c))
  terms.sort(key=lambda t:(len(t['offsets']),t['offsets']))
  program=dict(r=r,q=q,offsets=list(range(q)),degree=3,terms=terms)
  m.stage='verification';fresh=Features(r,q,m);out=[0]*r
  for term in terms:
   v=fresh.feature(tuple(term['offsets']));out=[m.add(x,m.mul(term['coefficient'],y)) for x,y in zip(out,v)]
  assert out==[0,1]+[0]*(r-2);dual=None
 else:
  program=None;t=next(i for i,x in enumerate(residual) if x);dual=[int(i==t) for i in range(r)]
  for b in sorted(basis,key=lambda x:x['pivot'],reverse=True):
   dot=0
   for x,y in zip(dual,b['v']):dot=m.add(dot,m.mul(x,y))
   dual[b['pivot']]=m.sub(dual[b['pivot']],dot)
  assert dual[1]
 return dict(program=program,counts=m.counts,certificate=dict(status='represented' if program else 'excluded',attempts=attempts,basis=basis,probes=probes,residual=residual,dual=dual,available_columns=len(available),generated_columns=len(attempts),rank=len(basis)),profile=dict(basis_coefficient_slots=r*len(basis),history_pairs=sum(len(b['history']) for b in basis),single_slots=sum(map(len,features.single)),pair_cache_slots=sum(map(len,features.pairs.values())),residual_slots=r,weight_slots=len(weights),diagnostic_probe_slots=r*len(probes)))

def evaluate(program,oracle):
 signs=[oracle(a) for a in program['offsets']];value=products=adds=0;r=program['r']
 for term in program['terms']:
  v=term['coefficient']
  for a in term['offsets']:v=v*signs[a]%r;products+=1
  value=(value+v)%r;adds+=1
 return value,dict(signs=signs,decoder_products=products,decoder_additions=adds)
