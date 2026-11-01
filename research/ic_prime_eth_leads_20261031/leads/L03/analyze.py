import json, sys, glob, math, collections
import numpy as np
G = collections.defaultdict(list); chk = collections.Counter()
for f in sorted(glob.glob('raw*.jsonl')):
    for l in open(f):
        d = json.loads(l); G[(d['bits'], d['m'], d['fam'])].append(d)
        chk['n'] += 1; chk['cons_ok'] += d['cons']; chk['planted_ok'] += d['planted']
print('checks: instances', chk['n'], 'conservation ok', chk['cons_ok'], 'planted ok', chk['planted_ok'])
def rat(num, den):
    ok = ~(np.isnan(num)|np.isnan(den)); num, den = num[ok], den[ok]
    if len(num) < 3 or den.mean() == 0: return (float('nan'), float('nan'))
    # ratio of means with paired delta-method se
    n = len(num); mn, md = num.mean(), den.mean()
    r = mn/md; se = math.sqrt(max(0, ((num - r*den)**2).sum()/(n*(n-1))))/md
    return r, se
res = {}
fams = ['bochum','dense','sp3','sp5','sp7','reg3','reg5','sp3c','sp5c','reg3c','window','random']
raw = {}
print('\nTABLE  bits m fam n | P1 arm | P1 null | ratio P1 +-se | E arm | Eratio +-se | E[X^2]/E[X]^2 arm | F2 ratio +-se | P2 ratio')
for m in (2,3):
  for b in (14,16,18,20,22):
    for fam in fams:
        rows = G.get((b, m, fam)); 
        if not rows: continue
        A = lambda k, s='a': np.array([x[s][k] if x[s][k] is not None else np.nan for x in rows], float)
        p1, p1n = A('P1'), A('P1','ref'); E, En = A('E'), A('E','ref')
        F2, F2n = A('F2'), A('F2','ref'); P2, P2n = A('P2'), A('P2','ref')
        r1 = rat(p1, p1n); re_ = rat(E, En); rf = rat(F2, F2n); r2 = rat(P2, P2n)
        res[(b,m,fam)] = (r1, re_, rf)
        empt = np.mean([min(x['sizes'])==0 for x in rows]); raw[(b,m,fam)] = (p1.mean(), p1.std(ddof=1)/math.sqrt(len(p1)), E.mean())
        print(f'{b} {m} {fam:7s} n={len(rows):4d} | {p1.mean():.4f} | {p1n.mean():.4f} | {r1[0]:.4f}+-{r1[1]:.4f} | {E.mean():.3f} | {re_[0]:.4f}+-{re_[1]:.4f} | {F2.mean():.3f} | {rf[0]:.4f}+-{rf[1]:.4f} | {r2[0]:.4f}+-{r2[1]:.4f} | empty {empt:.3f}')
print('\nSPARSE/DENSE and SPARSE/BOCHUM Pr[X>0] ratio (ratio of arm/null ratios; 5% rule on sparse/dense)')
for m in (2,3):
  for b in (14,16,18,20,22):
    for fam in ('sp3','sp5','sp7','reg3','reg5','bochum'):
        if (b,m,fam) in res and (b,m,'dense') in res:
            a, d = res[(b,m,fam)][0], res[(b,m,'dense')][0]
            q = a[0]/d[0]; se = q*math.hypot(a[1]/a[0], d[1]/d[0])
            print(f'{b} {m} {fam:7s} vs dense: {q:.4f}+-{se:.4f}  ({(q-1)*100:+.2f}%)  {"WITHIN5" if q>0.95 else "DEFICIT"}')
print('\nRAW Pr[X>0] of sparse arm / dense arm (unpaired; includes empty-base failures and realised-size differences) and E ratio')
for m in (2,3):
  for b in (14,16,18,20,22):
    for fam in ('sp3','sp5','sp7','reg3','reg5','sp3c','sp5c','reg3c','bochum'):
        if (b,m,fam) in raw and (b,m,'dense') in raw:
            a, d = raw[(b,m,fam)], raw[(b,m,'dense')]; q = a[0]/d[0]; se = q*math.hypot(a[1]/a[0], d[1]/d[0])
            print(f'{b} {m} {fam:6s} rawP1/dense {q:.4f}+-{se:.4f} ({(q-1)*100:+.1f}%)  E {a[2]:.3f} vs dense {d[2]:.3f}; P1/E ratio {(a[0]/a[2])/(d[0]/d[2]):.4f}')
print('\nSLOPES of ln(ratio P1 arm/null) vs b (weighted LS), per bit')
for m in (2,3):
  for fam in fams:
    pts = [(b, *res[(b,m,fam)][0]) for b in (14,16,18,20,22) if (b,m,fam) in res]
    if len(pts) < 3: continue
    x = np.array([q[0] for q in pts], float); y = np.log([q[1] for q in pts]); s = np.array([q[2]/q[1] for q in pts])
    w = 1/s**2; xm = (w*x).sum()/w.sum(); sl = (w*(x-xm)*(y-(w*y).sum()/w.sum())).sum()/(w*(x-xm)**2).sum(); se = 1/math.sqrt((w*(x-xm)**2).sum())
    print(f'm={m} {fam:7s} slope {sl:+.5f} +- {se:.5f} per bit  (z={sl/se:+.2f}); ratios '+' '.join(f'{q[1]:.3f}' for q in pts))
print('\nTREND vs b of RAW sparse/dense Pr[X>0] (and E-normalised) and empty-base fraction (weighted LS slope per bit of ln ratio)')
def wls(x, y, s):
    x = np.array(x, float); y = np.array(y, float); w = 1/np.array(s)**2
    xm = (w*x).sum()/w.sum(); ym = (w*y).sum()/w.sum(); sxx = (w*(x-xm)**2).sum()
    return (w*(x-xm)*(y-ym)).sum()/sxx, 1/math.sqrt(sxx)
for m in (2,3):
  for fam in ('sp3','sp5','sp7','reg3','reg5','bochum'):
    xs, ys, ss, es, line = [], [], [], [], []
    for b in (14,16,18,20,22):
        if (b,m,fam) in raw and (b,m,'dense') in raw:
            a, d = raw[(b,m,fam)], raw[(b,m,'dense')]; q = a[0]/d[0]; se = q*math.hypot(a[1]/a[0], d[1]/d[0])
            xs.append(b); ys.append(math.log(q)); ss.append(se/q); line.append(f'{b}:{q:.3f}')
    if len(xs) >= 3:
        sl, se = wls(xs, ys, ss); print(f'm={m} {fam:6s} slope of ln(rawP1/dense) {sl:+.5f} +- {se:.5f}/bit (z={sl/se:+.1f});  '+' '.join(line))
print('\nEMPTY-BASE FRACTION (any slot empty) by b')
for m in (2,3):
  for fam in ('dense','bochum','sp3','sp5','sp7','reg3','reg5'):
    print(f'm={m} {fam:6s}', ' '.join(f"{b}:{np.mean([min(x['sizes'])==0 for x in G[(b,m,fam)]]):.3f}" for b in (14,16,18,20,22) if (b,m,fam) in G))
