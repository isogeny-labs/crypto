#!/usr/bin/env python3
"""Validator library for the ic_positive_control studies (independent of the
producers' analyze.py). Reads raw rows only; every key read is indexed with []
so a missing key raises instead of comparing None == None (lesson
L-99ee952350b2)."""
import json
import math
import random
import statistics

IC_KEYS = ["arm", "kind", "p", "curve_idx", "seed", "order", "h", "r", "certified",
           "exhausted", "group_ops", "setup_ops", "walk_ops", "verify_ops", "rel_mul",
           "rel_add", "la_mul", "la_add", "la_graph_mul", "la_graph_add", "la_peel_add",
           "la_wied_mul", "la_wied_add", "native_mul", "native_all", "total_w0",
           "total_w01", "total_w1", "fb_cols", "trials", "splits", "relations",
           "rels_lp0", "rels_lp1", "rels_lp2", "cycles", "core_rows", "core_cols",
           "attempts"]
RHO_KEYS = ["arm", "kind", "p", "curve_idx", "seed", "order", "h", "r", "certified",
            "group_ops", "walk_ops", "setup_and_other_ops", "native_mul", "steps",
            "total_w0", "total_w01", "total_w1"]
# fields that are deterministic in the seed (everything except wall time and the
# host-calibrated total_cal)
NONDET = {"wall_ns", "total_cal"}
WEIGHTS = (0.0, 0.1, 1.0)
WKEY = {0.0: "total_w0", 0.1: "total_w01", 1.0: "total_w1"}


def load(path):
    d = json.load(open(path))
    rows = d["rows"]
    for r in rows:
        need = RHO_KEYS if r["kind"] == "rho" else IC_KEYS
        for k in need:
            r[k]  # raises KeyError if absent
    return d, rows


def index(rows):
    """{arm: {(p, curve_idx, seed): row}}, asserting uniqueness."""
    out = {}
    for r in rows:
        k = (r["p"], r["curve_idx"], r["seed"])
        a = out.setdefault(r["arm"], {})
        assert k not in a, f"duplicate row {r['arm']} {k}"
        a[k] = r
    return out


def total(r, w):
    """The instrument's definition (ic_control.rs IcResult::total, RhoOut::total):
    group_ops + w * native_mul, recomputed from the counted fields."""
    return r["group_ops"] + w * r["native_mul"]


def total_all(r, w):
    """Sensitivity: additions priced too (IC only; rho records no additions)."""
    if r["kind"] == "rho":
        return r["group_ops"] + w * r["native_mul"]
    return r["group_ops"] + w * r["native_all"]


def accounting_check(rows):
    """Per-row identities between counted fields; returns list of violations."""
    bad = []
    for r in rows:
        k = (r["arm"], r["p"], r["curve_idx"], r["seed"])
        for w in WEIGHTS:
            if abs(r[WKEY[w]] - total(r, w)) > 1e-6 * max(1.0, abs(total(r, w))):
                bad.append(f"{k} {WKEY[w]}={r[WKEY[w]]} != group_ops+w*native_mul={total(r, w)}")
        if r["kind"] == "rho":
            if r["group_ops"] != r["walk_ops"] + r["setup_and_other_ops"]:
                bad.append(f"{k} rho group_ops != walk+setup_and_other")
            if r["native_mul"] < r["steps"]:
                bad.append(f"{k} rho native_mul < steps")
        else:
            if r["native_mul"] != r["rel_mul"] + r["la_mul"]:
                bad.append(f"{k} native_mul != rel_mul+la_mul")
            if r["native_all"] != r["rel_mul"] + r["rel_add"] + r["la_mul"] + r["la_add"]:
                bad.append(f"{k} native_all != rel+la mul+add")
            if r["la_mul"] != r["la_graph_mul"] + r["la_wied_mul"]:
                bad.append(f"{k} la_mul != graph+wied mul ({r['la_mul']} vs {r['la_graph_mul'] + r['la_wied_mul']})")
            if r["la_add"] < r["la_graph_add"] + r["la_peel_add"] + r["la_wied_add"]:
                bad.append(f"{k} la_add < graph+peel+wied add")
            if r["group_ops"] != r["setup_ops"] + r["walk_ops"] + r["verify_ops"]:
                bad.append(f"{k} group_ops != setup+walk+verify")
            if r["relations"] != r["rels_lp0"] + r["rels_lp1"] + r["rels_lp2"]:
                bad.append(f"{k} relations != sum rels_lp")
            if r["exhausted"]:
                bad.append(f"{k} exhausted")
    return bad


def cell(by, arm, p):
    return [r for k, r in sorted(by[arm].items()) if k[0] == p]


def mean(v):
    return sum(v) / len(v)


def ratio_of_means(num, den, fn_num, fn_den=None):
    fn_den = fn_den or fn_num
    return mean([fn_num(r) for r in num]) / mean([fn_den(r) for r in den])


def boot(num_rows, den_rows, fn_num, fn_den=None, B=4000, seed=987654321):
    """Own two-stage bootstrap: resample curves, then seeds within each curve,
    independently for numerator and denominator rows (as the protocols
    describe). Returns (point, lo, hi) with lo/hi the 2.5/97.5 percentiles."""
    fn_den = fn_den or fn_num
    rnd = random.Random(seed)
    def groups(rows, fn):
        g = {}
        for r in rows:
            g.setdefault(r["curve_idx"], []).append(fn(r))
        return g
    gn, gd = groups(num_rows, fn_num), groups(den_rows, fn_den)
    curves = sorted(gn)
    assert curves == sorted(gd)
    point = mean([x for c in curves for x in gn[c]]) / mean([x for c in curves for x in gd[c]])
    bs = []
    for _ in range(B):
        cs = [curves[rnd.randrange(len(curves))] for _ in curves]
        n, d = [], []
        for c in cs:
            n.extend(gn[c][rnd.randrange(len(gn[c]))] for _ in gn[c])
            d.extend(gd[c][rnd.randrange(len(gd[c]))] for _ in gd[c])
        bs.append(mean(n) / mean(d))
    bs.sort()
    lo = bs[int(math.floor(0.025 * (B - 1)))]
    hi = bs[int(math.ceil(0.975 * (B - 1)))]
    return point, lo, hi


def slope(xs, ys):
    lx = [math.log(x) for x in xs]
    ly = [math.log(y) for y in ys]
    mx, my = mean(lx), mean(ly)
    return sum((a - mx) * (b - my) for a, b in zip(lx, ly)) / sum((a - mx) ** 2 for a in lx)


def compare_rows(a, b, ignore=NONDET):
    """Field-by-field equality of two rows except non-deterministic fields;
    returns list of differing keys (keys present in either)."""
    diffs = []
    for k in sorted(set(a) | set(b)):
        if k in ignore:
            continue
        if k not in a or k not in b:
            diffs.append(f"{k}: missing in {'a' if k not in a else 'b'}")
        elif a[k] != b[k]:
            diffs.append(f"{k}: {a[k]} != {b[k]}")
    return diffs


def close(claimed, actual, rel=0.0, abs_tol=None, digits=None):
    """Is a RESULTS.md number consistent with the recomputed value at the
    precision it is printed? digits = decimals printed."""
    if digits is not None:
        return abs(claimed - actual) <= 0.5 * 10 ** (-digits) + 1e-12
    if abs_tol is not None:
        return abs(claimed - actual) <= abs_tol
    return abs(claimed - actual) <= rel * abs(actual)
