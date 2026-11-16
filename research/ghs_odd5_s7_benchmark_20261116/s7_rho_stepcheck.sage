#!/usr/bin/env sage
"""Matched order-seven Frobenius plus negation rho on the saved s=7 target."""
import hashlib
import json
import random
import time
from pathlib import Path

program_start=time.process_time_ns()
here=Path(__file__).resolve().parent
inp=here/'s7_transfer.json';d=json.loads(inp.read_text())
F3=GF(3);Z=PolynomialRing(F3,'z')
L=GF(3^35,'w',modulus=Z(d['field_modulus']))
a=L(d['a_embedding'])
E=EllipticCurve(L,[0,2,0,0,a+a^3])
P=E(L(d['P'][0]),L(d['P'][1]));Q=E(L(d['Q'][0]),L(d['Q'][1]))
r=ZZ(d['r']);secret=ZZ(d['secret']);lam=ZZ(1048494599)
assert is_prime(r) and r*P==E(0) and Q==secret*P
Fr=lambda T:E(T[0]^243,T[1]^243)
assert Fr(P)==lam*P and Fr(Q)==lam*Q and pow(int(lam),7,int(r))==1
setup_cpu_s=float((time.process_time_ns()-program_start)/1e9)

def key(T):
 if T.is_zero():return (-1,)
 def digits(v):
  vv=[int(c) for c in v.polynomial().list()]
  return tuple(vv+[0]*(35-len(vv)))
 return digits(T[0])+digits(T[1])

def canon(T,aa,bb):
 if T.is_zero():return T,aa,bb,(-1,)
 choices=[];D=T;factor=1
 for _ in range(7):
  choices.append((key(D),D,aa*factor%r,bb*factor%r))
  choices.append((key(-D),-D,-aa*factor%r,-bb*factor%r))
  D=Fr(D);factor=factor*lam%r
 state,D,aa,bb=min(choices,key=lambda v:v[0])
 return D,aa,bb,state

def trial(seed):
 rng=random.Random(int(seed));start=time.process_time_ns();steps_taken=0
 for restart in range(100):
  jumps=[(rng.randrange(int(r)),rng.randrange(int(r))) for _ in range(32)]
  jump_points=[u*P+v*Q for u,v in jumps]
  aa=rng.randrange(int(r));bb=rng.randrange(int(r));D=aa*P+bb*Q
  seen={}
  for _ in range(100000):
   D,aa,bb,state=canon(D,aa,bb)
   if steps_taken<10:
    assert D==aa*P+bb*Q
    print('STEP',int(steps_taken),'A',int(aa),'B',int(bb),'KEY',state[:8],flush=True)
   if steps_taken==10:raise SystemExit(0)
   if state in seen:
    aa0,bb0=seen[state];den=(bb-bb0)%r
    print(json.dumps({'phase':'collision','restart':restart,'steps':steps_taken,
                      'den_zero':bool(den==0),'aa0':int(aa0),'aa':int(aa),
                      'bb0':int(bb0),'bb':int(bb)},default=int),flush=True)
    if den:
     candidate=((aa0-aa)*inverse_mod(den,r))%r
     if candidate*P==Q:
      return {'seed':seed,'cpu_s':float((time.process_time_ns()-start)/1e9),
              'steps':steps_taken,'restarts':restart,'scalar':int(candidate)}
    break
   seen[state]=(aa,bb)
   j=(hashlib.blake2s(bytes(state),digest_size=2).digest()[0]%len(jumps)
      if state!=(-1,) else 0)
   D+=jump_points[j];aa=(aa+jumps[j][0])%r;bb=(bb+jumps[j][1])%r
   steps_taken+=1
   if steps_taken>=1000000:break
  if steps_taken>=1000000:break
 raise RuntimeError('rho cap reached without a verified scalar')

results=[]
for i in range(1):
 seed=20261116700+i
 result=trial(seed)
 assert result['scalar']==secret
 result.update({'schema':'ghs.odd5.s7-rho-trial/v1',
                'source_sha256':hashlib.sha256(Path(__file__).with_suffix('').read_bytes()).hexdigest(),
                'transfer_receipt_sha256':hashlib.sha256(inp.read_bytes()).hexdigest(),
                'r':int(r),'negation_and_frobenius_orbit_size':14})
 target=here/('rho_debug_trial_%02d.json'%(i+1))
 assert not target.exists(), 'refusing to overwrite rho trial'
 target.write_text(json.dumps(result,sort_keys=True)+'\n')
 results.append(result)
 print(json.dumps({k:result[k] for k in ('seed','cpu_s','steps','restarts','scalar')},sort_keys=True),flush=True)

times=sorted(z['cpu_s'] for z in results)
out={'schema':'ghs.odd5.s7-rho-summary/v1',
     'source_sha256':hashlib.sha256(Path(__file__).with_suffix('').read_bytes()).hexdigest(),
     'transfer_receipt_sha256':hashlib.sha256(inp.read_bytes()).hexdigest(),
     'r':int(r),'trials':len(results),'setup_cpu_s':setup_cpu_s,
     'median_trial_cpu_s':(times[14]+times[15])/2,
     'median_one_solve_cpu_s':setup_cpu_s+(times[14]+times[15])/2,
     'mean_trial_cpu_s':float(sum(times)/len(times)),
     'all_scalars_verified':True}
target=here/'rho_debug_summary.json'
assert not target.exists(), 'refusing to overwrite summary'
target.write_text(json.dumps(out,sort_keys=True,indent=2)+'\n')
print(json.dumps(out,sort_keys=True))
