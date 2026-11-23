#!/usr/bin/env python3
"""SNC-1 Stage 0: DERIVED verify share of the inc-c2 arm from committed c1 confirm sessions.
verify_adds is not exported by the wwalk oracle (only TargetWalk's tw_verify_adds is).  Per record:
  A = oracle_setup.adds - table_insert_attempts      (= setup_adds + verify_adds + identity steps)
  C = relations.free_collisions_checked              (collisions verified; 2 evals each)
OLS of A on C over the records of one k: intercept = setup (jump points, jump-seed dependent), slope = verify adds per collision.
Derived verify_i = slope * C_i; share = mean(verify_i)/sqrt(r_i) (sqrt(r) units) and / S."""
import json,sys,math,statistics as st
def load(k):
    rows=[]
    for l in open(f'c1/sessions/confirm-k{k}/records.jsonl'):
        d=json.loads(l)
        if d['arm']!='inc-c2' or d['warmup']: continue
        ph={p['name']:p for p in d['phases']}
        o=ph['oracle_setup']; rel=ph['relations']
        ins=o['work_native'].get('table_insert_attempts')
        rows.append(dict(r=d['workload']['curve_spec']['r'],S=d['cost']['s'],A=o['adds']-ins,C=rel['native'].get('free_collisions_checked',0),
          Dbl=o['doubles'],ins=ins,setup_gae=o['gae'],oracle=d['method']['params']['oracle'],ver_phase=ph['verify']['gae']))
    return rows
def ols(x,y):
    mx,my=st.mean(x),st.mean(y); sxx=sum((a-mx)**2 for a in x)
    b=sum((a-mx)*(c-my) for a,c in zip(x,y))/sxx; a0=my-b*mx
    res=[c-(a0+b*a) for a,c in zip(x,y)]; s2=sum(e*e for e in res)/(len(x)-2)
    return a0,b,math.sqrt(s2/sxx)
for k in (20,24,28,32):
    R=load(k); n=len(R)
    print(f'k={k} n={n} oracle={R[0]["oracle"]} mean S={st.mean(x["S"] for x in R):.3f} mean collisions C={st.mean(x["C"] for x in R):.3f} dist={ {c:sum(1 for x in R if x["C"]==c) for c in sorted(set(x["C"] for x in R))} }')
    if k==20: print('  box wsum:free=1 : oracle_setup.adds = table inserts + ?; see RESULTS.md'); 
    if len(set(x['C'] for x in R))<2: print('  no variation in C, no regression'); continue
    a0,b,se=ols([x['C'] for x in R],[x['A'] for x in R])
    print(f'  OLS A = {a0:.1f} + {b:.2f}(se {se:.2f}) * C  [adds per collision, 2 evals each]')
    # sqrt(r)-unit verify share, derived
    v=[b*x['C']/math.sqrt(x['r']) for x in R]
    print(f'  derived verify share = {st.mean(v):.4f} sqrt(r)  (se {st.pstdev(v)/math.sqrt(n):.4f}); share of S = {st.mean(v)/st.mean(x["S"] for x in R):.4f}; with slope+-se: [{(b-se)*st.mean(x["C"]/math.sqrt(x["r"]) for x in R):.4f},{(b+se)*st.mean(x["C"]/math.sqrt(x["r"]) for x in R):.4f}]')
