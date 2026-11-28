# F1 — cold folded-constant SAT solve with warm heuristic state: NO-CHANGE for transferred phases

Protocol: `PROTOCOL.md`, frozen 2026-11-16T01:09Z (sha256 `cee8ccfb…452b` in `PROTOCOL.sha256`)
before any scored row, with Amendment 1 (p3 rerun, C4 dropped). Every number below is
`[pcc:unverified]`. Raw rows: `raw/` (`SHA256SUMS`). Analysis: `tool/src/bin/analyze.rs` writes
`results.json`, `raw/analysis_table.md` and `figures/`.

## Verdict (section 5, C3 = m=23, k=2, w=2, 79 positions, 78 of them non-decomposable)

- **Primary WP/A0 conflicts: 1.006 [0.986, 1.025] → NO-CHANGE** `[pcc:unverified]`.
  Seeding a cold folded solve with the previous target's saved phases does not change its cost.
- **The positive control failed in both cells:** WS/A0 is 0.977 [0.953, 1.001] at C1 and
  0.992 [0.971, 1.013] at C3. Under the protocol, the verdict therefore carries this note: the
  instrument does not see phase seeding even when the phases come from the same target.
  The phase hook itself works: in the selftest, seeding a model gives a first call with 0 conflicts.
  So saved phases matter little for a complete enumeration that ends in a refutation.
- **Attribution:** WP/WR = 1.012 [0.992, 1.033]. Transferred phases do no better than random phases.
- **Not pre-registered as primary:** WL is the arm that carries target-free learnt clauses. It
  beats A0 in both cells, in conflicts and in propagations:
  - C3: WL/A0 0.903 [0.883, 0.922]; WL/WT 0.897 [0.880, 0.914]; propagations 0.852 [0.833, 0.871].
  - C1: WL/A0 0.894 [0.871, 0.918].
  This is learnt-clause reuse, not heuristic state. It falsifies prediction P5.

## C3 table (paired geometric mean of (c_X+1)/(c_Y+1), 95% bootstrap CI, n=79)

| pair | conflicts | propagations | role |
|---|---|---|---|
| WP/A0 | 1.006 [0.986, 1.025] | 1.002 [0.983, 1.020] | **primary** |
| AA/A0 | 0.989 [0.970, 1.009] | 0.989 [0.971, 1.008] | A/A |
| WS/A0 | 0.992 [0.971, 1.013] | 0.992 [0.973, 1.012] | positive control (fails) |
| WR/A0 | 0.994 [0.977, 1.011] | 0.992 [0.976, 1.009] | negative control |
| WP/WR | 1.012 [0.992, 1.033] | 1.009 [0.989, 1.030] | phase transfer vs random |
| WF/A0 | 0.960 [0.944, 0.976] | 0.988 [0.972, 1.006] | forced `phase()` API |
| WT/WP | 1.000 (identical rows) | 1.000 | LRAT tracer control |
| WL/A0 | 0.903 [0.883, 0.922] | 0.852 [0.833, 0.871] | + target-free learnts |
| WL/WT | 0.897 [0.880, 0.914] | 0.850 [0.832, 0.868] | learnt-clause effect |
| B/A0 | 0.867 [0.819, 0.918] | 1.882 [1.772, 2.003] | L3 persistent reference |

Mean per target at C3: A0 73,529 conflicts and 1.835e7 propagations; WP 73,964; WL 66,337 and
1.565e7; B 66,402 and 3.622e7. C1 (m=11, 79 positions, 19 decomposable):

| pair | conflicts |
|---|---|
| WP/A0 | 0.983 [0.960, 1.007] |
| AA/A0 | 1.009 [0.985, 1.030] |
| WR/A0 | 0.986 [0.962, 1.011] |
| WF/A0 | 0.997 [0.971, 1.021] |
| WL/WT | 0.909 [0.885, 0.934] |
| B/A0 | 0.795 [0.764, 0.828] |

Figures: `figures/fig_ratios.svg` (forest plot), `figures/fig_phase_agreement.svg`.

## Controls and integrity
- **Correctness:** 1,440 rows (C1 720, C3 720, positions 0–79) all enumerate to completion. Every model
  set equals brute force. There are 0 censored targets and 0 mismatches.
- **Same CNF:** the folded encoding is checked against A0's CNF on every target, with the same
  variable numbering. The target-independent core clause set (6,033 clauses at C3) is the same
  for every target.
- **Determinism:**
  - At position 0, every W arm equals A0, because it has nothing to seed.
  - The p3 rerun reproduces 124 of 124 rows of the stopped run.
- **Replication of L3:** A0, AA and B are identical to L3's raw rows on positions 0–39, for C1 and C3.
- **Carried clauses:** each clause WL carries was derived, through its LRAT antecedent chain, from
  core clauses only. A random sample was rechecked as implied by the core alone:
  - checked: 2,193 at C1 and 2,394 at C3, with 0 failures;
  - WL's model sets match brute force on the 19 decomposable C1 targets.
- **WT = WP:** attaching the LRAT tracer leaves the search unchanged in every row. WL/WT therefore
  isolates the carried clauses. At C3, 11% of derived clauses are target-free, and the carried set
  grows to 28,507 clauses.

## Is the phase transfer meaningful? (contrarian check)
The table shows the mean fraction of variables with equal phase, C3 / C1, by variable class.

| comparison | core vars | R-dependent vars | all |
|---|---|---|---|
| WP seed vs this target's own A0 final phases | 0.899 / 0.774 | 0.574 / 0.568 | 0.844 / 0.733 |
| A0 final, previous target vs this target | 0.890 / 0.776 | 0.562 / 0.556 | 0.835 / 0.733 |
| AA vs A0 final phases, same target (A/A) | 0.902 / 0.772 | 0.582 / 0.562 | 0.816 / 0.714 |
| WR seed vs A0 final phases (null) | 0.501 / 0.502 | 0.505 / 0.505 | 0.502 / 0.503 |

The transferred phases match a target's own final phases about as well as a second solve of the
same target does (A/A), and far above random. Yet WP, WR and WS cost the same. The high
core agreement reflects gate-polarity base rates (most AND/counter outputs end at 0). The phases
on the R-dependent part are near 0.56 under every comparison. The Pearson correlation between seed
agreement and log(WP/A0) is 0.15 at C3 and −0.17 at C1, which is no usable signal.

## Predictions
- P1 (NO-CHANGE at C3): held.
- P2 (WS < 1 at C1): not met. The CI upper bound is 1.001.
- P3 (WF > 1): falsified. WF/A0 is 0.960 [0.944, 0.976] at C3 and 0.997 at C1. Forced polarity
  costs nothing here, and it helps a little at C3.
- P4 (WR contains 1): held.
- P5 (WL/WT contains 1): falsified, at about −10% in both cells.
- P6: held, but read the agreement table first: core agreement is 0.77–0.90 and R-dependent
  agreement is about 0.56.

## Post-hoc observations, not pre-registered
- **The stream half matters for B.**
  - B/A0 is 1.02 on positions 1–39, which is exactly L3's result, and 0.74 on positions 40–79.
  - WL/A0 is 0.939 and then 0.868; WF/A0 is 0.98 and then 0.94.
  - The persistent and carry arms keep improving as the stream lengthens. L3's B NO-CHANGE was a
    40-target result.
  - B's propagations stay at 1.88× A0.
- On the 19 decomposable C1 targets, the first-call conflicts give WS/A0 0.64 and WP/A0 0.76
  (WR 0.97). Phases do help find the first model. The full enumeration, which ends in a
  refutation, dilutes this.

## §5a band and distance to rho
Per-target propagations at C3, priced at 0 / 0.1 / 1 GAE:
- A0: 0 / 1.84e6 / 1.84e7
- WL: 0 / 1.56e6 / 1.56e7
- B: 0 / 3.62e6 / 3.62e7

WL ranks ahead of A0 at both nonzero prices, and B ranks behind. At 0 GAE all arms tie.
Conflicts have no calibrated GAE price. This lane measures only the decomposition oracle: it has
no relation-yield or linear-algebra phase, so there is no end-to-end figure and no gap to matched
rho. Wall time is a note only, on a host shared with lanes F2 and F3: A0 6.0 s, WL 4.3 s, B 16.3 s
per target at C3.

## Deviations
- **Patched bridge.** cadical-sys 0.8.0 has no saved-phase getter and no seeder. The lane builds
  against a copy whose cxx bridge adds two functions through CaDiCaL's own `Testing` accessor
  (`tool/cadical-sys-f1.patch`, `tool/vendor.sh`); CaDiCaL sources are unchanged. The API's own
  `phase()` sets a persistent forced phase, which arm WF measures.
- **Score seeding not run.** Neither cadical-sys nor the CaDiCaL API exposes a way to read or
  seed activity (EVSIDS scores or VMTF order). This was not emulated.
- **WL adds carried learnts as irredundant clauses.**
- **C3 processes.** C3 ran as 3 processes, and p3 was rerun (Amendment 1).
- **C4 dropped.** k=3 at m=23 was dropped because the A0 smoke censored at 2e6 conflicts.
- **Pending prerequisites.** research-kb and the lead atlas are not reachable here, so no novelty
  claim is made. The repository's artifacts hook wrote `raw/.gitignore` and `raw/.artifacts.json`
  mid-run. Its recorded hashes for the C3 row files predate the end of the run, so they are stale;
  `raw/SHA256SUMS` is authoritative.

## Open threads
1. **WL carried learnts.** This is a target-free learnt-clause carry into a cold folded solve.
   It improves the oracle by about 10%. Next steps:
   - test it at longer streams, with k=3 (where the core is the whole first S3 link) and a
     larger budget;
   - try a clause-quality filter, such as LBD or size, to bound the carried set.
2. **B over long streams.** B/A0 is 0.74 on the second half of the stream. Re-test B and WL+B
   variants over 200+ targets, with the pre-registration done before the data.
3. **Score seeding.** It needs an internal heap/queue rebuild patch, or a solver with an activity API.
4. **First-model cost.** Measure phase seeding on first-model cost in a regime where only one
   decomposition is needed.
