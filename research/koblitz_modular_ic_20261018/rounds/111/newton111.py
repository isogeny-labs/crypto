#!/usr/bin/env python3
"""Exact x=0 f Newton edges and G coefficient lower bounds."""
import hashlib
import importlib.util
import json
import sys
from fractions import Fraction
from pathlib import Path

root=Path(__file__).resolve().parents[1]
donor=root/'51/runs/base23-struct51-03/marked23-triangular51.sing'
parser=root/'105/jets105.py'
protocol=json.loads(Path(__file__).with_name('protocol.json').read_text())

def load(path):
 spec=importlib.util.spec_from_file_location('jets105',path)
 mod=importlib.util.module_from_spec(spec);spec.loader.exec_module(mod);return mod

def support(fterms,shift):
 out=set()
 for ye,exps in fterms:
  for xe in exps:
   for ze in range(ye+1):
    if shift==0 and ze!=ye:continue
    if shift==1 and ze&~ye:continue
    pair=(ze,xe)
    if pair in out:out.remove(pair)
    else:out.add(pair)
 return sorted(out)

def hull(points):
 mins={}
 for z,x in points:mins[z]=min(x,mins.get(z,x))
 ordered=sorted(mins.items())
 out=[]
 for p in ordered:
  while len(out)>=2:
   a,b=out[-2:]
   cross=(b[0]-a[0])*(p[1]-b[1])-(b[1]-a[1])*(p[0]-b[0])
   if cross<=0:out.pop()
   else:break
  out.append(p)
 return out

def edges(points,vertices):
 out=[]
 for (z0,x0),(z1,x1) in zip(vertices,vertices[1:]):
  v=Fraction(x0-x1,z1-z0)
  if v<=0:continue
  on=sorted((z,x) for z,x in points if (x-x0)*(z1-z0)==(x1-x0)*(z-z0) and z0<=z<=z1)
  residual=sorted(z-z0 for z,x in on)
  out.append({'endpoints':[[z0,x0],[z1,x1]],'z_valuation':str(v),
              'ramification_denominator':v.denominator,'edge_support':[[z,x] for z,x in on],
              'residual_exponents':residual})
 return out

def g_bounds(gterms,vy):
 out={}
 for le,ye,den,exps in gterms:
  weight=Fraction(min(exps)-den,1)+ye*vy
  if le not in out or weight<out[le]:out[le]=weight
 return {str(j):str(out[j]) for j in sorted(out)}

def main():
 outdir=Path(sys.argv[1])
 for name,path in [('donor51',donor),('parser105',parser)]:
  if hashlib.sha256(path.read_bytes()).hexdigest()!=protocol['inputs_sha256'][name]:
   raise ValueError('source hash mismatch: '+name)
 fterms,gterms=load(parser).parse_source(donor.read_text())
 assert len(fterms)==8 and len(gterms)==29
 special=0
 for ye,exps in fterms:
  if 0 in exps:special^=1<<ye
 assert special==sum(1<<i for i in (4,5,6,7))
 out={'source_sha256':protocol['inputs_sha256'],'f_term_count':len(fterms),
      'g_term_count':len(gterms),'f_x0_bits':special}
 for name,shift in [('y0',0),('y1',1)]:
  s=support(fterms,shift);v=hull(s);ed=edges(s,v)
  out[name]={'f_support':[[a,b] for a,b in s], 'lower_hull':[[a,b] for a,b in v],
             'positive_edges':ed,'g_coefficient_lower_bounds':[
              {'edge':i,'bound_by_l_exponent':g_bounds(gterms,
                Fraction(e['z_valuation']) if shift==0 else Fraction(0))}
              for i,e in enumerate(ed)]}
 outdir.mkdir(parents=True,exist_ok=True)
 (outdir/'result.json').write_text(json.dumps(out,indent=2,sort_keys=True)+'\n')
 print('NEWTON111_ALL_CHECKS_PASS')
if __name__=='__main__':main()
