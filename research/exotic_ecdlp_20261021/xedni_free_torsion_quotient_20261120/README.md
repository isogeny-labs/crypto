# Explicit free-plus-torsion quotient for the xedni height bound

The quotient used by the [exact xedni bounded-relation argument](../xedni_uniform_quotient_20261120/README.md) now has a concrete Lean construction. For an arbitrary finite abelian group `A` and free rank `d`, the map

`ρ : (Fin d → ℤ) × A → (Fin d → ZMod N) × A`

reduces each free coordinate modulo positive `N` and retains the full finite coordinate. The target has exactly `|A| N^d` elements, the map is surjective, and every kernel element is an `N`-multiple with zero finite coordinate. Thus the preceding proof packet's quotient-cardinality and kernel-divisibility hypotheses hold for an explicit `ℤ^d ⊕ A` model.

## Mathematical proof

The cardinality is `|(Fin d → ZMod N)| · |A| = N^d |A|`. Every residue coordinate has an integer representative, giving surjectivity while retaining the finite coordinate. If `ρ(z)=0`, its `A` coordinate is zero and each integer coordinate `zᵢ` is divisible by `N`, by the exact criterion `(zᵢ : ZMod N)=0 ↔ N∣zᵢ`. Choose integer quotients `uᵢ` and form `u=(uᵢ,0)`; then `z=N•u`.

If `N` annihilates `A`, a point with nonzero `N`-multiple has a nonzero free coordinate. The checked `min_norm_of_nonzero_multiple` theorem therefore transfers a minimum norm on nonzero free coordinates to the precise norm hypothesis in the previous packet. For a rational elliptic curve, a Mordell–Weil decomposition identifies its rational-point group abstractly with such a product. The remaining formal bridge must instantiate that decomposition and the canonical-height norm for the selected lift family. The original paper's conditional height ratio and the xedni curve-selection law remain separate proof obligations; see [Jacobson–Koblitz–Silverman–Stein–Teske](https://pages.cpsc.ucalgary.ca/~jacobs/PDF/xedni.pdf), Lemma 4.1 and Theorem 4.1.

The independent arithmetic control `quotient_control.py` enumerated all `3,750` points in the window `[-12,12]^2 × ℤ/6ℤ` with `N=6`. It covered all `216` quotient elements and checked divisibility for all `25` kernel points in that window. This is a direct control of the implementation's residue and torsion handling; the Lean proof covers arbitrary free rank and finite abelian `A`.

## Proof map and verification

| Result | Lean declaration | Exact hypotheses | Status |
| --- | --- | --- | --- |
| Exact quotient cardinality | `XedniFreeTorsionQuotient.quotient_card` | `A` finite, `N≠0` | Checked |
| Kernel finite coordinate is zero | `XedniFreeTorsionQuotient.kernel_torsion_zero` | Additive `A`; `ρ(z)=0` | Checked |
| Surjectivity | `XedniFreeTorsionQuotient.quotient_surjective` | `A` additive, `N≠0` | Checked |
| Kernel divisibility | `XedniFreeTorsionQuotient.kernel_divisible` | Additive `A`; `ρ(z)=0` | Checked |
| Minimum free norm transfers to the precise multiple condition | `XedniFreeTorsionQuotient.min_norm_of_nonzero_multiple` | `N` annihilates `A`; nonzero free coordinate has norm at least `m` | Checked |
| Actual rational elliptic-curve group and canonical-height instantiation | — | Mordell–Weil equivalence, torsion coordinate, height norm and uniform height ratio | Open |

The Lean 4 source is `formal/FreeTorsionQuotient.lean`. The toolchain and Mathlib revision are pinned in `formal/lean-toolchain`, `formal/lakefile.toml`, and `formal/lake-manifest.json`. The executed build command, output, and `#print axioms` audit are retained in `formal/`. No research-specific axiom or admitted proof is used. `lean4checker --fresh` is unavailable on this host.
