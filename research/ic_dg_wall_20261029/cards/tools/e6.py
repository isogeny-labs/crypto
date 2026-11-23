"""Per-curve conditions 2, 3 and 4 of the 28-bit consequence (PROTOCOL.md section 6.2 as edited by APPROVAL.md E6), and condition 1 from wall_gate.json.
Reads files; compares only. It decides nothing and exits 0 whatever the values are.
usage: e6.py KIND EKS_DIR ANALYSIS_DIR B
KIND is wall (the session named by section 6.2) or counted (the same three per-curve conditions on the counted session, descriptive).
For B other than 28 it prints E6_NOT_APPLICABLE and exits 0.
Needs compare-srenew__dg-w{0,0.1,1}.json in ANALYSIS_DIR. Exits 1 with a line beginning STOP: only when a needed file is absent or unreadable."""
import collections, json, math, os, sys

kind, eks, an, b = sys.argv[1:5]
if kind not in ("wall", "counted"):
    print("STOP: KIND is not wall or counted")
    sys.exit(1)
if b != "28":
    print("E6_NOT_APPLICABLE b=" + b + " kind=" + kind)
    print("E6_OK")
    sys.exit(0)


def stop(msg):
    print("STOP: " + msg)
    sys.exit(1)


def jload(p):
    try:
        with open(p) as f:
            return json.load(f)
    except Exception as e:
        stop("cannot read JSON %s: %s" % (p, type(e).__name__))


def tf(x):
    return "true" if x else "false"


cmp = {w: jload(os.path.join(an, "compare-srenew__dg-w%s.json" % w)) for w in ("0", "0.1", "1")}
try:
    recs = [json.loads(l) for l in open(os.path.join(eks, "session", "records.jsonl")) if l.strip()]
except Exception as e:
    stop("cannot read records.jsonl: " + type(e).__name__)

NAME = "target_guard_probes_uncharged"
BASE = NAME[: -len("_uncharged")]


def guard_probes(r):
    """The count behind target_guard_probes_uncharged, by the rule of band.rs unpriced_counts: the top-level counters, else the sum over phases of native[NAME] or native[BASE]."""
    c = (r.get("counters") or {})
    if NAME in c:
        return c[NAME]
    total = 0
    found = False
    for p in r.get("phases") or []:
        nat = p.get("native") or {}
        v = nat.get(NAME)
        if v is None:
            v = nat.get(BASE)
        if v is not None:
            total += v
            found = True
    return total if found else None


S = collections.defaultdict(list)       # (arm, slug) -> [cost.s]
G = collections.defaultdict(list)       # (arm, slug) -> [total_gae]
P = collections.defaultdict(list)       # (arm, slug) -> [guard probes]
missing_guard = 0
for r in recs:
    arm = r.get("arm")
    if arm not in ("srenew", "dg"):
        continue
    if r.get("warmup") or (r.get("outcome") or {}).get("status") != "verified":
        continue
    slug = ((r.get("workload") or {}).get("curve") or {}).get("slug")
    S[(arm, slug)].append((r.get("cost") or {}).get("s"))
    G[(arm, slug)].append((r.get("cost") or {}).get("total_gae"))
    g = guard_probes(r)
    if g is None:
        missing_guard += 1
    else:
        P[(arm, slug)].append(g)

slugs = sorted({s for (_, s) in S})
rows0 = {row["slug"]: row for row in (cmp["0"].get("curves") or [])}
rows01 = {row["slug"]: row for row in (cmp["0.1"].get("curves") or [])}
rows1 = {row["slug"]: row for row in (cmp["1"].get("curves") or [])}
print("E6_KIND " + kind)
print("E6_CURVE_SLUGS_FROM_RECORDS " + json.dumps(slugs))
print("E6_CURVE_SLUGS_FROM_W0_COMPARE " + json.dumps(sorted(rows0)))
print("E6_RECORDS_WITHOUT_GUARD_COUNT " + str(missing_guard))
c2_all = True
c3_all = True
c4_all = True
c2_all_row = True
c2_all_recomputed = True
tot_s = {"srenew": [], "dg": []}
for slug in slugs:
    row0, row01, row1 = rows0.get(slug), rows01.get(slug), rows1.get(slug)
    r0 = (row0 or {}).get("ratio_b_over_a")
    ci01 = (row01 or {}).get("ci95")
    ci1 = (row1 or {}).get("ci95")
    sd, ss = math.fsum(S[("dg", slug)]), math.fsum(S[("srenew", slug)])
    gd, gs = math.fsum(G[("dg", slug)]), math.fsum(G[("srenew", slug)])
    pd, ps = sum(P[("dg", slug)]), sum(P[("srenew", slug)])
    tot_s["dg"] += S[("dg", slug)]
    tot_s["srenew"] += S[("srenew", slug)]
    row_le1 = (r0 is not None) and r0 <= 1
    recomp_le1 = sd <= ss
    c2 = row_le1 and recomp_le1
    c3 = (ci01 is not None and ci01[1] < 1) and (ci1 is not None and ci1[1] < 1)
    c4 = pd < ps
    c2_all_row = c2_all_row and row_le1
    c2_all_recomputed = c2_all_recomputed and recomp_le1
    c2_all = c2_all and c2
    c3_all = c3_all and c3
    c4_all = c4_all and c4
    print("E6_CURVE " + json.dumps({
        "slug": slug,
        "n_measured_verified_dg": len(S[("dg", slug)]),
        "n_measured_verified_srenew": len(S[("srenew", slug)]),
        "cond2_w0_curve_row_ratio_b_over_a_repr": repr(r0),
        "cond2_w0_curve_row_pairs": (row0 or {}).get("pairs"),
        "cond2_row_ratio_le_1": row_le1,
        "cond2_recomputed_sum_S_dg_repr": repr(sd),
        "cond2_recomputed_sum_S_srenew_repr": repr(ss),
        "cond2_recomputed_sum_total_gae_dg_repr": repr(gd),
        "cond2_recomputed_sum_total_gae_srenew_repr": repr(gs),
        "cond2_sum_S_dg_le_sum_S_srenew": recomp_le1,
        "cond3_w0.1_curve_row_ci95": ci01,
        "cond3_w1_curve_row_ci95": ci1,
        "cond3_both_upper_bounds_lt_1": c3,
        "cond4_guard_probes_sum_dg": pd,
        "cond4_guard_probes_sum_srenew": ps,
        "cond4_dg_lt_srenew": c4,
    }, sort_keys=True))
sd_all, ss_all = math.fsum(tot_s["dg"]), math.fsum(tot_s["srenew"])
print("E6_POOLED " + json.dumps({
    "recomputed_sum_S_dg_over_sum_S_srenew_repr": repr(sd_all / ss_all) if ss_all else None,
    "compare_w0_ops_ratio_b_over_a_repr": repr((cmp["0"].get("ops") or {}).get("ratio_b_over_a")),
    "recomputed_pooled_ratio_equals_compare_w0_ops_ratio": (ss_all != 0 and sd_all / ss_all == (cmp["0"].get("ops") or {}).get("ratio_b_over_a")),
    "guard_probes_pooled_dg_from_records": sum(sum(P[("dg", s)]) for s in slugs),
    "guard_probes_pooled_srenew_from_records": sum(sum(P[("srenew", s)]) for s in slugs),
    "guard_probes_pooled_dg_in_compare_w0_json_b": ((cmp["0"].get("b") or {}).get("unpriced_counts") or {}).get(NAME),
    "guard_probes_pooled_srenew_in_compare_w0_json_a": ((cmp["0"].get("a") or {}).get("unpriced_counts") or {}).get(NAME),
}, sort_keys=True))
print("E6_COND2_ALL_CURVES=" + tf(c2_all))
print("E6_COND2_ALL_CURVES_ROW_RATIO_ONLY=" + tf(c2_all_row))
print("E6_COND2_ALL_CURVES_RECOMPUTED_ONLY=" + tf(c2_all_recomputed))
print("E6_COND3_ALL_CURVES=" + tf(c3_all))
print("E6_COND4_ALL_CURVES=" + tf(c4_all))
if kind == "wall":
    wg = jload(os.path.join(an, "wall_gate.json")).get("lines") or {}
    c1 = bool(wg.get("W_ALL_1_TO_8")) and bool(wg.get("CORRECTED_CI95_UPPER_LT_1"))
    print("E6_COND1_W_ALL_1_TO_8=" + tf(wg.get("W_ALL_1_TO_8")))
    print("E6_COND1_CORRECTED_CI95_UPPER_LT_1=" + tf(wg.get("CORRECTED_CI95_UPPER_LT_1")))
    print("E6_COND1=" + tf(c1))
    print("E6_ALL_FOUR=" + tf(c1 and c2_all and c3_all and c4_all))
else:
    print("E6_COND1_NOT_EVALUATED (counted session carries no wall claim)")
print("E6_OK")
