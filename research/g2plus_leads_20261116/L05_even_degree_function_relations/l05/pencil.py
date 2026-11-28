import math
import time
from .field import Counter, F, GateFailure, horner_F, batch_inv, inv_table
from .rng import SM64, stream_seed, TAGS, mix64
from .rows import canon_key, digest, RowStats
from . import verify as V


def arm_spec(name, g):
    S = {
        "O":      dict(model="odd", e=None, j=g, R=g + 1, inv="batch", ip=None, im=None, dc2=0),
        "O_T":    dict(model="odd", e=None, j=g, R=g + 1, inv="table", ip=None, im=None, dc2=0),
        "E_0":    dict(model="even", e=0, j=g, R=g + 1, inv="batch", ip=-g, im=-(g + 1), dc2=1),
        "E_0_T":  dict(model="even", e=0, j=g, R=g + 1, inv="table", ip=-g, im=-(g + 1), dc2=1),
        "E_low":  dict(model="even", e=None, j=g, R=g + 2, inv="batch", ip=-(g + 1), im=-(g + 1), dc2=0),
        "E_1a":   dict(model="even", e=g - 1, j=1, R=g + 1, inv="batch", ip=-1, im=-(g + 1), dc2=g),
        "E_1a_T": dict(model="even", e=g - 1, j=1, R=g + 1, inv="table", ip=-1, im=-(g + 1), dc2=g),
        "E_g":    dict(model="even", e=g, j=0, R=g + 1, inv="none", ip=0, im=-(g + 1), dc2=g + 1),
    }
    d = dict(S[name])
    d["name"] = name
    return d


ARM_NAMES = ["O", "O_T", "E_0", "E_0_T", "E_low", "E_1a", "E_1a_T", "E_g"]
_IM = {0: 0, 1: 0, 2: 10, 3: 27}


def interp_mul(j, has_t, g):
    return _IM[j] + (j * (g + 1) if has_t else 0)


def interp_inv(j):
    return 1 if j >= 2 else 0


def interpolate(F_, anchors, t, g):
    j = len(anchors)
    xs = [x for x, _ in anchors]
    if t is not None:
        z = [F_.sub(y, horner_F(F_, t, x)) for x, y in anchors]
    else:
        z = [y for _, y in anchors]
    if j == 0:
        Pi, b0 = [1], []
    elif j == 1:
        Pi, b0 = [F_.neg(xs[0]), 1], [z[0]]
    else:
        Pi = [F_.neg(xs[0]), 1]
        for i in range(1, j):
            xi = xs[i]
            k = len(Pi) - 1
            new = [0] * (k + 2)
            new[0] = F_.neg(F_.mul(xi, Pi[0]))
            for m in range(1, k):
                new[m] = F_.sub(Pi[m - 1], F_.mul(xi, Pi[m]))
            new[k] = F_.sub(Pi[k - 1], xi)
            new[k + 1] = 1
            Pi = new
        w = []
        for i in range(j):
            acc = None
            for l in range(j):
                if l == i:
                    continue
                dd = F_.sub(xs[i], xs[l])
                acc = dd if acc is None else F_.mul(acc, dd)
            w.append(acc)
        winv = batch_inv(F_, w)
        cc = [F_.mul(z[i], winv[i]) for i in range(j)]
        b0 = [0] * j
        for i in range(j):
            q = [0] * j
            q[j - 1] = 1
            for m in range(j - 1, 0, -1):
                q[m - 1] = F_.add(Pi[m], F_.mul(xs[i], q[m]))
            for m in range(j - 1):
                b0[m] = F_.add(b0[m], F_.mul(cc[i], q[m]))
            b0[j - 1] = F_.add(b0[j - 1], cc[i])
    if t is not None:
        c = [F_.add(t[m], b0[m] if m < len(b0) else 0) for m in range(g + 2)]
    else:
        c = b0
    return c, Pi


class FD:
    def __init__(self, F_, cl):
        self.F = F_
        self.p = F_.p
        if not cl:
            cl = [0]
        d = len(cl) - 1
        self.d = d
        D = [horner_F(F_, cl, k) for k in range(d + 1)]
        for lev in range(1, d + 1):
            for i in range(d, lev - 1, -1):
                D[i] = F_.sub(D[i], D[i - 1])
        self.D = D

    def value(self):
        return self.D[0]

    def step(self):
        D, p, d = self.D, self.p, self.d
        for i in range(d):
            D[i] = (D[i] + D[i + 1]) % p
        self.F.ctr.fd_add += d


def sieve_pencil(spec, g, p, tab, anchors, F_, inv_tab, t_poly):
    ctr = F_.ctr
    snap = ctr.snap()
    j, R, mode = spec["j"], spec["R"], spec["inv"]
    has_t = t_poly is not None
    c, Pi = interpolate(F_, anchors, t_poly, g)
    A = {x for x, _ in anchors}
    fdC = FD(F_, c)
    fdP = FD(F_, Pi) if (mode == "batch" and j >= 1) else None
    X = []
    col = tab.col
    for x in range(p):
        if col[x] >= 0 and x not in A:
            X.append((x, fdC.value(), fdP.value() if fdP else None))
        if x < p - 1:
            fdC.step()
            if fdP:
                fdP.step()
    k = len(X)
    dinv = None
    if mode == "batch" and j >= 1:
        dinv = batch_inv(F_, [v[2] for v in X]) if k else []
    elif mode == "table":
        dinv = []
        for (x, _, _) in X:
            di = inv_tab[F_.sub(x, anchors[0][0])]
            for i in range(1, j):
                di = F_.mul(di, inv_tab[F_.sub(x, anchors[i][0])])
            dinv.append(di)
    cnt = [0] * p
    pts = {}
    ylift = tab.ylift
    for idx, (x, cx, _) in enumerate(X):
        y0 = ylift[x]
        for y in (y0, p - y0):
            if j >= 1:
                lam = F_.mul(F_.sub(y, cx), dinv[idx])
            else:
                lam = F_.sub(y, cx)
            cnt[lam] += 1
            ctr.bucket_ops += 1
            if lam in pts:
                pts[lam].append((x, y))
            else:
                pts[lam] = [(x, y)]
            if cnt[lam] > R:
                raise GateFailure("G4 bucket exceeds R", stage="G4")
    ctr.bucket_scan += p
    cands = []
    for lam in sorted(pts):
        if cnt[lam] == R:
            L = max(len(c), len(Pi))
            cp = c + [0] * (L - len(c))
            pp = Pi + [0] * (L - len(Pi))
            a = [(cp[m] + lam * pp[m]) % p for m in range(L)]
            while a and a[-1] == 0:
                a.pop()
            cands.append({"lam": lam, "anchors": list(anchors), "residual": sorted(pts[lam]), "a": a})
    d = ctr.delta(snap)
    npts = 2 * k
    exp_mul = interp_mul(j, has_t, g) + (len(c) - 1) * len(c)
    if mode == "batch" and j >= 1:
        exp_mul += j * (j + 1) + 3 * (k - 1)
    if mode == "table":
        exp_mul += k * (j - 1)
    if j >= 1:
        exp_mul += npts
    exp_inv = interp_inv(j) + (1 if (mode == "batch" and j >= 1) else 0)
    if d["mul"] != exp_mul or d["inv"] != exp_inv:
        raise GateFailure("count formula: got mul=%d inv=%d expected mul=%d inv=%d" % (d["mul"], d["inv"], exp_mul, exp_inv),
                          stage="G4")
    return cands, d, k


class AnchorStream:
    def __init__(self, spec, g, p, tab, cs, ss):
        self.j = spec["j"]
        self.n = tab.n
        self.tab = tab
        self.p = p
        self.rng = SM64(stream_seed(TAGS[spec["name"]], g, p, cs, ss)) if self.j >= 1 else None
        self.perm = None
        self.tp = 0

    def _fresh(self):
        n = self.n
        perm = list(range(n))
        for i in range(n - 1, 0, -1):
            jj = self.rng.below(i + 1)
            perm[i], perm[jj] = perm[jj], perm[i]
        self.perm = perm
        self.tp = 0

    def next(self):
        j = self.j
        if j == 0:
            return []
        if self.perm is None or j * (self.tp + 1) > self.n:
            self._fresh()
        idxs = self.perm[j * self.tp: j * self.tp + j]
        self.tp += 1
        out = []
        for idx in idxs:
            x = self.tab.B[idx]
            y = self.tab.ylift[x] if self.rng.bit() == 0 else self.p - self.tab.ylift[x]
            out.append((x, y))
        return out


def v2_sampled(i, n_target):
    thr = min(1000, math.ceil(1000 * 200 / n_target))
    return mix64(i ^ 0x5632) % 1000 < thr


def run_arm(name, ctx, g, p, cs, ss, budget, vmode):
    """vmode: 'full' (gate), 'sample' (root relphase), 'v2odd' (rootless/oddrand: odd arms sampled V2), 'none'.
    ctx fields: f, tab_even/tab_odd, s, omap, odd_h, setup counter etc."""
    spec = arm_spec(name, g)
    even = spec["model"] == "even"
    tab = ctx["tab_even"] if even else ctx["tab_odd"]
    h = tab.h
    n_cols = tab.n + (1 if even else 0)
    n_target = n_cols + 8 + n_cols // 100
    ctr = Counter()
    F_ = F(p, ctr)
    t0 = time.process_time()
    inv_tab = None
    table_mul = None
    if spec["inv"] == "table":
        inv_tab = inv_table(p, F_)
        table_mul = p - 2
    t_poly = None
    if spec["e"] is not None:
        from .models import top_part
        t_poly = top_part(ctx["s"], spec["e"], g)
    stream = AnchorStream(spec, g, p, tab, cs, ss)
    ip, im = spec["ip"], spec["im"]
    seen = set()
    keys = []
    rs = RowStats(tab.n)
    S = V.S_series(ctx["f"], g, p) if even else None
    cnts = dict(v1_checked=0, v1_failed=0, vinf_checked=0, vinf_failed=0, v2_checked=0, v2_failed=0,
                m1_checked=0, m1_accepted=0, m2_checked=0, m2_accepted=0)
    pencils = 0
    candidates = 0
    duplicates = 0
    rel_idx = 0
    pencil_records = []
    status = "ok"
    nrel = 0
    while True:
        anchors = stream.next()
        cands, d, k = sieve_pencil(spec, g, p, tab, anchors, F_, inv_tab, t_poly)
        new = 0
        for cd in cands:
            pts = cd["anchors"] + cd["residual"]
            try:
                cnts["v1_checked"] += 1
                V.v1(h, cd["a"], pts, ip, im, g, p, even)
                if even:
                    cnts["vinf_checked"] += 1
                    V.vinf(ctx["f"], cd["a"], ip, im, g, p, S)
            except GateFailure as e:
                if e.msg.startswith("V_inf"):
                    cnts["vinf_failed"] += 1
                else:
                    cnts["v1_failed"] += 1
                e.stage = e.stage or "V1/Vinf"
                e.relation = {"arm": name, "relation": cd, "ip": ip, "im": im}
                e.counts = dict(cnts)
                raise
            candidates += 1
            key = canon_key(p, cd["anchors"], cd["residual"], ip, im, even)
            if key in seen:
                duplicates += 1
                continue
            seen.add(key)
            keys.append(key)
            do_v2 = vmode == "full" or (vmode in ("sample", "v2odd") and v2_sampled(rel_idx, n_target))
            if vmode == "v2odd" and even:
                do_v2 = False
            if do_v2:
                try:
                    cnts["v2_checked"] += 1
                    odd_h = ctx["odd_h"]
                    V.v2(odd_h, ctx.get("omap"), pts, ip, im, g, p, even)
                    if even:
                        cnts["m1_checked"] += 1
                        cnts["m2_checked"] += 1
                        a1, a2 = V.mutations(odd_h, ctx["omap"], cd["anchors"], cd["residual"], ip, im, g, p)
                        if a1:
                            cnts["m1_accepted"] += 1
                        if a2:
                            cnts["m2_accepted"] += 1
                        if a1 or a2:
                            raise GateFailure("mutation accepted m1=%s m2=%s" % (a1, a2))
                except GateFailure as e:
                    if e.msg.startswith("mutation"):
                        pass
                    else:
                        cnts["v2_failed"] += 1
                    e.stage = e.stage or "V2"
                    e.relation = {"arm": name, "relation": cd, "ip": ip, "im": im}
                    e.counts = dict(cnts)
                    raise
            rs.add(pts, tab.col, even and True)
            rel_idx += 1
            new += 1
        # note: has_delta handled below via dc2
        pencils += 1
        pencil_records.append({"type": "pencil", "arm": name, "t": pencils - 1, "anchors": anchors, "k": k,
                               "npts": 2 * k, "mul": d["mul"], "inv": d["inv"], "cands": len(cands), "new": new})
        nrel = len(keys)
        if name == "E_g":
            break
        if nrel >= n_target:
            break
        if pencils >= 10000:
            status = "pencils_exhausted"
            break
        if time.process_time() - t0 > budget:
            status = "budget_exhausted"
            break
    snapc = ctr
    rep = rs.report(even)
    if even:
        rep["delta_fraction"] = (1.0 if spec["dc2"] != 0 else 0.0) if rs.cnt else None
    fpm = ctr.mul + 10 * ctr.inv
    from math import factorial
    rec = {
        "type": "run", "arm": name, "g": g, "p": p, "model": spec["model"], "n": tab.n, "n_cols": n_cols,
        "n_target": n_target, "status": status, "pencils": pencils, "candidates": candidates,
        "relations": nrel, "duplicates": duplicates, "mul": ctr.mul, "inv": ctr.inv, "add": ctr.add,
        "fd_add": ctr.fd_add, "bucket_ops": ctr.bucket_ops, "bucket_scan": ctr.bucket_scan, "fpm": fpm,
        "fpm_per_rel": (fpm / nrel) if nrel else None, "table_mul": table_mul, "R": spec["R"],
        "rels_per_pencil_norm": candidates / (pencils * p / factorial(spec["R"])),
        "keys_sha256": digest(keys), "cpu_seconds": time.process_time() - t0,
    }
    rec.update(rep)
    rec.update(cnts)
    return rec, pencil_records
