#!/usr/bin/env python3
"""Validator recompute for research/genus4_cost_calibration_20261119 (written without reusing analyze.py).

Reads the committed *.jsonl.gz files (checks each equals the gitignored *.jsonl beside it), re-derives
every headline number of RESULTS.md, and writes validation/recompute.json.

Independent choices (deliberately different from analyze.py):
  - rows are keyed by the row's own fields (p, curve_seed, target_seed, seed, w_max, fill_max) and the
    config row's cc, cross-checked against the file name, not by regex on the arm string;
  - OLS by numpy.linalg.lstsq (SVD) on column-scaled regressors, not Gauss-Jordan normal equations;
  - cluster bootstrap with numpy's PCG64 (seed 4242, 4000 resamples), percentile 2.5/97.5 by
    numpy.quantile; a separate, clearly labelled replica of the frozen resampling stream
    (random.Random(20261125), 2000 resamples, analyze.py's index rule) is computed only to check that the
    reported intervals are what the frozen procedure yields;
  - A/A median reported both as the statistical median and as analyze.py's upper median;
  - G3 checked in both directions and on every non-wall field; G2 on the 7 protocol fields and on every
    shared non-wall field.
Every key read from a record is indexed with [] (lesson L-99ee952350b2), so a missing key raises.
"""
import glob, gzip, hashlib, json, math, os, random, statistics, sys

import numpy as np

V = os.path.dirname(os.path.abspath(__file__))
S = os.path.dirname(V)
PO = os.path.join(os.path.dirname(S), "genus4_prime_order_20261118")
F7 = ["group_ops", "rel_mul", "rel_add", "la_mul", "la_add", "trials", "relations"]
CURVES = {16411: (373, 455), 32771: (355, 882)}
CCS = ("1.0", "1.4", "2.0", "2.8")
SGES = ((4, 1.5), (8, 2.0))
ARMS = {"x": "2.0", "x*": "1.4"}
WALL_FIELDS = {"wall_ns"}
findings = []


def note(level, msg):
    findings.append({"level": level, "msg": msg})


def read_jsonl_gz(path):
    raw = gzip.open(path, "rb").read()
    plain = path[:-3]
    same = os.path.exists(plain) and hashlib.sha256(open(plain, "rb").read()).hexdigest() == hashlib.sha256(raw).hexdigest()
    return [json.loads(l) for l in raw.decode().splitlines() if l.strip()], same


def sqrt_neg(r):
    return math.sqrt(math.pi * r / 4)


# ---------------------------------------------------------------- load IC
ic = []
inst = {}
files = {}
for p in CURVES:
    for ps in ("pass1", "pass2"):
        for cc in CCS:
            fn = os.path.join(S, f"ic_{ps}_p{p}_cc{cc}.jsonl.gz")
            rs, same = read_jsonl_gz(fn)
            files[os.path.basename(fn)] = {"gz_equals_jsonl": same, "lines": len(rs)}
            if not same:
                note("error", f"{fn}: gz content differs from .jsonl")
            cfg = [r for r in rs if r["kind"] == "config"]
            assert len(cfg) == 1, fn
            c = cfg[0]
            exp = {"arms": "x", "cc": float(cc), "curves": 2, "genera": [4], "max_h": 1, "primes": [p], "seeds": 2,
                   "targets": 1, "target_from": 0, "threads": 1,
                   "sge": [{"fill_max": 1.5, "w_max": 4}, {"fill_max": 2.0, "w_max": 8}]}
            for k, v in exp.items():
                if c[k] != v:
                    note("error", f"{fn}: config {k}={c[k]} expected {v}")
            for r in rs:
                if r["kind"] == "instance":
                    key = (r["p"], r["curve_seed"])
                    if key in inst and (inst[key]["f"], inst[key]["r"]) != (r["f"], r["r"]):
                        note("error", f"instance {key} differs between files")
                    inst[key] = r
            runs = [r for r in rs if r["kind"] == "run"]
            got = sorted((r["curve_seed"], r["seed"], r["w_max"], r["fill_max"]) for r in runs)
            want = sorted((cs, s, w, f) for cs in CURVES[p] for s in (1, 2) for (w, f) in SGES)
            if got != want:
                note("error", f"{fn}: run set {got} != {want}")
            for r in runs:
                if r["p"] != p:
                    note("error", f"{fn}: row p {r['p']}")
                if f"sge({r['w_max']},{r['fill_max']:g})" not in r["arm"]:
                    note("error", f"{fn}: arm {r['arm']} vs w_max/fill_max fields")
                r["_pass"], r["_cc"] = ps, cc
                ic.append(r)
            done = [r for r in rs if r["kind"] == "done"]
            if len(done) != 1:
                note("error", f"{fn}: done rows {len(done)}")

# ---------------------------------------------------------------- load rho
rho = []
rho_checks = []
for p, fn in ((16411, "rho_p16411.jsonl.gz"), (32771, "rho_p32771.jsonl.gz")):
    rs, same = read_jsonl_gz(os.path.join(S, fn))
    files[fn] = {"gz_equals_jsonl": same, "lines": len(rs)}
    cfg = [r for r in rs if r["kind"] == "config"][0]
    for k, v in {"arms": "Q", "primes": [p], "threads": 1, "targets": 1, "target_from": 0, "rho_cap": 2e9}.items():
        if cfg[k] != v:
            note("error", f"{fn}: config {k}={cfg[k]} expected {v}")
    ins = [r for r in rs if r["kind"] == "instance"]
    cell = [r for r in rs if r["kind"] == "cell"][0]
    done = [r for r in rs if r["kind"] == "done"][0]
    runs = [r for r in rs if r["kind"] == "run"]
    for r in runs:
        I = [i for i in ins if i["curve_seed"] == r["curve_seed"]][0]
        n_plain = math.sqrt(math.pi * r["r"] / 2)
        c = {
            "p": p, "seed": r["seed"], "curve_seed": r["curve_seed"],
            "arm_is_plain_nowin": r["arm"] == "R_rho_plain_nowin",
            "certified": r["certified"] is True, "ok": r["ok"] is True,
            "target_is_k0": r["target_seed"] == r["curve_seed"],
            "r_matches_instance": r["r"] == I["r"] == I["order"],
            "steps_eq_native_eq_walk_eq_hash": r["steps"] == r["native"] == r["walk_ops"] == r["hash_uncharged"],
            "coef_adds_eq_2steps": r["coef_adds_uncharged"] == 2 * r["steps"],
            "compares_zero_window_free": r["compares_uncharged"] == 0,
            "group_ops_minus_steps": r["group_ops"] - r["steps"],
            "cantor_mul_per_group_op": r["cantor_mul"] / r["group_ops"],
            "cantor_inv_per_group_op": r["cantor_inv"] / r["group_ops"],
            "steps_over_sqrt_pi_r_over_2": r["steps"] / n_plain,
            "rayleigh_tail_prob": math.exp(-(r["steps"] ** 2) / (2 * r["r"])),
            "wall_ns_per_group_op": r["wall_ns"] / r["group_ops"],
        }
        rho_checks.append(c)
        rho.append(r)
    sum_wall = sum(r["wall_ns"] for r in runs) / 1e9 + cell["search_wall_s"]
    rho_checks.append({"p": p, "done_wall_s": done["wall_s"], "sum_run_wall_plus_search_s": sum_wall,
                       "abs_diff_s": abs(done["wall_s"] - sum_wall)})
    for c in rho_checks[-len(runs) - 1:-1]:
        for k in ("arm_is_plain_nowin", "certified", "ok", "target_is_k0", "r_matches_instance",
                  "steps_eq_native_eq_walk_eq_hash", "coef_adds_eq_2steps", "compares_zero_window_free"):
            if not c[k]:
                note("error", f"rho {p} seed {c['seed']}: {k} false")

# ---------------------------------------------------------------- instances vs prime-order study
po_inst = {}
for fn in glob.glob(os.path.join(PO, "b_p*_x.jsonl.gz")):
    for r in read_jsonl_gz(fn)[0]:
        if r["kind"] == "instance":
            po_inst[(r["p"], r["curve_seed"])] = r
certs = json.load(open(os.path.join(PO, "validation", "out", "certs.json")))["curves"]
inst_checks = {}
for (p, cs), r in sorted(inst.items()):
    o = po_inst[(p, cs)]
    inst_checks[f"{p}:{cs}"] = {"f_eq_po": r["f"] == o["f"], "r_eq_po": r["r"] == o["r"], "order_eq_r": r["order"] == r["r"],
                                "h": r["h"], "g": r["g"], "po_validator_cert_ok": certs[f"{p}:{cs}"]["ok"]}
    if not all(v for k, v in inst_checks[f"{p}:{cs}"].items() if k not in ("h", "g")):
        note("error", f"instance {p}:{cs} check failed")
for r in rho:
    if (r["p"], r["curve_seed"]) not in inst:
        note("error", "rho curve not among IC instances")

# ---------------------------------------------------------------- IC row consistency
ic_row = {"rows": len(ic), "certified_ok": 0, "relations_eq_sum_rels_by_lp": 0, "group_eq_walk_setup_verify": 0,
          "la_mul_eq_sge_plus_wied": 0, "target_is_k0": 0, "r_eq_instance": 0, "not_exhausted": 0, "la_fallbacks_0": 0,
          "la_add_minus_sge_wied_min": None, "la_add_minus_sge_wied_max": None}
extra = []
for r in ic:
    ic_row["certified_ok"] += (r["certified"] is True and r["ok"] is True)
    ic_row["relations_eq_sum_rels_by_lp"] += r["relations"] == sum(r["rels_by_lp"])
    ic_row["group_eq_walk_setup_verify"] += r["group_ops"] == r["walk_ops"] + r["setup_ops"] + r["verify_ops"]
    ic_row["la_mul_eq_sge_plus_wied"] += r["la_mul"] == r["la_sge_mul"] + r["la_wied_mul"]
    ic_row["target_is_k0"] += r["target_seed"] == r["curve_seed"]
    ic_row["r_eq_instance"] += r["r"] == inst[(r["p"], r["curve_seed"])]["r"]
    ic_row["not_exhausted"] += r["exhausted"] is False
    ic_row["la_fallbacks_0"] += r["la_fallbacks"] == 0
    extra.append(r["la_add"] - r["la_sge_add"] - r["la_wied_add"])
ic_row["la_add_minus_sge_wied_min"], ic_row["la_add_minus_sge_wied_max"] = min(extra), max(extra)
# relation phase must not depend on the SGE setting
relphase = ["group_ops", "rel_mul", "rel_add", "trials", "relations", "candidates", "cantor_mul", "cantor_inv"]
by_run = {}
for r in ic:
    by_run.setdefault((r["_pass"], r["p"], r["_cc"], r["curve_seed"], r["seed"]), []).append(r)
ic_row["relation_phase_equal_across_sge"] = sum(all(a[f] == b[f] for f in relphase) for a, b in by_run.values())
ic_row["relation_phase_groups"] = len(by_run)
for k in ("certified_ok", "relations_eq_sum_rels_by_lp", "group_eq_walk_setup_verify", "la_mul_eq_sge_plus_wied",
          "target_is_k0", "r_eq_instance", "not_exhausted", "la_fallbacks_0"):
    if ic_row[k] != len(ic):
        note("error", f"IC consistency {k}: {ic_row[k]}/{len(ic)}")

# ---------------------------------------------------------------- gates
def key(r):
    return (r["p"], r["_cc"], r["w_max"], r["fill_max"], r["curve_seed"], r["target_seed"], r["seed"])


g1 = {"ic_certified_ok": ic_row["certified_ok"], "ic_rows": len(ic),
      "rho_certified": sum(r["certified"] is True for r in rho), "rho_ok": sum(r["ok"] is True for r in rho), "rho_rows": len(rho)}
g1["pass"] = g1["ic_certified_ok"] == g1["ic_rows"] == 128 and g1["rho_certified"] == g1["rho_rows"] == 4

po_rows = {}
for name, cc in (("x", "2.0"), ("xstar", "1.4")):
    for p in CURVES:
        rs = read_jsonl_gz(os.path.join(PO, f"b_p{p}_{name}.jsonl.gz"))[0]
        cfg = [r for r in rs if r["kind"] == "config"][0]
        assert cfg["cc"] == float(cc) and cfg["sge"] == [{"fill_max": 1.5, "w_max": 4}], (name, p)
        for r in rs:
            if r["kind"] == "run":
                k = (p, cc, r["w_max"], r["fill_max"], r["curve_seed"], r["target_seed"], r["seed"])
                assert k not in po_rows
                po_rows[k] = r
g2_rows, g2_bad7, g2_badall, g2_fields = 0, [], [], set()
for r in ic:
    k = key(r)
    if r["w_max"] == 4 and r["_cc"] in ("1.4", "2.0"):
        o = po_rows[k]  # raises if the reference row is missing
        g2_rows += 1
        if any(r[f] != o[f] for f in F7):
            g2_bad7.append(str(k))
        shared = (set(r) & set(o)) - WALL_FIELDS - {"_pass", "_cc"}
        g2_fields |= shared
        if any(r[f] != o[f] for f in shared):
            g2_badall.append([str(k), sorted(f for f in shared if r[f] != o[f])])
g2 = {"rows": g2_rows, "distinct_keys": len({key(r) for r in ic if r["w_max"] == 4 and r["_cc"] in ("1.4", "2.0")}),
      "mismatch_7_fields": g2_bad7, "mismatch_all_shared_nonwall_fields": g2_badall,
      "shared_fields_compared": len(g2_fields), "pass": g2_rows == 32 and not g2_bad7}

P1 = {key(r): r for r in ic if r["_pass"] == "pass1"}
P2 = {key(r): r for r in ic if r["_pass"] == "pass2"}
nonwall = sorted((set(ic[0]) - WALL_FIELDS - {"_pass"}))
g3_bad = [str(k) for k in set(P1) | set(P2) if k not in P1 or k not in P2 or any(P1[k][f] != P2[k][f] for f in nonwall)]
g3 = {"pass1": len(P1), "pass2": len(P2), "fields_compared": len(nonwall), "mismatches": g3_bad, "pass": not g3_bad and len(P1) == len(P2) == 64}

# ---------------------------------------------------------------- t_G
tG = {}
tG_out = {}
for p in CURVES:
    R = [r for r in rho if r["p"] == p]
    tG[p] = sum(r["wall_ns"] for r in R) / sum(r["group_ops"] for r in R)
    per = [r["wall_ns"] / r["group_ops"] for r in R]
    tG_out[p] = {"runs": len(R), "t_G_ns_pooled": tG[p], "per_run_ns": per,
                 "mean_of_runs_ns": statistics.mean(per), "t_G_ns_per_step": sum(r["wall_ns"] for r in R) / sum(r["steps"] for r in R),
                 "spread_pct": (max(per) - min(per)) / tG[p] * 100}

# ---------------------------------------------------------------- R2: OLS and bootstrap
def design(rs, p, free_G=False, intercept=True):
    cols = []
    if intercept:
        cols.append([1.0] * len(rs))
    if free_G:
        cols.append([float(r["group_ops"]) for r in rs])
    cols.append([float(r["rel_mul"] + r["rel_add"]) for r in rs])
    cols.append([float(r["la_mul"] + r["la_add"]) for r in rs])
    X = np.array(cols).T
    y = np.array([float(r["wall_ns"]) - (0.0 if free_G else tG[p] * r["group_ops"]) for r in rs])
    sc = np.abs(X).max(axis=0)
    b, *_ = np.linalg.lstsq(X / sc, y, rcond=None)
    return b / sc, X, y


def fit(rs, p, **kw):
    b, X, y = design(rs, p, **kw)
    names = (["const"] if kw.get("intercept", True) else []) + (["t_G_ic"] if kw.get("free_G") else []) + ["t_rel", "t_la"]
    return dict(zip(names, b.tolist())), X, y, b


rng = np.random.default_rng(4242)
NB_IND = 4000


def cluster_boot(rows, clkey, stat, nb, gen):
    cl = sorted({clkey(r) for r in rows})
    by = {c: [r for r in rows if clkey(r) == c] for c in cl}
    out = []
    for _ in range(nb):
        pick = gen.integers(0, len(cl), len(cl))
        rs = [x for i in pick for x in by[cl[i]]]
        out.append(stat(rs))
    return out


def replica_boot(rows, cl, by, stat, seed, nb=2000):
    """Replica of the frozen resampling stream (random.Random(seed).choice over the sorted cluster list)."""
    g = random.Random(seed)
    out = []
    for _ in range(nb):
        rs = []
        for c in [g.choice(cl) for _ in cl]:
            rs += by[c]
        out.append(stat(rs))
    s = sorted(out)
    return s[int(0.025 * nb)], s[int(0.975 * nb) - 1]


def pq(v):
    return [float(np.quantile(v, 0.025)), float(np.quantile(v, 0.975))]


R2 = {}
for p in CURVES:
    I = [r for r in ic if r["p"] == p]
    est, X, y, b = fit(I, p)
    resid = y - X @ b
    w_rel, w_la = est["t_rel"] / tG[p], est["t_la"] / tG[p]
    ind = cluster_boot(I, lambda r: (r["curve_seed"], r["seed"], r["_cc"], r["w_max"]),
                       lambda rs: fit(rs, p)[0], NB_IND, rng)
    # replica: analyze.py cluster tuples (curve_seed, seed, cc string, "w,fill" string) sorted
    def ak(r):
        return (r["curve_seed"], r["seed"], r["_cc"], f"{r['w_max']},{r['fill_max']:g}")
    acl = sorted({ak(r) for r in I})
    aby = {c: [r for r in I if ak(r) == c] for c in acl}
    rep_rel = replica_boot(I, acl, aby, lambda rs: fit(rs, p)[0]["t_rel"] / tG[p], 20261125)
    rep_la = replica_boot(I, acl, aby, lambda rs: fit(rs, p)[0]["t_la"] / tG[p], 20261125)
    corr = float(np.corrcoef(X[:, 1], X[:, 2])[0, 1])
    # model-free bounds: all non-group time charged to one class
    nong = [(r["wall_ns"] - tG[p] * r["group_ops"]) for r in I]
    ub_la = max(n / (r["la_mul"] + r["la_add"]) for n, r in zip(nong, I)) / tG[p]
    ub_rel = max(n / (r["rel_mul"] + r["rel_add"]) for n, r in zip(nong, I)) / tG[p]
    alt = {}
    e0 = fit(I, p, intercept=False)[0]
    alt["no_intercept"] = {"w_rel": e0["t_rel"] / tG[p], "w_la": e0["t_la"] / tG[p]}
    eg = fit(I, p, free_G=True)[0]
    alt["free_t_G_from_ic"] = {"t_G_ic_ns": eg["t_G_ic"], "const_s": eg["const"] / 1e9,
                               "w_rel_vs_rho_tG": eg["t_rel"] / tG[p], "w_la_vs_rho_tG": eg["t_la"] / tG[p]}
    for ps in ("pass1", "pass2"):
        e = fit([r for r in I if r["_pass"] == ps], p)[0]
        alt[ps] = {"w_rel": e["t_rel"] / tG[p], "w_la": e["t_la"] / tG[p], "const_s": e["const"] / 1e9}
    for w in (4, 8):
        e = fit([r for r in I if r["w_max"] == w], p)[0]
        alt[f"sge_w{w}_only"] = {"w_rel": e["t_rel"] / tG[p], "w_la": e["t_la"] / tG[p], "const_s": e["const"] / 1e9}
    for drop in CCS:
        e = fit([r for r in I if r["_cc"] != drop], p)[0]
        alt[f"drop_cc{drop}"] = {"w_rel": e["t_rel"] / tG[p], "w_la": e["t_la"] / tG[p], "const_s": e["const"] / 1e9}
    res_cc = {cc: float(np.mean([resid[i] for i, r in enumerate(I) if r["_cc"] == cc]) / 1e9) for cc in CCS}
    mean_wall = float(np.mean([r["wall_ns"] for r in I])) / 1e9
    R2[p] = {"t_rel_ns": est["t_rel"], "t_la_ns": est["t_la"], "const_s": est["const"] / 1e9,
             "const_over_mean_wall": est["const"] / 1e9 / mean_wall,
             "w_rel": w_rel, "w_la": w_la,
             "w_rel_ci_independent": [x / tG[p] for x in pq([d["t_rel"] for d in ind])],
             "w_la_ci_independent": [x / tG[p] for x in pq([d["t_la"] for d in ind])],
             "w_rel_ci_replica_of_frozen_stream": list(rep_rel), "w_la_ci_replica_of_frozen_stream": list(rep_la),
             "corr_rel_ops_la_ops": corr, "rows": len(I), "clusters": len(acl),
             "r2": float(1 - np.sum(resid ** 2) / np.sum((y - y.mean()) ** 2)),
             "resid_rms_s": float(np.sqrt(np.mean(resid ** 2))) / 1e9,
             "mean_resid_by_cc_s": res_cc,
             "model_free_upper_bound_w_la": ub_la, "model_free_upper_bound_w_rel": ub_rel,
             "alternatives": alt}

# ---------------------------------------------------------------- R1
R1 = {}
for p in CURVES:
    neg = statistics.mean(sqrt_neg(inst[(p, c)]["r"]) for c in CURVES[p])
    R1[p] = {"mean_sqrt_pi_r_over_4": neg, "ideal_negation_rho_s": tG[p] * neg / 1e9, "arms": {}}
    for arm, cc in ARMS.items():
        A = [r for r in ic if r["p"] == p and r["_cc"] == cc and r["w_max"] == 4]
        st = lambda rs: statistics.mean(r["wall_ns"] for r in rs) / (tG[p] * neg)
        ind = cluster_boot(A, lambda r: (r["curve_seed"], r["seed"]), st, NB_IND, rng)
        cl = sorted({(r["curve_seed"], r["seed"]) for r in A})
        by = {c: [r for r in A if (r["curve_seed"], r["seed"]) == c] for c in cl}
        rep = replica_boot(A, cl, by, st, 20261125 + 1)
        per_curve = {c: statistics.mean(r["wall_ns"] for r in A if r["curve_seed"] == c) / (tG[p] * sqrt_neg(inst[(p, c)]["r"]))
                     for c in CURVES[p]}
        # post hoc GAE cross-check at the measured weights
        gae = [r["group_ops"] + R2[p]["w_rel"] * (r["rel_mul"] + r["rel_add"]) + R2[p]["w_la"] * (r["la_mul"] + r["la_add"]) for r in A]
        cross = statistics.mean(gae) / neg
        fitted = [tG[p] * r["group_ops"] + R2[p]["t_rel_ns"] * (r["rel_mul"] + r["rel_add"]) + R2[p]["t_la_ns"] * (r["la_mul"] + r["la_add"]) for r in A]
        w1 = statistics.mean(r["group_ops"] + r["rel_mul"] + r["rel_add"] + r["la_mul"] + r["la_add"] for r in A) / neg
        R1[p]["arms"][arm] = {
            "runs": len(A), "mean_wall_s": st(A) * tG[p] * neg / 1e9, "R1": st(A),
            "ci_independent": pq(ind), "ci_replica_of_frozen_stream": list(rep), "clusters": len(cl),
            "per_curve_R1": per_curve,
            "crosscheck_GAE_at_measured_w": cross, "crosscheck_over_R1_minus_1": cross / st(A) - 1,
            "fitted_wall_without_const_s": statistics.mean(fitted) / 1e9,
            "w1_ratio_these_rows": w1,
            "share_s": {"group": statistics.mean(tG[p] * r["group_ops"] for r in A) / 1e9,
                        "rel": statistics.mean(R2[p]["t_rel_ns"] * (r["rel_mul"] + r["rel_add"]) for r in A) / 1e9,
                        "la": statistics.mean(R2[p]["t_la_ns"] * (r["la_mul"] + r["la_add"]) for r in A) / 1e9},
            "la_ops_over_group_ops": statistics.mean(r["la_mul"] + r["la_add"] for r in A) / statistics.mean(r["group_ops"] for r in A),
            "paired_by_pass": {ps: statistics.mean(r["wall_ns"] for r in A if r["_pass"] == ps) / (tG[p] * neg) for ps in ("pass1", "pass2")},
        }
    X = [r for r in ic if r["p"] == p and r["_cc"] == "2.0" and r["w_max"] == 4]
    Xs = {(r["_pass"], r["curve_seed"], r["seed"]): r for r in ic if r["p"] == p and r["_cc"] == "1.4" and r["w_max"] == 4}
    pr = [Xs[(r["_pass"], r["curve_seed"], r["seed"])]["wall_ns"] / r["wall_ns"] for r in X]
    R1[p]["xstar_over_x_wall_paired"] = {"mean": statistics.mean(pr), "min": min(pr), "max": max(pr), "n": len(pr)}
    Rr = [r for r in rho if r["p"] == p]
    R1[p]["measured_plain_rho_mean_wall_s"] = statistics.mean(r["wall_ns"] for r in Rr) / 1e9
    R1[p]["x_wall_over_measured_plain_rho"] = R1[p]["arms"]["x"]["mean_wall_s"] / R1[p]["measured_plain_rho_mean_wall_s"]

# ---------------------------------------------------------------- A/A
lr = []
for k in P1:
    lr.append(math.log(P2[k]["wall_ns"] / P1[k]["wall_ns"]))
ab = sorted(abs(x) for x in lr)
AA = {"pairs": len(lr), "median_abs_ln": statistics.median(ab), "upper_median_abs_ln_analyze_rule": ab[len(ab) // 2],
      "max_abs_ln": max(ab), "mean_signed_ln_pass2_over_pass1": statistics.mean(lr),
      "by_prime_mean_signed": {p: statistics.mean(math.log(P2[k]["wall_ns"] / P1[k]["wall_ns"]) for k in P1 if k[0] == p) for p in CURVES}}

# ---------------------------------------------------------------- per-cc wall table (post hoc)
cc_tab = {p: {cc: {w: statistics.mean(r["wall_ns"] for r in ic if r["p"] == p and r["_cc"] == cc and r["w_max"] == w) / 1e9
                   for w in (4, 8)} for cc in CCS} for p in CURVES}

# ---------------------------------------------------------------- decision and predictions
ups = [R2[p][k][1] for p in CURVES for k in ("w_rel_ci_independent", "w_la_ci_independent")]
dec_ind = "conservative" if max(ups) < 1 else ("fragile" if R2[16411]["w_la_ci_independent"][0] > 1.25 else "unresolved")
ups_rep = [R2[p][k][1] for p in CURVES for k in ("w_rel_ci_replica_of_frozen_stream", "w_la_ci_replica_of_frozen_stream")]
dec_rep = "conservative" if max(ups_rep) < 1 else ("fragile" if R2[16411]["w_la_ci_replica_of_frozen_stream"][0] > 1.25 else "unresolved")
ws = [R2[p][k] for p in CURVES for k in ("w_rel", "w_la")]
pred = {
    "P1": {"value_us": tG[16411] / 1000, "holds": 1.5 <= tG[16411] / 1000 <= 5},
    "P2": {"min": min(ws), "max": max(ws), "holds": all(0.002 <= w <= 0.05 for w in ws)},
    "P3": {"values": [R1[p]["arms"]["x*"]["R1"] for p in CURVES], "holds": all(0.003 <= R1[p]["arms"]["x*"]["R1"] <= 0.03 for p in CURVES)},
    "P4": {"value": AA["median_abs_ln"], "value_upper_median": AA["upper_median_abs_ln_analyze_rule"], "holds": AA["upper_median_abs_ln_analyze_rule"] < 0.05 and AA["median_abs_ln"] < 0.05},
    "P5": {"value": tG[32771] / tG[16411], "holds": 0.8 <= tG[32771] / tG[16411] <= 1.3},
}

# ---------------------------------------------------------------- prose arithmetic
p16 = R1[16411]["arms"]["x"]
sens_cost = p16["share_s"]["group"] * 0.1 + p16["share_s"]["rel"] + p16["share_s"]["la"]
sens_cost_with_const = p16["mean_wall_s"] - 0.9 * p16["share_s"]["group"]
prose = {
    "overcharge_factor_range": [1 / max(ws), 1 / min(ws)],
    "w0.1_overstatement": {f"{p}_{k}": 0.1 / R2[p][k] for p in CURVES for k in ("w_rel", "w_la")},
    "la_cheaper_than_group_x16411": 1 / R2[16411]["w_la"],
    "la_ops_per_group_op_x16411": p16["la_ops_over_group_ops"],
    "sensitivity_tG_div10": {"w_la_16411": R2[16411]["t_la_ns"] / (tG[16411] / 10), "w_la_32771": R2[32771]["t_la_ns"] / (tG[32771] / 10),
                             "cost_s_no_const": sens_cost, "bound_s": R1[16411]["ideal_negation_rho_s"] / 10,
                             "ratio_no_const": sens_cost / (R1[16411]["ideal_negation_rho_s"] / 10),
                             "cost_s_from_measured_wall": sens_cost_with_const,
                             "ratio_from_measured_wall": sens_cost_with_const / (R1[16411]["ideal_negation_rho_s"] / 10)},
    "xstar_slower_than_x_16411_pct": (R1[16411]["arms"]["x*"]["mean_wall_s"] / p16["mean_wall_s"] - 1) * 100,
    "xstar_vs_x_32771_pct": (R1[32771]["arms"]["x*"]["mean_wall_s"] / R1[32771]["arms"]["x"]["mean_wall_s"] - 1) * 100,
}

# ---------------------------------------------------------------- compare with analysis.json and RESULTS.md
aj = json.load(open(os.path.join(S, "analysis.json")))
cmp = []


def c(name, mine, theirs, tol=0.01):
    rel = abs(mine - theirs) / abs(theirs) if theirs else abs(mine)
    cmp.append({"quantity": name, "validator": mine, "reported": theirs, "rel_diff": rel, "over_1pct": rel > tol})


for p in CURVES:
    a = aj["primes"][str(p)]
    c(f"t_G {p}", tG[p], a["t_G_ns"])
    for i, v in enumerate(a["t_G_per_run_ns"]):
        c(f"t_G run {i} {p}", tG_out[p]["per_run_ns"][i], v)
    c(f"t_rel {p}", R2[p]["t_rel_ns"], a["t_rel_ns"])
    c(f"t_la {p}", R2[p]["t_la_ns"], a["t_la_ns"])
    c(f"const {p}", R2[p]["const_s"] * 1e9, a["t_const_ns"])
    c(f"w_rel {p}", R2[p]["w_rel"], a["w_rel"]["value"])
    c(f"w_la {p}", R2[p]["w_la"], a["w_la"]["value"])
    for j in (0, 1):
        c(f"w_rel ci[{j}] {p} replica", R2[p]["w_rel_ci_replica_of_frozen_stream"][j], a["w_rel"]["ci"][j])
        c(f"w_la ci[{j}] {p} replica", R2[p]["w_la_ci_replica_of_frozen_stream"][j], a["w_la"]["ci"][j])
        c(f"w_rel ci[{j}] {p} independent", R2[p]["w_rel_ci_independent"][j], a["w_rel"]["ci"][j], tol=0.05)
        c(f"w_la ci[{j}] {p} independent", R2[p]["w_la_ci_independent"][j], a["w_la"]["ci"][j], tol=0.05)
    c(f"ideal neg rho s {p}", R1[p]["ideal_negation_rho_s"], a["ideal_negation_rho_s"])
    for arm in ARMS:
        aa = a["arms"][arm]
        c(f"R1 {arm} {p}", R1[p]["arms"][arm]["R1"], aa["ratio_wall_vs_ideal_negation"])
        c(f"mean wall {arm} {p}", R1[p]["arms"][arm]["mean_wall_s"], aa["mean_wall_s"])
        for j in (0, 1):
            c(f"R1 ci[{j}] {arm} {p} replica", R1[p]["arms"][arm]["ci_replica_of_frozen_stream"][j], aa["ci"][j])
            c(f"R1 ci[{j}] {arm} {p} independent", R1[p]["arms"][arm]["ci_independent"][j], aa["ci"][j], tol=0.05)
c("A/A median (analyze rule)", AA["upper_median_abs_ln_analyze_rule"], aj["aa"]["median_abs_log_ratio"])
c("A/A median (true median)", AA["median_abs_ln"], aj["aa"]["median_abs_log_ratio"])
c("A/A max", AA["max_abs_ln"], aj["aa"]["max"])
c("tG ratio", tG[32771] / tG[16411], aj["tG_ratio_32771_over_16411"])

# numbers printed in RESULTS.md prose (rounded as printed there)
results_md = [
    ("headline w_rel 0.0026 (16411)", R2[16411]["w_rel"], 0.0026), ("w_rel 0.0025 (32771)", R2[32771]["w_rel"], 0.0025),
    ("w_la 0.0042", R2[16411]["w_la"], 0.0042), ("w_la 0.0045", R2[32771]["w_la"], 0.0045),
    ("t_G 2.00 us", tG[16411] / 1000, 2.00), ("t_G 2.03 us", tG[32771] / 1000, 2.03),
    ("overcharge ~220", prose["overcharge_factor_range"][0], 220), ("overcharge ~400", prose["overcharge_factor_range"][1], 400),
    ("ideal bound 477 s", R1[16411]["ideal_negation_rho_s"], 477), ("ideal bound 1926 s", R1[32771]["ideal_negation_rho_s"], 1926),
    ("arm x 2.5 s", p16["mean_wall_s"], 2.5), ("arm x 7.4 s (32771)", R1[32771]["arms"]["x"]["mean_wall_s"], 7.4),
    ("cross-check x 0.0056", R1[16411]["arms"]["x"]["crosscheck_GAE_at_measured_w"], 0.0056),
    ("cross-check x* 0.0060", R1[16411]["arms"]["x*"]["crosscheck_GAE_at_measured_w"], 0.0060),
    ("cross-check x 0.0043 (32771)", R1[32771]["arms"]["x"]["crosscheck_GAE_at_measured_w"], 0.0043),
    ("cross-check x* 0.0043 (32771)", R1[32771]["arms"]["x*"]["crosscheck_GAE_at_measured_w"], 0.0043),
    ("group ops 0.85 s", p16["share_s"]["group"], 0.85), ("rel 0.28 s", p16["share_s"]["rel"], 0.28),
    ("la 1.5 s", p16["share_s"]["la"], 1.5), ("mod-r 240x cheaper", prose["la_cheaper_than_group_x16411"], 240),
    ("400x as many la ops", prose["la_ops_per_group_op_x16411"], 400),
    ("w=0.1 overstates ~25x (la 16411)", prose["w0.1_overstatement"]["16411_w_la"], 25),
    ("x* 6% slower", prose["xstar_slower_than_x_16411_pct"], 6),
    ("sensitivity w_la ~0.04", prose["sensitivity_tG_div10"]["w_la_16411"], 0.04),
    ("sensitivity cost ~1.9 s", prose["sensitivity_tG_div10"]["cost_s_no_const"], 1.9),
    ("sensitivity bound 47.7 s", prose["sensitivity_tG_div10"]["bound_s"], 47.7),
    ("sensitivity ratio ~0.04", prose["sensitivity_tG_div10"]["ratio_no_const"], 0.04),
    ("intercept -0.15 s (16411)", R2[16411]["const_s"], -0.15),
    ("A/A 0.023", AA["median_abs_ln"], 0.023), ("A/A max 0.105", AA["max_abs_ln"], 0.105), ("P5 1.016", tG[32771] / tG[16411], 1.016),
]
for cc, v in zip(CCS, (3.69, 2.68, 2.52, 2.73)):
    results_md.append((f"cc table 16411 cc{cc}", cc_tab[16411][cc][4], v))
for cc, v in zip(CCS, (9.79, 7.43, 7.44, 8.87)):
    results_md.append((f"cc table 32771 cc{cc}", cc_tab[32771][cc][4], v))
md = [{"quantity": n, "validator": m, "printed": t, "rel_diff": abs(m - t) / abs(t)} for n, m, t in results_md]

out = {
    "files": files, "instances": inst_checks, "ic_row_consistency": ic_row, "rho_checks": rho_checks,
    "gates": {"G1": g1, "G2": g2, "G3": g3}, "t_G": {str(p): v for p, v in tG_out.items()},
    "R2": {str(p): v for p, v in R2.items()}, "R1": {str(p): v for p, v in R1.items()}, "AA": AA,
    "per_cc_mean_wall_s": {str(p): v for p, v in cc_tab.items()},
    "decision": {"independent_ci": dec_ind, "replica_ci": dec_rep, "reported": aj["decision"],
                 "max_upper_bound_independent": max(ups), "max_upper_bound_replica": max(ups_rep)},
    "predictions": pred, "prose": prose, "vs_analysis_json": cmp, "vs_results_md_printed": md,
    "notes": findings,
}
json.dump(out, open(os.path.join(V, "recompute.json"), "w"), indent=1, default=str)

print("files ok:", all(f["gz_equals_jsonl"] for f in files.values()), len(files))
print("gates:", {g: out["gates"][g]["pass"] for g in out["gates"]}, "G2 rows", g2_rows, "G2 all-field mismatches", len(g2_badall),
      "G3 fields", len(nonwall))
print("IC consistency:", ic_row)
for p in CURVES:
    print(p, "tG", round(tG[p], 2), tG_out[p]["per_run_ns"], "R2", {k: R2[p][k] for k in ("t_rel_ns", "t_la_ns", "const_s", "w_rel", "w_la",
                                                                                            "w_rel_ci_independent", "w_la_ci_independent")})
    for arm in ARMS:
        a = R1[p]["arms"][arm]
        print("  ", arm, "R1", a["R1"], a["ci_independent"], a["ci_replica_of_frozen_stream"], "cross", a["crosscheck_GAE_at_measured_w"],
              "cross/R1-1", round(a["crosscheck_over_R1_minus_1"], 4), "w1", a["w1_ratio_these_rows"])
print("AA", AA)
print("decision", out["decision"])
print("pred", {k: v["holds"] for k, v in pred.items()})
print("max rel diff vs analysis.json:", max(x["rel_diff"] for x in cmp), [x["quantity"] for x in cmp if x["over_1pct"]])
print("printed-number rel diffs > 3%:", [(x["quantity"], round(x["validator"], 5), x["printed"], round(x["rel_diff"], 3)) for x in md if x["rel_diff"] > 0.03])
print("notes:", findings)
