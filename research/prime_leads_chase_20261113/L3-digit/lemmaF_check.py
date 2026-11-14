# Check Lemma F prediction: for c < D, rank V_c = dim A_{<= c - deg f} (V_c = f * A_{<=c-deg f}), on every recorded trace.
import json,glob
def radices(d):
    k=d['k']; fam=d['fam']; m=d['m']
    if fam.startswith('uni'): blk=[d['B']]
    elif fam.startswith('lin') or fam=='sym': blk={'lin':[2]*k,'linrand':[2]*k,'sym':[2]*k,'linb3':[3]*k}[fam]; return blk
    else: blk={'bool':[2]*k,'supp':[2]*k,'b3':[3]*k,'b4':[4]*k,'mix':[2 if i%2==0 else 3 for i in range(k)]}[fam]
    return blk*m
def dimle(rad,j):
    if j<0: return 0
    poly=[1]
    for b in rad:
        new=[0]*(len(poly)+b-1)
        for i,c in enumerate(poly):
            for e in range(b): new[i+e]+=c
        poly=new
    return sum(poly[:j+1])
tot=ok=0; bad=[]; seen=set(); nrec=0
for f in glob.glob('runs/*.jsonl')+glob.glob('podout/*.jsonl'):
    if 'census' in f: continue
    for l in open(f):
        try: d=json.loads(l)
        except: continue
        if 'trace' not in d or d['target']!='unsat': continue
        key=(d['fam'],d['m'],d['k'],d['B'],d['p'],d['r'])
        if key in seen: continue
        seen.add(key); nrec+=1
        rad=radices(d); fd=d['fdeg']
        tr=d['trace'][:-1] if d.get('censored') else d['trace']
        for (c,cols,rk,hit,secs) in tr:
            if hit: continue
            tot+=1
            pr=dimle(rad,c-fd)
            if pr==rk: ok+=1
            else: bad.append((d['fam'],d['m'],d['k'],d['B'],c,rk,pr))
print(f"Lemma-F rank prediction ({nrec} distinct unsat records): {ok}/{tot} closure levels below D match; mismatches: {bad[:10]}")
