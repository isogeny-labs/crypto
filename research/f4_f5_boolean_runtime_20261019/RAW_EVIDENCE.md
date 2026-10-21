# Native evidence and replay

`runs.tar.xz` is the deterministic archive of native run receipts and
the auxiliary instruction profiles. Verify its adjacent SHA256 file before
extracting, from this directory:

```sh
sha256sum -c runs.tar.xz.sha256
tar -xJf runs.tar.xz
cd ../..
CARGO_TARGET_DIR=/tmp/boolean-replay-target bash research/f4_f5_boolean_runtime_20261019/replay-analysis.sh
```

The archive is committed directly in Git. A named-store upload to
`evidence/f4-f5-boolean-native-20261019/runs.tar.xz` was attempted, but the
configured store bucket rejected `HeadObject` with HTTP403. No store entry
was created and no account policy was changed. Git retains the complete
compressed receipts, rather than relying on a workstation or job URL.

The exact deterministic packing command, run in this directory, is:

```sh
tar --sort=name --mtime='2026-10-19 00:00:00Z' --owner=0 --group=0 --numeric-owner \
  --mode='u+rwX,go+rX,go-w' --format=gnu -cf - runs profiles profiles-corrected \
  | xz -9e -T1 > runs.tar.xz
sha256sum runs.tar.xz > runs.tar.xz.sha256
```

The archive has `runs/{initial,optimization,final,policy,serial-clear,serial-guard}/out/`, plus
`profiles/` and `profiles-corrected/`. Each native run contains every retry's
command, output, exit code and isolation receipt, build logs, status and
resource receipts, frozen protocol and source/binary manifests. The analyzers
select the first eligible of at most three attempts; they never choose the
fastest retry. Rejected attempts remain in the summary and archive. The
malformed first local profile attempt is retained separately, never pooled
with the corrected batch. Local Callgrind elapsed time is not native timing.

The binary executables are omitted from Git. Full immutable job trees,
including those executables and their SHA256 manifests, are stored under
`s3://ephemeral-runs-104186811093-us-east-1/jobs/<job-id>/`:

| Scope | Job ID | Source contract |
|---|---|---|
| Initial native comparison | `20261019-014036-boolean-native-baseline-8d3eb7` | Original `1340a6176` algorithm and single-write switch |
| Vector/Gray candidate | `20261019-013858-boolean-native-vector-gr-799a77` | Frozen original source vs first candidate; scalar/portable/legacy ablations |
| Semaev and phase3 costs | `20261019-014050-boolean-native-batch-fin-43e52b` | Exact scalar/batched cold table comparison and all16 solver arms |
| Cutoff/typed confirmation | `20261019-024352-boolean-native-policy-ty-fb16d2` | Frozen iteration1 vs cutoff/typed candidate; all18 solver arms |
| Incremental serial row clearing | `20261019-030905-boolean-native-serial-cl-25ad47` | Same-binary matrix switch, one worker and four-worker unchanged-path controls |
| Explicit-one-worker guard | `20261019-040338-boolean-native-serial-gu-6ec5e9` | Fresh incremental pairs, four-worker controls and direct legacy-kernel ablations |

These are independent hosts. Each implementation ratio is paired on its own
host; final method rankings use only the final confirmation host. No ratio
multiplies different studies and no sample pools hosts. `job.json`, local
patch, native source copies and source hashes identify the actual build,
rather than assuming later commits were what ran. The merge from main after
launch adds checkpoint plumbing in sparse diagnostics; measurements retain
their original exact source identity. Post-merge correctness checks are
separate receipts.

The first two sparse source-preparation failures and the later on-demand
quota refusals are documented in `EXECUTION.md`,
`ENVIRONMENT_ADDENDUM.md` and `preparation-failures/`. They supply no timing.
Environment export files are excluded. Compiler, CPU/ISA, memory, affinity,
binary hashes, commands and native statuses remain available for replay.

Rendering is separate:

```sh
/tmp/report-venv/bin/python research/f4_f5_boolean_runtime_20261019/render.py
python3 scripts/build_ic_leaderboard.py
python3 scripts/build_lab_browser.py
```

Python lays out the Rust-prepared native medians, ranges, ratios and
admission decisions. It performs no solver or qualification calculation.

Final replay receipt: extracting and repacking with the command above yields
the identical SHA256. Re-running all Rust analyzers against the extracted
receipts reproduces all nine derived JSON artifacts byte for byte.
