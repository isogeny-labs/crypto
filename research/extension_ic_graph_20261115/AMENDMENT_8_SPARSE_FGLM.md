# Amendment 8: checked sparse quotient-graph extraction

Frozen before component observations, 2026-11-16. Producer protocol;
[pcc:unverified]. User requires Rust and authorizes local or Runpod.

The full F4 stages09/10 identify dense characteristic-polynomial/eigenvector
arithmetic as the largest counted extraction cost. F4Krylov keeps exactly the
same native basis/staircase/border construction and replaces this extraction
with sparse weighted transpose walks on the quotient graph. F4Groebner stays
the dense baseline, and all historical defaults stay as before.

Primary literature: Faugere–Mou, *Sparse FGLM algorithms*, arXiv1304.1238v1,
section3.1, Theorem3.1 and Algorithm2, https://arxiv.org/pdf/1304.1238.
Primary PDF SHA2565074b642b97f027057eafb88b87141eb256eb4b1e717e2291486c37b13eb616b.
Its complexity discussion excludes construction of the multiplication matrix;
this study includes basis, border and matrix construction in the compared phase.
KB is unavailable (MCP absent; wrapper reported no executable); existing native
BM/matrix/normal-form code and primary text reviewed, no novelty assertion.

The fast path requires standard constant/coordinate monomials, BM degreeD,
nonzero constant coefficient, exact vector recurrence, a checked polynomial
inverse and all D Hankel equations for both remaining coordinates. At most
three projections (first coordinate functional, then two deterministic seeded
random projections). Original restricted equations and the symmetric cubic
are checked. Failed sparse work remains counted and falls back to dense
extraction; unsupported bases retain the old charged oracle fallback. No
unaccounted DLP/rho fallback. Projection RNG is separate from root RNG.
Generic modular sparse edges and existing root factorization are used; no
extra field/root kernel is bundled into this comparison.

First comparison is the **complete coordinate engine**, dense versus sparse:
both return identical canonical triples, including empty sets. Shared symbolic
preparation, generated public inputs and dense qualification are outside this
phase and recorded separately. This is not full IC recovery. Each arm charges
all native basis/border/matrix/extraction work and failed sparse attempts.
Fields67/127/1039; held-out seeds83/89/97; four residuals each; 36systems;
8calls/batch; dense_a/krylov_a/dense_b/krylov_b, reversed on odd rounds;
one retained warmup and five measured rounds. All864batches retained, including
invalid, capped or incomplete. Per-execution starts are flushed before timing.
Native degree24, no per-call deadline. Local CPU0/8GiB/900s, zero retries.
Source/checkpoint/binary/plan/host captured before the first call.

Acceptance: equal complete output to dense qualifier in every call, intra-batch
and A/A output signatures agree, both median A/A ratios within reciprocal1.10
and1.10, full cohort; deterministic10000paired-round bootstrap. Improvement
requires an eligible upper interval below1. If the sparse phase is viable on
eligible largest-field systems, freeze and run full IC comparisons including
the current prepared-pair baseline and same-instance rho; do not extrapolate
component cost into a pipeline/parity claim. Otherwise use observed failure
mechanism before selecting the next lead. Strongest-external comparison and
independent signed replay/recompute remain required for admitted rankings.

Planned create-new run:
/workspace/ic-local-runs/local-s4-krylov-component-20261116-11.
No experiment has run on this source at preregistration time. Canonical exported
schema is unavailable (encrypted VLT1 sources); the explicit local receipt does
not assert canonical validation or standard EKS/L2 admission. [pcc:unverified]
