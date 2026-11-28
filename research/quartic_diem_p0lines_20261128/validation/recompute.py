#!/usr/bin/env python3
"""Validator: independent recompute of research/quartic_diem_p0lines_20261128.

Written from PROTOCOL.md sections 3-5 and seeds.json.  The frozen analyze.py and the
exploratory explore.py are neither imported nor copied; their outputs (analysis.json,
explore.json) are read only at the end, to compare numbers.

Usage:
  python3 -I recompute.py SNAP OUT.json

SNAP is `git archive <sha> research/quartic_diem_p0lines_20261128
research/quartic_diem_nofb_20261126/raw` extracted.

Definitions (PROTOCOL.md section 5):
  native(r)  = rel_mul + rel_add + la_mul + la_add + idx_lookups + idx_inserts
  GAE_w(r)   = group_ops + w * native(r),  w in {0, 0.1, 1}
  paired     = GAE_w(arm) / GAE_w(nofb) per (p, curve, solver seed)
  curve      = geometric mean over the panel's seeds
  prime      = geometric mean over the 5 curves
  pooled     = geometric mean of the 4 per-prime values (equal weight per prime)
  rho        = sqrt(pi r / 4) (ideal negation rho, nothing charged at any w)
Intervals, three ways:
  (1) "producer stream": a cluster bootstrap over curves with numpy
      default_rng(20261128), 4000 replicates, consuming the generator in the order
      the protocol's analysis visits (comparison, panel, metric, prime), np.percentile
      2.5/97.5.  Written here from the protocol; it reproduces the producer's
      endpoints only if the producer used that order.
  (2) the EXACT law of the curve-cluster bootstrap: all 126 multisets of 5 curves per
      prime with multinomial weights; the pooled law is the independent sum of the four
      per-prime laws; read at the 2.5% / 97.5% generalized-inverse quantiles.
  (3) Monte Carlo noise: MC_REPS independent 4000-replicate bootstraps with fresh
      seeds; the range of their endpoints.
Every record key is read with [] (lesson L-99ee952350b2): a missing key raises.
"""
import json
import math
import sys
from collections import defaultdict
from itertools import combinations_with_replacement

import numpy as np

STUDY = "research/quartic_diem_p0lines_20261128"
PARENT_RAW = "research/quartic_diem_nofb_20261126/raw"
NOFB = "Q_diem_2lp_sfb_sge_nofb"
P0 = "Q_diem_2lp_sfb_sge_p0"
P0I3 = "Q_diem_2lp_sfb_sge_p0i3"
ARMS = (NOFB, P0, P0I3)
WS = (0.0, 0.1, 1.0)
NATIVE = ("rel_mul", "rel_add", "la_mul", "la_add", "idx_lookups", "idx_inserts")
LA_SPLITS = ("rows", "dsu", "peel", "tscan", "trim", "sge", "spmv", "bm", "vec")
PRIMARY = (5, 6)
ALLS = (1, 2, 3, 4, 5, 6)
B = 4000
BOOT_SEED = 20261128
MC_REPS = 200
KILL_LO = 0.98
# the 24 C0 fields the protocol names (PROTOCOL.md section 5 refers to the list in analyze.py)
C0_FIELDS = ("certified", "ok", "recovered", "group_ops", "cantor_mul", "cantor_inv",
             "rel_mul", "rel_add", "la_mul", "la_add", "relations", "rels_by_lp",
             "decomp_trials", "attempts", "core_rows", "core_cols", "core_nnz",
             "idx_lookups", "idx_inserts", "idx_points", "fb_samples",
             "la_fallbacks", "sq_dim", "sq_nnz")
COMPS = (("p0_over_nofb", P0, NOFB), ("p0i3_over_nofb", P0I3, NOFB), ("p0i3_over_p0", P0I3, P0))
PANELS = (("primary_seeds_5_6", PRIMARY), ("all_seeds_1_6", ALLS))


def jl(path):
    with open(path) as fh:
        return [json.loads(l) for l in fh if l.strip()]


def native(r):
    return sum(r[k] for k in NATIVE)


def gae(r, w):
    return r["group_ops"] + w * native(r)


METRICS = (("gae_0.0", lambda r: gae(r, 0.0)), ("gae_0.1", lambda r: gae(r, 0.1)),
           ("gae_1.0", lambda r: gae(r, 1.0)),
           ("la_total", lambda r: r["la_add"] + r["la_mul"]),
           ("rel_total", lambda r: r["rel_add"] + r["rel_mul"]),
           ("native_all", native))


def gmean(xs):
    xs = list(xs)
    return math.exp(sum(math.log(x) for x in xs) / len(xs))


# ---------- exact cluster bootstrap ----------
def exact_law_prime(logs):
    """all multisets of len(logs) curves drawn with replacement: (values, probabilities)"""
    n = len(logs)
    vals, probs = [], []
    for ms in combinations_with_replacement(range(n), n):
        cnt = np.bincount(ms, minlength=n)
        w = math.factorial(n)
        for c in cnt:
            w //= math.factorial(int(c))
        vals.append(sum(c * logs[i] for i, c in enumerate(cnt)) / n)
        probs.append(w / n ** n)
    return np.array(vals), np.array(probs)


def conv(a, b):
    va, pa_ = a
    vb, pb = b
    return (va[:, None] + vb[None, :]).ravel(), (pa_[:, None] * pb[None, :]).ravel()


def ginv_quantile(vals, probs, q):
    o = np.argsort(vals, kind="stable")
    v, c = vals[o], np.cumsum(probs[o])
    k = int(np.searchsorted(c, q - 1e-15))
    return float(v[min(k, len(v) - 1)])


def quantile_sum(A, Bl, q):
    """generalized-inverse q-quantile of X + Y, X ~ A, Y ~ Bl independent (finite laws)"""
    vb_o = np.argsort(Bl[0], kind="stable")
    vb, cb = Bl[0][vb_o], np.cumsum(Bl[1][vb_o])
    va, pa_ = A

    def cdf(t):
        k = np.searchsorted(vb, t - va, side="right")
        return float(np.sum(pa_ * np.where(k > 0, cb[np.maximum(k - 1, 0)], 0.0)))
    lo = float(va.min() + vb.min()) - 1e-9
    hi = float(va.max() + vb.max()) + 1e-9
    for _ in range(200):
        mid = 0.5 * (lo + hi)
        if cdf(mid) >= q - 1e-15:
            hi = mid
        else:
            lo = mid
        if hi - lo < 1e-15:
            break
    return hi


def exact_summary(per_prime):
    """per_prime: {p: [curve values]} -> per-prime and pooled exact 95% intervals"""
    laws, out = {}, {"per_prime": {}, "pooled": None}
    for p in sorted(per_prime):
        logs = [math.log(x) for x in per_prime[p]]
        laws[p] = exact_law_prime(logs)
        out["per_prime"][str(p)] = {
            "gm": gmean(per_prime[p]),
            "exact95": [math.exp(ginv_quantile(*laws[p], 0.025)), math.exp(ginv_quantile(*laws[p], 0.975))]}
    ps = sorted(per_prime)
    k = len(ps)
    pooled = math.exp(sum(math.log(out["per_prime"][str(p)]["gm"]) for p in ps) / k)
    A = conv(laws[ps[0]], laws[ps[1]])
    Bl = conv(laws[ps[2]], laws[ps[3]])
    out["pooled"] = {"gm": pooled,
                     "exact95": [math.exp(quantile_sum(A, Bl, 0.025) / k), math.exp(quantile_sum(A, Bl, 0.975) / k)]}
    return out


def mc_summary(per_prime, rng):
    """one cluster bootstrap over curves (B replicates), np.percentile endpoints"""
    logs_b, out = [], {"per_prime": {}}
    for p in sorted(per_prime):
        v = np.log(np.asarray(per_prime[p], dtype=float))
        idx = rng.integers(0, len(v), size=(B, len(v)))
        lb = v[idx].mean(axis=1)
        logs_b.append(lb)
        b = np.exp(lb)  # percentiles of the ratio itself (linear interpolation on the ratio scale)
        out["per_prime"][str(p)] = [float(np.percentile(b, 2.5)), float(np.percentile(b, 97.5))]
    pooled = np.exp(np.mean(logs_b, axis=0))
    out["pooled"] = [float(np.percentile(pooled, 2.5)), float(np.percentile(pooled, 97.5))]
    return out


def rule(up, lo1):
    """PROTOCOL.md section 5 decision rule"""
    if up[1.0] < 1 and up[0.1] < 1 and up[0.0] <= 1:
        return "IMPROVEMENT"
    if lo1 >= KILL_LO:
        return "KILL"
    return "INCONCLUSIVE"


def main():
    snap, outp = sys.argv[1], sys.argv[2]
    st = f"{snap}/{STUDY}"
    seeds = json.load(open(f"{st}/seeds.json"))
    panel = {int(p): v for p, v in seeds["panel"]["curve_seeds"].items()}
    out = {"schema": "validator-recompute/v1", "study": STUDY}

    rows = jl(f"{st}/raw/run.jsonl") + jl(f"{st}/raw/run131101.jsonl")
    runs = {}
    for r in rows:
        if r["kind"] != "run":
            continue
        k = (r["arm"], r["p"], r["curve_seed"], r["seed"])
        assert k not in runs, ("duplicate", k)
        runs[k] = r
    expected = {(a, p, c, s) for a in ARMS for p in panel for c in panel[p] for s in ALLS}
    out["completeness"] = {"runs": len(runs), "expected": len(expected),
                           "missing": sorted(map(list, expected - set(runs))),
                           "extra": sorted(map(list, set(runs) - expected))}
    assert set(runs) == expected

    # ---------- integrity ----------
    integ = {}
    integ["certified_ok_recovered_eq_t"] = sum(1 for r in runs.values()
                                               if r["certified"] is True and r["ok"] is True
                                               and r["recovered"] == r["t_secret"])
    integ["exhausted_any"] = sum(1 for r in runs.values() if r["exhausted"])
    same = 0
    for p in panel:
        for c in panel[p]:
            for s in ALLS:
                same += len({runs[(a, p, c, s)]["recovered"] for a in ARMS}) == 1
    integ["same_log_all_arms_pairs"] = same
    integ["p0_rows_with_p0_line_relations_gt0"] = sum(1 for k, r in runs.items() if k[0] != NOFB and r["p0_line_relations"] > 0)
    integ["nofb_rows_p0_lines_eq0_and_rel_eq0"] = sum(1 for k, r in runs.items() if k[0] == NOFB and r["p0_lines"] == 0 and r["p0_line_relations"] == 0)
    # P0 block structure: one P0 line per factor-base point (p0_lines = fb_cols - 1 = ceil(qc sqrt p)),
    # deterministic (same per curve across arms and seeds), relations <= lines
    struct = {"p0_lines_eq_fb_cols_minus_1": 0, "p0_lines_eq_ceil_3sqrtp": 0, "rel_le_lines": 0,
              "rels_line_ge_p0_rel": 0, "p0_counts_constant_per_curve": 0, "curves": 0,
              "lines_tried_eq_trials": 0}
    p0_means = {}
    for p in panel:
        lines, rels_ = [], []
        for c in panel[p]:
            vals = set()
            for a in (P0, P0I3):
                for s in ALLS:
                    r = runs[(a, p, c, s)]
                    struct["p0_lines_eq_fb_cols_minus_1"] += r["p0_lines"] == r["fb_cols"] - 1
                    struct["p0_lines_eq_ceil_3sqrtp"] += r["p0_lines"] == math.ceil(3.0 * math.sqrt(p))
                    struct["rel_le_lines"] += r["p0_line_relations"] <= r["p0_lines"]
                    struct["rels_line_ge_p0_rel"] += r["rels_line"] >= r["p0_line_relations"]
                    vals.add((r["p0_lines"], r["p0_line_relations"]))
                    if a == P0:
                        lines.append(r["p0_lines"])
                        rels_.append(r["p0_line_relations"])
            struct["curves"] += 1
            struct["p0_counts_constant_per_curve"] += len(vals) == 1
        p0_means[str(p)] = {"p0_lines_mean": sum(lines) / len(lines), "p0_line_relations_mean": sum(rels_) / len(rels_)}
    for r in runs.values():
        struct["lines_tried_eq_trials"] += r["lines_tried"] == r["trials"]
    integ["p0_structure"] = struct
    integ["p0_means_p0_arm_all_seeds"] = p0_means
    # counter identities (section 5a fields): la = sum of its splits; rel >= enum + decomp parts
    ident = {"la_mul_eq_splits": 0, "la_add_eq_splits": 0, "rel_ge_parts": 0, "n": len(runs)}
    for r in runs.values():
        ident["la_mul_eq_splits"] += r["la_mul"] == sum(r[f"la_{s}_mul"] for s in LA_SPLITS)
        ident["la_add_eq_splits"] += r["la_add"] == sum(r[f"la_{s}_add"] for s in LA_SPLITS)
        ident["rel_ge_parts"] += (r["rel_mul"] >= r["rel_enum_mul"] + r["rel_decomp_mul"]
                                  and r["rel_add"] >= r["rel_enum_add"] + r["rel_decomp_add"])
    integ["counter_identities"] = ident

    # ---------- C0 ----------
    par = jl(f"{snap}/{PARENT_RAW}/run.jsonl") + jl(f"{snap}/{PARENT_RAW}/run131101.jsonl")
    pruns = {(r["p"], r["curve_seed"], r["seed"]): r for r in par if r["kind"] == "run" and r["arm"] == NOFB}
    c0 = {"parent_rows": len(pruns), "compared": 0, "identical_24": 0, "identical_all_common_keys": 0,
          "key_sets": defaultdict(int), "mismatch": []}
    for k, ref in sorted(pruns.items()):
        a = runs[(NOFB,) + k]
        c0["compared"] += 1
        d24 = [f for f in C0_FIELDS if ref[f] != a[f]]
        common = (set(ref) & set(a)) - {"wall_ns"}
        dall = sorted(f for f in common if ref[f] != a[f])
        c0["identical_24"] += not d24
        c0["identical_all_common_keys"] += not dall
        c0["key_sets"][json.dumps({"only_parent": sorted(set(ref) - set(a)), "only_study": sorted(set(a) - set(ref))})] += 1
        if d24 or dall:
            c0["mismatch"].append({"key": list(k), "d24": d24, "dall": dall})
    c0["parent_keys"] = sorted({(p, s) for (p, c, s) in pruns})
    c0["key_sets"] = dict(c0["key_sets"])
    c0["parent_seeds"] = sorted({s for (_, _, s) in pruns})
    integ["C0"] = c0
    out["integrity"] = integ

    # ---------- attempts (all raw rows, no exclusion: lesson L-3013ef925e8d) ----------
    att = {}
    for pname, ss in PANELS + (("seeds_1_4", (1, 2, 3, 4)),):
        att[pname] = {}
        for a in ARMS:
            per = {str(p): [sum(1 for c in panel[p] for s in ss if runs[(a, p, c, s)]["attempts"] > 1),
                            len(panel[p]) * len(ss)] for p in sorted(panel)}
            att[pname][a] = {"multi": sum(v[0] for v in per.values()), "runs": sum(v[1] for v in per.values()),
                             "per_prime": per,
                             "max_attempts": max(runs[(a, p, c, s)]["attempts"] for p in panel for c in panel[p] for s in ss)}
    out["attempts"] = att

    # ---------- paired ratios ----------
    def per_curve(num, den, ss, f):
        res = {}
        for p in sorted(panel):
            res[p] = [gmean(f(runs[(num, p, c, s)]) / f(runs[(den, p, c, s)]) for s in ss) for c in panel[p]]
        return res

    rng = np.random.default_rng(BOOT_SEED)  # producer-stream replication (see docstring)
    paired = {}
    for cname, num, den in COMPS:
        paired[cname] = {}
        for pname, ss in PANELS:
            paired[cname][pname] = {}
            for mname, f in METRICS:
                pc = per_curve(num, den, ss, f)
                ex = exact_summary(pc)
                mc = mc_summary(pc, rng)
                ex["mc_producer_stream"] = mc
                ex["per_curve"] = {str(p): v for p, v in pc.items()}
                paired[cname][pname][mname] = ex
    out["paired"] = paired

    # ---------- IC / ideal negation rho ----------
    icr, ver = {}, {}
    for a in ARMS:
        icr[a], ver[a] = {}, {}
        for w in WS:
            pc = {p: [gmean(gae(runs[(a, p, c, s)], w) / math.sqrt(math.pi * runs[(a, p, c, s)]["r"] / 4)
                            for s in ALLS) for c in panel[p]] for p in sorted(panel)}
            ex = exact_summary(pc)
            ex["mc_producer_stream"] = mc_summary(pc, rng)
            icr[a][f"all_{w}"] = ex
        for p in sorted(panel):
            for src in ("exact95", "mc_producer_stream"):
                if src == "exact95":
                    ups = [icr[a][f"all_{w}"]["per_prime"][str(p)]["exact95"][1] for w in WS]
                    los = [icr[a][f"all_{w}"]["per_prime"][str(p)]["exact95"][0] for w in WS]
                else:
                    ups = [icr[a][f"all_{w}"]["mc_producer_stream"]["per_prime"][str(p)][1] for w in WS]
                    los = [icr[a][f"all_{w}"]["mc_producer_stream"]["per_prime"][str(p)][0] for w in WS]
                v = ("BELOW-RHO CANDIDATE" if max(ups) < 1 else "ABOVE RHO" if min(los) > 1 else "UNRESOLVED")
                ver[a].setdefault(str(p), {})[src] = {"verdict": v, "upper95": ups, "lower95": los,
                                                      "below_at": [w for w, u in zip(WS, ups) if u < 1],
                                                      "above_at": [w for w, l in zip(WS, los) if l > 1]}
    out["ic_over_rho"] = icr
    out["ic_over_rho_verdict"] = ver

    # ---------- decision ----------
    dec = {}
    for cname in ("p0_over_nofb", "p0i3_over_nofb"):
        prim = paired[cname]["primary_seeds_5_6"]
        up_x = {w: prim[f"gae_{w}"]["pooled"]["exact95"][1] for w in WS}
        lo_x = prim["gae_1.0"]["pooled"]["exact95"][0]
        up_m = {w: prim[f"gae_{w}"]["mc_producer_stream"]["pooled"][1] for w in WS}
        lo_m = prim["gae_1.0"]["mc_producer_stream"]["pooled"][0]
        dec[cname] = {"exact_law": {"verdict": rule(up_x, lo_x), "upper95": {str(w): up_x[w] for w in WS}, "lower95_w1": lo_x},
                      "producer_stream": {"verdict": rule(up_m, lo_m), "upper95": {str(w): up_m[w] for w in WS}, "lower95_w1": lo_m},
                      "points": {str(w): prim[f"gae_{w}"]["pooled"]["gm"] for w in WS}}
    # MC noise of a single 4000-replicate interval (fresh seeds), decision quantities only
    noise = {}
    for cname in ("p0_over_nofb", "p0i3_over_nofb"):
        noise[cname] = {}
        for pname, ss in PANELS:
            noise[cname][pname] = {}
            num = P0 if cname.startswith("p0_") else P0I3
            for w in WS:
                pc = per_curve(num, NOFB, ss, lambda r, w=w: gae(r, w))
                los, ups = [], []
                for k in range(MC_REPS):
                    m = mc_summary(pc, np.random.default_rng(10_000 + k))
                    los.append(m["pooled"][0])
                    ups.append(m["pooled"][1])
                noise[cname][pname][str(w)] = {"lower_range": [min(los), max(los)], "upper_range": [min(ups), max(ups)]}
    out["mc_noise_pooled"] = noise
    verdicts_noise = {}
    for cname in noise:
        n = noise[cname]["primary_seeds_5_6"]
        # can any single MC interval flip the verdict?  IMPROVEMENT needs max upper < 1 ...; KILL needs lo1 >= 0.98
        verdicts_noise[cname] = {"improvement_possible": n["1.0"]["upper_range"][0] < 1 and n["0.1"]["upper_range"][0] < 1
                                 and n["0.0"]["upper_range"][0] <= 1,
                                 "kill_possible": n["1.0"]["lower_range"][1] >= KILL_LO}
    dec["mc_noise_can_flip"] = verdicts_noise
    out["decision"] = dec

    # ---------- exploratory mechanism table (explore.py's classes, recomputed) ----------
    expl = {}
    for p in sorted(panel):
        cls = defaultdict(list)
        for c in panel[p]:
            for s in ALLS:
                rn, rp = runs[(NOFB, p, c, s)], runs[(P0, p, c, s)]
                cls[f"nofb{min(rn['attempts'], 2)}_p0{min(rp['attempts'], 2)}"].append(gae(rp, 1.0) / gae(rn, 1.0))
        expl[str(p)] = {k: {"n": len(v), "gm": gmean(v), "min": min(v), "max": max(v)} for k, v in sorted(cls.items())}
    out["explore"] = expl

    # ---------- compare with analysis.json / explore.json ----------
    A = json.load(open(f"{st}/analysis.json"))
    E = json.load(open(f"{st}/explore.json"))
    cmp_ = {"points": [], "producer_ci_vs_stream": [], "producer_ci_vs_exact": [], "producer_ci_vs_noise": []}

    def rel(a, b):
        return abs(a - b) / max(abs(b), 1e-300)
    for cname, _, _ in COMPS:
        for pname, _ in PANELS:
            for mname, _ in METRICS:
                mine = paired[cname][pname][mname]
                th = A["paired"][cname][pname][mname]
                cmp_["points"].append(rel(th["pooled"]["gm"], mine["pooled"]["gm"]))
                cmp_["producer_ci_vs_stream"].append(max(rel(th["pooled"]["ci95"][i], mine["mc_producer_stream"]["pooled"][i]) for i in (0, 1)))
                cmp_["producer_ci_vs_exact"].append(max(rel(th["pooled"]["ci95"][i], mine["pooled"]["exact95"][i]) for i in (0, 1)))
                for p in th["per_prime"]:
                    cmp_["points"].append(rel(th["per_prime"][p]["gm"], mine["per_prime"][p]["gm"]))
                    cmp_["producer_ci_vs_stream"].append(max(rel(th["per_prime"][p]["ci95"][i], mine["mc_producer_stream"]["per_prime"][p][i]) for i in (0, 1)))
                    cmp_["producer_ci_vs_exact"].append(max(rel(th["per_prime"][p]["ci95"][i], mine["per_prime"][p]["exact95"][i]) for i in (0, 1)))
    for a in ARMS:
        for w in WS:
            mine, th = icr[a][f"all_{w}"], A["ic_over_rho"][a][f"all_{w}"]
            cmp_["points"].append(rel(th["pooled"]["gm"], mine["pooled"]["gm"]))
            cmp_["producer_ci_vs_stream"].append(max(rel(th["pooled"]["ci95"][i], mine["mc_producer_stream"]["pooled"][i]) for i in (0, 1)))
            for p in th["per_prime"]:
                cmp_["points"].append(rel(th["per_prime"][p]["gm"], mine["per_prime"][p]["gm"]))
                cmp_["producer_ci_vs_stream"].append(max(rel(th["per_prime"][p]["ci95"][i], mine["mc_producer_stream"]["per_prime"][p][i]) for i in (0, 1)))
                cmp_["producer_ci_vs_exact"].append(max(rel(th["per_prime"][p]["ci95"][i], mine["per_prime"][p]["exact95"][i]) for i in (0, 1)))
    for cname in noise:
        for pname in noise[cname]:
            for w in WS:
                th = A["paired"][cname][pname][f"gae_{w}"]["pooled"]["ci95"]
                n = noise[cname][pname][str(w)]
                cmp_["producer_ci_vs_noise"].append({"cmp": cname, "panel": pname, "w": w, "producer": th,
                                                     "lower_in": n["lower_range"][0] <= th[0] <= n["lower_range"][1],
                                                     "upper_in": n["upper_range"][0] <= th[1] <= n["upper_range"][1]})
    summary = {"max_rel_point": max(cmp_["points"]),
               "max_rel_ci_vs_producer_stream": max(cmp_["producer_ci_vs_stream"]),
               "max_rel_ci_vs_exact": max(cmp_["producer_ci_vs_exact"]),
               "producer_ci_inside_mc_noise": all(x["lower_in"] and x["upper_in"] for x in cmp_["producer_ci_vs_noise"]),
               "n_points_compared": len(cmp_["points"])}
    # integrity / attempts / decision fields of analysis.json
    summary["analysis_n_runs"] = A["n_runs"] == {a: 120 for a in ARMS}
    summary["analysis_integrity_fields"] = (A["uncertified_or_wrong_log"] == [] and A["unpaired"] == [] and A["same_log_all_arms"] is True
                                            and A["p0_rows_have_p0_relations"] is True and A["nofb_rows_have_no_p0_lines"] is True
                                            and A["C0_reproduction"]["compared"] == c0["compared"]
                                            and A["C0_reproduction"]["identical"] == c0["identical_24"]
                                            and A["C0_reproduction"]["mismatches"] == [])
    am = True
    for pname in ("primary_seeds_5_6", "all_seeds_1_6"):
        for a in ARMS:
            th = A["attempts"][pname][a]
            mine = att[pname][a]
            am &= th["multi_attempt"] == mine["multi"] and th["runs"] == mine["runs"]
            am &= all(th["per_prime"][p] == mine["per_prime"][p] for p in mine["per_prime"])
    summary["analysis_attempts_equal"] = am
    summary["analysis_decision"] = {"verdict_p0": A["decision"]["verdict_p0"], "verdict_p0i3_secondary": A["decision"]["verdict_p0i3_secondary"],
                                    "integrity": A["decision"]["integrity"]}
    summary["analysis_ic_verdicts"] = {a: {p: A["ic_over_rho_verdict"][a][p]["verdict"] for p in A["ic_over_rho_verdict"][a]} for a in ARMS}
    ee = []
    for p in expl:
        for k, v in expl[p].items():
            ee.append(E[p][k]["n"] == v["n"] and E[p][k]["gm_gae1_ratio"] == round(v["gm"], 3))
        ee.append(set(E[p]) - {"p0_lines", "p0_line_relations"} == set(expl[p]))
        ee.append(E[p]["p0_lines"] == p0_means[p]["p0_lines_mean"] and abs(E[p]["p0_line_relations"] - p0_means[p]["p0_line_relations_mean"]) < 1e-12)
    summary["explore_json_equal"] = all(ee)
    out["compare"] = {"summary": summary, "producer_ci_vs_noise": cmp_["producer_ci_vs_noise"]}
    with open(outp, "w") as fh:
        json.dump(out, fh, indent=1, default=float)
        fh.write("\n")
    print(json.dumps(summary, indent=1))


if __name__ == "__main__":
    main()
