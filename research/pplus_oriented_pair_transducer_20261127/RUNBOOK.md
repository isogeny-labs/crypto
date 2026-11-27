# One-shot depth census runbook

The deterministic depth-1--6 census is launched only through:

```sh
research/pplus_oriented_pair_transducer_20261127/run_census.sh
```

The wrapper enforces checkpoint admission before building or executing the
release candidate. It then validates every source-custody SHA-256, builds all
three Rust binaries with `cargo --locked --release`, refuses to overwrite any
existing `evidence/` directory, and runs these stages in order:

1. exact oriented pair transducer;
2. independently coded flat-support verifier;
3. preregistered promotion analyzer.

Candidate and verifier standard error, elapsed time, peak resident memory,
environment metadata, and SHA-256 for every evidence file are retained. A
failed stage leaves its temporary `.census.*` directory in place for diagnosis;
it is not silently retried. A successful run atomically renames that directory
to `evidence/`. There is no random seed: the protocol fixes every input and
target.

The wrapper is orchestration only. All field/group arithmetic, exact support
construction, independent replay, growth calculations, and promotion logic are
implemented in the source-manifest-bound Rust binaries.
