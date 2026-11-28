import math
import time
from .field import Counter
from .rng import stream, mix64
from .curve import Curve, key_of, IDENT
from . import native, verify
from .rows import Acc, DSU, canon
from .pencil import sample_v2, Budget


def b_run(T, g, cs, ss, vc, N_target=None, lp_mode=False, nb=None, budget_check=None,
          fpm_cap=2e9):
    p = T.p
    f = T.f
    curve = Curve(p, g, f, T)
    W = stream(0x424C, g, p, cs, ss)
    Rr = stream(0x4252, g, p, cs, ss)
    M = [curve.random_divisor(W) for _ in range(16)]
    R1 = curve.random_divisor(W)
    R2 = curve.random_divisor(W)
    setup_mul, setup_inv = curve.ctr.mul, curve.ctr.inv
    rel = Counter()
    L = p.bit_length()
    ops0 = dict(curve.ops)
    st = {"trials": 0, "candidates": 0, "prefilter_rejects": 0, "splits": 0, "root_failures": 0,
          "walk_add": 0, "restart_add": 0, "restart_dbl": 0, "restarts": 0,
          "v1_checked": 0, "v1_failed": 0, "v2_checked": 0, "v2_failed": 0,
          "lp_rejects": 0, "empty_rejects": 0}
    if lp_mode:
        n = T.n
        if nb is None:
            nb = min(n, math.ceil(2.0 * p ** (1.0 - 1.0 / g)))
        S = {x for x in T.B[:nb]}
        target = nb + 8 + nb // 100
        dsu = DSU()
        rels_by_lp = {"0": 0, "1": 0, "2": 0}
        relations = 0
        duplicates = 0
        seen = set()
    else:
        acc = Acc(T)
    INF = float("inf")
    since = INF
    cur = IDENT
    status = "ok"
    done = False

    def fpm_now():
        return (curve.ctr.mul - setup_mul) + 10 * (curve.ctr.inv - setup_inv) + rel.mul + 10 * rel.inv

    while not done:
        if st["trials"] % 256 == 0 and budget_check:
            try:
                budget_check()
            except Budget:
                status = "budget_exhausted"
                break
        if since >= 4096:
            a = 1 + W.below(2 ** (g * L) - 1)
            b = 1 + W.below(2 ** (g * L) - 1)
            o = dict(curve.ops)
            cur = curve.cantor(curve.mul_ltr(R1, a), curve.mul_ltr(R2, b))
            st["restart_add"] += curve.ops["add"] - o["add"]
            st["restart_dbl"] += curve.ops["dbl"] - o["dbl"]
            since = 0
            st["restarts"] += 1
        else:
            j = (mix64(key_of(cur)) >> 24) % 16
            cur = curve.cantor(cur, M[j])
            since += 1
            st["walk_add"] += 1
        st["trials"] += 1
        u = list(cur[0])
        if len(u) - 1 != g:
            continue
        st["candidates"] += 1
        ch = native.disc_character(p, u, T.chi, rel)
        if ch != 1:
            st["prefilter_rejects"] += 1
            continue
        if g >= 3 and not native.splits_completely(p, u, rel):
            continue
        st["splits"] += 1
        roots = native.roots_of(p, u, Rr, rel)
        if roots is None:
            st["root_failures"] += 1
            continue
        v = list(cur[1]) or [0]
        points = [(x, native.horner_nat(v, x, rel, p)) for x in roots]
        if lp_mode:
            lps = [(x, y) for x, y in points if y != 0 and x not in S]
            inrow = [(x, y) for x, y in points if y != 0 and x in S]
            if len(lps) > 2:
                st["lp_rejects"] += 1
                continue
            if not inrow and not lps:
                st["empty_rejects"] += 1
                continue
        st["v1_checked"] += 1
        if not verify.v1_walk(p, f, g, cur, points):
            st["v1_failed"] += 1
        if lp_mode:
            key = canon(points, p)
            if key in seen:
                duplicates += 1
                continue
            seen.add(key)
            if sample_v2(relations, p, target):
                st["v2_checked"] += 1
                if verify.v2_sum(vc, points) != cur:
                    st["v2_failed"] += 1
            relations += 1
            rels_by_lp[str(len(lps))] += 1
            if len(lps) == 0:
                dsu.edge(0, 0)
            elif len(lps) == 1:
                dsu.edge(0, dsu.vid(lps[0][0]))
            else:
                dsu.edge(dsu.vid(lps[0][0]), dsu.vid(lps[1][0]))
            if dsu.cycles >= target:
                done = True
            elif fpm_now() >= fpm_cap:
                status = "exhausted"
                done = True
        else:
            if acc.add(points):
                i = acc.count - 1
                if sample_v2(i, p, N_target):
                    st["v2_checked"] += 1
                    if verify.v2_sum(vc, points) != cur:
                        st["v2_failed"] += 1
                if acc.count >= N_target:
                    done = True
    cm = curve.ctr.mul - setup_mul
    ci = curve.ctr.inv - setup_inv
    out = dict(st)
    out.update({"cantor_mul": cm, "cantor_inv": ci, "rel_mul": rel.mul, "rel_inv": rel.inv,
                "rel_add": rel.add, "setup_mul": setup_mul, "setup_inv": setup_inv,
                "status": status})
    out["mul"] = cm + rel.mul
    out["inv"] = ci + rel.inv
    out["strong_fpm"] = (35 * st["walk_add"] + 37 * st["restart_dbl"] + 35 * st["restart_add"]
                         + st["candidates"] + 4 * st["splits"]) if g == 2 else None
    if lp_mode:
        out.update({"nb": nb, "target_cycles": target, "relations": relations,
                    "duplicates": duplicates, "rels_by_lp": rels_by_lp, "cycles": dsu.cycles})
    else:
        out.update({"acc": acc})
    return out


def b_labelled(T, g, vc, G, Tt, t, r, h, L_true, rng, nrel=50, budget_check=None):
    """Gate only: labelled walk on [a]G + [b]T; returns (relations checked, bad)."""
    p = T.p
    curve = Curve(p, g, T.f, T)
    lim = min(r, 4096) - 1
    al, be = [], []
    M = []
    for _ in range(16):
        a_, b_ = 1 + rng.below(lim), 1 + rng.below(lim)
        al.append(a_)
        be.append(b_)
        M.append(curve.cantor(curve.mul_ltr(G, a_), curve.mul_ltr(Tt, b_)))
    since = float("inf")
    cur = IDENT
    ca = cb = 0
    seen = set()
    rels = bad = 0
    trials = 0
    while rels < nrel:
        trials += 1
        if trials > 5_000_000:
            raise RuntimeError("b_labelled: too many trials")
        if since >= 4096:
            a_, b_ = 1 + rng.below(r - 1), 1 + rng.below(r - 1)
            cur = curve.cantor(curve.mul_ltr(G, a_), curve.mul_ltr(Tt, b_))
            ca, cb = a_ % r, b_ % r
            since = 0
        else:
            j = (mix64(key_of(cur)) >> 24) % 16
            cur = curve.cantor(cur, M[j])
            ca = (ca + al[j]) % r
            cb = (cb + be[j]) % r
            since += 1
        u = list(cur[0])
        if len(u) - 1 != g:
            continue
        sink = Counter()
        if native.disc_character(p, u, T.chi, sink) != 1:
            continue
        if g >= 3 and not native.splits_completely(p, u, sink):
            continue
        roots = native.roots_of(p, u, rng, sink)
        if roots is None:
            continue
        v = list(cur[1]) or [0]
        points = [(x, native.horner_nat(v, x, sink, p)) for x in roots]
        key = canon(points, p)
        if key in seen:
            continue
        seen.add(key)
        half = (p - 1) // 2
        s = 0
        for x, y in points:
            if y == 0:
                continue
            s += (1 if 1 <= y <= half else -1) * L_true[T.col[x]]
        rels += 1
        yield_ok = (s - h * (ca + cb * t)) % r == 0
        if not yield_ok:
            bad += 1
    return rels, bad

