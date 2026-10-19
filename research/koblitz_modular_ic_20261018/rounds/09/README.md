# Round09 evidence and replay

[Report](REPORT.md), [PDF](report.pdf), [proof map](PROOF_MAP.md),
[ledger](ledger.json), [protocol](PROTOCOL.md), [validation](VALIDATION.md).

Four determinant19 quotients have explicit Frobenius coordinate circuits
and4,320 invariant rational generating circuits each. Exact class/slice
identities, chart compatibility, cocycle controls and trace-dual recovery
pass. F2 polynomial relations, saturation and closed embeddings remain
open. All50 signed twist/reflection controls fail in the specified known
component; this is not a universal exclusion. Eight coupled functions are
nonzero on720 known norm columns, without complete divisors or new rank.
Seventeen new named Lean results pass. No timing is measured.

From this directory:

```sh
sha256sum -c runs.tar.xz.sha256
tar -xJf runs.tar.xz
sha256sum -c runs.sha256
./REPLAY.sh
```

Five historical JSON inputs suffice for replay; additional copied inputs
are retained in the archive for provenance. All five new JSON outputs
must compare byte for byte. Round08 replays unchanged. The ledger binds
native and Lean sources, linked root primitives, pins, protocol and replay,
and every raw file except record-* assembly receipts to avoid circularity.
The archive member manifest includes those receipts. Earlier evidence is
unchanged. Deterministic repack:

```sh
tar --sort=name --mtime='2026-10-19 00:00:00Z' --owner=0 --group=0 \
  --numeric-owner --mode='u+rwX,go+rX,go-w' --format=gnu -cf - runs \
  | xz -6 -T1 > /tmp/koblitz-round09-runs.tar.xz
cmp runs.tar.xz /tmp/koblitz-round09-runs.tar.xz
```

Same-agent exact checks and a separate parser share declared primitives;
no external audit is claimed. See PROOF_MAP.md for the boundary between
Lean proofs, native calculations and written global geometry. Run the
pinned project with tools/lean/proof-gate.sh from the repository root.
Install report-requirements.txt in a scratch environment and run
python build_report.py to render frozen evidence only. All PDF pages are
visually inspected. No background experiment is scheduled.
