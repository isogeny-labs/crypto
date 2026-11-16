import json,collections,glob
print("| cell | p | #targets | unsat | sat | D distribution (unsat) | D distribution (sat; detection = all degree-<=1 consequences) | ffd bound | targets with D < ffd |")
print("|---|---|---|---|---|---|---|---|---|")
for f in sorted(glob.glob('podout/census_*.jsonl')):
    rows=[json.loads(l) for l in open(f)]
    p=rows[0]['p']; fam=rows[0]['fam']; k=rows[0]['k']
    ffd={('bool',4):6,('bool',5):6,('b3',3):7}[(fam,k)]
    un=collections.Counter(r['D'] for r in rows if r['nsol']==0)
    sa=collections.Counter(r['D'] for r in rows if r['nsol']>0)
    low=sum(1 for r in rows if isinstance(r['D'],int) and r['D']<ffd)
    cens=sum(1 for r in rows if not isinstance(r['D'],int))
    print(f"| m=2 {fam} k={k} (B={rows[0]['B']}) | {p} | {len(rows)} | {sum(un.values())} | {sum(sa.values())} | {dict(sorted(un.items()))} | {dict(sorted(sa.items()))} | {ffd} | {low} (censored {cens}) |")
