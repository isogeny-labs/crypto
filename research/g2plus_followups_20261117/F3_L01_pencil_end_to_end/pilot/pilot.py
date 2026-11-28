"""F3 pilot: validate that every correctness gate is REACHABLE and calibrate the
rate gates at the instance sizes the study uses, before PROTOCOL/SPEC are frozen.

This pilot touches ONLY gate quantities and instrument sanity. It never computes
the decision metric (total-cost P/W) on the decision instances.

It reuses the frozen L01 genus-2 package (pilot/l01/, provenance in
pilot/COPIED_FROM_L01.sha256) for Cantor arithmetic, #J, the pencil sieve, the
walk relation engine and the dense mod-r solver. New here: the cheap a1+a2-BSGS
order method (correction to the brief's O(p^2) F_p^2 method), a standalone
negation-map Pollard rho, a pure walk-arm solve, and the rate calibrations.

Run:  python3 -I pilot/pilot.py all   (writes pilot/raw/pilot.json, prints summary)
"""
import json
import math
import os
import sys
import time

HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, HERE)
from l01 import instance as I
from l01 import curve as C
from l01.curve import Curve, IDENT, key_of
from l01.rng import stream, mix64
from l01 import native
from l01.linalg import solve_mod_r
from l01.gate import run_gate

OUT = {}


# ---------------------------------------------------------------------------
# 1. Cheap order method: a1 from N1 over F_p, a2 by BSGS in the sharp Weil band.
#    Validated against l01.instance.jac_order (which uses O(p^2) F_p^2 counting).
# ---------------------------------------------------------------------------
def cheap_order(curve, p, f, chi, rng, confirm=4):
    """Return (#J, a1, a2, bsgs_ops) using only an O(p) F_p scan plus BSGS of
    width <= 4p. Genus 2 only."""
    N1 = I.N1(f, p, chi)
    a1 = p + 1 - N1
    base = p * p + 1 - a1 * (p + 1)          # #J = base + a2
    S = a1 / (2.0 * math.sqrt(p))            # cos th1 + cos th2 in [-2,2]
    aS = abs(S)
    a2lo = int(math.floor(2 * p * (2 * aS - 1))) - 4
    a2hi = int(math.ceil(2 * p + p * S * S)) + 4
    W = a2hi - a2lo
    assert W < 5 * p + 50, (W, p)            # sharp-band sanity
    D = curve.random_divisor(rng)
    ops0 = dict(curve.ops)
    # want M=base+a2 with [M]D=0, a2 in [a2lo,a2hi]. k'=a2-a2lo in [0,W].
    D0 = curve.mul_ltr(D, base + a2lo) if (base + a2lo) >= 1 else None
    # handle non-positive start by shifting into positives
    shift = 0
    while base + a2lo + shift < 1:
        shift += 1
    lo = base + a2lo + shift                 # lo>=1 ; search k' in [0, W-shift] ... keep simple: scan full W
    m = int(math.isqrt(W)) + 1
    baby = {}
    cur = IDENT
    for j in range(m):
        baby.setdefault(key_of(cur), j)
        cur = curve.cantor(cur, D)           # [j]D
    stepD = curve.mul_ltr(D, m)
    negstep = curve.negate(stepD)
    D_lo = curve.mul_ltr(D, lo)
    target = curve.negate(D_lo)              # want [i m]D + [j]D = -[lo]D  => [lo+im+j]D=0
    found_M = None
    q = target
    for i in range(m + 2):
        kk = key_of(q)
        if kk in baby:
            j = baby[kk]
            # verify exact (hash collisions): [lo + i*m + j]D == 0
            M = lo + i * m + j
            if curve.mul_ltr(D, M) == IDENT if M >= 1 else False:
                found_M = M
                break
        q = curve.cantor(q, negstep)
    bsgs_ops = (curve.ops["add"] + curve.ops["dbl"]) - (ops0["add"] + ops0["dbl"])
    if found_M is None:
        return None, a1, None, bsgs_ops
    # confirm it is #J: annihilates several random divisors and is in Weil band
    lo_w = (math.isqrt(p) - 1) ** 4 if p > 1 else 0
    hi_w = (int(math.isqrt(p)) + 2) ** 4
    ok = (lo_w - 4 * p <= found_M <= hi_w + 4 * p)
    for _ in range(confirm):
        if curve.mul_ltr(curve.random_divisor(rng), found_M) != IDENT:
            ok = False
    a2 = found_M - base
    return (found_M if ok else None), a1, a2, bsgs_ops


def check_order_method():
    """GATE G-order reachable: cheap #J == jac_order (F_p^2) on tiny primes, and
    the cheap method also runs at a study-size prime."""
    rows = []
    for p in [13, 17, 19, 23, 29, 37, 53, 101]:
        chi = I.chitab_plain(p)
        for s in range(1, 30):
            f = I.random_f(p, g=2, s=s)
            if not I.squarefree(f, p):
                continue
            T = I.Tables(p, 2, f)
            curve = Curve(p, 2, f, T)
            rng = stream(0x5A, 2, p, s, 0)
            Jref = I.jac_order(I.point_counts(f, p, 2, chi), p, 2)
            Jchp, a1, a2, _ = cheap_order(curve, p, f, chi, rng)
            rows.append({"p": p, "s": s, "Jref": Jref, "Jcheap": Jchp,
                         "match": Jchp == Jref})
            break
    mism = [r for r in rows if not r["match"]]
    # study-size run (p=8191): cheap method must return a verified #J
    p = 8191
    chi = I.chitab_plain(p)
    big = None
    for s in range(1, 60):
        f = I.random_f(p, 2, s)
        if not I.squarefree(f, p):
            continue
        T = I.Tables(p, 2, f)
        curve = Curve(p, 2, f, T)
        rng = stream(0x5A, 2, p, s, 1)
        J, a1, a2, ops = cheap_order(curve, p, f, chi, rng)
        if J is not None:
            big = {"p": p, "s": s, "J": J, "a1": a1, "a2": a2, "bsgs_group_ops": ops,
                   "weil_lo": (math.isqrt(p) - 1) ** 4, "weil_hi": (math.isqrt(p) + 1) ** 4}
            break
    return {"tiny": rows, "tiny_mismatches": len(mism), "study_size": big,
            "pass": len(mism) == 0 and big is not None}


# ---------------------------------------------------------------------------
# 2. Pencil split-rate at study size: relations per anchor tuple / (p/(g+1)!).
#    Instrument sanity (NOT the decision metric).
# ---------------------------------------------------------------------------
def pencil_split_rate(p=8191, s=3, tuples=4):
    f = None
    for ss in range(s, s + 40):
        ff = I.random_f(p, 2, ss)
        if I.squarefree(ff, p):
            f = ff
            s = ss
            break
    T = I.Tables(p, 2, f)
    n = T.n
    half = (p - 1) // 2
    # sieve: pick disjoint anchor pairs from base; per pair count full fibres (3 hits)
    rels = 0
    tup = 0
    used = set()
    base = T.B
    idx = 0
    model = p / 6.0
    per_tuple = []
    while tup < tuples and idx + 1 < len(base):
        x1, x2 = base[idx], base[idx + 1]
        idx += 2
        y1 = T.sq[T.fx[x1]]
        y2 = T.sq[T.fx[x2]]
        # a0 linear through (x1,y1),(x2,y2); Pi=(x-x1)(x-x2)
        # lambda(x)=(y - a0(x))/Pi(x); bucket points by lambda; full fibre => 3 pts
        buckets = {}
        a0_1 = (y2 - y1) * pow((x2 - x1) % p, p - 2, p) % p   # slope
        a0_0 = (y1 - a0_1 * x1) % p
        for x in base:
            if x == x1 or x == x2:
                continue
            Pix = (x - x1) * (x - x2) % p
            if Pix == 0:
                continue
            # two points (x,+y),(x,-y)
            yv = T.sq[T.fx[x]]
            for yy in (yv, (p - yv) % p):
                lam = (yy - (a0_1 * x + a0_0)) % p * pow(Pix, p - 2, p) % p
                buckets.setdefault(lam, []).append((x, yy))
        cnt = sum(1 for lam, pts in buckets.items() if len(pts) == 3)
        per_tuple.append(cnt)
        rels += cnt
        tup += 1
    obs = (rels / tup)
    ratio = obs / model
    return {"p": p, "s": s, "n_cols": n, "tuples": tup, "model_per_tuple": model,
            "obs_per_tuple": obs, "ratio_obs_over_model": ratio,
            "per_tuple": per_tuple, "pass": 0.85 <= ratio <= 1.15}


# ---------------------------------------------------------------------------
# 3. Walk split-rate at study size vs C(n,2) 2^2 / #J (lab finding 4).
# ---------------------------------------------------------------------------
def walk_split_rate(p=8191, s=5, trials=20000):
    chi = I.chitab_plain(p)
    f = None
    for ss in range(s, s + 40):
        ff = I.random_f(p, 2, ss)
        if I.squarefree(ff, p):
            f = ff
            s = ss
            break
    T = I.Tables(p, 2, f)
    curve = Curve(p, 2, f, T)
    J = I.jac_order(I.point_counts(f, p, 2, chi), p, 2)  # tiny: ok at 8191? O(p^2)=6.7e7
    rng = stream(0x7A, 2, p, s, 0)
    M = [curve.random_divisor(rng) for _ in range(16)]
    cur = curve.random_divisor(rng)
    cand = splits = 0
    sink = native.Counter() if hasattr(native, "Counter") else __import__("l01.field", fromlist=["Counter"]).Counter()
    from l01.field import Counter
    sink = Counter()
    for _ in range(trials):
        j = (mix64(key_of(cur)) >> 24) % 16
        cur = curve.cantor(cur, M[j])
        u = list(cur[0])
        if len(u) - 1 != 2:
            continue
        cand += 1
        if native.disc_character(p, u, T.chi, sink) == 1:
            # disc_character==1 means u splits into 2 distinct nonzero-square roots
            splits += 1
    n = T.n
    model = math.comb(n, 2) * 4 / J
    obs = splits / cand if cand else 0
    ratio = obs / model if model else 0
    return {"p": p, "s": s, "J": J, "n_cols": n, "cand": cand, "splits": splits,
            "model": model, "obs": obs, "ratio_obs_over_model": ratio,
            "pass": 0.85 <= ratio <= 1.15}


# ---------------------------------------------------------------------------
# 4. Negation-map Pollard rho: recovery gate + step calibration.
# ---------------------------------------------------------------------------
def rho_solve(curve, G, Tt, r, seed, dp_bits, max_steps, window=64):
    """Standalone r-adding negation-map rho with fruitless-cycle handling.

    Fruitless cycles (intrinsic to the negation map) are detected by a rolling
    window of the last `window` points; on detection the walk escapes
    deterministically from the cycle's minimal-key point by doubling it, so every
    trail entering the same cycle escapes identically (consistency preserved).
    Returns (t_recovered, steps, group_ops)."""
    rng = stream(0x52484F, 2, r % (1 << 40), seed, 0)
    nM = 32
    cs = [1 + rng.below(r - 1) for _ in range(nM)]
    ds = [1 + rng.below(r - 1) for _ in range(nM)]
    Ms = [curve.cantor(curve.mul_ltr(G, cs[k]), curve.mul_ltr(Tt, ds[k])) for k in range(nM)]

    def rep(X, a, b):
        nX = curve.negate(X)
        if key_of(nX) < key_of(X):
            return nX, (-a) % r, (-b) % r
        return X, a % r, b % r

    a0 = 1 + rng.below(r - 1)
    b0 = 1 + rng.below(r - 1)
    X = curve.cantor(curve.mul_ltr(G, a0), curve.mul_ltr(Tt, b0))
    X, a, b = rep(X, a0, b0)
    mask = (1 << dp_bits) - 1
    ops0 = dict(curve.ops)
    table = {}
    recent = {}        # key -> (idx, X, a, b) over the last `window` steps
    order = []         # keys in insertion order (for pruning)
    steps = 0
    while steps < max_steps:
        steps += 1
        k = (mix64(key_of(X)) >> 20) % nM
        Xn = curve.cantor(X, Ms[k])
        an = (a + cs[k]) % r
        bn = (b + ds[k]) % r
        Xn, an, bn = rep(Xn, an, bn)
        kn = key_of(Xn)
        if kn in recent:
            # fruitless cycle: gather it and escape from its minimal-key point
            start = recent[kn][0]
            cyc = [v for v in recent.values() if v[0] >= start]
            cyc.append((steps, Xn, an, bn))
            mk = min(key_of(v[1]) for v in cyc)
            e = next(v for v in cyc if key_of(v[1]) == mk)
            Xn = curve.cantor(e[1], e[1])
            an, bn = (2 * e[2]) % r, (2 * e[3]) % r
            Xn, an, bn = rep(Xn, an, bn)
            kn = key_of(Xn)
            recent.clear()
            order.clear()
        recent[kn] = (steps, Xn, an, bn)
        order.append(kn)
        if len(order) > window:
            old = order.pop(0)
            if recent.get(old, (None,))[0] is not None and old in recent and recent[old][0] <= steps - window:
                recent.pop(old, None)
        X, a, b = Xn, an, bn
        kk = kn
        if (kk & mask) == 0:
            if kk in table:
                a2, b2 = table[kk]
                db = (b - b2) % r
                if db != 0:
                    t = (a2 - a) % r * pow(db, r - 2, r) % r
                    gops = (curve.ops["add"] + curve.ops["dbl"]) - (ops0["add"] + ops0["dbl"])
                    return t, steps, gops
            else:
                table[kk] = (a, b)
    gops = (curve.ops["add"] + curve.ops["dbl"]) - (ops0["add"] + ops0["dbl"])
    return None, steps, gops


def rho_recover_tiny():
    """GATE R reachable: rho recovers planted t and verifies at a tiny near-prime curve."""
    results = []
    for p in [131, 257, 521]:
        chi = I.chitab_plain(p)
        sel = None
        for s in range(1, 400):
            f = I.random_f(p, 2, s)
            if not I.squarefree(f, p):
                continue
            N = I.point_counts(f, p, 2, chi)
            J = I.jac_order(N, p, 2)
            so = I.split_order(J)
            if so:
                sel = (s, f, J, so)
                break
        s, f, J, (h, r) = sel
        T = I.Tables(p, 2, f)
        curve = Curve(p, 2, f, T)
        rng = stream(0xA, 2, p, s, 0)
        while True:
            G = curve.mul_ltr(curve.random_divisor(rng), h)
            if G != IDENT:
                break
        assert curve.mul_ltr(G, r) == IDENT
        t = 1 + rng.below(r - 1)
        Tt = curve.mul_ltr(G, t)
        tr, steps, gops = rho_solve(curve, G, Tt, r, seed=1, dp_bits=2,
                                    max_steps=200 * int(math.isqrt(r) + 1))
        ok = (tr is not None and curve.mul_ltr(G, tr) == Tt)
        results.append({"p": p, "r": r, "t": t, "t_rec": tr, "verify": ok,
                        "steps": steps, "group_ops": gops})
    return {"rows": results, "pass": all(x["verify"] for x in results)}


def rho_step_calibration(p=8191, s=7, seeds=4):
    """Calibrate the rho step gate at study size: steps / sqrt(pi r / 2)."""
    chi = I.chitab_plain(p)
    curve = Tt = G = r = None
    for ss in range(s, s + 120):
        f = I.random_f(p, 2, ss)
        if not I.squarefree(f, p):
            continue
        T = I.Tables(p, 2, f)
        cv = Curve(p, 2, f, T)
        rng0 = stream(0x5A, 2, p, ss, 2)
        J, a1, a2, _ = cheap_order(cv, p, f, chi, rng0)
        if J is None:
            continue
        so = I.split_order(J)
        if not so:
            continue
        h, rr = so
        if rr < 1 << 20:
            continue
        rng = stream(0xB, 2, p, ss, 0)
        while True:
            G = cv.mul_ltr(cv.random_divisor(rng), h)
            if G != IDENT:
                break
        if cv.mul_ltr(G, rr) != IDENT:
            continue
        curve, r, s = cv, rr, ss
        tt = 1 + rng.below(r - 1)
        Tt = cv.mul_ltr(G, tt)
        t_plant = tt
        break
    expected = math.sqrt(math.pi * r / 2.0)
    runs = []
    for sd in range(1, seeds + 1):
        tr, steps, gops = rho_solve(curve, G, Tt, r, seed=sd, dp_bits=8,
                                    max_steps=60 * int(math.isqrt(r) + 1))
        runs.append({"seed": sd, "t_rec": tr, "verify": curve.mul_ltr(G, tr) == Tt if tr else False,
                     "steps": steps, "group_ops": gops, "steps_over_expected": steps / expected})
    good = [x for x in runs if x["verify"]]
    mean_ratio = sum(x["steps_over_expected"] for x in good) / len(good) if good else None
    return {"p": p, "s": s, "r": r, "t_planted": t_plant, "expected_sqrt_pi_r_2": expected,
            "runs": runs, "mean_steps_over_expected": mean_ratio,
            "all_verify": len(good) == len(runs)}


# ---------------------------------------------------------------------------
# 5. Walk-arm end-to-end recovery at tiny p (independent of the pencil arm).
# ---------------------------------------------------------------------------
def walk_arm_recover(p=251):
    chi = I.chitab_plain(p)
    sel = None
    for s in range(1, 400):
        f = I.random_f(p, 2, s)
        if not I.squarefree(f, p):
            continue
        J = I.jac_order(I.point_counts(f, p, 2, chi), p, 2)
        so = I.split_order(J)
        if so:
            sel = (s, f, J, so)
            break
    s, f, J, (h, r) = sel
    T = I.Tables(p, 2, f)
    curve = Curve(p, 2, f, T)
    n = T.n
    rng = stream(0xC, 2, p, s, 0)
    # order check
    okord = all(curve.mul_ltr(curve.random_divisor(rng), J) == IDENT for _ in range(8))
    while True:
        G = curve.mul_ltr(curve.random_divisor(rng), h)
        if G != IDENT:
            break
    assert curve.mul_ltr(G, r) == IDENT
    t = 1 + rng.below(r - 1)
    Tt = curve.mul_ltr(G, t)
    # BSGS logs of all base points (gate uses this to label; here used only to build RHS honestly via alpha,beta)
    half = (p - 1) // 2

    def accept(D):
        u, v = list(D[0]), list(D[1])
        if len(u) - 1 != 2:
            return None
        rts = []
        for x in range(p):
            val = 0
            for c in reversed(u):
                val = (val * x + c) % p
            if val == 0:
                y = 0
                for c in reversed(v):
                    y = (y * x + c) % p
                rts.append((x, y))
        if len(rts) != 2 or any(y == 0 for _, y in rts):
            return None
        row = {}
        for x, y in rts:
            row[T.col[x]] = row.get(T.col[x], 0) + (1 if 1 <= y <= half else -1)
        return row

    # collect walk relations [a]G+[b]T = sum; equation: sum row x_j - (-h b) t = h a
    sys_rows = []
    nun = n + 1
    tries = 0
    while len(sys_rows) < n + 6 and tries < 2_000_000:
        tries += 1
        a = 1 + rng.below(r - 1)
        b = rng.below(r)
        D = curve.cantor(curve.mul_ltr(G, a), curve.mul_ltr(Tt, b)) if b else curve.mul_ltr(G, a)
        row = accept(D)
        if row is None:
            continue
        c = [0] * nun
        for j, vv in row.items():
            c[j] = vv % r
        c[n] = (-h * b) % r
        sys_rows.append((c, (h * a) % r))
    cons, piv, Mred = solve_mod_r(sys_rows, nun, r)
    det = {}
    for col, ri in piv:
        if all(Mred[ri][cc] % r == 0 for cc in range(nun) if cc != col):
            det[col] = Mred[ri][nun] % r
    trec = det.get(n)
    ok = (cons and okord and trec == t and curve.mul_ltr(G, trec) == Tt if trec else False)
    return {"p": p, "s": s, "r": r, "n_cols": n, "rows_collected": len(sys_rows),
            "t": t, "t_rec": trec, "order_check": okord, "consistent": cons, "pass": bool(ok)}


def main():
    t0 = time.time()
    OUT["order_method"] = check_order_method()
    OUT["pencil_arm_gate"] = {("g2_p%d" % p): run_gate(2, p) for p in [131, 257]}
    OUT["pencil_arm_pass"] = all(v["pass"] for v in OUT["pencil_arm_gate"].values())
    OUT["walk_arm_recover"] = walk_arm_recover(251)
    OUT["rho_recover_tiny"] = rho_recover_tiny()
    OUT["pencil_split_rate"] = pencil_split_rate(8191)
    OUT["walk_split_rate"] = walk_split_rate(8191, trials=15000)
    OUT["rho_step_calibration"] = rho_step_calibration(8191)
    OUT["wall_seconds"] = round(time.time() - t0, 1)
    os.makedirs(os.path.join(HERE, "raw"), exist_ok=True)
    with open(os.path.join(HERE, "raw", "pilot.json"), "w") as fh:
        json.dump(OUT, fh, indent=1, sort_keys=True, default=str)
    # concise human summary
    print("=== F3 PILOT SUMMARY ===")
    om = OUT["order_method"]
    print("G-order  cheap #J == F_p^2 jac_order on %d tiny primes, mismatches=%d; study p=8191 #J=%s a1=%s a2=%s bsgs_ops=%s  PASS=%s"
          % (len(om["tiny"]), om["tiny_mismatches"], om["study_size"]["J"], om["study_size"]["a1"],
             om["study_size"]["a2"], om["study_size"]["bsgs_group_ops"], om["pass"]))
    print("G-solve P (pencil end-to-end) at p=131,257: pass=%s" % OUT["pencil_arm_pass"])
    for k, v in OUT["pencil_arm_gate"].items():
        print("   %s: t_planted=%s t_recovered=%s hom_rank=%s n_cols=%s pencil_rows_bad=%s labelled_bad=%s pass=%s"
              % (k, v.get("t_planted"), v.get("t_recovered"), v.get("hom_rank"), v.get("n_cols"),
                 v.get("pencil_rows_bad"), v.get("labelled_rels_bad"), v["pass"]))
    w = OUT["walk_arm_recover"]
    print("G-solve W (walk end-to-end) at p=251: rows=%s t=%s t_rec=%s pass=%s" % (w["rows_collected"], w["t"], w["t_rec"], w["pass"]))
    rr = OUT["rho_recover_tiny"]
    print("G-solve R (rho) recovery: pass=%s" % rr["pass"])
    for x in rr["rows"]:
        print("   p=%s r=%s t=%s t_rec=%s verify=%s steps=%s" % (x["p"], x["r"], x["t"], x["t_rec"], x["verify"], x["steps"]))
    ps = OUT["pencil_split_rate"]
    print("G-split pencil (p=8191): obs/tuple=%.1f model=%.1f ratio=%.3f PASS=%s" % (ps["obs_per_tuple"], ps["model_per_tuple"], ps["ratio_obs_over_model"], ps["pass"]))
    ws = OUT["walk_split_rate"]
    print("G-split walk   (p=8191): obs=%.5f model=%.5f ratio=%.3f PASS=%s" % (ws["obs"], ws["model"], ws["ratio_obs_over_model"], ws["pass"]))
    rc = OUT["rho_step_calibration"]
    print("R step calib  (p=8191): r=%s expected=%.0f mean_steps/expected=%.3f all_verify=%s"
          % (rc["r"], rc["expected_sqrt_pi_r_2"], (rc["mean_steps_over_expected"] or -1), rc["all_verify"]))
    for x in rc["runs"]:
        print("   seed=%s steps=%s ratio=%.3f verify=%s" % (x["seed"], x["steps"], x["steps_over_expected"], x["verify"]))
    print("wall_seconds=%s" % OUT["wall_seconds"])


if __name__ == "__main__":
    main()
