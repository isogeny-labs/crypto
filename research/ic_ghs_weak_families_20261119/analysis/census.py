#!/usr/bin/env python3
"""P5/P6: GHS magic-number census over F_{2^N}, N = n*l.  Closed form (module count) vs brute force,
then the C-fin weak set.  Desk computation, no solver.  usage: census.py <lane dir>"""
import json, math, sys, os, itertools
LANE = sys.argv[1] if len(sys.argv) > 1 else os.path.dirname(os.path.dirname(os.path.abspath(__file__)))

# ---- factor x^n - 1 over F_2: (x^n'-1)^(2^s), irreducible factors of x^n'-1 = cyclotomic cosets of 2 mod n'
def factors(n):
    s = 0
    while n % 2 == 0: n //= 2; s += 1
    seen = [False]*n; ds = []
    for r in range(n):
        if seen[r]: continue
        c, k = r, 0
        while not seen[c]: seen[c] = True; c = (c*2) % n; k += 1
        ds.append(k)               # degree of the irreducible factor; coset of 0 is (x+1), degree 1
    return ds, 2**s                # list of degrees d_i (ds[0] == 1 is x+1), exponent e = 2^s

def classes(n, l):
    """all (j_i) with count, deg Ord, type, m, genus.  K = R^l as F_2[x]-module."""
    ds, e = factors(n); out = []
    for js in itertools.product(range(e+1), repeat=len(ds)):
        cnt = 1
        for d, j in zip(ds, js):
            cnt *= 1 if j == 0 else 2**(d*j*l) - 2**(d*(j-1)*l)
        deg = sum(d*j for d, j in zip(ds, js))
        if deg == 0: continue                       # b = 0
        typeI = (js[0] == 0)                        # (x+1) does not divide Ord
        m = deg + (1 if typeI else 0)
        g = 2**(m-1) - 1 if typeI else 2**(m-1)
        out.append({'js': js, 'count': cnt, 'deg': deg, 'm': m, 'type': 'I' if typeI else 'II', 'genus': g})
    return out

def log2_cost_ic(g, l, kappa):                       # log2( kappa * g! * q^(2-2/g) ), q = 2^l
    return math.log2(kappa) + math.lgamma(g+1)/math.log(2) + l*(2-2/g)
def log2_rho(N): return math.log2(0.63) + N/2

# ---- brute force in F_{2^N}
def prim_poly(N):
    for p in range(2**N + 1, 2**(N+1), 2):
        x, k = 2, 1
        # multiply by x repeatedly
        a = 1
        ok = True
        seen = 0
        a = 2
        k = 1
        while a != 1:
            a <<= 1
            if a >> N & 1: a ^= p
            k += 1
            if k > 2**N: ok = False; break
        if ok and k == 2**N - 1: return p
    raise RuntimeError
def tables(N):
    p = prim_poly(N); exp = [0]*(2**N - 1); log = {}
    a = 1
    for i in range(2**N - 1):
        exp[i] = a; log[a] = i; a <<= 1
        if a >> N & 1: a ^= p
    return exp, log
def brute(N, l):
    n = N//l; exp, log = tables(N); M = 2**N - 1
    counts = {}
    for b in range(1, 2**N):
        lb = log[b]
        lv = lb//2 if lb % 2 == 0 else (lb + M)//2          # sqrt(b)
        # orbit sigma^i v, sigma = Frob^l
        vecs = []; lv_i = lv
        for i in range(n):
            vecs.append(exp[lv_i]); lv_i = (lv_i * 2**l) % M
        # minimal d with v_d in span(v_0..v_{d-1}); track combination as bitmask over indices
        basis = []                                        # (vec, comb)
        d = None; comb_final = 0
        for i, v in enumerate(vecs + [exp[lv_i]]):        # include sigma^n v = v (periodic) as sentinel
            c = 1 << i; w = v
            for bv, bc in basis:
                if w ^ bv < w: w ^= bv; c ^= bc
            if w == 0:
                d = i; comb_final = c; break
            basis.append((w, c)); basis.sort(reverse=True)
        deg = d; ordpoly = comb_final                      # Ord(x) = sum_{i in comb} x^i, degree d
        typeI = (bin(ordpoly).count('1') % 2 == 1)         # Ord(1) = 1  <=> (x+1) does not divide Ord
        m = deg + (1 if typeI else 0)
        key = (deg, 'I' if typeI else 'II', m)
        counts[key] = counts.get(key, 0) + 1
    return counts

res = {'verify': [], 'census': {}}
tot_dev = 0
for N in (6, 9, 10, 12, 15):
    for l in [d for d in range(1, N//2 + 1) if N % d == 0]:
        n = N//l
        cf = {}
        for c in classes(n, l):
            k = (c['deg'], c['type'], c['m']); cf[k] = cf.get(k, 0) + c['count']
        bf = brute(N, l)
        dev = sum(abs(cf.get(k, 0) - bf.get(k, 0)) for k in set(cf) | set(bf))
        tot_dev += dev
        res['verify'].append({'N': N, 'l': l, 'n': n, 'closed_form_total': sum(cf.values()), 'brute_total': sum(bf.values()),
                              'deviation': dev, 'classes': {f'deg{k[0]}_{k[1]}_m{k[2]}': [cf.get(k, 0), bf.get(k, 0)] for k in sorted(set(cf) | set(bf))}})
        print(f'N={N:2d} l={l:2d} n={n:2d} classes={len(cf)} closed-form vs brute-force deviation={dev}', flush=True)
res['total_deviation'] = tot_dev

# n = 3 closed forms of the committed census
for l in range(2, 8):
    cl = {(c['deg'], c['type']): c['count'] for c in classes(3, l)}
    assert cl[(1, 'II')] == 2**l - 1 and cl[(2, 'I')] == 2**(2*l) - 1 and cl[(3, 'II')] == (2**l - 1)*(2**(2*l) - 1)
res['n3_closed_forms_match_committed_census'] = True

# ---- C-fin census: per N, per tower, weak b counts; subfield bound
def weak_table(kappa):
    rows = []
    for N in range(4, 97):
        per_tower = []
        sub_bound = sum(2**s for s in range(1, N//2 + 1) if N % s == 0)       # |union of proper subfields| <= this
        for l in [d for d in range(1, N//2 + 1) if N % d == 0]:
            n = N//l
            wk = 0; wk_cls = []
            for c in classes(n, l):
                if c['genus'] < 2: continue                                  # genus 1: the curve itself
                if log2_cost_ic(c['genus'], l, kappa) < log2_rho(N):
                    wk += c['count']; wk_cls.append((n, c['type'], c['m'], c['genus'], c['count']))
            wk_lb = max(0, wk - sub_bound) if wk else 0
            per_tower.append({'l': l, 'n': n, 'weak_b': wk, 'weak_b_minus_subfield_bound': wk_lb,
                              'classes': [list(x) for x in wk_cls]})
        best = max(per_tower, key=lambda t: t['weak_b_minus_subfield_bound'])
        rows.append({'N': N, 'towers': per_tower, 'max_over_towers': best['weak_b_minus_subfield_bound'],
                     'sum_over_towers_upper': sum(t['weak_b'] for t in per_tower),
                     'frac_max': best['weak_b_minus_subfield_bound']/(2**N - 1)})
    return rows
res['census'] = {f'kappa{k}': weak_table(k) for k in (1, 10, 100)}
json.dump(res, open(f'{LANE}/census.json', 'w'))
# compact summary json (small)
summ = {}
for k, rows in res['census'].items():
    summ[k] = [{'N': r['N'], 'max_over_towers': r['max_over_towers'], 'sum_over_towers_upper': r['sum_over_towers_upper'],
                'frac_max': r['frac_max'], 'towers': [{'l': t['l'], 'n': t['n'], 'classes': t['classes']} for t in r['towers'] if t['weak_b']]}
               for r in rows if r['sum_over_towers_upper']]
json.dump({'total_deviation': tot_dev, 'verify': [{k: v for k, v in r.items() if k != 'classes'} for r in res['verify']], 'weak': summ},
          open(f'{LANE}/census_summary.json', 'w'), indent=1)
for k, v in summ.items():
    print(k, 'first N with any weak b:', v[0]['N'] if v else None)
