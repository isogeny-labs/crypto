#!/usr/bin/env python3
"""Source-bound direct Hensel lift of the x=0 wild base branch."""
import hashlib
import importlib.util
import json
import sys
from pathlib import Path
root=Path(__file__).resolve().parents[1]
paths={'donor51':root/'51/runs/base23-struct51-03/marked23-triangular51.sing',
       'parser105':root/'105/jets105.py','result113':root/'113/result.json'}
protocol=json.loads(Path(__file__).with_name('protocol.json').read_text())
P=128;MASK=(1<<P)-1

def load(path):
 spec=importlib.util.spec_from_file_location('jets105',path)
 mod=importlib.util.module_from_spec(spec);spec.loader.exec_module(mod);return mod

def mul(a,b):
 out=0
 while b:
  if b&1:out^=a
  a<<=1;b>>=1
 return out&MASK

def powers(a,n):
 out=[1]
 for _ in range(n):out.append(mul(out[-1],a))
 return out

def order(a):return None if a==0 else (a&-a).bit_length()-1

def main():
 out=Path(sys.argv[1])
 for name,path in paths.items():
  if hashlib.sha256(path.read_bytes()).hexdigest()!=protocol['inputs_sha256'][name]:
   raise ValueError('source hash mismatch: '+name)
 prior=json.loads(paths['result113'].read_text())
 assert prior['positive_edges'][0]['v_valuation']=='3/2'
 terms=set()
 for ve,xe in prior['support']:
  p=(ve,xe+ve-2)
  if p in terms:terms.remove(p)
  else:terms.add(p)
 if any(xe<0 for te,xe in terms):raise ValueError('x^2 division failed')
 assert sorted(te for te,xe in terms if xe==0)==[2]
 assert (0,1) in terms
 max_x=max(xe for te,xe in terms)
 def eval_E(x):
  xp=powers(x,max_x)
  out=0
  for te,xe in terms:out^=xp[xe]<<te
  return out&MASK
 x=0
 for k in range(P):
  if eval_E(x)>>k&1:x^=1<<k
 assert eval_E(x)==0 and order(x)==2
 t=2;xp=powers(x,3)
 v=mul(x,t)
 y=1^x^xp[2]^mul(xp[3],t)
 assert order(v)==3
 f,g=load(paths['parser105']).parse_source(paths['donor51'].read_text())
 xpow=powers(x,max(max(exps) for ye,exps in f))
 ypow=powers(y,7)
 residual=0
 for ye,exps in f:
  coeff=0
  for xe in exps:coeff^=xpow[xe]
  residual^=mul(coeff,ypow[ye])
 assert residual==0
 result={'source_sha256':protocol['inputs_sha256'],'precision_t_exclusive':P,
         'E_support':sorted([list(p) for p in terms]),
         'E_special_fiber_exponents':[2], 'E_dx_origin':1,
         'x_exponents':[i for i in range(P) if x>>i&1],
         'y_exponents':[i for i in range(P) if y>>i&1],
         'x_order':order(x),'v_order':order(v),'f_residual_mod_t128':residual}
 out.mkdir(parents=True,exist_ok=True)
 (out/'result.json').write_text(json.dumps(result,indent=2,sort_keys=True)+'\n')
 print('UNIFORMIZER115_ALL_CHECKS_PASS')
if __name__=='__main__':main()
