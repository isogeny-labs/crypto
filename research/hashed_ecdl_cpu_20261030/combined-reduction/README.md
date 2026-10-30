# Combined reduction snapshot

This unapplied candidate composes optional internal profiling with collection
target reuse. The original relation checker and full certificate verifier are
byte-identical to the shared source; both certificate replays remain charged.
The shared checkout is untouched while T-906 reserves it.

The constituent patches remain in `../optional-profile/` and `../target-reuse/`.
`source.json` pins both parent patch/source identities, the original source,
the exact combined source, patch and unchanged verifier region. Composition
adds both test cases and changes only the collector call in the profiled body;
it does not introduce another reduction implementation or algorithm.

The isolated CPU snapshot passes33 release tests, strict Clippy and formatting.
This includes identical query/certificate checks across profiling modes,
wrong-target and malformed-relation rejection, independent replay rejecting a
tampered coefficient, and exact archived certificates. Commands and log hashes
are in `validation.json`. Reproduce in fresh directories:

```sh
python3 research/hashed_ecdl_cpu_20261030/combined-reduction/validate.py   /tmp/hashed-ecdl-combined-unique /tmp/hashed-ecdl-combined-build
```

Next measure complete recovery against the profiling-only snapshot, retaining
original baseline2bd4588fba60, A/A, batch1, all three fixtures and every charged
phase. No performance claim follows from these development tests. Integration
remains separate from isolated measurement. The further10x goal is unmet.
