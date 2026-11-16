import math
import time
from .field import Counter, F
from .rng import stream, mix64
from . import native, verify
from .rows import Acc, DSU, canon


class GateFailure(Exception):
    pass


class Budget(Exception):
    pass


def interp_mul(g):
    return 10 if g == 2 else 27


def fd_init_mul(g):
    return (g + 1) * g + g * (g - 1)


def interpolate(Fq, g, anchors):
    xs = [x for x, _ in anchors]
    ys = [y for _, y in anchors]
    Pi = [Fq.neg(xs[0]), 1]
    for i in range(1, g):
        xi = xs[i]
        k = len(Pi) - 1
        new = [0] * (k + 2)
        new[0] = Fq.neg(Fq.mul(xi, Pi[0]))
        for j in range(1, k):
            new[j] = Fq.sub(Pi[j - 1], Fq.mul(xi, Pi[j]))
        new[k] = Fq.sub(Pi[k - 1], xi)
        new[k + 1] = 1
        Pi = new
    w = []
    for i in range(g):
        acc = None
        for j in range(g):
            if j == i:
                continue
            d = Fq.sub(xs[i], xs[j])
            acc = d if acc is None else Fq.mul(acc, d)
        w.append(acc)
    winv = Fq.batch_inv(w)
    c = [Fq.mul(ys[i], winv[i]) for i in range(g)]
    a0 = [0] * g
    for i in range(g):
        q = [0] * g
        q[g - 1] = 1
        for k in range(g - 1, 0, -1):
            q[k - 1] = Fq.add(Pi[k], Fq.mul(xs[i], q[k]))
        for j in range(g - 1):
            a0[j] = Fq.add(a0[j], Fq.mul(c[i], q[j]))
        a0[g - 1] = Fq.add(a0[g - 1], c[i])
    return a0, Pi


class FD:
    def __init__(self, Fq, c):
        self.p = Fq.p
        self.ctr = Fq.ctr
        d = len(c) - 1
        self.d = d
        vals = [Fq.horner(c, k) for k in range(d + 1)]
        D = []
        cur = vals
        for i in range(d + 1):
            D.append(cur[0])
            cur = [(cur[j + 1] - cur[j]) % self.p for j in range(len(cur) - 1)]
            Fq.ctr.add += len(cur)
        self.D = D

    def step(self):
        D = self.D
        p = self.p
        for i in range(self.d):
            D[i] = (D[i] + D[i + 1]) % p
        self.ctr.fd_add += self.d


class AnchorGen:
    def __init__(self, T, g, rng, idxlist=None):
        self.T, self.g, self.rng = T, g, rng
        self.idx = list(range(T.n)) if idxlist is None else list(idxlist)
        self.perms = 0
        self._new_perm()

    def _new_perm(self):
        perm = list(self.idx)
        for i in range(len(perm) - 1, 0, -1):
            j = self.rng.below(i + 1)
            perm[i], perm[j] = perm[j], perm[i]
        self.perm = perm
        self.pos = 0
        self.perms += 1

    def next_tuple(self, max_perms=None):
        g = self.g
        if self.pos + g > len(self.perm):
            if max_perms is not None and self.perms >= max_perms:
                return None
            self._new_perm()
        T = self.T
        p = T.p
        out = []
        for k in range(g):
            x = T.B[self.perm[self.pos + k]]
            s = T.sq[T.fx[x]]
            y = s if self.rng.bit() == 0 else p - s
            out.append((x, y))
        self.pos += g
        return out


def sieve_tuple(T, Fq, g, anchors, allowed, xmax, want_cnt):
    p = T.p
    ctr = Fq.ctr
    m0, i0 = ctr.mul, ctr.inv
    a0, Pi = interpolate(Fq, g, anchors)
    A = {x for x, _ in anchors}
    fdP, fdA = FD(Fq, Pi), FD(Fq, a0)
    X = []
    DP, DA = fdP.D, fdA.D
    for x in range(xmax + 1):
        if allowed[x] and x not in A:
            X.append((x, DP[0], DA[0]))
        if x < xmax:
            fdP.step()
            fdA.step()
    Dinv = Fq.batch_inv([t[1] for t in X]) if X else []
    cnt = [0] * p if want_cnt else None
    pts = {}
    fx, sq = T.fx, T.sq
    npts = 0
    mul = Fq.mul
    sub = Fq.sub
    for (x, Pix, a0x), di in zip(X, Dinv):
        if fx[x] != 0:
            s = sq[fx[x]]
            plist = ((x, s), (x, p - s))
        else:
            plist = ((x, 0),)
        for (xx, y) in plist:
            lam = mul(sub(y, a0x), di)
            npts += 1
            ctr.bucket_ops += 1
            if want_cnt:
                cnt[lam] += 1
                c = cnt[lam]
            lst = pts.get(lam)
            if lst is None:
                pts[lam] = [(xx, y)]
                c = 1
            else:
                lst.append((xx, y))
                c = len(lst)
            if c > g + 1:
                raise GateFailure("G5 bucket")
    info = {"nX": len(X), "npts": npts, "a0": a0, "Pi": Pi,
            "mul": ctr.mul - m0, "inv": ctr.inv - i0}
    return info, pts, cnt


def sample_v2(i, p, N_target):
    if p <= 4099:
        return True
    return (mix64(i ^ 0x5632) % 1000) < 1000 * 200 / N_target


def p_full(T, g, cs, ss, N_target, vc, budget_check=None, max_perms=50):
    p = T.p
    ctr = Counter()
    Fq = F(p, ctr)
    acc = Acc(T)
    rng = stream(0x5046, g, p, cs, ss)
    ag = AnchorGen(T, g, rng)
    allowed = [False] * p
    for x in T.B:
        allowed[x] = True
    for x in T.Z:
        allowed[x] = True
    per_tuple = []
    stat = {"v1_checked": 0, "v1_failed": 0, "v2_checked": 0, "v2_failed": 0, "max_bucket": 0}
    cands_total = 0
    t = 0
    first_cands = None
    status = "ok"
    while True:
        if budget_check:
            budget_check()
        anchors = ag.next_tuple(max_perms)
        if anchors is None:
            status = "anchors_exhausted"
            break
        info, pts, cnt = sieve_tuple(T, Fq, g, anchors, allowed, p - 1, True)
        ctr.bucket_scan += p
        # count-formula gate (7.4)
        expect = interp_mul(g) + fd_init_mul(g) + 3 * (info["nX"] - 1) + info["npts"]
        if info["mul"] != expect or info["inv"] != 2:
            raise GateFailure("count formula: mul %d expected %d inv %d" % (info["mul"], expect, info["inv"]))
        cands = [lam for lam in range(p) if cnt[lam] == g + 1]
        stat["max_bucket"] = max(stat["max_bucket"], max(cnt))
        new = 0
        cl = []
        for lam in cands:
            points = list(anchors) + sorted(pts[lam])
            cl.append((lam, tuple(points)))
            stat["v1_checked"] += 1
            if not verify.v1_pencil(p, T.f, g, info["a0"], info["Pi"], lam, points):
                stat["v1_failed"] += 1
            if acc.add(points):
                new += 1
                i = acc.count - 1
                if sample_v2(i, p, N_target):
                    stat["v2_checked"] += 1
                    if verify.v2_sum(vc, points) != ((1,), ()):
                        stat["v2_failed"] += 1
        if t == 0:
            first_cands = set(cl)
        cands_total += len(cands)
        per_tuple.append({"t": t, "mul": info["mul"], "inv": info["inv"], "cands": len(cands),
                          "new": new, "nX": info["nX"], "npts": info["npts"]})
        t += 1
        if acc.count >= N_target:
            break
    ratio = (cands_total / (t * p / math.factorial(g + 1))) if t else 0.0
    return {"ctr": ctr, "acc": acc, "per_tuple": per_tuple, "tuples": t, "stat": stat,
            "first_cands": first_cands, "first_anchors": None, "split_model_ratio": ratio,
            "status": status}


def first_anchors(T, g, cs, ss):
    rng = stream(0x5046, g, T.p, cs, ss)
    return AnchorGen(T, g, rng).next_tuple()


def p_naive1(T, g, cs, ss, anchors, budget_check=None):
    p = T.p
    f = T.f
    ctr = Counter()
    Fq = F(p, ctr)
    rng = stream(0x504E, g, p, cs, ss)
    a0, Pi = interpolate(Fq, g, anchors)
    A = {x for x, _ in anchors}
    out = set()
    root_failures = 0
    for lam in range(p):
        if lam % 1024 == 0 and budget_check:
            budget_check()
        a = [Fq.add(a0[k], Fq.mul(lam, Pi[k])) for k in range(g)] + [lam]
        sqa = [0] * (2 * g + 1)
        for i in range(g + 1):
            for j in range(g + 1):
                sqa[i + j] = Fq.add(sqa[i + j], Fq.mul(a[i], a[j]))
        N = [Fq.sub(f[i], sqa[i]) for i in range(2 * g + 1)] + [f[2 * g + 1]]
        r = list(N)
        R = [0] * (g + 2)
        for top in range(2 * g + 1, g - 1, -1):
            c = r[top]
            R[top - g] = c
            for j in range(g):
                r[top - g + j] = Fq.sub(r[top - g + j], Fq.mul(c, Pi[j]))
        if any(r[:g]):
            raise GateFailure("naive: nonzero remainder")
        assert R[g + 1] == 1
        ch = native.disc_character(p, R, T.chi, ctr)
        if ch != 1:
            continue
        if not native.splits_completely(p, R, ctr):
            continue
        roots = native.roots_of(p, R, rng, ctr)
        if roots is None:
            root_failures += 1
            continue
        if len(set(roots)) != g + 1 or any(x in A for x in roots):
            continue
        points = sorted((x, Fq.horner(a, x)) for x in roots)
        out.add((lam, tuple(list(anchors) + points)))
    return {"ctr": ctr, "relset": out, "root_failures": root_failures}


def synth_div(Fq, Q, xj):
    e = len(Q) - 1
    q = [0] * e
    q[e - 1] = 1
    for k in range(e - 1, 0, -1):
        q[k - 1] = Fq.add(Q[k], Fq.mul(xj, q[k]))
    rem = Fq.add(Q[0], Fq.mul(xj, q[0]))
    if rem != 0:
        raise GateFailure("G5 synthetic division remainder")
    return q


def p_2lp(T, g, cs, ss, vc, nb=None, tuples_override=None, keep=False, budget_check=None,
          fpm_cap=2e9, max_perms=50):
    import time as _t
    p = T.p
    f = T.f
    n = T.n
    if nb is None:
        nb = min(n, math.ceil(2.0 * p ** (1.0 - 1.0 / g)))
    S = T.B[:nb]
    sset = {x: i for i, x in enumerate(S)}
    target = nb + 8 + nb // 100
    ctr = Counter()
    Fq = F(p, ctr)
    rng = stream(0x5032, g, p, cs, ss)
    ag = AnchorGen(T, g, rng, idxlist=range(nb))
    allowed = [False] * p
    for x in S:
        allowed[x] = True
    xmax = S[-1]
    seen = set()
    dsu = DSU()
    res = {"relations": 0, "duplicates": 0, "candidates": 0, "split_rejects": 0,
           "anchor_rejects": 0, "lp_rejects": 0, "repeat_rejects": 0,
           "rels_by_lp": {"0": 0, "1": 0, "2": 0}, "v1_checked": 0, "v1_failed": 0,
           "v2_checked": 0, "v2_failed": 0, "tuples": 0, "max_bucket": 0}
    kept = []
    status = "ok"
    tcount = 0
    while True:
        if budget_check:
            budget_check()
        if tuples_override is not None:
            if tcount >= len(tuples_override):
                status = "tuples_done"
                break
            anchors = tuples_override[tcount]
        else:
            anchors = ag.next_tuple(max_perms)
            if anchors is None:
                status = "anchors_exhausted"
                break
        tcount += 1
        info, pts, _ = sieve_tuple(T, Fq, g, anchors, allowed, xmax, False)
        a0, Pi = info["a0"], info["Pi"]
        A = {x for x, _ in anchors}
        res["tuples"] += 1
        stop = False
        for lam in sorted(l for l, v in pts.items() if len(v) >= g - 1):
            bucket = sorted(pts[lam])
            k = len(bucket)
            res["max_bucket"] = max(res["max_bucket"], k)
            res["candidates"] += 1
            a = [Fq.add(a0[j], Fq.mul(lam, Pi[j])) for j in range(g)] + [lam]
            sqa = [0] * (2 * g + 1)
            for i in range(g + 1):
                for j in range(g + 1):
                    sqa[i + j] = Fq.add(sqa[i + j], Fq.mul(a[i], a[j]))
            N = [Fq.sub(f[i], sqa[i]) for i in range(2 * g + 1)] + [f[2 * g + 1]]
            r = list(N)
            Q = [0] * (g + 2)
            for top in range(2 * g + 1, g - 1, -1):
                c = r[top]
                Q[top - g] = c
                for j in range(g):
                    r[top - g + j] = Fq.sub(r[top - g + j], Fq.mul(c, Pi[j]))
            if any(r[:g]):
                raise GateFailure("G5 pencil remainder")
            for (xj, _) in bucket:
                Q = synth_div(Fq, Q, xj)
            e = g + 1 - k
            if e == 0:
                roots = []
            elif e == 1:
                roots = [Fq.neg(Q[0])]
            else:
                disc = Fq.sub(Fq.mul(Q[1], Q[1]), Fq.add(Fq.add(Q[0], Q[0]), Fq.add(Q[0], Q[0])))
                if disc == 0 or T.chi[disc] != 1:
                    res["split_rejects"] += 1
                    continue
                s = T.sq[disc]
                m1 = Fq.neg(Q[1])
                roots = [Fq.mul(Fq.add(m1, s), T.inv2), Fq.mul(Fq.sub(m1, s), T.inv2)]
            bxs = {xj for xj, _ in bucket}
            if any(xr in A for xr in roots):
                res["anchor_rejects"] += 1
                continue
            if any(xr in bxs for xr in roots):
                # repeated root of the residual at a bucket abscissa (ramified fibre)
                res["repeat_rejects"] += 1
                continue
            if any(xr in sset for xr in roots):
                raise GateFailure("G5 S-root")
            rpts = [(xr, Fq.horner(a, xr)) for xr in roots]
            lp = [(x, y) for x, y in rpts if y != 0]
            points = list(anchors) + bucket + rpts
            res["v1_checked"] += 1
            if not verify.v1_pencil(p, f, g, a0, Pi, lam, points):
                res["v1_failed"] += 1
            key = canon(points, p)
            if key in seen:
                res["duplicates"] += 1
                continue
            seen.add(key)
            if sample_v2(res["relations"], p, target):
                res["v2_checked"] += 1
                if verify.v2_sum(vc, points) != ((1,), ()):
                    res["v2_failed"] += 1
            res["relations"] += 1
            res["rels_by_lp"][str(len(lp))] += 1
            if len(lp) == 0:
                dsu.edge(0, 0)
            elif len(lp) == 1:
                dsu.edge(0, dsu.vid(lp[0][0]))
            else:
                dsu.edge(dsu.vid(lp[0][0]), dsu.vid(lp[1][0]))
            if keep:
                kept.append((lam, len(lp), tuple(sorted(points))))
            if dsu.cycles >= target or ctr.fpm() >= fpm_cap:
                stop = True
                break
        if stop:
            if dsu.cycles < target:
                status = "exhausted"
            break
        if ctr.fpm() >= fpm_cap:
            status = "exhausted"
            break
    res.update({"ctr": ctr, "nb": nb, "target_cycles": target, "cycles": dsu.cycles,
                "status": status, "kept": kept})
    return res
