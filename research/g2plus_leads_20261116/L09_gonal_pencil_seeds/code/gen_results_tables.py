#!/usr/bin/env python3
"""Emits the markdown tables of RESULTS.md from raw/summary.json (numbers only)."""
import json, sys
S = json.load(open(sys.argv[1]))
def f(x, d=3):
    if x is None: return "NA"
    if isinstance(x, float): return f"{x:.{d}f}"
    return str(x)
BET = ["1/8", "3/16", "1/4", "3/8", "1/2", "3/4"]
print("### Table 1: per cell, medians over 20 seeds (T = line trials; censored = runs of 20 censored, arms A/A2/B/PC1/PC4)\n")
print("| p | beta | n_FB | med T A | med T A2 | med T B | med T PC1 | med T PC4 | cens A/A2/B/PC1/PC4 | med r_B [q25,q75] | N | med r_PC1 | med r_PC4 | med(r_B-r_PC1) |")
print("|---|---|---|---|---|---|---|---|---|---|---|---|---|---|")
for k, c in sorted(S["cells"].items(), key=lambda kv: (int(kv[0].split("_")[0]), int(kv[0].split("_b")[1]))):
    p, b = k.split("_b"); p = int(p.split("_")[0]); b = int(b)
    m = c["median_T"]; ce = c["censored"]
    print(f"| {p} | {BET[b]} | {f(c['n_fb_median'],0)} | {f(m['A'],0)} | {f(m['A2'],0)} | {f(m['B'],0)} | {f(m['PC1'],0)} | {f(m['PC4'],0)} | {ce['A']}/{ce['A2']}/{ce['B']}/{ce['PC1']}/{ce['PC4']} | {f(c['r_B']['median'])} [{f(c['r_B']['q25'])},{f(c['r_B']['q75'])}] | {f(c['N'])} | {f(c['r_PC1']['median'])} | {f(c['r_PC4']['median'])} | {f(c['median_rB_minus_rPC1'])} |")
print("\n### Table 2: FPM per cell (medians; FPM_rel = line + pencil; FPM = mul + 10 inv)\n")
print("| p | beta | med FPM_rel A | med FPM_rel A2 | med FPM_rel B | med FPM_pencil (B) | med FPM_rel PC1 | med FPM_rel PC4 | med f_B [q25,q75] | N_f | med f_PC1 | med f_PC4 | cost_A2 (FPM_rel+24 n_cols^2) |")
print("|---|---|---|---|---|---|---|---|---|---|---|---|---|")
for k, c in sorted(S["cells"].items(), key=lambda kv: (int(kv[0].split("_")[0]), int(kv[0].split("_b")[1]))):
    p, b = k.split("_b"); p = int(p.split("_")[0]); b = int(b)
    m = c["median_fpm_rel"]
    print(f"| {p} | {BET[b]} | {f(m['A'],0)} | {f(m['A2'],0)} | {f(m['B'],0)} | {f(c['median_fpm_pencil_B'],0)} | {f(m['PC1'],0)} | {f(m['PC4'],0)} | {f(c['f_B']['median'])} [{f(c['f_B']['q25'])},{f(c['f_B']['q75'])}] | {f(c['N_f'])} | {f(c['f_PC1']['median'])} | {f(c['f_PC4']['median'])} | {f(c['cost_A2_fpm_plus_LA'],0)} |")
print("\n### Table 3: pencil supply of arm B per cell (medians over seeds) and comparator shortfalls\n")
print("| p | beta | N1 | pencil rows kept (<=2 LP) | split fibres with 0/1/2/3 LP | PC1 runs with shortfall | PC4 runs with shortfall |")
print("|---|---|---|---|---|---|---|")
for k, c in sorted(S["cells"].items(), key=lambda kv: (int(kv[0].split("_")[0]), int(kv[0].split("_b")[1]))):
    p, b = k.split("_b"); p = int(p.split("_")[0]); b = int(b)
    pl = c["pencil_lp_B_median"]
    print(f"| {p} | {BET[b]} | {f(c['N1_median'],0)} | {f(c['pencil_rows_B_median'],0)} | {'/'.join(f(x,0) for x in pl)} | {c['pc_shortfall_PC1_runs']}/20 | {c['pc_shortfall_PC4_runs']}/20 |")
print("\n### Table 4: beta*(p) (chosen from A2 only) and verdicts computed mechanically by analyze.py\n")
print("| p | beta* | trials: verdict | median r_B | CI95 | thr = 0.05+N | pos. control median r_PC4 | censor ok | gates ok | FPM: verdict | median f_B | CI95 | thr_f | median f_PC4 |")
print("|---|---|---|---|---|---|---|---|---|---|---|---|---|---|")
for p in ("251", "1021", "4093", "16381"):
    v = S["verdicts"]["at_beta_star"][p]
    bs = v["beta_idx"]
    if bs is None:
        print(f"| {p} | none | INCONCLUSIVE | | | | | | | | | | | |"); continue
    t, fp = v["trials"], v["fpm"]
    print(f"| {p} | {BET[bs]} | {t['verdict']} | {f(t['detail']['median'])} | [{f(t['detail']['ci95'][0])},{f(t['detail']['ci95'][1])}] | {f(t['detail']['thr'])} | {f(t['detail']['positive_control_median'])} | {t['censor_ok']} | {t['gates_ok']} | {fp['verdict']} | {f(fp['detail']['median'])} | [{f(fp['detail']['ci95'][0])},{f(fp['detail']['ci95'][1])}] | {f(fp['detail']['thr'])} | {f(fp['detail']['positive_control_median'])} |")
print("\n### Table 5: secondary mechanical verdicts per (p, beta) (same rule; gate G5 failure forces INCONCLUSIVE everywhere)\n")
print("| p | " + " | ".join(BET) + " |")
print("|---|" + "---|" * 6)
for p in (251, 1021, 4093, 16381):
    row = []
    for b in range(6):
        v = S["verdicts"]["secondary_all_cells"].get(f"{p}_b{b}")
        row.append("NA" if v is None else f"{v['trials'][:4]}/{v['fpm'][:4]}")
    print(f"| {p} | " + " | ".join(row) + " |")
print("\n(cell entry: trials verdict / FPM verdict, first four letters)\n")
print("### Table 6: exact sub-runs (median T_exact over seeds; censored runs counted in the median with T = trials done)\n")
print("| p | beta | A: med T_exact (cens/20) | B: med T_exact (cens/20) | B3: med T_exact (cens/20) |")
print("|---|---|---|---|---|")
for k, v in sorted(S["b3_exploratory"].items(), key=lambda kv: (int(kv[0].split("_")[0]), int(kv[0].split("_b")[1]))):
    p, b = k.split("_b"); b = int(b)
    print(f"| {p.split('_')[0]} | {BET[b]} | " + " | ".join(f"{f(v[a]['median_T_exact'],0)} ({v[a]['censored']}/{v[a]['n']})" for a in ("A", "B", "B3")) + " |")
print("\n### Table 7: gates\n")
g = S["gates"]
print("| gate | outcome | numbers |")
print("|---|---|---|")
print(f"| G0 unit tests | pass (see raw/g0_tests.log) | 9 passed, 0 failed |")
print("| G1/G2 | no assertion fired in any run (an abort would have produced no output line; every command exit=0) | see raw/COMMANDS.log |")
for p, v in g["G3"]["per_p"].items():
    print(f"| G3 p={p} | {'pass' if v['pass'] else 'FAIL'} | x-fibre {f(v['frac_split_xfibre'],4)}, y-fibre {f(v['frac_split_yfibre'],4)}, line residual {f(v['frac_split_line_residual'],4)}, tolerance +-{v['tolerance']}, point sums = N1: {v['pts_sum_equals_N1']} |")
print(f"| G4 | {'pass' if g['G4']['pass'] else 'FAIL'} | {g['G4']['records']} A/B exact records, {g['G4']['violations']} violations |")
g5 = g["G5"]
print(f"| G5 | {'pass' if g5['pass'] else 'FAIL'} | n_A={g5['n_A']} (all 20x9 A exact runs censored), n_B={g5['n_B']}, median A (T_exact/T_uf - 1) = {f(g5['median_A_Texact_over_Tuf_minus_1'])}, median B = {f(g5['median_B_Texact_over_Tuf_minus_1'])}, paired B-A median = {f(g5['median_paired_B_minus_A_ratio'])}, n_pairs={g5['n_pairs']}; required A,B in [0,0.03], paired within +-0.01 |")
print(f"| G6 | {'pass' if g['G6']['pass'] else 'FAIL'} | {g['G6']} |")
print(f"| G7 | {'pass' if g['G7']['pass'] else 'FAIL'} | {g['G7']['within']}/{g['G7']['runs']} runs with T_B <= T_A + 64 (fraction {f(g['G7']['fraction'])}), {len(g['G7']['violations'])} violations |")
