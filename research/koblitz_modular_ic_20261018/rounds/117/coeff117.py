#!/usr/bin/env python3
"""Precision-aware G51 Laurent coefficients on four x=0 base places."""
import hashlib
import importlib.util
import json
import sys
from fractions import Fraction
from pathlib import Path
root=Path(__file__).resolve().parents[1]
paths={'donor51':root/'51/runs/base23-struct51-03/marked23-triangular51.sing',
 'parser105':root/'105/jets105.py','result98':root/'98/result.json',
 'result116':root/'116/result.json'}
protocol=json.loads(Path(__file__).with_name('protocol.json').read_text())
P=640;MASK=(1<<P)-1

def load(path):
 spec=importlib.util.spec_from_file_location('jets105',path)
 mod=importlib.util.module_from_spec(spec);spec.loader.exec_module(mod);return mod

def mul(a,b,width=P):
 out=0
 while b:
  low=b&-b;out^=a<<(low.bit_length()-1);b^=low
 return out&((1<<width)-1)

def square(a,width):
 return sum(1<<(2*i) for i in range((width+1)//2) if a>>i&1)

def inverse(q,width):
 assert q&1
 inv=1;known=1
 while known<width:
  known=min(2*known,width)
  inv=mul(q,square(inv,known),known)
 assert mul(q,inv,width)==1
 return inv

def powers(a,n):
 out=[1]
 for _ in range(n):out.append(mul(out[-1],a))
 return out

def order(a):return None if not a else (a&-a).bit_length()-1

def hull(rows):
 pts=[(r['lambda_exponent'],r['s_order']) for r in rows]
 if any(v is None for j,v in pts):return None
 out=[]
 for p in pts:
  while len(out)>=2:
   a,b=out[-2:]
   if (b[0]-a[0])*(p[1]-b[1])-(b[1]-a[1])*(p[0]-b[0])<=0:out.pop()
   else:break
  out.append(p)
 return [{'lambda_exponent':j,'s_order':v} for j,v in out]

def branch(gterms,x_exps,y_exps,e):
 x=sum(1<<i for i in x_exps);y=sum(1<<i for i in y_exps)
 maxx=max(max(exps) for le,ye,den,exps in gterms)
 maxden=max(den for le,ye,den,exps in gterms)
 xp=powers(x,max(maxx,maxden));yp=powers(y,6)
 coeff={j:{} for j in (0,1,4,8,16)}
 bounds={j:P for j in coeff}
 term_stats=[]
 for le,ye,den,exps in gterms:
  a=0
  for xe in exps:a^=xp[xe]
  a=mul(a,yp[ye])
  d=e*den
  b=xp[den]
  assert order(b)==d and d<P
  n=order(a)
  if n is None:
   bounds[le]=min(bounds[le],P-d)
   term_stats.append({'lambda_exponent':le,'y_exponent':ye,'denominator_exponent':den,
                      'numerator_order':None,'denominator_order':d,'known_until_exclusive':P-d})
   continue
  width=P-max(n,d)
  assert width>0
  ratio=mul(a>>n,inverse(b>>d,width),width)
  offset=n-d;until=offset+width
  bounds[le]=min(bounds[le],until)
  term_stats.append({'lambda_exponent':le,'y_exponent':ye,'denominator_exponent':den,
                     'numerator_order':n,'denominator_order':d,'known_until_exclusive':until})
  for k in range(ratio.bit_length()):
   if ratio>>k&1:
    v=offset+k;coeff[le][v]=coeff[le].get(v,0)^1
 rows=[]
 for j in (0,1,4,8,16):
  support=sorted(v for v,c in coeff[j].items() if c and v<bounds[j])
  rows.append({'lambda_exponent':j,'s_order':support[0] if support else None,
               's_order_lower_bound':support[0] if support else bounds[j],
               'known_until_exclusive':bounds[j],'nonzero_support_below_bound':support})
 assert rows[-1]['s_order']==0 and rows[-1]['nonzero_support_below_bound']==[0]
 return {'coefficient_rows':rows,'newton_lower_hull_if_exact':hull(rows),
         'denominator_order_max':max(t['denominator_order'] for t in term_stats),
         'numerator_order_max_known':max(t['numerator_order'] for t in term_stats if t['numerator_order'] is not None),
         'hidden_numerator_terms':sum(t['numerator_order'] is None for t in term_stats),
         'term_stats':term_stats}

def main():
 out=Path(sys.argv[1])
 for name,p in paths.items():
  if hashlib.sha256(p.read_bytes()).hexdigest()!=protocol['inputs_sha256'][name]:
   raise ValueError('source hash mismatch: '+name)
 audit=json.loads(paths['result98'].read_text())
 assert audit['maximum_exponent']==161
 f,g=load(paths['parser105']).parse_source(paths['donor51'].read_text())
 assert len(g)==29
 base=json.loads(paths['result116'].read_text())
 assert base['precision_exclusive']==P
 result={'source_sha256':protocol['inputs_sha256'],'precision_base_exclusive':P,'g_term_count':len(g),'branches':{}}
 for key,b in base['branches'].items():
  result['branches'][key]=branch(g,b['x_exponents'],b['y_exponents'],b['x_order'])
 out.mkdir(parents=True,exist_ok=True)
 (out/'result.json').write_text(json.dumps(result,indent=2,sort_keys=True)+'\n')
 print('GCOEFF117_ALL_CHECKS_PASS')
if __name__=='__main__':main()
