#!/usr/bin/env python3
"""Lay out summary.json as README.md tables and report.typ (no computation beyond formatting). cwd = study directory."""
import json
d = json.load(open("summary.json")); V = d["verdict"]; R = d["rows"]; W = ["0", "0.1", "1"]
def tab(k):
    h = "| row | w | S ksum%d | S ksum%d-p | ratio p/u [95%%] | per replicate | A/A | gap to rho: u | gap to rho: p |\n|:--|--:|--:|--:|:--|:--|--:|--:|--:|\n" % (k, k)
    for r in R:
        for w in W:
            x = r["weights"][w]; c = x["ksum%d" % k]
            h += "| %s | %s | %.3f | %.3f | **%.4f** [%.4f, %.4f] | %s | %.4f | %.2f | %.2f |\n" % (r["id"] if w == "0" else "", w, c["s_unpruned"], c["s_pruned"], c["ratio"], c["ci95"][0], c["ci95"][1], " / ".join("%.4f" % q for q in c["per_replicate"]), x["aa_ratio"], c["gap_to_rho_unpruned"], c["gap_to_rho_pruned"])
    return h
def sv():
    h = "| row | build adds saved k5 | build adds saved k4 | k5 max upper (pooled / replicate) | k5 holds (A/A gate) | k5 holds (sibling delta) | k4 max upper | k4 holds (A/A gate) | k4 holds (sibling delta) | delta (sibling) k5 |\n|:--|--:|--:|:--|:--|:--|:--|:--|:--|--:|\n"
    for r in R:
        a, b = r["ksum5"], r["ksum4"]
        h += "| %s | %.1f%% | %.1f%% | %.4f / %.4f | %s | %s | %.4f / %.4f | %s | %s | %.3f |\n" % (r["id"], 100 * r["build_adds"]["ksum5"]["saved_fraction"], 100 * r["build_adds"]["ksum4"]["saved_fraction"], a["max_upper_pooled"], a["max_upper_replicate"], a["holds"], a["holds_sibling_delta"], b["max_upper_pooled"], b["max_upper_replicate"], b["holds"], b["holds_sibling_delta"], a["delta_sibling"])
    return h
def short(k):
    h = "| row | field, bits | R_0 | R_0.1 | R_1 | S(k%d-p)/S(rho) at w=0 / 0.1 / 1 |\n|:--|:--|:--|:--|:--|:--|\n" % k
    for r in R:
        c = [r["weights"][w]["ksum%d" % k] for w in W]
        h += "| %s | %s, %d | %.3f [%.3f, %.3f] | %.3f [%.3f, %.3f] | %.3f [%.3f, %.3f] | %s |\n" % (r["id"], r["field"], r["bits"], *[v for x in c for v in (x["ratio"], *x["ci95"])], " / ".join("%.2f" % x["gap_to_rho_pruned"] for x in c))
    return h
rng = lambda k, w: (min(r["weights"][w]["ksum%d" % k]["ratio"] for r in R), max(r["weights"][w]["ksum%d" % k]["ratio"] for r in R))
head = f"""**Standing: counted cost at L0, `[pcc:unverified]`** (no pcc claim filed; no second-host replay, independent validator or `VALIDATION.json`: the validation gate is not met). Wall time is not claimed. Track: generic decomposition (bounded by sqrt(r); cannot beat rho or BSGS).

## Outcome

- **`prune=1` lowers the counted cost of the k-sum arm on every one of the 8 rows, at all three weights of the section 5a band, pooled and in each replicate.** `ksum5-p / ksum5`: w = 0 {rng(5,'0')[0]:.3f}-{rng(5,'0')[1]:.3f}, w = 0.1 {rng(5,'0.1')[0]:.3f}-{rng(5,'0.1')[1]:.3f}, w = 1 {rng(5,'1')[0]:.3f}-{rng(5,'1')[1]:.3f}. `ksum4-p / ksum4`: w = 0 {rng(4,'0')[0]:.3f}-{rng(4,'0')[1]:.3f}, w = 0.1 {rng(4,'0.1')[0]:.3f}-{rng(4,'0.1')[1]:.3f}, w = 1 {rng(4,'1')[0]:.3f}-{rng(4,'1')[1]:.3f}. The largest 95% upper end of any pooled ratio is {max(r['ksum5']['max_upper_pooled'] for r in R):.4f} (k = 5) and {max(r['ksum4']['max_upper_pooled'] for r in R):.4f} (k = 4); of any replicate ratio {max(r['ksum5']['max_upper_replicate'] for r in R):.4f} and {max(r['ksum4']['max_upper_replicate'] for r in R):.4f}.
- **Correctness: no mismatch.** {V['correctness_pairs_checked']} per-target pairs (24 sessions x 2 k x 192 runs incl. warm-up): the `-p` arm recovered the same log, with identical counters (relations, targets tried, matrix rank/rows/work), identical `native` and `work_native` of every phase (so identical relation, trial, lookup, probe and `table_insert_attempts` counts), identical factor base and identical cost in every phase except `oracle_setup`, whose adds never increased. The record does not expose `ksum_table_entries`; insert attempts and hash probes are the proxy. {V['sessions_verified']}/24 sessions pass `ecbench verify --replay-all --exit-code` (27,648 records verified, 20,736 replays reproduced, 0 problems; same binary, same host).
- **The gain is larger than the director's prediction** (about 5% at k = 5, w = 0): it is 6-17% at w = 0, because the saved build adds are {100*min(r['build_adds']['ksum5']['saved_fraction'] for r in R):.0f}-{100*max(r['build_adds']['ksum5']['saved_fraction'] for r in R):.0f}% of the k = 5 build here (commit message: 10-12%) and {100*min(r['build_adds']['ksum4']['saved_fraction'] for r in R):.0f}-{100*max(r['build_adds']['ksum4']['saved_fraction'] for r in R):.0f}% at k = 4 (message: 3%), so the build adds are a larger share of S than the director assumed (build share of S not measured here). The prediction `R_0(ksum5) = 0.95 +/- 0.03` held on {len(V['prediction_ksum5_r0_0p95_pm_0p03_held_rows'])} of 8 rows ({', '.join(V['prediction_ksum5_r0_0p95_pm_0p03_held_rows'])}); the gain is smaller at w = 1 (native work, unchanged) as predicted.
- **Noise gates.** The paired A/A gate (verdict) holds on all rows for both k: `rho-neg-aa / rho-neg` is exactly 1.0000 on every row and weight (same code, same seeds, deterministic count), so the gate reduces to the upper end lying below 1. The sibling delta gate (largest max/min ratio across replicate sessions of one arm's mean S, which compares *different seeds* and does not enter a paired ratio) holds for `ksum5` on {', '.join(V['ksum5_p_holds_sibling_delta_rows'])} and for `ksum4` on {', '.join(V['ksum4_p_holds_sibling_delta_rows'])}, and fails on the rows where seed spread (delta up to {max(r['ksum5']['delta_sibling'] for r in R):.3f}) exceeds the gain. Both are reported; the registered verdict is the first.
- **Distance remaining.** Matched negation rho is still ahead on every row: `ksum5-p` costs {min(r['weights']['0']['ksum5']['gap_to_rho_pruned'] for r in R):.2f}-{max(r['weights']['0']['ksum5']['gap_to_rho_pruned'] for r in R):.2f}x rho at w = 0 (unpruned {min(r['weights']['0']['ksum5']['gap_to_rho_unpruned'] for r in R):.2f}-{max(r['weights']['0']['ksum5']['gap_to_rho_unpruned'] for r in R):.2f}x), {min(r['weights']['1']['ksum5']['gap_to_rho_pruned'] for r in R):.2f}-{max(r['weights']['1']['ksum5']['gap_to_rho_pruned'] for r in R):.2f}x at w = 1. `ecbench band --ranks`: rho-neg below every k-sum arm on every row at every weight. This is a measured improvement over the unpruned k-sum baseline, not a result against rho (the long-term goal is a gap below 1).
- **Not an accounting change:** the removed work is group additions of the table build that the unpruned build performed and never used (correctness check: nothing else moved). Both arms are priced by the same harness pricing.
"""
open("README.md", "w").write("# ksum `prune=1` against `prune=0` on Table B's eight curves\n\n" + head + f"""
Study: `research/ic_ksum_prune_20261115/` (PROTOCOL.md registered in commit fff219f63f before any data; binary sha256 `{d['binary_sha256']}`). Twin of `research/ic_tableb_ksum_20261115` and `research/ic_tableb_binary_ksum_20261115`: same curves, seeds, targets (48 per curve), 3 rounds, 1 warm-up, factor-base sizes; arms `rho-neg`, `rho-neg-aa`, `ksum4`, `ksum4-p`, `ksum5`, `ksum5-p`.

## ksum5-p / ksum5 (S: counted GAE per sqrt(r), mean over 3 sessions; ratio from `ecbench compare`, 4,000 resamples, seed 20261002)

""" + tab(5) + "\n## ksum4-p / ksum4\n\n" + tab(4) + "\n## Registered rule per row\n\n" + sv() + f"""
Verdicts (`summary.json`): `ksum5_p_holds_all_rows` = {V['ksum5_p_holds_all_rows']}, `ksum4_p_holds_all_rows` = {V['ksum4_p_holds_all_rows']}, `correctness_ok` = {V['correctness_ok']}, `all_sessions_verified` = {V['all_sessions_verified']}.

## Deviations and limits

- Route: local, `--cpus none`, L0, counted cost only, instead of the isolab-eks default (as both siblings, at the director's instruction). Replay on the same host with the same binary, not a second host class.
- The two siblings' arms `inc-*`, `bsgs-neg` are not run; this study compares prune against no prune only.
- Warm-up round records are included in the correctness check but not in the ratios (ecbench excludes warm-up).
- `ksum_table_entries` is not in the record; correctness rests on identical insert attempts, hash probes, relation/trial/lookup counts and recovered logs.
- The `.jsonl` records of the 24 sessions are kept gzipped as the siblings do (`gunzip` before `ecbench band/compare/verify`).
- Table B rows, not new curves; the factor-base sizes were frozen from the siblings and not re-tuned for the pruned build (a smaller-base re-tune could move the optimum; not tested).
""")
open("report.typ", "w").write(f"""#set page(paper: "a4", margin: 1.6cm)
#set text(size: 9pt)
#set heading(numbering: "1.")
#show table: set text(size: 7pt)
#show raw: set text(size: 7.5pt)

= `ksum` `prune=1` against `prune=0` on Table B's eight curves

*Study:* `research/ic_ksum_prune_20261115/` (PROTOCOL.md registered in commit fff219f63f before any data). *Record:* `benchmarks/records/20261115T2211Z-ksum-prune-tableb.json`. *Binary:* `{d['binary_sha256'][:16]}...`.

""" + head.replace("## Outcome", "== Outcome").replace("**", "*").replace("`[pcc:unverified]`", "`[pcc:unverified]`") + f"""
#figure(image("ratios.svg", width: 100%), caption: [`cost(prune=1) / cost(prune=0)`, paired by target and round, per row and weight (square: pooled; bar: 95% interval; circle: replicate session). `[pcc:unverified]`])

== k = 5

""" + "\n".join("" for _ in range(0)) + "#table(columns: 7, align: (left, right, left, left, left, left, left),\n  [*row*], [*w*], [*S k5*], [*S k5-p*], [*ratio p/u [95%]*], [*per replicate*], [*k5-p / rho*],\n" + "".join(
    "  [%s], [%s], [%.3f], [%.3f], [*%.4f* [%.4f, %.4f]], [%s], [%.2f],\n" % (r["id"] if w == "0" else "", w, c["s_unpruned"], c["s_pruned"], c["ratio"], c["ci95"][0], c["ci95"][1], " / ".join("%.3f" % q for q in c["per_replicate"]), c["gap_to_rho_pruned"])
    for r in R for w in W for c in [r["weights"][w]["ksum5"]]) + ")\n\n== k = 4\n\n" + "#table(columns: 7, align: (left, right, left, left, left, left, left),\n  [*row*], [*w*], [*S k4*], [*S k4-p*], [*ratio p/u [95%]*], [*per replicate*], [*k4-p / rho*],\n" + "".join(
    "  [%s], [%s], [%.3f], [%.3f], [*%.4f* [%.4f, %.4f]], [%s], [%.2f],\n" % (r["id"] if w == "0" else "", w, c["s_unpruned"], c["s_pruned"], c["ratio"], c["ci95"][0], c["ci95"][1], " / ".join("%.3f" % q for q in c["per_replicate"]), c["gap_to_rho_pruned"])
    for r in R for w in W for c in [r["weights"][w]["ksum4"]]) + ")\n\n== Registered rule per row\n\n" + "#table(columns: 10, align: left,\n  [*row*], [*saved k5*], [*saved k4*], [*k5 max upper (pool / rep)*], [*k5 A/A gate*], [*k5 delta gate*], [*k4 max upper*], [*k4 A/A gate*], [*k4 delta gate*], [*delta k5*],\n" + "".join(
    "  [%s], [%.1f%%], [%.1f%%], [%.4f / %.4f], [%s], [%s], [%.4f / %.4f], [%s], [%s], [%.3f],\n" % (r["id"], 100 * r["build_adds"]["ksum5"]["saved_fraction"], 100 * r["build_adds"]["ksum4"]["saved_fraction"], r["ksum5"]["max_upper_pooled"], r["ksum5"]["max_upper_replicate"], r["ksum5"]["holds"], r["ksum5"]["holds_sibling_delta"], r["ksum4"]["max_upper_pooled"], r["ksum4"]["max_upper_replicate"], r["ksum4"]["holds"], r["ksum4"]["holds_sibling_delta"], r["ksum5"]["delta_sibling"]) for r in R) + ")\n\n== Deviations and limits\n\n" + "\n".join("- " + l[2:] for l in open("README.md").read().split("## Deviations and limits")[1].strip().split("\n") if l.startswith("- ")) + "\n")
