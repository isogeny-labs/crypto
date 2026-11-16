# Lefschetz development controls, fixed before execution

2026-10-30. These are implementation correctness controls for the proposed
PRIME_LEFSCHETZ_CANDIDATE.md reduction, not n83 production certificates. No
new FFD/LFD/regularity or speed claim. Current source-bound intervals7–17
remain unchanged. Preserve all actual fixed bases and the full three-family
m2–9 objective. Primary references were re-read: Lundqvist–Nicklasson
https://arxiv.org/html/1604.06820 Theorem2.7/Section3, and Caminata–Gorla
https://arxiv.org/html/2112.05579 Definitions1.3–1.5.

## Fixed controls

1. Build explicit primitive kernels of the lowering operator D in each
   squarefree block ell1..9 at primes11 and101. Apply raising operator L to
   produce every graded string. Check the predicted string length, zero
   endpoint, primitive multiplicity and full basis rank in every degree.
   Retain all vectors, not just aggregate ranks. Negative characteristics:
   ell3,p2 and ell5,p3; the sufficient-characteristic gate must reject them
   and unrestricted attempted constructions must fail a basis/endpoint check.
2. Compare direct squarefree and decomposed-sector multiplication maps for
   m3,ell1/2/3 and m4,ell1/2; multiplier degrees0/1/2/3; primes11/101.
   This is40 fixed direct-versus-sector cases. Keep the full direct matrix,
   every sector matrix, multiplicity and rank. Compare both total domain
   dimension and kernel/rank dimension. Inputs and order are deterministic.
3. Compare all six cubic m3,ell1/2/3,p11/101 maps byte-semantically against
   the retained Rust reference fixtures from cubic development, including
   coefficients,columns,monomials and rank. A mismatched matrix fails even
   if its rank agrees. The Python direct builder expands six terms; the
   earlier Rust builder factors the quartic, providing independent paths.
4. Check proposed leading weights and disjoint leading supports for every
   m2..9, including terminal specialization. Equal/reversed intermediate
   weights must fail an appropriate strict-leading gate.
5. Record the sufficient no-truncation inequality ell-2d+6>=0 for ell27,
   d4..16; d17 must fail it. Verify the degree17 terminal boundary sector
   k_last13 has maximum L exponent1 and loses the L_last^2 term. Do not
   infer an original-system non-Koszul witness from this boundary test.

All finite-field arithmetic is exact. No stochastic samples, timing claims,
or statistical comparisons. Each attempt writes to a fresh output directory;
every completed fixture is written before its assertion. Failure is retained,
not retried in place. Source edits get a new attempt. Replay recomputes the
same records and checks them exactly. A full n83 certificate run requires
separate source checking, frozen protocol and independent review of the
mathematical argument, especially Koszul transport and FFD conventions.
