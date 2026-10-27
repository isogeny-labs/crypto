# Round06 evidence and replay

[Report](REPORT.md), [PDF](report.pdf), [proof map](PROOF_MAP.md),
[ledger](ledger.json), [protocol and amendments](PROTOCOL.md),
[validation](VALIDATION.md), [diagram](routes.svg).

All44 degree210 images are zero for both signs. The evaluated matrix
has rank5/588; the nonzero-image submatrices remain rank4 on510/520
columns. All ten frozen determinant5 graphs have explicit split quotient
equations and effective F2 descent. The even reverse divisor of one
order11 generator has all20 fibers in explicit point algebras, with
positive and negative degree6600. Pointwise forward norms, Jacobian
reduction, the odd reverse divisor and determinant19 quotients remain open.

From this directory:

```sh
sha256sum -c runs.tar.xz.sha256
tar -xJf runs.tar.xz
sha256sum -c runs.sha256
./REPLAY.sh
```

The24 JSON records and generated Lean matrix source replay byte for byte.
Separate checks cover maintained-library kernel points, BigInt matrices,
two-stage closed traces, curve equations, pairing conditions, reverse lines
and ranks. Construction and checking are by the same agent and share
arithmetic primitives; this is not an external audit. No live download or
CAS is needed for the native replay.

From the repository root:

```sh
cargo test --release --locked --manifest-path research/koblitz_modular_ic_20261018/native/Cargo.toml
tools/lean/proof-gate.sh research/koblitz_modular_ic_20261018/lean
```

Lean4.34.1 and mathlib are pinned in ../../lean. Fourteen new named proofs
and the generated step equation audit without nonstandard axioms. Two
identity certificates bind direct ring identities to their Lean source;
each has64 issuer and128 extra evaluation checks. Global modular/Picard
and divisor geometry remain written in PROOF_MAP.md.

The raw archive preserves all failed attempts. The ledger excludes record-*
assembly receipts from its raw hashes to avoid a circular binding; the
archive manifest includes those receipts. All earlier archives are unchanged.

Deterministic repack:

```sh
tar --sort=name --mtime='2026-10-19 00:00:00Z' --owner=0 --group=0 \
  --numeric-owner --mode='u+rwX,go+rX,go-w' --format=gnu -cf - runs \
  | xz -6 -T1 > /tmp/koblitz-round06-runs.tar.xz
cmp runs.tar.xz /tmp/koblitz-round06-runs.tar.xz
```

Install report-requirements.txt in a scratch environment and run
python build_report.py. Python renders the frozen ledger/report only.
Every PDF page was inspected. No timing, scalar recovery or n=131 reverse
map is supplied, and no background experiment is scheduled.
