"""Independent re-derivation of Theorem X counts (exact integers, mpmath floors), Hermite bounds, pooled rho stats.
Reads the lab's Mestre jsonl (read-only) from argv[1]. Writes nothing."""
import sys, json, glob, math, statistics as st
from math import comb
import mpmath as mp
mp.mp.dps = 60

EXACT = {1: mp.mpf(1), 2: mp.sqrt(mp.mpf(4)/3), 3: mp.cbrt(2), 4: mp.sqrt(2), 5: mp.mpf(8)**(mp.mpf(1)/5),
         6: (mp.mpf(64)/3)**(mp.mpf(1)/6), 7: mp.mpf(64)**(mp.mpf(1)/7), 8: mp.mpf(2)}

def minkowski(n):  # gamma_n <= 4 / v_n^{2/n}
    v = mp.pi**(mp.mpf(n)/2)/mp.gamma(mp.mpf(n)/2+1)
    return 4/v**(mp.mpf(2)/n)

def blichfeldt(n):  # gamma_n <= (2/pi) Gamma(2+n/2)^{2/n}
    return (2/mp.pi)*mp.gamma(2+mp.mpf(n)/2)**(mp.mpf(2)/n)

def gam(s, mode):
    if s == 0: return mp.mpf(1)
    if s in EXACT and mode != 'loose_all': return EXACT[s]
    if mode in ('loose', 'loose_all'): return 1 + mp.mpf(s)/4
    if mode == 'blich': return blichfeldt(s)
    if mode == 'lower2': return mp.mpf(2)   # gamma_9 >= 2 (Lambda_9); NOT a valid upper bound, sensitivity only
    raise ValueError

def Mexact(k, rho, mode='loose', S=None):
    S = k-1 if S is None else min(S, k-1)
    tot = 0
    for s in range(0, S+1):
        C = 1 if s == 0 else int(mp.floor((gam(s, mode)*mp.mpf(rho))**(mp.mpf(s)/2)))
        n = s+1
        tot += comb(k, n) * (((2*C+1)**n - 1)//2)
    return tot

def lg(x): return math.log2(x) if x < 2**1000 else (x.bit_length() + math.log2(x/2**(x.bit_length()-53)) - 53)

def rho_star(k, half=128, mode='loose'):
    lo, hi = 1.0, 1.0
    if lg(Mexact(k, 1.0, mode)) > half: return None
    while lg(Mexact(k, hi, mode)) <= half: hi *= 2
    lo = hi/2
    for _ in range(80):
        mid = (lo+hi)/2
        if lg(Mexact(k, mid, mode)) <= half: lo = mid
        else: hi = mid
    return lo

print('## Hermite: check 1+n/4 >= Minkowski bound (rigorous) for n=1..3000')
bad = [n for n in range(1, 3001) if minkowski(n) > 1 + mp.mpf(n)/4]
print('violations:', bad[:10], 'count', len(bad))
for n in (7, 8, 9, 10, 11, 12):
    print(n, 'exact' if n in EXACT else '', float(EXACT.get(n, mp.nan)), 'Minkowski %.4f Blichfeldt %.4f 1+n/4 %.2f' % (minkowski(n), blichfeldt(n), 1+n/4))

D = sys.argv[1]
pool = {7: [], 8: [], 9: []}
pool_small = {7: [], 8: [], 9: []}
for f in sorted(glob.glob(D + '/mestre_*.jsonl')):
    for l in open(f):
        d = json.loads(l)
        for c in d['cases']:
            if 'rho_G' in c:
                pool[c['s']].append(c['rho_G'])
                if 'small' in f: pool_small[c['s']].append(c['rho_G'])
print('\n## pooled rho (all Mestre files) and margins = 128 - log2|M| (exact integer |M|, gamma loose for s>=9)')
for s in (7, 8, 9):
    r = sorted(pool[s]); rs = sorted(pool_small[s]); k = s+1
    n = len(r)
    p95 = r[min(n-1, int(math.ceil(0.95*n))-1)]
    med = st.median(r)
    if s == 7:
        meds = st.median(rs)
        print(f's={s} small-only n={len(rs)} median {meds:.4f} max {rs[-1]:.4f}')
    print(f's={s} k={k} n={n} median {med:.4f} p95~{p95:.4f} max {r[-1]:.4f}')
    for lab, rho in (('median', med), ('max', r[-1])):
        for mode in ('loose', 'blich'):
            M = Mexact(k, rho, mode)
            print(f'   {lab:6s} rho={rho:.4f} mode={mode:5s} log2|M|={lg(M):8.3f} margin={128-lg(M):+8.3f}')
    for mode in ('loose', 'blich', 'lower2'):
        rs_ = rho_star(k, 128, mode)
        nex = sum(1 for x in r if x > rs_)
        print(f'   rho*({s}) mode={mode}: {rs_:.4f}; configs above rho*: {nex}/{n}')
    # sensitivity: more lifted points than rank+1
    for kk in (k+1, k+2):
        print(f'   k={kk} (rank<= {s}) rho*: {rho_star_S(kk, s) if False else ""}', end='')
        lo, hi = 1.0, 64.0
        f_ = lambda rho: lg(Mexact(kk, rho, 'loose', S=s)) <= 128
        for _ in range(80):
            mid = (lo+hi)/2
            if f_(mid): lo = mid
            else: hi = mid
        print(f'{lo:.4f}')

print('\n## Corollary X1 / F1 thresholds re-derived (k=S+1)')
def kstar(qbits, rho):
    for k in range(2, 400):
        if lg(Mexact(k, rho, 'loose')) >= qbits/2: return k
for qb in (256, 2048):
    print(qb, [kstar(qb, rho)-1 for rho in (1.5, 10, 100, 1e4)],
          [next(r for r in range(1, 10**6) if math.log2(16) + r*math.log2(1+2*math.sqrt(rho)) >= qb/2) for rho in (1.5, 10, 100, 1e4)])
print('\n## Petsche-style rho at beta=256 (lab inputs): S excluded?')
logD = 13*256*math.log(2); sig = logD/math.log(11)
lam = logD/(1e15*sig**6*math.log(104613*sig**2)**2); H = 2*256*math.log(2)
rp = H/lam
print('rho0 %.3e' % rp, [(S, round(128-lg(Mexact(S+1, rp, 'loose')), 1)) for S in (1, 2, 3)])
print('\n## tail requirement: P[rho>rho*] must be <~ 2^-128 for the verdict; rule-of-three 95% UB from n samples, 0 exceedances:')
for s in (7, 8, 9):
    print(s, 'n=%d UB=%.3g' % (len(pool[s]), 3/len(pool[s])))
