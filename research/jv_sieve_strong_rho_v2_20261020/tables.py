#!/usr/bin/env python3
"""Lays out summary.json (computed by examples/jv_sieve_study_v2.rs) as markdown. Layout only:
every number is copied from the file. Usage: tables.py <dir>"""
import json, sys
d = sys.argv[1]
sm = json.load(open(f"{d}/summary.json"))
def fm(v, p):
    return f"{v:.0f}" if abs(v) >= 10 ** p else f"{v:.{p}g}"
ci = lambda x, p=4: f"{fm(x['mean'], p)} [{fm(x['ci95'][0], p-1)}, {fm(x['ci95'][1], p-1)}]"
rc = lambda x, p=4: f"{fm(x['ratio'], p)} [{fm(x['ci95'][0], p-1)}, {fm(x['ci95'][1], p-1)}]"
out = []; w = out.append
acc = sm["acceptance"]
w("## T0. Acceptance rules (PROTOCOL.md): every decision is withheld unless all pass\n")
w("| rule | requirement | result |"); w("|:--|:--|:--|")
rows = [("A1", "complete seed inventories, every run correct and uncapped", acc["A1_samples_complete_and_valid"]),
        ("A2", "hash-scope completeness per run (2 per step plain, 4 with negation)", acc["A2_hash_scope_complete"]),
        ("A3", "plain rho reproduces the ledger's S ≈ 1.3 (interval contains 1.3, point in [1.0, 1.6])", acc["A3_positive_control"]["pass"]),
        ("A4", "rho A/A at p = 1009 (ratio in [0.85, 1.15], interval contains 1)", acc["A4_pass"]),
        ("A5", "sieve A/A at p = 1009 identical counts and digests", acc["A5_pass"]),
        ("A6", "dense and sparse FGLM identical descent digests and outcomes", acc["A6_dense_sparse_identical"]),
        ("A7", "strong-rho exponent point in [0.45, 0.55], interval contains 0.5", acc["A7_instrument_exponent"]["pass"])]
for r, t, ok in rows: w(f"| {r} | {t} | {'**pass**' if ok else '**FAIL**'} |")
w(f"\nAll pass: **{acc['all_pass']}**." + (f" Failed: {acc['failed']}" if acc['failed'] else ""))
w("\n## T1. Sieve against the strong rho (R3), paired by instance seed\n")
w("S, S⁺ in units of 331·√ℓ F_p multiplications. Ratios are sieve/rho: below 1 is the sieve cheaper. 95% bootstrap over instances.\n")
w("| p | log₂ℓ | n | sieve S | sieve S⁺ | rho S (harness / tuned) | rho S⁺ (harness / tuned) | S ratio, harness | S ratio, tuned | S⁺ ratio, harness | S⁺ ratio, tuned |")
w("|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|")
for e in sm["sizes"]:
    r = e.get("rho", {}).get("R3")
    if not r: continue
    io = r["ic_over_rho"]
    w(f"| {e['p']} | {e['log2_l']:.1f} | {r['n']} | {ci(e['ic_S'])} | {ci(e['ic_S_plus'])} | {r['S_harness']['mean']:.3f} / {r['S_tuned']['mean']:.3f} | {r['S_plus_harness']['mean']:.3f} / {r['S_plus_tuned']['mean']:.3f} | {rc(io['S_harness'])} | {rc(io['S_tuned'])} | {rc(io['S_plus_harness'])} | {rc(io['S_plus_tuned'])} |")
w("\n## T2. §5a band: sieve/rho with native work priced at w GAE per operation on every arm\n")
w("| p | w | A, harness | A, tuned | B, harness | B, tuned |"); w("|--:|--:|--:|--:|--:|--:|")
for e in sm["sizes"]:
    r = e.get("rho", {}).get("R3")
    if not r: continue
    for i, b in enumerate(r["band_ratio_ic_over_rho"]):
        w(f"| {e['p'] if i == 0 else ''} | {b['w']} | {rc(b['A_harness'],3)} | {rc(b['A_tuned'],3)} | {rc(b['B_harness'],3)} | {rc(b['B_tuned'],3)} |")
w("\n## T3. Rho factorial and controls (S, harness price)\n")
w("| p | arm | n | steps/√ℓ | S harness | S tuned | S⁺ harness | wall s (1 core) |"); w("|--:|:--|--:|--:|--:|--:|--:|--:|")
names = {"R0": "R0 plain", "R1": "R1 batched", "R2": "R2 negation", "R3": "R3 negation + batched", "R0_aa": "R0 A/A repeat", "R3_aa": "R3 A/A repeat"}
for e in sm["sizes"]:
    for a, nm in names.items():
        r = e.get("rho", {}).get(a)
        if r: w(f"| {e['p']} | {nm} | {r['n']} | {ci(r['steps_over_sqrt_l'],3)} | {ci(r['S_harness'],3)} | {ci(r['S_tuned'],3)} | {ci(r['S_plus_harness'],3)} | {r['wall_s']['mean']:.0f} |")
w("\n## T4. Dense against sparse FGLM in the sieve's descent\n")
w("| p | pairs | identical digest | identical outcome | S dense/sparse | linear stage dense/sparse | descent muls dense/sparse |"); w("|--:|--:|--:|--:|--:|--:|--:|")
for e in sm["sizes"]:
    v = e.get("dense_vs_sparse")
    if v: w(f"| {e['p']} | {v['pairs']} | {v['identical_digest']} | {v['identical_outcome']} | {rc(v['S_dense_over_sparse'])} | {rc(v['linear_stage_dense_over_sparse'])} | {rc(v['descent_muls_dense_over_sparse'])} |")
w("\n## T5. Exponent of total F_p multiplications in ℓ\n")
w("| quantity | slope [95%] | sizes |"); w("|:--|--:|:--|")
for nm, k, sz in (("sieve F1", "exponent_F1_all_sizes_sieve", "251–1823"), ("sieve F2", "exponent_F2_p_ge_1009_sieve", "1009–1823"),
                  ("strong rho, harness", "exponent_rho_R3_harness", "251–1823"), ("strong rho, tuned", "exponent_rho_R3_tuned", "251–1823")):
    v = sm.get(k)
    if v: w(f"| {nm} | {v['slope']:.3f} [{v['ci95'][0]:.3f}, {v['ci95'][1]:.3f}] | {sz} |")
dec = sm["decisions"]
w("\n## T6. Decisions (PROTOCOL.md D1–D6)\n")
if dec["status"] != "evaluated":
    w(f"**Withheld:** {dec.get('reason')}. {dec.get('failed')}")
else:
    w("| rule | result |"); w("|:--|:--|")
    for x in dec["D1_K1"]: w(f"| D1 (K1) p = {x['p']} | {x['result']}, S⁺ ratio interval [{x['ci95'][0]:.3g}, {x['ci95'][1]:.3g}] |")
    for x in dec["D2_K2"]: w(f"| D2 (K2) p = {x['p']} | ranked below at every weight: {x['ranked_below_at_every_weight']}; unresolved columns: {x['unresolved_columns']} |")
    f3 = dec["D3_exponent"]
    w(f"| D3 F1 (all sizes) | {f3['F1_all_sizes']['class']} |"); w(f"| D3 F2 (p ≥ 1009) | {f3['F2_p_ge_1009']['class']}; cost-grows-as-ℓ^(1/3) claim: {f3['F2_p_ge_1009']['claim_cost_grows_as_l13']} |")
    w(f"| D4 crossover (first p below 1) | S: {dec['D4_crossover_first_p_below_one']['S_harness']}; S⁺: {dec['D4_crossover_first_p_below_one']['S_plus_harness']} |")
    d5 = dec["D5"]
    w(f"| D5 negation buys (R0/R2 at 1009) | {d5['negation_buys_R0_over_R2']['buys']}, {rc(d5['negation_buys_R0_over_R2']['ratio'])} |")
    w(f"| D5 batching buys nothing, harness | {d5['batching_buys_nothing_harness']['holds']}; R0/R1 {rc(d5['batching_buys_nothing_harness']['R0_over_R1'])}, R2/R3 {rc(d5['batching_buys_nothing_harness']['R2_over_R3'])} |")
    w(f"| D5 batching costs, tuned (R3/R2) | {d5['batching_costs_tuned_R3_over_R2']['costs']}, {rc(d5['batching_costs_tuned_R3_over_R2']['ratio'])} |")
    for x in dec["D6_sparse_fglm"]: w(f"| D6 sparse gain p = {x['p']} | {x['gain']}, ratio {x['ratio']:.3f} [{x['ci95'][0]:.3f}, {x['ci95'][1]:.3f}] |")
open(f"{d}/TABLES.md", "w").write("\n".join(out) + "\n")
print("wrote TABLES.md")

# compact T1 spliced into README.md between its markers
rows = ["| p | S, harness | S, tuned | S⁺, harness | S⁺, tuned |", "|--:|--:|--:|--:|--:|"]
for e in sm["sizes"]:
    r = e.get("rho", {}).get("R3")
    if not r: continue
    io = r["ic_over_rho"]
    rows.append(f"| {e['p']} | {rc(io['S_harness'],3)} | {rc(io['S_tuned'],3)} | {rc(io['S_plus_harness'],3)} | {rc(io['S_plus_tuned'],3)} |")
rd = open(f"{d}/README.md").read()
a, b = "<!-- T1:begin -->", "<!-- T1:end -->"
if a in rd and b in rd:
    rd = rd[:rd.index(a) + len(a)] + "\n" + "\n".join(rows) + "\n" + rd[rd.index(b):]
    open(f"{d}/README.md", "w").write(rd)
