"""Independent recomputation of the Weil-descent round-3/4 headline numbers.

Usage: python3 -I recompute.py <repo-root> <out.json>

Stdlib only.  Reads the recorded CSVs only (paths below), re-derives every line
of the committed summary texts and of validation/gae-band/*.txt with the
validator's own implementation of the cost definitions (PROTOCOL.md
Amendment C, Addendum H / H-a / H-b / H3, Erratum V), compares each committed
line byte for byte, and extracts the headline numbers with their claimed
(printed) values.  Float sums are plain left-to-right loops so that the
summation order equals the producer's (Rust iterator) order.
"""
import csv
import hashlib
import io
import json
import math
import re
import sys

RES = "research/weil_descent_angles_20261018/results"
VAL = "research/weil_descent_angles_20261018/validation/gae-band"
M64 = (1 << 64) - 1


# ------------------------------------------------------------------ helpers
def fsum(xs):
    s = 0.0
    for x in xs:
        s += x
    return s


def mean(xs):
    xs = list(xs)
    return fsum(xs) / len(xs)


def rs(x):
    """Rust `{}` Display of an f64 (values used here: integers, 0.1, 0.125 ...)."""
    if isinstance(x, float) and math.isnan(x):
        return "NaN"
    if float(x) == int(x):
        return str(int(x))
    return repr(float(x))


def re_(x, n):
    """Rust `{:.ne}`: mantissa with n decimals, exponent without sign padding."""
    m, e = f"{x:.{n}e}".split("e")
    return f"{m}e{int(e)}"


def read(repo, rel):
    with open(f"{repo}/{rel}", newline="") as f:
        return f.read()


def rows_of(text):
    r = list(csv.reader(io.StringIO(text)))
    hdr = r[0]
    out = []
    for v in r[1:]:
        d = {}
        for k, x in zip(hdr, v):
            if k == "arm":
                d[k] = x
            else:
                try:
                    d[k] = float(x)
                except ValueError:
                    d[k] = float("nan")
        out.append(d)
    return hdr, out


def col(r, k):
    return r.get(k, 0.0)


# ------------------------------------------------------------------ cost model
def cost_frozen(r, ipr, jac_as_eadd, w, legendre=False):
    """Amendment C: F_p mults, I priced at ipr, native at w M; optional
    Jacobian op repriced as one E(F_{p^2}) addition; H3 Legendre test = one I per step."""
    m = col(r, "walk_M") + col(r, "la_M")
    i = col(r, "walk_I") + col(r, "la_I")
    n = col(r, "walk_native") + col(r, "la_native")
    c = m + ipr * i + w * n
    if jac_as_eadd:
        c -= col(r, "jac_M") + ipr * col(r, "jac_I")
        c += col(r, "jac_ops") * (col(r, "eadd_M") + ipr * col(r, "eadd_I"))
    if legendre:
        c += col(r, "steps") * ipr
    return c


def cost_gae(r, ipr, jac_as_eadd, w_gae, legendre=False):
    """Erratum V: as cost_frozen, but native work priced at w_gae GAE, one GAE =
    the row's own eadd_M + ipr * eadd_I."""
    gae = col(r, "eadd_M") + ipr * col(r, "eadd_I")
    m = col(r, "walk_M") + col(r, "la_M")
    i = col(r, "walk_I") + col(r, "la_I")
    n = col(r, "walk_native") + col(r, "la_native")
    c = m + ipr * i + w_gae * gae * n
    if jac_as_eadd:
        c -= col(r, "jac_M") + ipr * col(r, "jac_I")
        c += col(r, "jac_ops") * gae
    if legendre:
        c += col(r, "steps") * ipr
    return c


def ipr_of(kind):
    return (lambda r: 3.0) if kind == 0 else (lambda r: 2.0 * math.log2(r["p"]))


# ------------------------------------------------------------------ round 3 run mode
def summarize_routes(text):
    _, rows = rows_of(text)
    bits = sorted({int(r["bits"]) for r in rows})
    arms = [("J", 1.0), ("J", 2.0), ("J", 4.0), ("D", 1.0), ("D", 2.0), ("D", 4.0)]
    rsum = ["bits,arm,c,ipr,jac_pricing,native_w,runs,verified,mean_p,mean_cost,mean_rho_cost,ratio_to_rho,mean_steps_over_p,rho_steps_over_p"]
    rank = ["bits,arm,c,cells,ic_cheaper_cells,rho_cheaper_cells,min_ratio,max_ratio,verdict"]
    txt = []
    vals = {}
    ver_all = True

    def sel(b, arm, c):
        return [r for r in rows if int(r["bits"]) == b and r["arm"] == arm and (arm == "R" or r["c"] == c)]

    for b in bits:
        rho = sel(b, "R", 0.0)
        mean_p = mean(r["p"] for r in rho)
        rho_steps = mean(r["steps"] / r["p"] for r in rho)
        for arm, c in arms:
            ic = sel(b, arm, c)
            if not ic:
                continue
            ver = all(r["verified"] == 1.0 for r in ic + rho)
            ver_all &= ver
            steps = mean(r["steps"] / r["p"] for r in ic)
            lo, hi, cells, icw, rw = float("inf"), 0.0, 0, 0, 0
            for ik in (0, 1):
                ipr = ipr_of(ik)
                for jp in (False, True):
                    if arm == "D" and jp:
                        continue
                    for w in (0.0, 0.1, 1.0):
                        ic_c = mean(cost_frozen(r, ipr(r), jp, w) for r in ic)
                        rho_c = mean(cost_frozen(r, ipr(r), False, w) for r in rho)
                        ratio = ic_c / rho_c
                        lo, hi = min(lo, ratio), max(hi, ratio)
                        cells += 1
                        if ratio < 1.0:
                            icw += 1
                        else:
                            rw += 1
                        rsum.append(f"{b},{arm},{rs(c)},{'3M' if ik == 0 else '2log2p_M'},{'eadd' if jp else 'measured'},{rs(w)},{len(ic)},{int(ver)},{mean_p:.0f},{ic_c:.1f},{rho_c:.1f},{ratio:.4f},{steps:.4f},{rho_steps:.4f}")
            verdict = "IC cheaper in every cell" if icw == cells else ("rho cheaper in every cell" if rw == cells else "unresolved")
            rank.append(f"{b},{arm},{rs(c)},{cells},{icw},{rw},{lo:.4f},{hi:.4f},{verdict}")
            txt.append(f"O/J bits {b} arm {arm} c {rs(c)}: IC/rho ratio in [{lo:.3f}, {hi:.3f}] over {cells} cells -> {verdict}; steps/p {steps:.3f} (rho {rho_steps:.3f})")
            vals[f"band_frozen/{b}/{arm}{rs(c)}"] = dict(lo=lo, hi=hi, cells=cells, ic_cheaper=icw, steps_over_p=steps, rho_steps_over_p=rho_steps)
    for arm, c in [("R", 0.0)] + arms:
        xs, ys = [], []
        for b in bits:
            s = [r for r in rows if int(r["bits"]) == b and r["arm"] == arm and (arm == "R" or r["c"] == c)]
            if not s:
                continue
            xs.append(math.log(mean(r["p"] for r in s)))
            ys.append(math.log(mean(cost_frozen(r, 3.0, False, 0.1) for r in s)))
        if len(xs) >= 2:
            n = float(len(xs))
            mx, my = fsum(xs) / n, fsum(ys) / n
            sxy = fsum((x - mx) * (y - my) for x, y in zip(xs, ys))
            sxx = fsum((x - mx) ** 2 for x in xs)
            txt.append(f"O/J scaling arm {arm} c {rs(c)}: cost ~ p^{sxy / sxx:.3f}")
            vals[f"scaling/{arm}{rs(c)}"] = sxy / sxx
    for b in bits:
        rho = [r for r in rows if int(r["bits"]) == b and r["arm"] == "R"]
        n = float(len(rho))
        obs = [r["steps"] / r["p"] for r in rho]
        th = [0.886 * math.sqrt(r["ell"] / (r["p"] * r["p"])) for r in rho]
        mo, mt = fsum(obs) / n, fsum(th) / n
        sd = math.sqrt(fsum((x - mo) ** 2 for x in obs) / max(n - 1.0, 1.0))
        txt.append(f"K6 bits {b}: rho steps/p mean {mo:.4f} (s.e. {sd / math.sqrt(n):.4f}, {rs(n)} runs) vs 0.886*sqrt(ell)/p mean {mt:.4f}")
        vals[f"K6/{b}"] = dict(mean=mo, se=sd / math.sqrt(n), theory=mt, runs=n)
    txt.append(f"O/J all answers verified: {'true' if ver_all else 'false'}")
    vals["verified_rows"] = sum(1 for r in rows if r["verified"] == 1.0)
    vals["rows"] = len(rows)
    return "\n".join(rsum) + "\n", "\n".join(rank) + "\n", "\n".join(txt) + "\n", vals


# ------------------------------------------------------------------ control
def summarize_control(text):
    _, rows = rows_of(text)
    by, byh = {}, {}
    ok = True
    for r in rows:
        ok &= r["verified"] == 1.0
        by.setdefault((int(r["bits"]), r["arm"]), []).append(r)
        byh.setdefault((r["arm"], int(r["h"])), []).append(r["steps"] / r["p"])
    t = []
    for (b, arm) in sorted(by):
        v = by[(b, arm)]
        n = float(len(v))
        sp = [r["steps"] / r["p"] for r in v]
        m = fsum(sp) / n
        sd = math.sqrt(fsum((x - m) ** 2 for x in sp) / (n - 1.0))
        dup = fsum(r["repeat_events"] for r in v) / n
        sbd = fsum(r["solved_by_repeat"] for r in v)
        t.append(f"F bits {b} {arm}: runs {rs(n)} steps/p mean {m:.4f} sd {sd:.4f} | repeated divisors per run {dup:.2f} | solved by a repeat {rs(sbd)}/{rs(n)}")
    for (arm, h) in sorted(byh):
        v = byh[(arm, h)]
        n = float(len(v))
        m = fsum(v) / n
        sd = math.sqrt(fsum((x - m) ** 2 for x in v) / max(n - 1.0, 1.0))
        t.append(f"F by cofactor {arm} h={h}: runs {rs(n)} steps/p mean {m:.4f} sd {sd:.4f} (s.e. {sd / math.sqrt(n):.4f})")
    t.append(f"F all answers verified: {'true' if ok else 'false'}")
    return "\n".join(t) + "\n", dict(verified_rows=sum(1 for r in rows if r["verified"] == 1.0), rows=len(rows))


# ------------------------------------------------------------------ H1 / H4
def xorshift(s):
    x = s
    x ^= (x << 13) & M64
    x ^= x >> 7
    x ^= (x << 17) & M64
    return x & M64


def summarize_hsweep(text):
    _, rows = rows_of(text)
    per, theta_rows = {}, {}
    ok = True
    for r in rows:
        b, h, inst = int(r["bits"]), int(r["h"]), int(r["instance"])
        ok &= r["verified"] == 1.0
        arm, theta = r["arm"], r["theta"]
        if arm == "Jtheta" or (arm == "J" and theta == 1.0 and h == 4):
            theta_rows.setdefault((b, rs(theta)), []).append((r["steps"] / r["p"], r["stored_rows"] / r["p"]))
        per.setdefault((b, h, inst, arm), []).append((r["steps"] / r["p"], r["relations"] / max(r["steps"], 1.0),
                                                      r["model_first_cycle_edges"] / r["p"], r["stored_rows"] / r["p"], theta))
    out, vals = [], {}
    cells, model_cells = {}, {}
    for key in sorted(per):
        b, h, _inst, arm = key
        v = per[key]
        if v[0][4] != 1.0:
            continue
        cells.setdefault((b, h, arm), []).append(fsum(x[0] for x in v) / len(v))
        if arm == "J":
            split = fsum(x[1] for x in v) / len(v)
            me = v[0][2]
            if math.isfinite(me) and split > 0.0:
                model_cells.setdefault((b, h), []).append(me / split)
    for key in sorted(cells):
        b, h, arm = key
        v = cells[key]
        n = float(len(v))
        m = fsum(v) / n
        sd = math.sqrt(fsum((x - m) ** 2 for x in v) / max(n - 1.0, 1.0))
        model = ""
        if arm == "J" and (b, h) in model_cells:
            w = model_cells[(b, h)]
            model = f" | model {fsum(w) / len(w):.4f}"
        out.append(f"H1 bits {b} h {h} {arm}: trials/p {m:.4f} (s.e. over {len(v)} instances {sd / math.sqrt(n):.4f}){model} | rho theory 0.886/sqrt(h) {0.886 / math.sqrt(h):.4f}")
    for arm in ("J", "R", "Jind"):
        pts = [(math.log(h), cells[(b, h, a)]) for (b, h, a) in sorted(cells) if a == arm]
        if len(pts) < 3:
            continue

        def fit(ps):
            n = float(len(ps))
            mx = fsum(x for x, _ in ps) / n
            my = fsum(y for _, y in ps) / n
            sxy = fsum((x - mx) * (y - my) for x, y in ps)
            sxx = fsum((x - mx) ** 2 for x, _ in ps)
            return -sxy / sxx

        base = [(lh, math.log(fsum(v) / len(v))) for lh, v in pts]
        alpha = fit(base)
        rng = 0xA1FA
        boots = []
        for _ in range(2000):
            bp = []
            for lh, v in pts:
                acc = []
                for _ in range(len(v)):
                    rng = xorshift(rng)
                    acc.append(v[rng % len(v)])
                bp.append((lh, math.log(fsum(acc) / len(v))))
            boots.append(fit(bp))
        boots.sort()
        lo, hi = boots[int(0.025 * len(boots))], boots[int(0.975 * len(boots))]
        verdict = ""
        if arm == "J":
            verdict = (" -> alpha < 1/2: Arm J never beats rho on count (stop)" if hi < 0.5 else
                       " -> alpha > 1/2: report the crossover h*" if lo > 0.5 else " -> unresolved")
        out.append(f"H1 alpha fit {arm}: alpha {alpha:.3f} (bootstrap 95% [{lo:.3f}, {hi:.3f}], {len(pts)} cells){verdict}")
        vals[f"alpha/{arm}"] = dict(alpha=alpha, lo=lo, hi=hi, cells=len(pts))
    for key in sorted(theta_rows):
        b, th = key
        v = theta_rows[key]
        n = float(len(v))
        t = fsum(x[0] for x in v) / n
        m = fsum(x[1] for x in v) / n
        out.append(f"H4 bits {b} theta {th}: trials/p {t:.4f} stored/p {m:.5f} time*memory/p^2 {t * m:.5f} ({len(v)} runs)")
        vals[f"H4/{b}/{th}"] = dict(trials=t, stored=m, runs=len(v))
    out.append(f"H1/H4 all answers verified: {'true' if ok else 'false'}")
    vals["verified_rows"] = sum(1 for r in rows if r["verified"] == 1.0)
    vals["rows"] = len(rows)
    return "\n".join(out) + "\n", vals


# ------------------------------------------------------------------ H2
def summarize_glue(text):
    _, rows = rows_of(text)
    cells, per_op = {}, {}
    ok = True
    for r in rows:
        ok &= r["verified"] == 1.0
        cells.setdefault((int(r["bits"]), int(r["h"]), r["arm"]), []).append((r["steps"] / r["p"], r["walk_M"] + 3.0 * r["walk_I"]))
        if r["arm"] == "Jglue" and r["jac_ops"] > 0.0:
            e = per_op.setdefault("Jglue", [0.0, 0.0, 0.0])
            e[0] += r["jac_M"]
            e[1] += r["jac_I"]
            e[2] += r["jac_ops"]
    out, vals = [], {}
    for key in sorted(cells):
        b, h, arm = key
        v = cells[key]
        n = float(len(v))
        m = fsum(x[0] for x in v) / n
        sd = math.sqrt(fsum((x[0] - m) ** 2 for x in v) / max(n - 1.0, 1.0))
        c = fsum(x[1] for x in v) / n
        out.append(f"H2 bits {b} h {h} {arm}: steps/p {m:.4f} (s.e. {sd / math.sqrt(n):.4f}, {len(v)} runs) | mean cost (I=3M, w=0) {re_(c, 4)} | rho theory {0.886 / math.sqrt(h):.4f}")
        vals[f"steps/{b}/{h}/{arm}"] = dict(mean=m, se=sd / math.sqrt(n), runs=len(v))
    for key in sorted(cells):
        b, h, arm = key
        if arm != "Jglue" or (b, h, "R") not in cells:
            continue
        c = fsum(x[1] for x in cells[key]) / len(cells[key])
        cr = fsum(x[1] for x in cells[(b, h, "R")]) / len(cells[(b, h, "R")])
        out.append(f"H2 bits {b} h {h}: Jglue/rho cost ratio (measured, I=3M, w=0) {c / cr:.3f}")
    for arm in sorted(per_op):
        m, i, j = per_op[arm]
        out.append(f"H2 {arm}: measured per Jacobian op {m / j:.1f}M + {i / j:.2f}I")
        vals["per_op"] = dict(M=m / j, I=i / j, jac_ops=j)
    groups = {}
    for r in rows:
        groups.setdefault((int(r["bits"]), int(r["h"]), r["arm"]), []).append(r)
    for key in sorted(groups):
        b, h, arm = key
        if arm != "Jglue" or (b, h, "R") not in groups:
            continue
        v, rr = groups[key], groups[(b, h, "R")]
        lo, hi, icw, cells_n = float("inf"), 0.0, 0, 0
        for ik in (0, 1):
            ipr = ipr_of(ik)
            for jp in (False, True):
                for w in (0.0, 0.1, 1.0):
                    ci = mean(cost_frozen(r, ipr(r), jp, w, legendre=True) for r in v)
                    cr = mean(cost_frozen(r, ipr(r), False, w) for r in rr)
                    ratio = ci / cr
                    lo, hi = min(lo, ratio), max(hi, ratio)
                    cells_n += 1
                    icw += ratio < 1.0
        verdict = "IC cheaper in every cell" if icw == cells_n else ("rho cheaper in every cell" if icw == 0 else "unresolved")
        out.append(f"H2 band bits {b} h {h}: Jglue/rho in [{lo:.3f}, {hi:.3f}] over {cells_n} cells (IC cheaper in {icw}) -> {verdict}")
        vals[f"band_frozen/{b}/{h}"] = dict(lo=lo, hi=hi, ic_cheaper=icw, cells=cells_n)
    out.append(f"H2 all answers verified: {'true' if ok else 'false'}")
    vals["verified_rows"] = sum(1 for r in rows if r["verified"] == 1.0)
    vals["rows"] = len(rows)
    return "\n".join(out) + "\n", vals


# ------------------------------------------------------------------ Erratum V (GAE band) and the gate's three points
def gae_band(text):
    hdr, rows = rows_of(text)
    glue = "twist" in hdr
    groups = {}
    for r in rows:
        arm = r["arm"]
        if not glue and arm != "R":
            arm = f"{arm}{rs(r['c'])}"
        groups.setdefault((int(r["bits"]), int(r["h"]) if glue else 0, arm), []).append(r)
    out, vals = [], {}
    for key in sorted(groups):
        b, h, arm = key
        if arm == "R" or (b, h, "R") not in groups:
            continue
        v, rr = groups[key], groups[(b, h, "R")]
        leg = glue and arm == "Jglue"
        lo, hi, icw, cells_n = float("inf"), 0.0, 0, 0
        for ik in (0, 1):
            ipr = ipr_of(ik)
            for jp in (False, True):
                if arm.startswith("D") and jp:
                    continue
                for w in (0.0, 0.1, 1.0):
                    ci = mean(cost_gae(r, ipr(r), jp, w, leg) for r in v)
                    cr = mean(cost_gae(r, ipr(r), False, w) for r in rr)
                    ratio = ci / cr
                    lo, hi = min(lo, ratio), max(hi, ratio)
                    cells_n += 1
                    icw += ratio < 1.0
        verdict = "IC cheaper in every cell" if icw == cells_n else ("rho cheaper in every cell" if icw == 0 else "unresolved")
        out.append(f"GAE band bits {b} h {h} {arm}: {arm}/rho in [{lo:.3f}, {hi:.3f}] over {cells_n} cells (IC cheaper in {icw}) -> {verdict}")
        vals[f"band_gae/{b}/{h}/{arm}"] = dict(lo=lo, hi=hi, ic_cheaper=icw, cells=cells_n)
        # gate three points: I = 3M, measured Jacobian pricing, mean per-solve cost in GAE
        for w in (0.0, 0.1, 1.0):
            g = lambda r: col(r, "eadd_M") + 3.0 * col(r, "eadd_I")
            ci = mean(cost_gae(r, 3.0, False, w, leg) / g(r) for r in v)
            cr = mean(cost_gae(r, 3.0, False, w) / g(r) for r in rr)
            out.append(f"GAE point bits {b} h {h} {arm} w {rs(w)}: ic {re_(ci, 6)} GAE, rho {re_(cr, 6)} GAE, ratio {ci / cr:.4f}")
            vals[f"gate/{b}/{h}/{arm}/w{rs(w)}"] = dict(ic=ci, rho=cr, ratio=ci / cr, n_ic=len(v), n_rho=len(rr))
    return "\n".join(out) + "\n", vals


# ------------------------------------------------------------------ main
def compare(name, mine, committed):
    a, b = mine.splitlines(), committed.splitlines()
    diffs = [(i, x, y) for i, (x, y) in enumerate(zip(a, b)) if x != y]
    return dict(file=name, lines_committed=len(b), lines_recomputed=len(a),
                identical=(a == b), differing_lines=[dict(line=i + 1, recomputed=x, committed=y) for i, x, y in diffs[:20]])


def main():
    repo, outp = sys.argv[1], sys.argv[2]
    files = {
        "oj_routes": f"{RES}/itemOJ-run-20261018T2132Z/routes.csv",
        "oj_summary": f"{RES}/itemOJ-run-20261018T2132Z/summary.txt",
        "oj_rsum": f"{RES}/itemOJ-run-20261018T2132Z/routes_summary.csv",
        "oj_rank": f"{RES}/itemOJ-run-20261018T2132Z/ranking.csv",
        "f_control": f"{RES}/itemF-control-run-20261018T2128Z/control.csv",
        "f_summary": f"{RES}/itemF-control-run-20261018T2128Z/control_summary.txt",
        "h1_csv": f"{RES}/H1-hsweep-run-20261018T2240Z/hsweep.csv",
        "h1_summary": f"{RES}/H1-hsweep-run-20261018T2240Z/hsweep_summary.txt",
        "h2_csv": f"{RES}/H2-glue-run-20261018T2240Z/glue.csv",
        "h2_summary": f"{RES}/H2-glue-run-20261018T2240Z/glue_summary.txt",
        "h4_csv": f"{RES}/H4-memsweep-run-20261018T2209Z/memsweep.csv",
        "h4_summary": f"{RES}/H4-memsweep-run-20261018T2209Z/memsweep_summary.txt",
        "h6_routes": f"{RES}/H6-large-run-20261018T2210Z/routes.csv",
        "h6_summary": f"{RES}/H6-large-run-20261018T2210Z/summary.txt",
        "h6_rsum": f"{RES}/H6-large-run-20261018T2210Z/routes_summary.csv",
        "h6_rank": f"{RES}/H6-large-run-20261018T2210Z/ranking.csv",
        "gae_oj": f"{VAL}/itemOJ-run-20261018T2132Z.txt",
        "gae_h2": f"{VAL}/H2-glue-run-20261018T2240Z.txt",
        "gae_h6": f"{VAL}/H6-large-run-20261018T2210Z.txt",
    }
    T = {k: read(repo, v) for k, v in files.items()}
    res = dict(files={k: dict(path=v, sha256=hashlib.sha256(T[k].encode()).hexdigest()) for k, v in files.items()})
    cmp_, vals = [], {}
    rsum, rank, txt, v = summarize_routes(T["oj_routes"])
    vals["oj"] = v
    cmp_ += [compare(files["oj_summary"], txt, T["oj_summary"]), compare(files["oj_rsum"], rsum, T["oj_rsum"]),
             compare(files["oj_rank"], rank, T["oj_rank"])]
    rsum, rank, txt, v = summarize_routes(T["h6_routes"])
    vals["h6"] = v
    cmp_ += [compare(files["h6_summary"], txt, T["h6_summary"]), compare(files["h6_rsum"], rsum, T["h6_rsum"]),
             compare(files["h6_rank"], rank, T["h6_rank"])]
    txt, v = summarize_control(T["f_control"])
    vals["f"] = v
    cmp_.append(compare(files["f_summary"], txt, T["f_summary"]))
    txt, v = summarize_hsweep(T["h1_csv"])
    vals["h1"] = v
    cmp_.append(compare(files["h1_summary"], txt, T["h1_summary"]))
    txt, v = summarize_hsweep(T["h4_csv"])
    vals["h4"] = v
    cmp_.append(compare(files["h4_summary"], txt, T["h4_summary"]))
    txt, v = summarize_glue(T["h2_csv"])
    vals["h2"] = v
    cmp_.append(compare(files["h2_summary"], txt, T["h2_summary"]))
    for k, csvk in (("gae_oj", "oj_routes"), ("gae_h2", "h2_csv"), ("gae_h6", "h6_routes")):
        txt, v = gae_band(T[csvk])
        vals[k] = v
        cmp_.append(compare(files[k], txt, T[k]))
    res["text_comparisons"] = cmp_
    res["values"] = vals
    with open(outp, "w") as f:
        json.dump(res, f, indent=1)
    for c in cmp_:
        print(f"{c['file']}: committed {c['lines_committed']} lines, recomputed {c['lines_recomputed']}, identical {c['identical']}")
        for d in c["differing_lines"][:5]:
            print("   L%d mine: %s\n        com:  %s" % (d["line"], d["recomputed"], d["committed"]))


if __name__ == "__main__":
    main()
