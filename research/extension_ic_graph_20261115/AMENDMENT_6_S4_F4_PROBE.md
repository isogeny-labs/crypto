# Amendment 6: native F4 basis-phase feasibility probe

Freeze before data. Prepared S3 and longer epochs did not reach local rho
parity in runs06/07. In17of18 target/sampler cases, recovery coincides with
first repeated decomposable x; this is consistent with generic trajectory
coalescence. The distinct next lead is a cheaper three-point relation finder,
using existing native F4 rather than dense Macaulay elimination.
[pcc:unverified]

## Component boundary

Compare the native F4 basis phase with the existing complete S4 coordinate
solve. This is deliberately a lower-bound feasibility check: F4 does not yet
produce decompositions and still needs quotient/action matrices, eigen/root
recovery and group signs. A smaller basis time cannot be a pipeline or rho
win. Reject a competitive whole-solver interpretation if the basis alone is
already slower than the complete reference. The larger research objective
and the existing graph collector stay in scope.

Both phases include Weil restriction and input conversion. Shared symbolic
S4 curve preparation, public-input point generation and correctness checks
are outside the phase and reported separately; no cold IC interval is
claimed. Each timed arm performs8 successive solves on one fixed input;
report the batch interval and per-call descriptive values. No call or failed
call is dropped. This is an engine-phase experiment, not a fresh-target IC
solve. F4's returned basis is materialized during timing; its validation and
staircase enumeration occur afterward.

## Inputs and controls

- p67/127/1039; fresh fixture seeds59/61/71;4public residuals per curve,
  deterministic aG+bQ with a separate declared seed domain.36systems.
- Arms macaulay_a/f4_a/macaulay_b/f4_b, reversed on odd rounds.
- Retained warmup1 and measured5,8calls per arm:864execution batches,
  144warmup/720measured. All repeats and failures remain raw.
- Explicit reference solve mode: border_retry=false, lazy=Off. F4 grevlex,
  max_degree24, native deadline50ms per call, no early staircase/Hilbert stop.
- A/A output signatures and all-call consistency required; local median A/A
  tolerance10%; paired log geometric means and10000round-bootstrap draws
  as in the prior local analysis. Any invalid/capped/incomplete case remains
  diagnostic only. No automatic retries or substituted seeds.
- F4 feasibility requires untruncated, non-timeout native completion, finite
  staircase and preservation of every baseline rational solution in the
  returned basis. This is a necessary check, not independent certification
  of the new method's exhaustive root recovery.
- Whole process900s,8GiBvirtual, pinnedCPU0, compiler idle. Create-new local
  run directory local-s4-f4-probe-20261115-08; copied immutable binaries,
  source/runtime/input hashes, raw JSONL, exit/timing/provenance retained.

## Decision

Report every system's ratio, uncertainty, A/A and native completion. A ratio
upper interval below0.5 is a reason to investigate extraction, provided its
controls pass. It is not a formal improvement tier or multiplicity-corrected
claim. Slower, timed-out, truncated and inconclusive systems remain visible.
Do not promote an incomplete algebraic phase to ECDLP recovery or parity.

Sources checked: gaudry_cubic::SymmetrisedS4/solve_s4_subspace_with/
read_solutions, native f4_fp documentation/options/report/reduce/solve_visit,
and residual-walk note11.4–11.6. KB MCP unavailable; wrapper query for
F4/Gaudry/S4/action-matrix returns no executable. Primary Faugere F4 author
PDF retrieval at https://www-polsys.lip6.fr/~jcf/Papers/F99a.pdf returned503;
that primary-source review remains an evidence gap. Reuse the established
repository algorithm without a novelty claim.
