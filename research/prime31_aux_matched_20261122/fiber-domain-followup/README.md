# Prepared and conditional isogeny-fiber extraction

This continuation implements a concrete reduction in repeated fixed-input work
and a bounded extractor for exact isogeny fibers on the retained ordinary
Prime31 target. The complete cold comparison remains an open execution task.

## Read the tutorial

[Prime31_Fiber_Domains_Tutorial.pdf](reports/Prime31_Fiber_Domains_Tutorial.pdf)
has six pages of exact diagrams and a ten-step checklist. It explains the
prescribed field, separate target and auxiliary curves, the kernel/coset fiber,
ten quadratic versus five quartic maps, conditional branching, target recovery
from one augmented matrix, and the rationality conditions for a twist.

The earlier foundational tutorial and comparison remain unchanged in their
original study directories. The new supplement's source and supplied evidence
metadata are under `reports/`.

## Implemented behavior

- Prepare the fixed field, full auxiliary fiber, target base and tower once per
  cold cell. Each query still validates its own target and resource limits.
- Restrict leaf search to target-liftable coordinates while retaining full-V
  membership equations.
- Capture already-validated block endpoints in an opaque private token. Build
  the exact inverse-fiber index from those endpoints, preserving structural
  checks and accounting for allocation instead of repeating map arithmetic.
- Propagate singleton domains and branch only on informative univariates.
  Evaluate small conditional domains directly; otherwise use the pinned native
  root finder. Retain original equations and F4 consequences, exact signed
  replay, and Unknown when a bound prevents closure.
- Use one augmented recovery matrix in the new v3 path. Legacy v1/v2 behavior
  remains available as a control.
- Include pinned complete `prime_fast.ic_solve_fast` and specialized native
  batched-MITM controls, with their own setup and replay interfaces.

`COMPONENT_COST.md` derives the removed traversal's 624,640 multiplications for
the retained fixture and the 49,152-byte logical temporary path payload. These
are source-derived component quantities, not a wall-time ratio or complete-solve
cost. `BASELINE_FRONTIER.md` records the additional applicable word-arithmetic
and batch-inversion control candidate and its unresolved source conflict.
[pcc:unverified]

## Evidence and limits

`DEVELOPMENT_RECEIPT_v2.json` and `evidence/qualification-review-v2.json` record
104 passing fixed tests across 24 immutable executables. All 28 executable
hashes, all 104 compiled source files and every named test were independently
recounted against source559302c7df0cb411ce345b98b2650f33ee6e137d.
Claim PCC-20261123-3b68e0 remains unsigned. Candidate 1's 103-test evidence and
claim PCC-20261123-dfa99a remain bound to their original source. [pcc:unverified]

The two deterministic native archives preserve raw build/test records without
executable bodies. Extract with `tar -xzf evidence/native-qualification-attempt2.tar.gz`
into a new directory. The root-library attempt metadata archive is separate;
its large source-tree inventory stays loose because `root-library-gate.json`
names it directly by path and hash. See `QUALIFICATION.md` for exact scope.

All 138 prospective comparison inputs remain unselected and unrun. Fixed-test
durations are diagnostics. No comparative timing, complete Prime31 recovery,
signed PCC promotion, root-library pass or leaderboard update is claimed.
Canonical performance graphs are unchanged because there is no admitted new
measurement. [pcc:unverified]

## Next execution dependency

`IMAGE_ROUTE.md` identifies the existing supported route and required actual
checkpoint, protocol, budget/capacity, image and process evidence. The metadata
checker and source-payload constructor do not admit or launch a job.
`RUNTIME_BUNDLE.md` distinguishes its seven tiny metadata contracts from the
still-unqualified full Prime31 packaging and OCI runtime.

The next discriminating step is the protocol-frozen bounded screening pilot,
followed only by selected complete-recovery cells and matched controls when the
existing admission requirements are met. Preserve the original target, complete
cold boundary, one computational thread, 4096 MiB, 120-second outer cap, zero
retries and inherited USD5 aggregate ceiling. The shared access obstruction is
[recorded here](../../../problems/shared-s3-prime31-execution-gates.md).

## Source review

The primary paper is Petit, Kosters and Messeng,
[PKC2016 §§3.4-3.5](https://people.maths.ox.ac.uk/petit/files/16PKC_primeECDLP.pdf).
Large-prime-degree evaluation caveats are from Bernstein, De Feo, Leroux and
Smith, [Faster computation of isogenies of large prime degree](https://arxiv.org/pdf/2003.10118),
§§4.8-4.15. The repository's existing finite-domain root work (PR #4826) and
pinned native controls informed this continuation. Shared research-KB tools and
wrapper were unavailable; no novelty or exhaustive literature claim follows.
