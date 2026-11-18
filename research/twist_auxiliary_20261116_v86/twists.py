from itertools import product
import json,time
from .common import P,finish
from .algebra import Field,short_points,s3
START=time.monotonic()
def main():
 out={'triples':0,'coordinate_types':0,'cases':[],'qualified_speedup':None};error=None
 try:
  d=json.loads((P/'DATA.json').read_text())
  for row in d['fields']:
   f=Field(row['p'],row['mod']);q=f.q
   assert all(f.mul(x,f.inv(x))==1 for x in range(1,q))
   for A in ([0,1]if f.p==2 else[1]):
    B=1;delta=next(x for x in range(1,q)if(f.trace(x)==1 if f.p==2 else f.power(x,(q-1)//2)==f.neg(1)));a2=A^delta if f.p==2 else f.mul(f.sq(delta),A);b2=B if f.p==2 else f.mul(f.mul(f.sq(delta),delta),B);pt=short_points(f,A,B);qt=short_points(f,a2,b2);scale=1 if f.p==2 else delta;rootcount=0;torcount=0
    def tag(x,pts,AA,BB):
     rhs=f.add(f.mul(f.sq(x),x),f.add(f.mul(AA,x),BB))
     return 'T'if(x==0 if f.p==2 else rhs==0)else'R'if pts[x]else'A'
    for x in range(q):
     u=f.mul(scale,x);t=tag(x,pt,A,B);w=tag(u,qt,a2,b2)
     assert w=='T'if t=='T'else(t,w)in [('R','A'),('A','R')]
     out['coordinate_types']+=1
    factor=f.power(delta,4)if f.p!=2 else 1
    for xs in product(range(q),repeat=3):
     ys=tuple(f.mul(scale,x)for x in xs);v=s3(*xs,A,B,f);w=s3(*ys,a2,b2,f);assert w==f.mul(factor,v);out['triples']+=1
     if not v:rootcount+=1;torcount+=any(tag(x,pt,A,B)=='T'for x in xs)
    out['cases'].append({'field':row['name'],'A':A,'B':B,'delta':delta,'twist_A':a2,'twist_B':b2,'S3_root_count':rootcount,'torsion_root_triples':torcount,'binary_identical_polynomial':f.p==2});assert time.monotonic()-START<65
  out['complete']=True
 except Exception as e:error=repr(e)
 finish(out,error,START,'Complete curve/twist coordinate and S3 transfer over tiny prime/binary/odd-extension fields.','Use twists for complete type routing; do not count identical binary S3 systems as distinct hard instances or infer reduced polynomial difficulty.')
if __name__=='__main__':main()
