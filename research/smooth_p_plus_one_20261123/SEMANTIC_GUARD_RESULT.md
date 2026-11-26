# Canonical forms and complete-equation acceptance

2026-11-25 · `semantic_guard_v1` · Software qualification · PR #5142

Both fixed-form callers now omit zero terms and check a positive returned
parametrization against **every input equation and the exact fixed form**
before signed extraction. The shared guard performs polynomial substitution
modulo the eliminant, with no field scan. It records its receipt/error in the
attempt and uses the original absolute query deadline. Raw RUR retention,
capture caps and existing degree-separation retries remain in force.
Semantic failure stops the attempt without an additional retry.

The feature suite passes **60 tests**, the overlapping default suite passes
**32**, and the release build succeeds. Eleven guard tests plus caller-level
regressions cover the diagnosed defects and resource/error handling. These
are producer-local checks pinned by **PCC-20261125-475c5c**; no corrected-policy
scientific msolve panel has run. [pcc:unverified]

| Saved-output regression | Expected and observed behavior |
|:--|:--|
| Older RUR, effective form from parser diagnosis | Passes all nine equations |
| Same RUR, intended canonical form 0 | Rejects equation 9, the fixed form |
| Newer RUR, effective form | Rejects original equation 1 |
| Both caller stubs return rejected output | Raw output retained; one attempt; incomplete |

![Acceptance path implemented in both fixed-form callers. The existing degree-mismatch retry policy is preserved separately; semantic failures add no retry. No empirical timing is represented.](semantic-guard-flow.pdf){width=94% height=3.8in}

The change corrects the [explicit-zero serialization incompatibility](MSOLVE_INPUT_AUDIT_RESULT.md)
and prevents the saved invalid positive outputs from reaching accepted
signed extraction under these regression controls. The newer output's
underlying solver failure remains unexplained. All old scientific outputs,
missing queries and their historical policies remain unchanged.

\newpage

## Checks, accounting and scope

The guard validates canonical tokens, characteristic, distinct names,
polynomial metadata, scaling and numerator count. It keeps the original
eliminant-degree/quotient-dimension equality check, then requires monic,
nonconstant, square-free Q and gcd(D,Q)=1. Coordinates use the documented
negative-numerator/D convention, with the final variable equal to T.
For each equation of degree d, it checks the cleared-denominator substitution
modulo Q, including every cofactor coordinate and the fixed form.

This is standard residue validation of the provided positive parametrization.
It does not establish the solver's claimed quotient dimension, omitted
components or general completeness. The prior `[-1]` empty-result contract
is preserved and explicitly reported as `solver_empty_unchecked`, with zero
equations checked. It is not an independent UNSAT certificate. Existing
whole-answer comparisons and signed group replay remain separate checks.

Fixed per-check caps are 262144 input/output bytes, 64 variables, 2048 eliminant
degree, 20000 input terms, 16 total monomial degree and 8000000 scalar budget
units. Arithmetic and parsing check the caller's deadline; cap/deadline
failures remain incomplete. The receipt's `scalar_operations` counts budget
units, not measured CPU instructions. Canonicalization and validation are
inside the existing query deadline; no checking overhead is silently removed
from future query accounting. No runtime comparison is reported here.

Qualification also covers all 16 forms on both fixture primes 127 and 8191,
nonconstant unit denominators, bad auxiliary coordinates/forms, nonunit
denominators, repeated roots, invalid names/degrees/scaling, unknown,
duplicate or explicit-zero monomials, and exhausted budgets. Caller stubs
inspect the actual zero-free input file and retain rejected output. The two
suites overlap; their counts are not 92 distinct tests. No test failed or was
repeated. Shared dependencies' existing warnings are retained in the logs.

```text
source: 55f8d14f3b5fb1c997740ce8952360c2a31535e6
```

The [build identity](evidence/semantic_guard_v1/build_identity.json) binds the
executable and 20 source/input files to this source commit. The
[reproduction guide](SEMANTIC_GUARD_REPRODUCE.md),
[frozen protocol](SEMANTIC_GUARD.md) and command log describe the exact scope.
The directly attested full_cutoff.rs and production.rs remain byte-equal;
the [layout note](native/SOURCE_LAYOUT.md) identifies the compiled current
modules. Refined/single-system methods do not implicitly acquire this policy.
The pinned msolve/Sage contracts and unavailable KB queries are recorded in
the protocol. No new algebraic identity theorem or checked Lean proof is
asserted by this software component.

The next step is a separately frozen corrected-policy output panel under
normal checkpoint/resource gates, followed by complete-cost recovery
accounting. Existing study graphs, scoreboard and progress timeline remain
historical; this implementation qualification adds no timing datum. Prior
root-library compilation limits are not relabeled as a repository-wide pass.

PR #5142 stays draft while required artifact upload/byte replay for eleven
older raw files remains unfinished. The latest inspected migration,
45535484994/job138940927039, logged eight HeadObject HTTP 403 failures before
upload/replay; no logged outcome establishes the three liftable files' state.
No specific access-policy cause is inferred. Canonical experiment export
access remains unavailable; the local development-flow record is scoped.
The stream wrapper now selects the qualified executable for future explicit
captures. No experiment or sender was started: 713 records remain local,
with no configured Cloudflare receiver.
