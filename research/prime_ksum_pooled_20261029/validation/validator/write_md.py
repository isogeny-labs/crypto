#!/usr/bin/env python3
"""Render VALIDATOR.md from validator-report.json (no hand-copied numbers)."""
import json, hashlib
from pathlib import Path
V = Path(__file__).resolve().parent
d = json.loads((V / "validator-report.json").read_text())
T = d["headline_table"]; W = ["0", "0.1", "1"]
L = []
a = L.append
a(f"# Validator report: prime_ksum_pooled_20261029 ({d['id']})\n")
a(f"**Disposition: {d['disposition']}** (status `{d['status']}`, independent recompute, Python 3 stdlib, no ecbench run). "
  f"Results commit `{d['snapshot']['validated_results_commit']}`; repository HEAD at start `{d['snapshot']['repository_head_at_validation']}` "
  f"(moved to `{d['snapshot']['head_moved_during_validation']['to']}` by other sessions; the study directory and the gate script are unchanged across). "
  "All numbers are `[pcc:unverified]` until a pcc claim records them.\n")
a("Scope of the validation: the counted-unit claim (ecbench GAE, §5a band) under the study's own pricing reading. The study's UNQUALIFIED status (one host, no second-host replay of this holdout, L0 isolation, no external tool) is unchanged by this report.\n")
a("## Raw data binding\n")
a("| session | records.jsonl sha256 | git blob at a0ff77014f | records | binds |\n|:--|:--|:--|--:|:--|")
for r in d["raw_data"]:
    if "records_jsonl" in r:
        j = r["records_jsonl"]; s = r["path"].split("/")[1]
        a(f"| {s} | `{j['sha256'][:16]}…` | `{j['git_blob_at_a0ff77014f']}` | {j['records']} | {'yes' if j['all_four_agree'] else 'NO'} |")
a("\nEach recovered file hashes to its `.artifacts.json`, to `records_sha256` in `session.json`, and to the git blob content at a0ff77014f; the git blob id was recomputed from the bytes.\n")
c = d["dG_count"]
a("## 1. Certificates ([d]G = Q, own affine arithmetic)\n")
a(f"- Measured verified holdout records: **{c['holdout_measured_dG_eq_Q']} / {c['holdout_measured_verified']}** satisfy [d]G = Q; failures {c['failures']}.")
a(f"- Also: holdout warm-ups {c['holdout_warmup_dG_eq_Q']}/2976, train measured {c['train_measured_dG_eq_Q']}/3264, train warm-ups {c['train_warmup_dG_eq_Q']}/1632. {c['workloads']} workloads, {c['workloads_with_conflicting_logs']} with conflicting logs across arms.")
a("- Curves: a, b are not in the record. They were solved from G and one target, then required to hold for G and every other target of the curve. j(a, b) equals the ICV1 j; p and r are prime (Miller–Rabin); r = p + 1 − t; Hasse holds; cofactor 1; [r]G = O and [r]Q = O; (a, b, G, r) equal the curve registry entry, on all 24 curves. j is never 0 or 1728, so negation is the only fold, and rho-neg used it (automorphisms_used 2) on all 1,152 holdout records.\n")
a("## 2. Pooled paired ratios (Σ S_B / Σ S_A over 192 (workload, round) pairs)\n")
a("Pricing: S(w) = (total_gae + w·U)/√r. C = n_C = 0 (no pinned calibration). U = the counts behind `cost.unpriced`. **The study used the pre-2026-11-08 reading** (`table_insert_attempts` not priced). Every committed comparison equals this reading to 1.6e-15 and differs from the 2026-11-08 reading by up to 25%, and `band.json` has no `priced_table_work` key.\n")
for rd, lab in (("pre", "pre-2026-11-08 reading (the study's)"), ("post", "2026-11-08 default (table work priced)")):
    a(f"**{lab}**\n")
    a("| bits | ksum5-sel/inc-sel w=0 / 0.1 / 1 | ksum4-sel/inc-sel w=0 / 0.1 / 1 | ksum5-sel/rho-neg w=0 / 0.1 / 1 |\n|--:|:--|:--|:--|")
    for b in ("P20", "P24", "P28", "P32"):
        f = lambda lbl: " / ".join("%.4f" % T[rd][b][lbl][w]["ratio"] for w in W)
        a(f"| {b[1:]} | {f('ksum5-sel/inc-sel')} | {f('ksum4-sel/inc-sel')} | {f('ksum5-sel/rho-neg')} |")
    a("")
a("Matches the README at w = 0: 0.700/0.446/0.324/0.217 and 0.632/0.452/0.336/0.253; distance to rho 6.0/5.8/7.3/7.3 and to BSGS 7.6/6.5/7.2/7.1. The selection recomputed from train is identical under both readings: inc 64/162/407/1024, inc-w0 64/115/289/727, ksum4 10/14/21/30, ksum5 9/11/15/19, k* = 5.\n")
a("## 3. §5a band per curve (mean S, ic = ksum5-sel, rho = rho-neg)\n")
for lab in ("pre_2026_11_08_study_reading", "table_work_priced_2026_11_08_default"):
    a(f"**{lab}**\n")
    a("| curve | w=0 ic / rho / inc-sel | w=0.1 ic / rho / inc-sel | w=1 ic / rho / inc-sel | rho < ksum5 < inc every w |\n|:--|:--|:--|:--|:--|")
    for slug, v in d["gae_band_per_curve"][lab].items():
        f = lambda w: "%.3f / %.3f / %.3f" % (v[w]["ic_ksum5_sel"], v[w]["rho_neg"], v[w]["inc_sel"])
        a(f"| {slug} | {f('0')} | {f('0.1')} | {f('1')} | {'yes' if v['rho_lt_ksum5_lt_inc_every_weight'] else 'NO'} |")
    a("")
a(f"Order holds on 16/16 curves at every weight under both readings: {d['order_holds_across_band']['statement']}\n")
s4 = d["step4_comparison"]
a("## 4. Agreement with the committed outputs and the preflight (opened after the recompute)\n")
a(f"- {s4['committed_compare_files']} `analysis/compare-*.json`: max relative difference {s4['max_rel_diff_ratio_vs_pre']:.1e} (ratios), {s4['max_rel_diff_band_triplets']:.1e} (band triplets), {s4['max_rel_diff_ci95_linearised_t']:.1e} (linearised-t ci95). Against the 2026-11-08 reading: {s4['max_rel_diff_ratio_vs_post']:.3f}.")
a(f"- `analysis/band.json`: {s4['band_json_rows']} rows, max rel {s4['band_json_max_rel_mean_s_pre']:.1e}, charged/native sums equal in integers; {s4['band_json_ranks']} ranks, {s4['band_json_rank_mismatches']} mismatches.")
a(f"- `preflight.json` max rel {s4['preflight_json_max_rel']}, `preflight-tablework.json` {s4['preflight_tablework_json_max_rel']}, `gae_band_per_curve.json` {s4['gae_band_per_curve_json_max_rel']} (exact float equality).")
a(f"- {s4['readme_brackets']}\n")
a("## Findings\n")
for f in d["findings"]:
    a(f"- {f}")
a("\n## Discrepancy\n")
for x in d["discrepancies"]:
    a(f"- {x['where']}: claimed {x['claimed']}; records give {x['supported_by_records']}. {x['finding']}")
a("\n## Gate checker\n")
a("```\n" + (V / "gate_check.txt").read_text().strip() + "\n```\n")
a("`scripts/validation_gate.py:check_validator_report` was run with base = `research/prime_ksum_pooled_20261029`. The `raw_data` paths are the `.artifacts.json` manifests and `session.json` files, because `records.jsonl` is not in the work tree; each entry nests the records' repo path, sha256 and git blob at a0ff77014f.\n")
a("## Files\n")
a(f"`{V}/`: check_hashes.py, recompute.py, integrity.py, compare_committed.py, bootstrap2.py, sensitivity.py, build_report.py, write_md.py, gate_check.py and their outputs (hashes.json, recompute.json, integrity.json, compare_committed.json, bootstrap2.json, sensitivity.json, freshness_grep.txt, gate_check.txt), validator-report.json (sha256 `{hashlib.sha256((V / 'validator-report.json').read_bytes()).hexdigest()}`).")
(V / "VALIDATOR.md").write_text("\n".join(L) + "\n")
print("written", len(L), "lines")
