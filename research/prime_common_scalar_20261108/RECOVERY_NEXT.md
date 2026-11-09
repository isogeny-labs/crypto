# Next proof and exact replay: finite-precision Hermite multipliers

Continuation checkpoint, 2026-11-09. This is a prospective programme, not a
completed proof or an executed experiment. It does not reopen the frozen
normalizer panel or authorize a timed campaign.

## Current checkpoint: actual losses and finite-functional stability

The original sequence below is retained as the prospective record. The
quantitative goals of Steps1--4 now have source-connected proofs: actual ceiling/order bounds, actual basis and
finite reciprocal, actual multiplier loss6hD_i (by a grid pairing and finite
second moment), and actual evaluation loss4M(a+1). Step3's displayed weighted
identity was replaced by the proved pairing/moment argument for the same bound.
Step5's finite-functional
stability is also proved at the original source precision. The exact D.9
reconstruction identity from quotient jets remains open, followed by coordinate
and bit-cost refinements. See
[the precision report](reports/hermite-precision-20261109/REPORT.md).
[pcc:unverified]

The next code target is an explicit exact quotient truncation and interpolation
polynomial: show each sampled chart has the prescribed low jets, bound total
degree below the full source jet count, and apply exactHermiteUnique. Then
combine that identity with actualRecoveryFunctionalStable and the actual
inverse-buffer correspondence. No new comparative campaign has run.
[pcc:unverified]

## Original prospective sequence

The accepted source chain is actual affine clearing -> actual packed degree ->
actual grid counts and ceiling -> exact scaled-jet determination. See the two
versioned Nov9 reports and their unsigned statement pins. [pcc:unverified]

## Question and competing hypotheses

H1: for the actual cleared rational repair, the source's original taper q_t and
precision p_t suffice for origin recovery in the regime3<=u<=a, u<=M.
H0: clearing or multiplication by D^M adds a precision loss that invalidates
that budget. A failure must identify the actual lost valuation, coefficient
or interpolation step; compilation time or CPU contention cannot explain an
exact algebraic mismatch.

Evidence for following H1: the new exact source theorem uses the original
q_t with h=ceil(3M/u) even at doubled degree. Source D.10's interpolation
multipliers depend on that taper, not on the polynomial's degree once the
multiplicity exceeds it. This supports inspecting the original precision
rather than rerunning the old doubled-budget proxy. It is not yet a formal
finite-precision recovery theorem. [pcc:unverified]

## Actual definitions and proof sequence

1. Fix L=2^a, alpha=2L, actual nodes t_m=m-L, the existing sourceOrder and
   sourceCeiling. First prove the actual upper bound h*u<=4M. Keep zero
   excluded and all nonzero multiplicity nodes present. [pcc:unverified]
2. Define the actual rational polynomial

       E_t(T)=product_(s !=0,t) ((T-s)/(t-s))^q_s.

   Prove E_t(t)=1. Its local chart is the actual product of
   (1+alpha*z/(t-s))^q_s. Use the existing bounded integer-ratio theorem
   for each nonzero difference, not an assumed integral polynomial.
   Construct its integral inverse series with constant1 and prove the
   finite inverse buffer equals its low coefficients. [pcc:unverified]
3. Prove the translated-grid counts of v2(t-s), with t and zero exclusions.
   Derive the actual weighted valuation identity

       v2(E_t(0))=D_i*sum_(j=1)^(a-i) (j/2^j)*(q(i+j)-q(i)).

   The factor j is essential. Prove q(i+j)-q(i)>=-h*j and the series bound
   sum j^2/2^j<=6. Conclude loss at most6hD_i. [pcc:unverified]
4. For a local polynomial of degree below q_t<=4M, bound evaluation at
   -t/alpha by at most4M(a+1) denominator powers. This must be a statement
   about actual rational multipliers or coefficient lattices, not unit
   cancellation in a ring of characteristic2^p.
5. Construct the D.9 Hermite sum explicitly from low input jets and local
   inverse buffers. Prove its jets match the exact polynomial. Apply the
   existing exact uniqueness theorem; do not assume the reconstruction
   equality as a premise. Propagate arbitrary residue-lift errors and show
   p_t=p+8hD_i+8M(a+1) leaves at least p correct bits at zero.
6. Connect the actual cleared polynomial's integral charts coefficientwise
   to actualLocalContractionClearing. For rational integral tensor weights,
   its target at zero is unchanged since D(0)=1. Then extend the rational
   multiplier/error argument coordinatewise to the source's extension and
   nilpotent coefficient lattice, with explicit bases and precision maps.
7. Bound construction operations and rational numerator/denominator bit
   lengths before integrating recovery into the recursive evaluator.

No supplied theorem of the form "assume recovery correct" or "assume total
cost sub-exponential" discharges an obligation above.

## Prospective exact replay, after the precision proof

Before any execution, freeze a separate native protocol/source commit and
resource cap. Keep this a correctness check; timings need their own matched
isolation and external reference.

Use actual packed small matrices and sparse immutable A,B weights, so the
actual cleared polynomial can be constructed exactly without asserting a
fast evaluator for arbitrary tensors. Cover a>=3, 3<=u<=a, M>=u, all active
source levels, different injective placements, constant and nonconstant
contractions, and arbitrary allowed residue lifts. Derive the sampled finite
panel sizes from the cap before execution, with held-out sizes recorded.

Compare reconstruction against the exact target contraction at zero. Compare
original source p_t with the earlier doubled-budget proxy, but retain both
as distinct protocol arms. Include an intentionally insufficient jet budget,
a perturbed jet, and a scaling-cancellation error as negative controls. Verify
that each negative control fails for its stated mathematical reason. A single
local control is not a counterexample to the full all-node interpolation rule.
Use a second native implementation or independently derived recount against
all raw outputs; leave any absent independent standing explicit.

A passed panel would be bounded correctness evidence, not an asymptotic
runtime theorem. A failed panel must preserve the exact polynomial, nodes,
orders, precisions, jets, lifts and first mismatching reconstruction term.
