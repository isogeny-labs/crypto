# Independent ML dataset consumer

This companion checks saved native dataset artifacts, restores one completed
cell from S3, and exports an allowlisted view for static solver-routing and cost
models. It does not generate cryptographic instances or implement a second
cryptographic solver. The native producer remains separately owned under
`codex/ml-ecdlp-dataset-20261028` (Conductor T-684); the initial consumer was T-693; phase-label continuation is T-700.

## Checks and boundaries

`consumer.py audit` checks artifact sizes and SHA-256 digests, canonical paths,
the frozen plan and mixed-radix cell index, run identity, shard membership,
public instance family/field size/arity/twist, private sampling provenance,
factor-base requested size, and basic positive-certificate consistency.
Certificate-bearing cells must contain their public instance, witness and
factor base. A successful PDP certificate must have the configured arity and
target; positive and exhaustive-negative outcomes cannot coexist.

These are structural checks. They do **not** prove curve orders, point sums,
discrete logarithms, UNSAT, exact operation counts, or first/last fall degrees.
The report always keeps `mathematics_independently_verified: false`.
Optional native replay is explicitly producer replay, with the verifier binary
hash recorded before invocation and checked afterward. All artifact hashes
are rechecked afterward, and run/manifest digests must remain unchanged.
Use an isolated restored directory; this is not a filesystem locking protocol
against concurrent or adversarial writers.

`consumer.py export` requires native replay unless explicitly requested as an
unreplayed preview. It keeps failure, unsupported and timeout outcome rows;
missing public inputs yield null features. Labels remain under
`producer_reported` with `independently_verified: false`. Hash verification does
not convert an outcome into a scientifically accepted label.

Feature selection admits only typed static public inputs: curve/field
description, generator/target, factor-base points/actions/orbits, solver and
work limits. It excludes complete manifests and configs, private witnesses,
generation seeds, construction traces, and all solver traces, including the
terminal BSGS hit. Do not feed the entire exported record to a model: only
`features` is the feature view; IDs, labels and split metadata are separate.
The record schema `ml-dataset.static-routing-view/v1` is a narrow companion
view, not a claim of conformance to the broader `crypto-ml-v1` training schema.

Phase PMU telemetry is retained only under `labels.phase_measurements`. Each
readback retains its durable start, raw counts, scaled estimates, enabled/running
times, errors, scope exclusions and artifact digests. The consumer checks label
consistency against those raw fields and the producer's integer scaling rule.
An unmultiplexed instruction count must have equal enabled/running times; a
multiplexed estimate cannot populate the exact-count field. Missing PMU readings
remain null, and a start without readback remains interrupted or unfinished.
These checks do not calibrate the PMU or establish whole-worker instruction
cost: process startup, publication and work outside the phase remain excluded.
`whole_worker_instruction_count` stays null; phase counts and peak RSS are not
summed into an unsupported total. Telemetry never enters the feature view.

Whole field sizes are assigned deterministically to 80/10/10 hash buckets.
Every generator, key, twist and family at a fixed n stays together. This can
produce uneven or empty evaluation partitions for small size panels. It does
not yet identify isogeny, subfield or other linked components across sizes,
so it is not sufficient by itself for cross-size generalization claims.
Adaptive solver policies need a separate prefix-only trace export and
decision-time feature definitions; this tool intentionally has no such view.

## S3 restoration

`fetch` consumes a publication receipt with an independently supplied SHA-256
anchor. It accepts only the canonical bucket `setec-astronomy-artifacts`,
with optional prefix, and canonical content-addressed object keys. It checks
declared download limits (20 GiB and 100,000 references), HEAD size, GET with
the observed ETag, and SHA-256/size of every downloaded artifact. The restored
artifact set must exactly equal the manifest, including frozen provenance.
Failed downloads remain in an isolated incomplete directory and must not be
treated as completed cells. Existing destinations are never overwritten.

The receipt anchor must come from a trusted source such as a reviewed Git
record; calculating it from an untrusted receipt does not authenticate data.
S3 byte integrity is distinct from mathematical validity and from retention
guarantees. This tool does not configure versioning, Object Lock or lifecycle.

```sh
python3 consumer.py fetch --receipt completion.json \
  --expected-sha256 TRUSTED_COMPLETION_SHA256 --out /new/restored-run
python3 consumer.py audit --run /new/restored-run \
  --expected-run-sha256 TRUSTED_RUN_SHA256 --native-verifier /trusted/native/crypto
python3 consumer.py export --run /new/restored-run \
  --native-verifier /trusted/native/crypto --out /new/routing-view.jsonl
```

The native binary must expose `dataset verify --out PATH`. Its success status
must cover exactly the audited completed cells. This consumer supports the
original cofactor-projection rule and the producer's revised prime-primary
projection with cyclic-membership rule, preserving their distinct names.

## Validation and evidence

Run `python3 -m unittest -v test_consumer.py` from this directory. Fixtures
are intentionally artificial format data, not valid cryptographic experiments
or training examples. Tests cover corruption, duplicate members and JSON keys,
path traversal/symlinks, forged success without inputs, transplanted metadata,
contradictory labels, arity/domain mismatches, false factor-base completion,
feature leakage, atomic export failure, native replay coverage and changing
inputs/binaries, S3 restore using mocked HTTP-facing commands, and raw/scaled/
missing/interrupted instruction-label consistency. The current suite has 25
tests; the earlier 19-test receipt remains preserved as historical evidence.

`integration_s3.py --execute --evidence NEW_DIRECTORY` performs a separate
bounded live S3 software test (under 100 kB and 32 artifact references). It
uploads an explicitly named **software fixture**, restores and hashes every
artifact, and publishes the completion receipt last, then checks its readback.
`evidence/` contains small receipts and validation metadata, not scientific
measurements. Audit snapshots discussed in `AUDIT.md` are archived separately
with a hash and S3 receipt; they are immutable observations of development code.

## Progress

Implemented the independent consumer and adversarial tests without changing
the live producer or launching duplicate generation. Independent review found
a factor-base rule compatibility error and a replay/input race in this
consumer; both have dedicated regression checks. The live S3 test records
storage behavior only. No cost improvement, attack success, or general
fall-degree statement follows from this work, so research result plots are
unchanged.

## Decisions

Retain every completed outcome and keep all mathematical labels explicitly
producer-reported. Permit unreplayed exports only with an explicit preview
flag. Use public feature allowlists instead of excluding only private roles:
manifest seeds and terminal solver traces can also reveal supervised answers.
Keep this work in a separate path and worktree from the active producer.
The shared daily session log is reserved by another owner; this scoped
handoff records this work without editing that owner's log.

## Next steps

Run the consumer against the producer's first completed native dataset and
its published receipt, then rerun native arithmetic/certificate verification
on the isolated restore. A synthetic software integration does not substitute
for that end-to-end native test. Reconcile the pinned static findings in
`AUDIT.md` with the producer's final code and tests. Extend schema support only
alongside explicit feature-time and label-validity contracts.

## Outstanding work

The full generator, native solver coverage, instruction accounting, factor
base controls, all requested curve/twist families, scientific label replay,
large-scale dataset production and cross-size component graph remain owned
by the main dataset task. The campaign budget is a total ceiling of $200,
not a daily allowance; the budget record merged in PR #2751 reserves $188.3888
under conservative assumptions, but this consumer does not enforce account-wide
spend or authorize a new compute worker.
