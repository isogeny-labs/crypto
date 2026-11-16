#!/usr/bin/env sage
"""Matched source-group rho with setup accounted in a separate process."""
import hashlib
import json
import random
import time
from pathlib import Path

t0=time.process_time_ns()
PTH=Path(__file__).resolve().parent
d=json.loads((PTH/'s3_full_attack.json').read_text())
F3=GF(3);Z=PolynomialRing(F3,'z')
L=GF(3^15,'w',modulus=Z(d['field_modulus']));w=L.gen()
def v(cs):return sum((L(c)*w^i for i,c in enumerate(cs)),L(0))
a=v(d['a_embedding']);E=EllipticCurve(L,[0,2,0,0,a+a^3])
P=E(v(d['source_P'][0]),v(d['source_P'][1]))
Q=E(v(d['source_Q'][0]),v(d['source_Q'][1]))
r=ZZ(d['r']);assert is_prime(r) and r*P==E(0) and r*Q==E(0)
Fr=lambda Pt:E(Pt[0]^243,Pt[1]^243)
R=PolynomialRing(GF(r),'t');t=R.gen()
lam=next(int(z) for z,m in (t^2-26*t+243).roots() if Fr(P)==int(z)*P)
assert lam==d['rho_eigenvalue'] and pow(lam,3,r)==1 and lam!=1 and Fr(Q)==lam*Q
setup_cpu_s=float((time.process_time_ns()-t0)/1e9)
def digits(x):
 a=[int(c) for c in x.polynomial().list()]
 return tuple(a+[0]*(15-len(a)))
def key(Pt):return digits(Pt[0])+digits(Pt[1])
def canon(Pt,aa,bb):
 if Pt.is_zero():return Pt,aa,bb
 choices=[(Pt,aa,bb)];D=Pt
 for j in (1,2):
  D=Fr(D);choices.append((D,aa*pow(lam,j,r)%r,bb*pow(lam,j,r)%r))
 return min(choices,key=lambda z:key(z[0]))
def trial(seed):
 rng=random.Random(int(seed));start=time.process_time_ns();count=0
 for restart in range(100):
  steps=[(rng.randrange(r),rng.randrange(r)) for _ in range(16)]
  jumps=[u*P+v*Q for u,v in steps]
  aa=rng.randrange(r);bb=rng.randrange(r);D=aa*P+bb*Q;seen={}
  for _ in range(5000):
   D,aa,bb=canon(D,aa,bb);state=key(D) if not D.is_zero() else ('O',)
   if state in seen:
    aa0,bb0=seen[state];den=(bb-bb0)%r
    if den:
     candidate=((aa0-aa)*inverse_mod(den,r))%r
     if candidate*P==Q:
      return {'cpu_s':float((time.process_time_ns()-start)/1e9),
              'steps':count,'restarts':restart,'scalar':int(candidate)}
    break
   seen[state]=(aa,bb)
   j=hashlib.blake2s(bytes(state),digest_size=2).digest()[0]%len(jumps) if state!=('O',) else 0
   D+=jumps[j];aa=(aa+steps[j][0])%r;bb=(bb+steps[j][1])%r;count+=1
   if count>=20000:break
  if count>=20000:break
 raise RuntimeError('rho cap reached')
trials=[trial(20261116+i) for i in range(30)]
assert all(x['scalar']==d['validation_scalar'] for x in trials)
median=sorted(x['cpu_s'] for x in trials)[15]
out={'schema':'ghs.odd5.s3-rho-standalone/v1','source_sha256':hashlib.sha256(Path(__file__).with_suffix('').read_bytes()).hexdigest(),
     'attack_receipt_sha256':hashlib.sha256((PTH/'s3_full_attack.json').read_bytes()).hexdigest(),
     'r':int(r),'validation_scalar':d['validation_scalar'],'frobenius_eigenvalue':lam,
     'setup_cpu_s':setup_cpu_s,'median_trial_cpu_s':median,'median_one_solve_cpu_s':setup_cpu_s+median,
     'median_steps':sorted(x['steps'] for x in trials)[15],'trials':trials}
target=PTH/'s3_rho_standalone.json';assert not target.exists()
target.write_text(json.dumps(out,indent=2,sort_keys=True,default=int)+'\n')
print(json.dumps({k:out[k] for k in ['schema','r','setup_cpu_s','median_trial_cpu_s','median_one_solve_cpu_s','median_steps']},sort_keys=True,default=int))
