# Optional shared-reduction profiling: unapplied candidate

This is a tested patch artifact, not an integrated optimization. T-906 still
holds the shared source path; no shared source or GPU lane file was changed.
The current executable and round13 timings therefore do not include this patch.

## Change

`solve_with_oracle` currently delegates to `solve_with_oracle_profile` with a
throwaway statistics object. Its target, oracle, base, linear and replay stage
clocks still run. The patch gives the single reduction body a const PROFILE
parameter: false for the ordinary API and true for the existing profiled API.
A lazy `PROFILE.then(Instant::now)` skips the clock when disabled. The profiled
API retains additive diagnostic statistics, including discarded trials.

Every random draw, bound, decomposition query, relation check, elimination and
full certificate replay is unchanged. The complete driver, its outer timer,
its separate Timed oracle wrapper, serialization and both certificate checks
are unchanged. Removing discarded internal instrumentation does not remove
an algorithm phase or narrow the end-to-end timing interval.

[Official Rust Instant documentation](https://doc.rust-lang.org/std/time/struct.Instant.html)
was reviewed before implementation: now/elapsed read the platform clock;
these observations are not cryptographic work. No runtime saving is inferred
from a source change. This is an engineering optimization, not a new algorithm.

## Validation and reproduction

The exact base, candidate and patch SHA-256 values are in [source.json](source.json).
[optional-profile.patch](optional-profile.patch) applies to the pinned shared
source with `git apply --check`. [validate.sh](validate.sh) refuses an existing
destination, verifies source identities, copies only the standalone CPU/tensor
crates and archived certificate fixtures into a disposable snapshot, applies
the patch there, and runs release tests, strict Clippy and formatting checks.
It rechecks that the original shared source is unchanged.

```sh
bash research/hashed_ecdl_cpu_20261030/optional-profile/validate.sh   /tmp/hashed-ecdl-profile-snapshot-unique   /tmp/hashed-ecdl-profile-target
```

All 32 release tests pass in the snapshot, with strict Clippy and formatting
passing. The added test compares full certificates and exact target-query
sequences across scalar0/1/42/198 and seeds111/222/333; it also checks that the
profiled API preserves initial accumulated statistics and the disabled path
leaves all statistics untouched. Existing archived-certificate, exceptional
arithmetic, invalid-oracle, invalid-input and exhausted-trial tests also pass.
[Validation receipt](validation.json) pins the commands and log hashes.

Initial snapshot preparation stopped before compilation because the optional
system `patch` utility was unavailable. The reproducible helper now uses the
installed `git apply` and succeeded in a fresh snapshot; no failed benchmark or
performance data was excluded. These are development correctness checks only.

## Remaining work

Integrate only after shared-source ownership is reconciled, rechecking the
patch against any newer field-guard changes. Then freeze a full five-arm
comparison with original baseline2bd4588fba60, previous92463171363c, A/A and
batch1 controls. Preserve all fixtures and phases. No further10x claim follows;
strict isolation, independent review and matched external comparisons remain
outstanding. No new cloud job was launched for this artifact.

The active reproduction path uses thin shell tooling and the shared
`../validate-snapshot.sh` runner. It reconstructs source, crates and certificate
fixtures from the exact `base_commit` in the immutable descriptor, then checks
the base, patch and candidate digests. This remains reproducible after later
source integration. The current shared source is hash-checked before and after
and is never edited. Original Python helpers and historical receipts remain
unchanged as evidence, outside the active reproduction path. Requires Bash,
GNU coreutils, jq, Git and Cargo. No timing or scientific run is performed.

The [pinned shell reproduction receipt](pinned-shell-validation/receipt.json)
records successful release tests, strict Clippy and formatting after the later
source integration. The current shared source is retained byte-for-byte. The
source descriptors and Python-era receipts keep their original recorded state.
