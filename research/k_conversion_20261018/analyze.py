#!/usr/bin/env python3
import json, math, statistics as st
from collections import defaultdict
R=[json.loads(l) for l in open('raw.jsonl')]
rho=defaultdict(list)
for r in R:
    if r['kind']=='rho' and r['rep']==0: rho[r['cell']].append(r['ir']/r['walk_steps'])
Istep={k:dict(vals=v,median=st.median(v),min=min(v),max=max(v)) for k,v in rho.items()}
cells=defaultdict(list)
for r in R:
    if r['kind']=='closure' and r['toggle'] and r['rep']==0:
        cells[r['cell']].append(dict(draw=r['draw'],columns=r['columns'],ir=r['ir'],word_ops=r['word_ops'],
          Icol=r['ir']/r['columns'],Iword=r['ir']/r['word_ops']))
tot={r['cell']:r for r in R if r['kind']=='closure' and not r['toggle']}
share={c:next(x['ir'] for x in R if x['kind']=='closure' and x['toggle'] and x['rep']==0 and x['cell']==c and x['draw']==tot[c]['draw'])/tot[c]['ir'] for c in tot}
# A/A
aa_c=[r['ir'] for r in R if r['kind']=='closure' and r['cell']=='m3-16-6' and r['draw']==0 and r['toggle']]
aa_r=[r['ir'] for r in R if r['kind']=='rho' and r['cell']=='n61' and r['seed']==1]
# positive control: rank corr ir vs word_ops over degree-3 draws
from scipy.stats import spearmanr
d3=[x for c,v in cells.items() if c!='m3-11-5-deg4' for x in v]
rho_s=spearmanr([x['ir'] for x in d3],[x['word_ops'] for x in d3])
out=dict(Istep=Istep,cells={},share=share,
 AA=dict(closure=aa_c,closure_rel=abs(aa_c[0]-aa_c[1])/aa_c[0],rho=aa_r),
 spearman=dict(rho=rho_s.statistic,n=len(d3)))
for c,v in cells.items():
    for den in ('n83','n61'):
        ks=[x['Icol']/Istep[den]['median'] for x in v]
        out['cells'].setdefault(c,{})[den]=dict(K=ks,median=st.median(ks),min=min(ks),max=max(ks))
    out['cells'][c]['Icol']=[x['Icol'] for x in v]
    out['cells'][c]['Iword']=[x['Iword'] for x in v]
    out['cells'][c]['columns']=[x['columns'] for x in v]
    out['cells'][c]['draws']=[x['draw'] for x in v]
m4=[x for c in ('m4-11-5','m4-12-3','m4-12-5') for x in cells[c]]
pool={}
for den in ('n83','n61'):
    ks=[x['Icol']/Istep[den]['median'] for x in m4]
    pool[den]=dict(median=st.median(ks),min=min(ks),max=max(ks))
out['pooled_m4_deg3']=pool
conv=[]
for name,row,gap in (('degree 4, w=1, ell=33',68.43,7.62),('degree 5, w=1, ell=34',71.13,10.32)):
    e=dict(name=name,row=row,gap=gap)
    for lab,K in (('primary(median,n83)',pool['n83']['median']),('low(min,n83)',pool['n83']['min']),('high(max,n83)',pool['n83']['max']),
                  ('n61 median',pool['n61']['median']),('deg4 growth cell n83',out['cells']['m3-11-5-deg4']['n83']['median'])):
        e[lab]=dict(K=K,log2K=math.log2(K),row=row+math.log2(K),gap=gap+math.log2(K))
    conv.append(e)
out['converted']=conv
json.dump(out,open('RESULT.json','w'),indent=1)

