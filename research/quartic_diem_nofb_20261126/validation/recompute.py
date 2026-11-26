#!/usr/bin/env python3
"""Validator: independent recompute of research/quartic_diem_nofb_20261126.

Written from PROTOCOL.md section 5 (and seeds.json) alone.  The frozen analyze.py is
neither imported nor copied; its output (analysis.json) is only read at the end, to
compare numbers.

Usage:
  python3 -I recompute.py STUDY_DIR PARENT_RAW_DIR OUT.json

STUDY_DIR is a `git archive` snapshot of the study at the validated commit;
PARENT_RAW_DIR is research/quartic_diem_la_20261123/raw from the same snapshot.

Definitions (PROTOCOL.md section 5):
  native(r)  = rel_mul + rel_add + la_mul + la_add + idx_lookups + idx_inserts
  GAE_w(r)   = group_ops + w * native(r),  w in {0, 0.1, 1}
  paired     = GAE_w(nofb) / GAE_w(sge) per (p, curve, solver seed)
  curve      = geometric mean over the panel's seeds
  prime      = geometric mean over the 5 curves
  pooled     = geometric mean of the 4 per-prime values (equal weight per prime)
  rho        = sqrt(pi r / 4) (ideal negation rho, nothing charged)
Intervals: the EXACT law of the curve-cluster bootstrap (all 126 multisets of 5
curves per prime with multinomial weights; the pooled law is the independent sum of
the four per-prime laws), read at the 2.5% and 97.5% generalized-inverse quantiles.
A Monte Carlo study (independent 4000-replicate bootstraps with fresh seeds) gives the
noise range of a single 4000-replicate interval, for comparison with the producer's.
Every key is read with [] (lesson L-99ee952350b2): a missing key raises.
"""
import json
import math
import sys
import zlib
from collections import defaultdict

import numpy as np

SGE = "Q_diem_2lp_sfb_sge"
NOFB = "Q_diem_2lp_sfb_sge_nofb"
ARMS = (SGE, NOFB)
WS = (0.0, 0.1, 1.0)
PANEL = {4099: [35, 205, 220, 257, 295], 16411: [93, 158, 252, 366, 382],
         65537: [13, 57, 156, 166, 369], 131101: [53, 177, 193, 228, 288]}
SEEDS = (1, 2, 3, 4)
NATIVE = ("rel_mul", "rel_add", "la_mul", "la_add", "idx_lookups", "idx_inserts")
LA_SPLITS = ("rows", "dsu", "peel", "tscan", "trim", "sge", "spmv", "bm", "vec")
WALL = ("wall_ns",)
MC_REPS = 300


def read_jsonl(path):
    with open(path) as fh:
        return [json.loads(l) for l in fh if l.strip()]


def native(r):
    return sum(r[k] for k in NATIVE)


def gae(r, w):
    return r["group_ops"] + w * native(r)


def gmean(xs):
    xs = list(xs)
    return math.exp(sum(math.log(x) for x in xs) / len(xs))


# ---------- exact cluster bootstrap ----------

def compositions(n, k):
    """all (c_1..c_k) >= 0 with sum n"""
    if k == 1:
        yield (n,)
        return
    for i in range(n + 1):
        for rest in compositions(n - i, k - 1):
            yield (i,) + rest


def exact_law_prime(logs):
    """support (mean of resampled logs) and probabilities of the bootstrap of one prime"""
    n = len(logs)
    vals, probs = [], []
    nf = math.factorial(n)
    for c in compositions(n, n):
        w = nf
        for ci in c:
            w //= math.factorial(ci)
        vals.append(sum(ci * v for ci, v in zip(c, logs)) / n)
        probs.append(w / n ** n)
    return np.array(vals), np.array(probs)


def combine(a, b):
    va, pa = a
    vb, pb = b
    return (va[:, None] + vb[None, :]).ravel(), (pa[:, None] * pb[None, :]).ravel()


def quantiles_sum(A, B, qs, scale):
    """generalized-inverse quantiles of scale*(A+B), A, B independent discrete laws"""
    va, pa = A
    order = np.argsort(vb := B[0])
    vb_s, cb = vb[order], np.cumsum(B[1][order])

    def cdf(x):
        idx = np.searchsorted(vb_s, x - va, side="right")
        got = np.where(idx > 0, cb[np.maximum(idx - 1, 0)], 0.0)
        return float(np.sum(pa * got))

    lo_all, hi_all = va.min() + vb.min(), va.max() + vb.max()
    out = []
    for q in qs:
        lo, hi = lo_all - 1e-12, hi_all + 1e-12
        for _ in range(200):
            mid = (lo + hi) / 2
            if cdf(mid) >= q - 1e-12:
                hi = mid
            else:
                lo = mid
            if hi - lo < 1e-15 * max(1.0, abs(hi)):
                break
        out.append(math.exp(scale * hi))
    return out


def quantiles_single(law, qs):
    v, p = law
    o = np.argsort(v)
    v, c = v[o], np.cumsum(p[o])
    return [math.exp(float(v[np.searchsorted(c, q - 1e-12)])) for q in qs]


def summarize_exact(per_prime_vals):
    """per_prime_vals: {p: [5 curve-level values]} -> point + exact intervals"""
    laws, per = {}, {}
    for p in sorted(per_prime_vals):
        logs = [math.log(x) for x in per_prime_vals[p]]
        laws[p] = exact_law_prime(logs)
        per[str(p)] = {"gm": gmean(per_prime_vals[p]),
                       "exact95": quantiles_single(laws[p], (0.025, 0.975)),
                       "n_curves": len(logs)}
    ps = sorted(laws)
    pooled = math.exp(sum(math.log(per[str(p)]["gm"]) for p in ps) / len(ps))
    A = combine(laws[ps[0]], laws[ps[1]])
    B = combine(laws[ps[2]], laws[ps[3]])
    ex = quantiles_sum(A, B, (0.025, 0.975), 1.0 / len(ps))
    return {"per_prime": per, "pooled": {"gm": pooled, "exact95": ex}}


def mc_noise(per_prime_vals, reps, seed0):
    """range of the endpoints of `reps` independent 4000-replicate bootstraps
    (numpy percentile, the producer's estimator family, fresh generator per rep)"""
    ps = sorted(per_prime_vals)
    logs = {p: np.log(np.asarray(per_prime_vals[p], float)) for p in ps}
    lo_p, hi_p = defaultdict(list), defaultdict(list)
    lo, hi = [], []
    for k in range(reps):
        g = np.random.default_rng([seed0, k])
        boots = []
        for p in ps:
            v = logs[p]
            idx = g.integers(0, len(v), size=(4000, len(v)))
            m = v[idx].mean(axis=1)
            boots.append(m)
            lo_p[p].append(float(np.exp(np.percentile(m, 2.5))))
            hi_p[p].append(float(np.exp(np.percentile(m, 97.5))))
        pm = np.mean(boots, axis=0)
        lo.append(float(np.exp(np.percentile(pm, 2.5))))
        hi.append(float(np.exp(np.percentile(pm, 97.5))))
    return {"pooled": {"lo_range": [min(lo), max(lo)], "hi_range": [min(hi), max(hi)]},
            "per_prime": {str(p): {"lo_range": [min(lo_p[p]), max(lo_p[p])],
                                   "hi_range": [min(hi_p[p]), max(hi_p[p])]} for p in ps}}


# ---------- main ----------

def main():
    st, parent_raw, outp = sys.argv[1], sys.argv[2], sys.argv[3]
    rows = read_jsonl(f"{st}/raw/run.jsonl") + read_jsonl(f"{st}/raw/run131101.jsonl")
    out = {"schema": "validator-recompute/v1", "study": "research/quartic_diem_nofb_20261126"}

    runs, skips, insts = {}, {}, {}
    kinds = defaultdict(int)
    for r in rows:
        kinds[r["kind"]] += 1
        if r["kind"] == "run":
            k = (r["arm"], r["p"], r["curve_seed"], r["seed"])
            assert k not in runs, ("duplicate", k)
            runs[k] = r
        elif r["kind"] == "skip":
            k = (r["p"], r["curve_seed"], r["seed"])
            assert k not in skips
            skips[k] = r
        elif r["kind"] == "instance":
            k = (r["p"], r["curve_seed"])
            assert k not in insts
            insts[k] = r
    out["row_kinds"] = dict(kinds)

    # ---- completeness ----
    want = {(a, p, cs, s) for a in ARMS for p, css in PANEL.items() for cs in css for s in SEEDS}
    out["completeness"] = {"expected_runs": len(want), "found_runs": len(runs),
                           "missing": sorted(map(list, want - set(runs))),
                           "unexpected": sorted(map(list, set(runs) - want)),
                           "instances": len(insts), "skips": len(skips)}

    # ---- certificates as recorded (independent arithmetic is verify_certificates.py) ----
    bad = [list(k) for k, r in runs.items()
           if not (r["certified"] is True and r["ok"] is True and r["recovered"] == r["t_secret"]
                   and r["exhausted"] is False)]
    out["recorded_certified_all"] = {"bad": bad, "ok": not bad}

    # ---- split consistency: la_{k}_{add,mul} sum to la_add / la_mul ----
    split_bad = [list(k) for k, r in runs.items()
                 if sum(r[f"la_{s}_add"] for s in LA_SPLITS) != r["la_add"]
                 or sum(r[f"la_{s}_mul"] for s in LA_SPLITS) != r["la_mul"]]
    out["la_split_sums"] = {"bad": split_bad, "ok": not split_bad}

    # ---- pair identities, field by field ----
    keys = sorted({k[1:] for k in runs})
    pair_rows = []
    allowed_fb = {"arm", "wall_ns", "group_ops", "cantor_mul", "cantor_inv", "la_add", "la_mul",
                  "la_spmv_add", "la_spmv_mul", "la_bm_add", "la_bm_mul", "la_vec_add", "la_vec_mul",
                  "la_fallbacks", "la_fallbacks_skipped"}
    viol = []
    dominance_bad = []
    for k in keys:
        a, b = runs[(SGE,) + k], runs[(NOFB,) + k]
        assert set(a) == set(b), ("key sets differ", k)
        diff = sorted(f for f in a if a[f] != b[f])
        fb = a["la_fallbacks"]
        ent = {"p": k[0], "curve_seed": k[1], "seed": k[2], "sge_fallbacks": fb,
               "sge_rescues": a["la_fallback_rescues"], "nofb_skipped": b["la_fallbacks_skipped"],
               "sge_attempts": a["attempts"], "nofb_attempts": b["attempts"],
               "differing_fields": [f for f in diff if f not in ("arm", "wall_ns")]}
        if fb == 0:
            ok = set(diff) <= {"arm", "wall_ns"}
        else:
            ok = (set(diff) <= allowed_fb and b["la_fallbacks"] == 0 and a["la_fallbacks_skipped"] == 0
                  and b["la_fallbacks_skipped"] == fb
                  and a["la_add"] + a["la_mul"] > b["la_add"] + b["la_mul"]
                  and b["group_ops"] <= a["group_ops"])
            ent["la_total_ratio"] = (b["la_add"] + b["la_mul"]) / (a["la_add"] + a["la_mul"])
            ent["la_add_ratio"] = b["la_add"] / a["la_add"]
            ent["la_mul_ratio"] = b["la_mul"] / a["la_mul"]
            ent["extra_group_ops_sge"] = a["group_ops"] - b["group_ops"]
            ent["log2_r"] = math.log2(a["r"])
            ent["extra_ops_per_fallback_over_log2r"] = (a["group_ops"] - b["group_ops"]) / fb / math.log2(a["r"])
            ent["fallback_share_of_sge_la"] = 1 - ent["la_total_ratio"]
            ent["gae1_ratio"] = gae(b, 1.0) / gae(a, 1.0)
            ent["gae0_ratio"] = gae(b, 0.0) / gae(a, 0.0)
        ent["identity_ok"] = ok
        # component-wise dominance: nofb <= sge in group_ops and every native counter
        dom = all(b[f] <= a[f] for f in ("group_ops",) + NATIVE)
        ent["nofb_dominates"] = dom
        if not ok:
            viol.append(list(k))
        if not dom:
            dominance_bad.append(list(k))
        pair_rows.append(ent)
    fbp = [e for e in pair_rows if e["sge_fallbacks"] > 0]
    out["pairs"] = {
        "n_pairs": len(pair_rows),
        "identity_violations": viol,
        "dominance_violations": dominance_bad,
        "fallback_pairs": len(fbp),
        "fallbacks_total": sum(e["sge_fallbacks"] for e in pair_rows),
        "rescues_total_counter": sum(e["sge_rescues"] for e in pair_rows),
        "skipped_total": sum(e["nofb_skipped"] for e in pair_rows),
        "attempts_differ": [[e["p"], e["curve_seed"], e["seed"]] for e in pair_rows
                            if e["sge_attempts"] != e["nofb_attempts"]],
        "fallbacks_by_prime": {str(p): sum(e["sge_fallbacks"] for e in pair_rows if e["p"] == p) for p in PANEL},
        "fallback_pairs_list": fbp,
        "la_total_ratio_range": [min(e["la_total_ratio"] for e in fbp), max(e["la_total_ratio"] for e in fbp)],
        "la_add_ratio_range": [min(e["la_add_ratio"] for e in fbp), max(e["la_add_ratio"] for e in fbp)],
        "extra_group_ops_range": [min(e["extra_group_ops_sge"] for e in fbp), max(e["extra_group_ops_sge"] for e in fbp)],
        "extra_ops_per_fallback_over_log2r_range": [min(e["extra_ops_per_fallback_over_log2r"] for e in fbp),
                                                    max(e["extra_ops_per_fallback_over_log2r"] for e in fbp)],
        "fallback_rows_attempts": sorted({(e["sge_attempts"], e["sge_fallbacks"]) for e in fbp}),
        "nonfallback_rows_attempts": sorted({e["sge_attempts"] for e in pair_rows if e["sge_fallbacks"] == 0}),
    }
    # rescue logic: a rescue ends the baseline at a fallback attempt, which the nofb arm
    # cannot certify (cand = None), so it would need more attempts.  Equal attempts on
    # every pair therefore imply no rescue, independently of the la_fallback_rescues counter.
    out["pairs"]["zero_rescues_by_attempt_equality"] = not out["pairs"]["attempts_differ"]

    # ---- C0: baseline seeds 1, 2 against the parent's 2lp-sfb-sge rows ----
    prows = read_jsonl(f"{parent_raw}/run.jsonl") + read_jsonl(f"{parent_raw}/run131101.jsonl")
    pruns = {(r["arm"], r["p"], r["curve_seed"], r["seed"]): r for r in prows if r["kind"] == "run"}
    pinst = {(r["p"], r["curve_seed"]): r for r in prows if r["kind"] == "instance"}
    c0 = {"compared": 0, "identical_all_shared": 0, "mismatch": [], "n_shared_fields": set(),
          "only_in_study": set(), "only_in_parent": set()}
    C0_FIELDS_ANALYZE = ["certified", "ok", "recovered", "group_ops", "cantor_mul", "cantor_inv",
                         "rel_mul", "rel_add", "la_mul", "la_add", "relations", "rels_by_lp",
                         "decomp_trials", "attempts", "core_rows", "core_cols", "core_nnz",
                         "idx_lookups", "idx_inserts", "idx_points", "fb_samples",
                         "la_fallbacks", "sq_dim", "sq_nnz"]
    missing_c0_keys = []
    for k, ref in sorted(pruns.items()):
        if k[0] != SGE:
            continue
        mine = runs[k]
        c0["compared"] += 1
        for f in C0_FIELDS_ANALYZE:
            if f not in ref or f not in mine:
                missing_c0_keys.append([list(k), f])
        shared = (set(ref) & set(mine)) - set(WALL)
        c0["n_shared_fields"].add(len(shared))
        c0["only_in_study"] |= set(mine) - set(ref)
        c0["only_in_parent"] |= set(ref) - set(mine)
        d = sorted(f for f in shared if ref[f] != mine[f])
        if d:
            c0["mismatch"].append({"key": list(k), "fields": d})
        else:
            c0["identical_all_shared"] += 1
    c0["n_shared_fields"] = sorted(c0["n_shared_fields"])
    c0["only_in_study"] = sorted(c0["only_in_study"])
    c0["only_in_parent"] = sorted(c0["only_in_parent"])
    c0["analyze_c0_fields_present_everywhere"] = not missing_c0_keys
    c0["parent_sge_seeds"] = sorted({k[3] for k in pruns if k[0] == SGE})
    out["C0"] = c0
    inst_diff = [list(k) for k in insts if pinst[k] != insts[k]]
    out["instances_equal_parent"] = {"n": len(insts), "differ": inst_diff, "ok": not inst_diff}

    # ---- rho skip rows ----
    sk_bad = [list(k) for k, s in skips.items()
              if abs(s["expected_steps"] - math.sqrt(math.pi * s["r"] / 4)) > 1e-9 * s["expected_steps"]]
    sk_match = all(skips[(p, cs, s)]["t_secret"] == runs[(SGE, p, cs, s)]["t_secret"]
                   and skips[(p, cs, s)]["r"] == runs[(SGE, p, cs, s)]["r"]
                   for (p, cs, s) in skips)
    out["rho_skip_rows"] = {"n": len(skips), "expected_steps_eq_sqrt_pi_r_over_4": not sk_bad,
                            "bad": sk_bad, "same_instance_as_runs": sk_match}

    # ---- paired ratios ----
    metrics = {f"gae_{w}": (lambda r, w=w: gae(r, w)) for w in WS}
    metrics["la_total"] = lambda r: r["la_add"] + r["la_mul"]
    metrics["native_all"] = native
    panels = {"primary_seeds_3_4": (3, 4), "all_seeds_1_4": SEEDS}
    paired, noise = {}, {}
    for pname, ss in panels.items():
        paired[pname] = {}
        for mname, f in metrics.items():
            per = {p: [gmean(f(runs[(NOFB, p, cs, s)]) / f(runs[(SGE, p, cs, s)]) for s in ss)
                       for cs in PANEL[p]] for p in PANEL}
            paired[pname][mname] = summarize_exact(per)
            if mname.startswith("gae_"):
                noise[f"{pname}/{mname}"] = mc_noise(per, MC_REPS, zlib.crc32(f"{pname}/{mname}".encode()))
    out["paired_nofb_over_sge"] = paired

    # ---- IC / ideal negation rho ----
    icr, verdict = {}, {}
    for a in ARMS:
        icr[a], verdict[a] = {}, {}
        for w in WS:
            per = {p: [gmean(gae(runs[(a, p, cs, s)], w) / math.sqrt(math.pi * runs[(a, p, cs, s)]["r"] / 4)
                             for s in SEEDS) for cs in PANEL[p]] for p in PANEL}
            icr[a][f"all_{w}"] = summarize_exact(per)
            if w == 1.0 or w == 0.1:
                noise[f"icr/{a}/{w}"] = mc_noise(per, MC_REPS, zlib.crc32(f"icr/{a}/{w}".encode()))
        for p in PANEL:
            ups = [icr[a][f"all_{w}"]["per_prime"][str(p)]["exact95"][1] for w in WS]
            los = [icr[a][f"all_{w}"]["per_prime"][str(p)]["exact95"][0] for w in WS]
            v = ("BELOW-RHO CANDIDATE" if max(ups) < 1 else
                 "ABOVE RHO" if min(los) > 1 else "UNRESOLVED (order changes inside the band)")
            verdict[a][str(p)] = v
    out["ic_over_rho"] = icr
    out["ic_over_rho_verdict"] = verdict
    out["mc_noise"] = noise

    # ---- decision (PROTOCOL section 5) on the exact law ----
    prim = paired["primary_seeds_3_4"]
    up = {w: prim[f"gae_{w}"]["pooled"]["exact95"][1] for w in WS}
    lo1 = prim["gae_1.0"]["pooled"]["exact95"][0]
    integrity = (not out["completeness"]["missing"] and not out["completeness"]["unexpected"]
                 and out["recorded_certified_all"]["ok"] and not viol and not c0["mismatch"]
                 and c0["compared"] == 40)
    if not integrity:
        dec = "VOID"
    elif up[1.0] < 1 and up[0.1] < 1 and up[0.0] <= 1:
        dec = "IMPROVEMENT"
    elif lo1 >= 0.98:
        dec = "KILL"
    else:
        dec = "INCONCLUSIVE"
    out["decision"] = {"integrity": integrity, "verdict": dec, "upper95_exact": {str(w): up[w] for w in WS},
                       "lower95_exact_w1": lo1}

    # ---- RESULTS.md figures recomputed from rows ----
    extra = {}
    # IC/rho at w = 0, 131101, both arms
    extra["icr_w0_131101"] = {a: icr[a]["all_0.0"]["per_prime"]["131101"]["gm"] for a in ARMS}
    # second-attempt nofb runs vs single-attempt nofb runs of the same curve (GAE_1)
    two = []
    for e in fbp:
        p, cs, s = e["p"], e["curve_seed"], e["seed"]
        singles = [gae(runs[(NOFB, p, cs, t)], 1.0) for t in SEEDS if runs[(SGE, p, cs, t)]["la_fallbacks"] == 0]
        if singles:
            two.append({"p": p, "curve_seed": cs, "seed": s, "attempts": e["nofb_attempts"],
                        "ratio_to_single_gm": gae(runs[(NOFB, p, cs, s)], 1.0) / gmean(singles),
                        "n_single": len(singles)})
    extra["nofb_multi_attempt_over_single_gae1"] = two
    if two:
        extra["nofb_multi_attempt_over_single_gae1_range"] = [min(x["ratio_to_single_gm"] for x in two),
                                                              max(x["ratio_to_single_gm"] for x in two)]
    # share of the sge arm's la carried by fallback runs, per prime (all seeds)
    share = {}
    for p in PANEL:
        tot = sum(runs[(SGE, p, cs, s)]["la_add"] + runs[(SGE, p, cs, s)]["la_mul"] for cs in PANEL[p] for s in SEEDS)
        fbla = sum(runs[(SGE, p, cs, s)]["la_add"] + runs[(SGE, p, cs, s)]["la_mul"] for cs in PANEL[p] for s in SEEDS
                   if runs[(SGE, p, cs, s)]["la_fallbacks"] > 0)
        share[str(p)] = fbla / tot
    extra["sge_la_share_in_fallback_runs"] = share
    out["results_figures"] = extra

    # ---- compare with the producer's analysis.json ----
    with open(f"{st}/analysis.json") as fh:
        prod = json.load(fh)
    cmp = {"max_rel_point": 0.0, "points": 0, "intervals": []}

    def rel(x, y):
        return abs(x - y) / max(abs(x), abs(y), 1e-300)

    for pname in panels:
        for mname in metrics:
            mine = paired[pname][mname]
            theirs = prod["paired_nofb_over_sge"][pname][mname]
            for p in PANEL:
                cmp["max_rel_point"] = max(cmp["max_rel_point"], rel(mine["per_prime"][str(p)]["gm"], theirs["per_prime"][str(p)]["gm"]))
                cmp["points"] += 1
                cmp["intervals"].append({"what": f"paired/{pname}/{mname}/{p}", "producer": theirs["per_prime"][str(p)]["ci95"],
                                         "exact": mine["per_prime"][str(p)]["exact95"]})
            cmp["max_rel_point"] = max(cmp["max_rel_point"], rel(mine["pooled"]["gm"], theirs["pooled"]["gm"]))
            cmp["points"] += 1
            cmp["intervals"].append({"what": f"paired/{pname}/{mname}/pooled", "producer": theirs["pooled"]["ci95"],
                                     "exact": mine["pooled"]["exact95"]})
    for a in ARMS:
        for w in WS:
            mine = icr[a][f"all_{w}"]
            theirs = prod["ic_over_rho"][a][f"all_{w}"]
            for p in PANEL:
                cmp["max_rel_point"] = max(cmp["max_rel_point"], rel(mine["per_prime"][str(p)]["gm"], theirs["per_prime"][str(p)]["gm"]))
                cmp["points"] += 1
                cmp["intervals"].append({"what": f"icr/{a}/{w}/{p}", "producer": theirs["per_prime"][str(p)]["ci95"],
                                         "exact": mine["per_prime"][str(p)]["exact95"]})
            cmp["max_rel_point"] = max(cmp["max_rel_point"], rel(mine["pooled"]["gm"], theirs["pooled"]["gm"]))
            cmp["points"] += 1
    for it in cmp["intervals"]:
        it["max_rel_diff"] = max(rel(it["producer"][0], it["exact"][0]), rel(it["producer"][1], it["exact"][1]))
    cmp["max_rel_interval"] = max(it["max_rel_diff"] for it in cmp["intervals"])
    cmp["worst_intervals"] = sorted(cmp["intervals"], key=lambda x: -x["max_rel_diff"])[:5]
    # are the producer's key endpoints inside the Monte Carlo noise range?
    inside = {}
    for pname in panels:
        for w in WS:
            n = noise[f"{pname}/gae_{w}"]["pooled"]
            ci = prod["paired_nofb_over_sge"][pname][f"gae_{w}"]["pooled"]["ci95"]
            inside[f"{pname}/gae_{w}/pooled"] = (n["lo_range"][0] <= ci[0] <= n["lo_range"][1]
                                                 and n["hi_range"][0] <= ci[1] <= n["hi_range"][1])
    for a in ARMS:
        for w in (0.1, 1.0):
            n = noise[f"icr/{a}/{w}"]["per_prime"]["131101"]
            ci = prod["ic_over_rho"][a][f"all_{w}"]["per_prime"]["131101"]["ci95"]
            inside[f"icr/{a}/{w}/131101"] = (n["lo_range"][0] <= ci[0] <= n["lo_range"][1]
                                             and n["hi_range"][0] <= ci[1] <= n["hi_range"][1])
    cmp["producer_endpoints_inside_mc_range"] = inside
    # every per-prime endpoint the noise study covers (paired GAE_w, IC/rho at w = 0.1, 1)
    per = {}
    for pname in panels:
        for w in WS:
            n = noise[f"{pname}/gae_{w}"]["per_prime"]
            for p in PANEL:
                ci = prod["paired_nofb_over_sge"][pname][f"gae_{w}"]["per_prime"][str(p)]["ci95"]
                per[f"{pname}/gae_{w}/{p}"] = (n[str(p)]["lo_range"][0] <= ci[0] <= n[str(p)]["lo_range"][1]
                                               and n[str(p)]["hi_range"][0] <= ci[1] <= n[str(p)]["hi_range"][1])
    for a in ARMS:
        for w in (0.1, 1.0):
            n = noise[f"icr/{a}/{w}"]["per_prime"]
            for p in PANEL:
                ci = prod["ic_over_rho"][a][f"all_{w}"]["per_prime"][str(p)]["ci95"]
                per[f"icr/{a}/{w}/{p}"] = (n[str(p)]["lo_range"][0] <= ci[0] <= n[str(p)]["lo_range"][1]
                                           and n[str(p)]["hi_range"][0] <= ci[1] <= n[str(p)]["hi_range"][1])
    cmp["producer_per_prime_endpoints_inside_mc_range"] = {"n": len(per), "inside": sum(per.values()),
                                                           "outside": sorted(k for k, v in per.items() if not v)}
    cmp["decision_equal"] = prod["decision"]["verdict"].startswith(dec)
    cmp["verdicts_equal"] = all(prod["ic_over_rho_verdict"][a][str(p)]["verdict"] == verdict[a][str(p)]
                                for a in ARMS for p in PANEL)
    cmp["integrity_fields"] = {
        "n_runs": prod["n_runs"] == {a: 80 for a in ARMS},
        "rescues": prod["rescues"] == {"baseline_fallbacks": out["pairs"]["fallbacks_total"],
                                       "baseline_rescues": 0, "nofb_skipped": out["pairs"]["skipped_total"],
                                       "nofb_fallbacks": 0, "attempts_differ": []},
        "C0": prod["C0_reproduction"] == {"compared": 40, "identical": 40, "mismatches": []},
        "pair_identity_violations_empty": prod["pair_identity_violations"] == [],
    }
    del cmp["intervals"]
    out["compare_with_analysis_json"] = cmp
    with open(outp, "w") as fh:
        json.dump(out, fh, indent=1, default=lambda o: list(o) if isinstance(o, (set, tuple)) else float(o))
        fh.write("\n")
    print(json.dumps({"decision": out["decision"], "max_rel_point": cmp["max_rel_point"],
                      "max_rel_interval": cmp["max_rel_interval"],
                      "pairs": {k: out["pairs"][k] for k in ("identity_violations", "dominance_violations",
                                                             "fallback_pairs", "fallbacks_total",
                                                             "attempts_differ", "la_total_ratio_range")}},
                     indent=1, default=float))


if __name__ == "__main__":
    main()
