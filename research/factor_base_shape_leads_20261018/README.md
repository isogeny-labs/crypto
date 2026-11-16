# Native factor-base shape leads — 2026-10-18

**Trace-zero geometric progressions are a finite-fixture yield lead, not a
certified regularity or speed improvement.** On
`icv1-f2m13-t181-515ee569`, two 15-column GP bases support 226 and 214
nonidentity target-subgroup points, versus 144 for the 16-column coordinate
base. Fresh holdout probes find 15/128 and 17/128 verified decompositions,
versus 4/128. A third GP seed has 21 columns and supports 482 targets;
its larger size is explicit.

[Results PDF](report.pdf) · [screen run PDF](reports/run-20261018T0437Z/report.pdf) ·
[fresh-probe run PDF](reports/run-20261018T0443Z/report.pdf) ·
[validation](validation.md) · [native observations](observations.json) ·
[independent exact census](exact-coverage.json)

## What was pursued

| Lead | Observation | Status |
| --- | --- | --- |
| Trace-zero GP shapes | Two smaller GP bases support more subgroup targets than the coordinate control; three seeds pass the frozen fresh-probe yield/rank filter | Follow-up lead on the exact degree-13 fixture; no total-cost winner |
| Same points, different basis | Exact point sets and random-query truth agree; reversed and triangular bases use more successful F4 reduction work | No component-work improvement on the tested arms |
| Rank-aware witness selection | On `icv1-f2m7-tm13-ac10a42c`, reversing one basis raises training rank 10→11 at higher F4 work | Tradeoff; first-witness selection is an unrun follow-up |
| Target-subgroup alignment | Four primary degree-31 bases have zero supported nonidentity subgroup targets despite old whole-curve controls passing | Concrete qualification gap; native CLI guard fixed |
| Proper-subfield shape | Degree-9 subfield base covers zero of the 36 nonidentity target-subgroup points | Finite-fixture rejection, not an argument against all structured bases |

The 60 research configurations finish as 59 completed cases and one failed
empty inventory. There are 43 distinct materialized signed point sets including
that empty set; repeated coordinate seeds and reparameterizations are not
independent bases. Four plans requested 3,972 probes; 3,923 trial records were
produced because the empty case failed before its 49 trials. Native verification
checks all 60 seals and 1,006 point witnesses. A second Rust arithmetic
implementation independently checks all 3,923 truth verdicts and 1,006 witnesses.
This is same-host arithmetic checking, not an independent-host research certificate.

The primary is `icv1-f2m31-tm90707-c95f16f5`: field degree 31, subgroup order
1,439,393 and cofactor 1,492. Coordinate/natural, reversed and triangular
inventories have identical signed point sets and zero target-subgroup support.
The random case also has zero; the tested trace-zero/Frobenius case supports
only two of 1,439,392 possible nonidentity targets. All 24 random probes per
primary candidate miss. The degree-83 confidence gate and exact challenge are
unperformed. Degree 7 uses a=1, degree 13 uses a=0, and degree 9 is an explicit
composite-degree/subfield control; these are separate models, not size-only joins.

## Fresh confirmation

Same target seed 2026101802, 64 training and 128 holdout draws each.
The holdout contains 122 distinct targets. Coverage is an exact independent
enumeration of two-point sums in the subgroup, out of 2,002 possible targets.

| Constructor | Columns | Train rank | Holdout witnesses | Distinct successful holdout targets | Exact coverage |
| --- | ---: | ---: | ---: | ---: | ---: |
| Coordinate | 16 | 5 | 4 | 4 | 144 |
| Random seed 17 | 12 | 3 | 4 | 4 | 78 |
| Random seed 29 | 17 | 2 | 13 | 11 | 152 |
| Trace-zero GP seed 17 | 15 | 5 | 15 | 15 | 226 |
| Trace-zero GP seed 29 | 15 | 12 | 17 | 15 | 214 |
| Trace-zero GP seed 43 | 21 | 15 | 30 | 30 | 482 |

Maximum successful procedural solving degree remains **3** in every
confirmation arm. These observations do not establish lower degree of
regularity. The native F4 reduction counter omits other operations; incomplete
counter emission prevents a total ranking at symmetric native weights
0/0.1/1 on all arms. No accepted wall-clock, end-to-end, rho or fastest claim
is made. Matched Magma/msolve/FGb/Singular comparisons were not run; those tools
were not on PATH. The exhaustive census is a costly diagnostic, not a free oracle.

## Corrective CLI change

Earlier source `b3ed888964100f1bce4780a4c0500a399c7aee50` chose any finite
curve-point sum for its positive control. A control outside the target prime
subgroup can pass while the base is useless for every online target.

Source `392129454a1668b3c4eb15014fe6c15b2fdaf205` searches for a finite
control annihilated by the recorded prime subgroup order. If none exists,
it retains an unqualified failure with its inventory and exact reason.
Setup group operations include that search, under the original deadline.
Completed records declare `positive_control_scope: target-prime-subgroup`.
Two valid controls pass; the subfield and subgroup-empty primary controls fail
in the new four-case regression. Independent arithmetic checks the new controls.

The standalone installed CLI is updated; launch syntax is unchanged:

```sh
factor-base-sweep run --out runs/my-bases
```

The old native executable is retained locally for byte-bound resumes at
`/workspace/fb-shape-leads-20261018/original-factor-base-sweep`.
Old sealed research and authoring evidence keep their original control scope.
No old inventory, commitment, ledger row or historical trace is rewritten.

## Reproduce and inspect

The frozen [screen protocol](PROTOCOL.md),
[fresh-probe protocol](FOLLOWUP_PROTOCOL.md) and
[independent census protocol](COVERAGE_PROTOCOL.md) precede their respective runs.
Exact plans and checksums are in [plans/](plans/). Build the recorded native
source and invoke the CLI; fresh runs use new output directories. Replays of
completed records do not require rerunning F4.

```sh
sha256sum -c runs.tar.xz.sha256
tar -xJf runs.tar.xz
sha256sum -c RUNS_SHA256SUMS
factor-base-sweep verify --run runs/primary
factor-base-sweep verify --run runs/secondary
factor-base-sweep verify --run runs/confirmation
factor-base-sweep verify --run runs/representation
cargo build --locked --release --manifest-path analysis/Cargo.toml
analysis/target/release/factor-base-lead-audit runs primary secondary confirmation representation
analysis/target/release/factor-base-lead-audit --coverage runs ../../docs/curves/registry.json primary secondary confirmation representation
```

With a custom `CARGO_TARGET_DIR`, use the binary under that target directory.
The displayed build path assumes a normal local Cargo configuration. Rust is
needed only to build the native auditor; it does not orchestrate through Python.
`render-recorded-plots.py` lays out recorded native values only.

Raw records are one deterministic GNU tar/xz archive with normalized date,
owner and modes. The member manifest and a byte-identical extracted-tree
repack are checked before publication. Analysis uses the extracted values;
the published JSON copies match the native auditor's output.

## Remaining leads

The new card [XP-20261018-factor-base-shape-witness-leads](../../backlog/XP-20261018-factor-base-shape-witness-leads/README.md)
proposes cofactor-aware shape construction and rank-aware witness selection.
It remains draft, unrun, zero-funded and unleased, without a frozen runnable
protocol. Projection into the subgroup can destroy a linear low-degree
membership representation; that map/oracle obligation is unresolved.

Canonical graphs checked and left unchanged:
`docs/index-calculus-scoreboard.html`, `docs/ic-leaderboard.html` and its twins,
`docs/ic/progress-timeline.json`, and the lab browser roster. No admitted
full-cost value, registered curve/method, ecbench session or IC1 tournament
record was added. The study adds its own figures and a benchmark browsing
copy of these diagnostic records. No historical data, S3 object, registry,
alias/store entry or #1963 planning card is changed.
