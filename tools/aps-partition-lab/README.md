# Bounded APS partition lab

Native structural experiments for the partition proposals in
[the local design](../../research/aps_cost_leads_20261106/LOCAL-EXECUTION-DRAFT.md).
This executable is not a DLP solver, benchmark or experiment-admission tool.

The coloring experiment fixes point IDs, family generation and caps before
the call. It enumerates unordered distinct tuples and assigns their first
qualifying coloring. A separate cartesian traversal of the color cells must
produce the same represented union. Explicit uncovered tuples are retained;
a capped family is not automatically a complete cover. The hash mixer is a
deterministic fixture construction, not a cryptographic or perfect-hash claim.

The signature experiment expands the standard third summation polynomial
after fixing its third coordinate, and groups by nonzero monomial support
and coordinate degrees. A separate literal expression checks the expansion;
ordinary-residue affine group addition checks every distinct-coordinate
root over the finite nonzero-ordinate coordinate pool. Each specialization
checks unordered pairs: the reported counts are specialization incidents,
not counts of unique unordered triples. Two-torsion and repeated-coordinate
cases are outside this root-oracle comparison and are not silently treated
as absent curve points.

The source formula is the short-Weierstrass f3 of
[Semaev](https://eprint.iacr.org/2004/031); the
[existing source review](../../research/aps_leads_20261105/SOURCES.md) records
related APS and factor-base work. No novelty claim.

Same-signature/different-root counterexamples are retained when found. The
feature buckets cannot authorize reuse of numerical roots or a basis.

An additional tiny known-answer group control constructs points as aG+bQ,
canonicalizes their signs with the labels, recolors the same point IDs, and
retains every signed zero relation including B=0. Informative relations must
pass a separate scalar check. This is an order-19 public fixture, not the
production APS sampler or a large-curve solve. Recoloring does not inherit
the original one-point-per-slot informative-label guarantee.

The production APS sampler and existing certified F4 trace path are not
exercised by this standalone tool. Both remain next integration steps,
rather than mocked successful checks.

## Commands

```sh
cargo test --release --locked --manifest-path tools/aps-partition-lab/Cargo.toml
cargo clippy --locked --manifest-path tools/aps-partition-lab/Cargo.toml --all-targets -- -D warnings
cargo fmt --check --manifest-path tools/aps-partition-lab/Cargo.toml
cargo build --release --locked --manifest-path tools/aps-partition-lab/Cargo.toml
```

For a structural call, copy the built executable to an immutable qualifier
path and hash that copy before execution. Record source-file hashes, lockfile,
compiler and qualifier identity in distinct build and pre-call manifests;
create those manifests exclusively and do not overwrite them. Pass the
actual source revision and source-manifest digest, then use a new output path:

```sh
QUALIFIER structural --source-revision HEX40 --source-manifest-sha256 HEX64 --out NEW_REPORT.json
```

Those user-supplied pins are metadata, not an execution attestation. The
caller must check them against the actual immutable source and binary.
The output contains no timing, calibrated costs or speed ranking. Structural
correctness calls do not qualify a later performance trial; that still needs
the prospective checkpoint, complete accounting, frozen protocol and run
admission. Keep private ledgers/logs and detailed host data out of exports.
