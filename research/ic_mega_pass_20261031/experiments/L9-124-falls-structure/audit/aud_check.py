"""Independent auditor check of L9 claims. Own GF(2^n), own loader, own rank code. Uses only the generator
(N1 lib.system via L9 sys9, which is the object under study) from the L9 tree."""
import sys, os, itertools, random, json
sys.dont_write_bytecode = True
import numpy as np
E = '/Users/adamburan/ic-work/crypto/research/ic_mega_pass_20261031/experiments/L9-124-falls-structure/src'
sys.path.insert(0, E)
import sys9, lib  # generator under test

# ---------- own field ----------
def mulmod(a, b, m, n):
    r = 0
    while b:
        if b & 1: r ^= a
        b >>= 1; a <<= 1
        if a >> n & 1: a ^= m
    return r
def modulus_own(n):
    # lowest-weight irreducible trinomial/pentanomial, independent irreducibility test (prime n: x^(2^n)=x and no deg-1 factor)
    cands = [(1 << n) | (1 << c) | 1 for c in range(1, n)]
    for m in cands:
        x = 2
        t = x
        for _ in range(n): t = mulmod(t, t, m, n)
        if t == x:
            # no root in F2: m(0)=1, m(1)=weight parity
            if bin(m).count('1') % 2 == 1:
                return m
    raise SystemExit('no trinomial')
def trace(x, m, n):
    s = 0; t = x
    for _ in range(n):
        s ^= t; t = mulmod(t, t, m, n)
    assert s in (0, 1)
    return s
def frob(x, i, m, n):
    for _ in range(i % n): x = mulmod(x, x, m, n)
    return x

def draw(n, s):
    k = (n + 1) // 2; rng = random.Random(1000 * n + 100 * k + s)
    m = sys9.modulus(n)
    while True:
        V = [rng.getrandbits(n) for _ in range(k)]
        piv = {}; r = 0
        for v in V:
            while v:
                p = v.bit_length() - 1
                if p in piv: v ^= piv[p]
                else: piv[p] = v; r += 1; break
        if r == k: break
    z = rng.randrange(1, 1 << n); a6 = rng.randrange(1, 1 << n)
    return m, V, z, a6

def gen_check(n, s, arm='A', trials=200):
    """Does lib.system(n,k,s)['A'] equal the K-valued F = (ab)^2 + z ab + z^2(a^2+b^2) + a6 of PROTOCOL?"""
    k = (n + 1) // 2; m, V, z, a6 = draw(n, s)
    # is the modulus irreducible? (prime n)
    x = 2; t = x
    for _ in range(n): t = mulmod(t, t, m, n)
    assert t == x and bin(m).count('1') % 2 == 1, 'modulus not irreducible'
    eqs = lib.system(n, k, s)[arm]
    rng = random.Random(5)
    bad = 0
    for _ in range(trials):
        X = rng.getrandbits(k); Y = rng.getrandbits(k)
        a = 0; b = 0
        for j in range(k):
            if X >> j & 1: a ^= V[j]
            if Y >> j & 1: b ^= V[j]
        ab = mulmod(a, b, m, n); z2 = mulmod(z, z, m, n)
        F = mulmod(ab, ab, m, n) ^ mulmod(z, ab, m, n) ^ mulmod(z2, mulmod(a, a, m, n) ^ mulmod(b, b, m, n), m, n) ^ a6
        pt = X | (Y << k)
        val = 0
        for i, e in enumerate(eqs):
            v = 0
            for mm in e:
                if pt & mm == mm: v ^= 1
            val |= v << i
        # equations are bit-coordinates of F in the polynomial basis, so val should == F
        if val != F: bad += 1
    return bad, trials

# ---------- own dump loader / linear algebra ----------
def load(pref, rd):
    monos = [int(x) for x in open(pref + '_monos.txt')]
    b = open('%s_r%d.bin' % (pref, rd), 'rb').read()
    nl, nr = np.frombuffer(b[:16], dtype=np.int64)
    WL = (nl + 63) // 64
    a = np.frombuffer(b[16:], dtype=np.uint64).reshape(nr, WL)
    return monos, [int.from_bytes(r.tobytes(), 'little') for r in a]
def rank_basis(rows):
    piv = {}
    for r in rows:
        while r:
            lb = r & -r
            q = piv.get(lb)
            if q is None: piv[lb] = r; break
            r ^= q
    return piv
def rank(rows): return len(rank_basis(rows))

def Trows_for_window(n, k, V, V2, window, monos, m, maxdeg=3):
    idx = {mm: i for i, mm in enumerate(monos)}
    rows = []
    trm = 0
    for t in range(n):
        trm |= trace(1 << t, m, n) << t
    # linear forms a_i = sum_j frob^i(V_j) x_j  (bit j), b_i = sum_j frob^i(V2_j) y_j (bit k+j)
    cache = {}
    def form(blk, i):
        key = (blk, i % n)
        if key not in cache:
            VV = V if blk == 0 else V2
            cache[key] = {1 << (j + k * blk): frob(VV[j], i, m, n) for j in range(k)}
        return cache[key]
    for da in range(maxdeg + 1):
        for db in range(maxdeg + 1 - da):
            for Ia in itertools.combinations(window, da):
                for Ib in itertools.combinations(window, db):
                    p = {0: 1}
                    for blk, i in [(0, i) for i in Ia] + [(1, i) for i in Ib]:
                        f = form(blk, i); q = {}
                        for mm1, c1 in p.items():
                            for mm2, c2 in f.items():
                                mm = mm1 | mm2  # Boolean ring: x*x=x
                                q[mm] = q.get(mm, 0) ^ mulmod(c1, c2, m, n)
                        p = {a: b for a, b in q.items() if b}
                    rws = [0] * n
                    for mm, c in p.items():
                        bit = 1 << idx[mm]; cc = c
                        for t in range(n):
                            if bin(cc & trm).count('1') & 1: rws[t] ^= bit
                            cc <<= 1
                            if cc >> n & 1: cc ^= m
                    rows.extend(rws)
    return rows

def dim_cap(Wrows, Trows):
    return rank(Wrows) + rank(Trows) - rank(list(Wrows) + list(Trows))

def explained(pref, n, s, window, name='A', V2mode=None, Vover=None):
    k = (n + 1) // 2; m, V, z, a6 = draw(n, s)
    V2 = V
    if Vover is not None: V = Vover; V2 = Vover
    monos, R2 = load(pref, 2); _, R3 = load(pref, 3)
    T = Trows_for_window(n, k, V, V2, window, monos, m)
    d2 = dim_cap(R2, T); d3 = dim_cap(R3, T)
    return dict(window=list(window), dimT=rank(T), dimW2=rank(R2), dimW3=rank(R3), cap2=d2, cap3=d3, explained=d3 - d2, new=rank(R3) - rank(R2))

if __name__ == '__main__':
    cmd = sys.argv[1]
    if cmd == 'gen':
        for (n, s) in [(31, 2), (29, 0), (13, 1), (17, 0)]:
            print('gen A', n, s, gen_check(n, s))
    elif cmd == 'expl':
        pref = sys.argv[2]; n = int(sys.argv[3]); s = int(sys.argv[4]); win = [int(x) for x in sys.argv[5].split(',')]
        print(json.dumps(explained(pref, n, s, win)))
    elif cmd == 'null':  # random-subspace null: T built from an unrelated random subspace
        pref = sys.argv[2]; n = int(sys.argv[3]); s = int(sys.argv[4]); win = [int(x) for x in sys.argv[5].split(',')]
        k = (n + 1) // 2
        rng = random.Random(777)
        while True:
            Vr = [rng.getrandbits(n) for _ in range(k)]
            if rank(Vr) == k: break
        print(json.dumps(explained(pref, n, s, win, Vover=Vr)))
