import json,glob,math,random,statistics as st
random.seed(1)
modes=['school','kara','pc','kpc']
cells=[(6,2),(9,3),(12,4),(15,5)]
def load(b,md):
    try: return [json.loads(l) for l in open(f'res/b{b:02d}-{md}/runs.jsonl')]
    except: return []
def med(x): return st.median(x) if x else float('nan')
def slope(pts):
    n=len(pts); mx=sum(p[0] for p in pts)/n; my=sum(p[1] for p in pts)/n
    d=sum((p[0]-mx)**2 for p in pts); return sum((p[0]-mx)*(p[1]-my) for p in pts)/d if d else float('nan')
out=[];S={}
for kind in ['uniform','planted','all']:
  out.append(f'\n### {kind} targets\n| b (s) | mode | n | solved | UNKNOWN(censored@cap) | median conf | median dec | median prop(M) | median nvars | median nclauses | median wall s | T_enum med |\n|---|---|---|---|---|---|---|---|---|---|---|---|')
  for b,s in cells:
    for md in modes:
      R=[r for r in load(b,md) if kind=='all' or r['target_kind']==kind]
      if not R: continue
      sol=[r for r in R if r['status']!='UNKNOWN']
      out.append(f"| {b} ({s}) | {md} | {len(R)} | {len(sol)} | {len(R)-len(sol)} | {med([r['conflicts'] for r in R]):.0f} | {med([r['decisions'] for r in R]):.0f} | {med([r['propagations'] for r in R])/1e6:.1f} | {med([r['nvars'] for r in R]):.0f} | {med([r['nclauses'] for r in R]):.0f} | {med([r['wall_s'] for r in R]):.2f} | {med([r['t_enum'] for r in R]):.0f} |")
# slopes: per-instance log2 conflicts vs s (censored values included at cap = lower bound), bootstrap over instances (resample within cell)
out.append('\n### Slope e of log2(conflicts) vs s (bootstrap 2000, resample instances within cell; censored runs enter at conflicts reached = lower bound)\n| kind | mode | cells used | e | 95% CI | e_enum (log2 T_enum vs s) |\n|---|---|---|---|---|---|')
res={}
for kind in ['uniform','planted','all']:
  for md in modes+['enum']:
    data={}
    for b,s in cells:
      if md=='enum': R=load(b,'school')
      else: R=load(b,md)
      R=[r for r in R if kind=='all' or r['target_kind']==kind]
      if md=='enum': data[s]=[math.log2(r['t_enum']) for r in R]
      else: data[s]=[math.log2(max(r['conflicts'],1)) for r in R]
    data={k:v for k,v in data.items() if v}
    if len(data)<2: continue
    def est(d): return slope([(s,st.median(v)) for s,v in d.items()])
    e=est(data); bs=[]
    for _ in range(2000):
      bs.append(est({s:[random.choice(v) for _ in v] for s,v in data.items()}))
    bs=sorted(x for x in bs if x==x); lo,hi=bs[int(.025*len(bs))],bs[int(.975*len(bs))]
    res[(kind,md)]=(e,lo,hi)
    out.append(f"| {kind} | {md} | {sorted(data)} | {e:.2f} | [{lo:.2f}, {hi:.2f}] | |")
# pairwise slope difference vs school (paired bootstrap not possible across independent runs; same instances so resample instance index jointly)
out.append('\n### Slope difference vs school (joint resample of instance index, same instances across modes; uniform+planted)\n| mode | e_mode - e_school | 95% CI |\n|---|---|---|')
for md in ['kara','pc','kpc']:
    D={}
    for m in ['school',md]:
        D[m]={s:[math.log2(max(r['conflicts'],1)) for r in load(b,m)] for b,s in cells if load(b,m)}
    ss=[s for s in D['school'] if s in D[md] and len(D[md][s])==len(D['school'][s])]
    if len(ss)<2: continue
    def est(idx):
        return slope([(s,st.median([D[md][s][i] for i in idx[s]])) for s in ss])-slope([(s,st.median([D['school'][s][i] for i in idx[s]])) for s in ss])
    full={s:list(range(len(D['school'][s]))) for s in ss}
    d0=est(full); bs=sorted(est({s:[random.randrange(len(full[s])) for _ in full[s]] for s in ss}) for _ in range(2000))
    out.append(f"| {md} | {d0:.2f} | [{bs[50]:.2f}, {bs[1949]:.2f}] |")
# ratio vs school per cell
out.append('\n### Median conflict ratio mode/school (same instances)\n| b | kara | pc | kpc |\n|---|---|---|---|')
for b,s in cells:
    base=load(b,'school')
    if not base: continue
    row=[]
    for md in ['kara','pc','kpc']:
        R=load(b,md)
        row.append(f"{med([r['conflicts']/max(q['conflicts'],1) for r,q in zip(R,base)]):.2f}" if len(R)==len(base) else 'n/a')
    out.append(f"| {b} | "+" | ".join(row)+" |")
open('analysis.md','w').write('\n'.join(out)); print('\n'.join(out))
