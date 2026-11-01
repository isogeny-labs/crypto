# Round17: integral maps, odd theta lifts and survivor exclusions

[Report](REPORT.md) · [PDF](report.pdf) · [integral map](CM.md) ·
[theta descent](THETA.md) · [saturation](SATURATION.md) ·
[proof map](PROOF_MAP.md) · [protocol](PROTOCOL.md) · [ledger](ledger.json).

Actual integral lattice controls preserve the fixed11 line in the forward
map while the natural reverse kills it. A written Tate/local-lattice
argument supplies prime-to11 map existence under the modular bridge;
an evaluated correspondence remains open. The odd cover has exact Smith
kernel E[3]×E[105] and normalized factored theta lifts. All four Q22 models
are excluded; four new Q24 survivors keep full saturation open. No IC
cost or efficiency assessment is supplied.

The raw tree, including failed attempts, is frozen in runs.tar.xz;
fresh extraction/replay receipts are in validation-runs.tar.xz. Check:

```sh
cd research/koblitz_modular_ic_20261018/rounds/17
sha256sum -c runs.tar.xz.sha256 validation-runs.tar.xz.sha256
tar -xJf runs.tar.xz
tar -xJf validation-runs.tar.xz
sha256sum -c runs.sha256 validation-runs.sha256
./REPLAY.sh
../../native/target/release/koblitz-modular-carriers --round16-verify-sources .
```

The generic source verifier keeps its Round16 CLI name. Earlier rounds
are checked at their original Git snapshots, never by rewriting frozen
ledgers. REPLAY.sh repeats six commands, compares six JSON outputs and
five generated Lean modules, and checks three new source-bound IDC records.
The manual structural module is built/audited by the full proof gate.

The editable diagram/count graph, SVG,150dpi PNG and inspected PDF are
produced by render.py from native recorded values only. Canonical frontier
prose/source hashes are refreshed; historical IC cost/rank charts retain
their values. ARCHIVE_VALIDATION.txt and VALIDATION.md record completed
checks and exact remaining gaps.
