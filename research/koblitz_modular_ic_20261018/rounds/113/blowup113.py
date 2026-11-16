#!/usr/bin/env python3
"""Exact second blowup of the x=0 y=1 repeated branch."""
import hashlib
import importlib.util
import json
import sys
from fractions import Fraction
from pathlib import Path
root=Path(__file__).resolve().parents[1]
paths={'donor51':root/'51/runs/base23-struct51-03/marked23-triangular51.sing',
       'parser105':root/'105/jets105.py','result112':root/'112/result.json'}
protocol=json.loads(Path(__file__).with_name('protocol.json').read_text())

def load(path):
 spec=importlib.util.spec_from_file_location('jets105',path)
 mod=importlib.util.module_from_spec(spec);spec.loader.exec_module(mod);return mod

def mul(a,b):
 out=set()
 for xa,va in a:
  for xb,vb in b:
   p=(xa+xb,va+vb)
   if p in out:out.remove(p)
   else:out.add(p)
 return out

def power(a,n):
 ans={(0,0)}
 for _ in range(n):ans=mul(ans,a)
 return ans

def lower_hull(points):
 lowest={}
 for v,x in points:lowest[v]=min(x,lowest.get(v,x))
 ans=[]
 for p in sorted(lowest.items()):
  while len(ans)>=2:
   a,b=ans[-2:]
   if (b[0]-a[0])*(p[1]-b[1])-(b[1]-a[1])*(p[0]-b[0])<=0:ans.pop()
   else:break
  ans.append(p)
 return ans

def main():
 out=Path(sys.argv[1])
 for name,path in paths.items():
  if hashlib.sha256(path.read_bytes()).hexdigest()!=protocol['inputs_sha256'][name]:
   raise ValueError('source hash mismatch: '+name)
 prior=json.loads(paths['result112'].read_text())
 assert prior['positive_edges'][1]['residual_exponents']==[0,2]
 f,g=load(paths['parser105']).parse_source(paths['donor51'].read_text())
 assert len(f)==8 and len(g)==29
 y={(0,0),(1,0),(2,0),(2,1)}
 terms=set()
 for ye,exps in f:
  yp=power(y,ye)
  for xe in exps:
   for xp,vp in yp:
    p=(xe+xp,vp)
    if p in terms:terms.remove(p)
    else:terms.add(p)
 if any(x<6 for x,v in terms):raise ValueError('x^6 does not divide f')
 points=sorted((v,x-6) for x,v in terms)
 assert [v for v,x in points if x==0]==[2,3]
 vertices=lower_hull(points)
 edges=[]
 for a,b in zip(vertices,vertices[1:]):
  (v0,x0),(v1,x1)=a,b
  slope=Fraction(x0-x1,v1-v0)
  if slope<=0:continue
  on=sorted((v,x) for v,x in points if v0<=v<=v1 and (x-x0)*(v1-v0)==(x1-x0)*(v-v0))
  edges.append({'endpoints':[list(a),list(b)],'v_valuation':str(slope),
                'ramification_denominator':slope.denominator,
                'edge_support':[list(p) for p in on],
                'residual_exponents':[v-v0 for v,x in on]})
 result={'source_sha256':protocol['inputs_sha256'],'substitution':protocol['substitution'],
         'term_count':len(points),'support':[list(p) for p in points],
         'lower_hull':[list(p) for p in vertices], 'positive_edges':edges}
 out.mkdir(parents=True,exist_ok=True)
 (out/'result.json').write_text(json.dumps(result,indent=2,sort_keys=True)+'\n')
 print('BLOWUP113_ALL_CHECKS_PASS')
if __name__=='__main__':main()
