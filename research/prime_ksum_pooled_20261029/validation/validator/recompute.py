#!/usr/bin/env python3
"""Validator recompute for research/prime_ksum_pooled_20261029 (stdlib only).

Written by the validator, sharing no code with ecbench, the study's node
tooling, or the replayer's/preflight scripts. Reads records.jsonl only
(plus, as a cross-reference, the curve registry entry for these curves).

Checks
  1. Certificates: [d]G = Q in affine arithmetic for every record (counts
     reported for measured verified holdout records, and separately for
     warm-ups and train). Curve (a, b) are not in the record: they are
     solved from G and one target and must then hold for G and every other
     target of that curve; j(a, b) must equal the ICV1 j; p, r prime by
     Miller-Rabin; r = p + 1 - t; Hasse; cofactor 1; [r]G = O; [r]Q = O.
  2. Pricing: S(w) = (total_gae - C + w * (U + n_C)) / sqrt(r), with
     C = n_C = 0 (asserted: no calibration_pinned_units), U = sum of the
     counts behind cost.unpriced (counter lookup: top-level counters[name],
     else phase native[name] or native[name minus _uncharged], else
     solver.ops for solver_*), byte counters excluded (asserted absent).
     Reading "pre": U as above (pre-2026-11-08). Reading "post": U plus the
     phase sum of work_native.table_insert_attempts (and sibling counters)
     where a phase carries the counted:<name> marker (2026-11-08 default).
  3. Pooled paired ratios B/A = sum S_B / sum S_A over (workload, round)
     pairs of measured verified runs; per-curve band means; orders.
"""
import json, math, random, sys
from collections import defaultdict, Counter
from pathlib import Path

SESS = Path(sys.argv[1])          # recovered sessions dir
OUT = Path(sys.argv[2])           # output json
REGISTRY = Path(sys.argv[3]) if len(sys.argv) > 3 else None
WEIGHTS = (0.0, 0.1, 1.0)
WKEY = {0.0: "0", 0.1: "0.1", 1.0: "1"}
BYTE_COUNTERS = {"residual_table_bytes_uncharged", "factor_base_vec_bytes_floor_uncharged",
                 "native_table_bytes_uncharged"}
TABLE_WORK = ["table_insert_attempts"]
SIBLING_WORK = ["sibling_second_points", "sibling_lift_products"]
SESSIONS = ["train-P20", "train-P24", "train-P28", "train-P32",
            "holdout-P20", "holdout-P24", "holdout-P28", "holdout-P32"]

# ------------------------------------------------------------------ number theory
def is_probable_prime(n):
    if n < 2:
        return False
    small = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37]
    for q in small:
        if n % q == 0:
            return n == q
    d, s = n - 1, 0
    while d % 2 == 0:
        d //= 2; s += 1
    for a in small:  # deterministic for n < 3.3e24
        x = pow(a, d, n)
        if x in (1, n - 1):
            continue
        for _ in range(s - 1):
            x = x * x % n
            if x == n - 1:
                break
        else:
            return False
    return True

def ec_add(P, Q, a, p):
    if P is None:
        return Q
    if Q is None:
        return P
    x1, y1 = P; x2, y2 = Q
    if x1 == x2:
        if (y1 + y2) % p == 0:
            return None
        lam = (3 * x1 * x1 + a) * pow(2 * y1, -1, p) % p
    else:
        lam = (y2 - y1) * pow(x2 - x1, -1, p) % p
    x3 = (lam * lam - x1 - x2) % p
    y3 = (lam * (x1 - x3) - y1) % p
    return (x3, y3)

def ec_mul(k, P, a, p):
    R = None
    for bit in bin(k)[2:] if k > 0 else "":
        R = ec_add(R, R, a, p)
        if bit == "1":
            R = ec_add(R, P, a, p)
    return R

def on_curve(P, a, b, p):
    x, y = P
    return (y * y - (x * x * x + a * x + b)) % p == 0

def pt(v):
    return (int(v[0], 16) if isinstance(v[0], str) else int(v[0]),
            int(v[1], 16) if isinstance(v[1], str) else int(v[1]))

# ------------------------------------------------------------------ pricing
def unpriced_U(r):
    """Sum of counts behind cost.unpriced; returns (U, missing_names, byte_names)."""
    U = 0; missing = []; byte = []
    for name in r["cost"]["unpriced"]:
        if name in BYTE_COUNTERS:
            byte.append(name); continue
        base = name[:-len("_uncharged")] if name.endswith("_uncharged") else name
        if name in r.get("counters", {}):
            U += int(r["counters"][name]); continue
        found = False; tot = 0
        for ph in r["phases"]:
            nat = ph.get("native") or {}
            if name in nat:
                tot += int(nat[name]); found = True
            elif base in nat:
                tot += int(nat[base]); found = True
        if not found and base.startswith("solver_") and r.get("solver"):
            U += int(r["solver"]["ops"]); continue
        if found:
            U += tot
        else:
            missing.append(name)
    return U, missing, byte

def table_work(r):
    out = {}
    for name in TABLE_WORK + SIBLING_WORK:
        marker = "counted:" + name
        if any(marker in (ph.get("work_native") or {}) for ph in r["phases"]):
            out[name] = sum(int((ph.get("work_native") or {}).get(name, 0)) for ph in r["phases"])
    return out

# ------------------------------------------------------------------ load
recs = []
schema_issues = Counter()
for s in SESSIONS:
    with open(SESS / s / "records.jsonl") as f:
        for line in f:
            r = json.loads(line)
            c = r["workload"]["curve"]
            U, missing, byte = unpriced_U(r)
            tw = table_work(r)
            pinned = (r.get("detail") or {}).get("calibration_pinned_units") or []
            if pinned:
                schema_issues["calibration_pinned_units non-empty"] += 1
            if byte:
                schema_issues["byte counter in cost.unpriced"] += 1
            if missing:
                schema_issues["unpriced counter without count"] += 1
            if r["cost"]["unit"] != "ecbench.gae":
                schema_issues["unit not ecbench.gae"] += 1
            recs.append({
                "session": s, "size": int(s.split("P")[1]), "kind": s.split("-")[0],
                "seq": r["seq"], "round": r["round"], "warmup": r["warmup"], "arm": r["arm"],
                "method": r["method"]["id"], "params": r["method"]["params"],
                "wid": r["workload"]["workload_id"], "slug": c["slug"], "icv1": c["icv1"],
                "r": int(c["r"]), "order": int(c["group_order"]), "cof": int(c["cofactor"]),
                "G": pt(c["generator"]), "Q": pt(r["workload"]["target"]),
                "status": r["outcome"]["status"], "rec": r["outcome"]["recovered"],
                "matches_target": r["outcome"]["matches_target"],
                "G_gae": float(r["cost"]["total_gae"]), "s_rec": r["cost"]["s"],
                "U": U, "TW": tw.get("table_insert_attempts", 0), "SIB": sum(v for k, v in tw.items() if k in SIBLING_WORK),
                "tw_marker": "table_insert_attempts" in tw,
                "seed": r["algorithm_seed"], "aut": r["workload"]["curve"].get("automorphisms_available"),
                "aut_used": (r.get("boundaries") or {}).get("automorphisms_used"),
                "unpriced_names": tuple(r["cost"]["unpriced"]),
            })

# ------------------------------------------------------------------ 1. curves and certificates
curves = {}
by_slug = defaultdict(list)
for x in recs:
    by_slug[x["slug"]].append(x)
registry = {}
if REGISTRY and REGISTRY.is_file():
    for cv in json.loads(REGISTRY.read_text())["curves"]:
        registry[cv["slug"]] = cv
curve_problems = []
for slug, xs in sorted(by_slug.items()):
    icv = xs[0]["icv1"].split(":")
    assert icv[0] == "ICV1" and icv[1].startswith("fp-"), icv
    p = int(icv[1][3:]); t = int(icv[2]); order_icv = int(icv[3]); j_icv = int(icv[4])
    G = xs[0]["G"]; r = xs[0]["r"]
    consistent = all(y["icv1"] == xs[0]["icv1"] and y["G"] == G and y["r"] == r for y in xs)
    targets = sorted({y["Q"] for y in xs})
    Q0 = next(Q for Q in targets if (Q[0] - G[0]) % p)
    lhsG = (G[1] ** 2 - G[0] ** 3) % p; lhsQ = (Q0[1] ** 2 - Q0[0] ** 3) % p
    a = (lhsG - lhsQ) * pow(G[0] - Q0[0], -1, p) % p
    b = (lhsG - a * G[0]) % p
    disc = (4 * a ** 3 + 27 * b ** 2) % p
    j = 1728 * 4 * a ** 3 * pow(disc, -1, p) % p if disc else None
    all_on = all(on_curve(Q, a, b, p) for Q in targets) and on_curve(G, a, b, p)
    rG = ec_mul(r, G, a, p)
    rQ_ok = all(ec_mul(r, Q, a, p) is None for Q in targets)
    reg = registry.get(slug)
    reg_match = None
    if reg is not None:
        reg_match = (reg["p"] == p and reg["a"] % p == a and reg["b"] % p == b
                     and (reg["generator"]["x"], reg["generator"]["y"]) == G
                     and reg["subgroup_order"] == r and reg["cofactor"] == 1)
    cv = {
        "p": p, "a": a, "b": b, "trace": t, "r": r, "j_icv1": j_icv, "j_from_ab": j,
        "G": list(G), "n_targets": len(targets), "n_records": len(xs),
        "sizes": sorted({y["size"] for y in xs}), "kinds": sorted({y["kind"] for y in xs}),
        "p_prime": is_probable_prime(p), "r_prime": is_probable_prime(r),
        "r_eq_p_plus_1_minus_t": r == p + 1 - t, "order_fields_agree": r == order_icv == xs[0]["order"],
        "cofactor_1": all(y["cof"] == 1 for y in xs), "hasse": t * t <= 4 * p,
        "disc_nonzero": disc != 0, "j_matches": j == j_icv, "j_not_0_1728": j not in (0, 1728),
        "all_points_on_curve": all_on, "rG_is_O": rG is None, "rQ_is_O_all": rQ_ok,
        "record_fields_consistent": consistent, "registry_match": reg_match,
        "automorphisms_available": sorted({y["aut"] for y in xs}),
        "log2_r": math.log2(r),
    }
    cv["ok"] = all(cv[k] for k in ["p_prime", "r_prime", "r_eq_p_plus_1_minus_t", "order_fields_agree",
                                   "cofactor_1", "hasse", "disc_nonzero", "j_matches", "all_points_on_curve",
                                   "rG_is_O", "rQ_is_O_all", "record_fields_consistent"]) and reg_match is not False
    if not cv["ok"]:
        curve_problems.append(slug)
    curves[slug] = cv

cert = Counter()
cert_fail = []
dlog_by_wid = defaultdict(set)
for x in recs:
    cv = curves[x["slug"]]
    p, a = cv["p"], cv["a"]
    grp = ("holdout" if x["kind"] == "holdout" else "train") + ("_warmup" if x["warmup"] else "_measured")
    cert[grp + "_records"] += 1
    if x["status"] != "verified":
        cert[grp + "_not_verified_status"] += 1
        continue
    cert[grp + "_status_verified"] += 1
    d = int(x["rec"])
    ok = 0 <= d < x["r"] and ec_mul(d, x["G"], a, p) == x["Q"]
    if ok:
        cert[grp + "_dG_eq_Q"] += 1
        dlog_by_wid[x["wid"]].add(d)
    else:
        cert[grp + "_dG_ne_Q"] += 1
        cert_fail.append({"session": x["session"], "seq": x["seq"], "arm": x["arm"], "d": x["rec"]})
    if x["matches_target"] is not True:
        cert[grp + "_matches_target_not_true"] += 1
cert["workloads_with_multiple_distinct_logs"] = sum(1 for v in dlog_by_wid.values() if len(v) > 1)
cert["workloads"] = len(dlog_by_wid)

# ------------------------------------------------------------------ 2. pricing
def S(x, w, reading):
    U = x["U"] + ((x["TW"] + x["SIB"]) if reading == "post" else 0)
    return (x["G_gae"] + w * U) / math.sqrt(x["r"])

s_rec_maxrel = 0.0
for x in recs:
    if x["s_rec"] is not None:
        s_rec_maxrel = max(s_rec_maxrel, abs(S(x, 0.0, "pre") - x["s_rec"]) / x["s_rec"])

meas = [x for x in recs if not x["warmup"] and x["status"] == "verified"]
H = [x for x in meas if x["kind"] == "holdout"]
idx = defaultdict(dict)  # (size, arm) -> {(wid, round): rec}
for x in H:
    k = (x["wid"], x["round"])
    assert k not in idx[(x["size"], x["arm"])], ("duplicate pair key", x["session"], x["arm"], k)
    idx[(x["size"], x["arm"])][k] = x

# t quantile (Student) for the linearised interval, own implementation
def betacf(a, b, x):
    MAXIT, EPS, FPMIN = 300, 3e-16, 1e-300
    qab, qap, qam = a + b, a + 1.0, a - 1.0
    c, d = 1.0, 1.0 - qab * x / qap
    d = 1.0 / (d if abs(d) > FPMIN else FPMIN); h = d
    for m in range(1, MAXIT + 1):
        m2 = 2 * m
        aa = m * (b - m) * x / ((qam + m2) * (a + m2))
        d = 1.0 + aa * d; d = FPMIN if abs(d) < FPMIN else d
        c = 1.0 + aa / c; c = FPMIN if abs(c) < FPMIN else c
        d = 1.0 / d; h *= d * c
        aa = -(a + m) * (qab + m) * x / ((a + m2) * (qap + m2))
        d = 1.0 + aa * d; d = FPMIN if abs(d) < FPMIN else d
        c = 1.0 + aa / c; c = FPMIN if abs(c) < FPMIN else c
        d = 1.0 / d; de = d * c; h *= de
        if abs(de - 1.0) < EPS:
            break
    return h

def betai(a, b, x):
    if x <= 0: return 0.0
    if x >= 1: return 1.0
    bt = math.exp(math.lgamma(a + b) - math.lgamma(a) - math.lgamma(b) + a * math.log(x) + b * math.log(1 - x))
    if x < (a + 1) / (a + b + 2):
        return bt * betacf(a, b, x) / a
    return 1.0 - bt * betacf(b, a, 1 - x) / b

def t_cdf(t, nu):
    x = nu / (nu + t * t)
    tail = 0.5 * betai(nu / 2, 0.5, x)
    return 1 - tail if t > 0 else tail

def t_quant(q, nu):
    lo, hi = 0.0, 100.0
    for _ in range(200):
        mid = (lo + hi) / 2
        if t_cdf(mid, nu) < q: lo = mid
        else: hi = mid
    return (lo + hi) / 2

def paired(size, A, B, w, reading, boot=False):
    ia, ib = idx.get((size, A)), idx.get((size, B))
    if not ia or not ib:
        return None
    keys = sorted(set(ia) & set(ib))
    unpaired = len(set(ia) ^ set(ib))
    sa = sum(S(ia[k], w, reading) for k in keys); sb = sum(S(ib[k], w, reading) for k in keys)
    R = sb / sa
    # clusters = workloads
    cl = defaultdict(lambda: [0.0, 0.0])
    for k in keys:
        cl[k[0]][0] += S(ia[k], w, reading); cl[k[0]][1] += S(ib[k], w, reading)
    Ak = [v[0] for v in cl.values()]; Bk = [v[1] for v in cl.values()]
    K = len(Ak); mA = sum(Ak) / K
    z = [(bk - R * ak) / mA for ak, bk in zip(Ak, Bk)]
    mz = sum(z) / K
    sd = math.sqrt(sum((v - mz) ** 2 for v in z) / (K - 1))
    se = sd / math.sqrt(K)
    tq = t_quant(0.975, K - 1)
    lin = (R * math.exp(-tq * se / R), R * math.exp(tq * se / R))
    out = {"ratio": R, "pairs": len(keys), "unpaired": unpaired, "workloads": K,
           "sum_a": sa, "sum_b": sb, "ci95_linearised_own": list(lin),
           "same_seeds": all(ia[k]["seed"] == ib[k]["seed"] for k in keys)}
    if boot:
        rng = random.Random(20261002)
        pairs = list(zip(Ak, Bk)); stats = []
        for _ in range(4000):
            sA = sB = 0.0
            for _ in range(K):
                ak, bk = pairs[rng.randrange(K)]
                sA += ak; sB += bk
            stats.append(sB / sA)
        stats.sort()
        out["ci95_cluster_bootstrap_own_rng"] = [stats[int(0.025 * 4000)], stats[int(0.975 * 4000) - 1]]
    return out

PAIRS = [("inc-sel", "ksum5-sel"), ("inc-sel", "ksum4-sel"), ("inc-w0", "ksum5-sel"), ("inc-w0", "ksum4-sel"),
         ("inc-rec", "ksum5-sel"), ("inc-rec", "ksum4-sel"), ("inc-rec", "inc-sel"), ("ksum4-sel", "ksum5-sel"),
         ("rho-neg", "ksum5-sel"), ("rho-neg", "ksum4-sel"), ("rho-neg", "inc-sel"), ("bsgs-neg", "ksum5-sel"),
         ("bsgs-neg", "ksum4-sel"), ("rho-neg", "rho-neg-aa"), ("rho-neg", "bsgs-neg")]
ratios = {}
for reading in ("pre", "post"):
    for size in (20, 24, 28, 32):
        for A, B in PAIRS:
            for w in WEIGHTS:
                res = paired(size, A, B, w, reading, boot=(reading == "pre"))
                if res is not None:
                    ratios[f"{reading}|P{size}|{A}__{B}|w{WKEY[w]}"] = res

# band rows: per curve x arm, mean S over measured verified runs
band = {}
for reading in ("pre", "post"):
    g = defaultdict(list)
    for x in H:
        g[(x["slug"], x["arm"])].append(x)
    allruns = defaultdict(int)
    for x in recs:
        if x["kind"] == "holdout" and not x["warmup"]:
            allruns[(x["slug"], x["arm"])] += 1
    for (slug, arm), xs in g.items():
        band[f"{reading}|{slug}|{arm}"] = {
            "curve": slug, "arm": arm, "size": xs[0]["size"], "log2_r": math.log2(xs[0]["r"]),
            "runs": allruns[(slug, arm)], "verified": len(xs),
            "mean_s": {WKEY[w]: sum(S(x, w, reading) for x in xs) / len(xs) for w in WEIGHTS},
            "charged_sum": sum(x["G_gae"] for x in xs),
            "native_sum": sum(x["U"] + ((x["TW"] + x["SIB"]) if reading == "post" else 0) for x in xs),
            "mean_unpriced_U_pre": sum(x["U"] for x in xs) / len(xs),
            "mean_table_insert_attempts": sum(x["TW"] for x in xs) / len(xs),
            "unpriced_counters": sorted({n for x in xs for n in x["unpriced_names"]}),
        }

def order_check(reading):
    res = {}
    for slug in sorted({x["slug"] for x in H}):
        row = {}
        for w in WEIGHTS:
            k = WKEY[w]
            rho = band[f"{reading}|{slug}|rho-neg"]["mean_s"][k]
            k5 = band[f"{reading}|{slug}|ksum5-sel"]["mean_s"][k]
            k4 = band[f"{reading}|{slug}|ksum4-sel"]["mean_s"][k]
            inc = band[f"{reading}|{slug}|inc-sel"]["mean_s"][k]
            bsgs = band[f"{reading}|{slug}|bsgs-neg"]["mean_s"][k]
            w0 = band.get(f"{reading}|{slug}|inc-w0", {}).get("mean_s", {}).get(k)
            row[k] = {"rho_neg": rho, "ksum5_sel": k5, "ksum4_sel": k4, "inc_sel": inc, "bsgs_neg": bsgs,
                      "inc_w0": w0,
                      "rho_lt_ksum5_lt_inc": rho < k5 < inc, "ksum4_lt_inc": k4 < inc,
                      "ksum5_lt_inc_w0": (k5 < w0) if w0 is not None else None,
                      "ksum4_lt_inc_w0": (k4 < w0) if w0 is not None else None,
                      "bsgs_lt_ksum5": bsgs < k5, "bsgs_lt_ksum4": bsgs < k4}
        row["holds_all_weights"] = all(row[WKEY[w]]["rho_lt_ksum5_lt_inc"] for w in WEIGHTS)
        row["ksum4_lt_inc_all_weights"] = all(row[WKEY[w]]["ksum4_lt_inc"] for w in WEIGHTS)
        row["size"] = band[f"{reading}|{slug}|rho-neg"]["size"]
        res[slug] = row
    return res
orders = {r: order_check(r) for r in ("pre", "post")}

# ------------------------------------------------------------------ selection recompute (train)
T = [x for x in recs if x["kind"] == "train"]
sel = {}
for reading in ("pre", "post"):
    for size in (20, 24, 28, 32):
        cells = defaultdict(list); allc = defaultdict(int)
        for x in T:
            if x["size"] != size or x["warmup"]:
                continue
            allc[x["arm"]] += 1
            if x["status"] == "verified":
                cells[x["arm"]].append(x)
        out = {}
        for fam, pref, pkey in (("inc", "inc-n", "factor_base"), ("ksum4", "ksum4-a", "factor_base"),
                                ("ksum5", "ksum5-a", "factor_base")):
            cand = []
            for arm, xs in cells.items():
                if not arm.startswith(pref) or len(xs) != allc[arm]:
                    continue
                size_param = int(xs[0]["params"]["factor_base"].split("size=")[1])
                for w in WEIGHTS:
                    pass
                m1 = sum(S(x, 1.0, reading) for x in xs) / len(xs)
                m0 = sum(S(x, 0.0, reading) for x in xs) / len(xs)
                cand.append((m1, size_param, arm, m0))
            best1 = min(cand, key=lambda c: (c[0], c[1]))
            best0 = min(cand, key=lambda c: (c[3], c[1]))
            out[fam] = {"sel_w1": best1[1], "sel_w1_mean_S": best1[0], "sel_w0": best0[1], "sel_w0_mean_S": best0[3],
                        "grid": sorted((c[1], round(c[0], 6), round(c[3], 6)) for c in cand)}
        k4 = out["ksum4"]["sel_w1_mean_S"]; k5 = out["ksum5"]["sel_w1_mean_S"]
        out["k_star"] = 5 if k5 < k4 else 4
        sel[f"{reading}|P{size}"] = out

# ------------------------------------------------------------------ simple log-log exponent (cross-check only)
def ols_exponent(arm, w, reading):
    pts = []
    for x in H:
        if x["arm"] == arm:
            pts.append((math.log(x["r"]), math.log(S(x, w, reading) * math.sqrt(x["r"]))))
    n = len(pts); mx = sum(p[0] for p in pts) / n; my = sum(p[1] for p in pts) / n
    return sum((p[0] - mx) * (p[1] - my) for p in pts) / sum((p[0] - mx) ** 2 for p in pts)
expo = {f"{reading}|{arm}|w{WKEY[w]}": ols_exponent(arm, w, reading)
        for reading in ("pre", "post") for arm in ("rho-neg", "bsgs-neg", "inc-sel", "ksum4-sel", "ksum5-sel")
        for w in WEIGHTS}

counts = Counter()
for x in recs:
    counts[f"{x['kind']}_records"] += 1
    counts[f"{x['kind']}_{x['status']}"] += 1
    counts[f"{x['kind']}_{'warmup' if x['warmup'] else 'measured'}"] += 1

out = {
    "schema_issues": dict(schema_issues), "record_counts": dict(counts),
    "curves": curves, "curve_problems": curve_problems,
    "certificates": dict(cert), "certificate_failures": cert_fail[:50],
    "cost_s_vs_gae_over_sqrt_r_max_rel": s_rec_maxrel,
    "ratios": ratios, "band": band, "orders": orders, "selection": sel, "ols_exponent_crosscheck": expo,
    "table_work_marker_arms": sorted({(x["session"].split("-")[0], x["arm"].split("-")[0]) for x in recs if x["tw_marker"]}),
    "sibling_work_total": sum(x["SIB"] for x in recs),
}
OUT.write_text(json.dumps(out, indent=1, default=list))
print("schema issues:", dict(schema_issues))
print("record counts:", dict(counts))
print("curve problems:", curve_problems, "| curves:", len(curves))
print("certificates:", dict(cert))
print("cost.s max rel diff:", s_rec_maxrel)
