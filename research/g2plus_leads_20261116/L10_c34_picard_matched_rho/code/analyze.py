#!/usr/bin/env python3
"""L10 analysis (stdlib only). Emits verdict strings mechanically per PROTOCOL section 7."""
import argparse, json, math, os, random, statistics

ap = argparse.ArgumentParser()
ap.add_argument("--raw", required=True)
ap.add_argument("--bound-r", required=True)
ap.add_argument("--out", required=True)
ap.add_argument("--tables", required=True)
a = ap.parse_args()


def load(name):
    path = os.path.join(a.raw, name)
    if not os.path.exists(path):
        return []
    return [json.loads(l) for l in open(path) if l.strip().startswith("{")]


def geomean(xs):
    return math.exp(sum(math.log(x) for x in xs) / len(xs))


tiny = load("tiny.jsonl")
rhoc = load("rhocost.jsonl")
meas = load("measure.jsonl")
pc = load("pc.jsonl")
replay_txt = open(os.path.join(a.raw, "replay_cmp.txt")).read() if os.path.exists(os.path.join(a.raw, "replay_cmp.txt")) else ""

# ------------------------------------------------------------------ gates
gates = {}
fails = {}


def gate(name, ok, failing):
    gates[name] = bool(ok)
    fails[name] = failing


tp = [d for d in tiny if d.get("family") == "P" and "T1" in d]
tother = [d for d in tiny if d.get("family") in ("C", "Q")]
key = lambda d: "%s/p%s/s%s" % (d.get("family"), d.get("p"), d.get("curve_seed"))
for g in ["T1", "T2", "T3", "T4", "T5", "T6", "T7a", "T8"]:
    bad = [key(d) for d in tp if d[g] is not True]
    gate(g, not bad and len(tp) > 0, bad)
bad = [key(d) for d in tother if d["T6"] is not True]
gate("T6_CQ", not bad and len(tother) > 0, bad)
ndet = sum(1 for d in tp if d["T7b"] == "determined")
nfail = [key(d) for d in tp if d["T7b"] == "fail"]
gate("T7b", ndet >= 8 and not nfail, nfail + ["determined=%d of %d" % (ndet, len(tp))])
ndet_orb = sum(1 for d in tp if d.get("T7b_orbit") == 2)
bad = [key(d) for d in tp if d["T7c"] is False]
gate("T7c", not bad, bad)
allm = meas + pc
mp = [d for d in allm if d["family"] == "P"]
gate("M1", all(d["M1"] is True for d in mp) and len(mp) > 0, [key(d) + "/" + d["arm"] for d in mp if d["M1"] is not True])
gate("M2", all(d["M2_verified"] == min(50, d["accepted"]) for d in allm) and len(allm) > 0,
     [key(d) + "/" + d["arm"] for d in allm if d["M2_verified"] != min(50, d["accepted"])])
for g in ["M3", "M4", "M5"]:
    gate(g, all(d[g] is True for d in allm) and len(allm) > 0, [key(d) + "/" + d["arm"] + "/p%d" % d["p"] for d in allm if d[g] is not True])
gate("R1", all(d["R1"] for d in rhoc) and len(rhoc) > 0, [key(d) for d in rhoc if not d["R1"]])
gate("R_T5", all(d["T5"] is True for d in rhoc if d["family"] == "P") and any(d["family"] == "P" for d in rhoc),
     [key(d) for d in rhoc if d["family"] == "P" and d["T5"] is not True])
gate("replay", "exit=0" in replay_txt, [replay_txt.strip()] if "exit=0" not in replay_txt else [])
# per-row ok flags
gate("measure_row_ok", all(d["ok"] for d in allm) and len(allm) > 0, [key(d) + "/" + d["arm"] for d in allm if not d["ok"]])

# ------------------------------------------------------------------ Part 1
def pooled(family, p, op):
    rows = [d for d in rhoc if d["family"] == family and d["p"] == p]
    if not rows:
        return None
    if op == "R":
        n = sum(d["n_rho"] for d in rows)
        if n == 0:
            return None
        return sum(d["rho_mul_lab"] + d["rho_mul_own"] + 10 * d["rho_inv_lab"] for d in rows) / n
    k = {"A": "add", "N": "neg", "D": "dbl"}[op]
    n = sum(d["n_" + k] for d in rows)
    return sum(d[k + "_mul"] + 10 * d[k + "_inv"] for d in rows) / n


def pooled_mul(family, p, k):
    rows = [d for d in rhoc if d["family"] == family and d["p"] == p]
    return sum(d[k + "_mul"] for d in rows) / sum(d["n_" + k] for d in rows)


rho_primes = [2053, 4099, 8209]
part1 = {"per_op": {}, "Q": {}, "P": {}, "R2": {}}
bound = [json.loads(l) for l in open(a.bound_r) if l.strip().startswith("{")]
for fam in ["Q", "C", "P"]:
    for p in rho_primes:
        part1["per_op"]["%s/%d" % (fam, p)] = {op: pooled(fam, p, op) for op in ["A", "N", "D", "R"]}
r2_ok = True
for p in rho_primes:
    A, N = pooled("Q", p, "A"), pooled("Q", p, "N")
    FQ = (A + N) / (math.sqrt(2) * A)
    part1["Q"][p] = {"A": A, "N": N, "F_Q": FQ}
    ratios = sorted(d["cantor_mul"] / d["group_ops"] for d in bound if d.get("arm") == "Q_rho_neg" and d.get("ok") is True and d.get("p") == p)
    med = statistics.median(ratios) if ratios else None
    mine = pooled_mul("Q", p, "add") + pooled_mul("Q", p, "neg")
    ok = med is not None and abs(mine / med - 1) <= 0.05
    r2_ok &= ok
    part1["R2"][p] = {"bound_r_median_mul_per_op": med, "n_bound_rows": len(ratios), "A_mul_plus_N_mul": mine, "rel_diff": (mine / med - 1) if med else None, "pass": ok}
gate("R2", r2_ok, [str(p) for p in rho_primes if not part1["R2"][p]["pass"]])
fqs = [part1["Q"][p]["F_Q"] for p in rho_primes]
if not (gates["R1"] and gates["R2"]):
    h1 = "gate failed"
elif all(f >= 1.10 for f in fqs):
    h1 = "supported"
elif all(0.90 < f < 1.10 for f in fqs):
    h1 = "no material move"
elif all(f <= 0.90 for f in fqs):
    h1 = "negation walk legitimate"
else:
    h1 = "inconclusive"
AFFECTED = ["research/genus_ladder_20261114/RESULTS.md:60, 73, 95-99, 165",
            "research/genus_ladder_bound_r_20261114/RESULTS.md:29-35, 41-46"]
part1["H1"] = {"verdict": h1, "F_Q": dict(zip(rho_primes, fqs)),
               "affected_rows_if_supported": AFFECTED,
               "ghs_weak_rows": "unaffected (elliptic reference)",
               "note": "R2 failed: per-operation costs only, model not reconciled" if not gates["R2"] else ""}
S = {}
for p in rho_primes:
    A, N, R = pooled("P", p, "A"), pooled("P", p, "N"), pooled("P", p, "R")
    s1 = math.sqrt(2) * A
    s2 = A + N
    s3 = (A + 2 * R) * math.sqrt(2.0 / 3.0)
    s6 = (A + N + 4 * R) / math.sqrt(3)
    d = {"S1": s1, "S2": s2, "S3": s3, "S6": s6}
    am = min(d, key=d.get)
    S[p] = {"A": A, "N": N, "R": R, **d, "argmin": am, "min_over_S2": d[am] / s2}
part1["P"] = S

# ------------------------------------------------------------------ Part 2
def pair_rows(p):
    full = {(d["curve_seed"], d["solver_seed"]): d for d in meas if d["p"] == p and d["arm"] == "P_full"}
    orb = {(d["curve_seed"], d["solver_seed"]): d for d in meas if d["p"] == p and d["arm"] == "P_orbit"}
    ks = sorted(set(full) & set(orb))
    return [(full[k], orb[k]) for k in ks]


def boot(xs):
    rng = random.Random(12345)
    n = len(xs)
    vals = []
    for _ in range(2000):
        vals.append(geomean([xs[rng.randrange(n)] for _ in range(n)]))
    vals.sort()
    return vals[int(0.025 * 2000)], vals[int(0.975 * 2000) - 1]


mprimes = sorted(set(d["p"] for d in meas))
part2 = {}
for p in mprimes:
    pr = pair_rows(p)
    if not pr:
        continue
    out = {"n_pairs": len(pr)}
    for m in ["fpm_total", "trials", "fpm_la_w", "fpm_line", "fpm_enum"]:
        xs = [f[m] / o[m] for f, o in pr]
        out[m] = {"geomean": geomean(xs), "min": min(xs), "max": max(xs)}
    xs = [f["fpm_total"] / o["fpm_total"] for f, o in pr]
    out["fpm_total"]["boot95"] = list(boot(xs))
    out["fpm_la_unw_geomean"] = geomean([f["fpm_la_unw"] / o["fpm_la_unw"] for f, o in pr])
    part2[p] = out

# A/A
aa = {}
for p in mprimes:
    full = {(d["curve_seed"], d["solver_seed"]): d for d in meas if d["p"] == p and d["arm"] == "P_full"}
    rs = []
    for cs in (1, 2, 3):
        if (cs, 1) in full and (cs, 2) in full:
            rs.append(full[(cs, 1)]["fpm_total"] / full[(cs, 2)]["fpm_total"])
    if rs:
        g = geomean(rs)
        aa[p] = {"per_curve": rs, "geomean": g, "pass": 0.9 <= g <= 1.1, "checked": p >= 1033}
# PC
pcres = {}
cmeas = {}
for d in meas + pc:
    if d["family"] == "C":
        cmeas.setdefault(d["p"], {})[(d["curve_seed"], d["solver_seed"])] = d
for small, big in [(349, 1033), (1381, 4099), (5479, 16411)]:
    if small in cmeas and big in cmeas:
        tr = geomean([d["trials"] for d in cmeas[big].values()]) / geomean([d["trials"] for d in cmeas[small].values()])
        nc = geomean([d["n_cols"] + 10 for d in cmeas[big].values()]) / geomean([d["n_cols"] + 10 for d in cmeas[small].values()])
        pcres["%d/%d" % (big, small)] = {"trials_ratio": tr, "ncols_plus10_ratio": nc, "rel_diff": tr / nc - 1, "pass": abs(tr / nc - 1) <= 0.15}
# yield
yld = {}
for p in mprimes:
    for arm in ["Q_full", "C_full", "P_full", "P_orbit"]:
        rows = [d for d in meas if d["p"] == p and d["arm"] == arm]
        if rows:
            yld["%d/%s" % (p, arm)] = sum(d["accepted"] for d in rows) / sum(d["trials"] for d in rows)

# decision
used = [4099, 8209, 16411]
RT = {p: part2[p]["fpm_total"]["geomean"] for p in used if p in part2}
gate_used = {}
for p in used:
    bad = []
    if p in aa and not aa[p]["pass"]:
        bad.append("A/A p=%d" % p)
    gate_used[p] = bad
for k, v in pcres.items():
    if int(k.split("/")[0]) in used and not v["pass"]:
        gate_used[int(k.split("/")[0])].append("PC " + k)
meas_gate_bad = [g for g in ["M1", "M2", "M3", "M4", "M5"] if not gates[g]]
tiny_gate_bad = [g for g in ["T1", "T2", "T3", "T4", "T5", "T6", "T6_CQ", "T7a", "T7b", "T7c", "T8"] if not gates[g]]
if len(RT) == 3 and all(RT[p] >= 2.0 for p in used):
    numeric = "supported"
elif 8209 in RT and 16411 in RT and RT[8209] < 2.0 and RT[16411] < 2.0:
    numeric = "killed"
else:
    numeric = "inconclusive"
gate_blockers = meas_gate_bad + tiny_gate_bad + [x for v in gate_used.values() for x in v]
h3 = "inconclusive" if gate_blockers else numeric
h3p = {}
if 16411 in part2:
    tr = part2[16411]["trials"]["geomean"]
    la = part2[16411]["fpm_la_w"]["geomean"]
    h3p = {"trials_ratio": tr, "relation_prediction": "held" if 2.5 <= tr <= 3.5 else "missed",
           "la_w_ratio": la, "la_prediction": "held" if 7 <= la <= 11 else "missed"}


def nearest_rho_prime(p):
    return min(rho_primes, key=lambda q: abs(math.log(q) - math.log(p)))


dist = {}
for p in mprimes:
    if p not in part2:
        continue
    q = 8209 if p >= 8209 else nearest_rho_prime(p)
    s = S[q]
    rt = part2[p]["fpm_total"]["geomean"]
    base = min(s["S1"], s["S2"]) / min(s["S1"], s["S2"], s["S3"], s["S6"])
    orbit_rows = [d for d in meas if d["p"] == p and d["arm"] == "P_orbit"]
    t_orb = geomean([d["fpm_total"] for d in orbit_rows])
    proxy = t_orb / (math.sqrt(math.pi * p ** 3 / 4) * min(s["S1"], s["S2"], s["S3"], s["S6"]))
    dist[p] = {"rhocost_prime_used": q, "G": rt / base, "G_gt_1": rt / base > 1, "R_T": rt, "min_S1S2_over_min_all": base,
               "proxy_T_orbit_geomean": t_orb, "proxy_distance": proxy, "label": "proxy (r_proxy = p^3, no descent, not a DLP cost)"}

# per-arm table of means
arm_tab = {}
for p in mprimes:
    for arm in ["Q_full", "C_full", "P_full", "P_orbit"]:
        rows = [d for d in meas if d["p"] == p and d["arm"] == arm]
        if not rows:
            continue
        mean = lambda k: sum(d[k] for d in rows) / len(rows)
        arm_tab["%d/%s" % (p, arm)] = {k: mean(k) for k in ["n_points", "n_cols", "trials", "accepted", "dup", "trivial", "nonsquare", "tangent", "la_w", "L_bm", "fpm_enum", "fpm_line", "fpm_la_unw", "fpm_la_w", "fpm_total"]}
        arm_tab["%d/%s" % (p, arm)]["n_runs"] = len(rows)
        arm_tab["%d/%s" % (p, arm)]["yield"] = yld["%d/%s" % (p, arm)]

out = {"label": "[pcc:unverified]", "gates": gates, "gate_failures": fails, "tiny_T7b_determined": ndet,
       "tiny_T7b_orbit_only_determined": ndet_orb, "tiny_P_curves": len(tp),
       "part1": {"H1": part1["H1"], "per_op": part1["per_op"], "Q": {str(k): v for k, v in part1["Q"].items()},
                 "R2": {str(k): v for k, v in part1["R2"].items()}, "H2_P": {str(k): v for k, v in S.items()}},
       "part2": {str(k): v for k, v in part2.items()}, "A_A": {str(k): v for k, v in aa.items()}, "PC": pcres, "yield": yld,
       "H3": {"verdict": h3, "numeric_only_verdict": numeric, "R_T": {str(k): v for k, v in RT.items()},
              "gate_blockers": gate_blockers},
       "H3_prime": h3p, "distance": {str(k): v for k, v in dist.items()}, "arm_table": arm_tab}
json.dump(out, open(a.out, "w"), indent=1, sort_keys=True)

# ------------------------------------------------------------------ tables.md
f = open(a.tables, "w")
w = lambda s="": f.write(s + "\n")
w("# L10 tables [pcc:unverified]")
w()
w("All numbers: one session, shared 4-core container, no independent replay. Counts, not wall time.")
w()
w("## Gates")
w("| gate | pass | failing records |")
w("|---|---|---|")
for g, v in gates.items():
    w("| %s | %s | %s |" % (g, "pass" if v else "FAIL", "; ".join(fails[g][:12])))
w()
w("Tiny Picard curves with T7b status determined (worst of full and orbit arm): %d of %d; orbit arm alone: %d of %d." % (ndet, len(tp), ndet_orb, len(tp)))
w()
w("## Per-operation Pic costs (mean FPM per operation; mul + 10 inv)")
w("| family | p | A (add) | N (neg) | D (double) | R (rho) |")
w("|---|---|---|---|---|---|")
for k, v in part1["per_op"].items():
    fam, p = k.split("/")
    w("| %s | %s | %s | %s | %s | %s |" % (fam, p, *["%.1f" % v[o] if v[o] is not None else "-" for o in ["A", "N", "D", "R"]]))
w()
w("## H1: F_Q = (A+N)/(sqrt(2) A), Q family")
w("| p | A | N | F_Q | R2 bound_r median mul/op | A_mul+N_mul | rel diff | R2 |")
w("|---|---|---|---|---|---|---|---|")
for p in rho_primes:
    q, r = part1["Q"][p], part1["R2"][p]
    w("| %d | %.1f | %.1f | %.4f | %s | %.1f | %s | %s |" % (p, q["A"], q["N"], q["F_Q"], "%.1f" % r["bound_r_median_mul_per_op"] if r["bound_r_median_mul_per_op"] else "-", r["A_mul_plus_N_mul"], "%.4f" % r["rel_diff"] if r["rel_diff"] is not None else "-", "pass" if r["pass"] else "FAIL"))
w()
w("H1 verdict: **%s**. Rows that would be affected if supported: %s." % (h1, "; ".join(AFFECTED)))
w()
w("## H2: Picard class walks, ideal cost / sqrt(pi r / 4) (model values)")
w("| p | A | N | R | S1 | S2 | S3 | S6 | argmin | min/S2 |")
w("|---|---|---|---|---|---|---|---|---|---|")
for p in rho_primes:
    s = S[p]
    w("| %d | %.1f | %.1f | %.1f | %.1f | %.1f | %.1f | %.1f | %s | %.4f |" % (p, s["A"], s["N"], s["R"], s["S1"], s["S2"], s["S3"], s["S6"], s["argmin"], s["min_over_S2"]))
w()
w("## Measurement tier: means over the (3 curve x 2 solver seed) runs per arm")
w("| p | arm | n_points | n_cols | trials | accepted | yield | dup | trivial | nonsq | w | fpm_enum | fpm_line | fpm_la_w | fpm_total |")
w("|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|")
for k, v in arm_tab.items():
    p, arm = k.split("/")
    w("| %s | %s | %.1f | %.1f | %.1f | %.1f | %.4f | %.1f | %.1f | %.1f | %.1f | %.0f | %.0f | %.0f | %.0f |" % (p, arm, v["n_points"], v["n_cols"], v["trials"], v["accepted"], v["yield"], v["dup"], v["trivial"], v["nonsquare"], v["la_w"], v["fpm_enum"], v["fpm_line"], v["fpm_la_w"], v["fpm_total"]))
w()
w("## Part 2: P_full / P_orbit paired ratios (geomean [min, max])")
w("| p | fpm_total | boot95 (fpm_total) | trials | fpm_la_w | fpm_line | fpm_enum |")
w("|---|---|---|---|---|---|---|")
for p, v in part2.items():
    fm = lambda m: "%.3f [%.3f, %.3f]" % (v[m]["geomean"], v[m]["min"], v[m]["max"])
    w("| %d | %s | [%.3f, %.3f] | %s | %s | %s | %s |" % (p, fm("fpm_total"), *v["fpm_total"]["boot95"], fm("trials"), fm("fpm_la_w"), fm("fpm_line"), fm("fpm_enum")))
w()
w("H3 verdict: **%s** (numeric-only: %s; gate blockers: %s)" % (h3, numeric, ", ".join(gate_blockers) or "none"))
w()
w("R_T at 4099, 8209, 16411: %s" % ", ".join("%.3f" % RT[p] for p in used if p in RT))
w()
if h3p:
    w("H3' at p=16411: trials ratio %.3f (%s, band [2.5, 3.5]); LA_w ratio %.3f (%s, band [7, 11])." % (h3p["trials_ratio"], h3p["relation_prediction"], h3p["la_w_ratio"], h3p["la_prediction"]))
    w()
w("## A/A (P_full solver seed 1 / seed 2, fpm_total geomean over 3 curves; band [0.9, 1.1] for p >= 1033)")
w("| p | geomean | per curve | pass | checked |")
w("|---|---|---|---|---|")
for p, v in aa.items():
    w("| %d | %.4f | %s | %s | %s |" % (p, v["geomean"], ", ".join("%.3f" % x for x in v["per_curve"]), "pass" if v["pass"] else "out of band", v["checked"]))
w()
w("## PC (C_full trials ratio vs (n_cols+10) ratio; within 15%)")
w("| big/small | trials ratio | (n_cols+10) ratio | rel diff | pass |")
w("|---|---|---|---|---|")
for k, v in pcres.items():
    w("| %s | %.4f | %.4f | %.4f | %s |" % (k, v["trials_ratio"], v["ncols_plus10_ratio"], v["rel_diff"], "pass" if v["pass"] else "FAIL"))
w()
w("## Distance to the matched rho (reported, not a gate)")
w("| p | rhocost prime | R_T | min(S1,S2)/min(S1,S2,S3,S6) | G | G>1 | proxy distance (r=p^3) |")
w("|---|---|---|---|---|---|---|")
for p, v in dist.items():
    w("| %d | %d | %.3f | %.4f | %.4f | %s | %.4g |" % (p, v["rhocost_prime_used"], v["R_T"], v["min_S1S2_over_min_all"], v["G"], v["G_gt_1"], v["proxy_distance"]))
w()
f.close()
print("wrote", a.out, a.tables)
