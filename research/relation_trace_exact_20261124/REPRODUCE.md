# Reproduce the relation diagnostics

All polynomial, field, rank, certificate and replay calculations run in Rust.
The shell scripts perform thin orchestration. Python is used only for source
editing, file metadata and report rendering. This study contains no comparative
timing experiment. [pcc:unverified]

## Producer checks

From the repository root:

```sh
cargo test --release --locked --lib --manifest-path crates/certified-relations/Cargo.toml
cargo test --release --locked --lib --features count-field-ops --manifest-path crates/certified-relations/Cargo.toml
```

The recorded builds used Rust 1.100.0, one build job, CPU0, release opt-level0,
LTO off and16 codegen units. These deliberately unoptimized correctness builds
are not performance samples. UNIT_PLAN_415/416.json bind separately copied test
executables before calls; UNIT_OUTCOME_415/416.json and raw/ retain every test
outcome. Source415 contains the final producer modules. Source419 changes only
driver serialization/output budget. The full crypto_lib root is not qualified
by these component tests. [pcc:unverified]

## Build and use the native driver

```sh
cargo build --release --locked --manifest-path research/relation_trace_exact_20261124/harness/Cargo.toml
```

Copy the resulting relation-trace-exact executable to a new path, record its
SHA256 and make the copy read-only before calls. Do not edit a worker in place.
Use CPU0, one numerical thread, the declared1200-second/2GiB process limits,
and new output paths. SCAN_PLAN_419.json is the prospective correctness plan;
PRECALL_419.json binds its source, worker, inputs and thin runner. The runner's
absolute worker, scratch and study paths describe the recorded environment;
change paths in a separate runner and retain a new pre-call manifest when
reproducing elsewhere. No fresh or reserved seed is needed.

FIXED_INPUTS_412.json names six original seed7 LAST-star fixtures. Its paths
point to the recorded worktree. For a different checkout, retain that file and
derive a new manifest changing only each case's path to the matching checked-in
inputs/p<prime>-n<n>.json. Keep all input hashes, case IDs, primes and n values.
The native driver refuses a changed input digest or a missing/extra case.

```sh
relation-trace-exact scan relocated-inputs.json /dev/shm/new-degree-output
relation-trace-exact replay relocated-inputs.json /dev/shm/new-degree-output
relation-trace-exact index /dev/shm/new-degree-output new-index.json
```

Scan constructs every cheap-consequence single, all and every leave-one-out at
D=2,3,4. It does not consult a reference Gröbner basis. Replay reconstructs this
entire roster from the originals, verifies every supplied original-ideal and
degree-bounded derivation witness, checks both space inclusions, and recomputes
the complete summary. Unknown solving degree and regularity remain null.
Nonzero scan exits, resource failures and censored spaces are retained; a
difference of incomplete rank lower bounds is never called an exact gain. [pcc:unverified]

## Artifacts and negative controls

The partial417 attempt ended in OS ENOSPC and is preserved in
partial-scan-417.tar.gz, PARTIAL_INDEX_418.json, POSTCALL_418.json and raw/.
It has no complete scan summary or full-matrix admission. Its byte index was
constructed after the failure, as explicitly labelled. [pcc:unverified]

Extract a scan archive into an empty directory before replay. Native index
files live outside the audited directory. Never insert README files or indexes
inside that directory: the semantic replayer enforces the exact file roster.

The native modes tamper-summary, tamper-proof, tamper-degree, tamper-roster and
tamper-inputhash each take a complete scan directory and a fresh canary path.
They hardlink unchanged files, atomically replace the changed inode (or remove
the designated canary file), and write a fresh external index. Replay each
canary; it must refuse. Never truncate a hardlinked file. Compare the original
native indexes before/after all controls. The checked-in runner gives the exact
command order and captures stdout, stderr and every process exit code.

## Scalar-label audit

```sh
relation-trace-exact labels label-inputs/p2147483647.json new-label-audit.json
relation-trace-exact labels-replay label-inputs/p2147483647.json new-label-audit.json
```

Repeat for the other supplied scalar-prime demonstration. These are explicitly
abstract linear systems. They do not supply curve points, group orders or
subgroup proofs. The six polynomial fixtures have no such group-row metadata;
no P/Q labels are invented for them. The audit always leaves
curve_scalar_verified=false. Its scalar modulus must be provided separately
from the polynomial coordinate prime. [pcc:unverified]

The public Rust APIs are degree_space::{snapshot,replay,counterfactual} and
relation_labels::{audit,replay}. They accept the existing Poly/Row types or a
LabelSystem, respectively. See the module tests for certified-consequence,
formal-label tautology, useful homogeneous connector, inconsistent-model,
bounded-derivation and resource-censor controls.
