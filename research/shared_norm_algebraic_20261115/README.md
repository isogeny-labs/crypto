# Shared-norm algebraic index calculus: fixed-fixture end-to-end prototype

**The requested subexponential algorithm is still open.** This component
implements a complete cold recovery path on a fixed tiny curve, using
algebraic norm/remainder solving in place of the earlier coefficient census.
The source is native C++ with the pinned msolve/FLINT backend. All observations
are producer-local, **PCC-20261115-be28ec: unverified**. [pcc:unverified]

The owner selected a complete end-to-end algorithm and asked to investigate
an approach different from tensor contraction. This study advances the
unfinished shared-divisor coefficient route. It does not claim a new
subexponential theorem, a performance win, or a general prime-field solver.

## What the candidate actually does

The cold solver receives the public point Q and a development RNG seed. For
the declared F17 fixture it constructs the factor base by factoring X^8-1
and the curve lift quadratics, admits the whole order 19 using an order-19
point and Hasse's interval [10,26], draws a,b and forms R=aP+bQ. The oracle
receives the curve/domain and R only. It builds the retained quadratic shared
norm/remainder system with every coefficient unknown, calls native msolve,
extracts rational assignments, and independently checks each decomposition.

After B+1 accepted signed rows, a left dependency is selected from C alone.
The label sums alpha,beta recover k=-alpha/beta; zero denominators restart.
The returned scalar is verified by multiplying P. Factor-base logs, a hidden
target scalar, a norm witness, and decomposition tables do not enter the
candidate. Field-point enumeration remains solely in the bounded reference
and harness, outside the final cold candidate. [pcc:unverified]

For odd m=2h-1 the system has monic A of degree h, C of degree at most h-2,
monic U of degree m, and retained quotient/remainder layers:

```text
A^2-(X^3+aX+b)C^2 = (X-x_R)U,   A(x_R)-y_R C(x_R)=0
V_0=X,   V_(j-1)^2=Q_j U+V_j,   V_t=c.
```

Bezout witnesses enforce the declared C/U and squarefree charts; a target
inverse enforces U(x_R) != 0. Extraction recovers y=-A(x)/C(x), and checks
curve membership, subgroup, factor-base membership and the exact point sum.
The generator supports m=3,5,7 within its explicit bounded interface.

A separate `--solve-pruned` mode omits the two Bezout witness blocks. For
nonzero c in odd characteristic, membership implies squarefree U; target
separation and the norm identity then imply gcd(U,C)=1. The
[projection argument and formalization map](chart-pruning.md) explain the
hypotheses and the 9/16/23 removed variables at arities3/5/7. The shared Lean
library checks the supporting separability/coprimality laws; bounded witness
lifting and the complete circuit endpoint remain unformalized. The original
explicit-chart mode and historical records are preserved. [pcc:unverified]

## Rational extraction without a separating linear form

The retained m3,t4 system has 28 rational solutions over F17. No F17-valued
linear form can separate 28 points. The current one-linear-form extractor in
PR #4091 reports unknown on collided kernels. Here coordinate multiplication
matrices act on the dual of the finite quotient algebra. For each coordinate,
take its exact characteristic polynomial on the current invariant subspace,
factor all rational roots, and intersect the coordinate eigenspaces. A
one-dimensional joint eigenspace is an evaluation functional; recover every
coordinate and evaluate every original input equation. No projected random
Krylov sequence is used as a completeness certificate.

This is standard finite-algebra machinery, not a novelty claim. The utility
also handles a nonreduced quotient and extension-only points. Supplied GB
provenance in the original ideal is still a backend obligation. A corrupted
unit basis cannot be certified as an original-input refutation by this
extractor. Returned point relations and scalar answers are checked directly;
empty output means no verified relation, not independently certified UNSAT.
[pcc:unverified]

## Frozen software qualification

The [protocol](PROTOCOL.md) predates execution and records amendments before
their calls. Setup and solving are cold per target. No scientific wall-time,
unified operation cost or scaling comparison was measured.

| Control | Producer-local outcome |
| --- | --- |
| Dense F5 grid, 25 points; nonreduced, extension-only and unit quotients | exact rational sets; bad operators and caps rejected |
| Retained F17 m3,t4 system | all 28 assignments; every original equation; complete model set; generator coefficient maps identical |
| Fresh F17 m3 domains/targets | 90 cases, 400 models; exact complete sets agree with independent OpenSSL enumeration |
| Cold F17 recovery | all 18 nonzero targets; 72 accepted relations and 18 kernels replay with OpenSSL |
| m5,t3,c16,R=(0,6) | 2 reference/candidate models, quotient dimension 8 |
| m5,t4,c1,R=(0,6) | 88 reference models; backend cap, unknown |
| m7,t4,c1,R=(0,6) | 56 reference models; backend cap, unknown |
| Wrong scalar / duplicate control case | independent replay rejects |
| Witness-pruned encoding | same90 complete model sets /400 models and18 cold recoveries; OpenSSL agrees; same denser arity caps |

The two denser arity controls used unchanged **20-second backend caps** and
2 GiB address-space caps. Those are configured limits, not performance
measurements. A time/resource failure remains unknown. None of these controls
establishes a growing-family degree bound or useful-relation yield.
[pcc:unverified]

The separately compiled [OpenSSL replayer](replay.cpp) includes no candidate
source. It independently exhausts the finite model sets, recomputes every
mixed query, binds signed rows, checks the left-null equations and label
sums, and verifies scalar multiplication. This is an independent arithmetic
implementation within the producing session, not a signed external verdict.
[pcc:unverified]

## Reproduce

Linux x86_64 with g++, GMP/OpenSSL development headers, curl and dpkg-deb:

```sh
bash research/shared_norm_algebraic_20261115/bootstrap_backend.sh /tmp/norm-prefix
bash research/shared_norm_algebraic_20261115/build.sh /tmp/norm-prefix /tmp/norm-build
/tmp/norm-build/norm_algebraic --selftest
/tmp/norm-build/norm_algebraic --retained research/prime_one_shot_20261029/continuation/canonical/unbound-check/remainder.gb
```

A point-only cold solve on the fixed curve E/F17: y^2=x^3+2x+2, P=(5,1):

```sh
LD_LIBRARY_PATH=/tmp/norm-prefix/usr/lib/x86_64-linux-gnu \
  /tmp/norm-build/norm_algebraic --solve \
  /tmp/norm-prefix/usr/bin/msolve /tmp/new-norm-solve 6 3 1234
```

The output directory must be new; every backend input/output, attempt,
accepted relation and kernel is retained. The given coordinates are Q, not
an input scalar. The CLI is explicitly a fixed-fixture interface, not a
full-width arbitrary-curve attack.

Frozen complete-model and cold controls:

```sh
LD_LIBRARY_PATH=/tmp/norm-prefix/usr/lib/x86_64-linux-gnu \
  /tmp/norm-build/norm_algebraic --qualify-models \
  /tmp/norm-prefix/usr/bin/msolve /tmp/new-models /tmp/models.tsv
/tmp/norm-build/replay --models /tmp/new-models /tmp/models.tsv
LD_LIBRARY_PATH=/tmp/norm-prefix/usr/lib/x86_64-linux-gnu \
  /tmp/norm-build/norm_algebraic --qualify-cold \
  /tmp/norm-prefix/usr/bin/msolve /tmp/new-cold /tmp/cold.tsv
/tmp/norm-build/replay --cold /tmp/new-cold /tmp/cold.tsv
```

## Cost chain and remaining acceptance criteria

The complete cost to justify is

```text
setup + sum_over_all_attempts(target_generation + encoding + GB_construction
    + rational_extraction + relation_verification)
    + linear_algebra + recovery + final_scalar_verification.
```

The current code records calls, failures, relations and kernels, but does not
price every term in a common unit. The small quotient controls extraction
cost only after the initial basis is available. It does not control GB
construction. The existing sufficient [initial-elimination question](../prime_one_shot_20261029/continuation/theory/initial-elimination-obligation.md)
asks for a certificate-degree bound O(m+t) on an effectively bounded useful
fraction of natural positive targets, with all failed attempts charged.
With v=O(mt)=O(n), balanced m,t=O(sqrt(n)) and such a proved bound, the
Macaulay dimension would be exp(O(sqrt(n) log(n))). That remains a conditional
argument, not a demonstrated property of this implementation.
[pcc:unverified]

| Requested requirement | Status |
| --- | --- |
| A different route from tensor contraction | shared norm/remainder algebraic route implemented locally |
| Complete cold functional path | fixed order-19 fixture implemented and replayed |
| Growing arity | m5 positive control; denser m5/m7 unknown under fixed caps |
| General prime fields / arbitrary full-width curve | not implemented by this bounded backend/interface |
| Source-ideal provenance and faithful uniform solving bound | open |
| Effective useful-target yield with all costs | open |
| Original subexponential algorithm criterion | **not met** |

No leaderboard or performance chart is changed. The next mathematical task is
the growing-family initial-basis/cost-yield bound; the next empirical task is
a prospectively frozen larger study through the existing EKS/admission route.
Current runtime status reports no configured cloud identity/secret, and live
budget/pool/checkpoint receipts are unavailable. No paid run was launched.

See the [validation and custody record](VALIDATION.md),
[complete raw archives](evidence/), [pipeline diagram](pipeline.svg),
[PDF report](report.pdf) and [publication checkpoint](PUBLICATION.md).
These are deterministic development checks, not comparative/statistical
measurements. No performance export or retrospective protocol freeze is
presented. The canonical cross-repository protocol skill could not be read:
its connected API returned `transform: short source buffer` / `VLT1`.
The next scientific study must resolve that dependency and freeze the actual
schema/checker protocol before collecting affected measurements.

## Sources checked and claim scope

- Petit–Kosters–Messeng, PKC 2016, [primary paper](https://people.maths.ox.ac.uk/petit/files/16PKC_primeECDLP.pdf),
  sections 3.1–3.4: complete IC framework, low-degree map chains, and the
  explicitly open dedicated-solving complexity.
- Existing [one-shot continuation](../prime_one_shot_20261029/continuation/README.md),
  [solver/reference](../prime_one_shot_20261029/continuation/solver/README.md),
  and retained CAS receipt: existing equations, charts, coefficient-census
  baseline, label-blind recovery and the 28>17 extraction issue.
- PRs [#4091](https://github.com/isogeny-labs/crypto/pull/4091),
  [#4147](https://github.com/isogeny-labs/crypto/pull/4147), and
  [#4188](https://github.com/isogeny-labs/crypto/pull/4188): newer native
  PKM tooling/extraction and scoped norm-rank work. Source-reported outcomes
  are not promoted to independent results here.
- msolve v0.7.5 @ `1d0d10971467d83985b2f1197e5fe7e4ed4be24f`, input/output
  contract and [FLINT matrix/root APIs](https://flintlib.org/doc/nmod_mat.html).

KB unavailable: no MCP tools and no backend executable for the repository
wrapper. Exact primary/code fallback used; no novelty/exhaustiveness claim.
PCC-20261115-be28ec is a statement with no proof artifacts or signed verdict.
The two supporting shared Lean laws have a focused build and axiom audit;
they do not formalize the solver, projection endpoint or ECDLP cost theorem.
[pcc:unverified]

## Subsequent core variant

The [core graph-elimination study](../norm_core_elimination_20261115/README.md)
adds exact target/support and public-prefix substitutions, preserving this
study's frozen records and original modes. Its report gives the complete new
controls and the dense backend allocation/cap outcomes. The generic oracle
adapter changes the source execution identity; archived parent hashes bind
their recorded historical commits, not a later working tree. [pcc:unverified]
