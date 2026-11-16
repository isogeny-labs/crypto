# Round 03 evidence and replay

[Report](REPORT.md), [PDF](report.pdf), [protocol](PROTOCOL.md),
[ledger](ledger.json), [proof map](PROOF_MAP.md), [diagram](routes.svg),
[validation](VALIDATION.md).

The raw tree is archived once as `runs.tar.xz`. It contains the native
outputs, all intermediate failed/partial runs, exact command receipts,
and pinned published genus-5 and genus-6 inputs. `runs.sha256` covers every
member. The summaries, proofs, protocol, integer identity certificates and
editable document sources stay loose. Earlier rounds are unchanged.

From this directory:

```sh
sha256sum -c runs.tar.xz.sha256
tar -xJf runs.tar.xz
sha256sum -c runs.sha256
./REPLAY.sh
```

Replay regenerates eight final JSON outputs byte for byte, then checks the
polarization kernels and n=5 divisor relations with separate native algorithms.
It uses the frozen local census inputs; it does not fetch mutable data or
execute Sage. Generator and verifier were run by the same agent. External
independent replay has not been claimed.

All research arithmetic and ledger assembly are Rust. Lean 4.34.1 and the
exact mathlib revision are pinned in `../../lean`. From the repository root:

```sh
cargo test --release --locked --manifest-path research/koblitz_modular_ic_20261018/native/Cargo.toml
tools/lean/proof-gate.sh research/koblitz_modular_ic_20261018/lean
```

The new module has thirteen named lemmas; its audit counts fourteen
including the generated defining equation. The previous thirteen named
lemmas remain checked. See PROOF_MAP.md for the geometric and census
hypotheses that are not formalized.

Repack the extracted tree with the same bytes:

```sh
tar --sort=name --mtime='2026-10-18 00:00:00Z' --owner=0 --group=0 \
  --numeric-owner --mode='u+rwX,go+rX,go-w' --format=gnu -cf - runs \
  | xz -6 -T1 > /tmp/koblitz-round03-runs.tar.xz
cmp runs.tar.xz /tmp/koblitz-round03-runs.tar.xz
```

For the PDF, install `report-requirements.txt` in a scratch environment and
run `python build_report.py`. This script only lays out recorded quantities.
No performance experiment is recorded; the user assesses IC implications.
The actual F2 modular-factor divisor evaluator and its interior relations
remain open in the shared carrier-transfer problem record.
