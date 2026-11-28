#!/usr/bin/env python3
"""Exact higher-term transform of the wild marked23 x=0 edge."""
import hashlib
import json
import math
import sys
from fractions import Fraction
from pathlib import Path
root=Path(__file__).resolve().parents[1]
paths={'donor51':root/'51/runs/base23-struct51-03/marked23-triangular51.sing',
       'parser105':root/'105/jets105.py','result113':root/'113/result.json'}
protocol=json.loads(Path(__file__).with_name('protocol.json').read_text())

def lower_hull(points):
 lowest={}
 for h,r in points:lowest[h]=min(r,lowest.get(h,r))
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
 prior=json.loads(paths['result113'].read_text())
 assert prior['positive_edges'][0]['residual_exponents']==[0,2]
 points=set()
 for ve,xe in prior['support']:
  for he in range(ve+1):
   if math.comb(ve,he)&1:
    p=(he,2*xe+3*ve-6)
    if p in points:points.remove(p)
    else:points.add(p)
 if any(re<0 for he,re in points):raise ValueError('r^6 does not divide transform')
 points=sorted(points)
 assert [he for he,re in points if re==0]==[2]
 vertices=lower_hull(points)
 edges=[]
 for a,b in zip(vertices,vertices[1:]):
  (h0,r0),(h1,r1)=a,b
  slope=Fraction(r0-r1,h1-h0)
  if slope<=0:continue
  on=sorted((h,re) for h,re in points if h0<=h<=h1 and (re-r0)*(h1-h0)==(r1-r0)*(h-h0))
  edges.append({'endpoints':[list(a),list(b)],'h_valuation':str(slope),
                'ramification_denominator':slope.denominator,
                'edge_support':[list(p) for p in on],
                'residual_exponents':[h-h0 for h,re in on]})
 result={'source_sha256':protocol['inputs_sha256'],'substitution':protocol['substitution'],
         'term_count':len(points),'support':[list(p) for p in points],
         'lower_hull':[list(p) for p in vertices], 'positive_edges':edges}
 out.mkdir(parents=True,exist_ok=True)
 (out/'result.json').write_text(json.dumps(result,indent=2,sort_keys=True)+'\n')
 print('WILD114_ALL_CHECKS_PASS')
if __name__=='__main__':main()
