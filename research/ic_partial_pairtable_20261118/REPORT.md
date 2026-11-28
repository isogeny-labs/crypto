# Partial pair-table index calculus — preregistered toy study (2026-11-18)

Status: **EMPIRICAL / [pcc:unverified]**. Full raw witness outputs in the downloadable experiment archive (not yet committed to Git; see limitation below). Protocol was frozen in commit [c2abd77](https://github.com/isogeny-labs/crypto/commit/c2abd77ddb5f54e7d28f3f2e1169331b70ae9652), **before running**. Local GitHub connectivity failed, so authored scripts and compact per-seed records were subsequently committed through the GitHub connector. No formal CI or Conductor ownership receipt exists.

## Hypothesis and verdict

Frozen H1: on a public toy prime-order elliptic curve with factor base B=32, preselecting half of all unordered pair entries independently of target and logarithms reduces median cold, **single-target** group additions by >=10% versus full pair table across 24 fixed seeds, with ≥22 complete independently checked discrete-log recoveries. H1 *passes within this toy domain*: 50%/full median=**0.87051164**, paired seed bootstrap 95% interval **[0.836565, 0.894836]**, 24/24 correct. At B=16, the separate 8-seed sensitivity **reverses**: median ratio **1.11940559**. This trade is known generic advice/coverage engineering; no novel ECDLP algorithm, asymptotic improvement, real-size curve attack or native speedup is claimed.

Toy: E/F_1009, y²=x³+4x+10, #E(F_1009)=1069 prime, G=(0,162); known-answer generator and factor-base scalars. Primary B32 seed integer range 30924403185728...30924403185751. Sensitivity B16 range 30924403185744...30924403185751. Each seed includes full, 75%, 50%, 25%, and A/A full arms; shared point stream, base, target, table-pair shuffled ordering. Cold means the instrument charges factor-base construction, table build, relation walk, matrix setup, modular linear algebra, and target descent. A full sweep capped at 1069 relation attempts. No tuning after measuring.

## Phase accounting, 24 primary seeds

| arm | pairs | FB EC adds | table EC adds | relation EC adds | matrix | graph LA ops in F_1069 | descent EC adds | total EC adds | median EC IC / matched rho |
| --- | ---: | ---: | ---: | ---: | --- | ---: | ---: | ---: | ---: |
| 100% | 528 | 380.75 | 528 | 169.42 | wall_ns recorded | 369.8 | 1.04 | 1079.2 | 14.58x |
| 75% | 396 | 380.75 | 396 | 216.46 | wall_ns recorded | 383.9 | 2.17 | 995.4 | 13.37x |
| 50% | 264 | 380.75 | 264 | 290.50 | wall_ns recorded | 378.0 | 3.33 | 938.6 | **12.05x** |
| 25% | 132 | 380.75 | 132 | 550.04 | wall_ns recorded | 392.9 | 9.12 | 1071.9 | 13.09x |

Group-op costs are simple counted affine EC additions including startup. \`graph LA ops\` count modular scalar operations, **not** EC additions; matrix setup independently recorded time and dimensions. Python time is an *unisolated diagnostic* and cannot be compared with ranked Rust wall time. Primary mean relation attempts: full 158.4, 75% 205.5, 50% 279.6, 25% 539.4. Complete median half/full cost ratios at weights w = 0, 0.1, 1 modular F_r operations per group addition: **0.87051 / 0.87568 / 0.89429**. B16 ratios **1.11941 / 1.10861 / 1.03512**. Across 24 B32 seeds, 24/24 half-table group-op strict wins, 18/24 gains at least 10%; all 24 successes. Across B16, 3/8 half-table strict wins, 0/8 gains at least 10%.

The strongest locally implemented same-target rho comparator is a deterministic, seeded four-partition Pollard rho walk (startup and restarts charged; no negation folding). Its mean additions: B32 panel 82.375; B16 77.125. Full table IC/rho median **14.58x**, half IC/rho **12.05x** B32; B16 half IC/rho **7.29x**. Generic ideal \`sqrt(pi*1069/2)≈40.98\` is a heuristic reference, **not** this sampler's measured cost. All native 24/28/32-bit IC leaderboard ratios have different workloads, so they are contextual rather than directly comparable.

## Independent local validation and failures

The independent \`audit.py\` does not import the producer. It enumerates all affine curve points, proves toy cardinality by census, checks generator order, reconstructs every pair table and ordered relation stream from deterministic seeds, checks every relation using affine arithmetic, computes dense modular rank, recovers the planted target and compares rho recovery. **160/160 arms passed local audit**, 0 censored; 120 primary arms, 40 sensitivity arms (including A/A). These are **local code checks**, not independent signed PCC standing.

First run failed **for 24+8 seeds** because an A/A assertion compared wall-clock nanoseconds, which cannot be identical. Failed raw outputs and logs are preserved. Instrument repair changed only the assertion to compare deterministic scientific/cost metrics; it did not change frozen hypothesis, parameters, or seed order. Second run completed. Raw evidence includes exact rows, row SHA-256, matrix dimensions/timers, relations, ranks, source and summaries; \`raw/paired_costs_compact.csv\` in this PR is the committed 32-seed subset. Full raw JSON, command/error logs, paired cost CSV and source hashes are supplied in the separate attached experiment archive; GitHub DNS prevented native checkout/push and the full raw JSON has **not** been uploaded to this PR, a validation/publication blocker. Do not claim that the compact CSV suffices for independent reconstruction of raw runs; rerunning \`run.py\` is required.

## Context / novelty boundary

- [Semaev 2004](https://eprint.iacr.org/2004/031): summation-polynomial IC; this study changes no algebraic solver.
- [Shoup 1997](https://www.shoup.net/papers/dlbounds1.pdf): generic DLP lower bounds; selective advice is not a generic lower-bound break.
- [IC mega pass closure](../../research/ic_mega_pass_20261031/brief/BRIEF.md), [lead-atlas prime-field survey](../../research/prime_field_lead_assessment_20261107/atlas_survey.md), [atlas delta](../../research/prime_ic_lead_heatmap_20261114/ATLAS_DELTA.md): generic table decomposition already known; factor-base yield conservation forecloses claiming a new exponent on the basis of this toy test.
- [Native prime IC frontier](../../research/prime_ic_collection_frontier_20261117/README.md): pair table O(B²) construction competes with O(p/B²) query collection, dense LA; native batched code—not this Python toy—is the proper engineering baseline. The previous R1 graph-elimination study failed its frozen ≥4x LA target despite a component gain and did not improve end-to-end group additions.

## Reproduction and remaining limitations

\`\`\`sh
python3 run.py --out raw/primary.json --base 32 --seeds 24 --seed0 0x1c2026111840
python3 run.py --out raw/sensitivity.json --base 16 --seeds 8 --seed0 0x1c2026111850
python3 audit.py raw/primary.json raw/sensitivity.json
python3 summarize.py
\`\`\`

The required \`/workspace/isogeny-workspace\` was absent; a **named offline subset**, not the official trunk, ran \`bin/ws route\` then \`bin/ws need crypto --add research/ic_partial_pairtable_20261118/\` (blobless, depth-1, no recurse), which failed on github.com DNS. Official \`isogeny-labs/workspace\` \`AGENTS.md\`, bin/ws, and crypto/cryptanalysis AGENTS were read via GitHub connector (crypto-autoresearcher AGENTS decode failed). Conductor and research-kb servers were absent; atlas and prior results were used, with no claimed coordination ownership. No native Rust tests, per-host isolation, CPU/NUMA/occupancy calibration, or PCC claims were performed. Atlas encrypted and not edited. Shared research-control record \`EXP-IC-PARTIAL-TABLE-20261118-R2\` and next experiment queue \`EXP-IC-PARTIAL-PAIR-NATIVE-20261118-01\` were updated separately (revision 1); none is verified.

**Next discriminating experiment (queued):** on official sparse workspace + Conductor claim, implement a half-table knob in native \`prime_fast\` batched IC, preregister paired 24/28-bit cells near B≈p^(1/3) and stronger rho, run independent replay with A/A, recorded cold phase counts, and w=0/0.1/1 GAE. Halt promotion if improvement reverses, rank/descent fails, or independent validation is missing.
