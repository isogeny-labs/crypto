#!/usr/bin/env python3
"""Independent validator recompute for XP-20261029-dg-wall.

Stdlib only.  Shares no code with ecbench or with research/ic_dg_wall_20261029/cards/tools.
The estimator definitions follow the frozen protocol (PROTOCOL.md section 5) and the
documented ecbench formulas (compare.rs / stats.rs / band.rs at be2969ce, read, not reused).

Usage: python3 -I dgwval.py <evidence_dir> <out_dir>
"""
import collections
import hashlib
import json
import math
import os
import random
import sys

EV = sys.argv[1]
OUT = sys.argv[2]
SIZES = (24, 28, 32)
WEIGHTS = (0.0, 0.1, 1.0)
WKEY = {0.0: "0", 0.1: "0.1", 1.0: "1"}
LEVEL = {"L0": 0, "L1": 1, "L2": 2, "L3": 3}
BOOT_RESAMPLES = 10000
BOOT_SEED = 20261029


# ---------------------------------------------------------------- Student t (own code)
def _betacf(a, b, x):
    # Lentz continued fraction for the regularised incomplete beta.
    tiny = 1e-300
    qab, qap, qam = a + b, a + 1.0, a - 1.0
    c, d = 1.0, 1.0 - qab * x / qap
    d = tiny if abs(d) < tiny else d
    d = 1.0 / d
    h = d
    for m in range(1, 10000):
        m2 = 2 * m
        aa = m * (b - m) * x / ((qam + m2) * (a + m2))
        d = 1.0 + aa * d
        d = tiny if abs(d) < tiny else d
        c = 1.0 + aa / c
        c = tiny if abs(c) < tiny else c
        d = 1.0 / d
        h *= d * c
        aa = -(a + m) * (qab + m) * x / ((a + m2) * (qap + m2))
        d = 1.0 + aa * d
        d = tiny if abs(d) < tiny else d
        c = 1.0 + aa / c
        c = tiny if abs(c) < tiny else c
        d = 1.0 / d
        de = d * c
        h *= de
        if abs(de - 1.0) < 1e-16:
            break
    return h


def betainc(a, b, x):
    if x <= 0.0:
        return 0.0
    if x >= 1.0:
        return 1.0
    lbt = math.lgamma(a + b) - math.lgamma(a) - math.lgamma(b) + a * math.log(x) + b * math.log1p(-x)
    bt = math.exp(lbt)
    if x < (a + 1.0) / (a + b + 2.0):
        return bt * _betacf(a, b, x) / a
    return 1.0 - bt * _betacf(b, a, 1.0 - x) / b


def t_cdf(t, df):
    x = df / (df + t * t)
    tail = 0.5 * betainc(df / 2.0, 0.5, x)
    return 1.0 - tail if t > 0 else tail


def t_q975(df):
    lo, hi = 0.0, 1.0
    while t_cdf(hi, df) < 0.975:
        lo, hi = hi, hi * 2.0
    for _ in range(300):
        mid = 0.5 * (lo + hi)
        if t_cdf(mid, df) < 0.975:
            lo = mid
        else:
            hi = mid
        if hi - lo < 1e-15:
            break
    return 0.5 * (lo + hi)


# ---------------------------------------------------------------- estimators (own code)
def se_from_influence(u):
    k = len(u)
    m = sum(u) / k
    return math.sqrt(k / (k - 1.0) * sum((x - m) ** 2 for x in u))


def log_ci(est, se, df):
    q = t_q975(df)
    h = q * se / est
    return [est * math.exp(-h), est * math.exp(h)]


def ns_per_gae_ratio(pairs):
    """pairs: list of (a_wall, b_wall, a_gae, b_gae). Returns dict with ns/GAE per arm,
    ratio rho = ns_b/ns_a, its pair-linearised se and df, and per-pair log-influence."""
    aw = sum(p[0] for p in pairs)
    bw = sum(p[1] for p in pairs)
    ag = sum(p[2] for p in pairs)
    bg = sum(p[3] for p in pairs)
    na, nb = aw / ag, bw / bg
    rho = nb / na
    infl_log = [p[1] / bw - p[3] / bg - p[0] / aw + p[2] / ag for p in pairs]
    se = se_from_influence([rho * x for x in infl_log])
    return {"ns_a": na, "ns_b": nb, "rho": rho, "se": se, "df": len(pairs) - 1.0, "infl_log": infl_log,
            "sums": (aw, bw, ag, bg)}


def totals_ratio(clusters):
    """Ratio of totals sum b / sum a over clusters (a, b), linearised se, df = K - 1."""
    sa = sum(c[0] for c in clusters)
    sb = sum(c[1] for c in clusters)
    r = sb / sa
    u = [(c[1] - r * c[0]) / sa for c in clusters]
    return {"r": r, "se": se_from_influence(u), "df": len(clusters) - 1.0}


def ratio_ci(est, se, df):
    if est > 0:
        return log_ci(est, se, df)
    q = t_q975(df)
    return [est - q * se, est + q * se]


def divide_indep(n_est, n_se, n_df, d_est, d_se, d_df):
    s1, s2 = n_se / n_est, d_se / d_est
    est = n_est / d_est
    v = s1 * s1 + s2 * s2
    df = v * v / (s1 ** 4 / n_df + s2 ** 4 / d_df) if v > 0 else min(n_df, d_df)
    return est, est * math.sqrt(v), df


# ---------------------------------------------------------------- record loading
def seal_ok(line, rid):
    field = '"record_id":"%s"' % rid
    if field not in line or not rid.startswith("ECR1h"):
        return False
    body = line.replace(field, '"record_id":""', 1)
    return hashlib.sha256(body.encode()).hexdigest()[:12] == rid[5:]


def unpriced_count(r):
    """Count behind each name in cost.unpriced, by the documented band rule: top-level
    counters first, then the phases' native maps under the name or its base."""
    out = {}
    for name in r["cost"]["unpriced"]:
        base = name[:-len("_uncharged")] if name.endswith("_uncharged") else name
        if name in (r.get("counters") or {}):
            out[name] = r["counters"][name]
            continue
        tot, found = 0, False
        for p in r.get("phases") or []:
            nat = p.get("native") or {}
            if name in nat:
                tot += nat[name]; found = True
            elif base in nat:
                tot += nat[base]; found = True
        if not found and base.startswith("solver_") and r.get("solver"):
            out[name] = r["solver"]["ops"]
            continue
        out[name] = tot if found else None
    return out


def load(path):
    recs = []
    seals_bad = 0
    raw_lines = 0
    with open(path) as fh:
        for line in fh:
            line = line.rstrip("\n")
            if not line:
                continue
            raw_lines += 1
            r = json.loads(line)
            if not seal_ok(line, r["record_id"]):
                seals_bad += 1
            up = unpriced_count(r)
            rel = {}
            for p in r.get("phases") or []:
                if p["name"] == "relations":
                    rel = p.get("native") or {}
            recs.append({
                "seq": r["seq"], "arm": r["arm"], "role": r["role"], "warmup": r["warmup"], "round": r["round"],
                "wid": r["workload"]["workload_id"], "slug": r["workload"]["curve"]["slug"],
                "tidx": r["workload"]["target_index"], "r": r["workload"]["curve"]["r"],
                "icv1": r["workload"]["curve"]["icv1"], "auts": r["workload"]["curve"]["automorphisms_available"],
                "auts_used": r["boundaries"]["automorphisms_used"],
                "method_id": r["method"]["method_id"], "method": r["method"]["id"], "params": r["method"].get("params"),
                "seed": r["algorithm_seed"], "status": r["outcome"]["status"],
                "matches_target": r["outcome"]["matches_target"],
                "gae": r["cost"]["total_gae"], "s": r["cost"]["s"], "unpriced": up,
                "pinned": (r.get("detail") or {}).get("calibration_pinned_units") or [],
                "level": r["isolation"]["level"], "blockers": r["isolation"]["blockers"],
                "solve_wall": r["time"]["solve_wall_ns"],
                "phase_wall": {p["name"]: p.get("wall_ns") for p in r.get("phases") or []},
                "phase_gae": {p["name"]: p.get("gae") for p in r.get("phases") or []},
                "online_phases": (r.get("online") or {}).get("phases_ns"),
                "online_wall": (r.get("online") or {}).get("wall_ns"),
                "rel": {k: rel.get(k) for k in ("trials", "lookups", "relations", "repeated_targets_skipped",
                                                 "target_guard_probes", "stride_renewals", "stride_steps")},
                "binary": r["binary_sha256"], "env": r["env_class_id"], "session_id": r["session_id"],
            })
    return recs, seals_bad, raw_lines


def counts(r):
    return (not r["warmup"]) and r["status"] == "verified"


def priced_s(r, w):
    if r["pinned"]:
        raise SystemExit("record %s carries pinned calibration units; not expected" % r["seq"])
    u = r["unpriced"]
    if w > 0 and any(v is None for v in u.values()):
        return None
    return (r["gae"] + w * sum(v for v in u.values() if v is not None)) / math.sqrt(r["r"])


def by_key(recs, arm):
    return {(r["wid"], r["round"]): r for r in recs if r["arm"] == arm and counts(r)}


# ---------------------------------------------------------------- wall block
def method_of(recs):
    m = {}
    for r in recs:
        m.setdefault(r["arm"], (r["method_id"], r["role"]))
    return m


def choose_aa(arms, a, b):
    names = sorted(arms)
    if arms[a][0] == arms[b][0]:
        return None
    cands = []
    for i, x in enumerate(names):
        for y in names[i + 1:]:
            if arms[x][0] == arms[y][0] and {x, y} != {a, b}:
                cands.append((x, y))
    for x, y in cands:
        if a in (x, y):
            return a, (y if x == a else x), "includes arm A"
    refs = [n for n in names if arms[n][1] == "reference"]
    for x, y in cands:
        if x in refs or y in refs:
            den = x if x in refs else y
            return den, (y if den == x else x), "includes the reference arm"
    return (cands[0][0], cands[0][1], "first alphabetical") if cands else None


def wall_pairs(recs, a, b, required="L2"):
    A, B = by_key(recs, a), by_key(recs, b)
    keys = sorted(set(A) & set(B))
    admitted, below, n = [], 0, 0
    akeys = []
    for k in keys:
        ra, rb = A[k], B[k]
        if ra["solve_wall"] is None or rb["solve_wall"] is None:
            continue
        n += 1
        lo = LEVEL[ra["level"]] < LEVEL[required] or LEVEL[rb["level"]] < LEVEL[required]
        below += lo
        if not lo and ra["solve_wall"] > 0 and rb["solve_wall"] > 0 and ra["gae"] > 0 and rb["gae"] > 0:
            admitted.append((float(ra["solve_wall"]), float(rb["solve_wall"]), ra["gae"], rb["gae"]))
            akeys.append(k)
    return admitted, akeys, n, below


def wall_block(recs, a, b):
    arms = method_of(recs)
    adm, akeys, n, below = wall_pairs(recs, a, b)
    w = {"pairs": n, "pairs_below_level": below, "admitted": len(adm)}
    f = ns_per_gae_ratio(adm)
    w.update(ns_per_gae_a=f["ns_a"], ns_per_gae_b=f["ns_b"], ratio_b_over_a=f["rho"],
             ci95=log_ci(f["rho"], f["se"], f["df"]), dof=f["df"])
    tw = totals_ratio([(p[0], p[1]) for p in adm])
    w.update(total_wall_ratio=tw["r"], total_wall_ci95=log_ci(tw["r"], tw["se"], tw["df"]))
    if arms[a][0] == arms[b][0]:
        w.update(self_aa=True, aa_arms=None)
        return w, f, None, akeys
    w["self_aa"] = False
    ch = choose_aa(arms, a, b)
    den, num, why = ch
    aadm, aakeys, _, _ = wall_pairs(recs, den, num)
    af = ns_per_gae_ratio(aadm)
    est, se, df = divide_indep(f["rho"], f["se"], f["df"], af["rho"], af["se"], af["df"])
    cci = log_ci(est, se, df)
    w.update(aa_arms=[den, num], aa_selection=why, aa_pairs=len(aadm), aa_ratio=af["rho"],
             aa_ci95=log_ci(af["rho"], af["se"], af["df"]), corrected_ratio=est, corrected_ci95=cci,
             corrected_dof=df, outside_noise=(cci[0] > 1.0 or cci[1] < 1.0))
    return w, f, (af, aakeys, (den, num)), akeys


# ---------------------------------------------------------------- ops block (counted units)
def ops_block(recs, a, b, w):
    A, B = by_key(recs, a), by_key(recs, b)
    keys = sorted(set(A) & set(B))
    per_w = collections.defaultdict(list)
    for k in keys:
        sa, sb = priced_s(A[k], w), priced_s(B[k], w)
        if sa is None or sb is None:
            continue
        per_w[k[0]].append((sa, sb, A[k]["slug"]))
    pairs = sum(len(v) for v in per_w.values())

    def rep(wids):
        cl = [(sum(p[0] for p in per_w[x]), sum(p[1] for p in per_w[x])) for x in wids]
        tr = totals_ratio(cl)
        return {"ratio_b_over_a": tr["r"], "ci95": ratio_ci(tr["r"], tr["se"], tr["df"]), "dof": tr["df"],
                "workloads": len(wids), "pairs": sum(len(per_w[x]) for x in wids),
                "sum_a": sum(c[0] for c in cl), "sum_b": sum(c[1] for c in cl)}
    out = rep(sorted(per_w))
    out["pairs"] = pairs
    out["unpaired"] = (len(A) - len(keys)) + (len(B) - len(keys))
    slugs = sorted({v[0][2] for v in per_w.values()})
    out["curves"] = []
    for s in slugs:
        c = rep(sorted(x for x in per_w if per_w[x][0][2] == s))
        c["slug"] = s
        out["curves"].append(c)
    return out


def aa_exact(recs, a, b):
    """Counted A/A: every matched pair identical in charged GAE and in every unpriced count."""
    A, B = by_key(recs, a), by_key(recs, b)
    keys = sorted(set(A) & set(B))
    same = all(A[k]["gae"] == B[k]["gae"] and A[k]["unpriced"] == B[k]["unpriced"] and A[k]["seed"] == B[k]["seed"]
               for k in keys)
    ratios = {}
    for w in WEIGHTS:
        sa = sum(priced_s(A[k], w) for k in keys)
        sb = sum(priced_s(B[k], w) for k in keys)
        ratios[WKEY[w]] = sb / sa
    return {"pairs": len(keys), "pairwise_identical_counts_and_seeds": same, "ratio": ratios,
            "exactly_1_at_all_weights": all(v == 1.0 for v in ratios.values())}


# ---------------------------------------------------------------- clustered intervals (descriptive)
def corrected_clustered(recs, a, b, aa_den, aa_num, keys_filter=None, boot=True, seed=BOOT_SEED):
    """Corrected ratio (ns_b/ns_a)/(ns_aa_num/ns_aa_den) with targets as clusters:
    (1) delta method on the joint log influence summed per target, df = K - 1;
    (2) percentile bootstrap resampling targets (all curves pooled) and stratified by curve."""
    A, B = by_key(recs, a), by_key(recs, b)
    D, N = by_key(recs, aa_den), by_key(recs, aa_num)
    keys = sorted(set(A) & set(B) & set(D) & set(N))
    if keys_filter:
        keys = [k for k in keys if keys_filter(A[k])]
    for k in keys:
        for X in (A, B, D, N):
            assert LEVEL[X[k]["level"]] >= 2
    cl = collections.OrderedDict()
    for k in keys:
        c = cl.setdefault(k[0], {"slug": A[k]["slug"], "v": [0.0] * 8})
        v = c["v"]
        for i, X in enumerate((A, B, D, N)):
            v[2 * i] += X[k]["solve_wall"]
            v[2 * i + 1] += X[k]["gae"]

    def stat(vs):
        t = [sum(v[i] for v in vs) for i in range(8)]
        rba = (t[2] / t[3]) / (t[0] / t[1])
        raa = (t[6] / t[7]) / (t[4] / t[5])
        return rba / raa, rba, raa
    vs = [c["v"] for c in cl.values()]
    est, rba, raa = stat(vs)
    T = [sum(v[i] for v in vs) for i in range(8)]
    # joint log influence per target cluster
    U = []
    for v in vs:
        ub = v[2] / T[2] - v[3] / T[3] - v[0] / T[0] + v[1] / T[1]
        ua = v[6] / T[6] - v[7] / T[7] - v[4] / T[4] + v[5] / T[5]
        U.append(ub - ua)
    K = len(U)
    se_log = se_from_influence(U)
    q = t_q975(K - 1.0)
    out = {"targets": K, "pairs": len(keys), "corrected_ratio": est, "ratio_b_over_a": rba, "aa_ratio": raa,
           "target_clustered_linearised_ci95": [est * math.exp(-q * se_log), est * math.exp(q * se_log)],
           "target_clustered_dof": K - 1.0}
    if boot:
        rng = random.Random(seed)
        bs, bss = [], []
        slugs = sorted({c["slug"] for c in cl.values()})
        strata = {s: [c["v"] for c in cl.values() if c["slug"] == s] for s in slugs}
        for _ in range(BOOT_RESAMPLES):
            bs.append(stat([vs[rng.randrange(K)] for _ in range(K)])[0])
            samp = []
            for s in slugs:
                st = strata[s]
                samp += [st[rng.randrange(len(st))] for _ in range(len(st))]
            bss.append(stat(samp)[0])
        bs.sort(); bss.sort()
        pct = lambda xs, p: xs[min(len(xs) - 1, max(0, int(math.floor(p * len(xs)))))]
        out["target_cluster_bootstrap_ci95"] = [pct(bs, 0.025), pct(bs, 0.975)]
        out["target_cluster_bootstrap_curve_stratified_ci95"] = [pct(bss, 0.025), pct(bss, 0.975)]
        out["bootstrap"] = {"resamples": BOOT_RESAMPLES, "seed": seed, "method": "percentile; resample targets "
                            "(workload ids) with replacement; each target carries its 6 rounds of all four arms"}
    return out


# ---------------------------------------------------------------- mechanism (descriptive)
def mechanism(recs, a="srenew", b="dg"):
    A, B = by_key(recs, a), by_key(recs, b)
    keys = sorted(set(A) & set(B))
    out = {}
    for name, X in ((a, A), (b, B)):
        d = collections.Counter()
        for k in keys:
            r = X[k]
            d["solve_wall_ns"] += r["solve_wall"]
            d["total_gae"] += r["gae"]
            for ph, v in r["phase_wall"].items():
                if v is not None:
                    d["phase_wall_ns." + ph] += v
            for ph, v in r["phase_gae"].items():
                d["phase_gae." + ph] += v
            for kk, v in (r["online_phases"] or {}).items():
                d["online_ns." + kk] += v
            for kk, v in r["rel"].items():
                if v is not None:
                    d["relations_native." + kk] += v
            for kk, v in r["unpriced"].items():
                if v is not None:
                    d["unpriced." + kk] += v
        out[name] = dict(sorted(d.items()))
    gdiff = [(B[k]["gae"] - A[k]["gae"]) for k in keys]
    rep_b = [B[k]["rel"]["repeated_targets_skipped"] for k in keys]
    out["charged_gae_diff_dg_minus_srenew"] = {
        "pairs": len(keys), "pairs_equal": sum(1 for x in gdiff if x == 0),
        "pairs_dg_higher": sum(1 for x in gdiff if x > 0), "pairs_dg_lower": sum(1 for x in gdiff if x < 0),
        "max_diff": max(gdiff), "sum_diff": sum(gdiff),
        "diff_equals_dg_repeated_targets_skipped_on_every_pair": all(gdiff[i] == rep_b[i] for i in range(len(keys))),
    }
    sa, sb = out[a], out[b]
    dw = sb["solve_wall_ns"] - sa["solve_wall_ns"]
    drel = sb.get("phase_wall_ns.relations", 0) - sa.get("phase_wall_ns.relations", 0)
    dprobe = sb["relations_native.target_guard_probes"] - sa["relations_native.target_guard_probes"]
    dlook = sb["relations_native.lookups"] - sa["relations_native.lookups"]
    out["deltas_dg_minus_srenew"] = {
        "solve_wall_ns": dw, "relations_phase_wall_ns": drel,
        "other_phases_wall_ns": {ph: sb.get("phase_wall_ns." + ph, 0) - sa.get("phase_wall_ns." + ph, 0)
                                 for ph in ("factor_base", "oracle_setup", "linear_algebra", "verify")},
        "relations_share_of_solve_wall_delta": drel / dw if dw else None,
        "target_guard_probes": dprobe, "lookups": dlook,
        "relations_wall_delta_per_removed_probe_ns": drel / dprobe if dprobe else None,
        "solve_wall_delta_per_removed_probe_ns": dw / dprobe if dprobe else None,
    }
    return out


# ---------------------------------------------------------------- band rows for the gate
def band_rows(recs, arms):
    out = {}
    slugs = sorted({r["slug"] for r in recs})
    for s in slugs:
        out[s] = {}
        for arm in arms:
            rs = [r for r in recs if r["arm"] == arm and counts(r) and r["slug"] == s]
            out[s][arm] = {WKEY[w]: sum(priced_s(r, w) for r in rs) / len(rs) for w in WEIGHTS}
            out[s][arm]["runs"] = len(rs)
            out[s][arm]["charged_sum_gae"] = sum(r["gae"] for r in rs)
            out[s][arm]["native_sum"] = sum(sum(v for v in r["unpriced"].values() if v is not None) for r in rs)
    return out


# ---------------------------------------------------------------- comparison to committed files
def close(x, y, tol=1e-9):
    if x is None or y is None:
        return x is y
    if isinstance(x, (list, tuple)):
        return len(x) == len(y) and all(close(p, q, tol) for p, q in zip(x, y))
    if isinstance(x, bool) or isinstance(y, bool) or isinstance(x, str) or isinstance(y, str):
        return x == y
    return x == y or abs(x - y) <= tol * max(abs(x), abs(y))


def compare_fields(mine, theirs, fields, prefix, headline, rel_tol=1e-9):
    res = []
    for f in fields:
        mv, tv = mine.get(f), theirs.get(f)
        ok = close(mv, tv, rel_tol)
        res.append({"field": prefix + f, "claimed": tv, "recomputed": mv, "agree": ok})
        if isinstance(mv, (int, float)) and not isinstance(mv, bool) and isinstance(tv, (int, float)) and not isinstance(tv, bool):
            headline.append({"name": prefix + f, "claimed": tv, "recomputed": mv, "rel_tol": rel_tol})
        elif isinstance(mv, list) and isinstance(tv, list) and len(mv) == 2:
            for i, side in enumerate(("lo", "hi")):
                headline.append({"name": prefix + f + "." + side, "claimed": tv[i], "recomputed": mv[i], "rel_tol": rel_tol})
    return res


WALL_FIELDS = ["pairs", "pairs_below_level", "ns_per_gae_a", "ns_per_gae_b", "ratio_b_over_a", "ci95", "dof",
               "total_wall_ratio", "total_wall_ci95", "self_aa", "aa_selection", "aa_arms", "aa_pairs", "aa_ratio",
               "aa_ci95", "corrected_ratio", "corrected_ci95", "corrected_dof", "outside_noise"]
OPS_FIELDS = ["ratio_b_over_a", "ci95", "pairs", "workloads", "dof"]


def main():
    os.makedirs(OUT, exist_ok=True)
    report = {"sizes": {}}
    headline = []
    mismatches = []
    raw_data = []
    for b in SIZES:
        S = {}
        for sess in ("wall", "counted"):
            p = os.path.join(EV, "P%d" % b, sess, "eks-results", "session", "records.jsonl")
            recs, seals_bad, nlines = load(p)
            raw_data.append({"path": os.path.relpath(p, os.path.dirname(EV)),
                             "sha256": hashlib.sha256(open(p, "rb").read()).hexdigest()})
            S[sess] = (recs, seals_bad, nlines)
        wr, cr = S["wall"][0], S["counted"][0]
        R = {"records": {s: {"lines": S[s][2], "seal_failures": S[s][1]} for s in S}}

        # ---- record-level integrity and gate recompute
        for s, recs in (("wall", wr), ("counted", cr)):
            lv = collections.Counter(r["level"] for r in recs)
            st = collections.Counter(r["status"] for r in recs)
            arms = collections.Counter(r["arm"] for r in recs)
            ic_levels = collections.Counter(r["level"] for r in recs if r["method"] == "ic.pipeline")
            rho = [r for r in recs if r["method"] == "rho.negation"]
            rho_below = [r for r in rho if LEVEL[r["level"]] < 2]
            rho_ok = all(r["level"] == "L1" and all(x.startswith("L2: solve too short to admit wall time")
                                                     for x in r["blockers"] if x.startswith("L2: "))
                         and any(x.startswith("L2: ") for x in r["blockers"]) for r in rho_below)
            below_by_arm = collections.Counter(r["arm"] for r in recs if LEVEL[r["level"]] < 2)
            R["records"][s].update({
                "levels": dict(lv), "statuses": dict(st), "arms": dict(arms),
                "measured_per_arm": dict(collections.Counter(r["arm"] for r in recs if not r["warmup"])),
                "ic_levels": dict(ic_levels), "below_l2_by_arm": dict(below_by_arm),
                "rho_below_l2_all_L1_with_only_too_short_L2_blockers": rho_ok if rho else None,
                "rho_below_l2_max_solve_wall_lt_1ms": (max(r["solve_wall"] for r in rho_below) < 1_000_000) if rho_below else None,
                "all_matches_target": all(r["matches_target"] for r in recs),
                "binaries": sorted({r["binary"] for r in recs}), "env_classes": sorted({r["env"] for r in recs}),
                "session_ids": sorted({r["session_id"] for r in recs}),
                "curves": sorted({(r["slug"], r["icv1"], r["auts"]) for r in recs}),
                "s_equals_gae_over_sqrt_r": all(abs(r["s"] - r["gae"] / math.sqrt(r["r"])) <= 1e-12 * r["s"] for r in recs),
                "pinned_calibration_records": sum(1 for r in recs if r["pinned"]),
                "uncounted_unpriced_records": sum(1 for r in recs if any(v is None for v in r["unpriced"].values())),
                "targets_per_curve": {sl: len({r["tidx"] for r in recs if r["slug"] == sl}) for sl in sorted({r["slug"] for r in recs})},
                "rounds": sorted({r["round"] for r in recs if not r["warmup"]}),
            })
        # ---- cross-session consistency: same IC counts and seeds in both sessions
        cross = {}
        for arm in ("srenew", "dg"):
            W, C = by_key(wr, arm), by_key(cr, arm)
            ks = sorted(set(W) | set(C))
            cross[arm] = {"keys_wall": len(W), "keys_counted": len(C), "same_keys": set(W) == set(C),
                          "identical_gae_unpriced_seed": all(k in W and k in C and W[k]["gae"] == C[k]["gae"] and
                                                             W[k]["unpriced"] == C[k]["unpriced"] and W[k]["seed"] == C[k]["seed"]
                                                             for k in ks)}
        R["cross_session_ic_identical"] = cross

        # ---- wall decision block (w independent)
        wb, f, aa, akeys = wall_block(wr, "srenew", "dg")
        R["wall_decision"] = wb
        # descriptive wall blocks
        R["wall_descriptive"] = {}
        for (a_, b_) in (("walk", "srenew"), ("walk", "dg"), ("walk", "dg-small"), ("srenew", "dg-small"),
                         ("srenew", "srenew-aa")):
            R["wall_descriptive"]["%s__%s" % (a_, b_)] = wall_block(wr, a_, b_)[0]
        # target-clustered (pooled) and per-curve corrected
        R["wall_target_clustered"] = corrected_clustered(wr, "srenew", "dg", "srenew", "srenew-aa")
        slugs = sorted({r["slug"] for r in wr})
        R["wall_per_curve"] = {}
        for s in slugs:
            sub = [r for r in wr if r["slug"] == s]
            cwb = wall_block(sub, "srenew", "dg")[0]
            ccl = corrected_clustered(wr, "srenew", "dg", "srenew", "srenew-aa", keys_filter=lambda r, s=s: r["slug"] == s)
            R["wall_per_curve"][s] = {"pairs": cwb["admitted"], "ratio_b_over_a": cwb["ratio_b_over_a"],
                                      "aa_ratio": cwb["aa_ratio"], "corrected_ratio": cwb["corrected_ratio"],
                                      "pair_linearised_corrected_ci95": cwb["corrected_ci95"],
                                      "target_clustered_linearised_ci95": ccl["target_clustered_linearised_ci95"],
                                      "target_cluster_bootstrap_ci95": ccl["target_cluster_bootstrap_ci95"],
                                      "targets": ccl["targets"]}
        # equivalence: corrected ratio with arm-A A/A equals ns(dg)/ns(srenew-aa) over the same pairs
        eq = ns_per_gae_ratio(wall_pairs(wr, "srenew-aa", "dg")[0])["rho"]
        R["wall_identity_corrected_equals_dg_over_srenew_aa"] = {"dg_over_srenew_aa": eq,
                                                                 "corrected_ratio": wb["corrected_ratio"],
                                                                 "rel_diff": eq / wb["corrected_ratio"] - 1.0}
        R["mechanism"] = mechanism(wr)

        # ---- counted units, wall session
        R["ops_wall_session"] = {}
        for (a_, b_) in (("srenew", "dg"), ("srenew", "srenew-aa"), ("walk", "srenew"), ("walk", "dg"),
                         ("walk", "dg-small"), ("srenew", "dg-small")):
            R["ops_wall_session"]["%s__%s" % (a_, b_)] = {WKEY[w]: ops_block(wr, a_, b_, w) for w in WEIGHTS}
        R["aa_counted_wall_session"] = aa_exact(wr, "srenew", "srenew-aa")
        # ---- counted session
        R["ops_counted_session"] = {}
        for (a_, b_) in (("rho-neg", "rho-neg-aa"), ("rho-neg", "srenew"), ("rho-neg", "dg"), ("srenew", "dg")):
            R["ops_counted_session"]["%s__%s" % (a_, b_)] = {WKEY[w]: ops_block(cr, a_, b_, w) for w in WEIGHTS}
        R["aa_counted_counted_session"] = aa_exact(cr, "rho-neg", "rho-neg-aa")
        R["band_counted_session"] = band_rows(cr, ("rho-neg", "srenew", "dg"))

        # ---- compare with committed analysis files
        cmpres = []
        for sess, pairs, files in (
            ("wall", R["ops_wall_session"], "P%d/wall/analysis/compare-%s-w%s.json"),
            ("counted", R["ops_counted_session"], "P%d/counted/analysis/compare-%s-w%s.json")):
            for pk, byw in pairs.items():
                for wk, mine in byw.items():
                    fn = os.path.join(EV, files % (b, pk, wk))
                    if not os.path.exists(fn):
                        cmpres.append({"file": os.path.relpath(fn, EV), "present": False})
                        continue
                    d = json.load(open(fn))
                    res = compare_fields(mine, d["ops"], OPS_FIELDS, "P%d/%s/%s/w%s/ops." % (b, sess, pk, wk), headline)
                    for c in mine["curves"]:
                        tc = [x for x in d["curves"] if x["slug"] == c["slug"]][0]
                        res += compare_fields(c, tc, ["ratio_b_over_a", "ci95", "pairs", "workloads"],
                                              "P%d/%s/%s/w%s/curves[%s]." % (b, sess, pk, wk, c["slug"]), headline)
                    if sess == "wall":
                        a_, b_ = pk.split("__")
                        mw = R["wall_decision"] if pk == "srenew__dg" else R["wall_descriptive"][pk]
                        tw = d["wall"]
                        fields = WALL_FIELDS if not mw.get("self_aa") else [x for x in WALL_FIELDS if not x.startswith(("aa_", "corrected", "outside"))]
                        res += compare_fields(mw, tw, fields, "P%d/wall/%s/w%s/wall." % (b, pk, wk), headline)
                        if tw.get("status") != "admitted":
                            res.append({"field": "P%d/wall/%s/w%s/wall.status" % (b, pk, wk), "claimed": tw.get("status"),
                                        "recomputed": "admitted", "agree": False})
                    cmpres.append({"file": os.path.relpath(fn, EV), "present": True,
                                   "fields": len(res), "disagree": [x for x in res if not x["agree"]],
                                   "resample_seed": d.get("resample_seed"), "pricing_weight": d.get("pricing_weight")})
        # control-comparison.json (native A/A admission, in-job)
        cc = json.load(open(os.path.join(EV, "P%d/wall/eks-results/control-comparison.json" % b)))
        cca, ccb = cc["a"]["arm"], cc["b"]["arm"]
        mine_cc = wall_block(wr, cca, ccb)[0]
        res = compare_fields(mine_cc, cc["wall"], WALL_FIELDS, "P%d/wall/control-comparison(%s__%s)/wall." % (b, cca, ccb), headline)
        cmpres.append({"file": "P%d/wall/eks-results/control-comparison.json" % b, "present": True, "fields": len(res),
                       "disagree": [x for x in res if not x["agree"]], "arms": [cca, ccb]})
        R["committed_file_comparison"] = cmpres
        for c in cmpres:
            for x in c.get("disagree", []):
                mismatches.append(dict(x, file=c["file"]))
        report["sizes"]["P%d" % b] = R
        print("P%d done: corrected %.12f ci %s; mismatching fields %d" % (
            b, wb["corrected_ratio"], wb["corrected_ci95"], sum(len(c.get("disagree", [])) for c in cmpres)), flush=True)
    report["mismatches"] = mismatches
    report["headline"] = headline
    report["raw_data"] = raw_data
    report["t_quantile_selfcheck"] = {str(df): t_q975(df) for df in (1.0, 2.0, 23.0, 47.0, 143.0, 287.0, 440.97243064769395)}
    json.dump(report, open(os.path.join(OUT, "recompute.json"), "w"), indent=1, default=str)


if __name__ == "__main__":
    main()
