#!/usr/bin/env python3
"""Validator's independent recompute for research/koblitz_ksum_fold_study_20261115.

Written by the validator from the AGENTS.md section 5a rule, the band.rs module
comment (priced_gae(w) = G - C + w (U + n_C)) and the compare.rs module comment
(ratio of totals over (workload, round) pairs, linearised t interval with
workloads as clusters).  It does not import or read the producer's analyze.py,
make_report.py, results.json or summary.json.  Python stdlib only.

Usage: recompute.py OUT.json LABEL=SESSION_DIR [LABEL=SESSION_DIR ...]
Each SESSION_DIR holds an uncompressed records.jsonl (gunzipped copy).
"""
import json, math, random, sys, collections, pathlib

ROOT = pathlib.Path("/home/user/crypto")
CALIB = json.loads((ROOT / "docs/ic/calibration.json").read_text())["instances"]
WEIGHTS = [0.0, 0.1, 1.0]
NON_OPERATION = {"residual_table_bytes_uncharged", "factor_base_vec_bytes_floor_uncharged",
                 "native_table_bytes_uncharged"}
TABLE_WORK = ["table_insert_attempts", "sibling_second_points", "sibling_lift_products"]
# counter -> calibration unit (methods.rs PRICED_NATIVE)
PRICED_NATIVE = [("sqrt_solves", "ns_per_sqrt"), ("as_solves", "ns_per_as_solve"),
                 ("s4_pairs", "ns_per_s4_pair"), ("lookups", "ns_per_lookup"),
                 ("row_ops", "ns_per_row_op"), ("legendre_symbols", "ns_per_legendre"),
                 ("inversions", "ns_per_inversion"), ("frobenius_maps", "ns_per_frobenius"),
                 ("canonicalisations", "ns_per_canon"), ("target_guard_probes", "ns_per_lookup")]


def calib_row(rec):
    c = rec["workload"]["curve"]; spec = rec["workload"]["curve_spec"]
    if c["family"] != "koblitz":
        return None
    return CALIB.get(f"koblitz/K_{spec['a']} / GF(2^{spec['n']})")


def priced(rec):
    """(S at each w, U, n_C, C, missing) for one record."""
    G = rec["cost"]["total_gae"]
    if G is None:
        return None
    phases = rec.get("phases") or []
    counters = rec.get("counters") or {}
    U, missing = 0, False
    for name in rec["cost"].get("unpriced") or []:
        if name in NON_OPERATION:
            continue
        base = name[:-len("_uncharged")] if name.endswith("_uncharged") else name
        if name in counters:
            U += counters[name]; continue
        found, tot = False, 0
        for p in phases:
            nat = p.get("native") or {}
            if name in nat:
                tot += nat[name]; found = True
            elif base in nat:
                tot += nat[base]; found = True
        if not found and base.startswith("solver_") and rec.get("solver"):
            tot = rec["solver"]["ops"]; found = True
        if not found:
            missing = True
        U += tot
    for name in TABLE_WORK:
        if any(f"counted:{name}" in (p.get("work_native") or {}) for p in phases):
            U += sum((p.get("work_native") or {}).get(name, 0) for p in phases)
    pinned = set((rec.get("detail") or {}).get("calibration_pinned_units") or [])
    C, nC = 0.0, 0
    if pinned:
        row = calib_row(rec)
        if row is None:
            raise SystemExit(f"{rec['record_id']}: pinned units but no calibration row")
        for p in phases:
            nat = p.get("native") or {}
            for cnt, unit in PRICED_NATIVE:
                v = nat.get(cnt, 0)
                if v > 0 and unit in pinned:
                    C += v * row[unit]; nC += v
    sq = math.sqrt(rec["workload"]["curve"]["r"])
    S = []
    for w in WEIGHTS:
        if missing and w > 0:
            S.append(None)
        else:
            S.append((G - C + w * (U + nC)) / sq)
    return S, U, nC, C, missing


def load(label, d):
    out = []
    with open(pathlib.Path(d) / "records.jsonl") as f:
        for line in f:
            r = json.loads(line)
            pr = priced(r) if r["outcome"]["status"] == "verified" else None
            out.append({
                "session": label, "session_id": r["session_id"], "arm": r["arm"],
                "workload": r["workload"]["workload_id"], "round": r["round"],
                "warmup": r["warmup"], "status": r["outcome"]["status"],
                "matches_target": r["outcome"].get("matches_target"),
                "seed": r["algorithm_seed"], "slug": r["workload"]["curve"]["slug"],
                "S": pr[0] if pr else None, "U": pr[1] if pr else None,
                "lower_bound": r["cost"].get("lower_bound"),
                "frobfold_mismatches": sum((p.get("native") or {}).get("frobfold_mismatches", 0)
                                           for p in r.get("phases") or []),
                "relations": (r.get("counters") or {}).get("relations_found"),
                "trials": (r.get("counters") or {}).get("targets_tried"),
                "table_entries": sum((p.get("native") or {}).get(k, 0) for p in r.get("phases") or []
                                     for k in ("ksum_table_entries", "pair_table_entries")),
                "recovered": r["outcome"].get("recovered"),
                "error": r["outcome"].get("error"),
            })
    return out


def t975(df):
    # Student-t 0.975 quantile by bisection on the CDF (regularised incomplete beta).
    def betacf(a, b, x):
        qab, qap, qam = a + b, a + 1, a - 1
        c, d = 1.0, 1 - qab * x / qap
        d = 1 / d if abs(d) > 1e-300 else 1e300
        h = d
        for m in range(1, 300):
            m2 = 2 * m
            aa = m * (b - m) * x / ((qam + m2) * (a + m2))
            d = 1 + aa * d; d = 1 / d if abs(d) > 1e-300 else 1e300
            c = 1 + aa / c if abs(c) > 1e-300 else 1e300
            h *= d * c
            aa = -(a + m) * (qab + m) * x / ((a + m2) * (qap + m2))
            d = 1 + aa * d; d = 1 / d if abs(d) > 1e-300 else 1e300
            c = 1 + aa / c if abs(c) > 1e-300 else 1e300
            de = d * c; h *= de
            if abs(de - 1) < 1e-15:
                break
        return h
    def ibeta(a, b, x):
        if x <= 0: return 0.0
        if x >= 1: return 1.0
        lb = math.lgamma(a + b) - math.lgamma(a) - math.lgamma(b) + a * math.log(x) + b * math.log(1 - x)
        if x < (a + 1) / (a + b + 2):
            return math.exp(lb) * betacf(a, b, x) / a
        return 1 - math.exp(lb) * betacf(b, a, 1 - x) / b
    def cdf(t):
        x = df / (df + t * t)
        p = 0.5 * ibeta(df / 2, 0.5, x)
        return 1 - p if t > 0 else p
    lo, hi = 0.0, 100.0
    for _ in range(200):
        mid = (lo + hi) / 2
        if cdf(mid) < 0.975: lo = mid
        else: hi = mid
    return (lo + hi) / 2


def ratio(recs, a_arm, b_arm, sessions=None, wi=0, boot=2000, seed=0):
    """B/A = sum S_B / sum S_A over (session, workload, round) pairs, measured rounds."""
    idx = {}
    for r in recs:
        if r["warmup"] or (sessions and r["session"] not in sessions):
            continue
        idx[(r["arm"], r["session"], r["workload"], r["round"])] = r
    clusters = collections.OrderedDict()
    same_seeds, npairs, unpaired = True, 0, 0
    for (arm, s, wl, rd), ra in sorted(idx.items()):
        if arm != a_arm:
            continue
        rb = idx.get((b_arm, s, wl, rd))
        if rb is None or ra["S"] is None or rb["S"] is None or ra["S"][wi] is None or rb["S"][wi] is None:
            unpaired += 1; continue
        same_seeds &= ra["seed"] == rb["seed"]
        c = clusters.setdefault((s, wl), [0.0, 0.0])
        c[0] += ra["S"][wi]; c[1] += rb["S"][wi]; npairs += 1
    cl = list(clusters.values())
    K = len(cl)
    if K < 2:
        return None
    sa = sum(c[0] for c in cl); sb = sum(c[1] for c in cl)
    R = sb / sa
    u = [(c[1] - R * c[0]) / sa for c in cl]
    ub = sum(u) / K
    se = math.sqrt(K / (K - 1) * sum((x - ub) ** 2 for x in u))
    q = t975(K - 1)
    half = q * se / R
    lin = (R * math.exp(-half), R * math.exp(half))
    rng = random.Random(seed)
    bs = []
    for _ in range(boot):
        xa = xb = 0.0
        for _ in range(K):
            c = cl[rng.randrange(K)]
            xa += c[0]; xb += c[1]
        bs.append(xb / xa)
    bs.sort()
    pb = (bs[int(0.025 * boot)], bs[int(0.975 * boot) - 1])
    return {"ratio": R, "ci95_linearised_t": lin, "ci95_cluster_bootstrap": pb, "clusters": K,
            "pairs": npairs, "unpaired": unpaired, "same_seeds": same_seeds}


def band(recs, arms=None):
    by = collections.defaultdict(list)
    for r in recs:
        if r["warmup"]:
            continue
        by[r["arm"]].append(r)
    out = {}
    for arm, rs in sorted(by.items()):
        if arms and arm not in arms:
            continue
        ver = [r for r in rs if r["status"] == "verified"]
        means = []
        for wi in range(3):
            vals = [r["S"][wi] for r in ver if r["S"] is not None]
            means.append(sum(vals) / len(vals) if vals and len(vals) == len(ver) and all(v is not None for v in vals) else None)
        out[arm] = {"runs": len(rs), "verified": len(ver),
                    "status_counts": dict(collections.Counter(r["status"] for r in rs)),
                    "mean_S": means,
                    "lower_bound_any": any(r["lower_bound"] for r in ver),
                    "mean_trials": (sum(r["trials"] or 0 for r in ver) / len(ver)) if ver else None,
                    "mean_relations": (sum(r["relations"] or 0 for r in ver) / len(ver)) if ver else None,
                    "frobfold_mismatches": sum(r["frobfold_mismatches"] for r in ver),
                    "matches_target_all": all(r["matches_target"] for r in ver) if ver else None}
    return out


if __name__ == "__main__":
    outp = sys.argv[1]
    recs = []
    for a in sys.argv[2:]:
        label, d = a.split("=", 1)
        recs += load(label, d)
    json.dump({"records": len(recs)}, sys.stdout); print()
    pathlib.Path(outp).write_text(json.dumps(recs))
