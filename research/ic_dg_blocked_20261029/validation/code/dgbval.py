#!/usr/bin/env python3
"""Independent validator recomputation for XP-20261029-dg-blocked.

Validator's own code.  Reads the S3 pod originals (checked byte-equal to the
evidence directory, and bound to bf899979a6 through git blobs or the committed
.artifacts.json manifests: provenance() and snapshot_binding()), the S3 launcher
receipts copied by fetch_launcher.sh, the frozen specs extracted from cd04cfd2d1,
and the committed analysis JSON (only to compare against).  Writes
work/recompute.json under /dev/shm/dgb-validation.  Never writes into the
evidence directory; git is used read-only (ls-tree, show, rev-parse, hash-object).
"""
import hashlib
import json
import math
import os
import subprocess
import sys
from collections import Counter, defaultdict

import numpy as np

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import dgbstats as st  # noqa: E402

EV = "/dev/shm/wt-blk/research/ic_dg_blocked_20261029"
S3 = "/dev/shm/dgb-val/s3"
S3L = "/dev/shm/dgb-val/s3-launcher"
FROZEN = "/dev/shm/dgb-validation/frozen"
OUT = "/dev/shm/dgb-validation/work"
SIZES = (24, 28, 32)
NINC = {24: 162, 28: 407, 32: 1024}
LEV = {"L0": 0, "L1": 1, "L2": 2, "L3": 3}
WEIGHTS = (0.0, 0.1, 1.0)
WTAG = {0.0: "w0", 0.1: "w0.1", 1.0: "w1"}
BE = "be2969cebf30a0cfb79088fd10094324c3efb3bb"
BIN = "88070375e77648aa12a561ff9ee59e34d469c55e1da56987234811b0956e412c"
U32MAX = 4294967295
WALL_ORDER = ["dg", "dg-aa", "dgb", "dgb-edge"]


def jload(p):
    with open(p) as f:
        return json.load(f)


def sha(p):
    h = hashlib.sha256()
    with open(p, "rb") as f:
        for chunk in iter(lambda: f.read(1 << 20), b""):
            h.update(chunk)
    return h.hexdigest()


def records(b, s):
    p = f"{S3}/P{b}-{s}/session/records.jsonl"
    out = []
    with open(p) as f:
        for line in f:
            out.append(json.loads(line))
    return out


def rel(a, b):
    if a is None or b is None:
        return None
    if a == b:
        return 0.0
    return abs(a - b) / max(abs(a), abs(b))


# ---------------------------------------------------------------- pricing (§5a)
def unpriced_counts(r):
    out = {}
    for name in r["cost"].get("unpriced") or []:
        base = name[: -len("_uncharged")] if name.endswith("_uncharged") else name
        if name in (r.get("counters") or {}):
            out[name] = r["counters"][name]
            continue
        tot, found = 0, False
        for ph in r.get("phases") or []:
            nat = ph.get("native") or {}
            v = nat.get(name, nat.get(base))
            if v is not None:
                tot += v
                found = True
        if not found and base.startswith("solver_") and r.get("solver"):
            out[name] = r["solver"].get("ops")
            continue
        out[name] = tot if found else None
    return out


def pinned(r):
    return (r.get("detail") or {}).get("calibration_pinned_units") or []


def priced_s(r, w):
    g = r["cost"]["total_gae"]
    if g is None:
        return None
    if pinned(r):
        raise SystemExit(f"record {r['record_id']} carries pinned calibration units; not handled")
    u = unpriced_counts(r)
    if any(v is None for v in u.values()) and w > 0:
        return None
    return (g + w * sum(v for v in u.values() if v is not None)) / math.sqrt(r["workload"]["curve"]["r"])


# ---------------------------------------------------------------- wall pairs
def wall_pairs(recs, a, b):
    by = defaultdict(dict)
    for r in recs:
        if r["warmup"] or r["outcome"]["status"] != "verified" or r["arm"] not in (a, b):
            continue
        by[(r["workload"]["workload_id"], r["round"])][r["arm"]] = r
    pairs, below = [], 0
    for k in sorted(by):
        d = by[k]
        if a not in d or b not in d:
            continue
        ra, rb = d[a], d[b]
        ta, tb = ra["time"]["solve_wall_ns"], rb["time"]["solve_wall_ns"]
        if ta is None or tb is None:
            continue
        bl = LEV[ra["isolation"]["level"]] < 2 or LEV[rb["isolation"]["level"]] < 2
        below += bl
        pairs.append({
            "curve": ra["workload"]["curve"]["slug"],
            "target": ra["workload"]["target_index"],
            "round": ra["round"],
            "aw": float(ta), "bw": float(tb),
            "ag": float(ra["cost"]["total_gae"]), "bg": float(rb["cost"]["total_gae"]),
            "below": bl,
            "a_seq": ra["seq"], "b_seq": rb["seq"],
        })
    adm = [p for p in pairs if not p["below"] and p["aw"] > 0 and p["bw"] > 0 and p["ag"] > 0 and p["bg"] > 0]
    return {"pairs": len(pairs), "below": below, "all": pairs, "admitted": adm}


def tup(p):
    return (p["aw"], p["bw"], p["ag"], p["bg"])


def fig(pairs):
    return st.per_op_ratio([tup(p) for p in pairs])


def compare_wall(recs, a, b, aa=None):
    d = wall_pairs(recs, a, b)
    f = fig(d["admitted"])
    out = {"a": a, "b": b, "pairs": d["pairs"], "pairs_below_level": d["below"],
           "admitted": len(d["admitted"]),
           "below_pairs": [{k: p[k] for k in ("curve", "target", "round", "a_seq", "b_seq")} for p in d["all"] if p["below"]],
           "ns_per_gae_a": f["ns_per_gae_a"], "ns_per_gae_b": f["ns_per_gae_b"],
           "ratio_b_over_a": f["ratio"], "ci95": f["ci95"], "dof": f["df"], "se": f["se"],
           "total_wall_ratio": f["total_wall_ratio"]}
    fa = fig(d["all"])
    out["all_pairs_ratio_b_over_a"] = fa["ratio"]
    out["all_pairs_ci95"] = fa["ci95"]
    if aa:
        da = wall_pairs(recs, aa[0], aa[1])
        af = fig(da["admitted"])
        c = st.divide_independent(f, af)
        out.update({"aa_arms": list(aa), "aa_pairs": len(da["admitted"]), "aa_ratio": af["ratio"],
                    "aa_ci95": af["ci95"], "corrected_ratio": c["ratio"], "corrected_ci95": c["ci95"],
                    "corrected_dof": c["df"],
                    "outside_noise": (c["ci95"][0] > 1.0 or c["ci95"][1] < 1.0)})
        # all pairs (descriptive, includes below-level pairs)
        afa = fig(da["all"])
        ca = st.divide_independent(fa, afa)
        out["all_pairs_corrected_ratio"] = ca["ratio"]
        out["all_pairs_corrected_ci95"] = ca["ci95"]
    return out, d


# ---------------------------------------------------------------- number theory
def is_prime(n):
    if n < 2:
        return False
    for q in (2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37):
        if n % q == 0:
            return n == q
    d, s = n - 1, 0
    while d % 2 == 0:
        d //= 2
        s += 1
    for a in (2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37):
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


def squarefree_part(n):
    n = abs(n)
    out, q = 1, 2
    while q * q <= n:
        e = 0
        while n % q == 0:
            n //= q
            e += 1
        if e % 2:
            out *= q
        q += 1
    return out * n


def curve_facts(recs_by_slug):
    out = {}
    for slug, rs in sorted(recs_by_slug.items()):
        r0 = rs[0]
        c = r0["workload"]["curve"]
        parts = c["icv1"].split(":")
        assert parts[0] == "ICV1" and parts[1].startswith("fp-"), c["icv1"]
        p = int(parts[1][3:])
        t = int(parts[2])
        order = int(parts[3])
        j_icv1 = int(parts[4])
        G = [int(v, 0) for v in c["generator"]]
        pts = [G] + [[int(v, 0) for v in r["workload"]["target"]] for r in rs[:6]]
        uniq = []
        for q in pts:
            if q not in uniq:
                uniq.append(q)
        (x1, y1), (x2, y2) = uniq[0], uniq[1]
        d1 = (y1 * y1 - x1 ** 3) % p
        d2 = (y2 * y2 - x2 ** 3) % p
        a = (d1 - d2) * pow((x1 - x2) % p, -1, p) % p
        bco = (d1 - a * x1) % p
        on_curve = all((y * y - (x ** 3 + a * x + bco)) % p == 0 for x, y in uniq)
        disc = (4 * a ** 3 + 27 * bco ** 2) % p
        j = 1728 * 4 * pow(a, 3, p) * pow(disc, -1, p) % p
        D = t * t - 4 * p
        out[slug] = {
            "icv1": c["icv1"], "p": p, "p_bits": p.bit_length(), "p_prime": is_prime(p),
            "p_le_u32max": p <= U32MAX, "trace": t, "r": c["r"], "group_order": order,
            "r_eq_p_plus_1_minus_t": order == p + 1 - t and c["r"] == order,
            "r_prime": is_prime(c["r"]), "cofactor": c["cofactor"],
            "hasse": t * t <= 4 * p,
            "a": a, "b": bco, "points_checked_on_curve": len(uniq), "on_curve": on_curve,
            "j_recomputed": j, "j_icv1": j_icv1, "j_matches": j == j_icv1,
            "j_is_0_or_1728": j in (0, 1728 % p),
            "frobenius_discriminant_t2_minus_4p": D,
            "squarefree_part_of_D": -squarefree_part(D),
            "automorphisms_available_record": c.get("automorphisms_available"),
            "automorphism_group": "{+1,-1} (j not in {0, 1728})" if j not in (0, 1728 % p) else "larger",
            "field_degree": c.get("field_degree"), "field_bits": c.get("field_bits"),
        }
    return out


# ---------------------------------------------------------------- record comparison (cond. 8)
def phase_ops(r):
    return {ph["name"]: {"adds": ph.get("adds"), "doubles": ph.get("doubles"),
                         "scalar_mults": ph.get("scalar_mults"), "gae": ph.get("gae"),
                         "native": ph.get("native") or {}} for ph in r.get("phases") or []}


def phase_work_native(r):
    return {ph["name"]: ph.get("work_native") or {} for ph in r.get("phases") or []}


def work_prims(r):
    w = r.get("work") or {}
    return {ph["name"]: ph.get("prims") for ph in w.get("phases") or []}


def charged_view(r):
    return {
        "recovered": r["outcome"]["recovered"],
        "status": r["outcome"]["status"],
        "matches_target": r["outcome"]["matches_target"],
        "total_gae": r["cost"]["total_gae"],
        "unpriced": unpriced_counts(r),
        "phase_ops": phase_ops(r),
        "algorithm_seed": r["algorithm_seed"],
        "counters": r.get("counters"),
    }


def extra_view(r):
    w = r.get("work") or {}
    return {"work_native": phase_work_native(r), "work_prims": work_prims(r),
            "bitops": w.get("bitops"), "cost_s": r["cost"].get("s"),
            "automorphisms_used": (r.get("boundaries") or {}).get("automorphisms_used"),
            "fb_sha256": r["factor_base"]["fb_sha256"],
            "points_sha256": r["factor_base"]["points_sha256"]}


def diff_keys(x, y, prefix=""):
    out = []
    if isinstance(x, dict) and isinstance(y, dict):
        for k in sorted(set(x) | set(y)):
            out += diff_keys(x.get(k), y.get(k), f"{prefix}.{k}" if prefix else k)
    elif x != y:
        out.append(prefix)
    return out


def key_of(r):
    return (r["workload"]["curve"]["slug"], r["workload"]["target_index"], r["round"])


def cond8(recs, ref_arm="dg", arms=("dgb", "dgb-edge")):
    by = {}
    for r in recs:
        if r["arm"] == ref_arm:
            by[key_of(r)] = r
    res = {}
    for arm in arms:
        n = nwarm = 0
        charged_mismatch = []
        extra_mismatch = Counter()
        unmatched = []
        for r in recs:
            if r["arm"] != arm:
                continue
            n += 1
            nwarm += r["warmup"]
            ref = by.get(key_of(r))
            if ref is None or ref["warmup"] != r["warmup"]:
                unmatched.append(r["seq"])
                continue
            dk = diff_keys(charged_view(ref), charged_view(r))
            if dk:
                charged_mismatch.append({"seq": r["seq"], "fields": dk})
            for f in diff_keys(extra_view(ref), extra_view(r)):
                extra_mismatch[f] += 1
        res[arm] = {"records": n, "warmups": nwarm, "unmatched": unmatched,
                    "charged_mismatches": len(charged_mismatch),
                    "charged_mismatch_examples": charged_mismatch[:10],
                    "non_charged_differences_by_field": dict(extra_mismatch)}
    return res


def cross_session(wall, counted, arms=("dg", "dgb")):
    res = {}
    for arm in arms:
        w = {key_of(r): r for r in wall if r["arm"] == arm}
        c = {key_of(r): r for r in counted if r["arm"] == arm}
        mism = []
        for k in sorted(set(w) | set(c)):
            if k not in w or k not in c:
                mism.append({"key": list(k), "fields": ["missing"]})
                continue
            if w[k]["warmup"] != c[k]["warmup"]:
                mism.append({"key": list(k), "fields": ["warmup"]})
            dk = diff_keys(charged_view(w[k]), charged_view(c[k]))
            if dk:
                mism.append({"key": list(k), "fields": dk})
        res[arm] = {"wall_records": len(w), "counted_records": len(c), "mismatches": len(mism),
                    "examples": mism[:10]}
    return res


# ---------------------------------------------------------------- A1
def a1(recs, arms=("dgb", "dgb-edge")):
    n = 0
    fails = []
    ps, sps, srcs = set(), set(), Counter()
    for r in recs:
        if r["arm"] not in arms:
            continue
        n += 1
        p = int(r["workload"]["curve"]["icv1"].split(":")[1][3:])
        fb = r["factor_base"]
        if fb.get("signed_points") is not None:
            sp, src = fb["signed_points"], "signed_points"
        elif fb.get("abscissae") is not None:
            sp, src = 2 * fb["abscissae"], "2*abscissae"
        else:
            sp, src = None, "missing"
        srcs[src] += 1
        ps.add(p)
        sps.add(sp)
        if not (p <= U32MAX and sp is not None and sp <= 65536):
            fails.append(r["seq"])
        if r["method"]["params"]["oracle"] != "mitm-prime-blocked:m=2":
            fails.append(("oracle", r["seq"]))
    return {"records": n, "p_values": sorted(ps), "max_p": max(ps), "signed_points_values": sorted(sps),
            "max_signed_points": max(sps), "signed_point_source": dict(srcs), "failures": fails,
            "holds": not fails and n > 0}


# ---------------------------------------------------------------- positions
def positions(recs):
    blocks = defaultdict(list)
    for r in recs:
        blocks[(r["workload"]["workload_id"], r["round"])].append(r)
    pos = {a: Counter() for a in WALL_ORDER}
    pred = {a: Counter() for a in WALL_ORDER}
    direction = Counter()
    contiguous = True
    pos_meas = {a: Counter() for a in WALL_ORDER}
    pred_meas = {a: Counter() for a in WALL_ORDER}
    dir_meas = Counter()
    order_relative = {"dgb_before_dg-aa": 0, "dg-aa_before_dgb": 0}
    for k, rs in blocks.items():
        rs = sorted(rs, key=lambda r: r["seq"])
        seqs = [r["seq"] for r in rs]
        if seqs != list(range(seqs[0], seqs[0] + len(seqs))):
            contiguous = False
        arms = [r["arm"] for r in rs]
        if arms == WALL_ORDER:
            dname = "forward"
        elif arms == WALL_ORDER[::-1]:
            dname = "reversed"
        else:
            dname = "other:" + ",".join(arms)
        direction[dname] += 1
        meas = not rs[0]["warmup"]
        if meas:
            dir_meas[dname] += 1
            if arms.index("dgb") < arms.index("dg-aa"):
                order_relative["dgb_before_dg-aa"] += 1
            else:
                order_relative["dg-aa_before_dgb"] += 1
        for i, a in enumerate(arms):
            pos[a][i] += 1
            pred[a][arms[i - 1] if i else "(block start)"] += 1
            if meas:
                pos_meas[a][i] += 1
                pred_meas[a][arms[i - 1] if i else "(block start)"] += 1
    return {"blocks": len(blocks), "contiguous_seq_blocks": contiguous,
            "direction_all": dict(direction), "direction_measured": dict(dir_meas),
            "position_measured": {a: {str(k): v for k, v in sorted(c.items())} for a, c in pos_meas.items()},
            "predecessor_measured": {a: dict(c) for a, c in pred_meas.items()},
            "position_all_incl_warmup": {a: {str(k): v for k, v in sorted(c.items())} for a, c in pos.items()},
            "dgb_vs_dg-aa_precedence_measured": order_relative,
            "dgb_and_dg-aa_same_position_distribution": pos_meas["dgb"] == pos_meas["dg-aa"]
            and pos_meas["dgb"].get(1) == 144 and pos_meas["dgb"].get(2) == 144}


# ---------------------------------------------------------------- phase sums
def phase_sums(recs):
    out = {}
    for arm in sorted({r["arm"] for r in recs}):
        rs = [r for r in recs if r["arm"] == arm and not r["warmup"]]
        ph = Counter()
        on = Counter()
        sw = 0
        onw = 0
        rss = []
        for r in rs:
            for p in r.get("phases") or []:
                ph[p["name"]] += p.get("wall_ns") or 0
            o = r.get("online") or {}
            for k, v in (o.get("phases_ns") or {}).items():
                on[k] += v or 0
            onw += o.get("wall_ns") or 0
            sw += r["time"]["solve_wall_ns"] or 0
            rss.append(r["time"]["max_rss_kib"])
        out[arm] = {"runs": len(rs), "solve_wall_ms": sw / 1e6,
                    "phase_wall_ms": {k: v / 1e6 for k, v in sorted(ph.items())},
                    "online_wall_ms": onw / 1e6,
                    "online_phases_ms": {k: v / 1e6 for k, v in sorted(on.items())},
                    "oracle_setup_share_of_solve": ph.get("oracle_setup", 0) / sw if sw else None,
                    "max_rss_kib_mean": float(np.mean(rss)) if rss else None,
                    "max_rss_kib_max": max(rss) if rss else None}
    return out


# ---------------------------------------------------------------- band
def band(recs, ic_arms, rho_arm="rho-neg"):
    by_curve = defaultdict(lambda: defaultdict(list))
    for r in recs:
        if r["warmup"] or r["outcome"]["status"] != "verified":
            continue
        by_curve[r["workload"]["curve"]["slug"]][r["arm"]].append(r)
    out = {}
    for slug, arms in sorted(by_curve.items()):
        row = {}
        for arm, rs in sorted(arms.items()):
            row[arm] = {"runs": len(rs)}
            for w in WEIGHTS:
                vals = [priced_s(r, w) for r in rs]
                row[arm][str(w).rstrip("0").rstrip(".") if w else "0"] = (
                    sum(vals) / len(vals) if all(v is not None for v in vals) else None)
        dist = {}
        for arm in ic_arms:
            if arm in row and rho_arm in row:
                dist[arm] = {k: row[arm][k] / row[rho_arm][k] for k in ("0", "0.1", "1")}
                dist[arm]["order_holds_across_band"] = all(row[rho_arm][k] < row[arm][k] for k in ("0", "0.1", "1"))
        out[slug] = {"mean_S": row, "ic_over_rho": dist}
    # pooled
    pooled = defaultdict(dict)
    for arm in {a for v in by_curve.values() for a in v}:
        rs = [r for v in by_curve.values() for r in v.get(arm, [])]
        for w in WEIGHTS:
            vals = [priced_s(r, w) for r in rs]
            pooled[arm][str(w).rstrip("0").rstrip(".") if w else "0"] = sum(vals) / len(vals)
    return {"per_curve": out, "pooled_mean_S": dict(pooled)}


def unpriced_names(recs):
    c = Counter()
    for r in recs:
        c[(r["arm"], tuple(r["cost"].get("unpriced") or []))] += 1
    return {f"{a}: {','.join(n)}": v for (a, n), v in sorted(c.items())}


def pairwise_s_equal(recs, a, b):
    """Exact pairwise equality of priced S at the three weights, measured verified runs."""
    A = {(r["workload"]["workload_id"], r["round"]): r for r in recs if r["arm"] == a and not r["warmup"]}
    B = {(r["workload"]["workload_id"], r["round"]): r for r in recs if r["arm"] == b and not r["warmup"]}
    res = {}
    for w in WEIGHTS:
        ks = sorted(set(A) & set(B))
        diffs = sum(priced_s(A[k], w) != priced_s(B[k], w) for k in ks)
        sa = sum(priced_s(A[k], w) for k in ks)
        sb = sum(priced_s(B[k], w) for k in ks)
        res[WTAG[w]] = {"pairs": len(ks), "pairs_differing": diffs, "sum_ratio_b_over_a": sb / sa}
    return res


# ---------------------------------------------------------------- gate checks
def wall_gate(b, recs, committed):
    base = f"{S3}/P{b}-wall"
    g = {}
    je = jload(f"{base}/job-exit.json")
    g["1_exit"] = {"exit_code": je["exit_code"], "pod_uid": je["pod_uid"], "holds": je["exit_code"] == 0}
    au = jload(f"{base}/audit.json")
    reps = au.get("replays") or []
    measured = sum(1 for r in recs if not r["warmup"])
    g["2_audit"] = {"ok": au["ok"], "session_status": au["session_status"], "problems": len(au["problems"]),
                    "replays": len(reps), "replays_identical": sum(1 for x in reps if x.get("detail") == "identical" and x.get("reproduced")),
                    "measured_records": measured,
                    "holds": au["ok"] is True and au["session_status"] == "complete" and not au["problems"]
                    and len(reps) == measured and all(x.get("detail") == "identical" and x.get("reproduced") for x in reps)}
    # 3 counted A/A
    c3 = {}
    ok3 = True
    for (a, bb) in (("dg", "dg-aa"), ("dgb-edge", "dgb")):
        for w in WEIGHTS:
            f = jload(f"{EV}/P{b}/wall/analysis/compare-{a}__{bb}-{WTAG[w]}.json")
            v = f["ops"]["ratio_b_over_a"]
            c3[f"{a}/{bb} {WTAG[w]} (committed ops.ratio_b_over_a)"] = v
            ok3 &= v == 1.0
        rec = pairwise_s_equal(recs, a, bb)
        c3[f"{a}/{bb} recomputed"] = rec
        ok3 &= all(x["pairs_differing"] == 0 and x["pairs"] == 288 for x in rec.values())
    c3["holds"] = ok3
    g["3_counted_aa"] = c3
    # 4 spec and build
    frozen = jload(f"{FROZEN}/spec-P{b}.json")
    ins = jload(f"{base}/input-spec.json")
    sess_spec = jload(f"{base}/session/spec.json")
    bi = jload(f"{base}/build-info.json")
    bis = json.dumps(bi)
    g["4_spec_build"] = {"frozen_spec_sha256": sha(f"{FROZEN}/spec-P{b}.json"),
                         "input_spec_sha256": sha(f"{base}/input-spec.json"),
                         "input_spec_json_equal_frozen": ins == frozen,
                         "session_spec_json_equal_frozen": sess_spec == frozen,
                         "input_spec_bytes_equal_frozen": open(f"{base}/input-spec.json", "rb").read() == open(f"{FROZEN}/spec-P{b}.json", "rb").read(),
                         "build_info": bi,
                         "names_be2969ce": "be2969ce" in bis, "names_binary": BIN in bis,
                         "records_binary_sha256": sorted({r["binary_sha256"] for r in recs})}
    g["4_spec_build"]["holds"] = (ins == frozen and "be2969ce" in bis and BIN in bis
                                  and g["4_spec_build"]["records_binary_sha256"] == [BIN])
    # 5 gate
    l2 = jload(f"{base}/l2-gate.json")
    levels = Counter(r["isolation"]["level"] for r in recs)
    below = [{"seq": r["seq"], "arm": r["arm"], "warmup": r["warmup"], "level": r["isolation"]["level"],
              "round": r["round"], "curve": r["workload"]["curve"]["slug"], "target": r["workload"]["target_index"],
              "solve_wall_ns": r["time"]["solve_wall_ns"],
              "blockers_L2": [x for x in r["isolation"]["blockers"] if x.startswith("L2")],
              "schedstat_solve": r["time"].get("schedstat_solve"),
              "solve_cpu_sched": (r["time"].get("solve") or {}).get("cpu_sched"),
              "foreign_busy_ticks": r["isolation"].get("foreign_busy_ticks")}
             for r in recs if LEV[r["isolation"]["level"]] < 2]
    bad_outcomes = [r["seq"] for r in recs if r["outcome"]["status"] != "verified" or r["outcome"]["matches_target"] is not True]
    g["5_gate"] = {"accepted": l2["accepted"], "executions": l2["executions"], "records": len(recs),
                   "grades_committed": l2["grades"], "levels_recomputed": dict(levels),
                   "below_l2_committed": [(x["seq"], x["arm"]) for x in l2["below_l2"]],
                   "below_l2_recomputed": below, "invalid_outcomes": l2["invalid_outcomes"],
                   "non_verified_or_wrong_records": bad_outcomes,
                   "holds": l2["accepted"] is True and not below and not l2["invalid_outcomes"] and not bad_outcomes}
    # 6 native A/A admission
    cc_path = f"{base}/control-comparison.json"
    if os.path.exists(cc_path):
        cc = jload(cc_path)
        w = cc["wall"]
        c6 = {"present": True, "a": cc["a"]["arm"], "b": cc["b"]["arm"], "status": w["status"], "pairs": w["pairs"],
              "pairs_below_level": w["pairs_below_level"], "aa_pairs": w["aa_pairs"], "aa_ci95": w["aa_ci95"],
              "self_aa": w["self_aa"], "aa_arms": w["aa_arms"], "aa_selection": w["aa_selection"]}
        c6["holds"] = (cc["a"]["arm"] == "dgb-edge" and cc["b"]["arm"] == "dg-aa" and w["status"] == "admitted"
                       and w["pairs"] == 288 and w["pairs_below_level"] == 0 and w["aa_pairs"] == 288
                       and w["aa_ci95"] is not None and w["self_aa"] is False
                       and set(w["aa_arms"] or []) == {"dgb", "dgb-edge"} and w["aa_selection"] == "includes arm A")
    else:
        c6 = {"present": False, "holds": False}
    g["6_native_aa"] = c6
    # 7 decision block
    dec = {w: jload(f"{EV}/P{b}/wall/analysis/compare-dg__dgb-{WTAG[w]}.json") for w in WEIGHTS}
    w0 = dec[0.0]["wall"]
    c7 = {k: w0[k] for k in ("status", "pairs", "pairs_below_level", "self_aa", "aa_arms", "aa_selection", "aa_pairs", "corrected_ci95")}
    c7["wall_block_identical_w0.1"] = dec[0.1]["wall"] == w0
    c7["wall_block_identical_w1"] = dec[1.0]["wall"] == w0
    c7["holds"] = (w0["status"] == "admitted" and w0["pairs"] == 288 and w0["pairs_below_level"] == 0
                   and w0["self_aa"] is False and w0["aa_arms"] == ["dg", "dg-aa"]
                   and w0["aa_selection"] == "includes arm A" and w0["aa_pairs"] == 288
                   and w0["corrected_ci95"] is not None and c7["wall_block_identical_w0.1"] and c7["wall_block_identical_w1"])
    g["7_decision_block"] = c7
    # 8 identical charged work
    c8 = {"committed_ops_ratio": {WTAG[w]: dec[w]["ops"]["ratio_b_over_a"] for w in WEIGHTS},
          "recomputed_pairwise_S": pairwise_s_equal(recs, "dg", "dgb"),
          "record_by_record": cond8(recs)}
    c8["holds"] = (all(v == 1.0 for v in c8["committed_ops_ratio"].values())
                   and all(x["pairs_differing"] == 0 for x in c8["recomputed_pairwise_S"].values())
                   and all(v["charged_mismatches"] == 0 and not v["unmatched"] and v["records"] == 336
                           for v in c8["record_by_record"].values()))
    g["8_identical_work"] = c8
    # 9 identity
    f9 = jload(f"{EV}/P{b}/wall/analysis/compare-dg-aa__dgb-w0.json")["wall"]
    r9 = rel(f9["ratio_b_over_a"], w0["corrected_ratio"])
    g["9_identity"] = {"pairs": f9["pairs"], "dg-aa_dgb_ratio": f9["ratio_b_over_a"],
                       "decision_corrected_ratio": w0["corrected_ratio"], "rel_diff": r9,
                       "holds": f9["pairs"] == 288 and r9 is not None and r9 < 1e-9}
    g["A1_packed_layout"] = a1(recs)
    g["all_hold"] = all(v["holds"] for v in g.values() if isinstance(v, dict) and "holds" in v)
    g["failed"] = [k for k, v in g.items() if isinstance(v, dict) and "holds" in v and not v["holds"]]
    return g


def counted_gate(b, recs, wall_recs):
    base = f"{S3}/P{b}-counted"
    g = {}
    je = jload(f"{base}/job-exit.json")
    g["exit"] = {"exit_code": je["exit_code"], "pod_uid": je["pod_uid"]}
    au = jload(f"{base}/audit.json")
    reps = au.get("replays") or []
    measured = sum(1 for r in recs if not r["warmup"])
    g["v3_1_audit"] = {"ok": au["ok"], "session_status": au["session_status"], "problems": len(au["problems"]),
                       "replays": len(reps), "measured_records": measured,
                       "replays_identical": sum(1 for x in reps if x.get("detail") == "identical" and x.get("reproduced")),
                       "holds": au["ok"] is True and au["session_status"] == "complete" and not au["problems"]
                       and len(reps) == measured and all(x.get("detail") == "identical" and x.get("reproduced") for x in reps)}
    c2 = {}
    ok2 = True
    for w in WEIGHTS:
        f = jload(f"{EV}/P{b}/counted/analysis/compare-rho-neg__rho-neg-aa-{WTAG[w]}.json")
        c2[f"committed {WTAG[w]}"] = f["ops"]["ratio_b_over_a"]
        ok2 &= f["ops"]["ratio_b_over_a"] == 1.0
    rec = pairwise_s_equal(recs, "rho-neg", "rho-neg-aa")
    c2["recomputed"] = rec
    ok2 &= all(x["pairs_differing"] == 0 and x["pairs"] == 288 for x in rec.values())
    c2["holds"] = ok2
    g["v3_2_aa"] = c2
    frozen = jload(f"{FROZEN}/spec-P{b}-rho.json")
    ins = jload(f"{base}/input-spec.json")
    bi = jload(f"{base}/build-info.json")
    bis = json.dumps(bi)
    g["v3_3_spec_build"] = {"frozen_spec_sha256": sha(f"{FROZEN}/spec-P{b}-rho.json"),
                            "input_spec_sha256": sha(f"{base}/input-spec.json"),
                            "input_spec_json_equal_frozen": ins == frozen,
                            "session_spec_json_equal_frozen": jload(f"{base}/session/spec.json") == frozen,
                            "names_be2969ce": "be2969ce" in bis, "names_binary": BIN in bis,
                            "records_binary_sha256": sorted({r["binary_sha256"] for r in recs}),
                            "holds": ins == frozen and "be2969ce" in bis and BIN in bis}
    ic_below = [{"seq": r["seq"], "arm": r["arm"], "warmup": r["warmup"], "level": r["isolation"]["level"],
                 "solve_wall_ns": r["time"]["solve_wall_ns"],
                 "blockers_L2": [x for x in r["isolation"]["blockers"] if x.startswith("L2")]}
                for r in recs if r["method"]["id"] == "ic.pipeline" and LEV[r["isolation"]["level"]] < 2]
    g["v3_4_ic_executions"] = {"ic_executions": sum(1 for r in recs if r["method"]["id"] == "ic.pipeline"),
                               "below_l2": ic_below, "holds": not ic_below}
    rho = [r for r in recs if r["arm"] in ("rho-neg", "rho-neg-aa")]
    l1_ok, l1_bad, short = [], [], defaultdict(list)
    for r in rho:
        lv = LEV[r["isolation"]["level"]]
        if lv >= 2:
            continue
        l2b = [x for x in r["isolation"]["blockers"] if x.startswith("L2: ")]
        if lv == 1 and l2b and all(x.startswith("L2: solve too short to admit wall time") for x in l2b):
            l1_ok.append(r["seq"])
            short[r["arm"]].append(r["time"]["solve_wall_ns"])
        else:
            l1_bad.append({"seq": r["seq"], "arm": r["arm"], "level": r["isolation"]["level"], "blockers": l2b})
    g["v3_5_rho_executions"] = {"rho_executions": len(rho), "l1_short_solve": len(l1_ok), "violations": l1_bad,
                                "per_arm_short_count_and_solve_ns_range": {a: [len(v), min(v), max(v)] for a, v in short.items()},
                                "holds": not l1_bad}
    l2 = jload(f"{base}/l2-gate.json")
    bad_outcomes = [r["seq"] for r in recs if r["outcome"]["status"] != "verified" or r["outcome"]["matches_target"] is not True]
    g["v3_6_outcomes"] = {"invalid_outcomes": l2["invalid_outcomes"], "non_verified_or_wrong": bad_outcomes,
                          "holds": not l2["invalid_outcomes"] and not bad_outcomes}
    below_entries = [(x["seq"], x["arm"]) for x in l2["below_l2"]]
    l1_ok_set = set(l1_ok)
    clause = (je["exit_code"] == 42 and all(g[k]["holds"] for k in g if k.startswith("v3_"))
              and all(a in ("rho-neg", "rho-neg-aa") and s in l1_ok_set for s, a in below_entries))
    recomputed_below = sorted([r["seq"] for r in recs if LEV[r["isolation"]["level"]] < 2])
    g["exit42_clause"] = {"exit_code": je["exit_code"], "below_l2_committed_count": len(below_entries),
                          "below_l2_recomputed_count": len(recomputed_below),
                          "committed_equals_recomputed": sorted(s for s, _ in below_entries) == recomputed_below,
                          "below_l2_arms": dict(Counter(a for _, a in below_entries)),
                          "l2_gate_accepted": l2["accepted"],
                          "holds": bool(clause)}
    g["exit43_clause"] = {"applies": je["exit_code"] == 43}
    g["route_holds"] = (je["exit_code"] == 0 and all(g[k]["holds"] for k in g if k.startswith("v3_"))) or clause
    g["counted_equals_wall_dg_dgb"] = cross_session(wall_recs, recs)
    g["counted_equals_wall_holds"] = all(v["mismatches"] == 0 for v in g["counted_equals_wall_dg_dgb"].values())
    g["A1_counted_dgb"] = a1(recs, arms=("dgb",))
    g["all_hold"] = g["route_holds"] and g["counted_equals_wall_holds"] and g["A1_counted_dgb"]["holds"]
    return g


# ---------------------------------------------------------------- recompute vs committed
def recompute(b, recs):
    out = {}
    dec, dd = compare_wall(recs, "dg", "dgb", aa=("dg", "dg-aa"))
    out["decision_dg__dgb"] = dec
    com = jload(f"{EV}/P{b}/wall/analysis/compare-dg__dgb-w0.json")["wall"]
    cmpf = {}
    for k in ("ns_per_gae_a", "ns_per_gae_b", "ratio_b_over_a", "aa_ratio", "corrected_ratio", "total_wall_ratio"):
        cmpf[k] = {"committed": com[k], "recomputed": dec[k], "rel_diff": rel(com[k], dec[k])}
    for k in ("ci95", "aa_ci95", "corrected_ci95"):
        cmpf[k] = {"committed": com[k], "recomputed": dec[k],
                   "rel_diff": max(rel(x, y) for x, y in zip(com[k], dec[k]))}
    cmpf["corrected_dof"] = {"committed": com["corrected_dof"], "recomputed": dec["corrected_dof"],
                             "rel_diff": rel(com["corrected_dof"], dec["corrected_dof"])}
    cmpf["pairs"] = {"committed": com["pairs"], "recomputed": dec["pairs"]}
    cmpf["pairs_below_level"] = {"committed": com["pairs_below_level"], "recomputed": dec["pairs_below_level"]}
    cmpf["aa_pairs"] = {"committed": com["aa_pairs"], "recomputed": dec["aa_pairs"]}
    cmpf["max_rel_diff"] = max(v["rel_diff"] for v in cmpf.values() if "rel_diff" in v)
    out["decision_vs_committed"] = cmpf
    # descriptive comparisons
    desc = {}
    for (a, bb) in (("dg-aa", "dgb"), ("dg", "dgb-edge"), ("dgb-edge", "dgb"), ("dg", "dg-aa")):
        r, _ = compare_wall(recs, a, bb)
        c = jload(f"{EV}/P{b}/wall/analysis/compare-{a}__{bb}-w0.json")["wall"]
        r["committed_ratio_b_over_a"] = c["ratio_b_over_a"]
        r["committed_ci95"] = c["ci95"]
        r["rel_diff_ratio"] = rel(c["ratio_b_over_a"], r["ratio_b_over_a"])
        r["rel_diff_ci95"] = max(rel(x, y) for x, y in zip(c["ci95"], r["ci95"])) if c["ci95"] else None
        r["committed_aa_arms"] = c.get("aa_arms")
        r["committed_aa_selection"] = c.get("aa_selection")
        r["committed_self_aa"] = c.get("self_aa")
        r["committed_pairs"] = c["pairs"]
        desc[f"{a}__{bb}"] = r
    out["descriptive"] = desc
    out["identity_cond9_recomputed"] = {"dg-aa__dgb_ratio": desc["dg-aa__dgb"]["ratio_b_over_a"],
                                        "corrected_ratio": dec["corrected_ratio"],
                                        "rel_diff": rel(desc["dg-aa__dgb"]["ratio_b_over_a"], dec["corrected_ratio"])}
    # position control (§6.2)
    edge = desc["dg__dgb-edge"]
    ov = not (edge["ci95"][1] < dec["corrected_ci95"][0] or edge["ci95"][0] > dec["corrected_ci95"][1])
    out["position_control"] = {
        "edge_estimate_dgb-edge_over_dg": edge["ratio_b_over_a"], "edge_ci95": edge["ci95"],
        "corrected_middle": dec["corrected_ratio"], "corrected_ci95": dec["corrected_ci95"],
        "intervals_overlap": ov,
        "gap_if_disjoint": None if ov else (dec["corrected_ci95"][0] - edge["ci95"][1] if edge["ci95"][1] < dec["corrected_ci95"][0] else edge["ci95"][0] - dec["corrected_ci95"][1]),
        "middle_over_edge_dg": {"aa_ratio_dg-aa_over_dg": dec["aa_ratio"], "aa_ci95": dec["aa_ci95"]},
        "middle_over_edge_dgb": {"dgb_over_dgb-edge": desc["dgb-edge__dgb"]["ratio_b_over_a"], "ci95": desc["dgb-edge__dgb"]["ci95"]},
        "positions": "edge arms dg and dgb-edge at block positions {0,3}; middle arms dg-aa and dgb at {1,2}"}
    # per-curve corrected
    pc = {}
    da = wall_pairs(recs, "dg", "dg-aa")
    for slug in sorted({p["curve"] for p in dd["admitted"]}):
        f1 = fig([p for p in dd["admitted"] if p["curve"] == slug])
        f2 = fig([p for p in da["admitted"] if p["curve"] == slug])
        c = st.divide_independent(f1, f2)
        e = fig([p for p in wall_pairs(recs, "dg", "dgb-edge")["admitted"] if p["curve"] == slug])
        pc[slug] = {"pairs": f1["n"], "aa_pairs": f2["n"], "ratio_b_over_a": f1["ratio"], "ci95": f1["ci95"],
                    "aa_ratio": f2["ratio"], "corrected_ratio": c["ratio"], "corrected_ci95": c["ci95"],
                    "edge_dgb-edge_over_dg": e["ratio"], "edge_ci95": e["ci95"],
                    "ns_per_gae_dg": f1["ns_per_gae_a"], "ns_per_gae_dgb": f1["ns_per_gae_b"]}
    out["per_curve"] = pc
    # target-clustered: linearised (clusters = 48 targets) and stratified bootstrap
    def unit_sums(pairs, key):
        s = defaultdict(lambda: [0.0, 0.0, 0.0, 0.0])
        for p in pairs:
            t = s[key(p)]
            for i, v in enumerate(tup(p)):
                t[i] += v
        return s
    tk = lambda p: (p["curve"], p["target"])  # noqa: E731
    pk = lambda p: (p["curve"], p["target"], p["round"])  # noqa: E731
    clusters = {}
    for name, key in (("target_clusters", tk), ("pairs", pk)):
        sd = unit_sums(dd["admitted"], key)
        sa = unit_sums(da["admitted"], key)
        units = [{"dec": sd.get(k, [0.0] * 4), "aa": sa.get(k, [0.0] * 4)} for k in sorted(set(sd) | set(sa))]
        clusters[name] = st.joint_corrected(units)
    # target-clustered independent combination (tool's method, clusters = targets)
    sd = unit_sums(dd["admitted"], tk)
    sa = unit_sums(da["admitted"], tk)
    f1 = st.per_op_ratio([tuple(v) for _, v in sorted(sd.items())])
    f2 = st.per_op_ratio([tuple(v) for _, v in sorted(sa.items())])
    clusters["target_clusters_independent_combination"] = st.divide_independent(f1, f2)
    clusters["target_clusters_independent_combination"]["n_clusters"] = f1["n"]
    strata = []
    for slug in sorted({k[0] for k in sd}):
        ks = sorted(k for k in set(sd) | set(sa) if k[0] == slug)
        strata.append(np.array([sd.get(k, [0.0] * 4) + sa.get(k, [0.0] * 4) for k in ks]))
    clusters["stratified_target_bootstrap"] = st.cluster_bootstrap(strata, 20000, 2026102900 + b)
    out["clustered"] = clusters
    return out


def band_vs_committed(b, bc):
    out = {}
    worst = 0.0
    for arm in ("dg", "dgb", "rho-neg-aa"):
        for w in WEIGHTS:
            wk = "0" if w == 0 else str(w).rstrip("0").rstrip(".")
            f = jload(f"{EV}/P{b}/counted/analysis/compare-rho-neg__{arm}-{WTAG[w]}.json")
            mine = bc["pooled_mean_S"][arm][wk] / bc["pooled_mean_S"]["rho-neg"][wk]
            e = {"committed_ops_ratio": f["ops"]["ratio_b_over_a"], "recomputed": mine,
                 "rel_diff": rel(f["ops"]["ratio_b_over_a"], mine), "per_curve": {}}
            worst = max(worst, e["rel_diff"])
            for c in f["curves"]:
                m = bc["per_curve"][c["slug"]]["mean_S"]
                mc = m[arm][wk] / m["rho-neg"][wk]
                e["per_curve"][c["slug"]] = {"committed": c["ratio_b_over_a"], "recomputed": mc,
                                             "rel_diff": rel(c["ratio_b_over_a"], mc)}
                worst = max(worst, e["per_curve"][c["slug"]]["rel_diff"])
            out[f"rho-neg__{arm} {WTAG[w]}"] = e
    out["max_rel_diff"] = worst
    return out


def phase_vs_committed(b, ps):
    f = jload(f"{EV}/P{b}/wall/analysis/phase_sums.json")["by_arm"]
    worst = 0.0
    n = 0
    for arm, d in f.items():
        mine = ps[arm]
        for k, v in d.items():
            if k.startswith("phases.") and k.endswith(".wall_ns"):
                m = mine["phase_wall_ms"][k.split(".")[1]] * 1e6
            elif k.startswith("online.phases_ns."):
                m = mine["online_phases_ms"][k.split(".")[2]] * 1e6
            elif k == "online.wall_ns":
                m = mine["online_wall_ms"] * 1e6
            elif k == "time.solve_wall_ns":
                m = mine["solve_wall_ms"] * 1e6
            elif k == "n_records":
                m = mine["runs"]
            else:
                continue
            n += 1
            worst = max(worst, rel(float(v), m))
    return {"fields_compared": n, "max_rel_diff": worst}


SNAP = "bf899979a6a6304fbbfbbc27204e50acfc52f6b8"
WT = "/dev/shm/wt-blk"
EVREL = "research/ic_dg_blocked_20261029"


def git(*a):
    return subprocess.run(["git", "-C", WT, *a], check=True, capture_output=True, text=True).stdout


def snapshot_binding():
    """Bind every evidence file to the snapshot: git-tracked at SNAP, or listed in an
    artifacts.manifest/v1 .artifacts.json committed at SNAP with the same sha256 and
    size (artifact-store files hydrated by tools/artifacts/artifacts.py pull)."""
    tracked = set(git("ls-tree", "-r", "--name-only", SNAP, EVREL + "/").split())
    manifests = sorted(x for x in tracked if x.endswith("/.artifacts.json"))
    bound = {}
    bad = []
    for m in manifests:
        man = json.loads(git("show", f"{SNAP}:{m}"))
        assert man["schema"] == "artifacts.manifest/v1"
        base = os.path.dirname(m)
        for fn, ent in man["files"].items():
            rel = f"{base}/{fn}"
            path = f"{WT}/{rel}"
            ok = os.path.isfile(path) and os.path.getsize(path) == ent["bytes"] and sha(path) == ent["sha256"]
            bound[rel[len(EVREL) + 1:]] = {"manifest": m, "sha256": ent["sha256"], "bytes": ent["bytes"], "hydrated_matches": ok,
                                           "also_git_tracked": rel in tracked}
            if not ok:
                bad.append(rel)
    # tracked eks-results files: working-tree copy equals the blob at SNAP
    blob_mismatch = []
    for rel in sorted(x for x in tracked if "/eks-results/" in x and not x.endswith("/.artifacts.json") and not x.endswith("/.gitignore")):
        if git("hash-object", f"{WT}/{rel}").strip() != git("rev-parse", f"{SNAP}:{rel}").strip():
            blob_mismatch.append(rel)
    return {"snapshot": SNAP, "manifests": manifests, "manifest_files": bound, "manifest_mismatches": bad,
            "tracked_eks_results_blob_mismatches": blob_mismatch,
            "tracked": sorted(x[len(EVREL) + 1:] for x in tracked)}


def pod_exit(pod):
    cs = pod["status"].get("containerStatuses", [])
    return cs[0]["state"]["terminated"]["exitCode"] if len(cs) == 1 and "terminated" in cs[0]["state"] else None


def provenance():
    out = {}
    for b in SIZES:
        for s in ("wall", "counted"):
            ev = f"{EV}/P{b}/{s}/eks-results"
            s3 = f"{S3}/P{b}-{s}"
            files = []
            for root, _, fs in os.walk(s3):
                for fn in fs:
                    files.append(os.path.relpath(os.path.join(root, fn), s3))
            same = all(os.path.exists(f"{ev}/{x}") and sha(f"{ev}/{x}") == sha(f"{s3}/{x}") for x in files)
            sums = {}
            for line in open(f"{s3}/SHA256SUMS"):
                h, fn = line.split()
                sums[fn] = h
            sums_ok = all(sha(f"{s3}/{fn}") == h for fn, h in sums.items())
            out[f"P{b}-{s}"] = {"s3_files": len(files), "evidence_equals_s3": same,
                                "sha256sums_entries": len(sums), "sha256sums_verify": sums_ok,
                                "records_jsonl_sha256": sha(f"{s3}/session/records.jsonl"),
                                "pod_uid": jload(f"{s3}/job-exit.json")["pod_uid"]}
            # launch receipts bind the frozen spec.  Read from the durable S3 launcher
            # receipts (copied read-only to /dev/shm/dgb-val/s3-launcher): the
            # evidence directory's launch/ folders are untracked at bf899979a6
            # (working-tree only) and are not used as evidence.
            ld = f"{S3L}/P{b}-{s}"
            rq = jload(f"{ld}/request.json")
            res = jload(f"{ld}/result.json")
            jx = jload(f"{s3}/job-exit.json")
            st = jload(f"{ld}/status.json")
            wt = f"{EV}/P{b}/{s}/launch/receipts"
            fz = jload(f"{FROZEN}/spec-P{b}.json" if s == "wall" else f"{FROZEN}/spec-P{b}-rho.json")
            out[f"P{b}-{s}"].update({
                "receipt_source": "s3://isolab-results-104186811093-us-east-2/runs/" + open(f"{ld}/PREFIX").read().strip() + "/",
                "receipt_request_sha256": sha(f"{ld}/request.json"),
                "receipt_result_sha256": sha(f"{ld}/result.json"),
                "receipt_status_sha256": sha(f"{ld}/status.json"),
                "receipt_submission_sha256": sha(f"{ld}/submission.json"),
                "status_job_uid": st["job"]["uid"], "status_pod_name": st["pods"][0]["name"] if st["pods"] else None,
                "original_request_equals_request": sha(f"{ld}/original-request.json") == sha(f"{ld}/request.json"),
                "receipt_request_spec_equals_frozen": rq["job"]["spec"] == fz,
                "receipt_spec_equals_session_spec": rq["job"]["spec"] == jload(f"{s3}/session/spec.json"),
                "result_request_sha256_matches": sha(f"{ld}/request.json") == res["request_sha256"],
                # On ecbench exit 42 the container errors, the Job fails (no retry) and the
                # launcher writes state 'failed', exit 23, with an empty pod_uid and
                # results_uri (v3 PROTOCOL: "exit of 42 (launcher state failed, exit 23)");
                # the pod is then bound through status.json.
                "result_job_name_equals_job_exit": res["job_name"] == jx.get("job_name"),
                "result_job_uid_equals_status": res["job_uid"] == st["job"]["uid"],
                "status_single_pod": len(st["pods"]) == 1 and st["job"]["status"].get("failed", 0) + st["job"]["status"].get("succeeded", 0) == 1,
                "status_pod_uid_equals_job_exit": len(st["pods"]) == 1 and st["pods"][0]["uid"] == jx["pod_uid"],
                "status_node_equals_job_exit": len(st["pods"]) == 1 and st["pods"][0]["node"] == jx["node_name"],
                "status_container_exit_equals_job_exit": len(st["pods"]) == 1 and pod_exit(st["pods"][0]) == jx["exit_code"],
                "status_restart_count_zero": len(st["pods"]) == 1 and all(c.get("restartCount", 0) == 0 for c in st["pods"][0]["status"].get("containerStatuses", [])),
                "launcher_state_consistent_with_exit": ((res["state"], res["exit_code"]) == ("succeeded", 0) and jx["exit_code"] == 0 and res["pod_uid"] == jx["pod_uid"] and jx["pod_uid"] in res["results_uri"])
                                                       or ((res["state"], res["exit_code"]) == ("failed", 23) and jx["exit_code"] == 42),
                "container_window": [st["pods"][0]["status"]["containerStatuses"][0]["state"]["terminated"]["startedAt"],
                                     st["pods"][0]["status"]["containerStatuses"][0]["state"]["terminated"]["finishedAt"]],
                "job_exit_code": jx["exit_code"],
                "working_tree_launch_copy_equals_s3": (os.path.exists(f"{wt}/request.json") and sha(f"{wt}/request.json") == sha(f"{ld}/request.json")
                                                       and sha(f"{wt}/result.json") == sha(f"{ld}/result.json")),
                "result_state": res["state"], "result_exit_code": res["exit_code"],
                "result_build": res["build"], "instance_type": rq["job"]["instance_type"],
                "deadline_s": rq["job"]["seconds"], "harness": rq["provenance"]["harness"],
                "provenance_run_url": rq["provenance"].get("run_url")})
    return out


def main():
    os.makedirs(OUT, exist_ok=True)
    worst_t = st.self_test()
    res = {"t975_self_test_max_abs_err": worst_t, "sizes": {}}
    for b in SIZES:
        wall = records(b, "wall")
        counted = records(b, "counted")
        size = {}
        size["records"] = {"wall": len(wall), "counted": len(counted),
                           "wall_arms": dict(Counter(r["arm"] for r in wall)),
                           "counted_arms": dict(Counter(r["arm"] for r in counted)),
                           "wall_unique_keys": len({(r["arm"], r["workload"]["workload_id"], r["round"]) for r in wall}),
                           "counted_unique_keys": len({(r["arm"], r["workload"]["workload_id"], r["round"]) for r in counted}),
                           "wall_seq_contiguous": sorted(r["seq"] for r in wall) == list(range(len(wall))),
                           "counted_seq_contiguous": sorted(r["seq"] for r in counted) == list(range(len(counted))),
                           "pinned_calibration_records": sum(1 for r in wall + counted if pinned(r)),
                           "unpriced_names": {"wall": unpriced_names(wall), "counted": unpriced_names(counted)},
                           "fb_size_params": sorted({r["method"]["params"].get("factor_base") for r in wall + counted if r["method"]["id"] == "ic.pipeline"}),
                           "oracles_by_arm": {a: sorted({r["method"]["params"].get("oracle") for r in wall + counted if r["arm"] == a}) for a in sorted({r["arm"] for r in wall + counted})},
                           "targets_by_arm": {a: sorted({r["method"]["params"].get("targets", "") for r in wall + counted if r["arm"] == a}) for a in sorted({r["arm"] for r in wall + counted})}}
        size["wall_gate"] = wall_gate(b, wall, None)
        size["counted_gate"] = counted_gate(b, counted, wall)
        size["recompute"] = recompute(b, wall)
        size["positions"] = positions(wall)
        size["phase_sums_wall"] = phase_sums(wall)
        size["band_counted"] = band(counted, ("dg", "dgb"))
        size["band_wall_ic"] = band(wall, ())["pooled_mean_S"]
        by_slug = defaultdict(list)
        for r in wall:
            by_slug[r["workload"]["curve"]["slug"]].append(r)
        size["curves"] = curve_facts(by_slug)
        size["automorphisms_used"] = {f"{a} (counted)": dict(Counter((r.get("boundaries") or {}).get("automorphisms_used") for r in counted if r["arm"] == a)) for a in sorted({r["arm"] for r in counted})}
        size["band_vs_committed"] = band_vs_committed(b, size["band_counted"])
        size["phase_sums_vs_committed"] = phase_vs_committed(b, size["phase_sums_wall"])
        res["sizes"][f"P{b}"] = size
        print(f"P{b}: wall gate all_hold={size['wall_gate']['all_hold']} failed={size['wall_gate']['failed']}; "
              f"counted all_hold={size['counted_gate']['all_hold']}; "
              f"corrected={size['recompute']['decision_dg__dgb']['corrected_ratio']:.6f} "
              f"{[round(x, 6) for x in size['recompute']['decision_dg__dgb']['corrected_ci95']]}; "
              f"max rel diff vs committed {size['recompute']['decision_vs_committed']['max_rel_diff']:.2e}", flush=True)
    res["provenance"] = provenance()
    res["snapshot_binding"] = snapshot_binding()
    with open(f"{OUT}/recompute.json", "w") as f:
        json.dump(res, f, indent=1, sort_keys=True, default=str)


if __name__ == "__main__":
    main()
