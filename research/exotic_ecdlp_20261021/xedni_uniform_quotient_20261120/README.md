# Exact quotient bound for xedni lift relations

An exact finite quotient gives a basis-free bounded-relation lemma for the original rational-lift xedni setting. The argument replaces an asymptotic height-ball count with a quotient of cardinality `T N^d`, where `d` is the Mordell–Weil rank, `T` is the torsion order, and `N` is a chosen modulus. The core collision, kernel-gap, and rank-count implications are checked in Lean 4.

## Mathematical statement and proof

Let `H` be an abelian group isomorphic to `ℤ^d ⊕ Tors`, with finite torsion order `T`. Let `L₁,…,Lᵣ` be lifted points and `d<r`. On the free quotient, let `‖P‖=√ĥ(P)` for a positive-definite canonical height, and suppose every point with nonzero free component has norm at least `μ>0`. Suppose each `Lᵢ` has norm at most `U`. A sum `Σ aᵢLᵢ` with `|aᵢ|≤K` has norm at most `rKU`.

Reduce the free coordinates modulo `N`, retaining the exact torsion coordinate. This defines a finite quotient of cardinality `T N^d`. Every nonzero element of its kernel is `N` times a point with nonzero free component, hence has norm at least `Nμ`. If `2rKU<Nμ`, two box sums with the same quotient image must be equal: their difference lies in the kernel and has norm at most `2rKU`. If also `(2K+1)^r>T N^d`, pigeonholing the coefficient box gives two different coefficient vectors with equal sums. Their difference is a nonzero integer relation with every coefficient bounded by `2K`.

Both inequalities admit constants independent of the chosen lifts when `r`, `T`, and `U/μ` are uniformly bounded and `d<r`. Choose an integer `c` with `cμ>2rU`, set `N=cK`, and choose `K>Tc^d`. Taking `c` divisible by the torsion exponent also makes `N` annihilate torsion, so the Lean minimum-norm premise `N•u≠0 → μ≤‖u‖` follows from positivity on nonzero free components. Then `2rKU<Nμ` and the checked `rank_quotient_count` lemma gives `T N^d<(2K+1)^r`. For rational elliptic curves, the discriminant-to-point-height and Lang-type minimum-height hypotheses used in [Jacobson–Koblitz–Silverman–Stein–Teske, *Analysis of the Xedni Calculus Attack*](https://pages.cpsc.ucalgary.ca/~jacobs/PDF/xedni.pdf), Lemma 4.1, would supply the bounded ratio; the Mordell–Weil decomposition supplies the finite quotient. This is an exact alternative proof route for that conditional bounded-coefficient step.

The Lean theorem states the quotient map, its kernel divisibility and cardinality, the norm laws, and the box-radius bound as explicit hypotheses. A bridge from these abstract hypotheses to the canonical height and Mordell–Weil group of every selected rational lift is still to be formalized. The original xedni probability conclusion additionally needs a model of its curve-selection law; a finite candidate list fixed independently of the random samples is handled in the existing reduction-map packet, but arbitrary sample-dependent selection is a separate obligation.

An exact integer-lattice control in `quotient_control.py` checks the collision mechanism with rank `1`, points `(2,3)`, `K=14`, and `N=182`. It enumerated all `841` coefficient pairs, found `139` quotient residues, verified that every repeated residue had the same actual sum, and produced the relation `(-3,2)`. The recorded radius bound is `84`, so `2·84<182`; the quotient has `182` elements. This checks the finite implementation against the theorem's inequalities, not an elliptic-curve height estimate.

## Proof map

| Result | Lean declaration | Exact assumptions | Status |
| --- | --- | --- | --- |
| Coefficient-box cardinality and coordinate bound | `coefficientBox_card`, `coefficientBox_bounded` | `r,K : ℕ` | Checked |
| Quotient collision gives a bounded relation | `bounded_relation_of_quotient_gap` | Additive groups, finite quotient, triangle law, box radius, nonzero-kernel gap, strict cardinality inequality | Checked |
| Divisibility gives the kernel gap | `kernel_gap_of_divisible` | Kernel is `N`-divisible; `N`-scaling; minimum norm for `N•u≠0` | Checked |
| Rank gap gives a strict quotient count | `rank_quotient_count` | `d<r`, `N≤cK`, `Tc^d<K` | Checked |
| Combined abstract relation theorem | `bounded_relation_of_rank_quotient` | Explicit quotient and norm hypotheses above | Checked |
| Mordell–Weil/canonical-height instantiation | — | Finite rank decomposition, exact torsion label, canonical norm, uniform height ratio for selected lifts | Open |
| Curve-selection probability law | — | Exact adaptive selection distribution | Open |

## Reproduction

The source is `formal/UniformQuotient.lean`. `formal/lean-toolchain`, `formal/lakefile.toml`, and `formal/lake-manifest.json` pin Lean and Mathlib. The executed command, output, and `#print axioms` results are retained in `formal/`. The formalization uses standard Lean foundations only; the build receipt records the exact audit. `lean4checker --fresh` is unavailable on this host.
