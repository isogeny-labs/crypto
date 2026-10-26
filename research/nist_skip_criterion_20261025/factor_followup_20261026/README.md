# Four incomplete factorization targets

Status: prepared; no new factoring run or completion is claimed.

The requested targets are p239 index 2491's `n+1`, P-384's Frobenius
discriminant, and P-521's `n-1` and `n+1`. The exact curves and orders remain
in [the historical input table](../inputs/historical.tsv). The four FactorDB
responses in `hints/` were fetched again on 2026-10-26; all still have status
`CF`. They are proposed partial factors, not prime certificates.

`prepare_factor.cpp` recomputes the target from the frozen curve parameters,
checks the proposed factor product, proves each known prime with native PARI,
and writes unresolved composite residuals separately. It refuses to overwrite
records or launch a factorization with multiple unsupported residuals.

`run.sh` first requires the repository's S3 checkpoint gate. It then builds
the native input checker and runs two GMP-ECM workers over the four targets.
The first batch uses 64 curves per residual, stage-one bound 1,000,000, and
a 900-second watchdog per target. These are batch limits, not evidence that
the targets can be fully factored within that work. Outputs, exit statuses
and saved curve residues are retained in a fresh output directory.

```sh
NIST_FACTOR_OUT="$PWD/research/nist_skip_criterion_20261025/factor_followup_20261026/runs/batch-001" \
  sh research/nist_skip_criterion_20261025/factor_followup_20261026/run.sh
```

The script defaults to the installed native PARI and GMP-ECM under
`/workspace/tools/sage_skeleton_env`; `PARI_PREFIX` and `GMP_ECM` can select
another installed native build. It invokes no Sage or Python research code.
ECM status and its text alone do not certify a complete factorization.
Outputs must be inside this follow-up's new `runs/` directory, so the
checkpoint collector captures them. The launcher checkpoints again after
input validation and after the ECM batch. Do not ignore or remove unarchived
run records; archive and store them under the existing policy when finalized.
Recombine any discovered factors and run the existing independent native
`verify_factor_hint.cpp` before enabling complete-factor features. Preserve
every unsuccessful batch and checkpoint before the next one. Use a separate
output directory for each batch.

Once all four targets have proved complete factorizations, regenerate the
complete-factor feature tables on all 21 curves and replay the same declared
short-rule search and grouped-label calibration. A successful factorization
does not itself establish a DLP attack or historical selection criterion.

The current blocker is checkpoint upload access. The environment's current
configuration exposes no secrets or outbound identity; the installed AWS
client cannot locate credentials. [AGENTS.md](../../../AGENTS.md#resume-checkpoints-in-s3-before-every-run)
requires a successful checkpoint before a new experiment. The requested
factorizations remain unfinished while that gate is unavailable.
