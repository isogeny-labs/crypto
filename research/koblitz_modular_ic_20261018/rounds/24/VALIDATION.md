# Round24 verification scope

Native Singular4.4.1: cf24-factor-01, o24-charts-02 and o24-series-02 pass.
O24-charts-01 and o24-series-01 are rejected implementation failures, with
their full scripts, errors and source revisions retained. Cf24-corrupt-01
is an expected rejection: native product residual1/error, native exit0,
later marker present, wrapper exit1. Acceptance requires exit0, marker and
no error diagnostics together. Six invocations total, no timeout or migration
resume, same4GiB virtual/1200s bounds throughout.

Lean02 recompiles all transitive project imports and audits14 new theorems,
0 failing and0 trusted-native. Lean01 is rejected with elaboration errors and
its exact failed source is retained. Standard foundations only. The full
normalization/proper/Hensel/constant-field bridge is written, not formalized;
PROOF_MAP.md lists the exact scope. Lean4.34.1 and mathlib d13f23b are pinned.
lean4checker is absent; no fresh-checker pass is claimed.

The four O-series branches have zero chart residuals modulo w192 and zero
residuals for all five original equations modulo w100. Their nu expansions
have effective precision at least115 after denominator cancellation. All
claimed leading terms have order below100. The helper's unit/order1/order7
controls and all8 actual registered entries are checked before acceptance.

The source checkpoint after the archive-ignore fix was freshly downloaded;
its exact ./untracked.tar.gz member includes checkpoint-evidence.tar.xz.
No host migration occurred while the earlier checkpoints omitted that archive.
Final raw/source manifests and the artifact-store receipt are separate records.
All R01-23 immutable evidence remains as published.

## Software and CI are separate

R24 changes native research source, Lean and documentation; it adds no root
Rust implementation or test expectation. R23's last complete library run
was4913 passed,3 failed,122 ignored, followed by a passing3-test diagnostic.
It did not establish a single green full run. Do not present that older
receipt as a new full run after merging the advanced remote PR branch.

At the first live PR check this session, remote head b6f501148 had upstream
base merges and unrelated Rust changes; backend-leaderboard/current,
clippy(crypto) and ecbench/harness failed. Other reported checks passed or
were skipped. Those outcomes apply to that head only. Auto-merge had been
enabled externally and was disabled to preserve this task's no-merge rule.
Remote history will be merged without force-pushing; final software/check
status is recorded in the publication receipt, not inferred from math success.
