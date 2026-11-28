#!/usr/bin/env python3
"""Renders RESULTS.md from raw/summary.json and raw/gsp_reference.json (numbers only)."""
import json
s = json.load(open("raw/summary.json"))
g = json.load(open("raw/gsp_reference.json"))
PS = ["257", "521", "1031", "2053", "4099"]
L = []
w = L.append
def f(x, d=4):
    return "n/a" if x is None else f"{x:.{d}f}"
w("# L06 results\n")
w("Executor output. Standing: all numbers `[pcc:unverified]`. Not a benchmark. Numbers only; no interpretation.")
w("Source: `raw/summary.json` (made by `python3 -I analyze.py raw`), `raw/*.jsonl`, `raw/gsp_reference.json`. Hashes: `data.sha256`, `raw/COMMANDS.log`.\n")
w("## 1. Gates\n")
w("| gate | pass | counts |\n|---|---|---|")
G = s["gates"]
w(f"| G0 unit tests | {G['G0']['pass']} | 11 tests, failed lines {G['G0']['failed_lines']} |")
g1 = G["G1"]
w(f"| G1 two chi methods | **{g1['pass']}** | draws {g1['total']}, unique chi {g1['ok']}, agree {g1['agree']}, ambiguous {g1['ambiguous']} ({g1['ambiguous_fraction']*100:.2f}%, limit 2%); p=101: {g1['files']['101']['status']}, p=257: {g1['files']['257']['status']} |")
w(f"| G2 annihilation | {G['G2']['pass']} | {G['G2']['checked']} curves, failed {G['G2']['failed']} |")
w(f"| G3 Weil bounds | {G['G3']['pass']} | {G['G3']['checked']} curves, failed {G['G3']['failed']} |")
w(f"| G4 two criterion tests | {G['G4']['pass']} | census (curve, l) pairs {G['G4']['census_checked']}, failed {G['G4']['census_failed']}; reference samples {G['G4']['gsp']['samples_checked']}, failed {G['G4']['gsp']['failed']} |")
w(f"| G5 mod-2 structure | {G['G5']['pass']} | {G['G5']['checked']} curves, failed {G['G5']['failed']} |")
w(f"| G6 J[2] Lagrangians | {G['G6']['pass']} | total {G['G6']['total']}, pair {G['G6']['pair']}, other {G['G6']['other']}, pair_rule mismatches {G['G6']['pair_rule_mismatch']} |")
w(f"| G7 sampler | {G['G7']['pass']} | similitude failures {G['G7']['similitude_failures']}, genus-1 cells {G['G7']['cells']}, max abs z {G['G7']['max_abs_z']:.3f} (limit 4) |")
w("| G8 order determination | " + str(G['G8']['pass']) + " | per p, see below |\n")
w("G8 per p (ambiguous / chi-requested draws; limit 2%):\n")
w("| p | chi-requested | ambiguous | error | fraction | pass |\n|---|---|---|---|---|---|")
for p in PS:
    r = G["G8"]["per_p"][p]
    w(f"| {p} | {r['chi_requested']} | {r['ambiguous']} | {r['error']} | {r['ambiguous_fraction']:.5f} | {r['pass']} |")
w(f"\nGates among G0-G7 that failed: {s['gate_failures_G0_G7']}. p dropped by G8: {s['g8_dropped_p']}.\n")
w("## 2. Verdict computed by `analyze.py` (PROTOCOL.md section 6)\n")
w(f"**{s['verdict']}**\n")
w(f"For the record, the outcome of rules 2-5 (n and F_l thresholds) applied to the pooled PRIME population without regard to the gates is `{s['outcome_of_rules_2_to_5_regardless_of_gates_NOT_THE_VERDICT']}`. This line is not the verdict: rule 1 (a failed gate among G0-G7) takes precedence.\n")
w("## 3. Populations\n")
w("| p | draws scanned | stop reason | U-block squarefree | status counts | ALL | PRIME | C2 | ODD |\n|---|---|---|---|---|---|---|---|---|")
for p in PS:
    r = s["populations"]["per_p"][p]
    w(f"| {p} | {r['draws']} | {r['stop_reason']} | {r['u_block_squarefree_seen']} | {r['status_counts']} | {r['ALL']} | {r['PRIME']} | {r['C2']} | {r['ODD']} |")
pp = s["populations"]["pooled_over_p_passing_G8"]
w(f"| pooled | | | | | {pp['ALL']} | {pp['PRIME']} | {pp['C2']} | {pp['ODD']} |\n")
w("ALL, C2, ODD use U-block draws with a determined chi (status ok); PRIME uses every ok draw with #J prime.\n")
w("## 4. Criterion fire counts (pooled over the five p)\n")
w("| l | population | n | fire | fraction | Wilson 95% | class fire / none / undetermined |\n|---|---|---|---|---|---|---|")
for l in ("3", "5", "7"):
    for k in ("PRIME", "ALL", "C2"):
        t = s["criterion"][l][k]["pooled"]
        c = t["class"]
        w(f"| {l} | {k} | {t['n']} | {t['fire']} | {f(t['fraction'])} | [{f(t['wilson95'][0])}, {f(t['wilson95'][1])}] | {c['fire']} / {c['none']} / {c['undetermined']} |")
w("\nPrimary metric F_l (pooled PRIME) with the kill threshold 0.01:\n")
w("| l | n | fire | F_l | lo | hi |\n|---|---|---|---|---|---|")
for l in ("3", "5", "7"):
    r = s["pooled_prime_F_l"][l]
    w(f"| {l} | {r['n']} | {r['fire']} | {f(r['F'])} | {f(r['lo'])} | {f(r['hi'])} |")
w("\n## 5. Per-p criterion tables\n")
for k in ("PRIME", "ALL", "C2"):
    w(f"### {k}\n")
    w("| l | p | n | fire | fraction | Wilson 95% | fire / none / undetermined |\n|---|---|---|---|---|---|---|")
    for l in ("3", "5", "7"):
        for p in PS:
            t = s["criterion"][l][k]["per_p"][p]
            c = t["class"]
            w(f"| {l} | {p} | {t['n']} | {t['fire']} | {f(t['fraction'])} | [{f(t['wilson95'][0])}, {f(t['wilson95'][1])}] | {c['fire']} / {c['none']} / {c['undetermined']} |")
    w("")
w("## 6. n_h and ext_deg on firing PRIME curves (pooled)\n")
w("| l | n_h histogram | ext_deg histogram |\n|---|---|---|")
for l in ("3", "5", "7"):
    t = s["criterion"][l]["PRIME"]["pooled"]
    w(f"| {l} | {t['n_h_hist_on_firing']} | {t['ext_deg_hist_on_firing']} |")
w("\nSame on C2 (pooled):\n")
w("| l | n_h histogram | ext_deg histogram |\n|---|---|---|")
for l in ("3", "5", "7"):
    t = s["criterion"][l]["C2"]["pooled"]
    w(f"| {l} | {t['n_h_hist_on_firing']} | {t['ext_deg_hist_on_firing']} |")
w("\nPer p, PRIME firing curves:\n")
w("| l | p | n_h histogram | ext_deg histogram |\n|---|---|---|---|")
for l in ("3", "5", "7"):
    for p in PS:
        t = s["criterion"][l]["PRIME"]["per_p"][p]
        w(f"| {l} | {p} | {t['n_h_hist_on_firing']} | {t['ext_deg_hist_on_firing']} |")
w("\n## 7. Arm A2 (l = 2)\n")
w("| population | n | fire_2 | stable pair-type kernel exists | pair_rule true | stable_other >= 2 | stable_other histogram | stable_pair histogram |\n|---|---|---|---|---|---|---|---|")
for k in ("ODD", "ALL"):
    t = s["A2"][k]["pooled"]
    w(f"| {k} (pooled) | {t['n']} | {t['fire_2']} | {t['stable_pair_exists']} | {t['pair_rule']} | {t['stable_other_ge2']} | {t['stable_other_hist']} | {t['stable_pair_hist']} |")
for k in ("ODD", "ALL"):
    for p in PS:
        t = s["A2"][k]["per_p"][p]
        w(f"| {k} p={p} | {t['n']} | {t['fire_2']} | {t['stable_pair_exists']} | {t['pair_rule']} | {t['stable_other_ge2']} | {t['stable_other_hist']} | {t['stable_pair_hist']} |")
pr = s["A2"]["predictions_on_ODD_pooled"]
w(f"\nFrozen predictions on ODD (pooled): (a) fire_2 on all: {pr['a_fire_2_all']}; (b) no pair-type stable kernel: {pr['b_no_pair_type']}; (c) stable_other >= 2 on all: {pr['c_other_ge2_all']}.\n")
w("`raw/l2_lagrangians.json` holds the per-cycle-type table (15 rows).\n")
w("## 8. Reference sampler (arm R), 200000 samples per (l, m), genus 3\n")
w("| l | m | fire | fire/samples | nz1 (charpoly(1) != 0) | fire_nz1 | fire_nz1/nz1 | chi mod l squarefree | none | undetermined | G4 agree |\n|---|---|---|---|---|---|---|---|---|---|---|")
for r in g["g3"]:
    w(f"| {r['l']} | {r['m']} | {r['fire']} | {r['fire']/r['samples']:.5f} | {r['nz1']} | {r['fire_nz1']} | {r['fire_nz1']/r['nz1']:.5f} | {r['squarefree']} | {r['none']} | {r['undetermined']} | {r['g4_agree']} |")
w("\nGenus-1 sampler gate (G7), 200000 samples per cell:\n")
w("| l | m | fire | exact | z |\n|---|---|---|---|---|")
for r in g["g1"]:
    w(f"| {r['l']} | {r['m']} | {r['fire']} | {r['exact']:.6f} | {r['z']:.3f} |")
w("\n## 9. S4: census cell versus reference, z scores\n")
w(f"Cells {s['S4']['n_cells']}, cells with |z| > 3: {s['S4']['n_abs_z_gt_3']}.\n")
w("| p | l | m = p mod l | population | n | fire | F_obs | F_ref | z |\n|---|---|---|---|---|---|---|---|---|")
for c in s["S4"]["cells"]:
    w(f"| {c['p']} | {c['l']} | {c['m']} | {c['pop']} | {c['n']} | {c['fire']} | {c['F_obs']:.4f} | {c['F_ref']:.4f} | {c['z']:.2f} |")
w("\n## 10. Deviations\n\nSee `DEVIATIONS.md`. Summary: G1 failed on its ambiguity limit; the remaining commands were run anyway and are reported here; the computed verdict is the one in section 2.\n")
open("RESULTS.md", "w").write("\n".join(L) + "\n")
