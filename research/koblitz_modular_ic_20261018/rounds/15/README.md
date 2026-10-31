# Round 15

Direct odd CM characters give lower-level modular carriers; actual theta
corrections give a3lambda target; two further coupled norms are factored;
first jets delimit the saturation search. Explicit maps, proper equations
and full saturation remain open. The user assesses IC implications.

[Report](REPORT.md) · [PDF](report.pdf) · [visual](routes.svg) ·
[CM construction](CM.md) · [theta construction](THETA.md) ·
[protocol and amendments](PROTOCOL.md) · [proof map](PROOF_MAP.md) ·
[ledger](ledger.json) · [validation](VALIDATION.md).

Extract and check the immutable raw tree with:

```sh
sha256sum -c runs.tar.xz.sha256
tar -xJf runs.tar.xz
sha256sum -c runs.sha256
./REPLAY.sh
```

The replay needs Rust, the frozen source/input files and the retained LMFDB
responses. It makes no live database query. Lean4.34.1/mathlib are pinned
in ../../lean. Run tools/lean/proof-gate.sh on that project for the full
kernel/axiom audit. Archive extraction/replay/repack receipts have their
own validation-runs.tar.xz and member manifest.
