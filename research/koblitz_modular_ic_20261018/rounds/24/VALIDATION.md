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

## Integrated-base local run, 2026-10-28 UTC

The new run is bound to integrated source7fcc0e26b64593e3372cc2751efe0511588b89af.
Root01 fails before tests (exit101):38 missing include diagnostics,31 exact
HEAD blobs restored after scope checks. Root02 runs5077 tests under the
original1200s cap and exits124 without a whole-suite summary. Its log has27
observed FAILED test lines.81 exact known runtime blobs were restored after
the tests had started; this explains why those inputs cannot make that
capped run green retrospectively. No root implementation or expectation changes.

Targeted diagnostic01 selects precisely those27 names from the same test
binary:21 pass,6 fail. Five fail on absent emulator source; the sixth is the
inherited ic.hyperelliptic registry contract. After clear scope and exact
Git-blob verification of emulator.cpp/arithmetic.h/wire.hpp, diagnostic02
replays the same27:26 pass,1 fail. The remaining test is
cryptanalysis::ecbench::methods::tests::legacy_numeric_hec_contract_stays_distinct_from_thread_controls;
resolve reports unknown method ic.hyperelliptic. This research package does
not patch the unrelated registry. No further whole-suite run is performed.

Raw software receipts are sealed separately as validation.tar.xz with
validation.sha256/validation.files and validation.json. They do not alter
R24's immutable70-member scientific archive or source19b8ef5e8. Targeted
passes are diagnostics only; the final integrated root suite is not green.
