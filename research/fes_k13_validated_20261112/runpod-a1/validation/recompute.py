#!/usr/bin/env python3
"""Validator recompute for FES-K13-VALIDATED-20261112, Runpod attempt a1 (session ECBS1h695ef8e76c34).

Reads only the committed session files (records.jsonl, plan.json, session.json, spec.json) and the
frozen protocol.json.  No repository code is imported and no ecbench output is read here; the
producer's claimed figures are compared afterwards by build_reports.py.

The band follows the definition of src/cryptanalysis/ecbench/band.rs at the measured revision
e9d5dbdccd (transcribed, not imported):

  priced_gae(w) = G - C + w * (U + n_C),   priced_S(w) = priced_gae(w) / sqrt(r)

  G    cost.total_gae
  C    GAE the pinned calibration put on native work: for every phase, each PRICED_NATIVE /
       PINNABLE_NATIVE counter whose unit is in detail.calibration_pinned_units, count * ratio;
       plus solver.ops * ratio(ns_per_word_xor) when the solver op_unit contains "word XOR",
       ns_per_word_xor is pinned and no solver_* counter is listed unpriced.
  n_C  the operation count behind C.
  U    the counts behind cost.unpriced (top-level counters, else phase native X_uncharged or X,
       else solver.ops for solver_*), minus NON_OPERATION_COUNTERS, plus the phase sum of
       work_native[name] for name in TABLE_WORK_COUNTERS + SIBLING_WORK_COUNTERS when some phase
       carries work_native["counted:<name>"] (the 2026-11-08 default reading).

Means are over measured (non-warm-up) verified runs, as band_with does.

Usage: python3 -I recompute.py <session_dir> <protocol.json> <out.json>
"""
import hashlib
import json
import math
import sys

# --- transcribed constants (e9d5dbdccd) --------------------------------------------------------
# src/cryptanalysis/ecbench/methods.rs PRICED_NATIVE
PRICED_NATIVE = [
    ("sqrt_solves", "ns_per_sqrt"), ("as_solves", "ns_per_as_solve"), ("s4_pairs", "ns_per_s4_pair"),
    ("lookups", "ns_per_lookup"), ("row_ops", "ns_per_row_op"), ("legendre_symbols", "ns_per_legendre"),
    ("inversions", "ns_per_inversion"), ("frobenius_maps", "ns_per_frobenius"),
    ("canonicalisations", "ns_per_canon"), ("target_guard_probes", "ns_per_lookup"),
]
# src/cryptanalysis/ic_framework/solver_units.rs PINNABLE_NATIVE
PINNABLE_NATIVE = [
    ("hashed_tensor_steps", "ns_per_hashed_tensor_step"), ("pkm_system_fp_mults_uncharged", "ns_per_fp_mul"),
    ("pkm_grid_fp_mults_uncharged", "ns_per_fp_mul"), ("pkm_shared_poly_fp_mults_uncharged", "ns_per_fp_mul"),
    ("pkm_trace_learn_spec_fp_mults_uncharged", "ns_per_fp_mul"), ("pkm_t0_base_map_fp_mults_uncharged", "ns_per_fp_mul"),
    ("pkm_base_check_fp_mults_uncharged", "ns_per_fp_mul"), ("pkm_t0_target_fp_mults_uncharged", "ns_per_fp_mul"),
]
# docs/ic/calibration.json at e9d5dbdccd, instances["koblitz/K_0 / GF(2^13)"] (the K_0, n = 13 curve)
PINNED_K0_GF2_13 = {
    "ns_per_double": 0.993644, "ns_per_as_solve": 0.051165, "ns_per_lookup": 0.011926, "ns_per_row_op": 0.016905,
    "ns_per_word_xor": 0.002304, "ns_per_frobenius": 0.067551, "ns_per_canon": 0.063438,
}
# src/cryptanalysis/ecbench/band.rs
NON_OPERATION_COUNTERS = ["residual_table_bytes_uncharged", "factor_base_vec_bytes_floor_uncharged", "native_table_bytes_uncharged"]
TABLE_WORK_COUNTERS = ["table_insert_attempts"]
SIBLING_WORK_COUNTERS = ["sibling_second_points", "sibling_lift_products"]
WEIGHTS = [0.0, 0.1, 1.0]

DESCENT = ["ic-descent-inherited-f4", "ic-descent-fes-f2"]
RHO = ["rho-sfstrong-v2", "rho-negation"]


# --- Student t quantile, written here (no scipy) ------------------------------------------------
def _betacf(a, b, x):
    MAXIT, EPS, FPMIN = 300, 3e-16, 1e-300
    qab, qap, qam = a + b, a + 1.0, a - 1.0
    c, d = 1.0, 1.0 - qab * x / qap
    d = 1.0 / (d if abs(d) > FPMIN else FPMIN)
    h = d
    for m in range(1, MAXIT + 1):
        m2 = 2 * m
        aa = m * (b - m) * x / ((qam + m2) * (a + m2))
        d = 1.0 + aa * d
        d = 1.0 / (d if abs(d) > FPMIN else FPMIN)
        c = 1.0 + aa / c
        c = c if abs(c) > FPMIN else FPMIN
        h *= d * c
        aa = -(a + m) * (qab + m) * x / ((a + m2) * (qap + m2))
        d = 1.0 + aa * d
        d = 1.0 / (d if abs(d) > FPMIN else FPMIN)
        c = 1.0 + aa / c
        c = c if abs(c) > FPMIN else FPMIN
        de = d * c
        h *= de
        if abs(de - 1.0) < EPS:
            break
    return h


def betai(a, b, x):
    if x <= 0.0:
        return 0.0
    if x >= 1.0:
        return 1.0
    lbt = math.lgamma(a + b) - math.lgamma(a) - math.lgamma(b) + a * math.log(x) + b * math.log(1.0 - x)
    if x < (a + 1.0) / (a + b + 2.0):
        return math.exp(lbt) * _betacf(a, b, x) / a
    return 1.0 - math.exp(lbt) * _betacf(b, a, 1.0 - x) / b


def t_cdf(t, df):
    x = df / (df + t * t)
    tail = 0.5 * betai(df / 2.0, 0.5, x)
    return 1.0 - tail if t >= 0 else tail


def t_quantile(p, df):
    lo, hi = -1e3, 1e3
    for _ in range(300):
        mid = 0.5 * (lo + hi)
        if t_cdf(mid, df) < p:
            lo = mid
        else:
            hi = mid
    return 0.5 * (lo + hi)


# --- helpers -------------------------------------------------------------------------------------
def mean(xs):
    return sum(xs) / len(xs)


def unpriced_counts(r):
    out = {}
    for name in r["cost"].get("unpriced") or []:
        base = name[: -len("_uncharged")] if name.endswith("_uncharged") else name
        if name in (r.get("counters") or {}):
            out[name] = r["counters"][name]
            continue
        total, found = 0, False
        for p in r.get("phases") or []:
            nat = p.get("native") or {}
            v = nat.get(name, nat.get(base))
            if v is not None:
                total += v
                found = True
        if not found and base.startswith("solver_") and r.get("solver"):
            out[name] = r["solver"]["ops"]
            continue
        out[name] = total if found else None
    return out


def table_work_counts(r):
    out = {}
    phases = r.get("phases") or []
    for name in TABLE_WORK_COUNTERS + SIBLING_WORK_COUNTERS:
        marker = "counted:" + name
        if not any(marker in (p.get("work_native") or {}) for p in phases):
            continue
        out[name] = sum((p.get("work_native") or {}).get(name, 0) for p in phases)
    return out


def priced_counts(r):
    c = {k: v for k, v in unpriced_counts(r).items() if k not in NON_OPERATION_COUNTERS}
    for k, n in table_work_counts(r).items():
        c[k] = (c.get(k) or 0) + n if c.get(k, 0) is not None else None
    return c


def calibrated_native(r):
    pinned = (r.get("detail") or {}).get("calibration_pinned_units") or []
    if not pinned:
        return 0.0, 0, []
    table = PINNED_K0_GF2_13
    notes = []
    gae, ops = 0.0, 0
    for p in r["phases"]:
        nat = p.get("native") or {}
        for counter, unit in PRICED_NATIVE + PINNABLE_NATIVE:
            n = nat.get(counter, 0)
            if n > 0 and unit in pinned:
                if unit not in table:
                    raise SystemExit(f"{r['record_id']}: pinned unit {unit} has no transcribed ratio")
                gae += n * table[unit]
                ops += n
    s = r.get("solver")
    if s:
        solver_charged = not any(u.startswith("solver_") for u in r["cost"].get("unpriced") or [])
        if s["ops"] > 0 and "word XOR" in s["op_unit"] and "ns_per_word_xor" in pinned and solver_charged:
            gae += s["ops"] * table["ns_per_word_xor"]
            ops += s["ops"]
        elif s["ops"] > 0:
            notes.append("solver ops not priced through ns_per_word_xor (op_unit %r)" % s["op_unit"])
    return gae, ops, notes


def linearised_ratio(clusters):
    k = len(clusters)
    sa = sum(a for a, _ in clusters)
    sb = sum(b for _, b in clusters)
    R = sb / sa
    u = [(b - R * a) / sa for a, b in clusters]
    ubar = mean(u)
    se = math.sqrt(k / (k - 1) * sum((x - ubar) ** 2 for x in u))
    df = k - 1
    q = t_quantile(0.975, df)
    half = q * se / R
    return {
        "estimate": R, "se": se, "df": df, "t975": q,
        "ci95_log": [R * math.exp(-half), R * math.exp(half)],
        "ci95_linear": [R - q * se, R + q * se],
    }


def order_verdict(a, b, sa, sb):
    lt = all(x < y for x, y in zip(sa, sb))
    gt = all(x > y for x, y in zip(sa, sb))
    if lt:
        return f"{a} < {b}"
    if gt:
        return f"{b} < {a}"
    return "unresolved inside the band"


def main():
    sdir, proto_path, out = sys.argv[1], sys.argv[2], sys.argv[3]
    raw = open(f"{sdir}/records.jsonl", "rb").read()
    recs = [json.loads(l) for l in raw.decode().splitlines() if l.strip()]
    session = json.load(open(f"{sdir}/session.json"))
    plan = json.load(open(f"{sdir}/plan.json"))
    spec = json.load(open(f"{sdir}/spec.json"))
    proto = json.load(open(proto_path))
    res = {"schema": "validator-recompute/v1", "session_id": session["session_id"]}
    findings = []

    # ---- integrity of the record set ----
    arms_spec = [a["name"] for a in spec["arms"]]
    W = spec["workloads"]["targets_per_curve"]
    rounds = spec["measurement"]["rounds"] + spec["measurement"]["warmup"]
    integ = {
        "records": len(recs),
        "expected": W * rounds * len(arms_spec),
        "records_sha256": hashlib.sha256(raw).hexdigest(),
        "records_sha256_matches_session_json": hashlib.sha256(raw).hexdigest() == session["records_sha256"],
        "seq_is_0_to_n_minus_1": sorted(r["seq"] for r in recs) == list(range(len(recs))),
        "one_session_id": {r["session_id"] for r in recs} == {session["session_id"]},
        "one_binary_sha256": sorted({r["binary_sha256"] for r in recs}),
        "binary_matches_session_json": {r["binary_sha256"] for r in recs} == {session["binary_sha256"]},
        "one_env_class": sorted({r["env_class_id"] for r in recs}),
        "statuses": {},
        "warmup_only_round0": all((r["round"] == 0) == r["warmup"] for r in recs),
    }
    for r in recs:
        integ["statuses"][r["outcome"]["status"]] = integ["statuses"].get(r["outcome"]["status"], 0) + 1
    cells = {}
    for r in recs:
        cells.setdefault((r["workload"]["workload_id"], r["round"], r["arm"]), []).append(r)
    integ["every_cell_exactly_once"] = len(cells) == integ["expected"] and all(len(v) == 1 for v in cells.values())
    # shared seeds per (workload, round)
    seeds = {}
    for r in recs:
        seeds.setdefault((r["workload"]["workload_id"], r["round"]), set()).add(r["algorithm_seed"])
    integ["algorithm_seed_shared_by_all_arms"] = all(len(v) == 1 for v in seeds.values())
    # plan agreement
    pe = {e["seq"]: e for e in plan["executions"]}
    wl = [w["workload_id"] for w in plan["workloads"]]
    integ["records_match_plan"] = all(
        pe[r["seq"]]["round"] == r["round"] and pe[r["seq"]]["warmup"] == r["warmup"]
        and arms_spec[pe[r["seq"]]["arm"]] == r["arm"] and wl[pe[r["seq"]]["workload"]] == r["workload"]["workload_id"]
        and str(pe[r["seq"]]["algorithm_seed"]) == str(r["algorithm_seed"]) for r in recs)
    # arm order: alternate_measured -> measured round index (round - warmup) even: spec order, odd: reversed
    order_ok = True
    for (w, rd) in seeds:
        seq_arms = [x[2] for x in sorted(((cells[(w, rd, a)][0]["seq"], 0, a) for a in arms_spec))]
        if rd == 0:
            continue
        idx = rd - spec["measurement"]["warmup"]
        want = arms_spec if idx % 2 == 0 else list(reversed(arms_spec))
        order_ok &= seq_arms == want
    integ["alternate_measured_order_holds"] = order_ok
    # frozen seeds
    def tag_seed(tag):
        return int(hashlib.sha256(tag.encode()).hexdigest()[:12], 16)
    integ["target_seed_from_protocol_tag"] = tag_seed("fes_k13_validated_20261112/a1/target_seed")
    integ["measurement_seed_from_protocol_tag"] = tag_seed("fes_k13_validated_20261112/a1/measurement_seed")
    integ["target_seed_matches"] = integ["target_seed_from_protocol_tag"] == spec["workloads"]["target_seed"] == recs[0]["workload"]["target_seed"]
    integ["measurement_seed_matches"] = integ["measurement_seed_from_protocol_tag"] == spec["measurement"]["seed"]
    res["integrity"] = integ

    measured = [r for r in recs if not r["warmup"]]
    by_arm = {a: [r for r in measured if r["arm"] == a] for a in arms_spec}

    # ---- per-record accounting checks and band pricing ----
    acct = {"total_gae_eq_sum_phase_gae": True, "s_eq_total_over_sqrt_r": True, "max_abs_dev_total_vs_phases": 0.0,
            "max_rel_dev_s": 0.0, "ic_G_minus_C_eq_adds_plus_doubles": True, "ic_max_abs_dev_G_minus_C": 0.0,
            "solver_pricing_notes": []}
    per_rec = {}
    for r in recs:
        sq = math.sqrt(r["workload"]["curve"]["r"])
        G = r["cost"]["total_gae"]
        ph = sum(p["gae"] for p in r["phases"])
        dev = abs(G - ph)
        acct["max_abs_dev_total_vs_phases"] = max(acct["max_abs_dev_total_vs_phases"], dev)
        if dev > 1e-6 * max(1.0, G):
            acct["total_gae_eq_sum_phase_gae"] = False
        rel = abs(r["cost"]["s"] - G / sq) / (G / sq)
        acct["max_rel_dev_s"] = max(acct["max_rel_dev_s"], rel)
        if rel > 1e-12:
            acct["s_eq_total_over_sqrt_r"] = False
        C, nC, notes = calibrated_native(r)
        acct["solver_pricing_notes"] += [f"seq {r['seq']}: {n}" for n in notes]
        pc = priced_counts(r)
        missing = any(v is None for v in pc.values())
        U = sum(v for v in pc.values() if v is not None)
        if r["method"]["family"] == "ic":
            group = sum(p["adds"] + p["doubles"] for p in r["phases"])
            d = abs((G - C) - group)
            acct["ic_max_abs_dev_G_minus_C"] = max(acct["ic_max_abs_dev_G_minus_C"], d)
            if d > 1e-6:
                acct["ic_G_minus_C_eq_adds_plus_doubles"] = False
        priced = [None if (missing and w > 0) else (G - C + w * (U + nC)) / sq for w in WEIGHTS]
        per_rec[r["seq"]] = {"G": G, "C": C, "n_C": nC, "U": U, "U_parts": pc, "missing": missing, "priced_s": priced}
    acct["solver_pricing_notes"] = acct["solver_pricing_notes"][:10]
    res["accounting_checks"] = acct

    # ---- per-arm means ----
    arms = {}
    for a in arms_spec:
        rs = by_arm[a]
        ok = [r for r in rs if r["outcome"]["status"] == "verified"]
        s = [r["cost"]["s"] for r in ok]
        band = []
        for i, w in enumerate(WEIGHTS):
            vals = [per_rec[r["seq"]]["priced_s"][i] for r in ok]
            band.append(None if any(v is None for v in vals) else mean(vals))
        unp_names = sorted({n for r in rs for n in (r["cost"].get("unpriced") or [])})
        unp_mean = {n: mean([(per_rec[r["seq"]]["U_parts"].get(n) or 0) for r in ok]) for n in sorted({k for r in ok for k in per_rec[r["seq"]]["U_parts"]})}
        lv = {}
        for r in rs:
            lv[r["isolation"]["level"]] = lv.get(r["isolation"]["level"], 0) + 1
        # 95 % interval of mean S, workloads as clusters, t(K-1), on the log scale
        wmeans = {}
        for r in ok:
            wmeans.setdefault(r["workload"]["workload_id"], []).append(r["cost"]["s"])
        wm = [mean(v) for v in wmeans.values()]
        m_s = mean(s)
        k = len(wm)
        se_w = math.sqrt(sum((x - mean(wm)) ** 2 for x in wm) / (k - 1)) / math.sqrt(k)
        q = t_quantile(0.975, k - 1)
        arms_ci = [m_s * math.exp(-q * se_w / m_s), m_s * math.exp(q * se_w / m_s)]
        arms[a] = {
            "role": rs[0]["role"], "method": rs[0]["method"]["id"], "measured": len(rs), "verified": len(ok),
            "mean_s": mean(s), "min_s": min(s), "max_s": max(s),
            "mean_s_ci95_log_cluster_t": arms_ci,
            "band_mean_s": dict(zip(["0", "0.1", "1"], band)),
            "charged_sum_gae": sum(per_rec[r["seq"]]["G"] - per_rec[r["seq"]]["C"] for r in ok),
            "native_sum": sum(per_rec[r["seq"]]["U"] + per_rec[r["seq"]]["n_C"] for r in ok),
            "unpriced_listed": unp_names, "priced_counter_mean_counts": unp_mean,
            "lower_bound_flag": sorted({r["cost"]["lower_bound"] for r in rs}),
            "isolation_levels": lv,
        }
    res["arms"] = arms

    # ---- paired comparison inherited (a) vs fes (b) ----
    A, B = DESCENT
    pairs, seed_ok = [], True
    for (w, rd, arm), v in cells.items():
        if arm != A or rd == 0:
            continue
        ra, rb = v[0], cells[(w, rd, B)][0]
        seed_ok &= ra["algorithm_seed"] == rb["algorithm_seed"]
        pairs.append((w, rd, ra["cost"]["s"], rb["cost"]["s"], ra["cost"]["total_gae"], rb["cost"]["total_gae"]))
    clusters_s, per_w = [], {}
    for w in sorted({p[0] for p in pairs}):
        ps = [p for p in pairs if p[0] == w]
        a_s, b_s = sum(p[2] for p in ps), sum(p[3] for p in ps)
        clusters_s.append((a_s, b_s))
        per_w[w] = {"pairs": len(ps), "a_mean_s": a_s / len(ps), "b_mean_s": b_s / len(ps), "ratio_b_over_a": b_s / a_s,
                    "per_pair_ratios": [round(p[3] / p[2], 6) for p in sorted(ps, key=lambda q: q[1])]}
    lin = linearised_ratio(clusters_s)
    lin_gae = linearised_ratio([(sum(p[4] for p in pairs if p[0] == w), sum(p[5] for p in pairs if p[0] == w)) for w in sorted(per_w)])
    per_pair = [p[3] / p[2] for p in pairs]
    res["paired_fes_over_inherited"] = {
        "a": A, "b": B, "pairs": len(pairs), "workloads": len(per_w), "same_seeds": seed_ok,
        "ratio_of_totals": lin["estimate"], "ci95_linearised_t_log": lin["ci95_log"], "ci95_linearised_t_linear": lin["ci95_linear"],
        "se": lin["se"], "df": lin["df"], "t975": lin["t975"],
        "gae_basis_ratio": lin_gae["estimate"], "gae_basis_ci95": lin_gae["ci95_log"],
        "ci_entirely_below_1": lin["ci95_log"][1] < 1.0,
        "per_workload": per_w,
        "pairs_with_b_lt_a": sum(1 for x in per_pair if x < 1), "per_pair_ratio_min": min(per_pair), "per_pair_ratio_max": max(per_pair),
        "band_ratio_b_over_a": {k: arms[B]["band_mean_s"][k] / arms[A]["band_mean_s"][k] for k in ["0", "0.1", "1"]},
    }
    res["t_quantile_selftest"] = {"t975_df7": t_quantile(0.975, 7), "textbook": 2.364624251592785,
                                  "abs_err": abs(t_quantile(0.975, 7) - 2.364624251592785)}

    # ---- band ranks over every arm pair ----
    ranks = []
    names = sorted(arms_spec)
    for i, a in enumerate(names):
        for b in names[i + 1:]:
            sa = [arms[a]["band_mean_s"][k] for k in ["0", "0.1", "1"]]
            sb = [arms[b]["band_mean_s"][k] for k in ["0", "0.1", "1"]]
            ranks.append({"a": a, "b": b, "verdict": order_verdict(a, b, sa, sb),
                          "order_by_weight": {k: ("<" if x < y else ">" if x > y else "=") for k, x, y in zip(["0", "0.1", "1"], sa, sb)}})
    res["band_ranks"] = ranks

    # ---- distance to rho (H3) ----
    dist = {}
    for ic in DESCENT + ["ic-pairtable-mitm2"]:
        for rho in RHO:
            dist[f"{ic}/{rho}"] = {
                "charged_mean_s_ratio": arms[ic]["mean_s"] / arms[rho]["mean_s"],
                "band_ratio": {k: arms[ic]["band_mean_s"][k] / arms[rho]["band_mean_s"][k] for k in ["0", "0.1", "1"]},
                "band_order": order_verdict(ic, rho, [arms[ic]["band_mean_s"][k] for k in ["0", "0.1", "1"]],
                                            [arms[rho]["band_mean_s"][k] for k in ["0", "0.1", "1"]]),
            }
    res["distance_to_rho"] = dist

    # ---- H2 ----
    desc_all = [r for r in recs if r["arm"] in DESCENT]
    pb = {}
    for r in desc_all:
        v = ((r.get("detail") or {}).get("decomposition") or {}).get("solver", {}).get("priced_by")
        pb[str(v)] = pb.get(str(v), 0) + 1
    solver_unch = sorted({u for r in recs for u in (r["cost"].get("unpriced") or []) if u.startswith("solver_") and u.endswith("_uncharged")})
    any_solver_unpriced = sorted({u for r in recs for u in (r["cost"].get("unpriced") or []) if u.startswith("solver_")})
    op_units = sorted({(r["arm"], r["solver"]["op_unit"]) for r in desc_all})
    pinned_xor = all("ns_per_word_xor" in ((r.get("detail") or {}).get("calibration_pinned_units") or []) for r in desc_all)
    res["H2"] = {
        "descent_records_all_rounds": len(desc_all), "priced_by_counts": pb,
        "priced_by_pinned_on_every_descent_record": pb == {"pinned": len(desc_all)},
        "solver_uncharged_counters_anywhere": solver_unch, "solver_counters_in_unpriced_anywhere": any_solver_unpriced,
        "ns_per_word_xor_pinned_on_every_descent_record": pinned_xor, "solver_op_units": [list(x) for x in op_units],
        "descent_cost_unpriced_empty": all(not (r["cost"].get("unpriced") or []) for r in desc_all),
        "supported": pb == {"pinned": len(desc_all)} and not solver_unch,
    }

    # ---- fes-f2 op-count structure check (candidate arm) ----
    fs = [r for r in recs if r["arm"] == "ic-descent-fes-f2"]
    over = sorted({r["solver"]["ops"] - 2 * r["solver"]["extra"]["points"] for r in fs})
    per_call_over = sorted({(r["solver"]["ops"] - 2 * r["solver"]["extra"]["points"]) / r["solver"]["calls"] for r in fs})
    res["fes_f2_count_structure"] = {
        "points_eq_calls_times_2_pow_nvars": all(r["solver"]["extra"]["points"] == r["solver"]["calls"] * 2 ** r["solver"]["n_vars"] for r in fs),
        "n_vars": sorted({r["solver"]["n_vars"] for r in fs}),
        "ops_minus_2_points_per_call_distinct": per_call_over[:10],
        "note": "fes-f2 is described as two word XORs per point of the Gray-code enumeration; ops - 2*points is the per-call set-up",
    }

    # ---- A/A control ----
    a0, a1 = "rho-sfstrong-v2", "rho-sfstrong-v2-aa"
    aa_pairs, aa_ident, aa_l2 = 0, 0, 0
    for (w, rd, arm), v in cells.items():
        if arm != a0 or rd == 0:
            continue
        x, y = v[0], cells[(w, rd, a1)][0]
        aa_pairs += 1
        if x["cost"]["total_gae"] == y["cost"]["total_gae"] and x["counters"] == y["counters"] and x["outcome"]["recovered"] == y["outcome"]["recovered"]:
            aa_ident += 1
        if x["isolation"]["level"] in ("L2", "L3") and y["isolation"]["level"] in ("L2", "L3"):
            aa_l2 += 1
    blockers = sorted({b.split(":")[0] + ": " + b.split(":", 2)[1].strip()[:60] for r in recs if r["arm"] in (a0, a1) and not r["warmup"] for b in r["isolation"]["blockers"] if b.startswith("L2")})
    res["aa_control"] = {
        "pairs": aa_pairs, "count_identical_pairs": aa_ident,
        "ops_ratio": arms[a1]["mean_s"] / arms[a0]["mean_s"],
        "pairs_both_at_least_L2": aa_l2, "protocol_threshold": "wall.status admitted, pairs_below_level 0, aa_pairs >= 5, aa_ci95 present",
        "wall_threshold_met": aa_l2 >= 5,
        "L2_blockers_seen_on_aa_arms": blockers,
    }

    # ---- positive control (pair table) ----
    pt = arms["ic-pairtable-mitm2"]
    res["positive_control"] = {
        "arm": "ic-pairtable-mitm2", "verified": pt["verified"], "measured": pt["measured"],
        "mean_s": pt["mean_s"], "above_both_descent_arms_charged": pt["mean_s"] > arms[A]["mean_s"] and pt["mean_s"] > arms[B]["mean_s"],
        "local_reference_s": 197.7,
    }

    # ---- H1 exactly as frozen ----
    band_order = order_verdict(B, A, [arms[B]["band_mean_s"][k] for k in ["0", "0.1", "1"]], [arms[A]["band_mean_s"][k] for k in ["0", "0.1", "1"]])
    all_verified = all(r["outcome"]["status"] == "verified" for r in recs)
    res["H1"] = {
        "frozen_text": [c for c in proto["claims"] if c["id"] == "H1"][0]["text"],
        "leg_interval_below_1": res["paired_fes_over_inherited"]["ci_entirely_below_1"],
        "leg_order_at_every_band_weight": band_order == f"{B} < {A}",
        "band_order_verdict": band_order,
        "order_by_weight": {k: ("fes < inherited" if arms[B]["band_mean_s"][k] < arms[A]["band_mean_s"][k] else "fes > inherited") for k in ["0", "0.1", "1"]},
        "every_measured_IC_run_verified": all_verified,
        "supported": res["paired_fes_over_inherited"]["ci_entirely_below_1"] and band_order == f"{B} < {A}" and all_verified,
    }
    h = res["H1"]
    if h["supported"]:
        h["verdict"] = "supported"
    else:
        why = []
        if not h["leg_interval_below_1"]:
            why.append("the paired 95 % interval is not entirely below 1")
        if not h["leg_order_at_every_band_weight"]:
            why.append("the order is not fes < inherited at every band weight (band verdict: %s)" % band_order)
        if not all_verified:
            why.append("not every run verified")
        if h["leg_interval_below_1"]:
            why.append("the paired 95 % interval itself is below 1")
        h["verdict"] = "not supported: " + "; ".join(why)
    json.dump(res, open(out, "w"), indent=1)
    open(out, "a").write("\n")


if __name__ == "__main__":
    main()
