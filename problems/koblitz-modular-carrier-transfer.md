# Koblitz modular carrier: executable subgroup transfer and IC relations

- **Status:** open
- **Kind:** implementation gap; low-genus existence question; partial formalization
- **Affected work:** [investigation](../research/koblitz_modular_ic_20261018/REPORT.md), [original construction](../research/notes/ecc2k130/RESEARCH_ECC2K130_CURVE_CONSTRUCTION.md)
- **Next action owner:** unassigned
- **Last updated:** 2026-10-18

## Problem

The modular construction realizes the ECC2K-130 trace-zero isogeny factor,
but no evaluated map preserving its prime subgroup or efficient IC relation
algorithm on that factor has been constructed.

The source is E_0/F_(2^131), registered as
`icv1-f2m131-tm22283658519494248867-115e0dc5`, prime subgroup order
`680564733841876926932320129493409985129`. The signed-character carrier
has genus 378,708,949 and desired factor dimension 130. Neither number is
a measured recovery cost. No challenge point is imported or attacked.

## Evidence and uncertainty

- **Observed:** the native prime-modulus scan and exact Frobenius checks
  in [results.json](../research/koblitz_modular_ic_20261018/results.json).
- **Derived:** the character modulo 787 supplies the trace sign without
  a separate conductor-3 twist. The written cyclic argument rules out
  geometrically cyclic degree-131 carriers of genus <394, for any quotient
  genus, with the descent and simplicity hypotheses in the report.
- **Suspected explanation:** no claim that every low-genus route is blocked.
  Noncyclic covers, gluing/polarization, other map degrees and projected
  relation algorithms remain separate possibilities.
- **Missing capability:** explicit map evaluation, r-torsion kernel/action
  certificate, destination relations/descent, and a full geometric Lean bridge.
- **Not established:** a carrier of genus 130--300, a useful modular
  quotient, faster IC, complete cost, or nonexistence outside the proved family.

## What would unblock it

- [ ] Supply a concrete F_2 curve/correspondence or a direct modular relation
  algorithm with exact inputs and an evaluator; retain all exceptional cases.
- [ ] Verify a nonzero order-r image and recoverability, including the map's
  kernel and any projector denominator, on public synthetic fixtures.
- [ ] Implement relation generation, rank, individual descent and recovery;
  compare complete cold costs with the current IC and signed-Frobenius rho
  baselines on identical one-target fixtures.

The first two conditions establish an executable transfer. The third is
needed for an IC improvement; existence of the transfer alone is insufficient.

## Next discriminating step

- **Action:** test a specified noncyclic construction, polarization/gluing
  candidate, or modular quotient at the n=5 rung; do not repeat the excluded
  degree-131 cyclic family as a search for a genus <=300 carrier.
- **Inputs and controls:** both Koblitz signs; known n=3 carriers; n=7 as
  a split-target control, not an analogue of simple A_131.
- **Limits:** preregister the selected construction and finite scope before
  running it; no execution is scheduled by this problem record.
- **Outcomes:** a verified curve and map justify scaling that specific
  mechanism; bounded failure leaves other constructions open; an abstract
  factor without a map leaves the executable-transfer condition unresolved.
- **Recheck trigger:** a specified construction or independently reviewed
  proof closes one of these missing bridges.

## Resolution and history

2026-10-18: new investigation records the signed-character construction,
exact arithmetic controls, and extension of the cyclic bound. Unresolved.

## Round 02: evaluated control and remaining constructions

[Round 02](../research/koblitz_modular_ic_20261018/rounds/02/REPORT.md)
adds an explicit n=3 control: all seven source subgroup points are transferred
through all 56 Klein quotient maps, 42 nonzero and 14 zero, with 393
principal-function certificates checked separately. Geometric line norms
produce 12,036 relations on 65 closed-point columns of rank64 modulo7;
anchoring the flex divisor recovers all seven public fixtures. The smaller
F8 factor base recovers four fixtures and retains three failed descents.
This closes the n=3 executable-control obligation; it does not resolve the
registered n=131 transfer or its relation algorithm.

The [ledger](../research/koblitz_modular_ic_20261018/rounds/02/ledger.json)
preserves finite searches of 722 genus-5 models and 2,424,825 genus-6 forms
or sections, with zero A5 polynomial candidates in the enumerated families.
Two del Pezzo Frobenius types (5 and 3+2), other genus-5 constructions and
higher genera remain untested. Frobenius-module gluing candidates at primes
5,11,19 have been enumerated; integral polarized gluing and curve realization
remain missing. The companion lattice calculation does not enumerate all
isogeny representatives.

At n=131 the separate-orbit recurrence matrices have rank260 and rank261
with a base anchor; a supplied cross row gives rank262 as an information
control. The Eisenstein character-pair screen fails its necessary condition
at five inert primes, but no general cusp/Hecke identification is proved.
Actual higher-n interior relations remain open.

Thirteen additional structural Lean lemmas compile and have standard-foundation
axiom audits. The full geometric bridges are still open and named in the
[proof map](../research/koblitz_modular_ic_20261018/rounds/02/PROOF_MAP.md).
Next discriminating constructions are the remaining del Pezzo twists,
Rosati-compatible integral gluing lattices, an evaluated n=5 map, and interior
principal-function relations on its modular factor. The user determines IC
speed assessment; this round contains no timing comparison.
