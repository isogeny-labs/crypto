# Round 04 evidence and replay

[Report](REPORT.md), [PDF](report.pdf), [proof map](PROOF_MAP.md),
[ledger](ledger.json), [protocol](PROTOCOL.md), [validation](VALIDATION.md),
[diagram](routes.svg).

This round adds explicit natural polarized quotients and integral presentations
for fourteen gluing cases, complete minimum-3 certificates, actual modular
11-isogeny transport and factor projections, and 102 direct modular principal
relations of rank 4 on 544 closed columns. There is no recovery or timing claim.
The original source-group-to-Jacobian transfer requirement remains open.

Raw outputs, the prior polarization input, the published phi11 input, failed
runs and validation receipts are archived once as `runs.tar.xz`.
`runs.sha256` covers every extracted member. Reports, proof maps, identity
certificates, ledger and editable document sources remain loose. Prior rounds
are not rewritten. Generation and separate checking were done by the same
agent; no independent external audit is claimed.

From this directory:

```sh
sha256sum -c runs.tar.xz.sha256
tar -xJf runs.tar.xz
sha256sum -c runs.sha256
./REPLAY.sh
```

Replay regenerates seven JSON outputs byte for byte and runs the separate
maintained-library/divisor/BigInt checker. Historical unpaired metadata is
serialized in its matching original format. No live download or CAS is used
by replay. The fixed integer modular polynomial is from
[Sutherland](https://math.mit.edu/~drew/ClassicalModPolys.html).

From the repository root:

```sh
cargo test --release --locked --manifest-path research/koblitz_modular_ic_20261018/native/Cargo.toml
tools/lean/proof-gate.sh research/koblitz_modular_ic_20261018/lean
```

Lean4.34.1/mathlib are pinned in `../../lean`. Eleven new named lemmas
build and audit; full modular moduli/Picard and polarization bridges are
listed in PROOF_MAP.md. All research arithmetic and record assembly are Rust.

Repack byte for byte:

```sh
tar --sort=name --mtime='2026-10-18 00:00:00Z' --owner=0 --group=0 \
  --numeric-owner --mode='u+rwX,go+rX,go-w' --format=gnu -cf - runs \
  | xz -6 -T1 > /tmp/koblitz-round04-runs.tar.xz
cmp runs.tar.xz /tmp/koblitz-round04-runs.tar.xz
```

Install `report-requirements.txt` in a scratch environment and run
`python build_report.py` to rebuild the document. This performs layout only.
The diagram reads quantities from ledger.json. The user assesses IC
implications; the next experiments are proposals, not background jobs.
