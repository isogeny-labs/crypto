# Local release CLI correctness fixture

This directory freezes a small **software correctness fixture**, excluded from
the requested ECDLP research corpus. It does not launch the startup gate, a Pod,
a grant, an experiment campaign, or any cloud operation. Its directory name
identifies the source integration being checked, not a deployment claim.

The runner uses the actual release binary for `dataset plan`, `preflight`,
two bounded `generate` calls, `verify`, and `status`. It then verifies a copied
cell with deliberately corrupted algebra and recomputed artifact hashes. No
test result exists when this preparation is committed.

## Frozen inputs and limits

- Plan: prime curves, n=7, m=3, F4, seed42, eight curve seeds, two generators,
  two keys, both twists:64 planned cells. Factor base2, degree bound4, work512.
- Execute only shards62/64 and63/64, each in a fresh output directory with
  `--max-cases 1`. These select curve_seed7, generator_index1, key_index1 and
  the two twist positions. Keep the native generator's natural targets.
- Each worker:20-second supervisor deadline,512MiB address-space limit,
  16MiB polled output threshold. This threshold is not a hard byte retention
  guarantee; manifest/publication metadata are outside its scope. The adapter
  also has its existing native deadline and separate trace/lift limits.
- Each command:45 seconds; total runner command budget240 seconds. Timeout,
  SIGINT or catchable SIGTERM reaches command-process-group cleanup and writes
  a failure receipt where storage remains available. The native Linux worker
  has its existing parent-death kill guard. Uncatchable SIGKILL or host loss can
  prevent runner cleanup and leave a CLI supervisor alive; reconcile retained
  process/output identities before any new attempt. No automatic retry or resume.
- `RAYON_NUM_THREADS=1`. Times are termination controls and operational records,
  not performance measurements. No warmup, A/A, statistical power, speed,
  instruction-count completeness, FFD/LFD, or cryptanalytic conclusion.

Both natural targets may be PDP-unsolved. Passing requires sealed cells,
retained exact chain input, matching source/binary/plan/protocol/configuration,
and successful native replay. No assertion requires a PDP solution or treats
known-answer construction as independent ECDLP recovery. Original native
outcomes and all intermediate artifacts remain intact and restricted.

The negative software control copies shard62, changes one S3 coefficient and
restricts its auxiliary variable to the leaf domain, then recomputes the entire
manifest's artifact hashes and lengths. Real CLI `status` must accept the
integrity-valid copy; real CLI `verify` must reject it with the chain semantic
reconstruction error. The original shard is never modified.

## Binding and execution after review

Commit these files before execution. The protocol body is frozen with the
native `verify_protocol` canonicalization contract: sorted JSON keys, compact
separators, UTF-8, and its `freeze` member excluded. This fixture uses integers,
strings, booleans and null only, avoiding floating-point canonicalization
differences. Its analysis field binds the runner SHA256. `fixture-manifest.json`
binds the completed README, protocol and runner bytes. The parent must separately
record the manifest SHA256 and actual commit revision; no self-referential
manifest hash is embedded. Formal external experiment-schema validation has
not been claimed; native preflight checks the actual generator protocol contract.

After the parent commits and builds the integrated clean release source, invoke:

```sh
python3 research/ml_dataset_20261028/startup-integration-cli-fixture/run.py \
  --binary /absolute/path/to/immutable/crypto \
  --binary-sha256 ACTUAL_64_HEX_BINARY_SHA256 \
  --source-commit ACTUAL_40_HEX_BINARY_SOURCE_COMMIT \
  --fixture-commit ACTUAL_40_HEX_FIXTURE_COMMIT \
  --scratch /absolute/existing/scratch-parent/new-cli-fixture-run
```

The placeholders are deliberately invalid until replaced. The scratch directory
must not exist. Source and binary identifiers are required arguments, then
checked against native clean-build preflight and both generated run bindings.
The binary is hashed before and after every command. The fixture commit is a
supplied provenance declaration, not a Git query or automatic proof of filing;
the parent owns that check. The runner never builds, runs Git, publishes, or
changes its source fixtures. It copies the frozen files to scratch, fsyncs them,
sets them read-only and records execution inputs before the first CLI call.

Subprocesses receive an explicit environment allowlist. Inherited
`ISOLAB_RESULTS_URI`, AWS/IRSA credentials, budget/store routes and proxies are
absent. Commands never pass `--publish`, `--remote`, or budget arguments.
These controls select native local execution; they do not claim a network
namespace/firewall proof. No cloud permission or billing check is performed.

Every command retains argv, complete stdout/stderr, timestamps, exit status,
timeout status and binary hash in scratch. Both native output trees and the
tampered copy remain there. `receipt.json` records pass/failure and a full file
inventory, keeps mathematical outcomes distinct, and states zero corpus rows
and zero cloud dataset cells. Any failed step stops the sequence, preserves its
prefix and fails the fixture. An interrupted or failed directory is never
silently reused; investigation and an explicitly recorded new attempt are
separate work.

## Source and claim boundaries

The protocol and runner follow `src/ml_dataset/mod.rs::verify_protocol`,
`DatasetPlan::cell`, `generate`, `verify_cell`, and the existing `TempRun.bound`
unit fixture. The equation and auxiliary-domain contract is frozen in
[PRIME-F4-CHAIN-CONTRACT.md](../../../docs/ml-dataset/PRIME-F4-CHAIN-CONTRACT.md),
which records the reviewed Semaev literature and affine-infinity limitations.
Only software CLI correctness is decided here. Startup activation, EKS,
S3 durability, restored workers, large-prime F4, model training and the full
requested corpus remain outside this fixture.
