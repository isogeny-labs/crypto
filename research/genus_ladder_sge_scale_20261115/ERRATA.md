# Errata (from the validator replay, `VALIDATION_REPORT.md`, 2026-11-21)

The validated files are left as frozen; corrections live here. [pcc:unverified]

- **E1, double count.** `joined_analysis.json` counts the unmerged arm twice
  (`runs: 12` per cell): the analyzer does not de-duplicate an arm that appears in two
  input files. Means and verdicts are unchanged. The unmerged arm's bootstrap intervals
  are too narrow; at p = 16411 the FPM_mid/rho interval is [0.499, 1.422], not
  [0.534, 1.175].
- **E2, overreach.** RESULTS.md line 34 calls the SGE(8,3) arm "the best measured arm of
  the program". No cross-program table in the study supports that, and the arm cannot be
  ranked against rho across the §5a GAE band (0.07 to 0.10 at 0 GAE, 1.9 to 2.6 at 0.1
  GAE, 9.9 to 14.0 at 1 GAE: unresolved). "About 0.3 of rho" holds only in the FPM
  reading. Read the sentence as "the best arm measured in this study".
