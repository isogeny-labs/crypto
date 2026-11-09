# Final tensor-publication consistency review

2026-10-30 UTC. Reviewer `/root/continuation_integrity`. Read the current
initial-study notice and continuation README; tensor-review README, frozen
source triage, full cost and precision reviews; lemma-unit README, summary,
review correction and final logs; and integration notes. No new scientific
computation or theorem/implementation audit was performed in this pass.

## Verdict

No blocking publication inconsistency or novelty/certification overclaim was
found in the reviewed prose. The report now makes the necessary prior-art
correction prominent and preserves the original objective as unmet.

The October2026 public manuscripts are identified as an **existing explicit
claimed classical sub-exponential method**, with exponent
2^{O(b/(log b)^{1/4})}. The external contraction result is correctly described
as a theorem accompanied by a proof in its source, rather than a conjecture.
The ECDLP reduction's reliance on that theorem is retained. Namespace and
author-label provenance are expressly separated from institutional endorsement,
peer review and correctness certification.

The favorable review wording is supported by its stated scope: both reviewers
say they read D.1 and all of D.4, independently checked their respective cost
and local algebra/precision steps, and found no concrete gap there. Neither
claims to certify the entire manuscripts, all applications or the native
implementation. The parent ECDLP-encoding/reduction reading is separately
described. “No concrete gap found” is not promoted to a formal proof certificate
or novel authorship.

Finite-size fallback and missing performance measurements are correctly kept
as concrete implementation/measurement limits. They are explicitly **not**
used to refute the asymptotic proof. The report neither dismisses an existing
explicit proof as merely conjectural nor declares a usable deployed-curve attack.

Transcribing, renaming or reviewing that existing method is expressly said
not to meet the user's original-algorithm novelty requirement. The original
norm/divisor procedure still lacks a proved sufficient sub-exponential
cost/yield bound. Its current p(p-1) census, conditional supporting reductions,
small-final-degree result and unresolved initial-elimination obligation remain
distinct from the tensor prior art.

## Development-evidence boundaries

The GMP headline counts agree with the retained final summary and logs:
72 polynomial cases,308 kernel charts,8,832 node errors,1,152 final errors,
four expected negative rejections and byte-identical replay. This pass checked
document/record consistency; it did not independently replay their arithmetic.
The documentation preserves the first negative-control coverage mismatch,
its actual-injection correction, both source/record snapshots and the
scalar-coordinate scope. It does not call the unit an extension-ring
multiplication test, contraction evaluator or theorem certificate.

The continuation reports the existing Rust crate's34 passing release tests as
a separate bounded correctness suite, with no asymptotic or challenge-solve
inference. This count was reported by the dedicated `rust_validation` task;
its final receipt was still being integrated when this review ran. I did not
independently rerun or certify that suite. Final packaging should bind its
exact source, toolchain, command and terminal test outcome, as the parent is
already doing. A root-library or general-recursion pass must not be inferred
from the scoped crate suite.

The initial-study README now has an explicit dated snapshot notice linking
the new prior-art assessment and admitting the initial audit missed these
papers. That notice prevents the historical research selection from being
presented as a complete current-literature assessment. The older frozen triage
likewise remains a dated limited reading, rather than being rewritten to
claim it had already performed the later decisive reviews.

## Optional wording refinements

- The continuation's first paragraph says “the current decomposition code
  remains exponential.” Now that tensor prior art leads the report, “the
  current norm-coefficient PDP remains exponential” is more precise. The
  subsequent section already makes that intended scope clear.
- “Two independent readings” can say “two independent within-session
  readings” to make the provenance immediately clear without suggesting
  external institutional review. The underlying review documents already
  disclose the actual reviewers and scope.

These refinements are clarity improvements, not blockers or mathematical
corrections. No source, raw record or repository document was modified by this
reviewer. Storage wrappers and the final Rust receipt remain the parent's
separate integration work.
