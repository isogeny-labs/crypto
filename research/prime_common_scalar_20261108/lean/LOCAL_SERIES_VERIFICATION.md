# Expanded symbolic proof receipts

Pinned Lean4.35.0, CryptoLean148f5dc389b341401e06d76abe7783c4ccf42f2e,
Mathlibcfc55fc58714a59dc06e6e79edc6d1d95140d3c2. Actual mathematical source
committed in bcf15297b and46cf6eef0. The latter contains all current modules
and default build targets. No native experiment or benchmark was executed.

Clean final builds: global-build-004.log and precision-build-003.log.
The actual staged-source repository gate reports76 declarations,0 failing,
0 trusted; only propext, Classical.choice and Quot.sound. Scope includes the
added public product API in the earlier KernelNodes module. Actual receipts
are in local-series-gate/. This is producer evidence, unsigned standing.
[pcc:unverified]

From repository root, with the pinned toolchain bin directory on PATH:

```sh
lake -d research/prime_common_scalar_20261108/lean build
lake -d research/prime_common_scalar_20261108/lean env leanchecker --fresh --verbose LocalPrecision
bash tools/lean/proof-gate.sh --out /tmp/local-series-gate research/prime_common_scalar_20261108/lean
```

The official import-inclusive GlobalPolynomials fresh replay completed exit0;
its exact output is global-fresh-001.log. The expanded LocalPrecision replay
and final status are retained in precision-fresh-001.log and
local-series-fresh-status.json. Earlier checks remain snapshots of their own
source versions, not substitutes for these new module checks. [pcc:unverified]

Development failures remain in local-build-001, chart-build-001..005,
global-build-001..003 and precision-build-001..002. Raw trailing whitespace
in chart-build-001 is preserved in deterministic gzip, not edited away.
No failed build's inferred sorryAx is accepted; the final actual transitive
source audit is the relevant gate. No trust manifest, proof checker or kernel
was altered. Only definitions needed for exact imported-product APIs were
exposed; their mathematical bodies are unchanged.
