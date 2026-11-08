#!/usr/bin/env python3
"""Frozen analysis of the P4 confirmatory study (rho.plain / rho.negation).

research/prime_medium_rho_p4_confirm_20261108, PROTOCOL.md sections 8-10.
Standard library only; run with `python3 -I`.

Subcommands
  analyze   the preregistered analysis of downloaded ecbench sessions
  synth     write synthetic sessions in ecbench record layout (testing only)
  selftest  synthesise named scenarios and check that the decision rule
            returns the verdict each scenario is built to produce

Nothing here reads the 2026-11-07 study's sessions: pooling them is
inadmissible (owner decision 2026-11-08). power_from_old_variance.py reads
their variance, for the sample size only.

The estimator (PROTOCOL.md section 8):
  pair     = (workload, measured round) with a verified-or-censored record of
             every main-job arm (rho-neg, rho-neg-aa, rho-plain, rho-neg-v2);
  S        = (total_gae + w * sum of the record's unpriced counters) / sqrt(r);
  theta    = mean_c(mean S_plain over curve c's pairs)
             / mean_c(mean S_neg over curve c's pairs)   (curve-equal weights);
  interval = stratified cluster bootstrap: every curve kept in every resample,
             workloads resampled with replacement within their curve, a
             workload's pairs kept together; RESAMPLES resamples from
             random.Random(seed), seed = the seeds.json rule on tag
             "analysis/bootstrap"; the per-resample index stream is drawn
             curve by curve in sorted-slug order and workload by workload in
             sorted-workload-id order, and every statistic is evaluated on the
             same resamples; percentile interval with linear interpolation
             (Hyndman-Fan type 7).
"""
import argparse
import hashlib
import json
import math
import os
import random
import sys

# ---------------------------------------------------------------- frozen ---
STUDY = "prime_medium_rho_p4_confirm_20261108"
SEED_PREFIX = STUDY + "/v1/"
RESAMPLES = 20000
RESAMPLES_LARGE = 200000
TIE_MARGIN = 0.002
THRESHOLD = 1.2
SQRT2 = math.sqrt(2.0)
OLD_POINT = 1.285          # the 2026-11-07 J1/J2 point value, a reference point only
PC_EXPECTED = 1.5          # bsgs.textbook / bsgs.negation, methods::expected_s
LOOKAHEAD_HEURISTIC = SQRT2 / (1.0 + 1.0 / 32.0)
WEIGHTS = (0.0, 0.1, 1.0)
MAIN_ARMS = ("rho-neg", "rho-neg-aa", "rho-plain", "rho-neg-v2")
PC_ARMS = ("bsgs-neg", "bsgs-neg-aa", "bsgs-tb")
NUM, DEN, AA, V2 = "rho-plain", "rho-neg", "rho-neg-aa", "rho-neg-v2"
PC_NUM, PC_DEN, PC_AA = "bsgs-tb", "bsgs-neg", "bsgs-neg-aa"
MAX_CENSORED_SHARE = 0.01
MIN_PAIR_SHARE = 0.90
EXPECTED_BINARY = "a18af09f9fb875dd09d0399259b5481b6952d9a8f1f52f5fccc7d9ab1a0cd3b9"
OVERHEAD_KEYS = ("look_ahead_additions", "cycle_escape_doublings", "setup_additions",
                 "setup_doublings", "start_additions", "verification_operations")
VERDICTS = ("confirm_sqrt2", "exceeds_1.2_below_sqrt2", "exceeds_sqrt2", "below_1.2",
            "inconclusive")


def derive_seed(tag):
    return int(hashlib.sha256((SEED_PREFIX + tag).encode()).hexdigest()[:12], 16)


# ------------------------------------------------------------ statistics ---
def quantile(sorted_xs, q):
    """Hyndman-Fan type 7 (linear interpolation) on a sorted list."""
    n = len(sorted_xs)
    if n == 1:
        return sorted_xs[0]
    h = (n - 1) * q
    lo = math.floor(h)
    hi = min(lo + 1, n - 1)
    return sorted_xs[lo] + (h - lo) * (sorted_xs[hi] - sorted_xs[lo])


def interval(xs, level):
    s = sorted(x for x in xs if x == x)
    a = (1.0 - level) / 2.0
    return [quantile(s, a), quantile(s, 1.0 - a)]


def chi2_sf(x, k):
    """Upper tail of chi-square with k dof (regularised upper gamma)."""
    a, x2 = k / 2.0, x / 2.0
    if x2 <= 0:
        return 1.0
    if x2 < a + 1:  # series for the lower gamma
        term = total = 1.0 / a
        n = 1
        while abs(term) > 1e-15 * abs(total):
            term *= x2 / (a + n)
            total += term
            n += 1
        lower = total * math.exp(-x2 + a * math.log(x2) - math.lgamma(a))
        return max(0.0, 1.0 - lower)
    # continued fraction (Lentz) for the upper gamma
    b = x2 + 1 - a
    c = 1e300
    d = 1 / b
    f = d
    i = 1
    while True:
        an = -i * (i - a)
        b += 2
        d = an * d + b
        d = 1e-300 if abs(d) < 1e-300 else d
        c = b + an / c
        c = 1e-300 if abs(c) < 1e-300 else c
        d = 1 / d
        delta = d * c
        f *= delta
        i += 1
        if abs(delta - 1) < 1e-15 or i > 10000:
            break
    return f * math.exp(-x2 + a * math.log(x2) - math.lgamma(a))


class Clusters:
    """Workloads grouped by curve; each workload carries a vector of sums."""

    def __init__(self, keys):
        self.keys = list(keys)          # names of the summed quantities
        self.curves = {}                # slug -> {"log2_r": x, "w": {wid: [sums]}}

    def add(self, slug, log2_r, wid, values):
        c = self.curves.setdefault(slug, {"log2_r": log2_r, "w": {}})
        v = c["w"].setdefault(wid, [0.0] * len(self.keys))
        for i, k in enumerate(self.keys):
            v[i] += values.get(k, 0.0)

    def frozen(self):
        out = []
        for slug in sorted(self.curves):
            c = self.curves[slug]
            out.append((slug, c["log2_r"], [c["w"][w] for w in sorted(c["w"])]))
        return out

    @staticmethod
    def aggregate(ws):
        tot = [0.0] * len(ws[0])
        for v in ws:
            for i, x in enumerate(v):
                tot[i] += x
        return tot


def run_bootstrap(clusters, stats, resamples, seed):
    """stats: name -> f(per-curve aggregates: list of (slug, log2_r, dict)) -> float."""
    fz = clusters.frozen()
    keys = clusters.keys

    def as_dict(slug, lr, tot):
        return (slug, lr, dict(zip(keys, tot)))

    point_aggs = [as_dict(s, lr, Clusters.aggregate(ws)) for s, lr, ws in fz]
    point = {name: f(point_aggs) for name, f in stats.items()}
    rng = random.Random(seed)
    draws = {name: [] for name in stats}
    nk = len(keys)
    for _ in range(resamples):
        aggs = []
        for slug, lr, ws in fz:
            n = len(ws)
            tot = [0.0] * nk
            for _i in range(n):
                v = ws[rng.randrange(n)]
                for j in range(nk):
                    tot[j] += v[j]
            aggs.append((slug, lr, dict(zip(keys, tot))))
        for name, f in stats.items():
            draws[name].append(f(aggs))
    return point, draws


def curve_equal_ratio(num, den, n="n"):
    def f(aggs):
        mn = sum(a[2][num] / a[2][n] for a in aggs) / len(aggs)
        md = sum(a[2][den] / a[2][n] for a in aggs) / len(aggs)
        return mn / md
    return f


def curve_equal_mean(key, n="n", scale=1.0):
    def f(aggs):
        return sum(a[2][key] / a[2][n] for a in aggs) / len(aggs) / scale
    return f


def per_curve_ratio(slug, num, den):
    def f(aggs):
        for a in aggs:
            if a[0] == slug:
                return a[2][num] / a[2][den]
        return float("nan")
    return f


def slope_ln_ratio(num, den):
    def f(aggs):
        xs = [a[1] for a in aggs]
        ys = [math.log(a[2][num] / a[2][den]) for a in aggs]
        mx, my = sum(xs) / len(xs), sum(ys) / len(ys)
        sxx = sum((x - mx) ** 2 for x in xs)
        return sum((x - mx) * (y - my) for x, y in zip(xs, ys)) / sxx if sxx > 0 else float("nan")
    return f


def ratio_with_interval(pairs, resamples, seed, level=0.95):
    """Primary estimator on bare tuples (curve, workload, round, num_s, den_s).

    Used by power_from_old_variance.py; analyze() uses the same estimator
    through Clusters/run_bootstrap with more statistics.
    """
    cl = Clusters(("num", "den", "n"))
    for c, w, _k, x, y in pairs:
        cl.add(c, 0.0, w, {"num": x, "den": y, "n": 1.0})
    point, draws = run_bootstrap(cl, {"theta": curve_equal_ratio("num", "den")}, resamples, seed)
    return {"theta": point["theta"], "ci95": interval(draws["theta"], 0.95)}


def primary_outcome(lo, hi):
    """PROTOCOL.md section 9, table 1 (gates are applied by the caller)."""
    flags = {
        "excludes_1.2_from_below": lo > THRESHOLD,
        "contains_sqrt2": lo <= SQRT2 <= hi,
        "contains_1.285": lo <= OLD_POINT <= hi,
        "halfwidth": (hi - lo) / 2.0,
        "precision_target_met": (hi - lo) / 2.0 <= 0.05,
    }
    if lo > THRESHOLD:
        if lo <= SQRT2 <= hi:
            v = "confirm_sqrt2"
        elif hi < SQRT2:
            v = "exceeds_1.2_below_sqrt2"
        else:
            v = "exceeds_sqrt2"
    elif hi < THRESHOLD:
        v = "below_1.2"
    else:
        v = "inconclusive"
    return {"verdict": v, **flags}


# ------------------------------------------------------------- loading ---
def read_jsonl(path):
    with open(path) as fh:
        return [json.loads(line) for line in fh if line.strip()]


def unpriced_sum(rec):
    c = rec.get("counters") or {}
    return float(sum(c.get(k, 0) for k in (rec["cost"].get("unpriced") or [])))


def load_session(d, audit_path, expected_spec_ids, problems, dev):
    sess = json.load(open(os.path.join(d, "session.json")))
    recs = read_jsonl(os.path.join(d, "records.jsonl"))
    sid = sess.get("session_id")
    if sess.get("status") != "complete":
        problems.append("%s: session status %r, not complete" % (d, sess.get("status")))
    if len(recs) != sess.get("executions_planned"):
        problems.append("%s: %d records, %s planned" % (d, len(recs), sess.get("executions_planned")))
    if not dev and sess.get("binary_sha256") != EXPECTED_BINARY:
        problems.append("%s: binary %s is not the frozen instrument" % (d, sess.get("binary_sha256")))
    if expected_spec_ids is not None and sess.get("spec_id") not in expected_spec_ids:
        problems.append("%s: spec_id %s is not one of the frozen specs" % (d, sess.get("spec_id")))
    if audit_path is None:
        problems.append("%s: no audit file given" % d)
    else:
        au = json.load(open(audit_path))
        if not (au.get("ok") is True and not au.get("problems") and au.get("session_id") == sid
                and au.get("replay_selection") == "all"):
            problems.append("%s: audit %s does not pass (ok, no problems, same session, replay all)" % (d, audit_path))
    for r in recs:
        if r.get("session_id") != sid:
            problems.append("%s: record %s from another session" % (d, r.get("record_id")))
            break
    return sid, sess.get("spec_id"), recs


def measured_index(recs):
    """(workload_id, round) -> arm -> record, measured rounds only."""
    idx = {}
    meta = {}
    for r in recs:
        if r.get("warmup"):
            continue
        wl = r["workload"]
        wid = wl["workload_id"]
        key = (wid, int(r["round"]))
        arms = idx.setdefault(key, {})
        if r["arm"] in arms:
            raise SystemExit("duplicate record for %s %s" % (key, r["arm"]))
        arms[r["arm"]] = r
        meta[wid] = (wl["curve"]["slug"], float(wl["curve"]["r"]))
    return idx, meta


def s_of(rec, r, w):
    return (float(rec["cost"]["total_gae"]) + w * unpriced_sum(rec)) / math.sqrt(r)


def censored(rec):
    return rec["outcome"]["status"] != "verified"


# ------------------------------------------------------------ analysis ---
def analyze(args):
    problems = []
    dev = bool(args.dev)
    resamples = args.resamples if (dev and args.resamples) else RESAMPLES
    seeds = json.load(open(args.seeds))
    tagged = {s["tag"]: s["seed"] for s in seeds["seeds"]}
    for tag, val in tagged.items():
        if derive_seed(tag) != val:
            problems.append("seeds.json: %s does not follow the rule" % tag)
    boot_seed = derive_seed("analysis/bootstrap")
    if tagged.get("analysis/bootstrap") != boot_seed:
        problems.append("seeds.json: analysis/bootstrap missing or wrong")
    expected = None
    if args.spec_ids:
        expected = set(json.load(open(args.spec_ids))["spec_ids"].values())

    main_recs, pc_recs = [], []
    sessions = []
    audits = args.audit or []
    if len(audits) != len(args.session):
        problems.append("one --audit per --session is required")
    for i, d in enumerate(args.session):
        sid, spec_id, recs = load_session(d, audits[i] if i < len(audits) else None, expected, problems, dev)
        arms = {r["arm"] for r in recs}
        sessions.append({"dir": d, "session_id": sid, "spec_id": spec_id, "records": len(recs),
                         "arms": sorted(arms)})
        if arms == set(MAIN_ARMS):
            main_recs.extend(recs)
        elif arms == set(PC_ARMS):
            pc_recs.extend(recs)
        else:
            problems.append("%s: arm set %s is neither the main nor the positive-control set" % (d, sorted(arms)))

    out = {"schema": "prime_medium_rho_p4_confirm.result/v1", "study": STUDY,
           "admissible": not dev, "dev": dev, "resamples": resamples,
           "bootstrap_seed": boot_seed, "python": sys.version.split()[0],
           "sessions": sessions}

    # ---------- main jobs: pairs, gates G2 (A/A determinism), G5, G6 ----------
    idx, meta = measured_index(main_recs)
    planned = len(idx)
    aa_mismatch, cens = 0, {a: 0 for a in MAIN_ARMS}
    keys = ["n"]
    for a in (NUM, DEN, V2):
        for w in WEIGHTS:
            keys.append("%s|w%s" % (a, w))
        keys.append("%s|steps" % a)
    keys += ["%s|%s" % (DEN, k) for k in OVERHEAD_KEYS]
    keys += ["%s|lost" % V2, "%s|cyc2" % DEN, "%s|cyc4" % DEN, "%s|cycother" % DEN,
             "%s|capped" % DEN, "%s|abandoned" % DEN, "%s|walks" % DEN]
    for half in ("even", "odd"):
        keys += ["n_" + half, "%s|%s" % (NUM, half), "%s|%s" % (DEN, half)]
    cl = Clusters(keys)
    used = 0
    for (wid, rnd), arms in sorted(idx.items()):
        if set(arms) != set(MAIN_ARMS):
            continue
        slug, r = meta[wid]
        for a in MAIN_ARMS:
            cens[a] += censored(arms[a])
        ra, rn = arms[AA], arms[DEN]
        if ra["cost"]["total_gae"] != rn["cost"]["total_gae"] or ra.get("counters") != rn.get("counters"):
            aa_mismatch += 1
        v = {"n": 1.0}
        for a in (NUM, DEN, V2):
            for w in WEIGHTS:
                v["%s|w%s" % (a, w)] = s_of(arms[a], r, w)
            v["%s|steps" % a] = float(arms[a]["counters"].get("steps", 0)) / math.sqrt(r)
        c = rn["counters"]
        for k in OVERHEAD_KEYS:
            v["%s|%s" % (DEN, k)] = float(c.get(k, 0)) / math.sqrt(r)
        v["%s|lost" % V2] = float(arms[V2]["counters"].get("lost_walk_steps", 0)) / math.sqrt(r)
        v["%s|cyc2" % DEN] = float(c.get("cycles_length_2", 0))
        v["%s|cyc4" % DEN] = float(c.get("cycles_length_4", 0))
        v["%s|cycother" % DEN] = float(c.get("cycles_other_length", 0))
        v["%s|capped" % DEN] = float(c.get("walks_capped", 0))
        v["%s|abandoned" % DEN] = float(c.get("walks_abandoned_in_a_cycle", 0))
        v["%s|walks" % DEN] = float(c.get("walks", 0))
        half = "even" if rnd % 2 == 0 else "odd"
        v["n_" + half] = 1.0
        v["%s|%s" % (NUM, half)] = v["%s|w0.0" % NUM]
        v["%s|%s" % (DEN, half)] = v["%s|w0.0" % DEN]
        cl.add(slug, math.log2(r), wid, v)
        used += 1
    gates = {}
    gates["G1_sessions_complete_and_audited"] = not problems
    gates["G2_aa_determinism"] = aa_mismatch == 0 and used > 0
    cens_share = {a: (cens[a] / used if used else 1.0) for a in MAIN_ARMS}
    gates["G5_censoring"] = all(x <= MAX_CENSORED_SHARE for x in cens_share.values())
    planned_pairs = args.planned_pairs or planned
    gates["G6_pair_coverage"] = used >= MIN_PAIR_SHARE * planned_pairs
    out["flow"] = {"pairs_planned": planned_pairs, "pairs_complete": used,
                   "aa_mismatched_pairs": aa_mismatch, "censored_share": cens_share,
                   "curves": len(cl.curves)}
    if used == 0:
        out["gates"] = gates
        out["verdict"] = "inconclusive"
        out["problems"] = problems + ["no complete main-job pairs"]
        return out

    stats = {}
    for w in WEIGHTS:
        stats["theta_w%s" % w] = curve_equal_ratio("%s|w%s" % (NUM, w), "%s|w%s" % (DEN, w))
    stats["theta_steps"] = curve_equal_ratio("%s|steps" % NUM, "%s|steps" % DEN)
    stats["theta_v2"] = curve_equal_ratio("%s|w0.0" % NUM, "%s|w0.0" % V2)
    stats["neg_R"] = curve_equal_mean("%s|w0.0" % DEN, scale=math.sqrt(math.pi / 4))
    stats["plain_R"] = curve_equal_mean("%s|w0.0" % NUM, scale=math.sqrt(math.pi / 2))
    stats["v2_R"] = curve_equal_mean("%s|w0.0" % V2, scale=math.sqrt(math.pi / 4))
    stats["slope_ln_theta_per_bit"] = slope_ln_ratio("%s|w0.0" % NUM, "%s|w0.0" % DEN)
    for k in OVERHEAD_KEYS:
        stats["neg_overhead_%s" % k] = curve_equal_ratio("%s|%s" % (DEN, k), "%s|steps" % DEN)
    stats["v2_lost_share"] = curve_equal_ratio("%s|lost" % V2, "%s|w0.0" % V2)
    slugs = sorted(cl.curves)
    for s in slugs:
        stats["curve:" + s] = per_curve_ratio(s, "%s|w0.0" % NUM, "%s|w0.0" % DEN)

    # Split-half A/A: mean S over even measured rounds / mean S over odd ones
    # (independent algorithm seeds, same method), curve-equal weights.
    def split(numk, denk):
        def f(aggs):
            me = sum(a[2][numk] / a[2]["n_even"] for a in aggs) / len(aggs)
            mo = sum(a[2][denk] / a[2]["n_odd"] for a in aggs) / len(aggs)
            return me / mo
        return f
    stats["aa_split_neg"] = split("%s|even" % DEN, "%s|odd" % DEN)
    stats["aa_split_plain"] = split("%s|even" % NUM, "%s|odd" % NUM)

    point, draws = run_bootstrap(cl, stats, resamples, boot_seed)
    ci = {k: interval(draws[k], 0.95) for k in stats}
    lo, hi = ci["theta_w0.0"]
    tie = (abs(lo - THRESHOLD) < TIE_MARGIN or abs(lo - SQRT2) < TIE_MARGIN
           or abs(hi - SQRT2) < TIE_MARGIN or abs(hi - THRESHOLD) < TIE_MARGIN)
    out["tie_break_rerun"] = False
    if tie and not dev:
        p2, d2 = run_bootstrap(cl, {"theta_w0.0": stats["theta_w0.0"]}, RESAMPLES_LARGE,
                               derive_seed("analysis/bootstrap-large"))
        lo, hi = interval(d2["theta_w0.0"], 0.95)
        ci["theta_w0.0"] = [lo, hi]
        out["tie_break_rerun"] = True

    # G4: split-half A/A (independent algorithm seeds), Bonferroni over two.
    aa_ci = {k: interval(draws[k], 0.975) for k in ("aa_split_neg", "aa_split_plain")}
    gates["G4_split_half_aa"] = all(v[0] <= 1.0 <= v[1] for v in aa_ci.values())

    # ---------- positive control job: G3 ----------
    pc = {"present": bool(pc_recs)}
    if pc_recs:
        pidx, pmeta = measured_index(pc_recs)
        pcl = Clusters(("n", "num", "den"))
        pc_aa_bad, pused = 0, 0
        for (wid, rnd), arms in sorted(pidx.items()):
            if set(arms) != set(PC_ARMS):
                continue
            slug, r = pmeta[wid]
            if arms[PC_AA]["cost"]["total_gae"] != arms[PC_DEN]["cost"]["total_gae"]:
                pc_aa_bad += 1
            pcl.add(slug, math.log2(r), wid, {"n": 1.0, "num": s_of(arms[PC_NUM], r, 0.0),
                                              "den": s_of(arms[PC_DEN], r, 0.0)})
            pused += 1
        pp, pd = run_bootstrap(pcl, {"pc": curve_equal_ratio("num", "den")}, resamples,
                               derive_seed("analysis/bootstrap-positive-control"))
        pc.update({"pairs": pused, "planned": len(pidx), "aa_mismatched_pairs": pc_aa_bad,
                   "ratio": pp["pc"], "ci95": interval(pd["pc"], 0.95), "ci99": interval(pd["pc"], 0.99),
                   "expected": PC_EXPECTED})
        pc["detects"] = pc["ci95"][0] > THRESHOLD
        pc["expected_inside_ci99"] = pc["ci99"][0] <= PC_EXPECTED <= pc["ci99"][1]
        gates["G3_positive_control"] = (pc["detects"] and pc["expected_inside_ci99"] and pc_aa_bad == 0
                                        and pused >= MIN_PAIR_SHARE * len(pidx))
    else:
        gates["G3_positive_control"] = False
    out["positive_control"] = pc

    # ---------- primary decision ----------
    prim = primary_outcome(lo, hi)
    all_gates = all(gates.values())
    out["gates"] = gates
    out["primary"] = {"theta": point["theta_w0.0"], "ci95": [lo, hi], **prim,
                      "estimator": "curve-equal ratio of mean S, w = 0; stratified workload-cluster bootstrap"}
    out["verdict"] = prim["verdict"] if all_gates else "inconclusive"
    if not all_gates:
        out["verdict_reason"] = "gate(s) failed: " + ", ".join(k for k, v in gates.items() if not v)

    # ---------- predeclared secondaries (never change the verdict) ----------
    sec = {}
    sec["band"] = {("w=%s" % w): {"theta": point["theta_w%s" % w], "ci95": ci["theta_w%s" % w]} for w in WEIGHTS}
    signs = [point["theta_w%s" % w] > 1.0 for w in WEIGHTS]
    los = [ci["theta_w%s" % w][0] > 1.0 for w in WEIGHTS]
    his = [ci["theta_w%s" % w][1] < 1.0 for w in WEIGHTS]
    sec["band"]["order"] = ("rho-plain costlier at every weight" if all(los) else
                            "rho-plain cheaper at every weight" if all(his) else
                            "unresolved inside the band")
    sec["band"]["point_order_by_weight"] = signs
    sec["split_half_aa"] = {k: {"ratio": point[k], "ci97.5": aa_ci[k]} for k in aa_ci}
    sec["R_vs_floor"] = {k: {"value": point[k], "ci95": ci[k]} for k in ("neg_R", "plain_R", "v2_R")}
    ts = ci["theta_steps"]
    sec["theta_steps"] = {"value": point["theta_steps"], "ci95": ts,
                          "contains_sqrt2": ts[0] <= SQRT2 <= ts[1]}
    th_lo, th_hi = lo, hi
    if ts[0] <= SQRT2 <= ts[1] and th_hi < SQRT2:
        attribution = "charged_overhead"
    elif ts[1] < SQRT2:
        attribution = "walk_level_shortfall"
    elif th_lo <= SQRT2 <= th_hi:
        attribution = "no_shortfall"
    else:
        attribution = "unresolved"
    sec["sqrt2_attribution"] = attribution
    sec["lookahead_heuristic_reference"] = {
        "value": LOOKAHEAD_HEURISTIC,
        "inside_theta_ci95": th_lo <= LOOKAHEAD_HEURISTIC <= th_hi,
        "class": "heuristic: sqrt2 / (1 + 1/32), the look-ahead's expected extra additions per step only"}
    sec["neg_overhead_per_step"] = {k: {"value": point["neg_overhead_%s" % k], "ci95": ci["neg_overhead_%s" % k]}
                                    for k in OVERHEAD_KEYS}
    sec["theta_v2"] = {"value": point["theta_v2"], "ci95": ci["theta_v2"]}
    sec["v2_lost_walk_share"] = {"value": point["v2_lost_share"], "ci95": ci["v2_lost_share"]}
    sl = ci["slope_ln_theta_per_bit"]
    sec["size_trend"] = {"slope_ln_theta_per_log2r": point["slope_ln_theta_per_bit"], "ci95": sl,
                         "decreasing_with_size": sl[1] < 0, "increasing_with_size": sl[0] > 0}
    # Heterogeneity across curves (Cochran Q on ln theta_c, bootstrap variances).
    per = []
    for s in slugs:
        d = [math.log(x) for x in draws["curve:" + s] if x > 0]
        m = sum(d) / len(d)
        var = sum((x - m) ** 2 for x in d) / (len(d) - 1)
        per.append((s, cl.curves[s]["log2_r"], point["curve:" + s], interval(draws["curve:" + s], 0.95), var))
    wsum = sum(1 / p[4] for p in per)
    mbar = sum(math.log(p[2]) / p[4] for p in per) / wsum
    q = sum((math.log(p[2]) - mbar) ** 2 / p[4] for p in per)
    sec["curve_heterogeneity"] = {"Q": q, "dof": len(per) - 1, "p": chi2_sf(q, len(per) - 1),
                                  "test": "Cochran Q on ln theta_c, inverse bootstrap-variance weights"}
    sec["per_curve"] = [{"slug": p[0], "log2_r": p[1], "theta": p[2], "ci95": p[3]} for p in per]
    fz = {s: Clusters.aggregate(cl.curves[s]["w"].values()) for s in slugs}
    kix = {k: i for i, k in enumerate(keys)}
    sec["fruitless_cycles_per_walk"] = [
        {"slug": s, "log2_r": cl.curves[s]["log2_r"],
         "cycles2": fz[s][kix["%s|cyc2" % DEN]] / max(fz[s][kix["%s|walks" % DEN]], 1),
         "cycles4": fz[s][kix["%s|cyc4" % DEN]] / max(fz[s][kix["%s|walks" % DEN]], 1),
         "cycles_other": fz[s][kix["%s|cycother" % DEN]] / max(fz[s][kix["%s|walks" % DEN]], 1),
         "capped": fz[s][kix["%s|capped" % DEN]] / max(fz[s][kix["%s|walks" % DEN]], 1),
         "abandoned": fz[s][kix["%s|abandoned" % DEN]] / max(fz[s][kix["%s|walks" % DEN]], 1)}
        for s in slugs]
    out["secondary"] = sec
    out["problems"] = problems
    return out


# ------------------------------------------------------------- synthetic ---
CURVES = None


def load_curves(path):
    with open(path) as fh:
        d = json.load(fh)
    return [(c["slug"], int(c["r"])) for c in d["curves"]]


def synth(args):
    """Synthetic sessions in ecbench record layout. NOT data: tests only."""
    rng = random.Random(args.seed)
    curves = load_curves(args.curves)
    os.makedirs(args.out, exist_ok=True)
    theta, cv_n, cv_p = args.theta, 0.45, 0.55
    mu_neg_steps = 0.93
    made = []

    def gam(mean, cv):
        k = 1 / (cv * cv)
        return rng.gammavariate(k, mean / k)

    def write_session(name, arms, workloads, rounds, recgen):
        d = os.path.join(args.out, name)
        os.makedirs(d, exist_ok=True)
        sid = "SYNTH-" + name
        recs, seq = [], 0
        for rnd in range(rounds + 1):
            order = arms if rnd % 2 == 0 else tuple(reversed(arms))
            for wid, slug, r in workloads:
                base = recgen(wid, slug, r, rnd)
                for a in order:
                    rec = base[a]
                    rec.update({"schema": "ecbench.record/v1", "session_id": sid, "seq": seq,
                                "round": rnd, "warmup": rnd == 0, "arm": a,
                                "record_id": "R%s-%d" % (name, seq),
                                "workload": {"workload_id": wid, "curve": {"slug": slug, "r": r}}})
                    recs.append(rec)
                    seq += 1
        with open(os.path.join(d, "records.jsonl"), "w") as fh:
            for rec in recs:
                fh.write(json.dumps(rec) + "\n")
        with open(os.path.join(d, "session.json"), "w") as fh:
            json.dump({"schema": "ecbench.session/v1", "session_id": sid, "status": "complete",
                       "executions_planned": len(recs), "binary_sha256": EXPECTED_BINARY,
                       "spec_id": "SYNTH"}, fh)
        with open(os.path.join(d, "audit.json"), "w") as fh:
            json.dump({"schema": "ecbench.audit/v1", "session_id": sid, "ok": True, "problems": [],
                       "replay_selection": "all"}, fh)
        made.append(d)

    def rho_rec(steps, neg, r, status="verified"):
        sq = math.sqrt(r)
        st = int(steps * sq)
        la = int(st / 32) if neg else 0
        esc = int(st * 0.0012) if neg else 0
        setup = 360
        walks = max(1, int(st / 2 ** (int(math.log2(r) + 1) // 4)))
        tot = st + la + esc + setup + walks + 50
        c = {"steps": st, "look_ahead_additions": la, "cycle_escape_doublings": esc,
             "setup_additions": 156, "setup_doublings": 204, "start_additions": walks,
             "verification_operations": 50, "walks": walks, "cycles_length_2": 0,
             "cycles_length_4": int(st * 0.0009) if neg else 0, "cycles_other_length": 0,
             "walks_capped": 0, "walks_abandoned_in_a_cycle": 0,
             "canonicalisations_uncharged": tot if neg else 0}
        return {"cost": {"total_gae": float(tot), "s": tot / sq,
                         "unpriced": ["canonicalisations_uncharged"] if neg else []},
                "counters": c, "outcome": {"status": status}}

    def main_gen(wid, slug, r, rnd):
        a = gam(mu_neg_steps, cv_n)
        p = gam(mu_neg_steps * theta * (1 + 1 / 32), cv_p)   # theta is the S ratio, not the steps ratio
        v = gam(mu_neg_steps, cv_n)
        recs = {DEN: rho_rec(a, True, r), NUM: rho_rec(p, False, r), V2: rho_rec(v, True, r)}
        recs[V2]["counters"]["lost_walk_steps"] = 0
        aa = json.loads(json.dumps(recs[DEN]))
        if args.break_aa and rnd == 1 and wid.endswith("t000"):
            aa["cost"]["total_gae"] += 1.0
        recs[AA] = aa
        return recs

    target_u = {}

    def pc_gen(wid, slug, r, rnd):
        # BSGS is deterministic given the target: one draw of the planted
        # scalar d per workload, the same counts in every round. Textbook:
        # 1 + frac(d / m) of sqrt r; negation-folded: 0.5 + a different
        # uniform function of d. pc_scale != 0 breaks the control.
        u, v = target_u.setdefault(wid, (rng.random(), rng.random()))
        sq = math.sqrt(r)
        neg = (0.5 + u) * sq
        tb = (1.0 + v) * sq * (args.pc_scale if args.pc_scale else 1.0)
        out = {}
        for a, g in ((PC_DEN, neg), (PC_AA, neg), (PC_NUM, tb)):
            out[a] = {"cost": {"total_gae": float(int(g)), "s": int(g) / sq,
                               "unpriced": ["inserts_uncharged", "lookups_uncharged"]},
                      "counters": {"inserts_uncharged": int(sq / 2), "lookups_uncharged": int(sq / 2)},
                      "outcome": {"status": "verified"}}
        return out

    t = args.targets
    for job_i, chunk in enumerate([curves[0:6], curves[6:8], curves[8:9], curves[9:10]]):
        wls = [("W%s-t%03d" % (slug[-8:], k), slug, r) for slug, r in chunk for k in range(t)]
        write_session("M%d" % (job_i + 1), MAIN_ARMS, wls, args.rounds, main_gen)
    pwls = [("P%s-t%03d" % (slug[-8:], k), slug, r) for slug, r in curves[0:6] for k in range(args.pc_targets)]
    write_session("PC", PC_ARMS, pwls, args.rounds, pc_gen)
    return made


def selftest(args):
    here = os.path.dirname(os.path.abspath(__file__))
    root = os.path.dirname(here)
    curves = os.path.join(root, "curves.json")
    seeds = os.path.join(root, "seeds.json")
    base = args.workdir
    scenarios = [
        # name, theta, targets, break_aa, pc_scale, expected verdict
        ("old_point_1.285", 1.285, 24, False, 0.0, "exceeds_1.2_below_sqrt2"),
        ("sqrt2", SQRT2, 24, False, 0.0, "confirm_sqrt2"),
        ("ratio_1.0", 1.0, 24, False, 0.0, "below_1.2"),
        ("straddle_1.2_small_n", 1.2, 4, False, 0.0, "inconclusive"),
        ("aa_broken", SQRT2, 24, True, 0.0, "inconclusive"),
        ("positive_control_blind", SQRT2, 24, False, 0.7, "inconclusive"),
    ]
    results = []
    ok = True
    for i, (name, th, tg, br, pcs, want) in enumerate(scenarios):
        d = os.path.join(base, name)
        ns = argparse.Namespace(out=d, curves=curves, seed=1000 + i, theta=th, targets=tg, rounds=6,
                                pc_targets=16, break_aa=br, pc_scale=pcs)
        dirs = synth(ns)
        an = argparse.Namespace(session=dirs, audit=[os.path.join(x, "audit.json") for x in dirs],
                                seeds=seeds, spec_ids=None, dev=True, resamples=args.resamples,
                                planned_pairs=None)
        res = analyze(an)
        got = res["verdict"]
        good = got == want
        ok &= good
        results.append({"scenario": name, "theta_true": th, "targets_per_curve": tg, "expected": want,
                        "got": got, "pass": good, "theta_hat": res.get("primary", {}).get("theta"),
                        "ci95": res.get("primary", {}).get("ci95"), "gates": res.get("gates"),
                        "theta_steps": res.get("secondary", {}).get("theta_steps"),
                        "attribution": res.get("secondary", {}).get("sqrt2_attribution"),
                        "positive_control": {k: res.get("positive_control", {}).get(k)
                                             for k in ("ratio", "ci95", "ci99")}})
        print("%-26s want %-26s got %-26s %s" % (name, want, got, "ok" if good else "FAIL"), flush=True)
    # Decision-table unit checks on fixed intervals.
    table = [((1.25, 1.33), "exceeds_1.2_below_sqrt2"), ((1.37, 1.46), "confirm_sqrt2"),
             ((1.42, 1.50), "exceeds_sqrt2"), ((1.05, 1.19), "below_1.2"), ((1.15, 1.30), "inconclusive"),
             ((1.2, 1.3), "inconclusive")]
    for (lo, hi), want in table:
        got = primary_outcome(lo, hi)["verdict"]
        ok &= got == want
        results.append({"unit": [lo, hi], "expected": want, "got": got, "pass": got == want})
    ok &= abs(chi2_sf(9.0, 9) - 0.4373) < 1e-3 and abs(chi2_sf(16.919, 9) - 0.05) < 1e-3
    results.append({"unit": "chi2_sf", "pass": abs(chi2_sf(16.919, 9) - 0.05) < 1e-3})
    doc = {"schema": "prime_medium_rho_p4_confirm.selftest/v1", "synthetic": True,
           "note": "synthetic data generated by this script; no measured data", "resamples": args.resamples,
           "all_pass": ok, "results": results, "python": sys.version.split()[0]}
    with open(args.out, "w") as fh:
        json.dump(doc, fh, indent=1, sort_keys=True)
        fh.write("\n")
    print("selftest", "PASS" if ok else "FAIL")
    return 0 if ok else 1


def main():
    ap = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    sub = ap.add_subparsers(dest="cmd", required=True)
    a = sub.add_parser("analyze")
    a.add_argument("--session", action="append", required=True)
    a.add_argument("--audit", action="append")
    a.add_argument("--seeds", required=True)
    a.add_argument("--spec-ids", help="specs/spec-ids.json (job -> ecbench spec_id)")
    a.add_argument("--planned-pairs", type=int)
    a.add_argument("--out", required=True)
    a.add_argument("--dev", action="store_true", help="development run: not admissible; allows --resamples")
    a.add_argument("--resamples", type=int)
    s = sub.add_parser("synth")
    s.add_argument("--out", required=True)
    s.add_argument("--curves", required=True)
    s.add_argument("--seed", type=int, default=1)
    s.add_argument("--theta", type=float, default=1.3)
    s.add_argument("--targets", type=int, default=8)
    s.add_argument("--rounds", type=int, default=6)
    s.add_argument("--pc-targets", type=int, default=16)
    s.add_argument("--break-aa", action="store_true")
    s.add_argument("--pc-scale", type=float, default=0.0)
    t = sub.add_parser("selftest")
    t.add_argument("--workdir", required=True)
    t.add_argument("--out", required=True)
    t.add_argument("--resamples", type=int, default=2000)
    args = ap.parse_args()
    if args.cmd == "analyze":
        res = analyze(args)
        with open(args.out, "w") as fh:
            json.dump(res, fh, indent=1, sort_keys=True)
            fh.write("\n")
        print("verdict:", res["verdict"], "theta:", res.get("primary", {}).get("theta"),
              "ci95:", res.get("primary", {}).get("ci95"))
        return 0
    if args.cmd == "synth":
        for d in synth(args):
            print(d)
        return 0
    return selftest(args)


if __name__ == "__main__":
    sys.exit(main())
