#!/usr/bin/env python3
"""Validator re-tabulation of g3_affine_twist_20261121 from the raw rows.

Independent of analyze.py: every key is read with [] (lesson L-99ee952350b2), the
kappa rule is applied as stated in PROTOCOL.md, Wilson intervals are recomputed,
and extra checks (nulls, within-curve variation, independence test, Lagrangian
combinatorics) are added.  Usage: python3 retabulate.py STUDY_DIR
"""
import itertools
import json
import math
import os
import sys
from collections import Counter, defaultdict


def leg(x, p):
    x %= p
    if x == 0:
        return 0
    return 1 if pow(x, (p - 1) // 2, p) == 1 else -1


def wilson(k, n, z=1.959963984540054):
    ph = k / n
    den = 1 + z * z / n
    cen = (ph + z * z / (2 * n)) / den
    half = z * math.sqrt(ph * (1 - ph) / n + z * z / (4 * n * n)) / den
    return (cen - half, cen + half)


def fisher_two_sided(a, b, c, d):
    """Exact two-sided Fisher test for [[a,b],[c,d]] (sum of tables with prob <= observed)."""
    r1, r2, c1 = a + b, c + d, a + c
    n = r1 + r2

    def lp(x):
        return (math.lgamma(r1 + 1) + math.lgamma(r2 + 1) + math.lgamma(c1 + 1) + math.lgamma(n - c1 + 1)
                - math.lgamma(n + 1) - math.lgamma(x + 1) - math.lgamma(r1 - x + 1)
                - math.lgamma(c1 - x + 1) - math.lgamma(r2 - c1 + x + 1))
    lo, hi = max(0, c1 - r2), min(r1, c1)
    obs = lp(a)
    return sum(math.exp(lp(x)) for x in range(lo, hi + 1) if lp(x) <= obs + 1e-9)


# ------------------------------------------------------------ Lagrangian combinatorics
def norm(s):
    s = frozenset(s)
    c = frozenset(range(8)) - s
    return min(s, c, key=lambda t: (len(t), sorted(t)))


def all_lagrangians():
    elems = sorted({norm(s) for k in (0, 2, 4) for s in itertools.combinations(range(8), k)},
                   key=lambda t: (len(t), sorted(t)))
    assert len(elems) == 64
    nz = [e for e in elems if e]

    def add(x, y):
        return norm(set(x) ^ set(y))

    def pair(x, y):
        return len(set(x) & set(y)) % 2
    lags = set()
    for x, y, z in itertools.combinations(nz, 3):
        if pair(x, y) or pair(x, z) or pair(y, z):
            continue
        span = {frozenset(), x, y, z, add(x, y), add(x, z), add(y, z), add(add(x, y), z)}
        if len(span) == 8:
            lags.add(frozenset(s for s in span if s))
    return lags


def combinatorics(rows_main):
    lags = all_lagrangians()
    kind = {L: ("pair" if any(len(s) == 2 for s in L) else "affine") for L in lags}
    cnt = Counter(kind.values())
    frob = [1, 2, 3, 4, 5, 6, 0, 7]
    stable = [L for L in lags if frozenset(norm([frob[i] for i in s]) for s in L) == L]
    st_kind = Counter(kind[L] for L in stable)
    data_L = {}
    for r in rows_main:
        data_L.setdefault(r["L"], set()).add(frozenset(norm(s) for s in r["L_subsets"]))
    data_ok = all(len(v) == 1 for v in data_L.values()) and \
        {next(iter(v)) for v in data_L.values()} == {L for L in stable if kind[L] == "affine"}
    return {"lagrangians": len(lags), "by_kind": dict(cnt), "stable_under_7cycle": len(stable),
            "stable_by_kind": dict(st_kind), "data_L_equal_the_stable_affine_pair": data_ok}


# ------------------------------------------------------------ control
def control(rows):
    out = {}
    within_curve = []
    null_c_only = Counter()
    for p in sorted({r["p"] for r in rows}):
        st = Counter()
        for r in [r for r in rows if r["p"] == p]:
            st["kernels"] += 1
            if "smith" in r:
                assert r["smith"] == "degenerate_system"
                st["degenerate_system"] += 1
                continue
            if r["n_maps"] == 0:
                st["no_map"] += 1
                continue
            v0, v1 = r["vals"]
            if v0 is None or v1 is None:
                st["G1_fail"] += 1
                continue
            if v0 != v1:
                st["G3_fail"] += 1
            syms = {s for ss in r["smith_symbols"] for s in ss}
            if len(syms) != 1:
                st["smith_inconsistent"] += 1
                continue
            if v0 == 0:
                st["S_zero_nmaps_%d" % r["n_maps"]] += 1
                continue
            sm = next(iter(syms))
            st["smith_pass" if sm == 1 else "smith_fail"] += 1
            raw = leg(r["c"], p) * leg(v0, p)
            st["raw_agree" if raw == sm else "raw_disagree"] += 1
            st["agree_kappa_chi(-1)" if leg(-1, p) * raw == sm else "disagree_kappa_chi(-1)"] += 1
            # null: chi(c) alone
            null_c_only[(p, leg(-1, p) * leg(r["c"], p) == sm)] += 1
            within_curve.append((p, r["curve"], sm, leg(v0, p), leg(r["c"], p)))
        n = st["raw_agree"] + st["raw_disagree"]
        kap = 1 if (n >= 50 and st["raw_disagree"] == 0) else (-1 if (n >= 50 and st["raw_agree"] == 0) else None)
        st["compared"] = n
        st["kappa_p"] = kap
        st["chi_p(-1)"] = leg(-1, p)
        out[p] = dict(st)
    pats = {u: all(out[p]["kappa_p"] == leg(u, p) for p in out) for u in (1, -1, 2, -2)}
    # within-curve variation of the Smith symbol (c fixed per curve)
    per_curve = defaultdict(Counter)
    for p, cur, sm, ls, lc in within_curve:
        per_curve[(p, cur)][sm] += 1
    mixed = sum(1 for v in per_curve.values() if len(v) == 2)
    return out, pats, null_c_only, mixed, len(per_curve), within_curve


# ------------------------------------------------------------ main arm
def main_arm(rows, kappa):
    res = {}
    allc = Counter()
    perp = defaultdict(Counter)
    curves = defaultdict(dict)
    g2 = Counter()
    byN1 = defaultdict(lambda: [0, 0])
    bins = defaultdict(lambda: [0, 0, 0.0])
    fseen = Counter()
    for r in rows:
        p = r["p"]
        fseen[(p, tuple(r["f"]))] += 1
        ev = r["evals"]
        e0 = ev[0]
        if len(ev) == 2:
            g2["rows"] += 1
            sg = []
            cs = []
            for e in ev:
                g2["inv_roots"] += bool(e["inv_roots"])
                g2["inv_zeta8"] += bool(e["inv_zeta8"])
                g2["inv_sigma"] += bool(e["inv_sigma"])
                g2["formula2_equal"] += (e["formula2_val"] == e["val"])
                g2["evals"] += 1
                sg.append(leg(e["c"], p) * leg(e["val"], p))
                cs.append(leg(e["c"], p))
            g2["model_t_same_sign"] += (sg[0] == sg[1])
            g2["model_t_informative(chi(c) differs)"] += (cs[0] != cs[1])
            g2["model_t_informative_and_same"] += (cs[0] != cs[1] and sg[0] == sg[1])
            g2["both_zero"] += (ev[0]["val"] == 0 and ev[1]["val"] == 0)
        # c recorded must be f(t)
        for e in ev:
            ft = sum(cf * pow(e["t"], i, p) for i, cf in enumerate(r["f"])) % p
            if ft != e["c"]:
                g2["c_mismatch"] += 1
        if e0["val"] is None:
            allc["G1_fail"] += 1
            continue
        if e0["val"] == 0:
            allc["degenerate"] += 1
            perp[p]["degenerate"] += 1
            curves[(p, tuple(r["f"]))][r["L"]] = 0
            continue
        s = kappa[p] * leg(e0["c"], p) * leg(e0["val"], p)
        allc["jac" if s == 1 else "twist"] += 1
        perp[p]["jac" if s == 1 else "twist"] += 1
        curves[(p, tuple(r["f"]))][r["L"]] = s
        n1 = r["N1"]
        grp = "N1>p+1" if n1 > p + 1 else ("N1<p+1" if n1 < p + 1 else "N1=p+1")
        byN1[grp][0] += (s == 1)
        byN1[grp][1] += 1
        x = n1 / (2 * p + 2)
        b = min(int(x * 10), 9)
        bins[b][0] += (s == 1)
        bins[b][1] += 1
        bins[b][2] += x
    res["duplicate_curves"] = sum(1 for v in fseen.values() if v != 2)
    res["all"] = dict(allc)
    nd = allc["jac"] + allc["twist"]
    res["r"] = (allc["jac"], nd, allc["jac"] / nd, wilson(allc["jac"], nd))
    res["per_prime"] = {p: (v["jac"], v["twist"], v["degenerate"], v["jac"] / (v["jac"] + v["twist"]),
                            wilson(v["jac"], v["jac"] + v["twist"])) for p, v in sorted(perp.items())}
    tab = Counter()
    cov = 0
    deg_per_curve = Counter()
    for key, d in curves.items():
        assert set(d) == {0, 1}, key
        a, b = d[0], d[1]
        deg_per_curve[(a == 0) + (b == 0)] += 1
        if a == 0 or b == 0:
            tab["with_degenerate:" + ("J" if 1 in (a, b) else "T")] += 1
        else:
            tab[("+" if a == 1 else "-") + ("+" if b == 1 else "-")] += 1
        cov += (a == 1 or b == 1)
    res["curves"] = len(curves)
    res["two_L_table"] = dict(tab)
    res["degenerate_per_curve"] = dict(deg_per_curve)
    res["coverage"] = (cov, len(curves), cov / len(curves), wilson(cov, len(curves)))
    a, b, c, d = tab["++"], tab["+-"], tab["-+"], tab["--"]
    n = a + b + c + d
    res["independence"] = {
        "n_both_nondeg": n, "phi": (a * d - b * c) / math.sqrt((a + b) * (c + d) * (a + c) * (b + d)),
        "fisher_two_sided_p": fisher_two_sided(a, b, c, d),
        "expected_--_from_marginals": (c + d) * (b + d) / n,
        "expected_--_at_r_pooled": n * (1 - res["r"][2]) ** 2,
    }
    # coverage expected under independence with pooled r and observed degenerate rate
    res["G2"] = dict(g2)
    res["byN1"] = {k: (v[0], v[1], v[0] / v[1], wilson(v[0], v[1])) for k, v in byN1.items()}
    res["bins"] = {b: (v[0], v[1], v[0] / v[1], v[2] / v[1]) for b, v in sorted(bins.items())}
    # what a global sign flip of the affine constant would give
    flip_cov = sum(1 for d in curves.values() if d[0] == -1 or d[1] == -1)
    res["if_affine_sign_flipped"] = {"r": allc["twist"] / nd, "coverage": flip_cov / len(curves)}
    return res


def main(study):
    ctl = [json.loads(l) for l in open(os.path.join(study, "data", "control.jsonl"))]
    mn = [json.loads(l) for l in open(os.path.join(study, "data", "main.jsonl"))]
    out = {}
    cst, pats, null_c, mixed, ncur, wc = control(ctl)
    kappa = {p: cst[p]["kappa_p"] for p in cst}
    out["control"] = cst
    out["control_total_compared"] = sum(v["compared"] for v in cst.values())
    out["control_total_agree_with_chi(-1)"] = sum(v.get("agree_kappa_chi(-1)", 0) for v in cst.values())
    out["control_pattern_u"] = pats
    out["control_null_chi(c)_only_agree"] = sum(v for (p, ok), v in null_c.items() if ok)
    out["control_curves_with_both_smith_outcomes"] = [mixed, ncur]
    # does chi(S_L) alone (without c) track the Smith symbol?  (c cancels on both sides)
    agree_SL_only = sum(1 for p, cur, sm, ls, lc in wc if leg(-1, p) * ls * lc == sm)
    out["control_agree_SL_times_c"] = agree_SL_only
    out["control_chi(c)_constant_share"] = Counter((lc) for p, cur, sm, ls, lc in wc)
    out["main"] = main_arm(mn, kappa)
    out["combinatorics"] = combinatorics(mn)
    print(json.dumps(out, indent=1, default=lambda o: list(o) if isinstance(o, (tuple, set)) else str(o)))


if __name__ == "__main__":
    main(sys.argv[1])
