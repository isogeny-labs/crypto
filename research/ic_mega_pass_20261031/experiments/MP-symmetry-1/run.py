#!/usr/bin/env python3
import sys, random, itertools, time, json
import numpy as np
sys.path.insert(0, "/Users/adamburan/ic-work/crypto/research/ic_mega_pass_20261031/ideas")
import symmetry_proto_z3_block as P
P.MODS[17] = (1 << 17) | 0b1001
B6 = 1

def vmul(a, b, n, mod):
    a = a.astype(np.int64).copy(); b = b.astype(np.int64).copy(); r = np.zeros_like(a)
    for _ in range(n):
        r ^= np.where(b & 1, a, 0); b >>= 1; a <<= 1
        a = np.where(a >> n, a ^ mod, a)
    return r

def unsat_check(Vs, xr, n, mod):
    x1, x2, x3 = np.meshgrid(np.array(Vs[0]), np.array(Vs[1]), np.array(Vs[2]), indexing='ij')
    x1, x2, x3 = x1.ravel(), x2.ravel(), x3.ravel(); x4 = np.full_like(x1, xr)
    def co(p, q):
        a = vmul(p ^ q, p ^ q, n, mod); b = vmul(p, q, n, mod); c = vmul(b, b, n, mod) ^ B6
        return a, b, c
    a1, b1, c1 = co(x1, x2); a2, b2, c2 = co(x3, x4)
    t1 = vmul(a1, c2, n, mod) ^ vmul(a2, c1, n, mod)
    t2 = vmul(a1, b2, n, mod) ^ vmul(a2, b1, n, mod)
    t3 = vmul(b1, c2, n, mod) ^ vmul(b2, c1, n, mod)
    s = vmul(t1, t1, n, mod) ^ vmul(t2, t3, n, mod)
    return not (s == 0).any()

def rand_basis(rng, n, l):
    while True:
        basis = [rng.randrange(1, 1 << n) for _ in range(l)]
        span = {0}
        for b in basis: span |= {s ^ b for s in span}
        if len(span) == 1 << l: return basis, sorted(span)

class Elim:
    """incremental F_2 echelon on python-int rows; counts XORs"""
    def __init__(self, ncols):
        self.piv = {}; self.xors = 0; self.words = (ncols + 63) // 64
    def add(self, v):
        while v:
            h = v.bit_length() - 1
            p = self.piv.get(h)
            if p is None:
                self.piv[h] = v; return True
            v ^= p; self.xors += 1
        return False
    def contains_one(self, one_bit):
        # constant monomial gets LOWEST bit index 0 -> reduce e_0
        v = 1 << one_bit
        while v:
            h = v.bit_length() - 1; p = self.piv.get(h)
            if p is None: return False
            v ^= p
        return True

def build(n, l, bases, xr):
    mod = P.MODS[n]; N = 3 * l
    def Xelem(i, basis):
        E = [frozenset() for _ in range(n)]
        for j in range(l):
            var = frozenset([frozenset([i * l + j])])
            for bit in range(n):
                if (basis[j] >> bit) & 1: E[bit] = P.padd(E[bit], var)
        return E
    X = [Xelem(i, bases[i]) for i in range(3)]
    F = P.S4_elem(X[0], X[1], X[2], P.elem_const(xr, n), B6, n, mod)
    return [f for f in F if f]

def mono_key(m): return (len(m), tuple(sorted(m)))

def filtered_basis(gens, N):
    # echelon with degree-graded order, leading = highest monomial; produce basis adapted to degree filtration
    allm = sorted(set().union(*gens), key=mono_key)
    idx = {m: i for i, m in enumerate(allm)}
    def toint(p): return sum(1 << idx[m] for m in p)
    E = Elim(len(allm)); out = []
    for v in sorted((toint(f) for f in gens), key=lambda v: v.bit_length()):
        # full reduce, then insert
        w = v
        while w:
            h = w.bit_length() - 1; p = E.piv.get(h)
            if p is None: break
            w ^= p
        if w: E.piv[w.bit_length() - 1] = w
    # interreduce lower to get minimal degrees: reduce each pivot by others below its lead
    basis = []
    for h in sorted(E.piv):
        w = E.piv[h]
        basis.append(frozenset(allm[i] for i in range(w.bit_length()) if (w >> i) & 1))
    return basis

def run_instance(n, l, bases, xr, gmap, cmin, cmax, do_full=True, do_triv=True, full_cmax=None, check_stab=True):
    t0 = time.time(); N = 3 * l
    gens = build(n, l, bases, xr)
    basis = filtered_basis(gens, N)
    degs = [max(len(m) for m in f) for f in basis]
    gmono = lambda m: frozenset(gmap(v) for v in m)
    gpoly = lambda p: frozenset(gmono(m) for m in p)
    # span stability
    allm = sorted(set().union(*basis, *[gpoly(f) for f in basis]), key=mono_key)
    ix = {m: i for i, m in enumerate(allm)}
    E1 = Elim(len(allm)); r1 = sum(E1.add(sum(1 << ix[m] for m in f)) for f in basis)
    r2 = r1 + sum(E1.add(sum(1 << ix[m] for m in gpoly(f))) for f in basis)
    span_stable = (r1 == r2)
    tbuild = time.time() - t0
    res = dict(n=n, l=l, xr=xr, ngens=len(gens), dim_span=len(basis), degs=sorted(degs), span_stable=span_stable,
               build_s=round(tbuild, 2), cells=[], cstar_full=None, cstar_triv=None)
    full_done = not do_full; triv_done = not do_triv
    for c in range(cmin, cmax + 1):
        if full_done and triv_done: break
        cols = [frozenset(s) for d in range(c + 1) for s in itertools.combinations(range(N), d)]
        cidx = {m: i for i, m in enumerate(cols)}
        mult = {}
        def mons(k):
            if k not in mult: mult[k] = [frozenset(s) for d in range(k + 1) for s in itertools.combinations(range(N), d)] if k >= 0 else []
            return mult[k]
        cell = dict(c=c)
        if not full_done and (full_cmax is None or c <= full_cmax):
            ts = time.time(); E = Elim(len(cols)); nrows = 0
            for f, d in zip(basis, degs):
                for m in mons(c - d):
                    p = P.pmul(f, frozenset([m]))
                    if p: E.add(sum(1 << cidx[q] for q in p)); nrows += 1
            one = E.contains_one(0)
            cell.update(full_rows=nrows, full_cols=len(cols), full_rank=len(E.piv), full_one=one,
                        full_wordops=E.xors * E.words, full_s=round(time.time() - ts, 2))
            if check_stab and one:
                # W_c G-stability: add g-images of a sample of rows
                E2 = Elim(len(cols)); E2.piv = dict(E.piv); added = 0; k = 0
                for f, d in zip(basis, degs):
                    for m in mons(c - d)[::7]:
                        p = gpoly(P.pmul(f, frozenset([m])))
                        if p and E2.add(sum(1 << cidx[q] for q in p)): added += 1
                cell['Wc_g_new_dims_sampled'] = added
            if one: res['cstar_full'] = c; full_done = True
        elif not full_done:
            full_done = True; res['full_censored_at'] = c - 1
        if not triv_done:
            ts = time.time()
            orbit_of = {}
            for m in cols:
                if m not in orbit_of:
                    o = frozenset([m, gmono(m), gmono(gmono(m))])
                    for x in o: orbit_of[x] = o
            orbits = sorted(set(orbit_of.values()), key=lambda o: mono_key(min(o, key=mono_key)))
            oidx = {o: i for i, o in enumerate(orbits)}
            Et = Elim(len(orbits)); nrows = 0
            for f, d in zip(basis, degs):
                seen = set()
                for m in mons(c - d):
                    o = orbit_of[m]
                    if o in seen: continue
                    seen.add(o)
                    p = P.pmul(f, frozenset([m])); v = 0
                    for q in p: v ^= 1 << oidx[orbit_of[q]]
                    if v: Et.add(v); nrows += 1
            one = Et.contains_one(oidx[orbit_of[frozenset()]])
            cell.update(triv_rows=nrows, triv_cols=len(orbits), triv_rank=len(Et.piv), triv_one=one,
                        triv_wordops=Et.xors * Et.words, triv_s=round(time.time() - ts, 2))
            if one: res['cstar_triv'] = c; triv_done = True
        res['cells'].append(cell)
    res['total_s'] = round(time.time() - t0, 2)
    return res

def find_unsat(rng, n, Vs, k, maxtry=4000):
    mod = P.MODS[n]; out = []; tries = 0
    while len(out) < k and tries < maxtry:
        tries += 1; xr = rng.randrange(1, 1 << n)
        if xr in out or not P.lifts(xr, 0, B6, n, mod): continue
        if unsat_check(Vs, xr, n, mod): out.append(xr)
    return out, tries

if __name__ == "__main__":
    arm = sys.argv[1]; n = int(sys.argv[2]); l = int(sys.argv[3]); k = int(sys.argv[4])
    cmax = int(sys.argv[5]); full_cmax = int(sys.argv[6]) if len(sys.argv) > 6 and sys.argv[6] != 'x' else None
    do_full = not (len(sys.argv) > 7 and sys.argv[7] == 'trivonly')
    rng = random.Random(1000 * n + l + (500 if arm == 'null' else 0))
    gmap = lambda v: ((v // l + 1) % 3) * l + (v % l)
    out = []
    for inst in range(k):
        if arm == 'null':
            bases = [rand_basis(rng, n, l) for _ in range(3)]
        else:
            b = rand_basis(rng, n, l); bases = [b, b, b]
        Vs = [bb[1] for bb in bases]
        xs, tries = find_unsat(rng, n, Vs, 1)
        if not xs: out.append(dict(inst=inst, error='no unsat target', tries=tries)); continue
        r = run_instance(n, l, [bb[0] for bb in bases], xs[0], gmap, 4, cmax, do_full=do_full, full_cmax=full_cmax)
        r.update(arm=arm, inst=inst, V_basis=[bb[0] for bb in bases], unsat_tries=tries)
        print(json.dumps({kk: r[kk] for kk in ('arm','n','l','inst','xr','dim_span','span_stable','cstar_full','cstar_triv','total_s')}), flush=True)
        out.append(r)
    json.dump(out, open(f"raw_{arm}_n{n}_l{l}.json", "w"), indent=1)
