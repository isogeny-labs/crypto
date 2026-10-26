# Four-target factorization follow-up

Three requested targets have proved complete factorizations. P-521 `n+1`
still contains a 125-digit composite. Start with the [report](REPORT.md),
[coverage record](results/factor-summary.json),
[hexadecimal factors](results/completed-targets-hex.md) and
[21-curve comparison](results/cohort-factor-bits.tsv).

| Target | Status | Canonical record |
| --- | --- | --- |
| p239 index 2491, `n+1` | Complete, largest prime 131 bits | [proof](results/curve-13-plus.json) |
| P-384 Frobenius discriminant magnitude | Complete, largest prime 265 bits | [proof](results/curve-19-disc.json) |
| P-521 `n-1` | Complete, largest prime 391 bits | [proof](results/curve-20-minus.json) |
| P-521 `n+1` | Partial, largest prime unknown | [partial record](results/p521-plus-partial.json) |

All inputs are the exact rows in [historical.tsv](../inputs/historical.tsv).
Indices, bit lengths, exponents and counts are decimal. Raw native factor
records encode integers as explicitly decimal strings; the separate factor
listing uses `0x` hexadecimal prefixes. This work adds no DLP experiment.

`prepare_factor.cpp` recomputes each target, proves known primes and emits
composite residuals separately. `refine_factor.cpp` only splits proposed
factors using backend output; it does not certify primes. The pre-existing
independent native `../native/verify_factor_hint.cpp` recomputes the original
target and proves all factors before a result is admitted. Its acceptance
records are preserved even when it rejects a partial factorization.

The cohort has 83 complete factorizations out of 84. Complete `n-1`, twist
and discriminant families are now available across all 21 rows. Complete
`n+1` remains unavailable to the all-21 search. Unknowns stay `NA` in the
descriptive table; no control is omitted. The unchanged search replay on
1,118 features retains a three-test structural separator and a two-test
generation separator. Neither is established as the historical selector.

## Native methods and evidence

[FACTORING_PROTOCOL.md](FACTORING_PROTOCOL.md) specifies follow-on methods,
admission and stop rules. [run-index.json](results/run-index.json) maps each
attempt to its commands, raw records and report. The first GMP-ECM batch,
Msieve SIQS, refinement checks, smoothness attempts and full-cohort search
have completed. The first two YAFU invocations did not read stdin and are
retained as failed admissions. Corrected YAFU ECM and GNFS attempts use the
exact residual on stdin and retain progress separately.

Launchers are thin shell wrappers around native binaries. Native PARI and
GMP-ECM default to `/workspace/tools/sage_skeleton_env`. Msieve and YAFU are
pinned by upstream commit in the report; no Python research driver runs.
Set `NIST_FACTOR_OUT` to a fresh directory under this directory's `runs/`.
`run_yafu.sh` also takes `NIST_YAFU_MODE=ecm` or `nfs`. It can invoke a
credential-supplying checkpoint child via `NIST_CHECKPOINT_WRAPPER`; native
factor workers do not inherit that child's credentials.

Every experiment first requires a successful S3 checkpoint under
[AGENTS.md](../../../AGENTS.md#resume-checkpoints-in-s3-before-every-run).
Credentials stay outside the repository. Raw attempts remain checkpointed
until archived and stored; never ignore or remove unarchived progress.
Timeouts and zero backend exits do not establish factor completion.

`summarize_factors.cpp` repeats prime/product checks for all admitted
records, emits the coverage table and converts completed targets to hex.
Compile with C++17, `-O2 -Wall -Wextra -Werror`, the existing native PARI
include/library paths, `-lpari -lcrypto`, and `../native` as an include
directory. Its arguments are the historical input table, complete record
directory and fresh output directory. The search and feature programs are
unchanged from the parent study.

The parent study's original PDF and evidence bytes stay frozen. The dated
follow-up has its own report and visuals. Cargo and Conductor are unavailable
in this environment; neither missing check is represented as passing.

## Hydrate the evidence

The [artifact manifest](evidence/.artifacts.json) binds the immutable raw
archives. `closed-runs.tar.xz` contains the first ten phases and its
`runs/manifest.json` (289 members). `backend-progress-001.tar.xz` contains
closed attempts008/009/011/012 and controls013, with
`runs/backend-manifest-001.json` (108 members). Its
[delivery receipt](evidence/backend-progress-001-delivery.json) records
full-object S3 GET and per-member verification. The second archive includes
the saved GNFS relations, not a complete P-521 plus factorization.

From the repository root, hydrate this directory with the repository
artifact tool, then unpack each archive into this follow-up directory.
The run reports' `runs/...` references are extracted archive locations.
New active raw directories remain visible to checkpoints until archived;
only the proved immutable closed records are ignored in Git.
