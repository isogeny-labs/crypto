# Round 02 evidence and replay

[Report](REPORT.md), [PDF](report.pdf), [protocol](PROTOCOL.md),
[ledger](ledger.json), [proof map](PROOF_MAP.md), [diagram](routes.svg).

Raw JSON outputs and logs are archived once in `runs.tar.xz`; extraction
restores `runs/`. `runs.sha256` hashes every extracted record. The summary,
protocol, proof map, report and integer identity certificate remain loose
for review; prior round artifacts are unchanged.

From this directory:

```sh
sha256sum -c runs.tar.xz.sha256
tar -xJf runs.tar.xz
sha256sum -c runs.sha256
./REPLAY.sh
```

The default replay uses a fresh temporary directory, checks thirteen output
files byte for byte and runs separate transfer/relation certificate checkers.
All arithmetic and result assembly are Rust. `build_report.py` performs only
layout of recorded results and renders the PDF for visual inspection.

From the repository root:

```sh
cargo test --release --locked --manifest-path research/koblitz_modular_ic_20261018/native/Cargo.toml
tools/lean/proof-gate.sh research/koblitz_modular_ic_20261018/lean
lean research/koblitz_modular_ic_20261018/CyclicWindow.lean
```

Lean 4.34.1 and the exact mathlib commit are pinned in `../../lean/`.
The proof gate fetches the cache, builds the module, and audits transitive
axioms. The mathematical interpretation limits are in PROOF_MAP.md.

Repack the extracted tree with the same bytes:

```sh
tar --sort=name --mtime='2026-10-18 00:00:00Z' --owner=0 --group=0 \
  --numeric-owner --mode='u+rwX,go+rX,go-w' --format=gnu -cf - runs \
  | xz -6 -T1 > /tmp/koblitz-round02-runs.tar.xz
cmp runs.tar.xz /tmp/koblitz-round02-runs.tar.xz
```

The user assesses implications for IC efficiency. No performance result or
matched timing experiment is recorded here. The next experiments listed in
the ledger are proposals, not background jobs.

[VALIDATION.md](VALIDATION.md) records the exact local checks and outcomes.
To rebuild the document, create a scratch virtual environment, install the
pinned `report-requirements.txt`, and run `python build_report.py`. This
layout-only step requires no cryptographic execution.
