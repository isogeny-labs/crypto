#!/usr/bin/env python3
"""Toy 2LP cycle-basis relation-stage test (stdlib only, deterministic)."""
import sys, json, random, math, time
from array import array
M = 2147483647  # rank modulus (prime)

def next_prime(n):
    def isp(k):
        if k < 2: return False
        i = 2
        while i * i <= k:
            if k % i == 0: return False
            i += 1
        return True
    while not isp(n): n += 1
    return n

def spf_sieve(n):
    s = array('I', range(n + 1))
    for i in range(2, int(n ** .5) + 1):
        if s[i] == i:
            for j in range(i * i, n + 1, i):
                if s[j] == j: s[j] = i
    return s

def factor(v, spf):
    f = {}
    while v > 1:
        q = spf[v]; f[q] = f.get(q, 0) + 1; v //= q
    return f

def gen_stream(p, B, L, spf, rng, tcap):
    """Yield (kind, fbvec dict idx->e, large list) per trial; kind in full/lp1/lp2/none."""
    fb = [q for q in range(2, B + 1) if spf[q] == q]
    idx = {q: i for i, q in enumerate(fb)}
    g = 2
    for _ in range(tcap):
        a = rng.randrange(1, p - 1)
        v = pow(g, a, p)
        f = factor(v, spf)
        vec, big = {}, []
        for q, e in f.items():
            if q in idx: vec[idx[q]] = e
            else: big.append((q, e))
        if not big: yield ('full', vec, [])
        elif all(q <= L for q, e in big) and sum(e for q, e in big) <= 2: yield ('lp%d' % sum(e for q, e in big), vec, big)
        else: yield ('none', None, None)

def run_arm(rels, nfb, maxlp, relabel=None):
    """Incremental sparse elimination, large-prime columns pivot first. Return trial index reaching rank nfb."""
    piv = {}; usable = 0
    for t, (kind, vec, big) in enumerate(rels):
        if kind == 'none': continue
        nl = len(big)
        if nl > maxlp or (kind == 'lp2' and maxlp < 2): continue
        if kind == 'lp2' and sum(e for q, e in big) > maxlp: continue
        if kind == 'lp1' and maxlp < 1: continue
        row = {c: e % M for c, e in vec.items()}
        for q, e in (relabel[t] if relabel is not None and t in relabel else big):
            row[nfb + q] = (row.get(nfb + q, 0) + e) % M
        # a unit must also carry an exponent-log consistency; rank only
        while row:
            c = max(row)
            if c in piv:
                pr = piv[c]; k = row[c] * pow(pr[c], M - 2, M) % M
                for cc, vv in pr.items():
                    nv = (row.get(cc, 0) - k * vv) % M
                    if nv: row[cc] = nv
                    else: row.pop(cc, None)
            else:
                piv[c] = row
                if c < nfb:
                    usable += 1
                break
        if usable >= nfb: return t + 1
    return None

def one_seed(p, B, L, spf, seed, tcap):
    rng = random.Random(seed)
    nfb = sum(1 for q in range(2, B + 1) if spf[q] == q)
    rels = list(gen_stream(p, B, L, spf, rng, tcap))
    t0 = run_arm(rels, nfb, 0)
    t1 = run_arm(rels, nfb, 1)
    t2 = run_arm(rels, nfb, 2)
    # configuration-model null: permute large-prime slots among partial relations in prefix
    prng = random.Random(seed ^ 0x5EED)
    pre = t2 if t2 else tcap
    ids = [i for i in range(min(pre, len(rels))) if rels[i][0] in ('lp1', 'lp2')]
    slots = [(i, j) for i in ids for j in range(len(rels[i][2]))]
    labels = [rels[i][2][j] for i, j in slots]
    prng.shuffle(labels)
    relabel = {}
    for (i, j), lab in zip(slots, labels):
        relabel.setdefault(i, list(rels[i][2]))[j] = lab
    tn = run_arm(rels, nfb, 2, relabel)
    return t0, t1, t2, tn

def med(x):
    x = sorted(x); n = len(x)
    return x[n // 2] if n % 2 else (x[n // 2 - 1] + x[n // 2]) / 2

def boot(r, rng, n=400):
    m = []
    for _ in range(n):
        s = [r[rng.randrange(len(r))] for _ in r]
        m.append(med(s))
    m.sort(); return [m[int(.025 * n)], m[int(.975 * n)]]

def main():
    smoke = '--smoke' in sys.argv
    bits_list = [14] if smoke else [18, 20]
    nseeds = 4 if smoke else 20
    t_start = time.time(); out = {'protocol': 'LEAD-ICE-relation-graph-rank-large-prime', 'smoke': smoke, 'sizes': []}
    verdicts = []
    for bits in bits_list:
        p = next_prime(2 ** bits + 1)
        B = int(math.ceil(p ** 0.25 * 1.5)); L = int(B ** 2.5)
        L = min(L, p - 1)
        spf = spf_sieve(p)
        tcap = 40000 if smoke else 120000
        rows = []
        for s in range(nseeds):
            rows.append(one_seed(p, B, L, spf, 20261110 + 1000 * bits + s, tcap))
        ok = [r for r in rows if None not in r]
        if len(ok) < max(2, nseeds // 2):
            out['sizes'].append({'bits': bits, 'p': p, 'B': B, 'L': L, 'note': 'too few completed seeds', 'completed': len(ok)})
            verdicts.append('inconclusive'); continue
        brng = random.Random(7)
        r1 = [r[1] / r[0] for r in ok]; r2 = [r[2] / r[0] for r in ok]
        rn = [r[2] / r[3] for r in ok]  # real/null, should be ~1
        # A/A: two independent seed sets of arm 0 (split halves, unpaired) median ratio
        t0s = [r[0] for r in ok]; h = len(t0s) // 2
        aa = med(t0s[:h]) / med(t0s[h:]) if h else float('nan')
        c1, c2, cn = boot(r1, brng), boot(r2, brng), boot(rn, brng)
        pos = c1[1] < 1.0                 # positive control: 1LP beats full-smooth only
        gain = c2[1] <= 0.8               # 2LP delta
        beyond = not (cn[0] <= 1.0 <= cn[1] or abs(med(rn) - 1) <= 0.05)  # structure beyond null
        if not pos: v = 'inconclusive'
        elif gain and not beyond: v = 'consistent_with_prediction'
        elif c2[0] >= 0.95: v = 'falsified'
        elif gain and beyond: v = 'falsified'
        else: v = 'inconclusive'
        verdicts.append(v)
        out['sizes'].append({'bits': bits, 'p': p, 'B': B, 'L': L, 'nfb': sum(1 for q in range(2, B + 1) if spf[q] == q),
            'seeds': nseeds, 'completed': len(ok), 'median_T0': med(t0s),
            'ratio_1lp_over_full': [med(r1), c1], 'ratio_2lp_over_full': [med(r2), c2],
            'ratio_real_over_confignull': [med(rn), cn], 'aa_ratio': aa,
            'positive_control_ok': pos, 'gain_le_0.8': gain, 'beyond_null': beyond, 'size_verdict': v})
    if all(v == 'consistent_with_prediction' for v in verdicts): final = 'consistent_with_prediction'
    elif any(v == 'falsified' for v in verdicts): final = 'falsified'
    else: final = 'inconclusive'
    out.update({'seed_base': 20261110, 'rank_modulus': M, 'wall_s': round(time.time() - t_start, 2), 'verdict': final})
    print(json.dumps(out))

if __name__ == '__main__':
    main()
