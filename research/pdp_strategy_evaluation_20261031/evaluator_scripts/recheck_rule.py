import json, math, collections, statistics as st
def load(fs):
    out=[]
    for f in fs:
        for l in open(f):
            out.append(json.loads(l))
    return out
m2=load(['m2.jsonl','geomtrace_m2.jsonl'])
print('m2 rows',len(m2),'distinct fb sha',len({r['factor_base_sha256'] for r in m2}))
def rule(e): return 2+(e<-3.5)+(e<-8.5)
errs=[];tab=collections.defaultdict(list)
for r in m2:
    D=r['degrees']['mxl']['D_solve_mean']; e=r['predictions']['linearization_excess']
    errs.append(abs(D-rule(e))); tab[rule(e)].append(D)
print('rows with direct mxl D, MAE of D=2+[e<-3.5]+[e<-8.5]: %.3f  (n=%d)'%(st.mean(errs),len(errs)))
for k in sorted(tab): print('  predicted',k,'n',len(tab[k]),'measured mean D %.3f'%st.mean(tab[k]),'max',max(tab[k]))
print('fields n:',sorted({r['cell']['n'] for r in m2}),'l range',min(r['cell']['l'] for r in m2),max(r['cell']['l'] for r in m2))
print('measured D_solve mxl overall max',max(r['degrees']['mxl']['D_solve_mean'] for r in m2))
# families
fam=collections.Counter(r['cell']['family'] for r in m2); print(fam)
# m=3
m3=load(['m3.jsonl','geomtrace_m3.jsonl','m3_l5.jsonl'])
print('m3 rows',len(m3),'distinct',len({r['factor_base_sha256'] for r in m3}))
D3=[r['degrees']['mxl']['D_solve_mean'] for r in m3]; print('m3 D mxl min/max',min(D3),max(D3))
# paired structured(prefix,geometric) vs random per (n,l)
cells=collections.defaultdict(lambda: collections.defaultdict(list))
for r in m3:
    c=r['cell']; cells[(c['n'],c['l'])][c['family']].append((r['degrees']['mxl']['D_solve_mean'], r['cost']['mxl']['ops_per_attempt_mean']))
rows=[]
for k,v in sorted(cells.items()):
    if 'random' in v and ('prefix' in v or 'geometric' in v):
        s=[x for f in ('prefix','geometric') for x in v.get(f,[])]
        rs=v['random']
        rows.append((k,st.mean(x[0] for x in rs)-st.mean(x[0] for x in s), st.mean(x[1] for x in rs)/st.mean(x[1] for x in s)))
print('m3 paired cells (random - structured): n=',len(rows))
for k,dD,ratio in rows: print('  n,l=%s  dD=%+.2f  ops/attempt ratio=%.2f'%(k,dD,ratio))
print('mean dD %.3f ; geo-mean ops ratio %.3f'%(st.mean(r[1] for r in rows), math.exp(st.mean(math.log(r[2]) for r in rows))))
