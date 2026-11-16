#!/usr/bin/env sage
"""Frobenius-quotient Pollard rho on the exact saved s=7 source target."""
import hashlib
import json
import os
import pickle
import random
import sys
import time
from pathlib import Path

t_program=time.process_time_ns()
HERE=Path(__file__).resolve().parent
SRC=HERE.parent/'ghs_twist_extension_20261116'
assert len(sys.argv)==2
step_cap=int(sys.argv[1]);assert 1<=step_cap<=50000000
attack_path=HERE/'s7_full_attack.json'
attack=json.loads(attack_path.read_text())
source=json.loads((SRC/'twist_s7_transfer.json').read_text())
attack_sha=hashlib.sha256(attack_path.read_bytes()).hexdigest()
F3=GF(3)
Z=PolynomialRing(F3,'z')
z=Z.gen()
L=GF(3^35,'w',modulus=Z(sage_eval(source['field_modulus'],locals={'x':z},preparse=True)))
w=L.gen()
def vec(t):return [int(c) for c in t.polynomial().list()]
def v(cs):return sum((L(c)*w^i for i,c in enumerate(cs)),L(0))
a=v(source['a_embedding'])
E=EllipticCurve(L,[0,1,0,0,2*(a+a^3)])
P=E(v(attack['source_P'][0]),v(attack['source_P'][1]))
Q=E(v(attack['source_Q'][0]),v(attack['source_Q'][1]))
r=ZZ(attack['r'])
assert is_prime(r) and r*P==E(0) and r*Q==E(0)
def Fr(D):return E(D[0]^243,D[1]^243)
R=PolynomialRing(GF(r),'t')
t=R.gen()
lam=next(int(x) for x,m in (t^2+26*t+243).roots() if Fr(P)==int(x)*P)
assert pow(lam,7,r)==1 and lam!=1 and Fr(Q)==lam*Q
lam_pow=[pow(lam,j,r) for j in range(7)]
def digits(x):
    c=[int(z) for z in x.polynomial().list()]
    return bytes(c+[0]*(35-len(c)))
def canon(D,aa,bb):
    if D.is_zero():return D,aa,bb,b'O'
    xx=D[0]
    best_x=xx
    best_key=digits(xx)
    best_j=0
    for j in range(1,7):
        xx=xx^243
        k=digits(xx)
        if k<best_key:best_x,best_key,best_j=xx,k,j
    if best_j:
        D=E(best_x,D[1]^(243^best_j))
        aa=aa*lam_pow[best_j]%r
        bb=bb*lam_pow[best_j]%r
    return D,aa,bb,best_key+digits(D[1])

checkpoint=HERE/'rho_checkpoint.pkl'
script_sha=hashlib.sha256(Path(__file__).with_suffix('').read_bytes()).hexdigest()
if checkpoint.exists():
    with checkpoint.open('rb') as f:state=pickle.load(f)
    assert state['schema']=='ghs.odd5.twist-s7-rho-checkpoint/v1'
    assert state['script_sha256']==script_sha and state['attack_sha256']==attack_sha
    count=int(state['steps'])
    seen=state['seen']
    jump_coeffs=state['jump_coeffs']
    aa=int(state['aa']);bb=int(state['bb'])
    D=E(v(state['D'][0]),v(state['D'][1])) if state['D'] else E(0)
    trial_cpu_prior=float(state['trial_cpu_s'])
    setup_cpu_s=float(state['setup_cpu_s'])+float((time.process_time_ns()-t_program)/1e9)
else:
    seed=20261116
    rng=random.Random(int(seed))
    jump_coeffs=[(rng.randrange(r),rng.randrange(r)) for _ in range(16)]
    aa=rng.randrange(r);bb=rng.randrange(r)
    D=aa*P+bb*Q
    seen={}
    count=0
    trial_cpu_prior=0.0
    setup_cpu_s=float((time.process_time_ns()-t_program)/1e9)
jump_setup_start=time.process_time_ns()
jumps=[int(u)*P+int(vv)*Q for u,vv in jump_coeffs]
setup_cpu_s+=float((time.process_time_ns()-jump_setup_start)/1e9)
start=time.process_time_ns()
def save_checkpoint():
    trial_cpu_s=float(trial_cpu_prior+(time.process_time_ns()-start)/1e9)
    state={'schema':'ghs.odd5.twist-s7-rho-checkpoint/v1',
           'script_sha256':script_sha,'attack_sha256':attack_sha,
           'steps':int(count),'aa':int(aa),'bb':int(bb),
           'D':[vec(D[0]),vec(D[1])] if not D.is_zero() else None,
           'jump_coeffs':[(int(u),int(vv)) for u,vv in jump_coeffs],
           'seen':seen,'trial_cpu_s':trial_cpu_s,'setup_cpu_s':setup_cpu_s}
    temp=checkpoint.with_suffix('.tmp')
    with temp.open('wb') as f:pickle.dump(state,f,protocol=5)
    os.replace(temp,checkpoint)
    return trial_cpu_s

solution=None
while count<step_cap:
    D,aa,bb,state_key=canon(D,aa,bb)
    digest=hashlib.blake2s(state_key,digest_size=2).digest()
    if digest[1]&127==0:
        if state_key in seen:
            aa0,bb0,step0=seen[state_key]
            den=(bb-bb0)%r
            if den:
                candidate=int((aa0-aa)*inverse_mod(den,r)%r)
                if candidate*P==Q:
                    solution={'scalar':candidate,'collision_steps':[step0,count],
                              'collision_coefficients':[[aa0,bb0],[int(aa),int(bb)]]}
                    break
        else:
            seen[state_key]=(int(aa),int(bb),int(count))
    j=digest[0]&15
    D+=jumps[j]
    aa=(aa+jump_coeffs[j][0])%r
    bb=(bb+jump_coeffs[j][1])%r
    count+=1
    if count%100000==0:
        elapsed=save_checkpoint()
        print(json.dumps({'phase':'rho_progress','steps':int(count),'distinguished':len(seen),
                          'trial_cpu_s':elapsed},sort_keys=True,default=int),flush=True)

trial_cpu_s=save_checkpoint()
if solution is None:
    out={'schema':'ghs.odd5.twist-s7-rho-censored/v1','status':'step_cap',
         'script_sha256':script_sha,'attack_sha256':attack_sha,
         'steps':int(count),'distinguished':len(seen),'setup_cpu_s':setup_cpu_s,
         'trial_cpu_s':trial_cpu_s,'step_cap':step_cap,'eigenvalue':lam}
    (HERE/('rho_s7_cap_'+str(step_cap)+'.json')).write_text(json.dumps(out,indent=2,sort_keys=True,default=int)+'\n')
else:
    out={'schema':'ghs.odd5.twist-s7-rho-result/v1','status':'verified_solve',
         'script_sha256':script_sha,'attack_sha256':attack_sha,
         'r':int(r),'source_P':attack['source_P'],'source_Q':attack['source_Q'],
         'steps':int(count),'distinguished':len(seen),'eigenvalue':lam,
         'setup_cpu_s':setup_cpu_s,'trial_cpu_s':trial_cpu_s,
         'one_solve_cpu_s':setup_cpu_s+trial_cpu_s,
         'solution':solution}
    target=HERE/'rho_s7_result.json'
    assert not target.exists(), 'refusing to overwrite immutable rho receipt'
    target.write_text(json.dumps(out,indent=2,sort_keys=True,default=int)+'\n')
print(json.dumps({k:v for k,v in out.items() if k not in ('source_P','source_Q','attack_sha256','script_sha256')},sort_keys=True,default=int),flush=True)
