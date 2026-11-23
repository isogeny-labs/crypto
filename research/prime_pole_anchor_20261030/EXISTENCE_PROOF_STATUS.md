# Scope of the requested Lean existence proof

2026-11-06. Goal: prove or disprove existence of a classical sub-exponential
algorithm for general prime-field ECDLP, retaining the original novelty and
full-cost requirements. Status: **blocked by missing mathematical results**;
this is not a proof of nonexistence. Audit record `PCC-20261106-4731c1` is a
producer statement, with standing unverified. [pcc:unverified]

## What was inspected

The exact source declaration `AnchorBudget.fibonacci_bound` in
[AnchorBudget.lean](repeated/lean/AnchorBudget.lean) quantifies over natural g
and lists ms satisfying Feasible g ms. Its conclusion bounds list sum and
weighted penalty using Fibonacci numbers. The proof map explicitly excludes
the rational valuation bridge and global algorithm claims. That declaration
does not define an elliptic curve, solve a discrete logarithm, quantify over
algorithms, or establish an ECDLP runtime. No fresh build is claimed in this
audit; its source was inspected without modification. [pcc:unverified]

The pinned Bochum v0.2 manuscript, Theorem1, explicitly assumes Theorem2 and
states a conditional expected-time bound. Its source and digest are recorded
in [sources.md](../prime_common_kernel_20261030/sources.md). The inspected
conditional manuscript statement is not an unconditional Lean theorem or a
new result from this project. Its correctness and hypotheses require their
own full formal development. [pcc:unverified]

Sources checked (KB): research-kb MCP status/search/theorem tools were not
loaded; `scripts/research-kb status` failed127 because no executable was
available. The wrapper was materialized from tracked sparse checkout first.
Fallback: lead-atlas brief and match for "Lean existence subexponential ECDLP
generic lower bound anchor budget", exact source/proof map, existing common-
kernel problem record, and pinned manuscript Theorem1. The atlas is dated
2026-10-29 and reports newer unreviewed paths. This is a scoped audit, not an
exhaustive search for every theorem or proof in every repository.

## Positive proof: concrete missing obligations

An existence theorem meeting the request must expose an actual uniform
classical algorithm, its prime-field/subgroup input assumptions, correctness
and success convention, and a full bit-cost function. With field bit length
b, the claimed bound must be exp(o(b)), including preprocessing, failed
attempts, recursive entries, relation recovery and verification. It must not
hide the desired bound in an assumed oracle. The original novelty requirement
also needs an independent priority review; Lean theorem validity alone does
not establish novelty. [pcc:unverified]

For this project's proposed common-kernel route, the existing
[problem record](../../problems/correlated-packet-common-kernel.md) gives
checkable intermediate acceptance criteria: a common deformation, actual
weighted crossing identities, charged mode density, uniform child shrink,
precision/degree invariants, and complete entry/recovery costs. These remain
the concrete construction and proof obligations, rather than an established
global algorithm theorem. [pcc:unverified]

## Negative proof: a different, stronger missing result

An impossibility theorem would need a precise computational model and a
lower bound applying to ALL algorithms in the allowed class. A generic-group
bound does not cover the coordinate-dependent algorithms explicitly allowed
by the user. The anchor budget only constrains a stated family and its stated
inequalities; it cannot rule out arbitrary nonscalar completions, other index
calculus methods, or unrelated classical algorithms. [pcc:unverified]

Neither assuming the desired conclusion as a Lean hypothesis nor proving a
finite counterexample, an abstract budget, or a theorem with conclusion True
would discharge this objective. No placeholder existence theorem or custom
axiom was introduced. The next useful formalization needs the missing actual
construction and its invariants; merely writing an existence proposition
would not supply a proof.
