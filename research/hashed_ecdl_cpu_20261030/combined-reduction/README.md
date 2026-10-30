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
bash research/hashed_ecdl_cpu_20261030/combined-reduction/validate.sh   /tmp/hashed-ecdl-combined-unique /tmp/hashed-ecdl-combined-build
```

Next measure complete recovery against the profiling-only snapshot, retaining
original baseline2bd4588fba60, A/A, batch1, all three fixtures and every charged
phase. No performance claim follows from these development tests. Integration
remains separate from isolated measurement. The further10x goal is unmet.

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

The integrated CPU source snapshot named in the receipt passes its standalone release suite,
strict Clippy and formatting: see [the source-bound software receipt](integrated-shell-validation/receipt.json).
These checks do not extend a measured result, change a scientific status or
claim a root crypto/CUDA build. Shared implementation and frozen descriptors
were not edited by this validation-runner repair.
