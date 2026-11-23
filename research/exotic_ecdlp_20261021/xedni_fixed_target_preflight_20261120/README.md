# Fixed-target coverage of the full-law CRT relation policy

**The fixed-coefficient construction has an exact target restriction:** a rational relation `P + 2Q = O` on a lift with good reduction at 47 projects to `T + 2G = O` on `E(F₄₇)`. For a fixed generator `G`, its only compatible target is `T = -2G`. The frozen input law has 42 nonidentity generators and 41 ordered, distinct nonidentity target choices per generator. Exactly one target per generator meets the reduced relation. Among those 42 compatible ordered inputs, the full-law CRT policy produces rational relations for two, at pair indices 425 and 467. It produces eight rational relation records on those two inputs.

| Fixed-generator projection | Exact count |
| --- | ---: |
| Generators in the input law | 42 |
| Distinct nonidentity targets per generator | 41 |
| Compatible targets per generator under coefficient pair `(1,2)` | 1 |
| Compatible ordered inputs | 42 of 1,722 |
| Ordered inputs with a rational CRT relation | 2 of 1,722 |
| Generators with a rational CRT relation on the compatible target | 2 of 42 |
| Local coefficient residues examined, full law | 2,249,856 |
| Centered CRT curves constructed, full law | 32,256 |

The two hit generators are `(17,10)` with compatible target `(14,7)` and `(17,37)` with compatible target `(14,40)` in the chosen field encoding. Each generator's 41-target scan examines 10,976 local residues at 7 and 42,592 at 11, constructs 768 centered CRT curves, and produces four rational relation records. The other 40 generators have no rational relation record under this policy. The full [machine-readable result](result.json) preserves counts and the compatible target for every generator. The [analysis script](analyze.py) checks the raw hashes of the frozen source and CRT census, verifies every pair's index and reduced-event flag, and independently recomputes the projection totals from those rows. The earlier [full-law packet](../xedni_full_law_crt_20261120/README.md) contains the independent arithmetic audit and Sage replay of the rational witnesses.

## Proof and scope

Let `ρ : E(ℚ) → E(F₄₇)` be the group homomorphism supplied by good reduction at 47. Applying it to `P + 2Q = O` gives `ρ(P) + 2ρ(Q) = O`. With `T=ρ(P)` and `G=ρ(Q)`, cancellation gives `T=-2G`. Thus a fixed `G` has at most one compatible `T`. The finite count above additionally uses the recorded 43-element group and the input law excluding the identity and `T=G`; the census itself checks that every `G` has exactly one compatible target among its 41 choices.

The [Lean theorem](formal/FixedTarget.lean) `XedniFixedTarget.projected_relation` proves the homomorphic projection for arbitrary additive groups and an additive homomorphism, retaining the exact coefficient `2`. `XedniFixedTarget.unique_target` proves uniqueness for fixed `G`. The formal hypotheses state the good-reduction map as an additive homomorphism; this module does not construct the arithmetic reduction map or formalize the 43-point census. The adjacent [rational witness theorem](../xedni_crt_lifts_20261120/formal/Witness.lean) checks one actual lifted curve relation. The proof check uses Lean `v4.35.0-rc4` and Mathlib revision `021c46bdd042f67b538b9f9871d556a93713a9a9`, with local pinned manifest and toolchain. [Build output](formal/build-output.txt) reports only the standard `propext` axiom for the two new theorems. `lean4checker --fresh` was unavailable on this host.

This is a structural cost gate for the **fixed coefficient policy**. For any other public target, the rational relation event is impossible by the reduction theorem, so running the CRT search cannot yield that relation. For the compatible target, the relation itself is already testable by one reduced-group doubling and addition. An unknown-target algorithm would require an independently justified way to obtain unknown coefficients or relations that carry new scalar information. Its query work, relation system, descent, recovery and same-target rho control remain separate experiments. The recorded local-residue and CRT-curve counts are algorithmic work counts, not interchangeable with rho group operations or calibrated wall time.

Replay:

```sh
python3 research/exotic_ecdlp_20261021/xedni_fixed_target_preflight_20261120/analyze.py > research/exotic_ecdlp_20261021/xedni_fixed_target_preflight_20261120/result.json
sh research/exotic_ecdlp_20261021/xedni_fixed_target_preflight_20261120/formal/build.sh > research/exotic_ecdlp_20261021/xedni_fixed_target_preflight_20261120/formal/build-output.txt
```
