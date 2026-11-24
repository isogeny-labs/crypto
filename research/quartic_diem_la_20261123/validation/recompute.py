#!/usr/bin/env python3
"""Validator's independent re-implementation of PROTOCOL.md section 6.

Written from the protocol text only; analyze.py is not imported or copied.

Usage:
  python3 -I recompute.py STUDY_DIR PARENT_RAW_DIR OUT.json

STUDY_DIR is a snapshot of research/quartic_diem_la_20261123 taken with
`git archive <validated sha>` (never the working tree).  Every record key the
script reads is indexed with [] (lesson L-99ee952350b2), so a missing key
raises instead of comparing None == None.

Section 6, as implemented here:
  R      = sqrt(pi r / 4) (ideal negation rho, nothing unpriced charged);
  GAE_w  = group_ops + w (rel_mul + rel_add + la_mul + la_add + idx_lookups
           + idx_inserts), w in {0, 0.1, 1};
  paired = GAE_w(sge) / GAE_w(sfb) per (curve, seed); curve level = geometric
           mean over the two solver seeds; per prime = geometric mean over the 5
           curves; pooled = exp(mean over primes of ln per-prime gm) (equal
           weight per prime); slope = OLS of ln(per-prime gm) on ln p (the
           design is balanced, so this equals OLS over the 20 curve values).
  CIs    = cluster bootstrap over curves (within prime for pooled and slope):
           (a) Monte Carlo, 4000 replicates, numpy default_rng(SEED_MC) -- a
               different generator and seed from the producer's, so agreement
               is up to bootstrap noise;
           (b) exact: the bootstrap distribution enumerated over all 5^5
               ordered resamples per prime (126 multisets with multinomial
               weights); pooled by exact convolution of the four per-prime
               distributions; 2.5/97.5% quantiles of that discrete law.
  verdict per prime and arm: BELOW-RHO CANDIDATE iff all three upper95 < 1,
           ABOVE RHO iff all three lower95 > 1, else UNRESOLVED.
  decision: IMPROVEMENT iff pooled upper95 < 1 at w = 1 and w = 0.1 and
           upper95 <= 1.01 at w = 0; KILL iff pooled w = 1 lower95 >= 0.90;
           else INCONCLUSIVE.  Integrity failure -> VOID.
"""
import hashlib
import itertools
import json
import math
import os
import sys
from collections import Counter, defaultdict

import numpy as np

SFB = "Q_diem_2lp_sfb"
SGE = "Q_diem_2lp_sfb_sge"
ARMS = (SFB, SGE)
WS = (0.0, 0.1, 1.0)
NATIVE = ("rel_mul", "rel_add", "la_mul", "la_add", "idx_lookups", "idx_inserts")
SEED_MC = 424242
NREP = 4000
PRIMES = (4099, 16411, 65537, 131101)
CURVES = {4099: [35, 205, 220, 257, 295], 16411: [93, 158, 252, 366, 382],
          65537: [13, 57, 156, 166, 369], 131101: [53, 177, 193, 228, 288]}
RAW = ("raw/run.jsonl", "raw/run131101.jsonl")
PARENT_RAW = ("run.jsonl", "run131101_odd.jsonl", "run131101_even.jsonl")
SPLIT = ("rows", "dsu", "peel", "tscan", "trim", "sge", "spmv", "bm", "vec")


def sha256(path):
    return hashlib.sha256(open(path, "rb").read()).hexdigest()


def load(paths):
    out = []
    for p in paths:
        with open(p) as fh:
            for line in fh:
                if line.strip():
                    out.append(json.loads(line))
    return out


def native(r):
    return sum(r[k] for k in NATIVE)


def gae(r, w):
    return r["group_ops"] + w * native(r)


def gm(xs):
    xs = list(xs)
    return math.exp(sum(math.log(x) for x in xs) / len(xs))


# ── exact cluster bootstrap ────────────────────────────────────────────────

def multisets(n):
    """All multisets of size n from range(n) with their multinomial counts."""
    out = []
    for combo in itertools.combinations_with_replacement(range(n), n):
        c = Counter(combo)
        mult = math.factorial(n)
        for v in c.values():
            mult //= math.factorial(v)
        out.append((combo, mult))
    assert sum(m for _, m in out) == n ** n
    return out


MS5 = multisets(5)


def exact_dist_mean_log(vals):
    """Exact bootstrap law of mean(ln v_resampled), 5 clusters: (atoms, probs)."""
    lv = [math.log(v) for v in vals]
    atoms = np.array([sum(lv[i] for i in combo) / 5 for combo, _ in MS5])
    probs = np.array([m for _, m in MS5], dtype=float) / 5 ** 5
    return atoms, probs


def quantile_discrete(atoms, probs, q):
    o = np.argsort(atoms, kind="stable")
    a, c = atoms[o], np.cumsum(probs[o])
    i = int(np.searchsorted(c, q - 1e-15))
    return float(a[min(i, len(a) - 1)])


def convolve(d1, d2):
    a1, p1 = d1
    a2, p2 = d2
    a = (a1[:, None] + a2[None, :]).ravel()
    p = (p1[:, None] * p2[None, :]).ravel()
    return a, p


def quantile_sum(dx, dy, q):
    """q-quantile of X + Y (independent, discrete) by bisection on the CDF."""
    ay, py = dy
    o = np.argsort(ay)
    ays, cy = ay[o], np.cumsum(py[o])
    ax, px = dx

    def cdf(t):
        idx = np.searchsorted(ays, t - ax, side="right")
        f = np.where(idx > 0, cy[np.maximum(idx - 1, 0)], 0.0)
        return float(np.sum(px * f))

    lo = float(ax.min() + ays.min()) - 1e-9
    hi = float(ax.max() + ays.max()) + 1e-9
    for _ in range(200):
        mid = (lo + hi) / 2
        if cdf(mid) >= q:
            hi = mid
        else:
            lo = mid
    return hi


def exact_pooled_ci(per_prime_vals):
    """per_prime_vals: list of 4 lists of 5 curve values. CI of exp(mean_p mean_c ln)."""
    ds = [exact_dist_mean_log(v) for v in per_prime_vals]
    d01 = convolve(ds[0], ds[1])
    d23 = convolve(ds[2], ds[3])
    lo = quantile_sum(d01, d23, 0.025)
    hi = quantile_sum(d01, d23, 0.975)
    k = len(per_prime_vals)
    return [math.exp(lo / k), math.exp(hi / k)]


def exact_prime_ci(vals):
    a, p = exact_dist_mean_log(vals)
    return [math.exp(quantile_discrete(a, p, 0.025)), math.exp(quantile_discrete(a, p, 0.975))]


# ── Monte Carlo cluster bootstrap (independent RNG) ──────────────────────

def mc_all(per_prime_vals, rng):
    """per_prime_vals: dict p -> array of 5 curve values.  Returns per-prime CIs,
    pooled CI and slope CI from one shared set of NREP within-prime resamples."""
    lnp = np.array([math.log(p) for p in PRIMES])
    reps = {}
    for p in PRIMES:
        lv = np.log(np.asarray(per_prime_vals[p], dtype=float))
        idx = rng.integers(0, len(lv), size=(NREP, len(lv)))
        reps[p] = lv[idx].mean(axis=1)
    M = np.stack([reps[p] for p in PRIMES], axis=1)  # NREP x 4 of ln gm
    pooled = np.exp(M.mean(axis=1))
    xc = lnp - lnp.mean()
    slopes = (M - M.mean(axis=1, keepdims=True)) @ xc / (xc @ xc)
    pq = lambda a: [float(np.percentile(a, 2.5)), float(np.percentile(a, 97.5))]
    return ({p: pq(np.exp(reps[p])) for p in PRIMES}, pq(pooled), pq(slopes))


def point_stats(per_prime_vals):
    lnp = np.array([math.log(p) for p in PRIMES])
    lg = np.array([math.log(gm(per_prime_vals[p])) for p in PRIMES])
    xc = lnp - lnp.mean()
    slope = float((lg - lg.mean()) @ xc / (xc @ xc))
    return ({p: gm(per_prime_vals[p]) for p in PRIMES}, math.exp(lg.mean()), slope)


def summarize(name, per_prime_vals, rng):
    pp, pooled, slope = point_stats(per_prime_vals)
    mc_pp, mc_pool, mc_slope = mc_all(per_prime_vals, rng)
    ex_pool = exact_pooled_ci([per_prime_vals[p] for p in PRIMES])
    return {
        "per_prime": {str(p): {"gm": pp[p], "ci95_mc": mc_pp[p],
                               "ci95_exact": exact_prime_ci(per_prime_vals[p]),
                               "n_curves": len(per_prime_vals[p])} for p in PRIMES},
        "pooled": {"gm": pooled, "ci95_mc": mc_pool, "ci95_exact": ex_pool},
        "slope_ln_p": {"slope": slope, "ci95_mc": mc_slope},
    }


def verdict(lows, ups):
    if all(u < 1 for u in ups):
        return "BELOW-RHO CANDIDATE"
    if all(lo > 1 for lo in lows):
        return "ABOVE RHO"
    return "UNRESOLVED"


def main():
    study, parent_raw, out_path = sys.argv[1], sys.argv[2], sys.argv[3]
    rng = np.random.default_rng(SEED_MC)
    res = {"schema": "validator-recompute/v1", "study": "research/quartic_diem_la_20261123",
           "inputs": {}, "independent_of": "analyze.py (not imported, not copied)"}

    # data.sha256 binding
    dsha = {}
    for line in open(os.path.join(study, "data.sha256")):
        h, f = line.split()
        dsha[f] = h
    for f in RAW:
        got = sha256(os.path.join(study, f))
        res["inputs"][f] = {"sha256": got, "matches_data_sha256": got == dsha[f]}
        assert got == dsha[f], f
    rows = load([os.path.join(study, f) for f in RAW])
    runs = [r for r in rows if r["kind"] == "run"]
    inst = {(r["p"], r["curve_seed"]): r for r in rows if r["kind"] == "instance"}

    # ── integrity ──
    integ = {}
    integ["n_runs"] = dict(Counter(r["arm"] for r in runs))
    integ["arms_seen"] = sorted(set(r["arm"] for r in runs))
    key = lambda r: (r["p"], r["curve_seed"], r["seed"])
    by = defaultdict(dict)
    for r in runs:
        assert r["arm"] not in by[key(r)], ("duplicate", key(r), r["arm"])
        by[key(r)][r["arm"]] = r
    expected = {(p, c, s) for p in PRIMES for c in CURVES[p] for s in (1, 2)}
    integ["grid_complete"] = set(by) == expected
    integ["pairs_complete"] = all(set(v) == set(ARMS) for v in by.values())
    bad = []
    for r in runs:
        if not (r["certified"] is True and r["ok"] is True and r["recovered"] == r["t_secret"]
                and r["exhausted"] is False):
            bad.append(key(r) + (r["arm"],))
        i = inst[(r["p"], r["curve_seed"])]
        for f in ("f", "order", "r", "h", "n_points"):
            if r[f] != i[f]:
                bad.append(key(r) + (r["arm"], "instance-field " + f))
    integ["uncertified_or_wrong_log"] = bad
    integ["same_log_both_arms"] = all(v[SFB]["recovered"] == v[SGE]["recovered"]
                                      and v[SFB]["t_secret"] == v[SGE]["t_secret"]
                                      for v in by.values())
    # same instance data on both arms (g_class, target_class, p0)
    integ["same_instance_both_arms"] = all(
        all(v[SFB][f] == v[SGE][f] for f in ("f", "p0", "r", "g_class", "target_class", "t_secret"))
        for v in by.values())
    # identical relation phase and attempt schedule (PROTOCOL section 3)
    relf = ("rel_mul", "rel_add", "idx_lookups", "idx_inserts", "relations", "trials",
            "attempts", "core_rows", "core_cols")
    integ["relation_phase_identical_fields"] = list(relf)
    integ["relation_phase_identical"] = all(all(v[SFB][f] == v[SGE][f] for f in relf)
                                            for v in by.values())
    integ["relation_phase_mismatch"] = [k for k, v in by.items()
                                        if not all(v[SFB][f] == v[SGE][f] for f in relf)]

    # C0 against the parent ladder, every shared field except wall_ns
    prow = load([os.path.join(parent_raw, f) for f in PARENT_RAW])
    P = {}
    for r in prow:
        if r["kind"] == "run" and r["arm"] == SFB:
            assert key(r) not in P
            P[key(r)] = r
    mism, nf = [], set()
    for k, v in by.items():
        a, b = v[SFB], P[k]
        common = (set(a) & set(b)) - {"wall_ns"}
        nf.add(len(common))
        d = sorted(f for f in common if a[f] != b[f])
        if d:
            mism.append([list(k), d])
    integ["C0"] = {"compared": len(by), "identical": len(by) - len(mism), "mismatches": mism,
                   "fields_compared_per_row": sorted(nf)}
    # la split identity: sources sum to la_add / la_mul
    split_ok = all(sum(r["la_%s_add" % s] for s in SPLIT) == r["la_add"]
                   and sum(r["la_%s_mul" % s] for s in SPLIT) == r["la_mul"] for r in runs)
    integ["la_split_sums_ok"] = split_ok
    # fallbacks: where, and do they coincide with multi-attempt sfb pairs
    fb = sorted(k for k, v in by.items() if v[SGE]["la_fallbacks"] > 0)
    multi = sorted(k for k, v in by.items() if v[SFB]["attempts"] > 1)
    integ["fallback_pairs"] = [list(k) for k in fb]
    integ["sfb_multi_attempt_pairs"] = [list(k) for k in multi]
    integ["fallbacks_equal_multi_attempt_pairs"] = fb == multi
    integ["fallback_total"] = sum(v[SGE]["la_fallbacks"] for v in by.values())
    integ["sfb_la_fallbacks_total"] = sum(v[SFB]["la_fallbacks"] for v in by.values())
    integ["la_skipped_total"] = sum(v[a]["la_skipped"] for v in by.values() for a in ARMS)
    dgo = {}
    for k in fb:
        v = by[k]
        dgo["%d/%d/%d" % k] = {"d_group_ops": v[SGE]["group_ops"] - v[SFB]["group_ops"],
                               "log2_r": math.log2(v[SFB]["r"])}
    integ["fallback_extra_group_ops"] = dgo
    integ["nonfallback_group_ops_equal"] = all(by[k][SGE]["group_ops"] == by[k][SFB]["group_ops"]
                                               for k in by if k not in fb)
    ok = (integ["grid_complete"] and integ["pairs_complete"] and not bad and
          integ["same_log_both_arms"] and not mism and
          integ["n_runs"] == {SFB: 40, SGE: 40})
    integ["integrity"] = ok
    res["integrity"] = integ

    # ── curve-level values ──
    def curve_vals(fn):
        out = {}
        for p in PRIMES:
            out[p] = [gm(fn(by[(p, c, s)]) for s in (1, 2)) for c in CURVES[p]]
        return out

    metrics = {
        "gae_0.0": lambda v: gae(v[SGE], 0.0) / gae(v[SFB], 0.0),
        "gae_0.1": lambda v: gae(v[SGE], 0.1) / gae(v[SFB], 0.1),
        "gae_1.0": lambda v: gae(v[SGE], 1.0) / gae(v[SFB], 1.0),
        "la_add": lambda v: v[SGE]["la_add"] / v[SFB]["la_add"],
        "la_mul": lambda v: v[SGE]["la_mul"] / v[SFB]["la_mul"],
        "la_total": lambda v: (v[SGE]["la_add"] + v[SGE]["la_mul"]) / (v[SFB]["la_add"] + v[SFB]["la_mul"]),
        "native_all": lambda v: native(v[SGE]) / native(v[SFB]),
    }
    res["paired_sge_over_sfb"] = {m: summarize(m, curve_vals(fn), rng) for m, fn in metrics.items()}

    # ── bootstrap noise: 400 independent 4000-replicate MC runs of the pooled CI ──
    nrng = np.random.default_rng(SEED_MC + 1)
    noise = {}
    for m, fn in metrics.items():
        L = {p: np.log(np.asarray(v, dtype=float)) for p, v in curve_vals(fn).items()}
        ends = []
        for _ in range(400):
            M = np.stack([L[p][nrng.integers(0, 5, size=(NREP, 5))].mean(axis=1) for p in PRIMES], axis=1)
            ends.append(np.percentile(np.exp(M.mean(axis=1)), [2.5, 97.5]))
        ends = np.array(ends)
        noise[m] = {"lo_range": [float(ends[:, 0].min()), float(ends[:, 0].max())],
                    "hi_range": [float(ends[:, 1].min()), float(ends[:, 1].max())]}
    res["bootstrap_noise_pooled_ci_400x4000"] = noise

    # ── IC / rho ──
    icr = {}
    verd = {}
    for arm in ARMS:
        icr[arm] = {}
        for w in WS:
            fn = (lambda a, ww: (lambda v: gae(v[a], ww) / math.sqrt(math.pi * v[a]["r"] / 4)))(arm, w)
            icr[arm]["w_%g" % w] = summarize("ic", curve_vals(fn), rng)
        verd[arm] = {}
        for p in PRIMES:
            lows = [icr[arm]["w_%g" % w]["per_prime"][str(p)]["ci95_mc"][0] for w in WS]
            ups = [icr[arm]["w_%g" % w]["per_prime"][str(p)]["ci95_mc"][1] for w in WS]
            lows_x = [icr[arm]["w_%g" % w]["per_prime"][str(p)]["ci95_exact"][0] for w in WS]
            ups_x = [icr[arm]["w_%g" % w]["per_prime"][str(p)]["ci95_exact"][1] for w in WS]
            verd[arm][str(p)] = {"verdict_mc": verdict(lows, ups), "verdict_exact": verdict(lows_x, ups_x),
                                 "lower95_mc": lows, "upper95_mc": ups}
    res["ic_over_rho"] = icr
    res["ic_over_rho_verdict"] = verd

    # ── decision ──
    ps = res["paired_sge_over_sfb"]
    dec = {}
    for src in ("ci95_mc", "ci95_exact"):
        u1 = ps["gae_1.0"]["pooled"][src][1]
        u01 = ps["gae_0.1"]["pooled"][src][1]
        u0 = ps["gae_0.0"]["pooled"][src][1]
        l1 = ps["gae_1.0"]["pooled"][src][0]
        if not ok:
            d = "VOID"
        elif u1 < 1 and u01 < 1 and u0 <= 1.01:
            d = "IMPROVEMENT"
        elif l1 >= 0.90:
            d = "KILL"
        else:
            d = "INCONCLUSIVE"
        dec[src] = d
    # strict section-5a order across the band (no tolerance): the pooled CI must sit
    # on the same side of 1 at w = 0, 0.1 and 1
    sides = []
    for w in ("gae_0.0", "gae_0.1", "gae_1.0"):
        lo, hi = ps[w]["pooled"]["ci95_exact"]
        sides.append("below" if hi < 1 else "above" if lo > 1 else "straddles")
    dec["strict_5a_pooled_sides_w0_w01_w1"] = sides
    dec["strict_5a_order_holds_across_band"] = len(set(sides)) == 1 and sides[0] != "straddles"
    res["decision"] = dec

    # ── crossover weight: smallest w at which the pooled point ratio is < 1 ──
    def pooled_point(w):
        vals = curve_vals(lambda v: gae(v[SGE], w) / gae(v[SFB], w))
        return point_stats(vals)[1]
    lo, hi = 0.0, 1e-3
    assert pooled_point(lo) > 1 and pooled_point(hi) < 1
    for _ in range(200):
        mid = (lo + hi) / 2
        if pooled_point(mid) < 1:
            hi = mid
        else:
            lo = mid
    pair_w = {}
    for k in fb:
        v = by[k]
        pair_w["%d/%d/%d" % k] = (v[SGE]["group_ops"] - v[SFB]["group_ops"]) / (native(v[SFB]) - native(v[SGE]))
    # counter-implied F_q multiplications per counted Pic^0 operation (not a timing calibration)
    fq_per_op = {str(p): gm(r["cantor_mul"] / r["group_ops"] for k, v in by.items() if k[0] == p
                            for r in (v[SFB],)) for p in PRIMES}
    res["band_crossover"] = {"pooled_point_ratio_equals_1_at_w": hi,
                             "per_fallback_pair_crossover_w": pair_w,
                             "pooled_point_ratio_at_w": {str(w): pooled_point(w) for w in
                                                         (0.0, 1e-6, 1e-5, 1e-4, 1e-3, 0.01, 0.1, 1.0)},
                             "counter_implied_fq_mul_per_pic_op_gm": fq_per_op,
                             "note": "cantor_mul / group_ops is a counter ratio, not a timing w_cal"}

    # ── la breakdown (descriptive) ──
    lb = {}
    for p in PRIMES:
        lb[str(p)] = {}
        for arm in ARMS:
            rs = [by[(p, c, s)][arm] for c in CURVES[p] for s in (1, 2)]
            n = len(rs)
            la_add = sum(r["la_add"] for r in rs) / n
            la_mul = sum(r["la_mul"] for r in rs) / n
            nat = sum(native(r) for r in rs) / n
            e = {"la_add_mean": la_add, "la_mul_mean": la_mul, "native_all_mean": nat,
                 "la_share_of_native": (la_add + la_mul) / nat,
                 "attempts_mean": sum(r["attempts"] for r in rs) / n,
                 "la_fallbacks": sum(r["la_fallbacks"] for r in rs),
                 "group_ops_mean": sum(r["group_ops"] for r in rs) / n,
                 "split_add_share": {s: sum(r["la_%s_add" % s] for r in rs) / (la_add * n) for s in SPLIT},
                 "split_mul_share": {s: sum(r["la_%s_mul" % s] for r in rs) / (la_mul * n) for s in SPLIT},
                 "spmv_share_of_la_total": sum(r["la_spmv_add"] + r["la_spmv_mul"] for r in rs)
                 / sum(r["la_add"] + r["la_mul"] for r in rs)}
            if arm == SGE:
                for f in ("sq_dim", "sq_nnz", "core_nnz", "core_rows", "core_cols"):
                    e[f + "_mean"] = sum(r[f] for r in rs) / n
                fbr = [r for r in rs if r["la_fallbacks"] > 0]
                tot_la = sum(r["la_add"] + r["la_mul"] for r in rs)
                tot_la_add = sum(r["la_add"] for r in rs)
                e["fallback_runs"] = len(fbr)
                e["fallback_share_of_la_total"] = sum(r["la_add"] + r["la_mul"] for r in fbr) / tot_la
                e["fallback_share_of_la_add"] = sum(r["la_add"] for r in fbr) / tot_la_add
            lb[str(p)][arm] = e
        # single-attempt pairs: la ratio (gm over pairs) and fallback pairs' la ratios
        one = [by[(p, c, s)] for c in CURVES[p] for s in (1, 2) if by[(p, c, s)][SFB]["attempts"] == 1]
        two = [by[(p, c, s)] for c in CURVES[p] for s in (1, 2) if by[(p, c, s)][SGE]["la_fallbacks"] > 0]
        lt = lambda v: (v[SGE]["la_add"] + v[SGE]["la_mul"]) / (v[SFB]["la_add"] + v[SFB]["la_mul"])
        lb[str(p)]["single_attempt_pairs"] = {"n": len(one), "la_total_ratio_gm": gm(lt(v) for v in one),
                                              "la_add_ratio_gm": gm(v[SGE]["la_add"] / v[SFB]["la_add"] for v in one)}
        if two:
            lb[str(p)]["fallback_pairs"] = {"n": len(two), "la_total_ratios": [lt(v) for v in two],
                                            "gae1_ratios": [gae(v[SGE], 1) / gae(v[SFB], 1) for v in two]}
    allfb = [by[k] for k in fb]
    lb["fallback_pairs_all"] = {
        "n": len(allfb),
        "la_total_ratio_gm": gm((v[SGE]["la_add"] + v[SGE]["la_mul"]) / (v[SFB]["la_add"] + v[SFB]["la_mul"]) for v in allfb),
        "gae1_ratio_gm": gm(gae(v[SGE], 1) / gae(v[SFB], 1) for v in allfb)}
    res["la_breakdown"] = lb
    json.dump(res, open(out_path, "w"), indent=1, sort_keys=False)
    print("integrity", ok, "decision", dec)


if __name__ == "__main__":
    main()
