# Prime F4 chain increment: frozen implementation contract

2026-10-29, source base `0a5d604ea97fc96f5665edc8e774d1efdc70a611`, Conductor T-779.
This is a software correctness and artifact contract, not an experiment protocol,
performance comparison or completed-corpus claim. No cloud generation is launched.

## Sources and mathematical scope

Reviewed the existing `solvers.rs::prime_f4`, native `f4_fp::solve_visit`, and
[upstream inventory](UPSTREAM-SOLVER-EXTENSIONS.md). Semaev's 2004 primary PDF
at <https://eprint.iacr.org/2004/031.pdf> returned HTTP403 to this browser.
The author's accessible [2015 paper, sections2–4](https://arxiv.org/html/1504.01175)
provides the odd-characteristic S3 formula, chained construction, algebraic-closure
interpretation and conditional nature of complexity assumptions. Gaudry's
[2004 ECC slides](https://cacr.uwaterloo.ca/conferences/2004/ecc2004/gaudry.pdf)
provide a concrete decomposition/lifting/linear-algebra separation. Reuse these
established constructions; no novelty or first-fall-degree heuristic is adopted.

For y²=x³+Ax+B in characteristic at least5, use
S3(x,y,z)=(x-y)²z²-2((x+y)(xy+A)+2B)z+(xy-A)²-4B(x+y).
For m summands and fixed affine target abscissa r, the chain has m leaf
variables, m-2 auxiliary variables, and m-1 S3 equations. Start with
S3(x0,x1,u0), continue S3(u[i-1],x[i+1],u[i]), and finish
S3(u[m-3],x[m-1],r). At m=2 use the existing reference adapter unchanged.
At m=15 there are28 variables; unpacked exponent vectors are legitimate.

Leaf coordinates use the exact signed factor-base abscissae and membership
polynomials. Auxiliary coordinates range over all Fp; no leaf-domain restriction
may be copied onto them. Variable roles, full equations, target, curve, ring,
order, domains and encoding strategy must be retained before native solving.

Affine partial sums can be infinity. This implementation does not cover every
exceptional chain branch or prove an equivalence with arbitrary PDP decompositions.
A found root is only a candidate: independently lift leaves to actual signed
factor-base points and verify their full group sum. Exhausting the supplied
encoding permits only an encoding-relative completion/no-root observation;
`exhaustive_no_decomposition` stays false. Never read the private known-answer
witness. A verified PDP witness is not recovered ECDLP or a fall-degree certificate.

## Native extension and artifacts

Add an opt-in traced visitor beside existing `solve_visit`; preserve untraced
behavior and avoid global observer mutation. Emit recursive-system input,
variable/assignment bindings, native F4 output with useful-row replay trace,
branch choice and candidate validation events. Callback failure is sticky and
terminates the traversal; it must never set complete=true or masquerade as a
negative mathematical result. Expose the trace termination separately from
ordinary positive visitor stop and deadline expiration. Invalid input remains
an error. Do not replace native F4 with enumeration under the same method label.

The dataset adapter stores and durably streams a bounded JSONL event prefix,
with explicit event/byte/work limits, retained termination metadata and separate
root/lift observations. Persist input before elimination. Each recursive native
call's useful-row trace is retained without an extra duplicate top-level run.
Useful-row traces still omit zero rows/private pivots and internal factorization
states; declare those gaps. No claim of complete machine-instruction accounting,
whole-call checkpoint recovery, or all internal matrices follows from this increment.

Preserve the existing m=2 artifact and strategy behavior. Label m=3..15 as a
new affine-S3-chain strategy. Keep p<2^32 as the native elimination boundary.
At larger requested primes, including2^61-1, retain exact equations computed
with wide intermediate arithmetic and explicit unsupported-backend outcomes;
never invoke the narrow reducer or silently substitute a smaller field.

Bound native time, degree and pair batches using existing adapter controls.
Track recursive F4 counts/field-operation observations, domain/root work, and
group-lift additions with distinct units and scopes. Trace exhaustion, lift
exhaustion, timeout and encoding-relative completion stay distinguishable.
No library stage counter becomes a retired-instruction measurement.

## Required correctness validation

- Traced/untraced solution-set parity on small independent polynomial systems,
  exact domain restriction, unrestricted auxiliary roots and degree-truncated
  candidate checking. Verify sink failure before work and during traversal
  prevents later callbacks and a complete result.
- Independently evaluate generated S3 equations against curve-group additions,
  including both twist positions and m2/3/4/15; verify actual variable-role/domain
  maps and membership equations. Preserve infinity exceptions as explicit scope.
- Successful m3/4 known-public fixtures with private witness cleared, independently
  replayed through the existing artifact verifier; tampered witnesses rejected.
- Bounded/no-solution/trace exhaustion outcomes must not claim PDP impossibility.
- Large-prime m3/15 construction retains equations without entering native F4.
- Centralized focused tests, strict applicable lint and required full-library
  validation before PR delivery; no speed comparison or scientific wave here.

ML use: public structural inputs can describe the equation/variable/domain graph.
Recursive traces and outcomes remain restricted targets/provenance unless a
separate prefix projection establishes what was available before each decision.
Potential targets are branch choice, useful reduction selection and lift success;
this increment neither trains a model nor establishes predictive efficacy.
