import json,time,math
from .common import P,finish
from .kernel import trace_jump,Frobenius,add
from research.semilinear_trace_20261116_v82.cases import fields,cases
from research.image_cells_20261115_v67.algebra import rem,pmul,evaluate
START=time.monotonic()
def reference(t,k,H,f):
 q=rem(t,H,f);out=[]
 for i in range(k):out=add(out,q);q=rem(pmul(q,q,f),H,f)
 return out

def shift(a,m,f):
 return[__import__('functools').reduce(int.__xor__,(a[i]for i in range(j,len(a))if math.comb(i,j)&1),0)for j in range(m)]
def main():
 out={'cases':0,'literal_points':0,'Taylor_cases':0,'twist_checks':0,'omitted_twist_counterexample':None,'qualified_speedup':None};error=None
 try:
  d=json.loads((P/'DATA.json').read_text())
  for f,name in fields(d):
   F=Frobenius(f)
   for j in range(f.n):
    for k in d['lengths']:
     x=1<<j;y=x
     for _ in range(k):y=f.sq(y)
     assert F.twist(x,k)==y;out['twist_checks']+=1
   for m,kind,label,H,t in cases(f,d,name):
    for k in d['lengths']:
     T=trace_jump(t,k,H,f,F);assert T==reference(t,k,H,f);out['cases']+=1
     if f.n<=4 and kind=='split':
      for x in range(f.q):
       if evaluate(H,x,f):continue
       v=evaluate(t,x,f);w=0
       for _ in range(k):w^=v;v=f.sq(v)
       assert evaluate(T,x,f)==w;out['literal_points']+=1
     if kind=='repeated':
      coeff=shift(t,m,f);expect=[0]*m
      for r in range(k):
       for j,c in enumerate(coeff):
        pos=j*(1<<r)
        if pos>=m:break
        v=c
        for _ in range(r%f.n):v=f.sq(v)
        expect[pos]^=v
      assert shift(T,m,f)==expect;out['Taylor_cases']+=1
   # For constant alpha, substitution without coefficient twist leaves alpha.
   x=2;actual=f.sq(x)
   if actual!=x and out['omitted_twist_counterexample']is None:out['omitted_twist_counterexample']={'field':name,'H':[0,0,1],'t':[x],'k':2,'correct_trace':[x^actual],'untwisted_trace':[]}
   assert time.monotonic()-START<65
  assert out['omitted_twist_counterexample'];out['complete']=True
 except Exception as e:error=repr(e)
 return out,error
