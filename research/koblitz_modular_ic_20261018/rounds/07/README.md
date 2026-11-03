# Round07 evidence and replay

[Report](REPORT.md), [PDF](report.pdf), [proof map](PROOF_MAP.md),
[ledger](ledger.json), [protocol and amendments](PROTOCOL.md),
[validation](VALIDATION.md), [diagram](routes.svg).

The corrected odd reverse divisor has all20 fibers in2200 point-algebra
entries, degree6600 on each side, formal F2 descent and an independently
computed forward image equal to its source. The even divisor also passes
independent forward evaluation on all7200 entries. Two complete full-map
principal rows raise the nonzero-image ranks from4 through5 to6, on675/678
columns; the all-column matrix has rank7/768.

Four determinant19 mixed quotient atlases are bound to actual torsion
points. Expanded supersingular[19] cofactor-cover equations and a Lean
polynomial-mask certificate supplement them. The intrinsic mixed fivefold
invariant image ideal remains open. General reduced Jacobian arithmetic,
higher11-power reverse fixtures and IC efficiency are not established.

From this directory:

```sh
sha256sum -c runs.tar.xz.sha256
tar -xJf runs.tar.xz
sha256sum -c runs.sha256
./REPLAY.sh
```

All50 new JSON records, the expanded polynomial text and generated Lean
source replay byte for byte. The replay uses18 historical JSON inputs
included in the archive. The first replay receipt predates the separate
geometric-base checker and covers49; ARCHIVE_VALIDATION.txt records the
final50-output replay. The earlier round06 replay also passes unchanged.
No CAS or live download is needed by the native replay after dependencies
are present. Raw failures and interrupted attempts are retained.

From the repository root:

```sh
cargo test --release --offline --locked --manifest-path research/koblitz_modular_ic_20261018/native/Cargo.toml
tools/lean/proof-gate.sh research/koblitz_modular_ic_20261018/lean
```

Lean4.34.1/mathlib are pinned in ../../lean. There are17 new named results,
18 audited declarations including the generated equation, with no native
trust or custom axiom. The two identity certificates each have64 issuer
and128 extra checks and bind their universal ring identities to Lean source.
Modular/Picard geometry and the binary-mask interpretation remain written
in PROOF_MAP.md. Construction, fresh replay and separate checks are by the
same agent; shared primitives are disclosed. There is no external audit.

The ledger excludes record-* receipts from its raw hashes to avoid a
circular binding; the archive manifest includes them. Earlier archives
remain unchanged. Deterministic repack:

```sh
tar --sort=name --mtime='2026-10-19 00:00:00Z' --owner=0 --group=0 \
  --numeric-owner --mode='u+rwX,go+rX,go-w' --format=gnu -cf - runs \
  | xz -6 -T1 > /tmp/koblitz-round07-runs.tar.xz
cmp runs.tar.xz /tmp/koblitz-round07-runs.tar.xz
```

Install report-requirements.txt in a scratch environment and run
python build_report.py. Python lays out frozen results only. Every rendered
PDF page was inspected. No background experiment is scheduled.
