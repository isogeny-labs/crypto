# Shared norm core: exact graph elimination and cold IC

The native core substitutes graph/public coordinates before calling the shared
norm algebraic solver. At depth4 it has15,18,22 variables for arities3,5,7,
compared with24,42,60 in the witness-pruned presentation; equations remain
of degree at most3. All90 complete tiny model sets and18 cold scalar recoveries
match the independent OpenSSL replay. The dense arity controls remain unknown.
PCC-20261115-be28ec:unverified. [pcc:unverified]

This is a descendant of the [parent prototype](../shared_norm_algebraic_20261115/README.md),
whose initial publication is [PR4234](https://github.com/isogeny-labs/crypto/pull/4234).
The owner requested a complete end-to-end alternative to tensors. The fixed-
fixture path is functional; a complete classical subexponential prime-field
algorithm with sufficient cost/yield bounds is still open. [pcc:unverified]

## What changes

[core.cpp](core.cpp) first constructs the existing unknown witness-pruned
system, then applies exact polynomial substitutions:

1. The target equation determines a0 with unit coefficient.
2. Synthetic division of A^2-f C^2 by monic X-x_R determines every U coefficient.
3. Before the first wrap modulo degree-m U, V_j=X^(2^j) and Q_j=0 are public.
4. The first wrapping quotient has a public degree bound; remove its higher
   zero coefficients. Keep all subsequent retained unknown layers and the
   target inverse. Do not expand X^(2^t).

[derivation.md](derivation.md) gives the projection argument and hypotheses.
Every extracted core assignment is lifted and checked against **every original
witness-pruned equation**, then decoded and re-added on the curve. The backend
receives unknown coefficients only. The generic cold recovery code is reused
through a templated oracle parameter; production group arithmetic, row/kernel
selection and final scalar checks are unchanged. [pcc:unverified]

The remaining variable count is still O(mt); the public prefix saves
O(m log(m)) coordinates and the norm graph O(m). No complexity conclusion
follows from these deterministic encoding counts. This is standard monic-
division preprocessing, not a novelty claim. [pcc:unverified]

## Frozen exact software controls

Source b5e2b31d4235834ad37486091ca4acf6470d16e4; all hashes captured before
backend calls and still match. One thread,20s CPU/wall,2GiB address space,
quotient128; same pinned msolve0.7.5/FLINT3.1.3 backend as the parent.

| Control | Producer-local exact outcome |
| --- | --- |
| Structural checks | planned15/18/22 counts; degree<=3; symbolic norm/target cancellation; coordinate round-trip; bad admission rejected |
| Rebuilt parent controls | quotient/recovery selftests and retained28-model identity check pass |
| m3 complete point sets |90 cases,400 models, exact set equality; OpenSSL agrees |
| Cold point-only IC |18 targets,72 signed relations,18 kernels; OpenSSL agrees |
| m5,t3,c16,R=(0,6) |2 complete models; quotient dimension8 |
| m5,t4,c1,R=(0,6) |backend exits139, empty basis; unknown |
| m7,t4,c1,R=(0,6) |backend cap124, empty basis; unknown |

[PROTOCOL.md](PROTOCOL.md) predates code and affected calls; D1/D2 diagnostic
amendments predate their calls. These are finite deterministic development
controls, not a scientific performance comparison, population estimate or
scaling campaign. Incidentally printed backend clocks are retained raw and
are not admitted timing evidence. No leaderboard or paid experiment changes.
[pcc:unverified]

## Crash diagnosis and bounded remedy

The sparse and sparse/dense D1 backends both reproduce exit139 on the same
public dense m5 input. A D2 memory-only strace then records repeated ENOMEM
from mmap/mremap and a null-address SIGSEGV under the unchanged2GiB address
limit. This explains the diagnostic reproduction as allocation failure followed
by a backend crash; it is not a mathematical refutation. Empty basis files
supply no relation and no UNSAT certificate. [pcc:unverified]

A separate D2 call with maximum pair batch512 reaches its unchanged cap and
exits137, still with no basis. Its partial F4 log reaches degree7. That is not
an input-ideal certificate degree or a completed basis bound. The original
unknown/crash/cap records are preserved; no call was rerun until green or given
more memory/time. There is no basis available for a model claim on either
dense case. [pcc:unverified]

## Shared formal support

[crypto-lean PR7](https://github.com/aburan28/crypto-lean/pull/7), pinned source
52bc3fbe8b5f8c92a9f8958319e0762e4414a8ba, supplies three generic laws:

- actual retained transitions imply powering congruence over a commutative ring;
- actual target-point/chart equations imply exact monic linear norm division;
- retained terminal membership, nonzero2/c and target separation discharge the
  existing separability/coprimality charts.

Assembly probe, focused build, explicit axiom print, module audit and full
shared-library gate pass:101 theorems, zero failing/trusted native proofs.
Only propext,Classical.choice,Quot.sound occur in the new declarations. Exact
pins, inherited laws and boundaries are in [PROOF-MAP.md](PROOF-MAP.md),
[proof-dependencies.json](proof-dependencies.json) and checks/. Full coefficient
graph equivalence, bounded witness lifting, native implementation and the
asymptotic solver cost/yield theorem are not formalized by these laws.
PCC standing remains unverified. [pcc:unverified]

## Reproduce and inspect

Use the parent's pinned bootstrap and the new thin build script:

```sh
bash research/shared_norm_algebraic_20261115/bootstrap_backend.sh /tmp/norm-prefix
bash research/norm_core_elimination_20261115/build.sh /tmp/norm-prefix /tmp/core-build
/tmp/core-build/core --selftest
LD_LIBRARY_PATH=/tmp/norm-prefix/usr/lib/x86_64-linux-gnu \
 /tmp/core-build/core --solve /tmp/norm-prefix/usr/bin/msolve /tmp/new-core-solve 6 3 1234
```

All CLI solves use the fixed F17 curve/order19 fixture and point coordinates.
For the exact panels, use `--qualify-models`, `--qualify-arity`, or
`--qualify-cold` with arguments BACKEND NEW_OUTDIR SUMMARY.tsv. Replay models
or cold traces with the separately compiled `/tmp/core-build/replay` using
`--models` or `--cold` and the same directory/summary. Never reuse output paths.
[pcc:unverified]

The [three complete raw panels](evidence/) include every input, output, status,
assignment and trace. Their whole-panel xz sizes are52240,2144,16536 bytes,
naturally below64KiB. Native panel source/binary hashes predate execution;
compiler/OpenSSL metadata and protocol history are retained. The native archive
also includes actual focused Lean receipts. Full proof-gate receipts are in
checks/. [pipeline.svg](pipeline.svg) and its editable DOT explain the flow;
[report.pdf](report.pdf) is the inspected two-page source-linked report.

## Next algorithmic work

The remaining central question is initial elimination with charged failure cost
and effective useful-target yield. The next implementation target is an explicit
capped Macaulay procedure that retains input-row provenance and verifies a
returned basis. This supplies a reviewable solver/certificate interface;
proving that a sufficient fraction of growing natural targets finish within
an exp(o(n)) cap remains a separate obligation. A small final quotient, degree7
partial log or these fixed controls does not settle it. [pcc:unverified]

Sources checked: the parent source/proof map, original norm continuation and
initial-elimination question, standard Mathlib monic division laws, pinned
msolve help. KB unavailable; no novelty/exhaustiveness claim. The auxiliary
arbitrary-prime rational-isogeny membership implementation is still absent.
