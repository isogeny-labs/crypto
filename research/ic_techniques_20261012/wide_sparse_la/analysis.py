#!/usr/bin/env python3
"""Frozen analysis of the wide-modulus sparse linear algebra benchmark.

Implements, exactly and only, PROTOCOL.md ("Counted units", "Controls",
"Acceptance and stop rules", "Analysis (frozen)", "How the extrapolation
model is falsified") as modified by PROTOCOL_AMENDMENT_1.md (one RunPod pod
per seed; A/A per pod).

    analysis.py --raw RAW_DIR --out results.json [--svg-dir DIR] [--md]

RAW_DIR holds seed1/ seed2/ seed3/, each with grid.jsonl aa.jsonl
wide_coeffs.jsonl runs.jsonl host.json done.json (any may be missing).
Pure python3 + numpy (+ matplotlib for --svg-dir).  The pod (directory)
is the seed: the seed recorded in grid/wide records equals the pod's seed;
A/A records are all seed 1 and are told apart by pod.

Test-only knobs (defaults are the frozen values): --resamples (10000),
--width-min-log2 (16).
"""
import argparse
import json
import math
import os
import sys

import numpy as np

BOOT_SEED = 20261112
N_BOOT = 10000
SEEDS = (1, 2, 3)
SOLVERS = ("full-bw", "full-bl")
EXPECTED_LOG2 = tuple(range(12, 21))
WIDTH_MIN_LOG2 = 16
HOLDOUT_LOG2 = 20
CURV_MIN_LOG2 = 14
COUNT_TOL = 1.10
BETA_EXPECT = (1.9, 2.1)
D_TARGET = 10 ** 9
TARGET_ARM = "U192@129"
CONTROL_ARM = "u64@61"
LIMBS = {"u64": 1, "u128": 2, "u192": 3, "u256": 4}
NATURAL_WORD = {61: "u64", 89: "u128", 127: "u128", 129: "u192", 190: "u192", 255: "u256"}
ARMS = ["u64@61", "U128@61", "U128@89", "U128@127", "U192@129", "U192@190", "U256@255"]
SAME_WORD_GROUPS = {"U128": ["U128@61", "U128@89", "U128@127"], "U192": ["U192@129", "U192@190"]}
COUNTED = ("row_ops", "krylov_products", "krylov_lanes", "core_dimension",
           "core_nonzeros", "filter_row_ops", "rows", "nonzeros")
FILES = ("grid", "aa", "wide_coeffs", "runs", "host", "done")


# ----------------------------------------------------------------- helpers
def arm_label(bits, word):
    """word is the record's word_width ('u64','u128','u192','u256')."""
    return "u64@%d" % bits if word == "u64" else "%s@%d" % (word.upper(), bits)


def label_from_runs(bits, word):
    """word as the launcher writes it: 'u64', 'u128' or '-' (natural)."""
    if word in (None, "-", ""):
        word = NATURAL_WORD.get(bits, "?")
    return arm_label(bits, word)


def limbs_of(label):
    return LIMBS[label.split("@")[0].lower()]


def clean(o):
    if isinstance(o, dict):
        return {str(k): clean(v) for k, v in o.items()}
    if isinstance(o, (list, tuple)):
        return [clean(v) for v in o]
    if isinstance(o, (np.floating, float)):
        f = float(o)
        return f if math.isfinite(f) else None
    if isinstance(o, np.integer):
        return int(o)
    if isinstance(o, np.bool_):
        return bool(o)
    return o


def read_jsonl(path):
    out, bad = [], 0
    try:
        with open(path) as f:
            for line in f:
                line = line.strip()
                if not line:
                    continue
                try:
                    v = json.loads(line)
                    if isinstance(v, dict):
                        out.append(v)
                    else:
                        bad += 1
                except ValueError:
                    bad += 1
    except OSError:
        return None, 0
    return out, bad


def read_json(path):
    try:
        with open(path) as f:
            return json.load(f)
    except (OSError, ValueError):
        return None


def log2D(d):
    return int(round(math.log2(d)))


# ----------------------------------------------------------------- Student t
def _betacf(a, b, x):
    tiny = 1e-300
    qab, qap, qam = a + b, a + 1.0, a - 1.0
    c, d = 1.0, 1.0 - qab * x / qap
    d = 1.0 / (d if abs(d) > tiny else tiny)
    h = d
    for m in range(1, 400):
        m2 = 2 * m
        aa = m * (b - m) * x / ((qam + m2) * (a + m2))
        d = 1.0 + aa * d
        d = 1.0 / (d if abs(d) > tiny else tiny)
        c = 1.0 + aa / c
        c = c if abs(c) > tiny else tiny
        h *= d * c
        aa = -(a + m) * (qab + m) * x / ((a + m2) * (qap + m2))
        d = 1.0 + aa * d
        d = 1.0 / (d if abs(d) > tiny else tiny)
        c = 1.0 + aa / c
        c = c if abs(c) > tiny else tiny
        de = d * c
        h *= de
        if abs(de - 1.0) < 1e-15:
            break
    return h


def _betai(a, b, x):
    if x <= 0:
        return 0.0
    if x >= 1:
        return 1.0
    bt = math.exp(math.lgamma(a + b) - math.lgamma(a) - math.lgamma(b)
                  + a * math.log(x) + b * math.log(1 - x))
    if x < (a + 1) / (a + b + 2):
        return bt * _betacf(a, b, x) / a
    return 1 - bt * _betacf(b, a, 1 - x) / b


def t_cdf(t, df):
    x = df / (df + t * t)
    p = 0.5 * _betai(df / 2.0, 0.5, x)
    return 1 - p if t > 0 else p


def t_ppf(p, df):
    lo, hi = 0.0, 1e4
    for _ in range(200):
        mid = 0.5 * (lo + hi)
        if t_cdf(mid, df) < p:
            lo = mid
        else:
            hi = mid
    return 0.5 * (lo + hi)


# ----------------------------------------------------------------- loading
def load_pod(raw, seed):
    d = os.path.join(raw, "seed%d" % seed)
    pod = {"seed": seed, "dir": d, "exists": os.path.isdir(d), "missing_files": [],
           "bad_lines": {}, "records": {}}
    for name in ("grid", "aa", "wide_coeffs", "runs"):
        recs, bad = read_jsonl(os.path.join(d, name + ".jsonl"))
        if recs is None:
            pod["missing_files"].append(name + ".jsonl")
            recs = []
        pod["records"][name] = recs
        pod["bad_lines"][name] = bad
    pod["host"] = read_json(os.path.join(d, "host.json"))
    pod["done"] = read_json(os.path.join(d, "done.json"))
    if pod["host"] is None:
        pod["missing_files"].append("host.json")
    if pod["done"] is None:
        pod["missing_files"].append("done.json")
    return pod


def build_cells(pod, fname):
    """cells[(label, solver, cols)] -> {attempts:[...], skipped:bool}.
    An attempt is one line of the jsonl: a record or a {"failed":true} line."""
    cells = {}
    for r in pod["records"][fname]:
        if r.get("failed"):
            try:
                bits = int(r["modulus_bits"])
                key = (label_from_runs(bits, r.get("word")), r.get("arm"), int(r["cols"]))
            except (KeyError, TypeError, ValueError):
                continue
            att = {"valid": False, "verified": None, "exit": r.get("exit"), "rec": None}
        else:
            try:
                key = (arm_label(int(r["modulus_bits"]), r["word_width"]), r["arm"], int(r["cols"]))
            except (KeyError, TypeError, ValueError):
                continue
            ver = r.get("verified")
            att = {"valid": ver is True, "verified": ver, "exit": 0, "rec": r}
        cells.setdefault(key, {"attempts": [], "skipped": False})["attempts"].append(att)
    if fname == "grid":  # skipped lines of the stop rule are in runs.jsonl
        for r in pod["records"]["runs"]:
            if "skipped" in r:
                try:
                    key = (label_from_runs(int(r["modulus_bits"]), r.get("word")), r.get("arm"), int(r["cols"]))
                except (KeyError, TypeError, ValueError):
                    continue
                cells.setdefault(key, {"attempts": [], "skipped": False})["skipped"] = True
    return cells


def cell_status(c):
    atts = c["attempts"]
    valid = [a for a in atts if a["valid"]]
    if valid:
        return "valid"
    ex = [a["exit"] for a in atts]
    if 124 in ex:
        return "timeout"
    if 137 in ex:
        return "oom"
    if c["skipped"] and not atts:
        return "skipped"
    if len(atts) >= 2:
        return "censored"
    if len(atts) == 1:
        return "invalid_unrepeated"
    return "missing"


def build_all(pods):
    """Return cells[(label,solver,D,seed)] = {"status","rec","attempts","note"}."""
    out = {}
    for pod in pods:
        s = pod["seed"]
        cells = build_cells(pod, "grid")
        for (lab, sol, D), c in cells.items():
            out[(lab, sol, D, s)] = {"status": cell_status(c), "attempts": c["attempts"],
                                      "rec": next((a["rec"] for a in c["attempts"] if a["valid"]), None),
                                      "n_valid": sum(a["valid"] for a in c["attempts"])}
        # stop rule: ignore larger D of an arm after its first timeout / OOM
        stop = {}
        for (lab, sol, D, ss), c in out.items():
            if ss == s and c["status"] in ("timeout", "oom"):
                k = (lab, sol)
                stop[k] = min(stop.get(k, D), D)
        for (lab, sol, D, ss), c in out.items():
            if ss == s and (lab, sol) in stop and D > stop[(lab, sol)] and c["status"] != "skipped":
                c["status"] = "skipped"
                c["rec"] = None
                c["note"] = "ignored after first timeout/OOM at D=%d" % stop[(lab, sol)]
    return out


# ----------------------------------------------------------------- OLS + bootstrap
def groups_by_size(points):
    """points: list of (log2D, y). Return sorted sizes and index groups."""
    xs = sorted(set(p[0] for p in points))
    groups = [[i for i, p in enumerate(points) if p[0] == x] for x in xs]
    return xs, groups


def design(xcol, order):
    x = np.asarray(xcol, float) - 16.0   # centred for conditioning (a is re-based by caller)
    return np.vstack([x ** k for k in range(order + 1)]).T


def boot_index(groups, n_pts, B, rng):
    idx = np.empty((B, n_pts), dtype=np.int64)
    for g in groups:
        g = np.asarray(g)
        idx[:, g] = g[rng.integers(0, len(g), (B, len(g)))]
    return idx


def ols_line(points):
    """Return (a, slope) of log2y = a + slope * log2D (a at log2D = 0)."""
    x = np.array([p[0] for p in points], float)
    y = np.array([p[1] for p in points], float)
    X = np.vstack([np.ones_like(x), x]).T
    coef = np.linalg.lstsq(X, y, rcond=None)[0]
    return float(coef[0]), float(coef[1])


def boot_line(points, B, rng):
    """Bootstrap over seeds within size. Returns arrays a (B,), slope (B,)."""
    x = np.array([p[0] for p in points], float)
    y = np.array([p[1] for p in points], float)
    xs, groups = groups_by_size(points)
    P = np.linalg.pinv(np.vstack([np.ones_like(x), x]).T)
    idx = boot_index(groups, len(points), B, rng)
    coef = y[idx] @ P.T
    return coef[:, 0], coef[:, 1]


def boot_quad(points, B, rng):
    x = np.array([p[0] for p in points], float) - 16.0
    y = np.array([p[1] for p in points], float)
    xs, groups = groups_by_size(points)
    P = np.linalg.pinv(np.vstack([np.ones_like(x), x, x ** 2]).T)
    idx = boot_index(groups, len(points), B, rng)
    coef = y[idx] @ P.T
    return coef[:, 2], P, y


def size_law(points, B, label_extra=None):
    """Fit one quantity. points: list of (log2D, log2 y)."""
    xs, _ = groups_by_size(points)
    if len(xs) < 3:
        return {"status": "insufficient sizes", "sizes_log2": xs, "n_points": len(points)}
    a, sl = ols_line(points)
    rng = np.random.default_rng(BOOT_SEED)
    ba, bs = boot_line(points, B, rng)
    res = {"status": "ok", "sizes_log2": xs, "n_points": len(points),
           "a": a, "a_ci95": [float(np.percentile(ba, 2.5)), float(np.percentile(ba, 97.5))],
           "slope": sl, "beta": sl + 1.0,
           "beta_ci95": [float(np.percentile(bs, 2.5)) + 1.0, float(np.percentile(bs, 97.5)) + 1.0],
           "bootstrap": {"resamples": B, "generator_seed": BOOT_SEED, "scheme": "percentile, seeds resampled within size"}}
    x = np.array([p[0] for p in points], float)
    y = np.array([p[1] for p in points], float)
    res["resid_sd_log2"] = float(np.std(y - (a + sl * x), ddof=2)) if len(points) > 2 else None
    return res


# ----------------------------------------------------------------- main analysis
def analyse(raw, B):
    notes = []
    pods = [load_pod(raw, s) for s in SEEDS]
    cells = build_all(pods)
    pods_info = {}
    for p in pods:
        h = p["host"] or {}
        pods_info["seed%d" % p["seed"]] = {
            "dir_exists": p["exists"], "missing_files": p["missing_files"],
            "bad_json_lines": p["bad_lines"],
            "status": (p["done"] or {}).get("status"),
            "host": {k: h.get(k) for k in ("nproc", "cpu_model", "mem_total_kb", "rustc", "kernel",
                                           "cgroup_cpu_max", "cgroup_mem_max", "binary_sha256",
                                           "src_sha256", "pod_id")},
            "n_grid_lines": len(p["records"]["grid"]), "n_runs_lines": len(p["records"]["runs"]),
        }
    cpus = {(p["host"] or {}).get("cpu_model") for p in pods if p["host"]}
    bins = {(p["host"] or {}).get("binary_sha256") for p in pods if p["host"]}
    if len(cpus) > 1:
        notes.append("pods report different cpu_model values: %s; wall-time comparisons across seeds cross hosts" % sorted(map(str, cpus)))
    if len(bins) > 1:
        notes.append("pods report different binary_sha256 values: %s" % sorted(map(str, bins)))
    # runs.jsonl vs record validity cross-check
    integrity = {}
    for p in pods:
        runs = [r for r in p["records"]["runs"] if "skipped" not in r]
        integrity["seed%d" % p["seed"]] = {
            "runs_attempt_lines": len(runs),
            "runs_valid": sum(1 for r in runs if r.get("valid") is True),
            "record_lines_all_files": sum(len(p["records"][f]) for f in ("grid", "aa", "wide_coeffs")),
            "records_verified_true_all_files": sum(1 for f in ("grid", "aa", "wide_coeffs")
                                                   for r in p["records"][f] if r.get("verified") is True)}

    valid = {k: c["rec"] for k, c in cells.items() if c["status"] == "valid"}

    # grid sanity: seed recorded vs pod
    for (lab, sol, D, s), c in cells.items():
        if c["rec"] is not None and c["rec"].get("seed") != s:
            notes.append("record seed %r in seed%d/grid.jsonl for %s %s D=%d" % (c["rec"].get("seed"), s, lab, sol, D))
            break

    # ---- coverage
    sizes_seen = sorted(set(k[2] for k in cells))
    cover_D = sorted(set(sizes_seen) | set(2 ** k for k in EXPECTED_LOG2))
    coverage = {}
    status_names = ("valid", "censored", "skipped", "timeout", "oom", "invalid_unrepeated", "missing")
    for lab in ARMS:
        for sol in SOLVERS:
            for D in cover_D:
                cnt = dict.fromkeys(status_names, 0)
                for s in SEEDS:
                    c = cells.get((lab, sol, D, s))
                    st = c["status"] if c else "missing"
                    cnt[st] = cnt.get(st, 0) + 1
                coverage["%s|%s|%d" % (lab, sol, D)] = cnt
    totals = dict.fromkeys(status_names, 0)
    for v in coverage.values():
        for k, n in v.items():
            totals[k] = totals.get(k, 0) + n
    repeated = sum(1 for c in cells.values() if c["status"] == "valid" and len(c["attempts"]) > 1)
    multi_valid = sum(1 for c in cells.values() if c.get("n_valid", 0) > 1)
    if multi_valid:
        notes.append("%d cells had two valid attempts; the first valid one is used" % multi_valid)

    # ---- size laws
    size = {}
    for lab in ARMS:
        for sol in SOLVERS:
            ent = {}
            for qname, fn in (("row_ops", lambda r, D: r["row_ops"] / D),
                              ("core_dimension", lambda r, D: r["core_dimension"] / D),
                              ("core_nonzeros", lambda r, D: r["core_nonzeros"] / D)):
                pts = []
                for (l, so, D, s), r in valid.items():
                    if l == lab and so == sol:
                        try:
                            y = fn(r, D)
                        except (KeyError, TypeError):
                            continue
                        if y > 0:
                            pts.append((log2D(D), math.log2(y)))
                ent[qname] = size_law(pts, B)
            r0 = ent["row_ops"]
            if r0.get("status") == "ok":
                r0["beta_in_expected_range_1.9_2.1"] = bool(BETA_EXPECT[0] <= r0["beta"] <= BETA_EXPECT[1])
            size["%s|%s" % (lab, sol)] = ent

    # ---- width law (timed)
    width = {}
    wmin = WIDTH_MIN_LOG2
    ns = {}  # (lab, sol, D, s) -> ns per row op
    for (lab, sol, D, s), r in valid.items():
        try:
            if r["row_ops"] > 0 and r["solve_s"] is not None:
                ns[(lab, sol, D, s)] = 1e9 * r["solve_s"] / r["row_ops"]
        except (KeyError, TypeError):
            pass

    # A/A first (the bands are needed by the width law)
    aa = {"per_pod": {}, "determinism": {}}
    aa_recs = {}
    for p in pods:
        for r in p["records"]["aa"]:
            if r.get("failed") or "word_width" not in r:
                continue
            lab = arm_label(int(r["modulus_bits"]), r["word_width"])
            aa_recs.setdefault((lab, r["arm"]), []).append((p["seed"], r))
        bands = {}
        for lab in (TARGET_ARM, CONTROL_ARM):
            ts = [r["solve_s"] for (sd, r) in aa_recs.get((lab, "full-bw"), []) if sd == p["seed"] and r.get("solve_s")]
            bands[lab] = {"n_runs": len(ts), "solve_s": ts,
                          "band_max_over_min": (max(ts) / min(ts)) if len(ts) >= 2 and min(ts) > 0 else None}
        bv = [b["band_max_over_min"] for b in bands.values() if b["band_max_over_min"] is not None]
        aa["per_pod"]["seed%d" % p["seed"]] = {"arms": bands, "pod_band": max(bv) if bv else None}
    all_det = True
    for (lab, sol), lst in sorted(aa_recs.items()):
        tuples = {}
        for sd, r in lst:
            tuples.setdefault(tuple(r.get(k) for k in COUNTED), []).append(sd)
        ident = len(tuples) == 1
        all_det &= ident
        diff = [k for i, k in enumerate(COUNTED) if len({t[i] for t in tuples}) > 1]
        aa["determinism"]["%s|%s" % (lab, sol)] = {
            "n_runs": len(lst), "pods": sorted(set(sd for sd, _ in lst)),
            "identical": ident, "fields_that_differ": diff,
            "values": [dict(zip(COUNTED, t)) for t in tuples]}
    aa["all_identical"] = all_det if aa_recs else None
    aa["expected_runs_per_arm"] = "3 per pod (before, mid, after), 9 over three pods"
    if not aa_recs:
        notes.append("no A/A records found")
    if aa_recs and not all_det:
        notes.append("A/A DETERMINISM FAILURE: per protocol the session is void")
    pod_band = {s: aa["per_pod"]["seed%d" % s]["pod_band"] for s in SEEDS}

    Ds_w = sorted(set(D for (_, _, D, _) in ns if log2D(D) >= wmin))
    width["min_log2_D"] = wmin
    width["bands_used_per_pod"] = pod_band
    width["table"] = {}
    width_law_viol = []
    for sol in SOLVERS:
        for D in Ds_w:
            ent = {"arms": {}, "limb_fit": None}
            for lab in ARMS:
                vals = {s: ns[(lab, sol, D, s)] for s in SEEDS if (lab, sol, D, s) in ns}
                if not vals:
                    continue
                v = np.array(list(vals.values()))
                e = {"limbs": limbs_of(lab), "n_seeds": len(v), "ns_per_row_op_median": float(np.median(v)),
                     "min": float(v.min()), "max": float(v.max()), "per_seed": {str(k): x for k, x in vals.items()}}
                ratios = {}
                for s, x in vals.items():
                    base = ns.get((CONTROL_ARM, sol, D, s))
                    if base:
                        rr = x / base
                        b = pod_band.get(s)
                        ratios[str(s)] = {"ratio_to_u64_61": rr, "pod_band": b,
                                          "resolved": (None if b is None else bool(rr > b or rr < 1.0 / b))}
                if ratios:
                    rv = np.array([q["ratio_to_u64_61"] for q in ratios.values()])
                    e["ratio_to_u64_61"] = {"median": float(np.median(rv)), "min": float(rv.min()),
                                            "max": float(rv.max()), "per_seed_within_pod": ratios}
                ent["arms"][lab] = e
            # limb fit per pod (t(L) = median over arms with L limbs; OLS c0 + c2 L^2)
            fits = {}
            for s in SEEDS:
                tl = {}
                for L in (1, 2, 3, 4):
                    xs = [ns[(lab, sol, D, s)] for lab in ARMS if limbs_of(lab) == L and (lab, sol, D, s) in ns]
                    if xs:
                        tl[L] = float(np.median(xs))
                if len(tl) == 4:
                    Ls = np.array(sorted(tl), float)
                    X = np.vstack([np.ones(4), Ls ** 2]).T
                    c0, c2 = np.linalg.lstsq(X, np.array([tl[int(l)] for l in Ls]), rcond=None)[0]
                    fits[s] = {"t_L_ns": tl, "c0": float(c0), "c2": float(c2),
                               "fitted_t_ns": {str(L): float(c0 + c2 * L * L) for L in (1, 2, 3, 4)},
                               "fitted_ratio_t_L_over_t_1": {str(L): float((c0 + c2 * L * L) / (c0 + c2)) for L in (1, 2, 3, 4)},
                               "observed_ratio_t_L_over_t_1": {str(L): float(tl[L] / tl[1]) for L in (1, 2, 3, 4)}}
            if fits:
                summ = {}
                for key in ("c0", "c2"):
                    v = np.array([f[key] for f in fits.values()])
                    summ[key] = {"median": float(np.median(v)), "min": float(v.min()), "max": float(v.max())}
                for key in ("fitted_ratio_t_L_over_t_1", "observed_ratio_t_L_over_t_1"):
                    summ[key] = {}
                    for L in ("1", "2", "3", "4"):
                        v = np.array([f[key][L] for f in fits.values()])
                        summ[key][L] = {"median": float(np.median(v)), "min": float(v.min()), "max": float(v.max())}
                ent["limb_fit"] = {"per_seed": {str(k): v for k, v in fits.items()}, "summary_over_seeds": summ,
                                   "n_seeds": len(fits)}
            else:
                ent["limb_fit"] = {"status": "needs all four limb counts in one pod"}
            width["table"]["%s|%d" % (sol, D)] = ent
            # same-word comparisons within a pod against the pod's A/A band
            for gname, labs in SAME_WORD_GROUPS.items():
                for s in SEEDS:
                    v = [ns[(l, sol, D, s)] for l in labs if (l, sol, D, s) in ns]
                    if len(v) >= 2:
                        rr = max(v) / min(v)
                        b = pod_band.get(s)
                        width_law_viol.append({"solver": sol, "D": D, "seed": s, "group": gname,
                                               "arms_present": [l for l in labs if (l, sol, D, s) in ns],
                                               "max_over_min_ns": rr, "pod_band": b,
                                               "beyond_band": None if b is None else bool(rr > b)})
    # limb control ratio
    # (reported inside width.table as ratio_to_u64_61 of U128@61)

    # ---- secondary: wide coeffs vs unit
    wide_cells = {}
    for p in pods:
        for (lab, sol, D), c in build_cells(p, "wide_coeffs").items():
            wide_cells[(lab, sol, D, p["seed"])] = next((a["rec"] for a in c["attempts"] if a["valid"]), None)
    wide = {}
    for (lab, sol, D, s), r in sorted(wide_cells.items(), key=str):
        u = valid.get((lab, sol, D, s))
        e = {"wide_row_ops": r["row_ops"] if r else None, "unit_row_ops": u["row_ops"] if u else None}
        e["ratio_wide_over_unit"] = (r["row_ops"] / u["row_ops"]) if r and u and u["row_ops"] else None
        wide["%s|%s|D=%d|seed%d" % (lab, sol, D, s)] = e

    # ---- falsification
    fals = {}
    # hold-out and curvature per (arm, solver)
    hold, curv = {}, {}
    for lab in ARMS:
        for sol in SOLVERS:
            pts = [(log2D(D), math.log2(r["row_ops"] / D), s) for (l, so, D, s), r in valid.items()
                   if l == lab and so == sol]
            key = "%s|%s" % (lab, sol)
            tr = [(x, y) for x, y, s in pts if x < HOLDOUT_LOG2]
            te = [(x, y, s) for x, y, s in pts if x == HOLDOUT_LOG2]
            trx = sorted(set(x for x, _ in tr))
            if len(trx) < 3 or not te:
                hold[key] = {"status": "not evaluable", "reason": "needs >=3 sizes below 2^%d and observed D=2^%d" % (HOLDOUT_LOG2, HOLDOUT_LOG2),
                             "train_sizes_log2": trx, "n_test": len(te)}
            else:
                x = np.array([p[0] for p in tr], float)
                y = np.array([p[1] for p in tr], float)
                n = len(x)
                a, sl = ols_line(tr)
                s2 = float(np.sum((y - a - sl * x) ** 2) / (n - 2))
                sxx = float(np.sum((x - x.mean()) ** 2))
                tq = t_ppf(0.975, n - 2)
                per = []
                for xt, yt, s in te:
                    pred = a + sl * xt
                    half = tq * math.sqrt(s2 * (1 + 1.0 / n + (xt - x.mean()) ** 2 / sxx))
                    per.append({"seed": s, "observed_log2_y": yt, "predicted_log2_y": pred,
                                "pi95": [pred - half, pred + half], "outside": bool(abs(yt - pred) > half)})
                bad = any(q["outside"] for q in per)
                hold[key] = {"status": "FAIL" if bad else "pass", "train_sizes_log2": trx, "df": n - 2,
                             "t_975": tq, "per_seed": per}
            cs = [(x, y) for x, y, s in pts if x >= CURV_MIN_LOG2]
            csx = sorted(set(x for x, _ in cs))
            if len(csx) < 4:
                curv[key] = {"status": "not evaluable", "reason": "needs >=4 sizes at D>=2^%d" % CURV_MIN_LOG2, "sizes_log2": csx}
            else:
                rng = np.random.default_rng(BOOT_SEED)
                b2, P, y = boot_quad(cs, B, rng)
                x = np.array([p[0] for p in cs], float) - 16.0
                coef = (np.linalg.pinv(np.vstack([np.ones_like(x), x, x ** 2]).T) @ np.array([p[1] for p in cs]))
                lo, hi = float(np.percentile(b2, 2.5)), float(np.percentile(b2, 97.5))
                curv[key] = {"status": "FAIL" if (lo > 0 or hi < 0) else "pass", "sizes_log2": csx,
                             "coef_log2D_sq": float(coef[2]), "ci95": [lo, hi]}
    fals["holdout"] = hold
    fals["curvature"] = curv
    # width independence of the count
    worst, ncells = None, 0
    for sol in SOLVERS:
        for D in sorted(set(k[2] for k in valid)):
            for s in SEEDS:
                v = {l: valid[(l, sol, D, s)]["row_ops"] for l in ARMS if (l, sol, D, s) in valid}
                if len(v) >= 2:
                    ncells += 1
                    rr = max(v.values()) / min(v.values())
                    if worst is None or rr > worst["max_over_min"]:
                        worst = {"solver": sol, "D": D, "seed": s, "max_over_min": rr,
                                 "max_arm": max(v, key=v.get), "min_arm": min(v, key=v.get)}
    fals["width_independence_of_count"] = {
        "threshold": COUNT_TOL, "n_cells_compared": ncells, "worst": worst,
        "status": ("not evaluable" if worst is None else ("FAIL" if worst["max_over_min"] > COUNT_TOL else "pass"))}
    # width law
    evald = [v for v in width_law_viol if v["beyond_band"] is not None]
    fals["width_law"] = {
        "status": "not evaluable" if not evald else ("FAIL" if any(v["beyond_band"] for v in evald) else "pass"),
        "n_comparisons": len(width_law_viol), "n_without_band": len(width_law_viol) - len(evald),
        "violations": [v for v in evald if v["beyond_band"]], "comparisons": width_law_viol}
    # planted
    plant = []
    for (lab, sol, D, s), c in sorted(cells.items(), key=str):
        if c["status"] in ("censored", "invalid_unrepeated") and any(a["verified"] is False for a in c["attempts"]):
            plant.append({"arm": lab, "solver": sol, "D": D, "seed": s, "attempts": len(c["attempts"]),
                          "survived_repeat": len(c["attempts"]) >= 2})
    surv = [q for q in plant if q["survived_repeat"]]
    fals["planted_check"] = {"status": "FAIL" if surv else "pass", "unverified_cells": plant,
                             "n_surviving_repeat": len(surv)}
    # verdict for the extrapolation target, per solver
    verdict = {}
    for sol in SOLVERS:
        key = "%s|%s" % (TARGET_ARM, sol)
        parts = {"holdout": hold[key]["status"], "curvature": curv[key]["status"],
                 "width_independence_of_count": fals["width_independence_of_count"]["status"],
                 "width_law": fals["width_law"]["status"],
                 "planted_check_target_arm": "FAIL" if any(q["arm"] == TARGET_ARM and q["solver"] == sol and q["survived_repeat"] for q in plant) else "pass"}
        if any(v == "FAIL" for v in parts.values()):
            v = "rejected: extrapolation unsupported"
        elif any(v == "not evaluable" for v in parts.values()):
            v = "undetermined: not all tests evaluable"
        else:
            v = "survives all five tests"
        verdict[sol] = {"tests": parts, "verdict": v,
                        "session_void_aa_determinism_failure": (None if not aa_recs else (not all_det))}
    fals["verdict_for_extrapolation_target"] = verdict

    # ---- extrapolation (joint bootstrap)
    extrap = {}
    for sol in SOLVERS:
        key = "%s|%s" % (TARGET_ARM, sol)
        sl = size.get(key, {}).get("row_ops", {})
        ent = {"label": "extrapolated, about 3 decades beyond the largest measured size (2^20 ~ 1.05e6 -> 1e9); "
                        "single-host CPU seconds; no communication, distribution or memory beyond one host",
               "arm": TARGET_ARM, "solver": sol, "D_target": D_TARGET}
        if sl.get("status") != "ok":
            ent["status"] = "insufficient sizes"
            extrap[sol] = ent
            continue
        # t(3) per pod from the limb fit at the largest D with all four limb counts
        ref = None
        for D in sorted(Ds_w, reverse=True):
            lf = width["table"]["%s|%d" % (sol, D)]["limb_fit"]
            if lf and "per_seed" in lf:
                ref = D
                break
        if ref is None:
            ent["status"] = "no t(3): width law needs D>=2^%d with all four limb counts in a pod" % wmin
            extrap[sol] = ent
            continue
        lf = width["table"]["%s|%d" % (sol, ref)]["limb_fit"]["per_seed"]
        t3 = np.array([f["fitted_t_ns"]["3"] for f in lf.values()])
        t3_point = float(np.median(t3))
        pts = [(log2D(D), math.log2(r["row_ops"] / D)) for (l, so, D, s), r in valid.items()
               if l == TARGET_ARM and so == sol]
        a, slope = ols_line(pts)
        lg = math.log2(D_TARGET)
        row_ops_pt = 2.0 ** (lg + a + slope * lg)
        rng = np.random.default_rng(BOOT_SEED)
        ba, bs = boot_line(pts, B, rng)
        idx = rng.integers(0, len(t3), (B, len(t3)))
        bt3 = np.median(t3[idx], axis=1)
        row_ops_b = np.exp2(lg + ba + bs * lg)
        time_b = row_ops_b * bt3 * 1e-9
        time_pt = row_ops_pt * t3_point * 1e-9
        pc = lambda v: [float(np.percentile(v, 2.5)), float(np.percentile(v, 97.5))]
        ent.update({"status": "ok", "size_law_sizes_log2": sl["sizes_log2"], "a": a, "beta": slope + 1.0,
                    "D_ref_for_t3_log2": log2D(ref), "t3_ns_per_row_op_fitted_median_over_pods": t3_point,
                    "t3_n_pods": int(len(t3)),
                    "t3_ci95_ns": pc(bt3),
                    "row_ops_at_target": row_ops_pt, "row_ops_ci95": pc(row_ops_b),
                    "time_s": time_pt, "time_s_ci95": pc(time_b),
                    "time_h": time_pt / 3600.0, "time_h_ci95": [x / 3600.0 for x in pc(time_b)],
                    "time_days": time_pt / 86400.0, "time_days_ci95": [x / 86400.0 for x in pc(time_b)],
                    "interval": "95%% percentile of the joint bootstrap (size law a,beta resampled over seeds within size; "
                                "t(3) = median over resampled pods of the per-pod fitted c0+9*c2); %d resamples, generator seed %d" % (B, BOOT_SEED),
                    "falsification_verdict": verdict[sol]["verdict"]})
        extrap[sol] = ent

    # ---- notes (ambiguities resolved)
    notes += [
        "Seed = pod = directory seedN; A/A records (seed 1 in every pod) are attributed to their pod by directory.",
        "Arm identity = (modulus_bits, word_width) from the record; word_width 'u64' is the u64@61 control, 'u128' at 61 bits is U128@61. Failed lines carry the launcher's word ('u64','u128','-') and are mapped the same way ('-' = the natural word of the bit size).",
        "A run is valid iff its record has verified == true (the launcher writes a record only on exit 0). Failed lines and verified:false records are invalid attempts. Two valid attempts: first used.",
        "Cell status: valid; timeout (exit 124 on any attempt); oom (exit 137); skipped (runs.jsonl skipped line, or a larger D ignored after the arm's first timeout/OOM on that pod); censored (>=2 invalid attempts); invalid_unrepeated (one invalid attempt, no repeat yet); missing.",
        "Size-law fits need >=3 distinct sizes (else 'insufficient sizes'); y with value <= 0 is skipped. Bootstrap: for every size, seeds drawn with replacement among that size's valid points (same count), refit, 10,000 resamples, a fresh numpy default_rng(20261112) per fit; percentile 2.5/97.5; beta = slope + 1.",
        "The size law is fitted for every (arm, solver) that has data, including arms other than the extrapolation target; the five falsification tests are reported per (arm, solver) where they are per-arm (hold-out, curvature) and globally otherwise; the verdict for the extrapolation is evaluated on U192@129 per solver plus the global tests.",
        "Hold-out: OLS on sizes <= 2^19 (>=3 sizes), classical Student-t 95% prediction interval for one new observation at 2^20 on log2(row_ops/D); fail if any seed's observation lies outside. Curvature: quadratic in log2 D over D >= 2^14 (>=4 sizes), bootstrap as above; fail if the 95% interval of the (log2 D)^2 coefficient excludes 0.",
        "Width independence: max/min row_ops over the valid width arms of each (solver, D, seed) cell (cells with >=2 arms), threshold 1.10 at all D.",
        "Width law t(L): per pod, solver and D >= 2^16, t(L) = median ns_per_row_op over the arms with L limbs (L=1 u64@61; L=2 U128@61/89/127; L=3 U192@129/190; L=4 U256@255); OLS t = c0 + c2 L^2 over the four L; ratios t(L)/t(1) reported fitted and observed, median and min-max over pods. Ratios of individual arms to u64@61 are taken within a pod (same seed, solver, D).",
        "A/A wall-time band per pod and arm = max/min solve_s over that pod's A/A runs of the arm; the pod band used for 'beyond the band' and 'not resolved' = the larger of the pod's two arm bands (U192@129 and u64@61). The band is measured at D=2^16 and applied at every D >= 2^16.",
        "Same-word width-law test: within each pod, solver and D >= 2^16, max/min ns_per_row_op over U128@61/89/127 (and over U192@129/190) must not exceed the pod band; any exceedance in any pod fails the test. A ratio is 'resolved' only if it lies outside [1/band, band].",
        "Planted check: fails if a cell without a valid record has a verified:false attempt and at least two attempts (it survived its repeat). Applied over all cells (all arms are used in some fit); the verdict looks at the target arm separately.",
        "Extrapolation: row_ops(1e9) = 1e9 * 2^(a + (beta-1) log2 1e9) from the U192@129 size law of the solver; t(3) = median over pods of the fitted c0 + 9 c2 at the largest D >= 2^16 with all four limb counts in a pod; time = row_ops * t(3) * 1e-9 s. Interval: joint bootstrap (the (a,beta) resample and a resample of the pods' t(3) in the same iteration).",
        "Wide-coeffs secondary: ratio of wide to unit row_ops for the same (arm, solver, D, seed); unit counterpart from grid.jsonl; no threshold applied.",
        "Cells at D without 2^12..2^20 coverage reduce the fits to the sizes present; the sizes used are listed in each fit.",
    ]
    ok_sizes = sorted(set(log2D(k[2]) for k in valid))
    return {
        "protocol": "research/ic_techniques_20261012/wide_sparse_la/PROTOCOL.md + PROTOCOL_AMENDMENT_1.md",
        "raw_dir": os.path.abspath(raw),
        "pods": pods_info, "integrity": integrity,
        "sizes_log2_with_valid_data": ok_sizes,
        "coverage_totals_over_cells": totals, "cells_valid_after_repeat": repeated,
        "coverage": coverage,
        "size_law": size, "width_law": width, "extrapolation": extrap,
        "falsification": fals, "aa": aa, "wide_coeffs": wide, "notes": notes,
    }, valid, ns


# ----------------------------------------------------------------- svg
def make_svgs(res, valid, ns, outdir):
    import matplotlib
    matplotlib.use("Agg")
    import matplotlib.pyplot as plt
    os.makedirs(outdir, exist_ok=True)
    col = ["#000000", "#E69F00", "#56B4E9", "#009E73", "#D55E00", "#0072B2", "#CC79A7"]
    mk = ["o", "s", "^", "v", "D", "P", "X"]
    # 1: row_ops / D vs D
    fig, axes = plt.subplots(1, 2, figsize=(11, 4.5), sharey=True)
    for ax, sol in zip(axes, SOLVERS):
        n = 0
        for i, lab in enumerate(ARMS):
            pts = sorted((D, r["row_ops"] / D, s) for (l, so, D, s), r in valid.items() if l == lab and so == sol)
            if not pts:
                continue
            n += len(pts)
            ax.scatter([p[0] for p in pts], [p[1] for p in pts], s=14, color=col[i], marker=mk[i], alpha=0.6)
            Ds = sorted(set(p[0] for p in pts))
            ax.plot(Ds, [np.median([p[1] for p in pts if p[0] == D]) for D in Ds], color=col[i], lw=1.2, label=lab)
        ax.set_xscale("log", base=2)
        ax.set_yscale("log")
        ax.set_xlabel("D (columns)")
        ax.set_title(sol)
        ax.grid(alpha=0.3, which="both")
        if n == 0:
            ax.text(0.5, 0.5, "no valid data", transform=ax.transAxes, ha="center")
        else:
            ax.legend(fontsize=7, title="width arm", title_fontsize=7)
    axes[0].set_ylabel("row_ops / D (counted)")
    fig.suptitle("Counted work per column against D (points: seeds; line: median)")
    fig.tight_layout()
    fig.savefig(os.path.join(outdir, "row_ops_per_col_vs_D.svg"))
    plt.close(fig)
    # 2: ns/row_op vs limbs
    fig, axes = plt.subplots(1, 2, figsize=(11, 4.5), sharey=True)
    Ds_w = sorted(set(D for (_, _, D, _) in ns if log2D(D) >= WIDTH_MIN_LOG2))
    cm = plt.get_cmap("viridis")
    for ax, sol in zip(axes, SOLVERS):
        for j, D in enumerate(Ds_w):
            c = cm(j / max(1, len(Ds_w) - 1) * 0.9)
            xs, med, lo, hi = [], [], [], []
            for lab in ARMS:
                v = [ns[(lab, sol, D, s)] for s in SEEDS if (lab, sol, D, s) in ns]
                if v:
                    xs.append(limbs_of(lab))
                    med.append(np.median(v))
                    lo.append(np.median(v) - min(v))
                    hi.append(max(v) - np.median(v))
            if xs:
                ax.errorbar(xs, med, yerr=[lo, hi], fmt="o", ms=4, color=c, capsize=2, lw=0.8, label="D=2^%d" % log2D(D))
        ax.set_xticks([1, 2, 3, 4])
        ax.set_xlabel("limbs L (u64=1, U128=2, U192=3, U256=4)")
        ax.set_title(sol)
        ax.grid(alpha=0.3)
        if not Ds_w:
            ax.text(0.5, 0.5, "no D >= 2^%d data" % WIDTH_MIN_LOG2, transform=ax.transAxes, ha="center")
        else:
            ax.legend(fontsize=7)
    axes[0].set_ylabel("ns per row_op (1e9 * solve_s / row_ops; derived)")
    fig.suptitle("Time per counted operation against limbs (median, min-max over pods)")
    fig.tight_layout()
    fig.savefig(os.path.join(outdir, "ns_per_row_op_vs_limbs.svg"))
    plt.close(fig)


# ----------------------------------------------------------------- markdown
def f(x, p=3):
    return "-" if x is None else ("%.*g" % (p, x))


def make_md(res):
    L = []
    L.append("# Wide-modulus sparse LA: analysis summary")
    t = res["coverage_totals_over_cells"]
    L.append("\nCoverage over (arm, solver, D, seed) cells: " + ", ".join("%s %d" % (k, v) for k, v in t.items() if v)
             + "; sizes with valid data (log2): %s" % res["sizes_log2_with_valid_data"])
    for k, p in res["pods"].items():
        L.append("- %s: status %s, missing files %s" % (k, p["status"], p["missing_files"] or "none"))
    L.append("\n## Size law, row_ops/D (beta = slope + 1, 95% bootstrap)\n")
    L.append("| arm | solver | beta | CI | core_dim beta | core_nnz beta | note |\n|:--|:--|--:|:--|--:|--:|:--|")
    for k, e in res["size_law"].items():
        arm, sol = k.split("|")
        r = e["row_ops"]
        if r["status"] != "ok":
            if r["n_points"]:
                L.append("| %s | %s | - | - | - | - | %s |" % (arm, sol, r["status"]))
            continue
        cd, cn = e["core_dimension"], e["core_nonzeros"]
        L.append("| %s | %s | %s | [%s, %s] | %s | %s | %s |" % (
            arm, sol, f(r["beta"]), f(r["beta_ci95"][0]), f(r["beta_ci95"][1]),
            f(cd.get("beta")), f(cn.get("beta")), "in 1.9-2.1" if r.get("beta_in_expected_range_1.9_2.1") else "outside 1.9-2.1"))
    L.append("\n## Width law (t(L)/t(1), median [min, max] over pods)\n")
    wt = res["width_law"]["table"]
    if not wt:
        L.append("No D >= 2^%d data." % res["width_law"]["min_log2_D"])
    else:
        L.append("| solver | D | t(1) ns | fit c0, c2 | t(2)/t(1) | t(3)/t(1) | t(4)/t(1) |\n|:--|--:|--:|:--|:--|:--|:--|")
        for k, e in wt.items():
            sol, D = k.split("|")
            lf = e["limb_fit"]
            t1 = e["arms"].get(CONTROL_ARM, {}).get("ns_per_row_op_median")
            if lf and "summary_over_seeds" in lf:
                s = lf["summary_over_seeds"]
                rr = [s["observed_ratio_t_L_over_t_1"][str(i)] for i in (2, 3, 4)]
                L.append("| %s | 2^%d | %s | %s, %s | %s | %s | %s |" % (
                    sol, log2D(int(D)), f(t1), f(s["c0"]["median"]), f(s["c2"]["median"]),
                    *["%s [%s, %s]" % (f(r["median"]), f(r["min"]), f(r["max"])) for r in rr]))
            else:
                L.append("| %s | 2^%d | %s | no 4-limb fit | | | |" % (sol, log2D(int(D)), f(t1)))
    L.append("\n## Extrapolation to D = 1e9, %s (EXTRAPOLATED)\n" % TARGET_ARM)
    for sol, e in res["extrapolation"].items():
        if e.get("status") != "ok":
            L.append("- %s: %s" % (sol, e.get("status")))
        else:
            L.append("- %s: row_ops %s [%s, %s], t(3) %s ns, time %s h [%s, %s] (%s days), law: %s" % (
                sol, f(e["row_ops_at_target"]), f(e["row_ops_ci95"][0]), f(e["row_ops_ci95"][1]),
                f(e["t3_ns_per_row_op_fitted_median_over_pods"]), f(e["time_h"]), f(e["time_h_ci95"][0]),
                f(e["time_h_ci95"][1]), f(e["time_days"]), e["falsification_verdict"]))
    L.append("\n## Falsification tests\n")
    fa = res["falsification"]
    L.append("| test | solver | status | detail |\n|:--|:--|:--|:--|")
    for sol in SOLVERS:
        k = "%s|%s" % (TARGET_ARM, sol)
        h, c = fa["holdout"][k], fa["curvature"][k]
        L.append("| hold-out (%s) | %s | %s | %s |" % (TARGET_ARM, sol, h["status"], h.get("reason", "")))
        L.append("| curvature (%s) | %s | %s | %s |" % (TARGET_ARM, sol, c["status"],
                 c.get("reason", "coef %s CI [%s, %s]" % (f(c.get("coef_log2D_sq")), f((c.get("ci95") or [None, None])[0]), f((c.get("ci95") or [None, None])[1])))))
    w = fa["width_independence_of_count"]
    L.append("| width independence of count | all | %s | worst max/min %s |" % (w["status"], f((w["worst"] or {}).get("max_over_min"))))
    wl = fa["width_law"]
    L.append("| width law (same word) | all | %s | %d comparisons, %d beyond band |" % (wl["status"], wl["n_comparisons"], len(wl["violations"])))
    pc = fa["planted_check"]
    L.append("| planted check | all | %s | %d surviving unverified cells |" % (pc["status"], pc["n_surviving_repeat"]))
    for sol, v in fa["verdict_for_extrapolation_target"].items():
        L.append("\nVerdict %s: %s" % (sol, v["verdict"]))
    L.append("\n## A/A\n")
    L.append("Determinism (counted units identical across all A/A runs per arm): %s" % res["aa"]["all_identical"])
    for k, p in res["aa"]["per_pod"].items():
        L.append("- %s pod band %s (%s)" % (k, f(p["pod_band"]), ", ".join("%s n=%d band %s" % (a, b["n_runs"], f(b["band_max_over_min"])) for a, b in p["arms"].items())))
    L.append("\nNotes: %d ambiguity/handling notes in results.json `notes`." % len(res["notes"]))
    return "\n".join(L)


def main():
    global WIDTH_MIN_LOG2
    ap = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    ap.add_argument("--raw", required=True)
    ap.add_argument("--out", required=True)
    ap.add_argument("--svg-dir")
    ap.add_argument("--md", action="store_true")
    ap.add_argument("--resamples", type=int, default=N_BOOT, help="test only; frozen value 10000")
    ap.add_argument("--width-min-log2", type=int, default=WIDTH_MIN_LOG2, help="test only; frozen value 16")
    a = ap.parse_args()
    WIDTH_MIN_LOG2 = a.width_min_log2
    res, valid, ns = analyse(a.raw, a.resamples)
    if a.resamples != N_BOOT or a.width_min_log2 != 16:
        res["notes"].append("NON-FROZEN test knobs used: resamples=%d width_min_log2=%d" % (a.resamples, a.width_min_log2))
    with open(a.out, "w") as fh:
        json.dump(clean(res), fh, indent=1, allow_nan=False)
    if a.svg_dir:
        try:
            make_svgs(res, valid, ns, a.svg_dir)
        except ImportError as e:
            print("matplotlib unavailable, no SVG written: %s" % e, file=sys.stderr)
    if a.md:
        print(make_md(clean(res)))


if __name__ == "__main__":
    main()
