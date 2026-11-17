#!/usr/bin/env python3
"""Independent recomputation of the L2 ladder-check sessions (validator; stdlib only).

Reads each session's records.jsonl from the committed blob (git show <commit>:<path>), checks the blob
equals the working-tree file, and recomputes from the records alone:
  - record counts, arms, rounds, warm-ups, statuses;
  - a certificate for every record with independent affine short-Weierstrass arithmetic:
    G and Q on the curve, [r]G = O, [r]Q = O, [recovered]G = Q, h*r = group order, r prime,
    [group_order]P = O for three curve points, Hasse bound;
  - P2: per (workload, round) pairing of the two tw arms, every numeric field compared;
  - the GAE saving per pair against 2*(bitlength(d) - 1) doublings predicted by the code reading;
  - the paired ratio of totals sum S_B / sum S_A at w = 0, 0.1, 1 with U = unpriced counts
    (X_uncharged looked up as counters[X_uncharged], else phase native X_uncharged or X) plus the
    phase sum of work_native table_insert_attempts where a 'counted:table_insert_attempts' marker exists;
  - a delta-method (linearised) 95% interval with workloads as clusters and Student t(K-1),
    computed on the log scale, and per-curve ratios.
Usage: l2_recompute.py <repo> <commit> <out.json>
"""
import hashlib, json, math, subprocess, sys
from collections import defaultdict, Counter

REPO, COMMIT, OUT = sys.argv[1], sys.argv[2], sys.argv[3]
BASE = "research/curve25519_wave6_runs_20261115/L2_ladder_check"
ARMS = ("rho-neg", "tw2cj-lazy", "tw2cj-lazy-lcheck")
NON_OP = {"residual_table_bytes_uncharged", "factor_base_vec_bytes_floor_uncharged", "native_table_bytes_uncharged"}
WEIGHTS = (0.0, 0.1, 1.0)


def git(*a):
    return subprocess.run(["git", "-C", REPO, *a], check=True, capture_output=True).stdout


def blob(path):
    data = git("show", f"{COMMIT}:{path}")
    with open(f"{REPO}/{path}", "rb") as fh:
        wt = fh.read()
    return data, (data == wt)

# ---------------- elliptic-curve arithmetic (independent of the solver) ----------------

def ec_add(P, Q, a, p):
    if P is None:
        return Q
    if Q is None:
        return P
    x1, y1 = P
    x2, y2 = Q
    if x1 == x2:
        if (y1 + y2) % p == 0:
            return None
        lam = (3 * x1 * x1 + a) * pow(2 * y1, -1, p) % p
    else:
        lam = (y2 - y1) * pow(x2 - x1, -1, p) % p
    x3 = (lam * lam - x1 - x2) % p
    return (x3, (lam * (x1 - x3) - y1) % p)


def ec_mul(k, P, a, p):
    R, A = None, P
    while k:
        if k & 1:
            R = ec_add(R, A, a, p)
        A = ec_add(A, A, a, p)
        k >>= 1
    return R


def pt(v):
    return tuple(int(c, 0) if isinstance(c, str) else int(c) for c in v)


def on_curve(P, a, b, p):
    x, y = P
    return (y * y - (x * x * x + a * x + b)) % p == 0


def is_prime(n):
    if n < 2:
        return False
    i = 2
    while i * i <= n:
        if n % i == 0:
            return False
        i += 1
    return True


def some_points(a, b, p, n):
    pts, x = [], 1
    while len(pts) < n:
        rhs = (x ** 3 + a * x + b) % p
        if rhs == 0:
            pts.append((x, 0))
        elif pow(rhs, (p - 1) // 2, p) == 1:
            # Tonelli-Shanks
            q, s = p - 1, 0
            while q % 2 == 0:
                q //= 2; s += 1
            z = 2
            while pow(z, (p - 1) // 2, p) != p - 1:
                z += 1
            m, c, t, r = s, pow(z, q, p), pow(rhs, q, p), pow(rhs, (q + 1) // 2, p)
            while t != 1:
                i, t2 = 0, t
                while t2 != 1:
                    t2 = t2 * t2 % p; i += 1
                bb = pow(c, 1 << (m - i - 1), p)
                m, c, t, r = i, bb * bb % p, t * bb * bb % p, r * bb % p
            pts.append((x, r))
        x += 7
    return pts

# ---------------- statistics ----------------

def betacf(a, b, x):
    MAXIT, EPS, FPMIN = 400, 3e-16, 1e-300
    qab, qap, qam = a + b, a + 1, a - 1
    c, d = 1.0, 1 - qab * x / qap
    d = 1 / (d if abs(d) > FPMIN else FPMIN)
    h = d
    for m in range(1, MAXIT + 1):
        m2 = 2 * m
        aa = m * (b - m) * x / ((qam + m2) * (a + m2))
        d = 1 + aa * d; d = 1 / (d if abs(d) > FPMIN else FPMIN)
        c = 1 + aa / c; c = c if abs(c) > FPMIN else FPMIN
        h *= d * c
        aa = -(a + m) * (qab + m) * x / ((a + m2) * (qap + m2))
        d = 1 + aa * d; d = 1 / (d if abs(d) > FPMIN else FPMIN)
        c = 1 + aa / c; c = c if abs(c) > FPMIN else FPMIN
        de = d * c; h *= de
        if abs(de - 1) < EPS:
            break
    return h


def betai(a, b, x):
    if x <= 0:
        return 0.0
    if x >= 1:
        return 1.0
    bt = math.exp(math.lgamma(a + b) - math.lgamma(a) - math.lgamma(b) + a * math.log(x) + b * math.log(1 - x))
    if x < (a + 1) / (a + b + 2):
        return bt * betacf(a, b, x) / a
    return 1 - bt * betacf(b, a, 1 - x) / b


def t_cdf(t, df):
    x = df / (df + t * t)
    tail = 0.5 * betai(df / 2, 0.5, x)
    return 1 - tail if t > 0 else tail


def t_quantile(q, df):
    lo, hi = 0.0, 100.0
    for _ in range(200):
        mid = (lo + hi) / 2
        if t_cdf(mid, df) < q:
            lo = mid
        else:
            hi = mid
    return (lo + hi) / 2


def ratio_ci(clusters):
    """clusters: list of (A_k, B_k). R = sum B / sum A; delta-method z_k = (B_k - R A_k)/mean(A)."""
    K = len(clusters)
    SA = sum(a for a, _ in clusters); SB = sum(b for _, b in clusters)
    R = SB / SA
    meanA = SA / K
    z = [(b - R * a) / meanA for a, b in clusters]
    mz = sum(z) / K
    sd = math.sqrt(sum((v - mz) ** 2 for v in z) / (K - 1))
    se = sd / math.sqrt(K)
    t = t_quantile(0.975, K - 1)
    return R, (R * math.exp(-t * se / R), R * math.exp(t * se / R)), t, K

# ---------------- record helpers ----------------

def unpriced_U(rec):
    tot, missing = 0, []
    for name in rec["cost"].get("unpriced", []):
        if name in NON_OP:
            continue
        base = name[: -len("_uncharged")] if name.endswith("_uncharged") else name
        if name in rec.get("counters", {}):
            tot += rec["counters"][name]; continue
        found = False
        for ph in rec["phases"]:
            nat = ph.get("native", {}) or {}
            if name in nat:
                tot += nat[name]; found = True
            elif base in nat:
                tot += nat[base]; found = True
        if not found:
            missing.append(name)
    tw = 0
    if any("counted:table_insert_attempts" in (ph.get("work_native") or {}) for ph in rec["phases"]):
        tw = sum((ph.get("work_native") or {}).get("table_insert_attempts", 0) for ph in rec["phases"])
    return tot, tw, missing


def flatten(o, pre=""):
    out = {}
    if isinstance(o, dict):
        for k, v in o.items():
            out.update(flatten(v, f"{pre}.{k}" if pre else k))
    elif isinstance(o, list):
        if all(isinstance(x, dict) and "name" in x for x in o):
            for x in o:
                out.update(flatten({k: v for k, v in x.items() if k != "name"}, f"{pre}[{x['name']}]"))
        else:
            for i, x in enumerate(o):
                out.update(flatten(x, f"{pre}[{i}]"))
    else:
        out[pre] = o
    return out

# Fields that are timing / identity / placement and are expected to differ between any two runs.
VOLATILE_PREFIX = ("time.", "isolation.", "online.", "record_id", "run_id", "seq", "arm", "role", "method.",
                   "allocator.", "detail.label", "detail.decomposition.hit_rate")


def volatile(k):
    if k.endswith("wall_ns") or ".wall_ns" in k:
        return True
    return k.startswith(VOLATILE_PREFIX)


def analyse(k):
    path = f"{BASE}/sessions/k{k}/records.jsonl"
    data, same = blob(path)
    recs = [json.loads(l) for l in data.decode().splitlines() if l.strip()]
    res = {"records_path": path, "records_sha256": hashlib.sha256(data).hexdigest(), "working_tree_equals_commit_blob": same,
           "records": len(recs)}
    res["by_arm"] = dict(Counter(r["arm"] for r in recs))
    res["warmup_by_arm"] = dict(Counter(r["arm"] for r in recs if r["warmup"]))
    res["statuses"] = dict(Counter(r["outcome"]["status"] for r in recs))
    res["rounds"] = sorted(set(r["round"] for r in recs))
    res["binary_sha256"] = sorted(set(r["binary_sha256"] for r in recs))
    res["curves"] = sorted(set(r["workload"]["curve"]["slug"] for r in recs))
    res["workloads"] = len(set(r["workload"]["workload_id"] for r in recs))
    res["target_seed"] = sorted(set(r["workload"]["target_seed"] for r in recs))
    # ---- certificates ----
    cert_fail, curve_ok = [], {}
    for r in recs:
        cs = r["workload"]["curve_spec"]
        p, a, b, n, rr = cs["p"], cs["a"], cs["b"], cs["group_order"], cs["r"]
        G = (cs["gx"], cs["gy"])
        key = (p, a, b, n, rr, G)
        if key not in curve_ok:
            h = r["workload"]["curve"]["cofactor"]
            okc = (on_curve(G, a, b, p) and ec_mul(rr, G, a, p) is None and is_prime(rr) and h * rr == n
                   and abs(n - (p + 1)) <= 2 * math.isqrt(p) + 2 and is_prime(p)
                   and all(ec_mul(n, P, a, p) is None for P in some_points(a, b, p, 3))
                   and pt(r["workload"]["curve"]["generator"]) == G)
            curve_ok[key] = okc
        Q = pt(r["workload"]["target"])
        d = int(r["outcome"]["recovered"]) if r["outcome"]["recovered"] is not None else None
        ok = (curve_ok[key] and on_curve(Q, a, b, p) and ec_mul(rr, Q, a, p) is None and d is not None
              and 0 <= d < rr and ec_mul(d, G, a, p) == Q and r["outcome"]["status"] == "verified"
              and r["outcome"]["matches_target"] is True)
        if not ok:
            cert_fail.append(r["record_id"])
    res["certificates"] = {"checked": len(recs), "failed": len(cert_fail), "failed_ids": cert_fail[:20],
                           "curves_checked": len(curve_ok), "curves_ok": sum(curve_ok.values())}
    # ---- pairing ----
    idx = defaultdict(dict)
    dup = 0
    for r in recs:
        key = (r["warmup"], r["round"], r["workload"]["workload_id"])
        if r["arm"] in idx[key]:
            dup += 1
        idx[key][r["arm"]] = r
    res["pair_keys"] = len(idx); res["duplicate_arm_in_key"] = dup
    res["keys_missing_an_arm"] = sum(1 for v in idx.values() if set(v) != set(ARMS))
    # same target and same algorithm seed across arms
    res["same_target_all_arms"] = all(len({pt(x["workload"]["target"]) for x in v.values()}) == 1 for v in idx.values())
    res["same_algorithm_seed_tw_arms"] = all(v["tw2cj-lazy"]["algorithm_seed"] == v["tw2cj-lazy-lcheck"]["algorithm_seed"] for v in idx.values())
    res["same_algorithm_seed_all_arms"] = all(len({x["algorithm_seed"] for x in v.values()}) == 1 for v in idx.values())
    # ---- P2: field-by-field comparison of the two tw arms ----
    diff_fields = defaultdict(lambda: {"pairs_differing": 0, "deltas": Counter()})
    keysets_differ = 0; only_in = Counter()
    for v in idx.values():
        A = flatten(v["tw2cj-lazy"]); B = flatten(v["tw2cj-lazy-lcheck"])
        if set(A) != set(B):
            keysets_differ += 1
            for f in set(A) ^ set(B):
                only_in[("baseline_only:" if f in A else "candidate_only:") + f] += 1
        for f in set(A) | set(B):
            if volatile(f):
                continue
            x, y = A.get(f), B.get(f)
            if x != y:
                diff_fields[f]["pairs_differing"] += 1
                if isinstance(x, (int, float)) and isinstance(y, (int, float)):
                    diff_fields[f]["deltas"][round(x - y, 6)] += 1
                elif isinstance(x, str) and isinstance(y, str):
                    try:
                        diff_fields[f]["deltas"][int(x) - int(y)] += 1
                    except ValueError:
                        diff_fields[f]["deltas"]["str"] += 1
    res["P2"] = {"pairs": len(idx), "pairs_with_different_field_sets": keysets_differ, "fields_present_in_one_arm_only": dict(only_in),
                 "fields_differing": {f: {"pairs_differing": d["pairs_differing"],
                                          "delta_baseline_minus_candidate_min": min((q for q in d["deltas"] if not isinstance(q, str)), default=None),
                                          "delta_baseline_minus_candidate_max": max((q for q in d["deltas"] if not isinstance(q, str)), default=None)}
                                      for f, d in sorted(diff_fields.items())}}
    # ---- the check counters and the saving ----
    st = defaultdict(float); n = 0; mism = 0; mism_ids = []
    for v in idx.values():
        A, B = v["tw2cj-lazy"], v["tw2cj-lazy-lcheck"]
        pa = {ph["name"]: ph for ph in A["phases"]}; pb = {ph["name"]: ph for ph in B["phases"]}
        na, nb = pa["relations"]["native"], pb["relations"]["native"]
        d = int(A["outcome"]["recovered"])
        bl = d.bit_length(); pc = bin(d).count("1")
        n += 1
        st["base_check_doubles"] += na["tw_lazy_check_doubles"]; st["base_check_adds"] += na["tw_lazy_check_adds"]
        st["cand_check_doubles"] += nb["tw_lazy_check_doubles"]; st["cand_check_adds"] += nb["tw_lazy_check_adds"]
        st["base_verify_doubles"] += pa["verify"]["doubles"]; st["base_verify_adds"] += pa["verify"]["adds"]
        st["cand_verify_doubles"] += pb["verify"]["doubles"]; st["cand_verify_adds"] += pb["verify"]["adds"]
        st["lazy_checks_base"] += na["tw_lazy_checks"]; st["lazy_checks_cand"] += nb["tw_lazy_checks"]
        st["lazy_failures"] += na["tw_lazy_failures"] + nb["tw_lazy_failures"]
        st["bitlen_minus_1"] += max(bl - 1, 0); st["popcount_minus_1"] += max(pc - 1, 0)
        saving = A["cost"]["total_gae"] - B["cost"]["total_gae"]
        st["saving_gae"] += saving
        st["base_total_gae"] += A["cost"]["total_gae"]
        pred = 2 * max(bl - 1, 0) if (na["tw_lazy_checks"] == 1 and pa["verify"]["scalar_mults"] == 1) else None
        if pred is None or abs(saving - pred) > 1e-9:
            mism += 1
            if len(mism_ids) < 10:
                mism_ids.append({"wl": A["workload"]["workload_id"], "round": A["round"], "warmup": A["warmup"],
                                 "saving": saving, "pred": pred, "lazy_checks": na["tw_lazy_checks"]})
        # per-check model: double-and-add = (bl-1) doubles + (pc-1) adds; ladder = (pc-1) adds
        if na["tw_lazy_checks"] == 1:
            if (na["tw_lazy_check_doubles"], na["tw_lazy_check_adds"]) != (bl - 1, pc - 1):
                st["base_check_model_mismatch"] += 1
            if (nb["tw_lazy_check_doubles"], nb["tw_lazy_check_adds"]) != (0, pc - 1):
                st["cand_check_model_mismatch"] += 1
        if (pa["verify"]["doubles"], pa["verify"]["adds"]) != (bl - 1, pc - 1):
            st["base_verify_model_mismatch"] += 1
        if (pb["verify"]["doubles"], pb["verify"]["adds"]) != (0, pc - 1):
            st["cand_verify_model_mismatch"] += 1
    res["checks"] = {"pairs": n, **{f"mean_{k2}": v2 / n for k2, v2 in st.items() if not k2.endswith("mismatch")},
                     **{k2: int(v2) for k2, v2 in st.items() if k2.endswith("mismatch")},
                     "pairs_saving_not_equal_2x(bitlen-1)": mism, "examples": mism_ids,
                     "mean_saving_over_mean_base_total": st["saving_gae"] / st["base_total_gae"]}
    # ---- ratios (non-warm-up pairs) ----
    def S(rec, w):
        U, TW, miss = unpriced_U(rec)
        return (rec["cost"]["total_gae"] + w * (U + TW)) / math.sqrt(rec["workload"]["curve"]["r"])
    missing_counts = Counter()
    for r in recs:
        missing_counts.update(unpriced_U(r)[2])
    res["unpriced_names_without_count"] = dict(missing_counts)
    # cross-check record cost.s against total_gae / sqrt(r)
    res["max_abs_s_minus_gae_over_sqrt_r"] = max(abs(r["cost"]["s"] - r["cost"]["total_gae"] / math.sqrt(r["workload"]["curve"]["r"])) for r in recs)
    # cross-check total_gae = sum of phase gae = sum adds + doubles
    res["max_abs_total_minus_phase_gae_sum"] = max(abs(r["cost"]["total_gae"] - sum(ph["gae"] for ph in r["phases"])) for r in recs)
    res["max_abs_ic_phase_gae_minus_adds_doubles"] = max(abs(ph["gae"] - ph["adds"] - ph["doubles"]) for r in recs if r["arm"] != "rho-neg" for ph in r["phases"])
    ratios = {}
    for a_arm, b_arm in (("tw2cj-lazy", "tw2cj-lazy-lcheck"), ("rho-neg", "tw2cj-lazy-lcheck"), ("rho-neg", "tw2cj-lazy")):
        out = {}
        for w in WEIGHTS:
            cl = defaultdict(lambda: [0.0, 0.0]); curve = defaultdict(lambda: [0.0, 0.0])
            totA = totB = 0.0; gA = gB = 0.0; npairs = 0
            for key, v in idx.items():
                if key[0]:
                    continue
                sa, sb = S(v[a_arm], w), S(v[b_arm], w)
                cl[key[2]][0] += sa; cl[key[2]][1] += sb
                c = v[a_arm]["workload"]["curve"]["slug"]
                curve[c][0] += sa; curve[c][1] += sb
                gA += v[a_arm]["cost"]["total_gae"] + w * sum(unpriced_U(v[a_arm])[:2])
                gB += v[b_arm]["cost"]["total_gae"] + w * sum(unpriced_U(v[b_arm])[:2])
                npairs += 1
            R, ci, t, K = ratio_ci(list(cl.values()))
            out[str(w)] = {"ratio_sumS": R, "ci95_linearised": ci, "t_quantile": t, "clusters": K, "pairs": npairs,
                           "ratio_sum_priced_gae": gB / gA,
                           "per_curve": {c: v2[1] / v2[0] for c, v2 in sorted(curve.items())},
                           "order": ("b<a" if R < 1 else "b>a")}
        out["order_holds_across_band"] = len({out[str(w)]["order"] for w in WEIGHTS}) == 1 and all(
            (out[str(w)]["ci95_linearised"][1] < 1) or (out[str(w)]["ci95_linearised"][0] > 1) for w in WEIGHTS)
        ratios[f"{b_arm} / {a_arm}"] = out
    res["ratios"] = ratios
    # per-arm mean S (w=0) over non-warm-up runs
    res["mean_S_w0"] = {arm: sum(r["cost"]["s"] for r in recs if r["arm"] == arm and not r["warmup"]) /
                        sum(1 for r in recs if r["arm"] == arm and not r["warmup"]) for arm in ARMS}
    return res


def main():
    out = {"commit": git("rev-parse", COMMIT).decode().strip(), "sessions": {}}
    for k in (20, 24):
        out["sessions"][f"k{k}"] = analyse(k)
    with open(OUT, "w") as fh:
        json.dump(out, fh, indent=1, default=str)
    print(json.dumps(out, indent=1, default=str)[:20000])


if __name__ == "__main__":
    main()
