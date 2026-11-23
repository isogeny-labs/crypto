#!/usr/bin/env python3
"""Source-bound first blowup of the repeated x=0, y=1 edge."""
import hashlib
import importlib.util
import json
import sys
from fractions import Fraction
from pathlib import Path
root=Path(__file__).resolve().parents[1]
paths={'donor51':root/'51/runs/base23-struct51-03/marked23-triangular51.sing',
       'parser105':root/'105/jets105.py','result111':root/'111/result.json'}
protocol=json.loads(Path(__file__).with_name('protocol.json').read_text())

def load(path):
 spec=importlib.util.spec_from_file_location('jets105',path)
 mod=importlib.util.module_from_spec(spec);spec.loader.exec_module(mod);return mod

def support(fterms):
 pairs=set()
 for ye,exps in fterms:
  for xe in exps:
   for k in range(ye+1):
    if k&~ye:continue
    for j in range(k+1):
     if j&~k:continue
     p=(j,xe+k-3)
     if p in pairs:pairs.remove(p)
     else:pairs.add(p)
 if any(x<0 for _,x in pairs):raise ValueError('x^3 does not divide transformed f')
 return sorted(pairs)

def lower_hull(points):
 lowest={}
 for j,x in points:lowest[j]=min(x,lowest.get(j,x))
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
 for name,p in paths.items():
  if hashlib.sha256(p.read_bytes()).hexdigest()!=protocol['inputs_sha256'][name]:
   raise ValueError('source hash mismatch: '+name)
 prior=json.loads(paths['result111'].read_text())
 assert prior['y1']['positive_edges'][0]['residual_exponents']==[0,1,2,3]
 f,g=load(paths['parser105']).parse_source(paths['donor51'].read_text())
 assert len(f)==8 and len(g)==29
 points=support(f)
 assert [j for j,x in points if x==0]==[3]
 vertices=lower_hull(points)
 edges=[]
 for a,b in zip(vertices,vertices[1:]):
  (j0,x0),(j1,x1)=a,b
  slope=Fraction(x0-x1,j1-j0)
  if slope<=0:continue
  on=sorted((j,x) for j,x in points if j0<=j<=j1 and (x-x0)*(j1-j0)==(x1-x0)*(j-j0))
  edges.append({'endpoints':[list(a),list(b)],'w_valuation':str(slope),
                'ramification_denominator':slope.denominator,
                'edge_support':[list(p) for p in on],
                'residual_exponents':[j-j0 for j,x in on]})
 result={'source_sha256':protocol['inputs_sha256'],'substitution':protocol['substitution'],
         'term_count':len(points),'support':[list(p) for p in points],
         'lower_hull':[list(p) for p in vertices], 'positive_edges':edges}
 out.mkdir(parents=True,exist_ok=True)
 (out/'result.json').write_text(json.dumps(result,indent=2,sort_keys=True)+'\n')
 print('BLOWUP112_ALL_CHECKS_PASS')
if __name__=='__main__':main()
