# P-521 n+1 composite cofactor: resolved

- **Status:** `resolved`
- **Kind:** completed integer factorization and full-cohort comparison replay
- **Affected work:** [four-target follow-up](../research/nist_skip_criterion_20261025/factor_followup_20261026/REPORT.md), [PR 2635](https://github.com/isogeny-labs/crypto/pull/2635)
- **Next action owner:** none for this computation
- **Last updated:** 2026-10-26

## Original problem

The requested published P-521 `n+1` factorization retained a 125-digit,
414-bit composite. Its [historical partial record](../research/nist_skip_criterion_20261025/factor_followup_20261026/results/p521-plus-partial.json)
kept prime/composite flags separate. At that point the all-21 census had
83/84 complete factorizations; the complete `n+1` family stayed disabled
globally and unknowns stayed `NA`. No control was dropped.

## Resolution evidence

- Native GNFS014 completed on 26 October at 17:32:47 UTC, exit zero,
  proposing two factors of the exact residual. All earlier bounded failures
  and progress snapshots remain in the [run index](../research/nist_skip_criterion_20261025/factor_followup_20261026/results/run-index.json).
- Separate native admission015 recomputed the full original target, proved
  every prime and checked its complete product. The [canonical record](../research/nist_skip_criterion_20261025/factor_followup_20261026/results/curve-20-plus.json)
  has seven distinct primes, nine factors counting multiplicity, largest
  prime 270 bits and square-part root 15. The former composite splits into
  primes of 145 and 270 bits. Counts and bit lengths here are decimal;
  the [native hexadecimal listing](../research/nist_skip_criterion_20261025/factor_followup_20261026/results/completed-targets-hex.md)
  gives all exact integers.
- Native replay016 repeated prime/product checks across all 84 records,
  admitted all four complete-factor families over all 21 curves, emitted
  1,127 features per row, and replayed the unchanged three search families
  over all 4,480 grouped label assignments. Every native stage exited zero.
- The [final archive readback](../research/nist_skip_criterion_20261025/factor_followup_20261026/evidence/final-native-003-delivery.json)
  verified the full S3 object, internal manifest and all 223 listed file
  hashes. The backend deleted final relation/matrix scratch on success,
  as disclosed in the [source audit](../research/nist_skip_criterion_20261025/factor_followup_20261026/evidence/success-cleanup-audit.json).
  The last retained pool is snapshot002; a full independent GNFS replay is
  not claimed. Mathematical completeness rests on the independent native
  prime/product proof, without independent-host or Lean validation.

## Exit criteria

- [x] Split the exact composite and prove every terminal prime using the
  independent native verifier, checking the full original row20 `n+1` product.
- [x] Admit the complete record, regenerate all21 complete-factor families
  and replay the unchanged search/calibration without omitted controls.

These steps complete the requested factorization/comparison computation.
The shortest structural conjunction still uses three tests; generation
features retain a two-test empirical fit. The
[historical selection and weakness problem](nist-candidate-selection-weakness-hypotheses.md)
remains unresolved. No DLP experiment was requested or added.

## History

26 October: initially recorded the incomplete composite and preserved ECM,
P-1/P+1, failed input admissions and GNFS progress. Later the full native
proof and all84 replay met both exit criteria. The former partial evidence
and its unknown labels remain frozen in the historical raw rounds.
