# Round08 evidence and replay

[Report](REPORT.md), [PDF](report.pdf), [proof map](PROOF_MAP.md),
[ledger](ledger.json), [protocol](PROTOCOL.md), [validation](VALIDATION.md).

The complete line-norm family gives evaluated rank12/1308 and nonzero-image
rank11 for both signs, on1165/1160 columns. Exact geometric joins find540
new columns after round07. Separate F32 arithmetic checks every image
coordinate and principal sum, and separate elimination confirms the ranks.

Each of four determinant19 mixed quotients has two intrinsic Kummer
equations and six explicit covering charts over F2^360. Forty polynomial
Miller functions have checked divisors;200 exact rational transition
identities and verified19th roots give the global atlas. Effective F2
coordinate descent, a closed projective embedding, Jacobian identification
and individual descent remain open. No IC cost is measured.

From this directory:

```sh
sha256sum -c runs.tar.xz.sha256
tar -xJf runs.tar.xz
sha256sum -c runs.sha256
./REPLAY.sh
```

All eight new JSON outputs replay byte for byte. The first replay receipt
covers the initial six outputs before global gluing; the final archive
replay covers all eight. Nineteen historical JSON inputs are sufficient
for REPLAY.sh; an additional unused historical polarization input is
retained in the raw archive. Round07 also replays unchanged.

Construction, replay and separate checks are by the same agent and share
declared primitives. There is no external audit. Sixteen new named Lean
results and one generated equation pass the kernel build/transitive audit.
The global Picard/isogeny identification and interpretation of native
finite fields remain written in PROOF_MAP.md.

From the repository root:

```sh
cargo test --release --offline --locked --manifest-path research/koblitz_modular_ic_20261018/native/Cargo.toml
tools/lean/proof-gate.sh research/koblitz_modular_ic_20261018/lean
```

The ledger binds exact sources and raw files, excluding record-* assembly
receipts to avoid circularity. The archive manifest includes those receipts.
Failed builds and the incomplete-metadata M11 attempt remain in the archive.
No earlier archive was modified. Deterministic repack:

```sh
tar --sort=name --mtime='2026-10-19 00:00:00Z' --owner=0 --group=0 \
  --numeric-owner --mode='u+rwX,go+rX,go-w' --format=gnu -cf - runs \
  | xz -6 -T1 > /tmp/koblitz-round08-runs.tar.xz
cmp runs.tar.xz /tmp/koblitz-round08-runs.tar.xz
```

Install report-requirements.txt in a scratch environment and run
python build_report.py. Python lays out frozen evidence only. Every PDF
page was visually inspected. No background experiment is scheduled.
