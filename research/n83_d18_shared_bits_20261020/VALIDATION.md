# Validation receipts, version 1

The [archive](validation-v1.tar.xz) contains 48 files. This version preserves
the completed input/proof checks and unsuccessful build attempts; it does
**not** certify the release library or affected-example suites.

| Check | Observed outcome |
|---|---|
| Lean proof gate, 4.34.1 | PASS. `factorization` and `withinUnchangedContext` have no axioms; `clauseCount` uses only `propext`. |
| First independent full input inventory | Exit 0; all 16,627 roots checked. Source snapshot and original receipt retained. |
| Inventory v2, same source exposed through the Cargo wrapper | Exit 0; 66 shared zero bits, 17 varying bits; published `root-inventory.json` is this version. |
| Modified CSV | Rejected with `representative CSV digest differs`; no output produced. Modified input retained. |
| Existing output | Rejected with `AlreadyExists`; existing receipt preserved. |
| Shell syntax and ShellCheck | Pass after excluding SC2024, whose warning is inapplicable because receipt redirections intentionally run as the unprivileged invoking user. |
| Initial local release-library compilation | Interrupted before tests after static review found that unit placement affected construction counts. The source was corrected; no pass is inferred from this attempt. |
| Later local examples build | Exit 101, `No space left on device`; subsequent compilation stopped before moving this task's cache to T7. |
| First Linux release-library build | Exit 143, before a test result. The exact cause of termination is unknown. |
| Further Linux attempts | Refused to overlap another benchmark container; no native validation result. |
| Current T7 native compilation | Outside this frozen archive; still pending when version 1 was assembled. |

The original `host.txt` records revision `82221f8bd` before implementation.
`tested-sources.sha256` names the bytes supplied to an attempted validation,
not a passed test result. The successful Lean receipts bind the archived
`SharedRootBits.lean` bytes. Linux's first build separately records revision
`398abd50d32b3a1929ca7d8c38215b7022886afe`.

The v2 inventory was compiled with this command and native cached dependencies
on the same Apple arm64 development host:

```sh
root=/Volumes/T7/codex-n83-shared-bits-20261021
rustc --edition 2021 -O examples/n83_d18_root_inventory.rs \
  --extern sha2="$root/target/release/deps/libsha2-ada6e89a89a0557e.rlib" \
  --extern serde_json="$root/target/release/deps/libserde_json-aa1f2095037ebc36.rlib" \
  -L dependency="$root/target/release/deps" -o "$root/root_inventory_v2"
"$root/root_inventory_v2" \
  research/n83_d18_subgroup_orbit_census_20261018/run-20261019T0142Z/selected-representatives.csv \
  "$root/root-inventory-v2.json"
```

This direct compilation checks the independent inventory executable only.
It does not link or execute the changed SAT library. The mandatory complete
release-library and affected-example validation remains open.

## Extract and verify

From this study directory, use a fresh scratch directory:

```sh
shasum -a 256 -c validation-v1.sha256
mkdir /tmp/n83-shared-validation-v1-new
tar -xJf validation-v1.tar.xz -C /tmp/n83-shared-validation-v1-new
manifest="$PWD/validation-v1-files.sha256"
(cd /tmp/n83-shared-validation-v1-new && shasum -a 256 -c "$manifest")
```

The archive uses sorted file names, ustar format, uid/gid 0, root owner/group,
0644 file modes, timestamp 2026-10-20 00:00:00 UTC and xz level 6 with one
thread. Files only are archived; extraction creates their parent directories.
The archive was extracted, every file hash checked, then repacked and compared
byte-for-byte. It contains frozen failures as well as passing evidence.

Future completed native checks belong in a new archive; do not replace v1.
The launch/card preparation scripts have only static validation at this point.
Their future execution remains distinct from the input/proof successes above.
