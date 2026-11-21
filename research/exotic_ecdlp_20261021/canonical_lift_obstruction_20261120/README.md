# Additive lift and logarithm obstruction

For a finite abelian group of points, an additive lift followed by an additive
logarithm into a torsion-free group has zero image. The result applies to the
proposed canonical-lift variant when its lift and logarithm satisfy those
algebraic hypotheses. An arbitrary section instead has an addition defect in
the kernel of reduction; its logarithm contributes an explicit correction to
each addition equation.

## Mathematical statement and proof

Let `G` and `H` be abelian groups, `s : G → H` an additive lift, and
`L : H → A` an additive map. Assume `A` has no nonzero element killed by a
positive integer. If `nP = 0`, then

`n L(s(P)) = L(s(nP)) = L(s(0)) = 0`, hence `L(s(P)) = 0`.

When `G` is finite, `n = |G|` annihilates every `P`, so this conclusion holds
for all points. This includes a finite elliptic-curve group and a
torsion-free additive target such as the additive group of `ℚ_p`, when an
additive lift and an additive logarithm on the chosen lift group have actually
been constructed. The theorem
does not assume a particular elliptic curve, formal group, canonical-lift
construction, or algorithm for computing a logarithm.

Now let `ρ : H → G` be additive and let `s : G → H` be any section of `ρ`.
Define `δ(P,Q) = s(P+Q) − s(P) − s(Q)`. Then
`ρ(δ(P,Q)) = (P+Q) − P − Q = 0`. If an additive logarithm is defined on all of
`H`, its exact equation is

`L(s(P+Q)) = L(s(P)) + L(s(Q)) + L(δ(P,Q))`.

Thus the selected lift's additive equation carries a kernel-valued defect.
The identity alone does not establish how hard the defect is to compute or
whether a different lift-based algorithm could use it.

## Scope of the xedni connection

The [original xedni proposal](https://pages.cpsc.ucalgary.ca/~jacobs/PDF/xedni.pdf)
uses rational lifts and rational-point dependence. Its rank and selection-law
questions are treated separately in the existing xedni obstruction packets in
`/Volumes/SSD990/cryptanalysis/experiments/`. This packet addresses the
*canonical-lift/p-adic-log variant* in the supplied lead list. A formal-group
logarithm is normally defined on a specified domain, so application to an
actual elliptic-curve construction requires a proved map from the chosen lift
group into that domain and an additive law there. Those bridges are not
encoded by the abstract Lean theorem.

## Proof map and verification

| Result | Lean declaration | Exact assumptions | Status |
| --- | --- | --- | --- |
| Section defect reduces to zero | `defect_reduces_to_zero` | abelian `G,H`; additive `ρ`; `ρ ∘ s = id` | Checked |
| Additive section defect vanishes | `defect_of_additive_section` | abelian `G,H`; additive `s` | Checked |
| Log equation contains defect | `log_section_addition` | abelian `G,H,A`; additive `L : H → A` | Checked |
| Finite-order lift log vanishes | `log_homomorphic_lift_eq_zero` | abelian `G,H,A`; additive `s,L`; explicit positive-`n` torsion-freeness in `A`; `nP=0` | Checked |
| Finite-source log vanishes | `finite_source_log_homomorphic_lift_eq_zero` | preceding assumptions; `G` finite | Checked |
| Actual canonical elliptic lift and log domain bridge | none | exact curve, reduction map, log domain, lift law | Open |

The source is `formal/CanonicalLiftObstruction.lean`. It has no `sorry`,
`admit`, custom axiom, or kernel bypass. `#print axioms` reports only Lean's
standard `propext`, `Classical.choice`, and `Quot.sound` as applicable. The
checked toolchain and dependency revision are pinned in `formal/lean-toolchain`,
`formal/lakefile.toml`, and `formal/lake-manifest.json`; the executed command
is in `formal/build-command.txt`, and its output is in `formal/build-output.txt`.
The host's existing Mathlib build has the same pinned revisions and supplies
the compiled dependency objects. `lean4checker --fresh` was not
available on this host.
