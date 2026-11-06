# Structural probe execution receipt

Protocol: `509df89c5e`, 23 October 2026. The exact integer derivation is
shared by `examples/quartic_twist_s4_integer.rs` (standard library only)
and `examples/quartic_twist_s4_structure.rs` (repository identity
certificate). No result is inferred from a launcher or build failure.

## Local and remote attempts

1. `tools/isolated_bench.py busy -- cargo build ...` could not execute
   the script directly (mode did not grant execution). Calling the
   required isolation tool through its installed Python interpreter
   queued the build on `/tmp/crypto-bench.lock`.
2. The lock's live owner was an unrelated native n83 benchmark; its
   worker was independently observed using CPU. The queued local build
   had not started Cargo. It was terminated with exit 143 to move this
   probe to the repository's authorized ephemeral runner.
3. A clean detached sparse worktree was created at `509df89c5e` for the
   remote source snapshot, containing only this new example as a local
   edit. The first snapshot attempt was refused at 454 MiB because the
   pushed branch's remote-tracking ref was absent. Fetching that actual
   pushed branch reduced the snapshot to one protocol commit and a
   12 KiB source patch on `ab7bbcaed1`.
4. The on-demand launch was denied by the AWS identity policy
   `SpotOnlyEC2Launch`. No instance was launched. The supported `--spot`
   option succeeded under the same configured identity.
5. Job `20261023-015937-twist-s4-structure-00b8fb`, c7i.2xlarge Spot,
   terminated with build exit 101: the sparse checkout omitted the
   path dependency `tools/volcano-cli`. It never ran the derivation.
6. Job `20261023-020454-twist-s4-structure-v2-afc5d6`, c7i.2xlarge Spot,
   added the path dependencies and documentation inputs. It terminated
   during checkout because GitHub requested unavailable credentials.
   It never compiled or ran the probe. Both jobs' complete fetched
   records are retained under the ignored `runs/` extraction tree and
   will be packed in the deterministic raw-run archive.
7. The shared exact-integer implementation was split into a standard
   library Rust executable. The next ephemeral job uses `--no-source`
   and carries that public research source as a base64 job input. It
   requires no repository fetch or Cargo dependency. The full library
   certificate and finite-field profile remain required by the
   protocol; the standalone result does not discharge those checks.

The cloud attempts use the repository ephemeral CLI, configured
credentials and bounded TTLs. No credential values are written here.
The source snapshots and job manifests identify the actual executed
code. Native-time comparisons have not been attempted in this round.

## Completed native and proof jobs

8. `20261023-021219-twist-s4-integer-c988e0` (c7i.large Spot) ran
   the standard-library exact derivation and its mutation test. It
   returned 49 symmetric terms and zero integer reconstruction remainder.
9. `20261023-022455-twist-s4-profile-30c47f` (c7i.large Spot)
   checked all 46,200 finite peels against independently interpolated
   coefficients, issued and rechecked the 49-term IDC certificate, and
   passed 13 native tests. Every original and linearly reduced equation
   degree profile was `[4,4,4,4]`.
10. `20261023-023024-twist-s4-full-proof-dadb56` derived the full
    18-term symmetric identity correctly, but **its Lean result is
    invalid**. Extraction into `/tmp` exhausted temporary storage;
    unchecked shell stages then returned misleading zero exits. The
    version, build and audit logs were empty and the `.olean` artifact
    was absent. Its raw status says `done`, but it is never counted as a
    proof pass. The replacement script uses `/work`, `set -euo pipefail`,
    nonempty version/compiled-artifact checks and positive theorem names
    and counts in the axiom audit.
11. `20261023-023821-twist-s4-f4-roots-c91c59` returned the exact
    exhaustive field-root and signed-group sets on all eight immutable
    fixtures. The failed p73/seed1 system has precisely roots
    `(7,61,52)` and `(66,61,21)`. Both missing signed relations were
    recovered. A mutated coefficient was rejected by the input hash
    before solving (exit 101), and 48 native tests passed.
12. `20261023-024635-twist-s4-proof-disk-678af3` successfully
    compiled both Lean theorems and printed their standard axioms, but
    the separate audit failed: its subprocess could not find `lean` on
    `PATH`. The whole job therefore failed; it is not an audit pass.
13. `20261023-025202-twist-s4-f4-prefix-9b3f7b` compiled the new
    collector, then its test build failed because the source bundle
    omitted the existing `include_str!` certificate fixture. No prefix
    collection ran. `bundle-inputs.txt` now includes that required file.
14. `20261023-025323-twist-s4-proof-audit-734f57` compiled and
    audited both identities with Lean 4.34.1; 2 theorems, 0 failing,
    0 compiler-trusted. Both compiled artifacts and nonempty audit
    output were checked before success was recorded.
15. `20261023-025542-twist-s4-prefix-all-48c275` completed both
    full original-prefix collectors: 46,200 systems per run, seven
    exact group-oracle set matches, seven full-rank target/factor-log
    verifications, no undetermined systems, and byte-identical first/A/A
    output trees. It also passed 48 native tests and issued the separate
    full-symmetry IDC certificate. F4 row-reduction counts are explicitly
    incomplete as complete solver arithmetic and are not a priced speed
    comparison.

All ten research Spot instances terminated. Two additional launcher
refusals (on-demand IAM policy, unsupported `--env-file` argument) launched
no instance. Running-phase status records can contain literal newlines in
the CLI's reason field, so a transient status-parser error was checked by
fetching the same job rather than relaunching it. Final terminal status,
actual outputs and instance termination were checked separately.

The local mandatory library gate completed with 4,488 passed, 119 ignored,
zero failed. Both touched examples' tests passed (1 and 3 tests). These
commands used the required isolation lock; their correctness results make
no timing claim. The library receipt explicitly says its tool display was
truncated; the complete example test log is preserved.

## Document builds and archive check

The local document build was queued behind another live holder of the
required shared lock and was terminated before its renderer started.
The report build moved to the same authorized ephemeral CLI.
`20261023-031523-twist-s4-doc-build-2568a3` failed on a Typst table syntax
error after rendering the individual run reports. The corrected
`20261023-031923-twist-s4-pdf-fixed-1faa29` completed all ten research-job
reports and the round report. Both document-build instances terminated.
These are rendering builds, not additional algorithm experiments. Their
commands, public source inputs, host/rusage records and build logs are
retained separately in `build_receipts/`; published PDFs and editable
sources are in the study and `reports/`.

The ten research-job trees and local validation receipts are preserved
in `runs.tar.xz`. SHA-256 verification and xz integrity passed. Extracting
the archive into a fresh scratch directory and repacking with the stated
normalized GNU tar options produced byte-identical bytes. Source checks
in `SOURCE.sha256` passed; the executed final collector source hashes
also match the working native sources exactly.
