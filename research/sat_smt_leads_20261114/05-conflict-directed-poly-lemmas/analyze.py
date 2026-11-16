import json, math, statistics as st, random
rows=[json.loads(x) for x in open('rows.jsonl')]
def name(r): return r['arm'] if r['arm'] in('A0','A1') else f"{r['arm']}k{r['k']}"
for r in rows: r['armname']=name(r)
arms=['A0','A1','A2k8','A3k8','A2k12','A3k12']; ls=sorted({r['l'] for r in rows})
gm=lambda v: 2**(sum(math.log2(x) for x in v)/len(v)) if v else None
def sel(l,arm,status=None): return [r for r in rows if r['l']==l and r['armname']==arm and (status is None or r['status']==status)]
truth={(r['l'],r['idx']):('SAT' if r['bf_count']>0 else 'UNSAT') for r in rows}
def selt(l,arm,t): return [r for r in rows if r['l']==l and r['armname']==arm and (t is None or truth[(r['l'],r['idx'])]==t)]
summ={'cells':[], 'ratios':[], 'slopes':{}}
T=[]
T.append('| l | truth | n | arm | median confl | geomean confl | median decisions | median props | mean lemmas | mean lemma len | mean rounds | gen word-XORs (mean) | timeouts |')
T.append('|---|---|---|---|---|---|---|---|---|---|---|---|---|')
for l in ls:
  for t in (None,'UNSAT','SAT'):
    for a in arms:
        R=selt(l,a,t)
        if not R: continue
        c=[r['conflicts'] for r in R]
        d=dict(l=l,truth=t or 'all',arm=a,n=len(R),median_conflicts=st.median(c),geomean_conflicts=gm(c),
               median_decisions=st.median(r['decisions'] for r in R),median_propagations=st.median(r['propagations'] for r in R),
               mean_lemmas=st.mean(r['lemmas'] for r in R),mean_lemma_len=st.mean(r['lemma_mean_len'] for r in R),
               mean_rounds=st.mean(r['rounds'] for r in R),mean_gen_word_xors=st.mean(r['gen_word_xors'] for r in R),
               timeouts=sum(r['status']=='TIMEOUT' for r in R))
        summ['cells'].append(d)
        T.append(f"| {l} | {d['truth']} | {d['n']} | {a} | {d['median_conflicts']:.0f} | {d['geomean_conflicts']:.0f} | {d['median_decisions']:.0f} | {d['median_propagations']:.0f} | {d['mean_lemmas']:.1f} | {d['mean_lemma_len']:.1f} | {d['mean_rounds']:.1f} | {d['mean_gen_word_xors']:.0f} | {d['timeouts']} |")
def ratio(l,num,den,t=None):
    A={r['idx']:r['conflicts'] for r in selt(l,num,t)}; B={r['idx']:r['conflicts'] for r in selt(l,den,t)}
    q=[A[i]/B[i] for i in A if i in B]
    if not q: return None
    rng=random.Random(5); bs=sorted(gm([rng.choice(q) for _ in q]) for _ in range(2000))
    return dict(l=l,truth=t or 'all',num=num,den=den,n=len(q),geomean_ratio=gm(q),ci95=[bs[50],bs[1949]],min=min(q),max=max(q))
T2=['| l | truth | ratio | n | geomean | bootstrap 95% | min | max |','|---|---|---|---|---|---|---|---|']
for l in ls:
  for t in (None,'UNSAT','SAT'):
    for num,den in (('A3k8','A2k8'),('A3k12','A2k12'),('A0','A2k8'),('A0','A2k12'),('A0','A3k8'),('A0','A3k12'),('A0','A1')):
        d=ratio(l,num,den,t)
        if d: summ['ratios'].append(d); T2.append(f"| {l} | {d['truth']} | {num}/{den} | {d['n']} | {d['geomean_ratio']:.2f} | {d['ci95'][0]:.2f}..{d['ci95'][1]:.2f} | {d['min']:.2f} | {d['max']:.2f} |")
def slope(xs,ys):
    mx=st.mean(xs); my=st.mean(ys); return sum((x-mx)*(y-my) for x,y in zip(xs,ys))/sum((x-mx)**2 for x in xs)
T3=['| arm | truth | slope bits/l, l=2..4 (log2 median) | slope l=3..4 only | slope (log2 geomean) l=2..4 |','|---|---|---|---|---|']
for t in (None,'UNSAT','SAT'):
  for a in arms:
    ok=[l for l in ls if selt(l,a,t)]
    if len(ok)<2: continue
    med=[math.log2(st.median(r['conflicts'] for r in selt(l,a,t))) for l in ok]
    g=[math.log2(gm([r['conflicts'] for r in selt(l,a,t)])) for l in ok]
    s1=slope(ok,med); s2=med[-1]-med[-2]; s3=slope(ok,g)
    summ['slopes'][f"{a}/{t or 'all'}"]=dict(ls=ok,slope_median=s1,slope_last_two=s2,slope_geomean=s3)
    T3.append(f"| {a} | {t or 'all'} | {s1:.2f} | {s2:.2f} | {s3:.2f} |")
T3+=['| reference: enumeration 2^(3l) | - | 3.00 | 3.00 | 3.00 |','| reference: MITM ~2^(2l) | - | 2.00 | 2.00 | 2.00 |','| reference: linear 2+1 scan 2^l leaf solves | - | 1.00 | 1.00 | 1.00 |']
val=dict(rows=len(rows),status_matches_bruteforce=all(r['status_matches_bf'] for r in rows),
  sat_models_algebraic_ok=all(r['model_alg_ok'] for r in rows if 'model_alg_ok' in r),
  sat_models=sum('model_alg_ok' in r for r in rows), sat_models_lift=sum(r.get('model_lifts',False) for r in rows),
  planted_all_sat=all(r['status']=='SAT' for r in rows if r['planted'] and r['status']!='TIMEOUT'),
  lemmas_sound_on_planted=all(r['lemmas_sound_on_planted'] for r in rows if 'lemmas_sound_on_planted' in r),
  identical_to_A0={l:sum(1 for r in sel(l,'A2k8') if r['lemmas']==0) for l in ls})
summ['validation']=val
open('tables.md','w').write('\n'.join(T)+'\n\n'+'\n'.join(T2)+'\n\n'+'\n'.join(T3)+'\n\n'+json.dumps(val)+'\n')
json.dump(dict(meta=dict(n=13,irreducible='x^13+x^4+x^3+x+1',curve='y^2+xy=x^3+x^2+b, b per seed',m=3,solver='pysat Glucose4',B={2:470,3:1600,4:9500},lmax=64,seed_rule='100000*n+1000*l+idx',tag='pcc:unverified pilot'),summary=summ,rows=rows),open('results.json','w'),indent=1)
print(open('tables.md').read())
