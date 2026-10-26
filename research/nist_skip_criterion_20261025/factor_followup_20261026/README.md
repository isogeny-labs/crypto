# Four-target factorization follow-up

All four requested targets have independently proved complete factorizations.
The all-21 comparison now has 84/84 complete records. Start with the [report](REPORT.md),
[coverage record](results/factor-summary.json),
[hexadecimal factors](results/completed-targets-hex.md) and
[21-curve comparison](results/cohort-factor-bits.tsv).

| Target | Status | Canonical record |
| --- | --- | --- |
| p239 index 2491, `n+1` | Complete, largest prime 131 bits | [proof](results/curve-13-plus.json) |
| P-384 Frobenius discriminant magnitude | Complete, largest prime 265 bits | [proof](results/curve-19-disc.json) |
| P-521 `n-1` | Complete, largest prime 391 bits | [proof](results/curve-20-minus.json) |
| P-521 `n+1` | Complete, largest prime 270 bits | [proof](results/curve-20-plus.json) |

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

Complete `n-1`, `n+1`, twist and discriminant families are available across
all 21 rows. The unchanged search replay on 1,127 features retains a
three-test structural separator and a two-test generation separator. Neither
is established as the historical selector. The earlier partial record and
83-record raw round remain frozen; no unknown was encoded numerically or
silently excluded. See [the final replay protocol](FULL84_REPLAY_PROTOCOL.md).

The separate [SEA feature replay](../sea_replay_20261026/REPORT.md) matches
all 21 recorded coverage values and supplies exact P-256/P-224
counterexamples to uniform preferences for larger or smaller bounds.
It adds no p-value, point-counting cost or historical-selection claim.

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

The immutable `backend-progress-002.tar.zst` is a running snapshot, taken during
GNFS014 filtering with no active siever children. It includes the pool whose
native filter reported17,831,805 raw and14,715,396 unique relations. Its
`runs/backend-manifest-002.json`, [21-file manifest](evidence/backend-manifest-002.json)
and [full readback receipt](evidence/backend-progress-002-delivery.json) bind
that exact resume copy. Unpack with `tar --zstd -xf`. It records progress at
16:08 UTC, before completion, and remains unchanged.

The final `final-native-003.tar.zst` contains all surviving GNFS014 records,
the independent full-target admission015, the full84 replay016 and its
223-file [manifest](evidence/final-manifest-003.json). Its full-object GET,
internal manifest and every listed file hash [verified](evidence/final-native-003-delivery.json).
The pinned YAFU success cleanup deleted final relation/matrix scratch before
the launcher regained control; [the source audit](evidence/success-cleanup-audit.json)
records this retention gap. The final 26,716,780-relation pool is not retained;
snapshot002 is the last retained pool. The complete factorization is certified
by independent primality and exact-product checks, without a full GNFS replay.
