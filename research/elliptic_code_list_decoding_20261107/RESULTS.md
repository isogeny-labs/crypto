# RESULTS: elliptic-code list decoding as a relation finder, R2 panel (slug r2ecldan)

**Verdict (derived by the PROTOCOL section 7 rule, applied mechanically):** **UNRESOLVED.** `[pcc:unverified]`
P1 holds; D1 costs more than both E1 and E2 per verified relation in every cell with k >= 3 at w = 0.1 and w = 1 (measured), but P4 (slope within 0.25/0.4 of k-1) fails in all four k, and P3's factor-10 clause fails in two cells; the ENUMERATION-IN-DISGUISE branch needs P4, so it is not reached, and BELOW ENUMERATION is not met (D1 is never cheaper). No claim about the Zhang-Liu procedure. This is a toy relation-phase measurement, not a leaderboard row.

Evidence classes: measured = read from `panel.jsonl`; derived = computed from measured rows by `r2ecldan_analyze.py`; every number is `[pcc:unverified]`.

## 0. Execution vs `ecld_jobs.txt` (reported first)

- PROTOCOL.md sha256 = e8481f7c...ac4465 (matches the frozen value; verified before analysis). `ecld_toy.rs` sha256 de901e23... and `ecld_jobs.txt` sha256 d3bef30a... equal the values in `binary.sha256`/the freeze (measured).
- `sha256sum -c SHA256SUMS`: 132/132 OK, re-run after the coordinator's note that panel.jsonl was briefly absent (measured). My analysis read 696 rows; `sort lines/*.jsonl` and `sort panel.jsonl` have identical sha256 (measured).
- 64 job lines in ecld_jobs.txt (56 curve jobs + 8 per-target jobs for cell 5,256), 64 `line_NN.jsonl` (all non-empty), 64 `.err` files, all 0 bytes, progress.txt: `lines 64 fail 0`, every line exit 0 (measured).
- Rows: 696 = 232 instances x 3 arms. Expected from `cells_full`: 14 cells x 16 + 1 cell (5,256) x 8 = 232. Missing: none. Extra: none. Deviation from ecld_jobs.txt: **none found** (measured).
- Host: 2 vCPU AMD EPYC 9R45 pod (nproc 2), jobs run sequentially by the entrypoint; protocol said one thread per job, no co-running panel jobs (the entrypoint's concurrency not audited here). Wall time is a practicality note only.

## P1 and controls (measured)

| check | result |
|---|---|
| E1=E2=D1 as sets (`sets_consistent_across_arms`) | 232/232 instances |
| verified == relations, all arms | 232/232 |
| planted control recovered by all 3 arms | 58/58 planted instances |
| A/A `aa_e1_identical` (k<=4) | 144/144 true; k=5,6 null by design (88/88) |
| D1 gae == 0 | violations: 0 |
| negative-control cells (k=2 all B, k=3 B=64): arms agree where enumeration finds none | arms agree on all instances (consistent 232/232). Random-target relations observed: (2, 64): 0 (exp 0.38); (2, 128): 0 (exp 1.53); (2, 256): 6 (exp 6.25); (3, 64): 11 (exp 7.96). Planted instances give the nonzero counts (not the null set); the control is about random targets. |

**P1: HOLDS.** Random-word control: deferred in the protocol, not run (stated gap).

## P2: D1 gae = 0 and native count within 25% of plan (measured vs plan)

| k,B | n inst | D1 native / instance (mean) | plan | ratio | within 25% |
|---|---|---|---|---|---|
| 2,64 | 16 | 6432 | 6368 | 1.0101 | yes |
| 2,128 | 16 | 2.515e+04 | 2.502e+04 | 1.0053 | yes |
| 2,256 | 16 | 9.946e+04 | 9.92e+04 | 1.0026 | yes |
| 3,64 | 16 | 2.468e+05 | 2.446e+05 | 1.0090 | yes |
| 3,128 | 16 | 1.862e+06 | 1.853e+06 | 1.0047 | yes |
| 3,256 | 16 | 1.444e+07 | 1.441e+07 | 1.0020 | yes |
| 4,64 | 16 | 6.573e+06 | 6.323e+06 | 1.0395 | yes |
| 4,128 | 16 | 9.209e+07 | 9.004e+07 | 1.0227 | yes |
| 4,256 | 16 | 1.364e+09 | 1.348e+09 | 1.0122 | yes |
| 5,64 | 16 | 1.36e+08 | 1.271e+08 | 1.0698 | yes |
| 5,128 | 16 | 3.512e+09 | 3.363e+09 | 1.0443 | yes |
| 5,256 | 8 | 9.781e+10 | 9.537e+10 | 1.0256 | yes |
| 6,32 | 16 | 4.823e+07 | 4.32e+07 | 1.1164 | yes |
| 6,48 | 16 | 4.603e+08 | 4.175e+08 | 1.1025 | yes |
| 6,64 | 16 | 2.273e+09 | 2.083e+09 | 1.0914 | yes |

**P2: HOLDS** (D1 gae = 0 on all 232 instances; max ratio 1.116 at k=6,B=32).

## P3: priced cost, D1 against E1 and E2, k >= 3 (derived: cell totals over all instances)

Columns: D1/E1 and D1/E2 priced-cost ratios at w=0.1 and w=1; w* = E gae / D1 native (protocol definition) vs E1 and E2. Factor-10 clause applies at k >= 4 (all four ratios must be >= 10).

| k,B | D1/E1 w.1 | D1/E1 w1 | D1/E2 w.1 | D1/E2 w1 | w* (E1) | w* (E2) | D1 > both | w*<0.1 | >=10x (k>=4) |
|---|---|---|---|---|---|---|---|---|---|
| 3,64 | 5.82 | 40.38 | 5.8 | 40.4 | 0.0163 | 0.01634 | True | True | None |
| 3,128 | 10.90 | 75.95 | 10.9 | 75.9 | 0.0087 | 0.00873 | True | True | None |
| 3,256 | 21.06 | 147.07 | 21.1 | 147.1 | 0.0045 | 0.00452 | True | True | None |
| 4,64 | 7.34 | 51.72 | 101.9 | 652.1 | 0.0130 | 0.00092 | True | True | False |
| 4,128 | 12.70 | 89.20 | 354.0 | 2265.9 | 0.0075 | 0.00026 | True | True | True |
| 4,256 | 23.38 | 163.93 | 1306.4 | 8360.7 | 0.0041 | 0.00007 | True | True | True |
| 5,64 | 9.87 | 69.73 | 148.2 | 1037.6 | 0.0097 | 0.00064 | True | True | False |
| 5,128 | 15.44 | 108.55 | 478.5 | 3349.4 | 0.0062 | 0.00020 | True | True | True |
| 5,256 | 26.45 | 185.54 | 1665.8 | 11660.4 | 0.0036 | 0.00006 | True | True | True |
| 6,32 | 10.39 | 74.71 | 286.0 | 1869.9 | 0.0092 | 0.00033 | True | True | True |
| 6,48 | 12.09 | 86.04 | 799.1 | 5187.5 | 0.0079 | 0.00012 | True | True | True |
| 6,64 | 13.62 | 96.52 | 1655.1 | 10705.6 | 0.0070 | 0.00006 | True | True | True |

**P3: FAILS as written.** The ordering clause and w* < 0.1 hold in all 12 cells. The factor-10 clause fails at (4,64) (D1/E1 = 7.34 at w=0.1) and (5,64) (9.87 at w=0.1); at w=1 all are >= 10. E1 is the binding comparator; against E2 the factor is >= 101.9 everywhere. No threshold changed.

## P4: slope of D1 native count vs B (derived)

| k | B range | slope (endpoints) | LS slope over all B | pre-specified k-1 | tolerance | holds |
|---|---|---|---|---|---|---|
| 3 | 64-256 | 2.935 | 2.935 | 2 | 0.25 | NO |
| 4 | 64-256 | 3.849 | 3.849 | 3 | 0.25 | NO |
| 5 | 64-256 | 4.745 | 4.745 | 4 | 0.25 | NO |
| 6 | 32-64 | 5.559 | 5.559 | 5 | 0.4 | NO |

**P4: FAILS in all four k** (slopes 2.94, 3.85, 4.75, 5.56 against 2, 3, 4, 5). Heuristic observation, not a re-binning: the protocol's own P2 cost formula C(B,k-1) x [(B-k+1)(2k-1)/k + ...] contains a factor (B-k+1), so it scales as B^k, and the measured slopes follow that (they sit below k only through lower-order terms). The prediction text 'B^(k-1)' is inconsistent with the protocol's own formula; the prediction is nonetheless scored as written. C3 (D1 grows as B^(k-1)) is therefore not supported; the observed growth is about B^k.

## P5: relations per random target vs C(B,k)/N (measured vs derived)

Pooled over random targets (3 per curve) using each curve's own N. Evaluable rule: expected total >= 10; ratio within [0.5, 2]. Below 10 the protocol checks 'direction only' with no numeric rule, so those cells are listed, not scored.

| k,B | observed verified (random) | expected | ratio | evaluable | holds |
|---|---|---|---|---|---|
| 2,64 | 0 | 0.4 | 0.00 | False | not evaluable (direction only) |
| 2,128 | 0 | 1.5 | 0.00 | False | not evaluable (direction only) |
| 2,256 | 6 | 6.2 | 0.96 | False | not evaluable (direction only) |
| 3,64 | 11 | 8.0 | 1.38 | False | not evaluable (direction only) |
| 3,128 | 88 | 65.0 | 1.35 | True | True |
| 3,256 | 509 | 528.4 | 0.96 | True | True |
| 4,64 | 122 | 118.8 | 1.03 | True | True |
| 4,128 | 2044 | 2010.7 | 1.02 | True | True |
| 4,256 | 33657 | 33653.1 | 1.00 | True | True |
| 5,64 | 1513 | 1458.8 | 1.04 | True | True |
| 5,128 | 50737 | 50820.3 | 1.00 | True | True |
| 5,256 | 843199 | 843442.5 | 1.00 | True | True |
| 6,32 | 185 | 172.7 | 1.07 | True | True |
| 6,48 | 2347 | 2367.9 | 0.99 | True | True |
| 6,64 | 14386 | 14298.2 | 1.01 | True | True |

**P5: HOLDS** on the 11 evaluable cells (ratios 0.96-1.35, mostly 0.96-1.07); (2,64), (2,128), (2,256), (3,64) have expected < 10 and are not evaluable by the numeric rule (observed 0, 0, 6, 11 against 0.4, 1.5, 6.2, 8.0).

## Decision rule (PROTOCOL section 7)

- P1 holds: yes. Cells with k >= 3 and expected relations >= 1 per target: 3,128, 3,256, 4,64, 4,128, 4,256, 5,64, 5,128, 5,256, 6,32, 6,48, 6,64. In each, D1 cost per verified relation > E2's at w = 0.1 and 1: True (also > E1's in all 12 k>=3 cells, measured).
- Slope of P4 holds: **no**. So ENUMERATION IN DISGUISE is not reached by the rule.
- BELOW ENUMERATION (D1 cheaper than both in any k>=3 cell at both w): no cell.
- **Outcome: UNRESOLVED** (pattern 'any other', here driven by P4, with the cost ordering favouring enumeration everywhere). No win for either side is declared by the rule. Derived summary only: nothing in these rows shows D1 cheaper than enumeration; the formal discriminator is not satisfied because the slope clause failed.

## Cost / operation-count figure

![fig1](r2ecldan_fig1.png)

SVG: `results/R2/analysis/r2ecldan_fig1.svg`. Left: priced cost per instance at w=0.1 (derived). Middle: D1 native count vs B with fitted slopes. Right: relation yield, observed vs expected.

Relation-phase distance to rho is not computed here (protocol: toy, no replay certificate/validator/matched measured rho; not a leaderboard row).

## experiment.export/v1

Not assembled. `tools/experiment_protocol.py` (crypto-autoresearcher) `from-rows`/`check-export` need a machine-readable experiment.protocol/v1 JSON; PROTOCOL.md states that record is for the experiment director to produce and none exists in the study dir. `from-ecbench`/`from-taskq` do not fit this command-job output. Skipped, not forced.

## Provenance of this analysis

AGENTS.md forbids Python for research result generation, so these scripts are not committed; they live in the scratchpad (used only to tabulate/plot recorded values; a native re-derivation would be needed before any promotion). sha256:

- `r2ecldan_analyze.py` 54f11359b2f8e7a5af40ea4631af3997c601544e8ec5ffc05213f208938b6589
- `r2ecldan_fig.py` 908bc2dce3111f91a05081166a9b52a26256cabba5f4ad9dee2315ce6b585ed8
- `r2ecldan_md.py` (this generator; hash not self-referential)
- `results/R2/analysis/r2ecldan_summary.json` b094e2e0dca7dafaa0347767fd6c5148fc84cb47f8a5b65e9a1aaf4c2bb5e8de
- Raw: `results/R2/c24ac183-0e85-4a80-a46b-b51eb4db180d/panel.jsonl` (SHA256SUMS verified).

Sources checked: research-kb and Conductor are not installed in this environment; none queried.

PDF: built with matplotlib's PDF backend (no typst, no LaTeX; pandoc present but no PDF engine).