# One-shot depth census runbook

The deterministic depth-1--6 census is launched only through:

```sh
research/pplus_oriented_pair_transducer_20261127/run_census.sh
```

The wrapper enforces checkpoint admission before building or executing the
release candidate. Because S3 is unavailable, the default gate requires a
clean tracked checkpoint whose local head exactly matches the same branch on
`origin`. It then validates every source-custody SHA-256, builds all three Rust
binaries with `cargo --locked --release`, refuses to overwrite any existing
`evidence/` directory, and runs these stages in order:

1. exact oriented pair transducer;
2. independently coded flat-support verifier;
3. preregistered promotion analyzer.

Candidate and verifier standard error, elapsed nanoseconds, explicit
`max_rss_kib=unavailable`, environment metadata, and SHA-256 for every evidence
file are retained. A
failed stage leaves its temporary `.census.*` directory in place for diagnosis;
it is not silently retried. A successful run atomically renames that directory
to `evidence/`. There is no random seed: the protocol fixes every input and
target.

The wrapper is orchestration only. All field/group arithmetic, exact support
construction, independent replay, growth calculations, and promotion logic are
implemented in the source-manifest-bound Rust binaries.

The active custody record is `SOURCE_MANIFEST_3.json`; v1 and v2 are retained
as historical pre-amendment records. If S3 transport later becomes available,
`PPLUS_CHECKPOINT_MODE=s3` selects the repository S3 gate explicitly.

Failed attempt 1 is separately retained and hash-bound. It stopped before
candidate execution because `/usr/bin/time` was unavailable; amendment 4
replaced that non-scientific dependency before the single permitted retry.
