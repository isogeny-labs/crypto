#!/usr/bin/env python3
"""Build RESULTS.json / RESULTS.md / graph.svg from results/*.json (pure python)."""
import json, glob, math, os
D = os.path.dirname(os.path.abspath(__file__))
R = D + "/results"
REF = 0.9322          # rescore protocol pooled negation-rho reference, U1 (group additions / sqrt l)
RHO_NATIVE = 15.81    # rescore: rho native ops / sqrt l
REF_A = {1301: 0.9203, 1451: 1.0095, 1601: 0.8669}
def load(pat):
    out = []
    for f in sorted(glob.glob(R + "/" + pat)):
        out.append(json.load(open(f)))
    return out
def geo(xs): return math.exp(sum(math.log(x) for x in xs) / len(xs))
def sw(arm, w):
    return arm["S"] + w * arm["native_per_sqrt_l"]
def ratio(arm, w):
    return sw(arm, w) / (REF + w * RHO_NATIVE)
paired = [d for d in load("paired_p*_s*.json") if d["p"] >= 1000]
naive = []
narm = json.load(open(R + "/narm_rank_analysis.json"))
census = []
for f in sorted(glob.glob(R + "/census_inst*.jsonl")):
    census += [json.loads(l) for l in open(f) if l.startswith("{")]
rand = [json.loads(l) for f in sorted(glob.glob(R + "/census_random*.jsonl")) for l in open(f) if l.startswith("{")]
ctrl = [json.loads(l) for f in sorted(glob.glob(R + "/control*.jsonl")) for l in open(f) if l.startswith("{")]
res = {"unit": "[pcc:unverified] local development check", "ref_U1": REF, "rho_native_per_sqrt_l": RHO_NATIVE,
       "census_instances": census, "census_random": rand, "controls": ctrl, "paired": [], "naive": [], "n_arm_rank_analysis": narm, "by_prime": {}}
for d in paired:
    a = d["arm_A"]; q = d["arm_Q"]
    res["paired"].append({"p": d["p"], "seed": d["seed"], "B": d["base_B"], "rows_Q_equal_rows_A": d["rows_Q_equal_rows_A"],
        "digest": d["relation_digest"], "S": a["S"], "S_rel": a["S_rel"], "S_la": a["S_la"], "relations": a["relations"],
        "unknowns_core": a["unknowns_core"], "la_ops": a["la_ops"], "native_per_sqrt_l": a["native_per_sqrt_l"],
        "S_Q_over_S_A": q["S"] / a["S"], "S_over_negrho_w0": ratio(a, 0), "S_over_negrho_w0.1": ratio(a, 0.1),
        "S_over_negrho_w1": ratio(a, 1), "S_over_negrho_perprime_ref_w0": a["S"] / REF_A[d["p"]], "solved_correct": a["solved_correct"]})
for p in sorted({x["p"] for x in res["paired"]}):
    xs = [x for x in res["paired"] if x["p"] == p]
    res["by_prime"][p] = {"n": len(xs), "geo_S": geo([x["S"] for x in xs]),
        **{f"geo_S_over_negrho_w{w}": geo([x[f"S_over_negrho_w{w}"] for x in xs]) for w in ("0", "0.1", "1")},
        "range_w0": [min(x["S_over_negrho_w0"] for x in xs), max(x["S_over_negrho_w0"] for x in xs)]}
for d in naive:
    a, q, n = d["arm_A"], d["arm_Q"], d["arm_N"]
    res["naive"].append({"p": d["p"], "seed": d["seed"], "B": d["base_B"],
        "unknowns_full_Q": q["unknowns_full"], "unknowns_full_N": n["unknowns_full"],
        "unknowns_core_Q": q["unknowns_core"], "unknowns_core_N": n["unknowns_core"],
        "relations_Q": a["relations"], "relations_N": n["relations"], "relations_ratio_Q_over_N": a["relations"] / max(1, n["relations"]),
        "S_Q": a["S"], "S_N": n["S"], "S_ratio_Q_over_N": a["S"] / n["S"] if n["S"] else None,
        "S_rel_ratio": a["S_rel"] / n["S_rel"] if n["S_rel"] else None, "la_ops_Q": a["la_ops"], "la_ops_N": n["la_ops"],
        "N_solved_correct": n["solved_correct"]})
RM = json.load(open(D + "/rho_matched.json"))
NM = json.load(open(R + "/narm_rank_multi.json"))
res["rho_matched"] = RM
res["n_arm_rank_multi"] = NM
res["kummer_j_check"] = json.loads(open(R + "/census_random_kummer103_jcheck.jsonl").read())
json.dump(res, open(D + "/RESULTS.json", "w"), indent=1)
L = ["# RESULTS (all numbers [pcc:unverified]; local development check)\n"]
L.append("## Automorphism census (exhaustive Moebius search, linear and semilinear)\n")
L.append("| p | seed | l | deg h | rational finite roots | linear maps (incl. id) | nontrivial linear | semilinear | iota = lambda -1 on G,Q |")
L.append("|---|---|---|---|---|---|---|---|---|")
for c in census:
    k = c["census"]
    L.append(f'| {c["p"]} | {c["seed"]} | {c["l"]} | {c["deg_h"]} | {k["rational_finite_roots"]} | {k["linear_total_incl_identity"]} | {k["linear_nontrivial"]} | {k["semilinear_total"]} | {c["iota_lambda_minus1_on_G"]}, {c["iota_lambda_minus1_on_Q"]} |')
L.append("\n## Random and Kummer alpha census, controls\n")
for r in rand:
    L.append(f'* p={r["p"]} kummer={r["kummer"]}: {r["n_alpha"]} alpha, {r["with_nontrivial_linear_aut"]} with a nontrivial linear automorphism ({r["nontrivial_linear_total"]} maps), {r["with_semilinear"]} with a semilinear map')
for c in ctrl:
    L.append(f'* control p={c["p"]}: planted x^7+5x total {c["planted_x7_plus_5x"]["linear_total"]} (expected 6); random septic nontrivial {c["random_septic"]["linear_nontrivial"]}')
L.append("\n## Paired Nagao arm (A = repo baseline rows, Q = orbit-quotient rows from G = {1, iota})\n")
L.append("| p | seed | B | rows Q = rows A | relations | unknowns (core) | LA ops | S | S_rel | S_la | S_Q/S_A | S/negrho w=0 | w=0.1 | w=1 |")
L.append("|---|---|---|---|---|---|---|---|---|---|---|---|---|---|")
for x in res["paired"]:
    L.append(f'| {x["p"]} | {x["seed"]} | {x["B"]} | {x["rows_Q_equal_rows_A"]} | {x["relations"]} | {x["unknowns_core"]} | {x["la_ops"]} | {x["S"]:.4f} | {x["S_rel"]:.4f} | {x["S_la"]:.5f} | {x["S_Q_over_S_A"]:.3f} | {x["S_over_negrho_w0"]:.3f} | {x["S_over_negrho_w0.1"]:.3f} | {x["S_over_negrho_w1"]:.3f} |')
L.append("\nPer prime geometric means (S/negation-rho, reference 0.9322 U1, rho native 15.81 per sqrt l):\n")
for p, b in res["by_prime"].items():
    L.append(f'* p={p} (n={b["n"]}): w=0 {b["geo_S_over_negrho_w0"]:.3f} (range {b["range_w0"][0]:.3f}-{b["range_w0"][1]:.3f}), w=0.1 {b["geo_S_over_negrho_w0.1"]:.3f}, w=1 {b["geo_S_over_negrho_w1"]:.3f}')
L.append("\n## Arm N (unquotiented signed unknowns), rank analysis on recorded relations (p=251 seed 1, B=%d)\n" % narm["B"])
L.append("* d determined by the plain N system (2B+1 = %d unknowns): first at %s relations (rank %d of %d at 420 relations)" % (narm["unknowns_N"], narm["first_relations_d_determined"]["plain"], narm["prefix"][-1]["rank_plain"], narm["unknowns_N"]))
L.append("* d determined once the B free iota rows are added: first at %s relations (grid step 10); arm A solved this instance with 154 relations" % narm["first_relations_d_determined"]["with_iota_rows"])
L.append("\n## Arm N rank analysis on three instances (p=251 seeds 1-3, first 420 relations)\n")
L.append("| seed | B | unknowns N | rank plain N at 420 | first k, plain N | first k, N + iota rows | first k, orbit-quotient system (arm A) |")
L.append("|---|---|---|---|---|---|---|")
for r in NM:
    f = r["first_k_d_determined"]
    L.append(f'| {r["seed"]} | {r["B"]} | {r["unknowns_N"]} | {r["rank_and_columns_at_420"]["plain"][0]} of {r["rank_and_columns_at_420"]["plain"][1]} | {f["plain"]} | {f["iota"]} | {f["quot"]} |')
L.append("\n## Matched negation-map rho on the same instances (rho_matched.py; same l and c_add_e asserted per cell)\n")
L.append("Per instance: rho reference = mean of that instance's 12 rho runs; interval = 95% bootstrap over those 12 runs only (the cover run is one deterministic run).\n")
L.append("| p | seed | rho steps/sqrt l | rho native/sqrt l | S/rho w=0 [95%] | w=0.1 [95%] | w=1 [95%] |")
L.append("|---|---|---|---|---|---|---|")
for c in RM["cells"]:
    L.append(f'| {c["p"]} | {c["seed"]} | {c["rho_steps_per_sqrt_l"]:.4f} | {c["rho_native_per_sqrt_l"]:.2f} | ' + " | ".join(f'{c["S_over_rho_w"+w]:.3f} [{c["ci_w"+w][0]:.3f}, {c["ci_w"+w][1]:.3f}]' for w in ("0","0.1","1")) + " |")
L.append(f'\nPooled reference ({RM["pooled_rho_n"]} rho runs over the 9 instances, steps/sqrt l {RM["pooled_rho_steps_per_sqrt_l"]:.4f}, native {RM["pooled_rho_native_per_sqrt_l"]:.2f}); bootstrap over rho runs and over the 3 instances (too few instances for a trustworthy cover-side interval):\n')
L.append("| p | S/rho w=0 [95%] | w=0.1 [95%] | w=1 [95%] |")
L.append("|---|---|---|---|")
for p, d in RM["by_prime_pooled_ref"].items():
    L.append(f"| {p} | " + " | ".join(f'{d["geo_w"+w]:.3f} [{d["ci_w"+w][0]:.3f}, {d["ci_w"+w][1]:.3f}]' for w in ("0","0.1","1")) + " |")
L.append("\n(old per-cell table of the abandoned naive runs removed)\n")
open(D + "/RESULTS.md", "w").write("\n".join(L) + "\n")
# SVG: S/negation-rho per instance at w=0, 0.1, 1 by prime, with the 1.0 line
W, H, m = 720, 420, 60
ps = sorted(res["by_prime"])
if ps:
    ymax = max(1.6, max(x["S_over_negrho_w1"] for x in res["paired"]) * 1.1)
    X = lambda i: m + (i + 0.5) * (W - 2 * m) / len(ps)
    Y = lambda v: H - m - v / ymax * (H - 2 * m)
    s = [f'<svg xmlns="http://www.w3.org/2000/svg" width="{W}" height="{H}" font-family="sans-serif" font-size="12">',
         f'<rect width="{W}" height="{H}" fill="white"/>',
         f'<line x1="{m}" y1="{Y(1)}" x2="{W-m}" y2="{Y(1)}" stroke="#c00" stroke-dasharray="5,4"/><text x="{W-m-4}" y="{Y(1)-4}" text-anchor="end" fill="#c00">matched negation rho = 1</text>',
         f'<line x1="{m}" y1="{H-m}" x2="{W-m}" y2="{H-m}" stroke="black"/><line x1="{m}" y1="{m}" x2="{m}" y2="{H-m}" stroke="black"/>',
         f'<text x="{W/2}" y="22" text-anchor="middle" font-size="14">Nagao arm (quotient = baseline, G = {{1, iota}}): S / negation-rho</text>',
         f'<text x="16" y="{H/2}" transform="rotate(-90 16 {H/2})" text-anchor="middle">S / matched rho</text>']
    for v in range(0, int(ymax * 2) + 1):
        y = Y(v / 2)
        if y > m - 1:
            s.append(f'<text x="{m-6}" y="{y+4}" text-anchor="end">{v/2:.1f}</text>')
    cols = {"0": "#1f77b4", "0.1": "#ff7f0e", "1": "#2ca02c"}
    for i, p in enumerate(ps):
        s.append(f'<text x="{X(i)}" y="{H-m+18}" text-anchor="middle">p = {p}</text>')
        xs = [x for x in res["paired"] if x["p"] == p]
        for j, w in enumerate(("0", "0.1", "1")):
            for k, x in enumerate(xs):
                cx = X(i) + (j - 1) * 40 + (k - 1) * 8
                s.append(f'<circle cx="{cx}" cy="{Y(x["S_over_negrho_w"+w])}" r="4" fill="{cols[w]}"/>')
    for i, p in enumerate(ps):
        d = RM["by_prime_pooled_ref"][str(p)] if str(p) in RM["by_prime_pooled_ref"] else RM["by_prime_pooled_ref"][p]
        for j, w in enumerate(("0", "0.1", "1")):
            cx = X(i) + (j - 1) * 40 + 22
            lo, hi = d["ci_w" + w]
            s.append(f'<line x1="{cx}" y1="{Y(lo)}" x2="{cx}" y2="{Y(hi)}" stroke="{cols[w]}" stroke-width="2"/><line x1="{cx-4}" y1="{Y(d["geo_w"+w])}" x2="{cx+4}" y2="{Y(d["geo_w"+w])}" stroke="{cols[w]}" stroke-width="2"/>')
    s.append(f'<text x="{W-m}" y="38" text-anchor="end" font-size="10">dots: instances (reference pooled mean); bars: geo-mean and 95% bootstrap, pooled rho, 3 instances</text>')
    for j, w in enumerate(("0", "0.1", "1")):
        s.append(f'<circle cx="{m+20+j*110}" cy="{H-14}" r="4" fill="{cols[w]}"/><text x="{m+28+j*110}" y="{H-10}">w = {w} GAE</text>')
    s.append("</svg>")
    open(D + "/graph.svg", "w").write("\n".join(s))
print("ok", len(paired), len(naive), len(census))
