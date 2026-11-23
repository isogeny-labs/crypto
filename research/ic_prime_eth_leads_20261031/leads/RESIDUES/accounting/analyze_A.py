#!/usr/local/bin/python3.13
import json,glob,math,random,re,collections,os
os.chdir(os.path.dirname(os.path.abspath(__file__)))
random.seed(7)
rows=collections.defaultdict(list)  # (b,seed)->runs
for f in sorted(glob.glob('../cube/raw/sweep/b*-s*/runs.jsonl')):
    m=re.search(r'b(\d+)-s(\d+)',f); b,s=int(m[1]),int(m[2])
    for l in open(f):
        d=json.loads(l); d['seed']=s; rows[(b,s)].append(d)
CAP=100000
def cens(d): return d['status']=='UNKNOWN'
def stats(runs,C):
    ch=sum(min(d['conflicts'],C) for d in runs)
    rel=sum(1 for d in runs if d['status']=='SAT' and d['conflicts']<=C)
    dec=sum(1 for d in runs if d['status'] in('SAT','UNSAT') and d['verdict']=='ok' and d['conflicts']<=C)
    pr=sum(d['propagations'] if d['conflicts']<=C else d['propagations']*C/max(d['conflicts'],1) for d in runs)
    return rel,dec,ch,pr
def boot(curves,C,B=4000):
    # curves: list of run-lists. ratio-of-sums over curves, bootstrap over curves
    def est(ix):
        r=d=ch=pr=0
        for i in ix:
            a,b_,c,p=stats(curves[i],C); r+=a;d+=b_;ch+=c;pr+=p
        return r/ch*1e3, d/ch*1e3, r/pr*1e6 if pr else float('nan')
    n=len(curves); pt=est(range(n)); bs=[est([random.randrange(n) for _ in range(n)]) for _ in range(B)]
    ci=[(sorted(x[k] for x in bs)[int(.025*B)],sorted(x[k] for x in bs)[int(.975*B)]) for k in range(3)]
    return pt,ci
out=[]
out.append('## Relations per conflict at fixed cutoff C (relation = SAT answer; censored charged C; CI = bootstrap over curves, 4000 draws)')
out.append('b | curves | targets | SAT-oracle | cutoff C | relations | answers(SAT+UNSAT) | conflicts charged | rel per 1k conflicts [95% CI] | answers per 1k conflicts [CI] | rel per 1M propagations [CI] | censored@C | mean T_enum | conflicts per relation')
summary={}
for b in (6,9,12):
    cs=[rows[k] for k in sorted(rows) if k[0]==b and len(rows[k])>0]
    if not cs: continue
    allr=[d for c in cs for d in c]
    for C in (10000,30000,100000):
        pt,ci=boot(cs,C)
        rel,dec,ch,pr=[sum(stats(c,C)[i] for c in cs) for i in range(4)]
        cen=sum(1 for d in allr if d['conflicts']>C or cens(d))
        te=sum(d['t_enum'] for d in allr)/len(allr)
        out.append(f"{b} | {len(cs)} | {len(allr)} | {sum(d['oracle_sat'] for d in allr)} | {C} | {rel} | {dec} | {ch} | {pt[0]:.3f} [{ci[0][0]:.3f},{ci[0][1]:.3f}] | {pt[1]:.3f} [{ci[1][0]:.3f},{ci[1][1]:.3f}] | {pt[2]:.3f} [{ci[2][0]:.3f},{ci[2][1]:.3f}] | {cen}/{len(allr)} | {te:.0f} | {ch/rel if rel else float('nan'):.0f}")
        summary[(b,C)]=pt[0]
out.append('')
out.append('verdict mix per b (kissat status): '+'; '.join(f"b={b}: "+str(dict(collections.Counter(d['status'] for k in rows if k[0]==b for d in rows[k]))) for b in (6,9,12)))
out.append('verifier verdicts non-ok: '+str(sum(1 for k in rows for d in rows[k] if d['verdict'] not in('ok','timeout'))))
out.append('fb_xs by b: '+'; '.join(f"b={b}: "+str(sorted({d['fb_xs'] for k in rows if k[0]==b for d in rows[k]})) for b in (6,9,12)))
out.append('')
out.append('descriptive: log2 ratio of conflicts-per-relation, C=100000: '+'; '.join(f"b{a}->b{c}: "+(f"{math.log2(summary[(a,100000)]/summary[(c,100000)]):+.2f}" if summary.get((a,100000)) and summary.get((c,100000)) else 'undefined (zero relations at one size)') for a,c in ((6,9),(9,12)))+'  (positive = yield per conflict falls; b=12 is censored so its charge is a floor on cost, i.e. this is a lower bound on the drop)')
open('raw/A_yield.txt','w').write('\n'.join(out)+'\n'); print('\n'.join(out))
