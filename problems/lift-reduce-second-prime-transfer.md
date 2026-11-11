# Lift to characteristic zero, reduce at a second prime p′: untracked transfer route

- **Status:** `open` (proposed lead; partly reduced to existing leads by derivation; the higher-dimensional target is not examined)
- **Kind:** proof gap (the degree of the field that defines a global N-torsion lift is the gating quantity, and no cost bound for it is recorded here); unexamined route
- **Affected work:** any ECDLP transfer that lifts a prime-field instance to an abelian variety over a number field K and reduces it at a different prime. Related records: `LEAD-TRF-global-lift-small-height`, `LEAD-ICP-char0-lifting-xedni-smoothness` (both active, see [`research/ecdlp_subexp_desk_20261101/README.md`](../research/ecdlp_subexp_desk_20261101/README.md) header); `LEAD-TRF-cm-canonical-lift-heights` and `LEAD-TRF-padic-canonical-section` (closed within scope, summarised in [`papers/ecdlp-index-calculus-treatise/sections/transfer_ext.tex`](../papers/ecdlp-index-calculus-treatise/sections/transfer_ext.tex) §§ `sec:tp-padic`, `sec:tp-global`); [`genus3-image-destination-calculus-and-common-cost-missing.md`](genus3-image-destination-calculus-and-common-cost-missing.md) (a higher-genus target on the binary side)
- **Next action owner:** unassigned (transfer lane)
- **Last updated:** 2026-11-11

## Problem

We cannot say whether the route E/𝔽_p → A/K (characteristic 0) → A mod 𝔭′
over 𝔽_{p′^f} can weaken a prime-field ECDLP, because the lab has no lead or
record for it. Two of its three cases reduce to existing leads by the
derivation below. The third, a target that is a higher-dimensional abelian
variety (for example a Jacobian of genus g) over a small field, is not
examined. Its gate is the degree of the number field over which an
N-torsion point can be lifted, and nothing in this repository bounds that
cost.

Instance: E/𝔽_p ordinary, p > 3, P ∈ E(𝔽_p) of prime order N ≠ p,
Q = nP. Route: choose a number field K, an abelian variety A/K of dimension
g with good reduction at primes 𝔭 | p and 𝔭′ | p′ (p′ ≠ p, 𝔭′ ∤ N), and an
isogeny or embedding that puts E inside A mod 𝔭 up to isogeny; lift (P, Q) to
(P̃, Q̃) ∈ A(K); reduce at 𝔭′ and solve the DLP there. Acceptance criterion:
the CLAUDE.md success criterion, a measured gain over the current
prime-field index-calculus baseline with every phase charged, the lift and
the field of definition included.

## Evidence and uncertainty

- **Observed:** none. No run, no instrument. Text search of `crypto` @
  `c7c4c2874f` and `cryptanalysis` @ `68594c88` for lift/reduction terms
  found no lead, problem or study for reduction at a second prime. The lead
  atlas (`crypto-autoresearcher` @ `43a90a1e`) could not be read: its lead
  files are encrypted in that checkout. Absence there is therefore not
  established.
- **Derived (this record, not reviewed, not in Lean) [pcc:unverified]:**
  1. *Torsion lifts preserve the problem size.* For 𝔭′ ∤ N, reduction
     A(K)[N] → A(𝔽_{𝔭′}) is injective (the kernel of reduction is the formal
     group, a pro-p′ group). If P̃ has order N, the target instance has order
     N and N divides #A(𝔽_{q′}), q′ = p′^f. Hasse–Weil gives
     #A(𝔽_{q′}) ≤ (√q′ + 1)^{2g}, so q′ ≥ (N^{1/(2g)} − 1)². For g = 1,
     q′ ≳ N: an elliptic target is never over a smaller field, and its
     generic DLP has the same order N.
  2. *Non-torsion lifts make the second prime redundant.* If P̃ has infinite
     order, put D = Q̃ − nP̃ ∈ A(K). Reduction at 𝔭 gives D mod 𝔭 = 0 (up to
     the kernel of the isogeny), but nothing constrains D mod 𝔭′. So Q′ = nP′
     holds at 𝔭′ only when D mod 𝔭′ = 0, and for D ≠ 0 that happens at
     finitely many 𝔭′ that cannot be found without knowing n. If D = 0, then
     n is already determined in A(K), which is exactly the xedni condition.
     This case adds nothing beyond `LEAD-TRF-global-lift-small-height` and
     `LEAD-ICP-char0-lifting-xedni-smoothness`.
  3. *Target-independent output sets.* A route that fixes its candidate
     outputs before seeing Q falls under the counting bound of the corrected
     CM closure (`CORR-20260810-930c42`, Pr ≤ p^{−1+o(1)}) within that
     closure's stated scope (fixed arity 2–9, bounded coefficients)
     [pcc:unverified]. Outside that scope it is not covered.
- **Suspected explanation (conjecture):** the remaining case, g ≥ 2 with q′
  as small as N^{1/g}, is where index calculus on high-genus Jacobians
  (Gaudry; Enge–Gaudry) would apply. The gate is that P̃ must be an
  N-torsion point of A defined over K, and the Galois orbit of such a point,
  hence [K : ℚ], is expected to grow polynomially in N. For CM elliptic
  curves bounds of the form [K : ℚ] ≫ N / log log N are in the literature
  (Silverman; Prasad; Bourdon–Clark). They were not re-read for this record
  and no bound for g ≥ 2 was checked. If [K : ℚ] ≳ N^c for c > 0, writing
  down K or P̃ already costs exp(Ω(log N)) times a polynomial in N, so the
  route would be dominated by rho. This is the same obstruction the
  `cryptanalysis` xedni audit records for binary fields: residue degree m
  forces [K : ℚ] ≥ m.
- **Missing capability:** a sourced lower bound on the degree of the field
  of definition of an N-torsion point on an abelian variety of dimension g
  whose reduction contains E up to isogeny, and a construction of A for
  g ≥ 2 at all.
- **What this evidence does not establish:** no lower bound on the ECDLP and
  no closure. Derivations 1–3 are unreviewed. The atlas search is
  incomplete, so this may duplicate an existing lead.

## What would unblock it

- [ ] Read the atlas (or the research-kb catalog) for this route, and either
  link the existing lead here and mark this record `superseded`, or file
  the lead in `crypto-autoresearcher` with this record as its source.
- [ ] Independent review (red-team) of derivations 1–3, with the scope of
  each written as a statement a Lean proof could target.
- [ ] For g ≥ 2: a sourced lower bound (or a new proof of one) on
  [K(P̃) : ℚ] as a function of N and g, or an explicit family where it
  stays below N^{o(1)}, checked on a toy instance (N ≤ 2^20) by exact computation of the Galois orbit.

A bound with [K : ℚ] ≥ N^c, c > 0, closes the g ≥ 2 case as dominated
within its hypotheses. A family below N^{o(1)} would not by itself clear the
success criterion: the lift, the field arithmetic and the target index calculus
would still have to be charged and measured against matched rho.

## Next discriminating step

- **Action or proof obligation:** the atlas or KB lookup above, then the
  literature bound on the field of definition of N-torsion for CM and
  non-CM abelian varieties of dimension g.
- **Inputs and controls:** none until a construction exists. A toy check
  would compute the Galois orbit of an N-torsion point on a known CM
  elliptic curve over ℚ for small N, as a control for the g = 1 bound.
- **Execution limits and stop condition:** desk work; no run is planned.
- **Outcomes and next actions:** a polynomial lower bound closes the case
  within its scope; a sub-polynomial family opens a preregistered toy
  measurement; a duplicate lead supersedes this record.
- **Recheck trigger:** research-kb or atlas access; a new construction of
  higher-dimensional lifts.

A planned step is not a completed experiment.

## Resolution and history

- 2026-11-11: opened from a discussion of the characteristic-zero bridge
  (Tate modules cross between fibres because reduction is injective on
  prime-to-characteristic torsion, which is also why the DLP crosses at the
  same size). Unresolved.
