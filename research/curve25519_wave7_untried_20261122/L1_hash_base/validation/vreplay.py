#!/usr/bin/env python3
"""Validator: independent replay of L1 bases with own point arithmetic (does not import producer code).

usage: vreplay.py L1_DIR MODE K NBASES OUT_JSON
  MODE full : k = 16 style. Own discrete-log table (own generator), every signed pair sum computed by
              explicit affine addition, E[r] membership read from the own log of the computed sum,
              exact coverage per base compared with the recorded column; MC with OWN targets
              (own generator, own seed) and every hit re-verified by explicit arithmetic.
  MODE spot : any k. No log table. Coset t_i of each base point from [r]P_i matched against
              [t]Q (Q = [r]G, G of order 8r); only the signed sums with s_i t_i + s_j t_j = 0 mod 8 are
              added explicitly; exact coverage compared with the recorded column for bases 0..NBASES-1.
Bases are rebuilt from the PROTOCOL.md rules (sections 4 and 5): seed(parts) = first 8 bytes BE of
SHA-256("L1|v1|" + "|".join(parts)); hash base = rank window [bN, (b+1)N) of the valid u sorted by
SHA-256(key || u as 4 bytes LE)[:8] read big-endian.
"""
import bisect, hashlib, json, math, sys, time
import numpy as np

L1, MODE, K, NB, OUTF = sys.argv[1], sys.argv[2], int(sys.argv[3]), int(sys.argv[4]), sys.argv[5]
CURVES = {16: (65437, 1058), 18: (262133, 78), 20: (1048573, 926), 22: (4194301, 43186), 24: (16777141, 7842)}
KEYS = {'key1': b'curve25519-wave7-L1-hashbase-key-1', 'key2': b'curve25519-wave7-L1-hashbase-key-2'}
PLANT = {'plant3': 1.0149, 'plant5': 1.0247}
p, A = CURVES[K]
facts = {f['k']: f for f in json.load(open('vfacts.json'))}[K]
n = facts['order_euler']; r = facts['r']
Ns = facts['N']


def seed(*parts):
    return int.from_bytes(hashlib.sha256(('L1|v1|' + '|'.join(str(x) for x in parts)).encode()).digest()[:8], 'big')


# ---------------- own field / curve arithmetic ----------------
def fx(u):
    return (u * u % p * u + A * u % p * u + u) % p


def tonelli(a):
    a %= p
    if a == 0:
        return 0
    if pow(a, (p - 1) // 2, p) != 1:
        return None
    q, s = p - 1, 0
    while q % 2 == 0:
        q //= 2; s += 1
    z = 2
    while pow(z, (p - 1) // 2, p) != p - 1:
        z += 1
    m, c, t, x = s, pow(z, q, p), pow(a, q, p), pow(a, (q + 1) // 2, p)
    while t != 1:
        i, t2 = 0, t
        while t2 != 1:
            t2 = t2 * t2 % p; i += 1
        b = pow(c, 1 << (m - i - 1), p)
        m, c, t, x = i, b * b % p, t * b * b % p, x * b % p
    assert x * x % p == a
    return x


def add(P, Q):
    if P is None:
        return Q
    if Q is None:
        return P
    x1, y1 = P; x2, y2 = Q
    if x1 == x2:
        if (y1 + y2) % p == 0:
            return None
        lam = (3 * x1 * x1 + 2 * A * x1 + 1) * pow(2 * y1 % p, p - 2, p) % p
    else:
        lam = (y2 - y1) * pow((x2 - x1) % p, p - 2, p) % p
    x3 = (lam * lam - A - x1 - x2) % p
    return (x3, (lam * (x1 - x3) - y1) % p)


def neg(P):
    return None if P is None else (P[0], (-P[1]) % p)


def smul(k, P):
    Rr = None; Q = P
    while k:
        if k & 1:
            Rr = add(Rr, Q)
        Q = add(Q, Q); k >>= 1
    return Rr


def pt(u):
    v = tonelli(fx(u)); assert v is not None
    return (u, v)


T0 = (0, 0)
t_start = time.time()
# ---------------- valid columns (own Euler-criterion mask) ----------------
e = (p - 1) // 2
mask = np.zeros(p, dtype=bool)
for lo in range(0, p, 1 << 21):
    x = np.arange(lo, min(p, lo + (1 << 21)), dtype=np.int64)
    f = (x * x % p * x % p + A * (x * x % p) % p + x) % p
    res = np.ones_like(f); b = f.copy(); ee = e
    while ee:
        if ee & 1:
            res = res * b % p
        b = b * b % p; ee >>= 1
    mask[lo:lo + len(x)] = res == 1
E8u = set(facts['E8_u_values'])
for u in E8u:
    mask[u] = False
VU = np.nonzero(mask)[0].astype(np.int64)          # valid u, ascending
M = len(VU)
assert M == facts['valid_columns_expected']
A3 = A * pow(3, p - 2, p) % p
Y = (VU + A3) % p
oy = np.argsort(Y)
VY = VU[oy]; Ys = Y[oy]                              # valid u in Weierstrass-x order


def hash_rank(key):
    base = hashlib.sha256(key)
    H = np.empty(M, dtype=np.uint64)
    for i, u in enumerate(VU.tolist()):
        h = base.copy(); h.update(u.to_bytes(4, 'little')); H[i] = int.from_bytes(h.digest()[:8], 'big')
    o = np.argsort(H, kind='stable')
    return VU[o], H[o]


HU1, H1 = hash_rank(KEYS['key1']); HU2, H2 = hash_rank(KEYS['key2'])
rep = dict(k=K, mode=MODE, nbases=NB, M=M, prep_s=None, cells={})
rep['hash_ties_key1'] = int((np.diff(H1) == 0).sum()); rep['hash_ties_key2'] = int((np.diff(H2) == 0).sum())


def proj_first(us, N):
    seen = set(); outp = []; used = 0
    for u in us:
        used += 1
        Q = smul(8, pt(int(u)))
        assert Q is not None
        if Q[0] not in seen:
            seen.add(Q[0]); outp.append(Q)
            if len(outp) == N:
                break
    assert len(outp) == N
    return outp, used


def bases(N, b):
    """dict arm -> list of points (u, v); plus info"""
    out = {}; info = {}
    if b == -1:
        out['bits_lo0'] = [pt(int(u)) for u in VY[:N]]
        return out, info
    L = int(np.random.default_rng(seed('bitsL', K, N, b)).integers(0, p))
    s = int(np.searchsorted(Ys, L)) % M
    out['bits_rand'] = [pt(int(VY[(s + i) % M])) for i in range(N)]
    for nm in ('rand', 'rand2'):
        idx = np.random.default_rng(seed(nm, K, N, b)).choice(M, N, replace=False)
        out[nm] = [pt(int(VU[i])) for i in idx]
    out['hash'] = [pt(int(u)) for u in HU1[b * N:(b + 1) * N]]
    out['hash2'] = [pt(int(u)) for u in HU2[b * N:(b + 1) * N]]
    idx = np.random.default_rng(seed('randp', K, N, b)).choice(M, min(M, int(1.3 * N) + 10), replace=False)
    out['rand_proj'], _ = proj_first(VU[idx], N)
    out['hash_proj'], used = proj_first(HU1[b * N:b * N + int(1.3 * N) + 10], N)
    info['hash_proj_used'] = used
    for nm, fct in PLANT.items():
        rg = np.random.default_rng(seed(nm, K, N, b)); x = fct * N
        Np = int(x) + int(rg.random() < x - int(x))
        idx = rg.choice(M, Np, replace=False)
        out[nm] = [pt(int(VU[i])) for i in idx]
    return out, info


# ---------------- coverage ----------------
if MODE == 'full':
    # own generator: smallest u >= 2 with a point of order exactly 8r
    u = 2
    while True:
        if mask[u]:
            G = pt(u)
            if smul(8 * r, G) is None and smul(4 * r, G) is not None and smul(8, G) is not None:
                break
        u += 1
    rep['own_generator_u'] = u
    ulog = [0] * p; uofk = [0] * (n // 2 + 1)
    P = G
    for kk in range(1, n // 2 + 1):
        ulog[P[0]] = kk; uofk[kk] = P[0]
        P = add(P, G)
    assert uofk[n // 2] == 0
    uofk_np = np.array(uofk, dtype=np.int64)

    def coverage(Pts, want_t0):
        cov = {}; cov0 = {}
        Nn = len(Pts)
        for i in range(Nn):
            Pi = Pts[i]
            sums = [(add(Pi, Pi), i, i, 1)]
            for j in range(i + 1, Nn):
                sums.append((add(Pi, Pts[j]), i, j, 1)); sums.append((add(Pi, neg(Pts[j])), i, j, -1))
            for S, a, bb, sg in sums:
                if S is None:
                    continue
                if ulog[S[0]] % 8 == 0 and S[0] != 0:
                    cov.setdefault(S[0], (a, bb, sg))
                elif want_t0 and S != T0 and ulog[S[0]] % 8 == 4:
                    S2 = add(S, T0)
                    cov0.setdefault(S2[0], (a, bb, sg))
        return cov, cov0
else:
    # spot mode: coset index of each point via [r]P against [t]Q
    u = 2
    while True:
        if mask[u]:
            G = pt(u)
            if smul(8 * r, G) is None and smul(4 * r, G) is not None and smul(8, G) is not None:
                break
        u += 1
    Q = smul(r, G)
    tQ = {}
    Pt_ = None
    for t in range(8):
        tQ[Pt_] = t; Pt_ = add(Pt_, Q)
    assert len(tQ) == 8

    def coverage(Pts, want_t0):
        ts = [tQ[smul(r, P_)] for P_ in Pts]
        cov = {}; cov0 = {}
        Nn = len(Pts)
        for i in range(Nn):
            ti = ts[i]
            if (2 * ti) % 8 == 0:
                S = add(Pts[i], Pts[i])
                if S is not None:
                    cov.setdefault(S[0], (i, i, 1))
            elif want_t0 and (2 * ti) % 8 == 4:
                S = add(Pts[i], Pts[i])
                if S != T0:
                    cov0.setdefault(add(S, T0)[0], (i, i, 1))
            for j in range(i + 1, Nn):
                for sg in (1, -1):
                    c = (ti + sg * ts[j]) % 8
                    if c == 0:
                        S = add(Pts[i], Pts[j] if sg == 1 else neg(Pts[j]))
                        if S is not None:
                            cov.setdefault(S[0], (i, j, sg))
                    elif want_t0 and c == 4:
                        S = add(Pts[i], Pts[j] if sg == 1 else neg(Pts[j]))
                        if S != T0:
                            cov0.setdefault(add(S, T0)[0], (i, j, sg))
        return cov, cov0

rep['prep_s'] = time.time() - t_start
recorded = {}
for N in Ns:
    d = np.load(f'{L1}/out/cell_k{K}_N{N}.npz'); recorded[N] = {a: d[a] for a in d.files if a != 'meta'}
    meta = json.loads(str(d['meta']))
    cell = dict(N=N, theta_b0_mine=int(H1[N - 1]), theta_b0_recorded=meta['theta_b0'],
                hash_V_size_at_theta=int((H1 <= H1[N - 1]).sum()),
                hash_literal_lt_next=int((H1 < H1[N]).sum()),
                exact_mismatch={}, N_mismatch={}, t0_mismatch=0, checked_bases=0, hash_proj_overlap_bases=0,
                window_disjoint=True)
    m_u = meta['m_u']; m_p = meta['m_p']
    # disjointness of hash windows (as constructed) over the full 512
    seen = set()
    for b in range(512):
        w = set(HU1[b * N:(b + 1) * N].tolist())
        if len(w) != N or (w & seen):
            cell['window_disjoint'] = False
        seen |= w
    mc = {}
    own_rng = np.random.default_rng(seed('validator-own-targets', K, N))
    verify_fail = 0; verified = 0
    blist = [-1] + list(range(NB))
    for b in blist:
        bs, info = bases(N, b)
        if info.get('hash_proj_used', N) > N:
            cell['hash_proj_overlap_bases'] += 1
        if MODE == 'full':
            js = own_rng.integers(1, r, size=max(m_u, m_p))
        for arm, Pts in bs.items():
            rec = recorded[N][arm][0 if b == -1 else b]
            want_t0 = arm == 'rand'
            cov, cov0 = coverage(Pts, want_t0)
            ex = 2 * len(cov)
            if len(Pts) != rec[0]:
                cell['N_mismatch'][arm] = cell['N_mismatch'].get(arm, 0) + 1
            if ex != rec[1]:
                cell['exact_mismatch'][arm] = cell['exact_mismatch'].get(arm, 0) + 1
            if want_t0:
                ex0 = 2 * len(set(cov) | set(cov0))
                if ex0 != rec[2]:
                    cell['t0_mismatch'] += 1
            if MODE == 'full':
                m = m_p if arm.endswith('_proj') else m_u
                kR = 8 * js[:m] % n; kR = np.minimum(kR, n - kR); uR = uofk_np[kR]
                covk = np.fromiter(cov.keys(), dtype=np.int64, count=len(cov))
                hit = np.isin(uR, covk)
                hits = int(hit.sum()); h0 = hits
                # verify every hit by explicit arithmetic
                for t in np.flatnonzero(hit):
                    a_, b_, sg = cov[int(uR[t])]
                    S = add(Pts[a_], Pts[b_] if sg == 1 else neg(Pts[b_])) if a_ != b_ else add(Pts[a_], Pts[a_])
                    if S is not None and S[0] == int(uR[t]):
                        verified += 1
                    else:
                        verify_fail += 1
                if want_t0:
                    # T0-keyed table: hit if R is a pair sum or R + T0 is a pair sum, i.e. u(R) in cov | cov0
                    cov0k = np.fromiter(cov0.keys(), dtype=np.int64, count=len(cov0)) if cov0 else np.zeros(0, np.int64)
                    hit0 = np.isin(uR, cov0k)
                    allh = hit | hit0
                    h0 = int(allh.sum())
                    for t in np.flatnonzero(hit0 & ~hit):
                        uRt = int(uR[t])
                        a_, b_, sg = cov0[uRt]
                        S = add(Pts[a_], Pts[b_] if sg == 1 else neg(Pts[b_])) if a_ != b_ else add(Pts[a_], Pts[a_])
                        RT = add(pt(uRt), T0)                       # explicit R + T0
                        okv = S is not None and add(S, T0)[0] == uRt and RT is not None and RT[0] == S[0]
                        verified += okv; verify_fail += (not okv)
                st = mc.setdefault(arm, dict(hits=0, targets=0, exact_sum=0, nb=0, per_base=[]))
                st['hits'] += hits; st['targets'] += m; st['exact_sum'] += ex; st['nb'] += 1
                st['per_base'].append([hits, m, ex])
                if want_t0:
                    st0 = mc.setdefault('T0', dict(hits=0, targets=0, exact_sum=0, nb=0, per_base=[]))
                    st0['hits'] += h0; st0['targets'] += m; st0['exact_sum'] += ex0; st0['nb'] += 1
                    st0['per_base'].append([h0, m, ex0, hits])
        cell['checked_bases'] += 1
    if MODE == 'full':
        cmp = {}
        for arm, st in mc.items():
            if arm == 'bits_lo0':
                continue
            recx = recorded[N]['rand' if arm == 'T0' else arm]
            hc = 5 if arm == 'T0' else 4
            rec_rate_all = recx[:, hc].sum() / recx[:, 3].sum()
            rec_rate_same = recx[:NB, hc].sum() / recx[:NB, 3].sum()
            mine = st['hits'] / st['targets']
            pb = np.array(st['per_base'], dtype=float)
            # binomial-only SE for same bases (exact per base identical): sum m y (1-y)
            y = pb[:, 2] / (r - 1)
            se_bin = math.sqrt((pb[:, 1] * y * (1 - y)).sum()) / pb[:, 1].sum()
            z_same = (mine - rec_rate_same) / (math.sqrt(2) * se_bin)
            # vs the full 512-base record: bases + targets (ratio-estimator SE for both)
            def se_ratio(h, m):
                R_ = h.sum() / m.sum(); return math.sqrt(((h - R_ * m) ** 2).sum() / m.sum() ** 2 * len(h) / (len(h) - 1))
            se_mine = se_ratio(pb[:, 0], pb[:, 1]); se_rec = se_ratio(recx[:, hc].astype(float), recx[:, 3].astype(float))
            z_all = (mine - rec_rate_all) / math.sqrt(se_mine ** 2 + se_rec ** 2)
            z_exact = (st['hits'] - (pb[:, 1] * y).sum()) / math.sqrt((pb[:, 1] * y * (1 - y)).sum())
            cmp[arm] = dict(mine_rate=mine, recorded_rate_same_bases=rec_rate_same, recorded_rate_all512=rec_rate_all,
                            z_vs_recorded_same_bases=z_same, z_vs_recorded_all512=z_all, z_mine_vs_own_exact=z_exact,
                            hits=st['hits'], targets=st['targets'])
        # ratios on my own replay (hash/rand etc.)
        def rr(a):
            return mc[a]['hits'] / mc[a]['targets']
        cell['replay_ratios'] = {'hash/rand': rr('hash') / rr('rand'), 'hash/bits_rand': rr('hash') / rr('bits_rand'),
                                 'hash/hash2': rr('hash') / rr('hash2'), 'rand/rand2': rr('rand') / rr('rand2'),
                                 'hash_proj/rand_proj': rr('hash_proj') / rr('rand_proj'), 'rand_proj/rand': rr('rand_proj') / rr('rand'),
                                 'plant3/rand': rr('plant3') / rr('rand'), 'plant5/rand': rr('plant5') / rr('rand'),
                                 'T0/blind': mc['T0']['hits'] / mc['rand']['hits']}
        cell['mc_compare'] = cmp
        cell['verified_hits'] = verified; cell['verify_fail'] = verify_fail
    rep['cells'][str(N)] = cell
    print(json.dumps({kk: vv for kk, vv in cell.items() if kk not in ('mc_compare',)}, default=float), flush=True)
rep['total_s'] = time.time() - t_start
json.dump(rep, open(OUTF, 'w'), indent=1, default=float)
