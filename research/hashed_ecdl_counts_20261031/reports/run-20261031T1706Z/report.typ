#set page(margin: 1.6cm, numbering: "1")
#set text(size: 9.5pt)
#set heading(numbering: none)
#show table: set text(size: 8.5pt)

= Counted-S v2 run report: hashed-factor-base ECDL tensor backend vs matched rho

Run directory `research/hashed_ecdl_counts_20261031/reports/run-20261031T1706Z/`. Protocol v2 (`protocol.json`, sha256 prefix 8a794feb), approval DEC-20261031-77e2b3. Counted units only: no wall-time claim. Status: *unvalidated for the gate* (no `VALIDATION.json`); numbers below are copied from the recorded `results/decision.json` (sha256 prefix eb5d791a8c63), produced by the frozen `analysis/decide.py`, and were recomputed independently in `VALIDATOR.md`.

== What ran (measured)

#table(columns: (auto, auto, auto, auto, auto), align: left,
  [*session*], [*spec / plan*], [*started (UTC)*], [*binary (rebuild)*], [*replay audit*],
  [C-10-14 `ECBS1hb5ca4da84455`], [tensor-m2, ic-mitm, rho-neg, rho-neg-aa; 10/12/14-bit; 96 executions], [2026-10-31 17:06:40], [`ecbench-C` 5cec72d2..., source 4df337e, rustc 1.97.0], [96/96 verified, 0 problems],
  [B `ECBS1he2397c126e47`], [native hashed arms + ic-mitm + rho references; 4 curves; 288 records (178 verified, 110 exhausted)], [2026-10-31 17:17:16], [`ecbench-B` 603c6c86..., source 3ae1ac2bd, rustc 1.99.1 musl], [288/288 replays reproduced, 0 problems],
  [C-16 `ECBS1hab6ef45acb59`], [same arms, 16-bit; 32 executions, pinned to CPU 3], [2026-10-31 20:01:42], [`ecbench-C` 5cec72d2...], [32/32 verified, 0 problems],
)

Arms and seeds: shared per-cell seeds across arm and A/A control (determinism rule), 1 round, 8 targets per cell, L0 isolation. A/A determinism: 0 mismatches in every A/A pair (this checks reproducibility, not the noise floor). Raw records: `s3://setec-astronomy-artifacts/validation/hashed-ecdl-counts/20261031/v2/sessions/{B,C-10-14,C-16}/` (hashes match the audit receipts).

== Track C: whole-point tensor IC vs references (measured counted S)

#figure(image("graph1-trackC-ratios.svg", width: 100%),
  caption: [Ratio of counted S, tensor-m2 over rho-neg (left) and over ic-mitm (right), at 0, 0.1 and 1 GAE per unpriced native operation. Error bars: recorded 95% linearised-t intervals. Sessions `ECBS1hb5ca4da84455` and `ECBS1hab6ef45acb59`. Source: `plot.py`, data `results/decision.json`.])

#table(columns: (auto, auto, auto), align: left,
  [*curve*], [*S(tensor) / S(rho-neg), w = 0 / 0.1 / 1*], [*S(tensor) / S(ic-mitm), w = 0 / 0.1 / 1*],
  [fp10], [4318 / 5008 / 9543], [259 / 304 / 630],
  [fp12], [27039 / 30522 / 49612], [2988 / 3505 / 7241],
  [fp14], [169644 / 190459 / 301042], [21014 / 24349 / 46389],
  [fp16], [673279 / 756951 / 1206158], [121098 / 136676 / 222417],
)
The order tensor-m2 > rho-neg holds across the whole 0 to 1 GAE band on every curve (Holm family of 32 tests; largest p among the 15 complete cells 4e-5). Distance remaining to matched rho: 3.6 to 6.1 orders of magnitude. Derived (by `VALIDATOR.md`, not by `decide.py`): the tensor/rho ratio grows about 5.1x per 2 bits (log-log slope 1.17, 95% CI 1.00 to 1.34). tensor-m2's own median S grows about 4x per 2 bits; rho's median S falls from 3.1 to 1.9, which is why the ratio grows faster.

== Track B: native hashed arms (measured; native S is a lower bound)

#figure(image("graph2-trackB-cells.svg", width: 85%),
  caption: [Cell completeness and recorded ratio at 0 GAE for session `ECBS1he2397c126e47`. Green: every pair verified. Grey: exhausted executions kept as unknown. Source: `plot.py`, data `results/decision.json`.])

Complete cells: native-selectors fp10 (1306 / 1451 / 2371 x rho-neg), native-partial fp10 (496 / 551 / 902) and fp12 (1752 / 1912 / 2801), and ic-mitm on all four curves (w = 0: 14.8, 9.76, 8.53, 5.19; w = 1: 12.6 to 5.0). native-source and native-width produced no verified execution (0 of 64); the other 110 exhausted executions are budget exhaustion and are not ranked.

== Controls and checks
- Replay: `ecbench verify --replay-all` on all three sessions (above). Independent re-run and [d]G = Q on 306 verified rows: `VALIDATOR.md`.
- Positive control: rho-neg-v2 verified 32/32. A/A: identical status, seed, S and counters.
- Reference: rho.negation (prime-order curves, +-1 automorphisms). Track B also ran rho.negation_v2, up to 11% cheaper at fp16 (w = 0); no ranking changes.

== Limits and missing data
Finite-size fallback only (lambda <= 64, root_u = 0): the recursive subexponential kernel was not exercised. One round, 8 targets, power test not met, L0 isolation, C-16 on a different host class, no wall time. The original C-10-14 and B binaries no longer exist; the audited binaries are rebuilds (hashes above), and all sessions record `git_commit = null`. Provenance files: `../../provenance/`. Corrections to the first results text: `../../results/RESULTS-v2-superseding.md`.
