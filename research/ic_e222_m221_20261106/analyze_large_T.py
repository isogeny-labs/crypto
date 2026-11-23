#!/usr/bin/env python3
"""Reproduces the large-T comparison with the COMPLETE, symmetric ledger (validator corrections applied).
Inputs: results/large_T_holdout/rows.json (IC v3 + anchored rho, kappa 0.5/0.7/1.0), results/bsgs_large_T/rows.json (BSGS, mfactor 0.7/1/1.4).
Unpriced operations, each priced at 0 / 0.1 / 1 GAE:
  IC  : 2*m^2 table-insert attempts (v2/v3 build) + trials_g + trials_targets (one hash per trial) + sparse-solve ops (la_ops) + 4*m*pbits factor-base build (hand estimate)
  rho : one hash per walk step: database-phase steps (adds_db minus bank scalar mults minus restart additions) + target_steps  [exact count: see below]
  BSGS: baby inserts + giant lookups.
rho hash count: adds_db = 64*ceil(1.5 bits) + 2*ceil(steps/32) + steps  (validator-verified identity on 264/264 rows) -> solve for steps.
Each arm's best parameter (kappa / mfactor) is chosen per configuration at each price (favours the generic arms)."""
import json,math,random,statistics as S,collections,sys
R=json.load(open("results/large_T_holdout/rows.json"));BS=json.load(open("results/bsgs_large_T/rows.json"))
IC=[r for r in R if r["kind"]=="IC"];RH=[r for r in R if r["kind"]=="RHO"]
key=lambda r:(r["pbits"],r["idx"],r["T"],r["seed"])
def rho_db_steps(r):
    bits=r["n_bits"];c=math.ceil(1.5*bits)
    rem=r["adds_db"]-64*c
    s=rem*32//34  # rem = 2*ceil(s/32)+s ~ s*(1+2/32)
    while 2*math.ceil(s/32)+s<rem: s+=1
    while 2*math.ceil(s/32)+s>rem: s-=1
    return s
ic_unp=lambda r:2*r["m"]**2+r["trials_g"]+r["trials_targets"]+r["la_ops"]+4*r["m"]*r["pbits"]
rho_unp=lambda r:rho_db_steps(r)+r["target_steps"]
bs_unp=lambda r:r["baby_inserts"]+r["giant_lookups"]
rk=collections.defaultdict(list);bk=collections.defaultdict(list)
for r in RH: rk[key(r)].append(r)
for r in BS: bk[key(r)].append(r)
rng=random.Random(17)
print("IC runs",len(IC),"rho runs",len(RH),"BSGS runs",len(BS),"| all verified:",all(r["ok"]==r["targets"] for r in IC+RH+BS))
print("complete symmetric ledger; ratios are medians over 22 paired configurations [bootstrap 95%]; 'cheaper' = configs where IC < the best generic arm")
print("pbits | T | price | IC/rho | IC/BSGS | BSGS/rho | IC/min(rho,BSGS) [boot95] | IC cheaper than best generic")
for pb in (24,25):
    for T in (4096,16384):
        c=[r for r in IC if r["pbits"]==pb and r["T"]==T]
        for price in (0,0.1,1.0):
            a=[];b=[];cc=[];m=[]
            for r in c:
                ic=(r["adds_total"]+price*ic_unp(r))/T
                rho=min((x["adds_total"]+price*rho_unp(x))/T for x in rk[key(r)])
                bs=min((x["adds_total"]+price*bs_unp(x))/T for x in bk[key(r)])
                a.append(ic/rho);b.append(ic/bs);cc.append(bs/rho);m.append(ic/min(rho,bs))
            bsr=sorted(S.median(rng.choices(m,k=len(m))) for _ in range(2000))
            print("%d | %d | %.1f | %.3f | %.3f | %.3f | %.3f [%.3f,%.3f] | %d/%d"%(pb,T,price,S.median(a),S.median(b),S.median(cc),S.median(m),bsr[50],bsr[1950],sum(x<1 for x in m),len(m)))
