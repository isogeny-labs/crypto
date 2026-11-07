# Continuation integrity review

Reviewer: `/root/continuation_integrity`, 2026-10-30 UTC. Scope: source interfaces,
fixed correctness records, publication wording and reproducibility metadata under
`research/prime_one_shot_20261029/continuation/`. This is not an independent
asymptotic proof, external scientific replay, timing experiment or novelty search.

## Findings requiring correction before publication

1. `solver/README.md` currently states
   `O(p^2*(t+log p)+p*M)`. The literal `divrem` routine computes its leading
   coefficient inverse by Fermat exponentiation on every remainder step,
   including monic divisions. Use the safe bound
   `O(p^2*(t+1)*log p+p*M)`, already used in the parent README and solver review,
   then refresh the README entry in `solver/files.sha256`. This changes no
   recorded result or exponential classification.
2. `readiness-run.json` still preserves the queued observation plus successful
   cancellation request, not the final outcome. A fresh read-only GitHub query
   returned `status=completed`, `conclusion=cancelled`, job completion
   `2026-10-30T00:32:31Z`, and zero steps. Add that terminal observation without
   relabeling the probe as an executed solver or successful smoke test.

The review snapshot had no blocking source-correctness defect for the declared
fixed F17 contract. The two items above are documentation/metadata consistency
fixes. Later additions or edits are outside this snapshot unless reviewed again.

## Checks actually executed

- `sha256sum -c files.sha256` from `continuation/solver/`: all 26 entries pass.
- Strict compilation of the separately implemented
  `reviews/verify_records.cpp` with
  `g++ -std=c++17 -O2 -Wall -Wextra -Wpedantic -Werror`, followed by execution
  against `solver/checks/`, exited zero:

      PASS fixed model census: cases=90 models=400 SAT=42 UNSAT=48
      PASS mixed-target relations=72 kernels=18 low_rank=12 internal_DLP=18 CLI_DLP=18
      PASS retained counters: attempts=230 identity=13 PDP_misses=145 PDP_calls=217 pairs=59024 accepted=72

  This checker includes no candidate source. It recomputes the fixed rational
  point-triple census, verifies stored relations and kernels, checks both sets of
  returned scalars by group multiplication and reconciles counters. It does not
  rerun the coefficient solver or search for logarithms.
- Read the unknown solver, point-only direct interface, C-only left-null routine,
  full frozen protocol and scripts. `coeff::solve` receives only public curve and
  base parameters plus R. `direct::Solver` receives only Q and a development seed.
  Fixture expected scalars are outside that class; left-null selection has no
  scalar-label input. The bounded source loops and records support the stated
  p(p-1) coefficient enumeration and accounting partitions.
- The original pre-addendum protocol hash in `build-receipt.txt` is exactly
  reconstructible: remove the direct-IC addendum and the blank line separating
  it from the original text. The resulting SHA-256 is
  `0ac7a84b8e7ac222f30eef2dc046e4a616e4952a4c8f629608c200e728394ef2`.
  Current protocol manifests correctly bind the combined document. This checks
  textual identity, not independently authenticated chronological preregistration.
- Read-only run query:
  `gh run view 40979213857 --repo isogeny-labs/crypto --json status,conclusion,updatedAt,jobs`.
  No dispatch, infrastructure change or job cancellation was performed by this
  reviewer.

## Evidence and interpretation remain separate

The 90-case unknown solver really searches unknown coefficients from public
inputs and agrees with complete rational model sets. The direct solver really
returns all 18 nonzero fixed-curve scalar answers through its point-only API and
18 separate CLI calls. This is stronger evidence than evaluating supplied norm
witnesses, but all cases use one order-19 curve and the PDP is exponential.

The canonical backend checks have a different role: all variables are bound to
known assignments before native msolve is called. Their successful assignment
ideals establish importer consistency only. They are not unknown solving,
growing-arity performance evidence, or an external competitive comparison.
The README and backend documentation state this distinction explicitly.

Both scalar panels use deterministic development randomness. No natural
zero-denominator retry occurred; only the synthetic rejection control covers
that branch. The documentation retains that qualification. Cold setup and
unsuccessful attempts are counted, but the diagnostics are not a complete
common field-operation price and must not be used as a speed comparison.

Coverage/rationality/recovery derivations and their within-session reviews are
supporting mathematics. They do not supply the missing uniform cheap complete
decomposition oracle. General-prime coverage, growing-arity implementation,
scientific scaling evidence, an exhaustive corpus review, certified novelty and
sub-exponential ECDLP remain unestablished. The parent report currently preserves
these limits.

## Source checks informing this review

Read the repository's report-evidence skill and session handoff. Rechecked PKM
2016 primary-text section 3.1 / Algorithm 1: mixed-target sampling and linear
algebra are prior art, and the paper explicitly leaves a better structured
solver uninvestigated. Rechecked the official msolve README's input grammar and
single-occurrence-per-monomial requirement. Neither source licenses inferring
fast solving from low-degree encoding or supplied-witness integration tests.

No solver, raw correctness records or repository files were modified by this
reviewer. The independent checker binary is outside Git at
`/workspace/audit/continuation-final-record-check`.
