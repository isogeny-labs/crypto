"""IC relation phase and linear algebra (SPEC sections 7, 8)."""
import hashlib, heapq, time
import ff, jac, aut
from ff import CTR, uncounted, rmul, rinv, fmul
from poly import *
from seeds import rng

M64 = (1 << 64) - 1

def make_instance(C, s):
    with uncounted():
        ff.set_p(C.p)
        rnd = rng("inst:%s:%d" % (C.curve_id, s))
        while True:
            D = jac.random_divisor(C, rnd)
            G = jac.smul(C.h, D, C)
            if not jac.is_ident(G):
                break
        t = rnd.randrange(1, C.r)
        T = jac.smul(t, G, C)
    return G, T, t

def solve(rows, tcs, rhs, r, certify):
    """Returns (t or None, stats). Counted with rmul/rinv."""
    rows = [dict(x) for x in rows]
    tc = list(tcs); rh = list(rhs)
    col_rows = {}
    for i, row in enumerate(rows):
        for c in row:
            col_rows.setdefault(c, set()).add(i)
    dim = len(col_rows)
    nnz = sum(len(x) for x in rows)
    heap = [(len(s), c) for c, s in col_rows.items()]
    heapq.heapify(heap)
    active = set(range(len(rows)))
    max_fill = max((len(x) for x in rows), default=0)
    while heap:
        n, c = heapq.heappop(heap)
        cr = col_rows.get(c)
        if cr is None or len(cr) != n or n == 0:
            continue
        piv = min(cr, key=lambda i: (len(rows[i]), i))
        others = sorted(cr - {piv})
        prow = rows[piv]
        if others:
            inv = rinv(prow[c], r)
        for R in others:
            rowR = rows[R]
            fac = rmul(rowR[c], inv, r)
            for j, val in prow.items():
                if j == c:
                    continue
                new = (rowR.get(j, 0) - rmul(fac, val, r)) % r
                if new:
                    if j not in rowR:
                        col_rows[j].add(R)
                    rowR[j] = new
                else:
                    if j in rowR:
                        del rowR[j]
                        col_rows[j].discard(R)
                heapq.heappush(heap, (len(col_rows[j]), j))
            del rowR[c]
            cr.discard(R)
            tc[R] = (tc[R] - rmul(fac, tc[piv], r)) % r
            rh[R] = (rh[R] - rmul(fac, rh[piv], r)) % r
            if len(rowR) > max_fill:
                max_fill = len(rowR)
        for j in prow:
            if j == c:
                continue
            col_rows[j].discard(piv)
            heapq.heappush(heap, (len(col_rows[j]), j))
        active.discard(piv)
        del col_rows[c]
    stats = {"dim": dim, "nnz": nnz, "max_fill": max_fill}
    for i in sorted(active):
        assert not rows[i], "residual columns in active row"
        if tc[i] != 0:
            t = rmul(rh[i], rinv(tc[i], r), r)
            if certify(t):
                return t, stats
    return None, stats

def pow_x_mod(base, e, u):
    res = base
    for bit in bin(e)[3:]:
        res = pdivmod(pmul(res, res), u)[1]
        if bit == '1':
            res = pdivmod(pmul(res, base), u)[1]
    return res

def x_pow_p_mod(u, p):
    res = [0, 1]
    for bit in bin(p)[3:]:
        res = pdivmod(pmul(res, res), u)[1]
        if bit == '1':
            res = pdivmod([0] + res, u)[1]
    return res

def cz_roots(u, z, p):
    if len(u) - 1 == 1:
        return [(-u[0]) % p]
    while True:
        delta = z.randrange(p)
        hp = pow_x_mod([delta, 1], (p - 1) // 2, u)
        g1 = pmonic(xgcd(u, psub(hp, [1]))[0])
        dg = len(g1) - 1
        if 0 < dg < len(u) - 1:
            return cz_roots(g1, z, p) + cz_roots(pdivmod(u, g1)[0], z, p)

def split_points(W, C, z):
    """returns list of (x, y) if W = sum of g distinct affine points with y != 0, else None"""
    u, v = W
    p = C.p
    if len(u) - 1 != C.g:
        return None
    if x_pow_p_mod(u, p) != [0, 1]:
        return None
    roots = cz_roots(u, z, p)
    pts = []
    for xi in roots:
        yi = peval(v, xi)
        if yi == 0:
            return None
        pts.append((xi, yi))
    return pts

def run_ic(C, s, arm):
    t0 = time.time()
    ff.set_p(C.p)
    p, r, g = C.p, C.r, C.g
    G, T, t_pl = make_instance(C, s)
    CTR.reset()
    jac.OPS[0] = 0
    if arm == "A0":
        ncols = 2 * C.nA
    elif arm == "A":
        ncols = C.nA
    else:
        ncols = C.nB
    target = ncols + 10
    w = rng("icwalk:%s:%d" % (C.curve_id, s))
    z = rng("cz:%s:%d" % (C.curve_id, s))
    half = (p - 1) // 2
    cm = C.aq > 1

    def certify(t):
        with uncounted():
            return t == t_pl and jac.smul_rl(t, G, C) == T

    jumps = []
    for j in range(16):
        al = w.randrange(r); be = w.randrange(r)
        Rj = jac.add(jac.smul(al, G, C), jac.smul(be, T, C), C)
        jumps.append((al, be, Rj))
    a = w.randrange(r); b = w.randrange(r)
    W = jac.add(jac.smul(a, G, C), jac.smul(b, T, C), C)
    rows = []; tcs = []; rhs = []
    trials = 0; restarts = 0
    hd = hashlib.sha256(); digests = []
    attempts = 0; need = target
    t_found = None; status = "la_fail"
    last_stats = {"dim": 0, "nnz": 0, "max_fill": 0}
    trial_cap = 50 * target + 20000
    while True:
        j = ((jac.K(W, g, p) * 0x9E3779B97F4A7C15) & M64) >> 60
        al, be, Rj = jumps[j]
        W = jac.add(W, Rj, C)
        a = (a + al) % r; b = (b + be) % r
        trials += 1
        hd.update(str(jac.K(W, g, p)).encode() + b";")
        if trials % 1000 == 0:
            digests.append(hd.hexdigest()[:16])
        pts = split_points(W, C, z)
        if pts is not None:
            row = {}
            for (xi, yi) in pts:
                if arm == "A0":
                    col = xi * p + yi; co = 1
                elif arm == "A":
                    col = xi; co = 1 if yi <= half else r - 1
                else:
                    if cm and xi == 0:
                        continue
                    xs, k = aut.orbit_rep_point(xi, C)
                    sg = 1 if yi <= half else -1
                    col = xs; co = (sg * C.LP[k]) % r
                row[col] = (row.get(col, 0) + co) % r
                if row[col] == 0:
                    del row[col]
            rows.append(row); tcs.append((-b) % r); rhs.append(a % r)
        if trials % 4096 == 0:
            a = w.randrange(r); b = w.randrange(r)
            W = jac.add(jac.smul(a, G, C), jac.smul(b, T, C), C)
            restarts += 1
        if len(rows) >= need:
            attempts += 1
            tf, last_stats = solve(rows, tcs, rhs, r, certify)
            if tf is not None:
                t_found = tf; status = "ok"; break
            if attempts >= 6:
                status = "la_fail"; break
            need = len(rows) + 10
        if trials > trial_cap:
            status = "trial_cap"; break
    with uncounted():
        certified = (t_found is not None and t_found == t_pl and jac.smul_rl(t_found, G, C) == T)
    return {
        "run_id": "%s|%s|%d" % (C.curve_id, arm, s), "family": C.family, "curve_id": C.curve_id,
        "bits": C.bits, "p": p, "g": g, "q": C.q, "h": C.h, "r": r, "lam": C.lam, "seed": s,
        "arm": arm, "ncols": ncols, "nA": C.nA, "delta0": C.delta0, "nB": C.nB,
        "trials": trials, "walk_restarts": restarts, "relations_collected": len(rows),
        "relations_target": target, "la_attempts": attempts, "grp_ops": jac.OPS[0],
        "rel_mul": CTR.mul, "rel_inv": CTR.inv, "la_mul": CTR.rmul, "la_inv": CTR.rinv,
        "dim": last_stats["dim"], "nnz": last_stats["nnz"], "max_fill": last_stats["max_fill"],
        "t_found": t_found, "t_planted": t_pl, "certified": bool(certified),
        "digests": digests, "final_digest": hd.hexdigest()[:16], "status": status,
        "wall_s": round(time.time() - t0, 3)}
