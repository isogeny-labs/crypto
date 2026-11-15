# JKSST xedni analysis: skeptical Lean audit

Status: **kernel-checked scoped audit; conditional theorem repaired, several
overbroad inferences refuted, algorithm-specific bridges still open**.

This study audits Jacobson--Koblitz--Silverman--Stein--Teske (JKSST),
*Analysis of the Xedni Calculus Attack*, Designs, Codes and Cryptography 20
(2000), 41--64. It does not replace their conditional result with the opposite
unconditional claim. It separates:

1. statements proved in Lean from explicit hypotheses;
2. literal or overbroad statements disproved by Lean counterexamples;
3. conjectures and algorithm-specific premises that remain open; and
4. interfaces whose two endpoints exist but have not been composed.

The [proof map](PROOF_MAP.md) binds the readable claims to Lean declarations.
The [claim-chain diagram](claim-chain.svg) and [rendered report](report.pdf)
show where the paper's conclusion is proved, conditional, or disconnected.

## Primary-source review

The following sources were retrieved and read before choosing the audit and
follow-on priorities:

- JKSST, [complete author copy](https://pages.cpsc.ucalgary.ca/~jacobs/PDF/xedni.pdf),
  especially Section 4, Lemma 4.1, Theorem 4.1, Section 5, and notes 7--8.
  Theorem 4.1 explicitly begins "under certain plausible assumptions";
  Lemma 4.1 assumes a uniform height/discriminant comparison and Lang's
  conjecture; the final finite-field step says the probability is
  "approximately" `(2*C2)^r/p`.
- Silverman, [The Xedni Calculus and the Elliptic Curve Discrete Logarithm
  Problem](https://uwaterloo.ca/combinatorics-and-optimization/sites/default/files/uploads/documents/1999_05.pdf).
  The public Waterloo file contains the proposal's first page only; JKSST
  reproduces the algorithm analyzed, but inaccessible proposal material was
  not treated as read.
- Shoup, [Lower Bounds for Discrete Logarithms and Related
  Problems](https://doi.org/10.1007/3-540-69053-0_18), for the explicit
  random-encoding/generic-algorithm boundary.
- Pointcheval--Stern, [Security Arguments for Digital Signatures and Blind
  Signatures](https://doi.org/10.1007/s001450010003), for the random-oracle,
  replay, query, simulator, and distinct-fork hypotheses.
- Bellare--Garay--Rabin, [Fast Batch Verification for Modular Exponentiation
  and Digital Signatures](https://doi.org/10.1007/BFb0054130), including its
  cancellation counterexample, randomization requirement, and prime-order
  boundary.
- Kate--Zaverucha--Goldberg, [Constant-Size Commitments to Polynomials and
  Their Applications](https://doi.org/10.1007/978-3-642-17373-8_11), for the
  stated degree, trusted-setup, DL, SDH/BSDH, and strong-correctness premises.
- SEC 1 v2.0, [Elliptic Curve Cryptography](https://www.secg.org/sec1-v2.pdf),
  Sections 3.2--3.3 and the small-subgroup discussion, and
  [RFC 9380](https://www.rfc-editor.org/rfc/rfc9380) on subgroup clearing,
  uniform versus nonuniform encodings, and domain separation.
- Fuchsbauer--Kiltz--Loss,
  [The Algebraic Group Model and its Applications](https://eprint.iacr.org/2017/620),
  Groth's [pairing-based SNARK](https://eprint.iacr.org/2016/260), and
  [PLONK](https://eprint.iacr.org/2019/953). At this stage these sources bound
  future target statements; no full SNARK security theorem is claimed here.

Repository sources checked first include
`research/notes/ecdlp-general/RESEARCH_LATTICE_ECDLP_ROUTES.md`,
`research/notes/ecdlp-general/RESEARCH_ECDLP_STATE_OF_THE_ART.md`, the
2026-11-01 cross-field xedni literature correction, and the prior Lean rho
retrospective in sibling `cryptanalysis`.

## Main results

### Exact counts and finite packing

- The integer coefficient cube has exact cardinality `(2*C+1)^r`; removing
  the zero vector leaves `(2*C+1)^r-1` candidates.
- Rank, radius, and minimum separation do **not** determine an exact lattice
  ball count. Explicit rank-two lattices with the same coarse parameters have
  squared-Euclidean radius-one counts `5` and `3` (and sup-norm counts `9`
  and `3`).
- A finite, basis-free replacement is proved:
  `#S <= T*(1+2*R/delta)^k`, with torsion/fiber multiplicity visible.
- The conditional height core composes a certified height ratio, packing, and
  pigeonhole into an explicit bounded nonzero relation.

### Canonical height and discriminant

- The suite constructs the canonical doubling limit from Mathlib's actual
  naive height on rational Weierstrass points. It proves convergence, exact
  parallelogram and square laws, nonnegativity, comparison constants, and,
  under Northcott, height zero exactly for torsion.
- Given a nontorsion rational point, no fixed model discriminant can bound the
  Neron--Tate heights of **all** multiples by one positive constant. This
  refutes only the universal-over-all-points reading; it is not a theorem
  about the points selected by the xedni lift algorithm.
- Mathlib variable changes give the exact weight-twelve model-discriminant
  law. Separately, an elementary short-model calculation proves unbounded
  integral rescalings and preservation of the affine equation under nonzero
  rational coordinate scaling. The suite does not compose these into a point-
  group equivalence, prove height invariance, or supply a global minimal-
  discriminant comparison.
- Lang's elliptic-curve height conjecture remains open. A scaled quadratic
  lattice countermodel proves that quadraticity, positivity, and finite rank
  alone do not imply its uniform gap.

### ECDLP and probability

- A point of exact prime order `q` induces an injective `ZMod q` scalar map.
  Group relations reduce to scalar relations; a nonzero denominator extracts
  the logarithm, while a zero denominator forces a tautology.
- For a fixed nonzero coefficient direction, the ordinary paired relation has
  exact probability `1/q`. The ECDLP-useful event, which additionally requires
  a nonzero denominator, has exact probability `(q-1)/q^2`.
- The deterministic height theorem plus a scalar-reduction certificate yields
  only the ordinary `N/q` upper bound. The tighter useful bound requires a
  separate relation-selection certificate. At the all-zero sample, no useful
  relation exists, so this bridge is not automatic.
- The denominator is subgroup order `q`, not base-field prime `p`. A `/p`
  statement is derived only from an explicit size comparison; the numerical
  Hasse-plus-cofactor module proves `p <= 4*H*q` for `p>=4` from an explicit
  `N=H*q`; its composed probability theorem substitutes `N=H*#F`. Uniform
  `C0/p` therefore additionally needs a uniformly bounded cofactor and an
  actual curve/subgroup-order instantiation.
- Composite scalar rings behave differently: over `ZMod 4`, coefficient `2`
  gives relation probability `1/2`, not `1/4`.

### Runtime

- A finite-horizon conditional-hazard recurrence implies a tail-sum lower
  bound. With a fixed numerator `K`, nonnegative normalized sample weights,
  and positive waiting times, the certified truncated expected runtime is
  genuinely `Omega(p)`.
- A one-shot relation probability is not itself the conditional-hazard
  recurrence for repeated, adaptive trials. Connecting those statements for
  the actual algorithm remains open.
- "At least `O(p)`" is not a lower-bound statement; the correct asymptotic
  direction is `Omega(p)`.

## What this changes

The accurate repository label is now:

> JKSST give a strong **conditional asymptotic obstruction** for classic
> prime-field xedni. Their height/discriminant, Lang, random-relation, and
> repeated-trial bridges are not an unconditional proof that every lifting
> strategy fails or that the displayed concrete `C0/p` bound applies.

This is a semantic correction, not evidence that xedni succeeds. The suite
proves exactly which finite statements survive and which extra premises would
be needed for an end-to-end exclusion.

## Remaining proof obligations

1. Specify the lift-selection algorithm and prove a family-uniform comparison
   between selected point heights and a **global minimal** discriminant.
2. Prove Lang for that family, or supply an unconditional direct height gap.
3. Instantiate Mordell--Weil finite generation, projection, separation, rank,
   and torsion fibers for the selected lifts.
4. Construct good reduction to the actual finite-field curve and prove exact
   prime subgroup order and sampling correspondence.
5. Prove every algorithmic success produces a bounded scalar relation; for the
   `(q-1)/q^2` bound, prove it selects a nonzero denominator.
6. Instantiate Hasse, curve order, and a cofactor bound uniform in the family.
7. Derive a repeated/adaptive conditional-hazard recurrence from the one-shot
   event theorem.
8. Prove full-row-rank for the actual cubic evaluation conditions if the
   `r<=9` step is used.

## Reproduction and trust

```sh
cd research/jksst_xedni_audit_20261101/lean
lake build
lake env lean AxiomAudit.lean
cd ../../..
tools/lean/proof-gate.sh research/jksst_xedni_audit_20261101/lean
```

The imported project pins Lean and Mathlib 4.34.1. Representative
`#print axioms` checks report only `propext`, `Classical.choice`, and
`Quot.sound`. There are no `sorry`, `admit`, project-defined axioms, or
`native_decide` proofs.

No performance measurement, curve identity, candidate route, or leaderboard
value changes in this study. The canonical index-calculus scoreboard and
progress timeline were checked and remain unchanged for that reason.
