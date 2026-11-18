# Trace-zero relation experiments — 2026-11-15

All three authorized finite-panel experiments completed: **858 primary query
executions**, with no invalid Found/Refuted decision. Every capped result is
retained. The native post-run audit rechecks all 33 known answers and replays
the 314 point tuples saved by the two new instruments without mismatch.
These are local producer checks, without independent signed review
[pcc:unverified].

| Experiment | Planned / attempted / completed | Main observation |
|---|---:|---|
| Original trace comparison | 264 / 264 / 264 | Certified and partial-trace each decide 50/66; explicit 38/66 |
| Matched encodings | 330 / 330 / 330 | Certified and constants 50/66; implicit 48/66; full-point 34/66 |
| Tau presentation | 264 / 264 / 264 | Direct and preimage 50/66; image 48/66 |

All rows above include the two repetitions of the same 33 instances; they
are not independent population samples. A/A outcomes agree in every panel;
conflict-count discrepancies occur in 8, 13 and 14 pairs respectively.
No isolated runtime, population, scaling or challenge-recovery claim follows
[pcc:unverified].

[Three-page illustrated report](report.pdf) ·
[original run](../trace_zero_experiments_20261109/reports/run-20261115T0450Z/README.md) ·
[encoding run](reports/run-20261115T0458Z-encoding/README.md) ·
[tau run](reports/run-20261115T0502Z-tau/README.md) ·
[integrity audit and exploratory profiles](integrity-audit.json).

## What the three checks establish

**1. Certified finite domains.** On the original frozen panel, explicit
membership has 28 Found, 10 Refuted and 28 UNKNOWN per 66 rows. Certified,
partial-trace and certified-AA each have 36 Found, 14 Refuted and 16 UNKNOWN.
Thus omission of redundant membership witnesses increases valid completion
by 12 query executions here; partial traces add no completions beyond
certified. These are paired fixed-panel observations, not a timing result
[pcc:unverified].

The original instrument at source `3ae8b2124` was not rebuilt. Its raw format
records original-group replay flags but omits successful point tuples.
The audit can recheck every decision against complete truth; it cannot replay
omitted original-panel tuples from those raw records. The two newer panels
save actual recovered points and support that additional replay.

The original canonical accepted artifact is
[export-final.json](../trace_zero_experiments_20261109/export-final.json).
Earlier `export.json` is a preserved, pinned adapter metadata/schema attempt,
not the accepted export. The exporter initially omitted source/binary/analysis
bindings and an added notes string violated its schema; the corrected export
passes with zero findings. No raw outcome was changed. Automation moved the
original JSONL bytes to the content-addressed store; their hashes remain in
[the manifest](../trace_zero_experiments_20261109/.artifacts.json), and their
earlier committed Git objects retain the same bytes.

**2. Allocation versus successful completion.** Constant-specialized S3
and the reference have identical completion counts here. The implicit
subspace model takes only a basis, without an enumerated base table, and
the complete finite-base full-point chain is also exercised. At n131:

| Method | Found | Refuted | UNKNOWN | Max allocated SAT variables |
|---|---:|---:|---:|---:|
| Certified S3 | 4 | 4 | 8 | 104,669 |
| Constant-specialized S3 | 4 | 4 | 8 | 4,217 |
| Implicit trace-zero membership + S3 | 4 | 4 | 8 | 6,572 |
| Full-point chain | 0 | 0 | 16 | 106,110 |

These are 16 query executions per method at n131 (eight original instances,
two repetitions). The same target/base pairs are used. Smaller allocation
does not produce more decisions in this panel. Summed conflicts include
deadline-censored rows and are not complete successful-work costs
[pcc:unverified].

**ECC2K-130 scope matters.** Its d4 base has only two points, +/-G' with one
x-coordinate. Its d5 base has eight points with four x-coordinates. All d5
n131 queries remain UNKNOWN, including planted and cancellation controls.
The successful wide S3 relations here therefore use the singleton-x base.
The public challenge Q is not a primary target: uniform targets use known
full-width sampled scalars, enabling exact known-answer checks. These runs
do not establish nontrivial ECC2K-130 relation finding [pcc:unverified].

**3. Z[tau] transport.** The direct, delta-image and delta-preimage arms
transport every actual base point and target through the bijection
delta=tau-1 on the odd-order kernel. They retain equal base cardinality and
original relation existence. Successful mapped sums are checked, brought back
and replayed in the original group. No transformed x set is assumed linear.

| Presentation, all 66 rows | Found | Refuted | UNKNOWN | Native additions + doublings |
|---|---:|---:|---:|---:|
| Direct | 36 | 14 | 16 | 390 |
| Tau image | 34 | 14 | 18 | 4,888 |
| Tau preimage | 36 | 14 | 16 | 31,238 |

At n131, all three presentations decide the same eight of sixteen queries;
transported maximum allocation reaches 61,728 variables versus 4,217 direct.
The recorded group work includes preprocessing, admission, lifting and final
replay, including censored attempts. Field/ANF construction, non-group field
inversions, Frobenius coordinate squaring and SAT work are not priced by that
unit. These totals are not runtime ratios or a full end-to-end ECDLP ledger
[pcc:unverified].

An explicitly **post-run exploratory** native profile finds that 126 x bits
are common constants for the n131 d5 base directly, versus 17 after either
transport. Constant folding can exploit this structure. This suggests keeping
original sparse coordinates in a lifted delta map as the next presentation
candidate; it does not prove the cause of every solve outcome. A free-witness
lifted delta encoding was not implemented or measured in this panel
[pcc:unverified].

## Exact protocol, controls and resource scope

The curve is E0: `y^2 + xy = x^3 + 1` over the stated odd-degree binary
field. ECC2K-130 uses degree 131 and modulus
`z^131 + z^13 + z^2 + z + 1`, with cofactor four and the original Certicom
subgroup/G from the pinned production fixture. The n83 modulus is
`z^83 + z^45 + z^2 + z + 1`; n7/9 use `z^7 + z + 1` and `z^9 + z^4 + 1`.
The n5 negative control uses `z^5 + z^2 + 1`. Exact subgroup data are pinned
by `trace_zero_leads::fixture` at the instrument commit, and every original
base/target coordinate is retained in the workload. V is the trace-zero
projection of the fixed polynomial window `span(z,...,z^d)`; it is not chosen
after seeing solver data.

- Original 33 cases are unchanged: n7/9/83/131, d4/5, planted relation,
  cancellation and two full-width uniform-scalar targets per cell, plus the
  n5 signed-base UNSAT control. Seeds and complete original point coordinates
  are in [workload.json](workload.json), identical to the original manifest.
- Arity three throughout. Candidate solvers receive no truth/witness table.
  The implicit arm gets a linear basis. The audit separately computes a
  complete original-group pair table. Identity intermediates and actual
  point signs are retained; every cap yields UNKNOWN, never false Refuted.
- Engineering pilots: 16 original, 25 encoding and 20 tau calls. The original
  n7/9 positive gate and both newer n5 positive gates pass. Every pilot row is
  valid; wide UNKNOWN is retained, not a detected positive. Pilots are separate
  from the 858 primary executions [pcc:unverified].
- Each panel uses CPU0, a 2-GiB address-space ceiling, at most 600 seconds
  including its pilot, freeze and primary run, and zero retries. Native solver
  query budgets are 2,000 cumulative conflicts and a 750-ms safety fuse.
  All three return exit zero. No cloud job or paid resource was launched.
- This is a shared local host, without L2 timing isolation. No wall time is
  admitted as an algorithm-performance metric. Exact per-row outcomes,
  counted conflicts, variables, native group counters, caps and reasons remain
  in the raw outputs. A/A conflict variation shows safety-fuse censoring.
- Deterministic native analysis was frozen before each panel's primary data;
  arm labels are deliberately unmasked for this predeclared descriptive
  tabulation. Post-run repeat counts, truth checks, saved-point replay and
  domain profiles are explicitly separated in `integrity-audit.json`.
- Every canonical export passes schema, protocol/source/analysis binding and
  file-hash checks. Flow has zero exclusions, interruptions or unattempted
  primary rows. No independent `experiment.review/v1` or signed PCC verdict
  exists; statement records remain unverified.

| Panel | Instrument source | Frozen protocol SHA-256 |
|---|---|---|
| Original | `3ae8b21241256517a9c867b4e78fb25e02ff502e` | `16186e61242f683843585a932962141c6bf0c33a49997be0955fc9cc0d360af3` |
| Encoding | `64296a66a8f69fe2c6d79c5e7969dc5d09b328da` | `02c280306ed527e8a741fafb506cb031501be867552202f4c4365173bd45805a` |
| Tau | `3c54f99045404f2b056085cc38a23b79ea691d40` | `3462390eea9a1434d5616810e40a965dbf50f3261c41352ef17c715aaeb8c29d` |

Original binary SHA256 is
`15c4d1b193999153c6ddc6ec94959167c8f4d016be197b55b26d6993001b1126`;
the two new panels share
`7689cd7efd4451b197dcffbfdfab3773e613280e1b5e196b93152c36d6f23c43`.
Execution source contents are unchanged between the latter two source commits;
the difference is protocol and artifact-custody metadata. A separate checkpoint
ref preserved the second freeze before an automation update was merged.

## Validation and reproduction

The new constants option is opt-in; default `ExplicitPoints` and existing
certified semantics are retained. Two new focused checks compare every small
kernel target across every arm and verify zero-fuse UNKNOWN semantics.
The prior 20-test point-chain/Kummer/subspace/transport suite passes on the
changed production code. Root library `cargo check --lib --no-default-features`
passes. The prior whole-root optimized-suite and benchmark-record merge
limitations remain open; focused checks do not clear them [pcc:unverified].

~~~sh
cargo test --locked --manifest-path research/trace_zero_comparisons_20261115/native/Cargo.toml -- --nocapture --test-threads=1
cargo test --locked --manifest-path research/trace_zero_point_chains_20261109/native/Cargo.toml -- --nocapture --test-threads=1
cargo check --lib --no-default-features
# Analyze retained rows; do not repeat trials to change outcomes:
research/trace_zero_comparisons_20261115/native/target/release/trace-zero-comparisons analyze research/trace_zero_comparisons_20261115/encoding/rows.jsonl
python3 ../crypto-autoresearcher/tools/experiment_protocol.py check-export research/trace_zero_comparisons_20261115/encoding/export.json --protocol research/trace_zero_comparisons_20261115/encoding/protocol.json --verify-files
~~~

The native [post-run audit](native/src/bin/audit.rs) takes workload.json and
all three raw row paths. Its exact command and output are in
[checks/audit.txt](checks/audit.txt) and `integrity-audit.json`.
[Host and binary receipt](checks/host.txt), [launch controller](checks/run-capped.sh),
[source pins](SOURCES.sha256), [new tests](checks/unit-tests.txt),
[prior regressions](checks/prior-regressions.txt) and
[root check](checks/root-check.txt) retain execution detail.
`render-reports.py` only renders recorded native results; Python performs
protocol/export serialization, graph layout and PDF rendering, never curve
arithmetic or solver experiments. All report pages were visually inspected.

The user's 2026-11-15 instruction authorized the previously described capped
local EKS/S3 exception for all three steps. No credential or validation gate
configuration changed. The PR's concurrent-main append-only ledger conflict
remains a separate [merge dependency](../../problems/trace-pr3744-ledger-base-conflict.md);
no frozen rows or claim files were rewritten.

Existing index-calculus scoreboard, progress timeline and performance-gains
graphs were checked and left unchanged: this round has no admitted runtime
or full-cost gain. Prior trace studies remain historical; current run reports
add dated evidence rather than replacing their source identity.

## Sources and remaining technical work

Sources checked (KB): tools absent and repository wrapper exit127. Fallback:
the existing Kosters–Yeo1503.08001 source copy, US7257709 condition iii,
production point-halving/addition, prior trace-zero studies and the previous
mechanism-atlas review. All candidate methods were specified before these
panels. No new mathematical originality claim is made.

The next technical candidates are branch ordering/propagation on the still
unresolved d5 cases, and a lifted tau map that retains original sparse
coordinates instead of expanding the dense image domain. Each needs a new
source-pinned pre-data comparison. A scheme-level chart over arbitrary
extension algebras and a full ECDLP cost/recovery experiment remain outside
these finite rational-point comparisons.
