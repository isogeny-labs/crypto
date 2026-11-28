# Validator report: research/quartic_diem_p0lines_20261128

This is a transcription of the validator subagent's final report, as relayed to the producing session. The authoritative structured record is `validator-report.json`, whose sha256 is bound in `VALIDATION.json`.

**Verdict: validated with errata.** Validated commit: c271f320e40. The gate checker returns `valid`.

**Claims.**
1. **Decision INCONCLUSIVE: reproduced** by an independent `recompute.py`.
   - **Primary panel.** p0/nofb GAE₁ is 0.98963 [0.93372, 1.04497]. At w = 0 it is 1.01775 [0.86067, 1.18060].
   - **All seeds.** The ratio is 0.95057 [0.92109, 0.98139].
   - **Exact law.** The exact bootstrap distribution gives the same verdict.
   - **Bootstrap stability.** Across 200 fresh bootstrap seeds, no interval flips the verdict.
   - **Reproducibility.** `analyze.py` regenerates `analysis.json` byte-identically.
2. **Integrity holds.**
   - 360/360 certificates re-verified with independent Picard arithmetic, with negative controls.
   - The log is the same across all arms.
   - **C0:** 80/80 rows on the 24 frozen fields, and on all 86 shared keys.
   - **P0-line counts:** consistent across arms and seeds.
3. **Second attempts confirmed.** nofb 25/120 against p0 12/120 on all seeds, and 10/40 against 8/40 on the primary panel.
4. **Exploratory table confirmed.** Rescued pairs cost 0.62 of their nofb pair. Pairs where both arms finish in one attempt cost 0.993–1.027.
5. **IC/rho is UNRESOLVED in all 12 cells.** At 131101, w = 1, p0 is 1.90874 [1.81993, 1.99452] against nofb 2.01857.

**Requirements.**
- **Freeze order:** holds.
- **Binary and source hashes:** match.
- **Binary built from the freeze commit:** supported by cargo's build records, without a rebuild.
- **Replay:** all 360 runs, 508/508 rows byte-identical with wall time masked.
- **Code review:** every P0 line is counted, there is no new uncounted work, and the nofb arm is unchanged.

**Errata and process findings:** E1–E3 and P1–P9, listed in `../ERRATA.md`.

**Lessons:** outcome O-2ec78849dbff and 11 confirming tests. New candidate lessons L-9ceae4541670 and L-0cb5abe0f6bd are recorded in `docs/lessons/entries/`.
