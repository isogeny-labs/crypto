
# H4: FPPR prime-degree descent, m=3, first-fall / solving-degree trend. Plain-Python Sage.
# Pipeline: Semaev S_4 = Res_u(S_3(x1,x2,u), S_3(u,x3,xR)) (chained S_3, one intermediate u eliminated
# symbolically), x_i in F_2-subspace V (dim l), Weil restriction => n Boolean eqs of degree<=6 in 3l vars.
# Degrees measured with our own Boolean Macaulay-matrix profile (random-projection ranks, M4RI ranks);
# PolyBoRi Groebner basis used for solution set / planted correctness control.
import os, sys, json, time, math, random, itertools, traceback
import multiprocessing as mp
from multiprocessing.connection import wait as mpwait
import numpy as np

OUT = os.environ.get('ISOLAB_OUTPUT_DIR', '.')
T0 = time.time()
GLOBAL_DEADLINE = 6500.0
SEED = 20261022
set_random_seed(SEED)
NS = [11, 13, 17, 19, 23, 29, 31, 37, 41]
BUDGET = {11: 600, 13: 600, 17: 600, 19: 600, 23: 900, 29: 900, 31: 900, 37: 1500, 41: 1500}
CONC = 4
ROWCAP = 9000
M64 = np.uint64
OMEGA = 2.37

# ---------------- GF(2^n) vector arithmetic (ints as bitvectors, poly basis) ----------------
def gfmul(a, b, n, poly):
    a = np.asarray(a, dtype=np.uint64); b = np.asarray(b, dtype=np.uint64)
    a, b = np.broadcast_arrays(a, b)
    a = a.copy(); res = np.zeros(a.shape, dtype=np.uint64)
    one = M64(1); pol = M64(poly); nn = M64(n)
    for i in range(n):
        res ^= a * ((b >> M64(i)) & one)
        a = a << one
        a ^= ((a >> nn) & one) * pol
    return res

def gfmul_s(a, b, n, poly):
    r = 0
    while b:
        if b & 1: r ^= a
        b >>= 1; a <<= 1
        if (a >> n) & 1: a ^= poly
    return r

def to_int(e):
    return sum(int(c) << i for i, c in enumerate(e.polynomial().list()))

def popcount(a):
    return np.bitwise_count(a).astype(np.int64)

# ---------------- instance construction ----------------
def build_instance(n, inst, arm):
    rng = random.Random(SEED * 1000 + n * 10 + inst)
    set_random_seed(SEED + n * 10 + inst)
    l = -(-n // 3); N = 3 * l
    mod = PolynomialRing(GF(2), 'x').irreducible_element(n)
    K = GF(2 ** n, 'z', modulus=mod)
    poly = sum(int(c) << i for i, c in enumerate(mod.list()))
    fromint = lambda v: K([(v >> i) & 1 for i in range(n)])
    a2 = fromint(rng.getrandbits(n)); a6 = fromint(rng.getrandbits(n))
    while a6 == 0: a6 = fromint(rng.getrandbits(n))
    E = EllipticCurve(K, [1, a2, 0, 0, a6])
    # subspace V
    while True:
        basis = [rng.getrandbits(n) for _ in range(l)]
        # rank check over GF(2)
        rows = list(basis); rk = 0
        for bit in range(n):
            piv = None
            for r in rows:
                if (r >> bit) & 1: piv = r; break
            if piv is None: continue
            rows.remove(piv); rk += 1
            rows = [r ^ piv if (r >> bit) & 1 else r for r in rows]
        if rk == l: break
    # planted points
    def point_with_x(xint):
        x = fromint(xint)
        if x == 0: return None
        c = x + a2 + a6 / x ** 2
        t = c; hs = c
        for _ in range((n - 1) // 2):
            t = t ** 4; hs += t
        # half-trace solution of w^2+w=c (n odd, Tr(c)=0)
        if hs ** 2 + hs != c: return None
        return E(x, x * hs)
    planted = None
    if arm == 'planted':
        pts = []; cm = []
        while len(pts) < 3:
            cbits = rng.getrandbits(l)
            if cbits == 0: continue
            xi = 0
            for j in range(l):
                if (cbits >> j) & 1: xi ^= basis[j]
            P = point_with_x(xi)
            if P is None: continue
            pts.append(P); cm.append(cbits)
        R = pts[0] + pts[1] + pts[2]
        if R == 0: return None
        planted = cm[0] | (cm[1] << l) | (cm[2] << (2 * l))
        xR = R[0]
        S4check = [to_int(P[0]) for P in pts]
    else:
        while True:
            R = E.random_point()
            if R != 0: break
        xR = R[0]
    # S_4 = Res_u via quadratic resultant
    A = PolynomialRing(K, 'x1,x2,x3'); x1, x2, x3 = A.gens()
    A2 = (x1 + x2) ** 2; A1 = x1 * x2; A0 = x1 ** 2 * x2 ** 2 + a6
    B2 = (x3 + xR) ** 2; B1 = x3 * xR; B0 = x3 ** 2 * xR ** 2 + a6
    S4 = (A2 * B0 - A0 * B2) ** 2 + (A2 * B1 - A1 * B2) * (A1 * B0 - A0 * B1)
    sabc = {tuple(int(e) for e in ex): to_int(c) for ex, c in S4.dict().items()}
    # basis monomials per group
    pairs = [(j, k) for j in range(l) for k in range(j + 1, l)]
    nb = 1 + l + len(pairs)
    bmask = [0] + [1 << j for j in range(l)] + [(1 << j) | (1 << k) for j, k in pairs]
    u = {a: np.zeros(nb, dtype=np.uint64) for a in range(5)}
    u[0][0] = 1
    for j in range(l):
        v = basis[j]; v2 = gfmul_s(v, v, n, poly); v4 = gfmul_s(v2, v2, n, poly); v3 = gfmul_s(v, v2, n, poly)
        u[1][1 + j] = v; u[2][1 + j] = v2; u[4][1 + j] = v4; u[3][1 + j] = v3
    for idx, (j, k) in enumerate(pairs):
        vj, vk = basis[j], basis[k]
        u[3][1 + l + idx] = gfmul_s(vj, gfmul_s(vk, vk, n, poly), n, poly) ^ gfmul_s(vk, gfmul_s(vj, vj, n, poly), n, poly)
    supp = {a: np.nonzero(u[a])[0] for a in range(5)}
    T = np.zeros((nb, nb, nb), dtype=np.uint64)
    for (a, b, c), s in sabc.items():
        if s == 0: continue
        sa, sb, sc = supp[a], supp[b], supp[c]
        t = gfmul(gfmul(u[a][sa][:, None], np.uint64(s), n, poly), u[b][sb][None, :], n, poly)
        t = gfmul(t[:, :, None], u[c][sc][None, None, :], n, poly)
        T[np.ix_(sa, sb, sc)] ^= t
    bm = np.array(bmask, dtype=np.uint64)
    eqs = []
    for k in range(n):
        ip, iq, ir = np.nonzero((T >> M64(k)) & M64(1))
        if len(ip) == 0: continue
        masks = bm[ip] | (bm[iq] << M64(l)) | (bm[ir] << M64(2 * l))
        eqs.append(masks)
    # field data for enumeration
    return dict(n=n, l=l, N=N, poly=poly, basis=basis, eqs=eqs, sabc=sabc, planted=planted,
                a6=to_int(a6), xR=to_int(xR))

def eval_eqs_at(eqs, cmask):
    cm = M64(cmask); bad = 0
    for m in eqs:
        cnt = int(np.count_nonzero((m & ~cm) == 0))
        bad += cnt & 1
    return bad

def enumerate_solutions(sysd):
    n, l, poly, basis, sabc = sysd['n'], sysd['l'], sysd['poly'], sysd['basis'], sysd['sabc']
    sz = 1 << l
    V = np.zeros(sz, dtype=np.uint64)
    for j in range(l):
        V[(1 << j):(1 << (j + 1))] = V[0:(1 << j)] ^ np.uint64(basis[j])
    P = [None] * 5; P[0] = np.ones(sz, dtype=np.uint64); P[1] = V
    P[2] = gfmul(V, V, n, poly); P[4] = gfmul(P[2], P[2], n, poly); P[3] = gfmul(V, P[2], n, poly)
    sols = []
    for i1 in range(sz):
        G = {}
        for c in range(5):
            g = np.zeros(sz, dtype=np.uint64)
            for a in range(5):
                for b in range(5):
                    s = sabc.get((a, b, c), 0)
                    if s:
                        g ^= gfmul(np.uint64(gfmul_s(s, int(P[a][i1]), n, poly)), P[b], n, poly)
            G[c] = g
        F = np.zeros((sz, sz), dtype=np.uint64)
        for c in range(5):
            if G[c].any():
                F ^= gfmul(G[c][:, None], P[c][None, :], n, poly)
        i2, i3 = np.nonzero(F == 0)
        for a_, b_ in zip(i2, i3):
            sols.append(i1 | (int(a_) << l) | (int(b_) << (2 * l)))
    return sols

# ---------------- Macaulay profile ----------------
C1 = M64(0x9E3779B97F4A7C15); C2 = M64(0xBF58476D1CE4E5B9); C3 = M64(0x94D049BB133111EB)
C4 = M64(0x2545F4914F6CDD1D); C5 = M64(0xD1B54A32D192ED03)

def hashvec(masks, W):
    seed = (np.arange(1, W + 1, dtype=np.uint64) * C5)
    x = masks[:, None] * C4 + seed[None, :]
    z = x + C1
    z = (z ^ (z >> M64(30))) * C2
    z = (z ^ (z >> M64(27))) * C3
    return z ^ (z >> M64(31))

def row_proj(fm, m, W, D, chunk=20000):
    full = np.zeros(W, dtype=np.uint64); top = np.zeros(W, dtype=np.uint64)
    for s in range(0, len(fm), chunk):
        mk = fm[s:s + chunk] | M64(m)
        h = hashvec(mk, W)
        full ^= np.bitwise_xor.reduce(h, axis=0)
        sel = popcount(mk) == D
        if sel.any():
            top ^= np.bitwise_xor.reduce(h[sel], axis=0)
    return full, top

def rank_rows(Wmat, K):
    if Wmat.shape[0] == 0: return 0
    bits = np.unpackbits(Wmat.view(np.uint8), axis=1, bitorder='little')[:, :K]
    r = bits.shape[0]
    M = matrix(GF(2), r, K)
    step = 1500
    for i in range(0, r, step):
        M.set_block(i, 0, matrix(GF(2), bits[i:i + step].tolist()))
    return int(M.rank())

def squarefree_masks(N, emax):
    out = []
    for e in range(emax + 1):
        for comb in itertools.combinations(range(N), e):
            mk = 0
            for j in comb: mk |= 1 << j
            out.append((e, mk))
    return out

def macaulay_profile(sysd, s_known, conn, deadline, deep=False):
    N = sysd['N']; eqs = sysd['eqs']
    degs = [int(popcount(m).max()) for m in eqs]
    D0 = min(degs); rprev = 0; dff = None; dsolv = None; recs = []
    tot = lambda D: sum(math.comb(N, k) for k in range(D + 1))
    for D in range(D0, N + 1):
        if time.time() > deadline: break
        emax = D - D0
        mult = squarefree_masks(N, emax)
        rowsel = []
        for i, d in enumerate(degs):
            if d <= D:
                for e, mk in mult:
                    if e + d <= D: rowsel.append((i, mk))
        nrows = len(rowsel)
        if nrows > (30000 if (deep and sysd['n'] <= 11) else ROWCAP):
            conn.send(('D', dict(D=D, status='rowcap', rows=nrows))); break
        K = nrows + 48; W = -(-K // 64)
        t = time.time()
        Fm = np.zeros((nrows, W), dtype=np.uint64); Tm = np.zeros((nrows, W), dtype=np.uint64)
        for ri, (i, mk) in enumerate(rowsel):
            Fm[ri], Tm[ri] = row_proj(eqs[i], mk, W, D)
            if (ri & 63) == 0 and time.time() > deadline:
                conn.send(('D', dict(D=D, status='timeout_building', rows=nrows))); return dff, dsolv, recs
        r = rank_rows(Fm, K)
        nz = Tm.any(axis=1)
        rt = rank_rows(Tm[nz], K)
        fall = r - rt - rprev
        Q = tot(D) - r
        rec = dict(D=D, rows=nrows, cols=tot(D), rank=r, rank_top=rt, fall=fall, Q=Q, t=time.time() - t)
        recs.append(rec); conn.send(('D', rec))
        if fall > 0 and dff is None: dff = D
        if s_known is not None and dsolv is None and Q == s_known: dsolv = D
        rprev = r
        if dsolv is not None: break
        if dff is not None and not deep: break
    return dff, dsolv, recs

# ---------------- workers ----------------
def worker_mac(n, inst, arm, deep, deadline, conn):
    try:
        t0 = time.time()
        sd = build_instance(n, inst, arm)
        if sd is None: conn.send(('fail', 'degenerate')); return
        info = dict(N=sd['N'], l=sd['l'], neq=len(sd['eqs']), build_s=time.time() - t0,
                    eq_deg=sorted(set(int(popcount(m).max()) for m in sd['eqs'])),
                    eq_terms=int(np.mean([len(m) for m in sd['eqs']])))
        if sd['planted'] is not None:
            info['planted_residual'] = eval_eqs_at(sd['eqs'], sd['planted'])
        s = None
        if sd['l'] <= 8:
            sols = enumerate_solutions(sd); s = len(sols)
            info['n_solutions'] = s
            if sd['planted'] is not None: info['planted_in_solutions'] = sd['planted'] in set(sols)
        conn.send(('info', info))
        dff, dsolv, recs = macaulay_profile(sd, s, conn, deadline, deep)
        conn.send(('done', dict(d_ff=dff, d_solv=dsolv, time=time.time() - t0)))
    except Exception:
        conn.send(('fail', traceback.format_exc()))

def worker_gb(n, inst, arm, deadline, conn):
    try:
        t0 = time.time()
        sd = build_instance(n, inst, arm)
        N = sd['N']
        R = BooleanPolynomialRing(N, 'c'); g = R.gens()
        polys = []
        for m in sd['eqs']:
            p = R(0)
            for mk in m.tolist():
                mon = R(1)
                j = 0
                while mk:
                    if mk & 1: mon = mon * g[j]
                    mk >>= 1; j += 1
                p += mon
            polys.append(p)
        tb = time.time() - t0
        I = R.ideal(polys)
        GB = I.groebner_basis()
        tg = time.time() - t0 - tb
        res = dict(gb_time=tg, build=tb, gb_len=len(GB), inconsistent=(len(GB) == 1 and GB[0] == 1))
        if not res['inconsistent']:
            V = I.variety()
            res['n_solutions'] = len(V)
            if sd['planted'] is not None:
                pm = sd['planted']
                tgt = {g[j]: (pm >> j) & 1 for j in range(N)}
                res['planted_in_variety'] = any(all(int(v[g[j]]) == (pm >> j) & 1 for j in range(N)) for v in V)
            res['max_gb_deg'] = max(p.degree() for p in GB)
        else:
            res['n_solutions'] = 0; res['max_gb_deg'] = 0
        conn.send(('gb', res))
    except Exception:
        conn.send(('fail', traceback.format_exc()))

def run_pool(tasks, deadline):
    ctx = mp.get_context('fork')
    pending = list(tasks); running = {}; results = {t[0]: dict(msgs=[], status='unstarted') for t in tasks}
    while (pending or running):
        while pending and len(running) < CONC and time.time() < deadline:
            key, fn, args = pending.pop(0)
            pr, cw = ctx.Pipe(duplex=False)
            p = ctx.Process(target=fn, args=args + (deadline, cw)); p.start(); cw.close()
            running[key] = (p, pr); results[key]['status'] = 'running'
        if not running: break
        conns = {v[1]: k for k, v in running.items()}
        for c in mpwait(list(conns), timeout=2):
            k = conns[c]
            try:
                msg = c.recv(); results[k]['msgs'].append(msg)
            except EOFError:
                p = running[k][0]; p.join(1); results[k]['status'] = 'exited'; del running[k]
        for k in list(running):
            if not running[k][0].is_alive() and not running[k][1].poll():
                results[k]['status'] = 'exited'; del running[k]
        if time.time() > deadline:
            for k, (p, pr) in list(running.items()):
                p.terminate(); results[k]['status'] = 'killed_deadline'; del running[k]
            for t_ in pending: results[t_[0]]['status'] = 'unstarted'
            break
    return results

# ---------------- S_3 relation validation ----------------
def validate_s3():
    ok = True; checks = 0
    for n in (11, 13, 17):
        K = GF(2 ** n, 'z', modulus=PolynomialRing(GF(2), 'x').irreducible_element(n))
        a2 = K.random_element(); a6 = K.random_element()
        while a6 == 0: a6 = K.random_element()
        E = EllipticCurve(K, [1, a2, 0, 0, a6])
        S3 = lambda p, q, r: (p * q + p * r + q * r) ** 2 + p * q * r + a6
        for _ in range(15):
            P = E.random_point(); Q = E.random_point()
            if P == 0 or Q == 0 or P == Q or P == -Q: continue
            checks += 1
            if S3(P[0], Q[0], (P + Q)[0]) != 0 or S3(P[0], Q[0], (P - Q)[0]) != 0: ok = False
    return ok, checks

# ---------------- fits ----------------
def ols(xs, ys):
    xs = np.array(xs, float); ys = np.array(ys, float); m = len(xs)
    if m < 2: return None
    xb = xs.mean(); sxx = ((xs - xb) ** 2).sum()
    b = ((xs - xb) * (ys - ys.mean())).sum() / sxx; a = ys.mean() - b * xb
    if m >= 3:
        res = ys - (a + b * xs); s2 = (res ** 2).sum() / (m - 2)
        se = math.sqrt(s2 / sxx)
        from scipy import stats
        tq = stats.t.ppf(0.975, m - 2); ci = [b - tq * se, b + tq * se]
    else:
        se = None; ci = None
    return dict(a=a, b=b, se=se, ci=ci, m=m)

def cost_bits(D, N=132):
    cols = sum(math.comb(N, k) for k in range(max(0, int(round(D))) + 1))
    return OMEGA * math.log2(cols)

def write_result(res):
    tmp = os.path.join(OUT, 'result.json.tmp')
    open(tmp, 'w').write(json.dumps(res, indent=1, default=str))
    os.replace(tmp, os.path.join(OUT, 'result.json'))

def med(v):
    v = [x for x in v if x is not None]
    return float(np.median(v)) if v else None

# ---------------- main ----------------
def main():
    global RES
    print('main start', flush=True)
    res = dict(experiment='H4', degree_method=(
        "d_ff and d_solv are measured with an in-house Boolean Macaulay-matrix profile (squarefree monomials, field "
        "equations built in). For D=d_min.. we form all multiples m*f_i with deg(m)+deg(f_i)<=D, rank r_D (random GF(2) "
        "column projection to rows+48 dims, exact M4RI rank, failure prob ~2^-48) and r_top (rank of degree-D block). "
        "Degree fall F_D = r_D - r_top - r_{D-1}; d_ff = first D with F_D>0. d_solv = first D with Q_D = dim P_{<=D} - r_D "
        "equal to the true number of Boolean solutions s (s by exhaustive enumeration over V^3 for l<=8, else not "
        "available -> d_solv censored). PolyBoRi groebner_basis()+variety() is used for the correctness control "
        "(planted solution recovered) for n<=19; PolyBoRi exposes no step degrees. Chained S_3 with the single intermediate "
        "abscissa u eliminated by resultant gives S_4 (m=3 needs one intermediate, not two; two would be m=4/S_5). "
        "No FGHR 4-torsion symmetrization (random curves, E[4] not rational): raw system."),
        ns_planned=NS, seed=SEED, cells={}, d_ff={}, d_solv={}, notes=[])
    RES = res
    ok3, chk = validate_s3()
    res['validation'] = dict(S3_relation_verified=ok3, S3_checks=chk, planted_residual_zero=None, planted_solved_per_n={})
    write_result(res)
    stop_ff = False
    for n in NS:
        left = GLOBAL_DEADLINE - (time.time() - T0)
        if left < 120: res['notes'].append('global time exhausted before n=%d' % n); break
        bud = min(BUDGET[n], left - 30)
        ninst = 5 if n <= 13 else 3
        l = -(-n // 3)
        tstart = time.time(); deadline = tstart + bud
        tasks = []
        for i in range(ninst):
            for arm in ('planted', 'random'):
                tasks.append(((n, i, arm, 'mac'), worker_mac, (n, i, arm, False)))
        if n <= 19:
            for i in range(2):
                tasks.append(((n, i, 'planted', 'gb'), worker_gb, (n, i, 'planted')))
        tasks_deep = []
        try:
            rs = run_pool(tasks, deadline)
        except Exception:
            res['notes'].append('pool error n=%d: %s' % (n, traceback.format_exc())); rs = {}
        print('n',n,'pool done',{str(k):(v['status'],[m[0] for m in v['msgs']]) for k,v in rs.items()},flush=True)
        for k,v in rs.items():
            for m in v['msgs']:
                if m[0] in ('fail','info'): print(k,str(m[1])[:1500],flush=True)
        inst_recs = []
        for (n_, i, arm, kind), r in rs.items():
            if kind != 'mac': continue
            info = {}; dff = dsolv = None; Ds = []; done = None; fail = None
            for tag, p in r['msgs']:
                if tag == 'info': info = p
                elif tag == 'D': Ds.append(p)
                elif tag == 'done': done = p; dff = p['d_ff']; dsolv = p['d_solv']
                elif tag == 'fail': fail = p
            if done is None:
                # partial: recover d_ff from streamed records
                for p in Ds:
                    if p.get('fall', 0) > 0 and dff is None: dff = p['D']
                    if info.get('n_solutions') is not None and p.get('Q') == info['n_solutions'] and dsolv is None: dsolv = p['D']
            gb = None
            dsolv_method = 'macaulay_Q=s' if dsolv is not None else None
            g = rs.get((n_, i, arm, 'gb'))
            if g:
                for tag, p in g['msgs']:
                    if tag == 'gb': gb = p
            lastD = max([p['D'] for p in Ds if 'Q' in p], default=None)
            colsD = next((p['cols'] for p in Ds if p.get('D') == dsolv and 'cols' in p), None)
            inst_recs.append(dict(inst=i, arm=arm, status=r['status'], done=done is not None, fail=fail, info=info,
                                  d_ff=dff, d_solv=dsolv, d_solv_method=dsolv_method, last_D_completed=lastD, cols_at_solv=colsD,
                                  profile=[p for p in Ds if 'Q' in p], gb=gb,
                                  time=(done or {}).get('time')))
        # aggregate
        dffs = [r['d_ff'] for r in inst_recs]; dsolvs = [r['d_solv'] for r in inst_recs]
        planted = [r for r in inst_recs if r['arm'] == 'planted']
        pl_ok = []
        for r in planted:
            inn = r['info'].get('planted_in_solutions')
            gbp = (r['gb'] or {}).get('planted_in_variety')
            if inn is not None or gbp is not None:
                pl_ok.append(bool((inn is None or inn) and (gbp is None or gbp) and r['info'].get('planted_residual', 1) == 0))
        resid = [r['info'].get('planted_residual') for r in planted if r['info'].get('planted_residual') is not None]
        ff_ok = [x for x in dffs if x is not None]
        solv_ok = [x for x in dsolvs if x is not None]
        cens_ff = len(ff_ok) < max(1, (len(dffs) + 1) // 2)
        cens_solv = len(solv_ok) < max(1, (len(dsolvs) + 1) // 2)
        lastDs = [r['last_D_completed'] for r in inst_recs if r['last_D_completed'] is not None]
        cell = dict(n=n, l=l, nvars=3 * l, n_instances=len(inst_recs),
                    planted_solved=(bool(pl_ok) and all(pl_ok)), planted_checked=len(pl_ok),
                    d_first_fall=med(dffs), d_solving=med(dsolvs),
                    d_ff_all=dffs, d_solv_all=dsolvs,
                    macaulay_cols_at_solv=med([r['cols_at_solv'] for r in inst_recs]),
                    time_s=time.time() - tstart, budget_s=bud,
                    max_D_completed=(max(lastDs) if lastDs else None),
                    instances=inst_recs)
        if cens_ff or cens_solv:
            cell['censored'] = True
            cell['censored_what'] = ('d_ff+d_solv' if cens_ff and cens_solv else ('d_ff' if cens_ff else 'd_solv'))
            if cens_solv and lastDs:
                cell['d_solv_lower_bound'] = max(lastDs) + 1
        res['cells'][str(n)] = cell
        if not cens_ff and cell['d_first_fall'] is not None: res['d_ff'][str(n)] = cell['d_first_fall']
        if not cens_solv and cell['d_solving'] is not None: res['d_solv'][str(n)] = cell['d_solving']
        res['validation']['planted_solved_per_n'][str(n)] = cell['planted_solved']
        if resid: res['validation'].setdefault('planted_residual_by_n', {})[str(n)] = resid
        print('CELL', n, 'd_ff', cell['d_first_fall'], dffs, 'd_solv', cell['d_solving'], dsolvs, 'planted_solved', cell['planted_solved'], 'time', round(cell['time_s']), flush=True)
        write_result(res)
        if cens_ff:
            res['notes'].append('d_ff not resolvable at n=%d within budget; stopping' % n)
            break
    # deep pass: full Macaulay profile past d_ff (solving-degree evidence) on one instance per arm at n=11,
    # one planted instance at n=13,17,19 (row cap 9000 -> lower bounds only)
    left = GLOBAL_DEADLINE - (time.time() - T0)
    res['deep'] = {}
    if left > 400:
        try:
            dl = time.time() + min(2400, left - 200)
            dtasks = [((n_, 0, a, 'deep'), worker_mac, (n_, 0, a, True)) for n_, a in
                      ((11, 'planted'), (11, 'random'), (13, 'planted'), (13, 'random'), (17, 'planted'), (19, 'planted'))
                      if str(n_) in res['cells']]
            rs = run_pool(dtasks, dl)
            for (n_, i_, arm, kind), r in rs.items():
                info = {}; Ds = []; done = None
                for tag, p in r['msgs']:
                    if tag == 'info': info = p
                    elif tag == 'D': Ds.append(p)
                    elif tag == 'done': done = p
                prof = [p for p in Ds if 'Q' in p]
                sol = info.get('n_solutions')
                lastD = max([p['D'] for p in prof], default=None)
                res['deep'].setdefault(str(n_), {})[arm] = dict(
                    N=info.get('N'), n_solutions=sol, profile=prof, status=r['status'],
                    stop_notes=[p for p in Ds if 'Q' not in p], finished=done is not None,
                    d_ff=(done or {}).get('d_ff'), d_solv=(done or {}).get('d_solv'),
                    max_D_completed=lastD, Q_at_max_D=(prof[-1]['Q'] if prof else None),
                    d_solv_lower_bound=((lastD + 1) if (lastD is not None and (done or {}).get('d_solv') is None) else None))
            for n_s, arms in res['deep'].items():
                sv = [v['d_solv'] for v in arms.values() if v['d_solv'] is not None]
                c = res['cells'][n_s]
                if sv:
                    res['d_solv'][n_s] = float(np.median(sv)); c['d_solving'] = float(np.median(sv))
                    c['d_solv_method'] = 'macaulay_Q=s (deep pass, 1 instance/arm)'
                    c['macaulay_cols_at_solv'] = sum(math.comb(c['nvars'], k) for k in range(int(np.median(sv)) + 1))
                else:
                    lbs = [v['d_solv_lower_bound'] for v in arms.values() if v['d_solv_lower_bound']]
                    if lbs: c['d_solv_lower_bound'] = max(lbs); c['censored'] = True
        except Exception:
            res['notes'].append('deep pass error: ' + traceback.format_exc())
    finalize(res)

def finalize(res):
    ns = sorted(int(k) for k in res['d_ff'])
    fit = {}
    if len(ns) >= 2:
        ys = [res['d_ff'][str(k)] for k in ns]
        lin = ols(ns, ys); lg = ols([math.log(k) for k in ns], ys)
        fit['d_ff_slope'] = lin['b']; fit['d_ff_intercept'] = lin['a']; fit['d_ff_slope_95ci'] = lin['ci']
        fit['d_ff_points'] = dict(zip(map(str, ns), ys))
        e_ff = lin['a'] + lin['b'] * 131
        fit['extrapolated_d_ff_131'] = e_ff
        if lin['ci'] is not None:
            # band: intercept refit through centroid
            xb = np.mean(ns); yb = np.mean(ys)
            fit['extrapolated_d_ff_131_band'] = [yb + lin['ci'][0] * (131 - xb), yb + lin['ci'][1] * (131 - xb)]
        fit['alt_log_fit'] = dict(a=lg['a'], b=lg['b'], b_95ci=lg['ci'], extrapolated_d_ff_131=lg['a'] + lg['b'] * math.log(131))
        # solving degree
        ds = sorted(int(k) for k in res['d_solv'])
        if len(ds) >= 3:
            sl = ols(ds, [res['d_solv'][str(k)] for k in ds])
            e_s = sl['a'] + sl['b'] * 131
            fit['d_solv_fit'] = dict(slope=sl['b'], intercept=sl['a'], slope_95ci=sl['ci'], points=dict(zip(map(str, ds), [res['d_solv'][str(k)] for k in ds])))
            fit['d_solv_method'] = 'linear fit of measured d_solv'
        elif ds:
            off = np.mean([res['d_solv'][str(k)] - res['d_ff'][str(k)] for k in ds if str(k) in res['d_ff']] or [0])
            e_s = e_ff + off
            fit['d_solv_method'] = 'd_ff extrapolation + mean(d_solv-d_ff) offset (fewer than 3 d_solv points)'
        else:
            e_s = e_ff
            fit['d_solv_method'] = 'no d_solv measured; d_solv>=d_ff so d_ff-based cost is a LOWER bound'
        e_s = max(e_s, e_ff)
        fit['extrapolated_d_solv_131'] = e_s
        bits = cost_bits(e_s)
        fit['extrapolated_solve_cost_bits_131'] = bits
        fit['cost_bits_at_d_ff_131_lower_bound'] = cost_bits(e_ff)
        fit['cost_bits_if_d_ff_slope_zero_at_last_n'] = cost_bits(ys[-1])
        fit['N_vars_131'] = 132; fit['omega'] = OMEGA
        fit['gap_bits_to_2e21'] = bits - 21
        fit['gap_bits_to_rho_2e65'] = bits - 65
        fit['gap_bits_lower_bound_to_2e21'] = cost_bits(e_ff) - 21
        if lin['ci'] is not None:
            lo = cost_bits(max(fit['extrapolated_d_ff_131_band'][0], min(ys)))
            fit['cost_bits_131_at_lower_band_d_ff'] = lo
        slope_pos = lin['ci'] is not None and lin['ci'][0] > 0
        if slope_pos and fit['extrapolated_solve_cost_bits_131'] > 21: verdict = 'kill'
        elif lin['b'] <= 0 and bits < 21: verdict = 'win'
        elif (lin['ci'] is not None and lin['ci'][0] <= 0 <= lin['ci'][1]) and bits < 21: verdict = 'win'
        else: verdict = 'inconclusive'
    else:
        verdict = 'inconclusive'; fit['note'] = 'fewer than 2 resolved d_ff points'
    res['fit'] = fit; res['verdict'] = verdict
    res['validation']['planted_solved_all_completed'] = all(res['validation']['planted_solved_per_n'].values()) if res['validation']['planted_solved_per_n'] else False
    res['total_time_s'] = time.time() - T0
    write_result(res)
    print(json.dumps({k: res[k] for k in ('d_ff', 'd_solv', 'fit', 'validation', 'verdict')}, indent=1, default=str))

RES = None
import threading
def _wd():
    try:
        if RES is not None:
            RES.setdefault('notes', []).append('watchdog fired: finalizing partial results')
            finalize(RES)
    finally:
        sys.stdout.flush(); os._exit(0)
_tm = threading.Timer(6900, _wd); _tm.daemon = True; _tm.start()
try:
    main()
except Exception:
    err = traceback.format_exc()
    print(err)
    try:
        r = json.load(open(os.path.join(OUT, 'result.json')))
    except Exception:
        r = dict(cells={}, d_ff={}, d_solv={}, notes=[])
    r.setdefault('notes', []).append('FATAL ' + err)
    r.setdefault('fit', {}); r.setdefault('validation', {}); r['verdict'] = 'inconclusive'
    write_result(r)
