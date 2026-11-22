#!/usr/bin/env python3
"""Source-bound order-640 base series at all four marked23 x=0 places."""
import hashlib
import importlib.util
import json
import sys
from pathlib import Path
root=Path(__file__).resolve().parents[1]
paths={'donor51':root/'51/runs/base23-struct51-03/marked23-triangular51.sing',
 'parser105':root/'105/jets105.py','result98':root/'98/result.json',
 'result111':root/'111/result.json','result112':root/'112/result.json',
 'result115':root/'115/result.json'}
protocol=json.loads(Path(__file__).with_name('protocol.json').read_text())
P=640;MASK=(1<<P)-1

def load(path):
 spec=importlib.util.spec_from_file_location('jets105',path)
 mod=importlib.util.module_from_spec(spec);spec.loader.exec_module(mod);return mod

def pmul(a,b):
 out=set()
 for sa,za in a:
  for sb,zb in b:
   p=(sa+sb,za+zb)
   if p in out:out.remove(p)
   else:out.add(p)
 return out

def ppow(a,n):
 out={(0,0)}
 for _ in range(n):out=pmul(out,a)
 return out

def compose_source(f,xpoly,ypoly,divide):
 terms=set()
 for ye,exps in f:
  yp=ppow(ypoly,ye)
  for xe in exps:
   xp=ppow(xpoly,xe)
   for p in pmul(xp,yp):
    if p in terms:terms.remove(p)
    else:terms.add(p)
 if any(se<divide for se,ze in terms):raise ValueError('source division failed')
 return sorted((se-divide,ze) for se,ze in terms)

def mul(a,b):
 out=0
 while b:
  low=b&-b;out^=a<<(low.bit_length()-1);b^=low
 return out&MASK

def powers(a,n):
 out=[1]
 for _ in range(n):out.append(mul(out[-1],a))
 return out

def order(a):return None if not a else (a&-a).bit_length()-1

def eval_terms(terms,z):
 zp=powers(z,max(ze for se,ze in terms))
 out=0
 for se,ze in terms:out^=zp[ze]<<se
 return out&MASK

def lift(terms,initial):
 assert eval_terms(terms,initial)&1==0
 z=initial
 for k in range(P):
  if eval_terms(terms,z)>>k&1:z^=1<<k
 assert eval_terms(terms,z)==0
 return z

def source_residual(f,x,y):
 xp=powers(x,max(max(exps) for ye,exps in f));yp=powers(y,7)
 out=0
 for ye,exps in f:
  coeff=0
  for xe in exps:coeff^=xp[xe]
  out^=mul(coeff,yp[ye])
 return out

def main():
 outdir=Path(sys.argv[1])
 for name,path in paths.items():
  if hashlib.sha256(path.read_bytes()).hexdigest()!=protocol['inputs_sha256'][name]:
   raise ValueError('source hash mismatch: '+name)
 audit=json.loads(paths['result98'].read_text())
 assert audit['maximum_exponent']==161
 prior115=json.loads(paths['result115'].read_text())
 assert prior115['precision_t_exclusive']==128
 f,g=load(paths['parser105']).parse_source(paths['donor51'].read_text())
 assert len(f)==8 and len(g)==29
 prior111=json.loads(paths['result111'].read_text())
 prior112=json.loads(paths['result112'].read_text())
 assert [e['z_valuation'] for e in prior111['y0']['positive_edges']]==['1','1/3']
 assert prior112['positive_edges'][0]['w_valuation']=='5'
 configs=[
  ('y0_e1',{(1,0)},{(1,1)},2,1,1),
  ('y0_e3',{(3,0)},{(1,1)},4,1,3),
  ('y1_e1',{(1,0)},{(0,0),(1,0),(6,1)},10,1,1),
 ]
 result={'source_sha256':protocol['inputs_sha256'],'precision_exclusive':P,
         'maximum_G_denominator_exponent':161,'branches':{}}
 for key,xpoly,ypoly,divide,initial,e in configs:
  terms=compose_source(f,xpoly,ypoly,divide)
  special=sorted(ze for se,ze in terms if se==0)
  assert special==({'y0_e1':[0,1],'y0_e3':[1,4],'y1_e1':[0,1]}[key])
  assert sum(ze==1 for ze in special)==1
  z=lift(terms,initial)
  s=2
  if key=='y0_e1':x=s;y=mul(s,z)
  elif key=='y0_e3':x=8;y=mul(s,z)
  else:x=s;y=1^s^mul(1<<6,z)
  assert order(x)==e and source_residual(f,x,y)==0
  result['branches'][key]={'equation_support':[[a,b] for a,b in terms],
   'equation_dividing_s_order':divide,'root_initial':initial,
   'x_order':e,'y_order':order(y),'denominator_pullback_order':161*e,
   'x_exponents':[i for i in range(P) if x>>i&1],
   'y_exponents':[i for i in range(P) if y>>i&1],
   'root_exponents':[i for i in range(P) if z>>i&1]}
 # The wild branch uses E(x,t) from the source-bound R115 result.
 terms=[tuple(p) for p in prior115['E_support']]
 assert sorted(se for se,ze in terms if ze==0)==[2] and (0,1) in terms
 x=lift(terms,0);s=2;xp=powers(x,3)
 y=1^x^xp[2]^mul(xp[3],s)
 assert order(x)==2 and source_residual(f,x,y)==0
 assert [i for i in prior115['x_exponents']]==[i for i in range(128) if x>>i&1]
 assert [i for i in prior115['y_exponents']]==[i for i in range(128) if y>>i&1]
 result['branches']['y1_e2']={'equation_support':[[a,b] for a,b in terms],
  'equation_dividing_s_order':0,'root_initial':0,'x_order':2,
  'y_order':order(y),'denominator_pullback_order':322,
  'x_exponents':[i for i in range(P) if x>>i&1],
  'y_exponents':[i for i in range(P) if y>>i&1],
  'root_exponents':[i for i in range(P) if x>>i&1]}
 assert max(b['denominator_pullback_order'] for b in result['branches'].values())==483<P
 outdir.mkdir(parents=True,exist_ok=True)
 (outdir/'result.json').write_text(json.dumps(result,indent=2,sort_keys=True)+'\n')
 print('SERIES116_ALL_CHECKS_PASS')
if __name__=='__main__':main()
