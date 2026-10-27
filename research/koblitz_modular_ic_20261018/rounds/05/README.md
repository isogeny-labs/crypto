# Round 05 evidence and replay

[Report](REPORT.md), [PDF](report.pdf), [proof map](PROOF_MAP.md),
[ledger](ledger.json), [protocol](PROTOCOL.md), [validation](VALIDATION.md),
[diagram](routes.svg).

This round derives the n=5 reverse composition32000 and 11-primary
splitting in written geometry, five explicit canonical polarized quotient
constructions, and one level-dependent symbolic projected principal row.
Its matrix has rank5/588; the evaluated matrix remains rank4/544 because
44 new degree210 images have not been evaluated. Reverse divisor reduction,
nine frozen graph quotients and determinant19 equations remain open.

From this directory:

```sh
sha256sum -c runs.tar.xz.sha256
tar -xJf runs.tar.xz
sha256sum -c runs.sha256
./REPLAY.sh
```

Six JSON records and the generated quotient Lean source replay byte for byte.
The checker separately verifies BigInt compositions/rank, Hermitian entries,
factorization, orbit partitions and maintained-library pole images. It does
not evaluate the new44 point images. No live download or CAS is required.

From the repository root:

```sh
cargo test --release --locked --manifest-path research/koblitz_modular_ic_20261018/native/Cargo.toml
tools/lean/proof-gate.sh research/koblitz_modular_ic_20261018/lean
```

Lean4.34.1/mathlib are pinned in `../../lean`. Eighteen new named theorems
build and audit. Full scheme/Picard and local divisor bridges remain written.
Three identity certificates bind exact polynomial claims to source hashes.

Raw outputs, historical inputs, unsuccessful attempts and validation receipts
are sealed once in runs.tar.xz; runs.sha256 covers every member. Assembly
logs beginning record- are excluded from the ledger's raw map to avoid a
circular hash, but included in the full archive manifest. All previous
archives stay unchanged. Construction and checking were by the same agent;
there is no external independent audit.

Deterministic archive check:

```sh
tar --sort=name --mtime='2026-10-19 00:00:00Z' --owner=0 --group=0 \
  --numeric-owner --mode='u+rwX,go+rX,go-w' --format=gnu -cf - runs \
  | xz -6 -T1 > /tmp/koblitz-round05-runs.tar.xz
cmp runs.tar.xz /tmp/koblitz-round05-runs.tar.xz
```

Install report-requirements.txt in a scratch environment and run
`python build_report.py`. Python only renders the frozen results and PDF.
Every rendered PDF page was inspected. No speed claim, scalar recovery,
n=131 reverse map or background experiment is supplied.
