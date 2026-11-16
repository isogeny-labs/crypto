#!/usr/bin/env python3
"""Lays out summary.json (computed by examples/jv_sieve_study.rs) as markdown
tables. Layout only: every number is copied from the file. Usage: tables.py <dir>"""
import json, sys
d = sys.argv[1]
sm = json.load(open(f"{d}/summary.json"))
def fm(v, p):
    return f"{v:.0f}" if abs(v) >= 10 ** p else f"{v:.{p}g}"
ci = lambda x, p=4: f"{fm(x['mean'], p)} [{fm(x['ci95'][0], p-1)}, {fm(x['ci95'][1], p-1)}]"
rc = lambda x, p=4: f"{fm(x['ratio'], p)} [{fm(x['ci95'][0], p-1)}, {fm(x['ci95'][1], p-1)}]"
out = []
w = out.append
w("## T1. Sieve against the strong rho (R3), paired by instance\n")
w("S, S⁺ in units of 331·√ℓ F_p multiplications. Ratios are sieve/rho: below 1 is the sieve cheaper. 95% bootstrap over instances.\n")
w("| p | log₂ℓ | n | sieve S | sieve S⁺ | rho S (harness / tuned) | rho S⁺ (harness / tuned) | S ratio, harness | S ratio, tuned | S⁺ ratio, harness | S⁺ ratio, tuned |")
w("|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|")
for e in sm["sizes"]:
    r = e.get("rho", {}).get("R3")
    if not r: continue
    if "ic_over_rho" not in r:
        w(f"| {e['p']} | {e['log2_l']:.1f} | {r['S_harness']['n']} | withheld: {r['status']} |||||||||")
        continue
    io = r["ic_over_rho"]
    w(f"| {e['p']} | {e['log2_l']:.1f} | {r['S_harness']['n']} | {ci(e['ic_S'])} | {ci(e['ic_S_plus'])} | {r['S_harness']['mean']:.3f} / {r['S_tuned']['mean']:.3f} | {r['S_plus_harness']['mean']:.3f} / {r['S_plus_tuned']['mean']:.3f} | {rc(io['S_harness'])} | {rc(io['S_tuned'])} | {rc(io['S_plus_harness'])} | {rc(io['S_plus_tuned'])} |")
w("\n## T2. §5a band: sieve/rho with native work priced at w GAE per operation on every arm\n")
w("Layer A prices every non-multiplication counted operation including F_p additions; layer B excludes F_p additions. The sieve side adds a bound for F4/linear-algebra additions it does not emit (one per multiplication outside the sieve and enumeration). A ratio is a rank only where its whole interval is on one side of 1 at every weight and in all four columns.\n")
w("| p | w | A, harness | A, tuned | B, harness | B, tuned | ranked at every weight |")
w("|--:|--:|--:|--:|--:|--:|:--|")
for e in sm["sizes"]:
    r = e.get("rho", {}).get("R3")
    if not r: continue
    for i, b in enumerate(r["band_ratio_ic_over_rho"]):
        w(f"| {e['p'] if i == 0 else ''} | {b['w']} | {rc(b['A_harness'],3)} | {rc(b['A_tuned'],3)} | {rc(b['B_harness'],3)} | {rc(b['B_tuned'],3)} | {('yes' if r['ranked_across_band_ic_below_rho'] else 'no') if i == 0 else ''} |")
w("\n## T3. Rho factorial: what negation and batched inversion each buy (S, harness price; n per cell)\n")
w("| p | arm | n | steps/√ℓ | S harness | S tuned | S⁺ harness | wall s (1 core) |")
w("|--:|:--|--:|--:|--:|--:|--:|--:|")
names = {"R0": "R0 plain", "R1": "R1 batched", "R2": "R2 negation", "R3": "R3 negation + batched", "R0_aa": "R0 A/A repeat", "R3_aa": "R3 A/A repeat"}
for e in sm["sizes"]:
    for a, nm in names.items():
        r = e.get("rho", {}).get(a)
        if not r: continue
        w(f"| {e['p']} | {nm} | {r['S_harness']['n']} | {ci(r['steps_over_sqrt_l'],3)} | {ci(r['S_harness'],3)} | {ci(r['S_tuned'],3)} | {ci(r['S_plus_harness'],3)} | {r['wall_s']['mean']:.0f} |")
w("\n## T4. Dense against sparse FGLM in the sieve's descent (paired, same instances)\n")
w("| p | pairs | identical descent digest | identical outcome | S dense/sparse | linear stage dense/sparse | descent muls dense/sparse | sparse→dense fallbacks per run |")
w("|--:|--:|--:|--:|--:|--:|--:|--:|")
for e in sm["sizes"]:
    v = e.get("dense_vs_sparse")
    if not v: continue
    w(f"| {e['p']} | {v['pairs']} | {v['identical_descent_digest']} | {v['identical_outcome']} | {rc(v['S_dense_over_sparse'])} | {rc(v['linear_stage_dense_over_sparse'])} | {rc(v['descent_muls_dense_over_sparse'])} | {v['sparse_fallbacks_per_run']:.2f} |")
w("\n## T5. Exponent of total F_p multiplications in ℓ\n")
w("| quantity | slope [95%] | sizes | status |")
w("|:--|--:|:--|:--|")
def row(nm, k, sizes, st):
    v = sm.get(k)
    if v: w(f"| {nm} | {v['slope']:.3f} [{v['ci95'][0]:.3f}, {v['ci95'][1]:.3f}] | {sizes} | {st} |")
row("sieve", "exponent_ic_muls_vs_l", "251–1823", "pre-registered; theory 1/3")
row("strong rho (harness price)", "exponent_rho_R3_harness_muls_vs_l", "251–1823", "pre-registered; theory 1/2 (instrument check)")
row("sieve", "exponent_ic_muls_vs_l_p_ge_1009_POST_HOC", "1009–1823", "**post hoc** (m = 9 regime), not in the protocol")
row("strong rho", "exponent_rho_R3_harness_muls_vs_l_p_ge_1009_POST_HOC", "1009–1823", "**post hoc**")
w("\nLocal sieve exponents between adjacent mean sizes: " + ", ".join(f"{x['ic']:.3f}" for x in sm["local_exponents_ic"]) + ".")
aa = [e["ic_aa"] for e in sm["sizes"] if e.get("ic_aa")]
if aa: w(f"\nIC A/A (sparse, p = 1009, run twice): {aa[0]['identical_counts']}/{aa[0]['pairs']} instances with identical counts and digests.")
w(f"\nPre-registered kill rules: K1 fired at p = {sm['K1_killed_at_p'] or 'none'}; K2 narrowed at p = {sm['K2_narrowed_at_p'] or 'none'}.")
open(f"{d}/TABLES.md", "w").write("\n".join(out) + "\n")
print("wrote TABLES.md")

# compact T1 spliced into README.md between its markers
rows = ["| p | S, harness | S, tuned | S⁺, harness | S⁺, tuned |", "|--:|--:|--:|--:|--:|"]
for e in sm["sizes"]:
    r = e.get("rho", {}).get("R3")
    if not r: continue
    if "ic_over_rho" not in r:
        rows.append(f"| {e['p']} | withheld ({r['status']}) | | | |"); continue
    io = r["ic_over_rho"]
    rows.append(f"| {e['p']} | {rc(io['S_harness'],3)} | {rc(io['S_tuned'],3)} | {rc(io['S_plus_harness'],3)} | {rc(io['S_plus_tuned'],3)} |")
rd = open(f"{d}/README.md").read()
a, b = "<!-- T1:begin -->", "<!-- T1:end -->"
if a in rd and b in rd:
    rd = rd[:rd.index(a) + len(a)] + "\n" + "\n".join(rows) + "\n" + rd[rd.index(b):]
    open(f"{d}/README.md", "w").write(rd)
