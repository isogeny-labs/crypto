# Corrected degree-four exclusion across twelve trace -14 carriers

16 November 2026. A source audit found that the prior trace -14 degree-four `linear_only` arm tested `B=x+1` even though the carrier's rational branch factor is `x`. That receipt alone did not cover the required `B=x` case. The corrected [source](screen.sage) retains the old receipt, uses the actual unique rational branch factor `L` for every denominator, and extends the screen to all twelve trace -14 monic odd-degree carrier equations in the recorded census. All twelve corrected cells completed with **zero degree-four numerator candidates** in all four normalized denominator classes. [pcc:unverified]

The exact field is `K=F3[a]/(a^5+2a+1)`. The source is `E: v²=u³+u²+a²+2`, with 258 points over `K`. For each mask `m` in the list below, the carrier is `Y_m: y²=f_m(x)`, where `f_m=x^11+Σ_(i=0)^10 digit_3(m,i)x^i`:

```text
9735  17371  21624  44227  64725  90937
92153  100118  124586  144548  156795  159256
```

For any degree-four map translated so the carrier's rational point at infinity maps to the elliptic origin, the elliptic x-coordinate is `u=A/B` with `deg A=4`, `deg B∈{1,3}`. The parity argument in [the earlier fixed-carrier theorem](../ghs_degree7_remaining_20261116/DEGREE_SEVEN_FIXED_CARRIER_THEOREM.md) leaves exactly `B=L`, `L³`, `L(x-c)²` for `c` away from the branch root, and `LQ_i`, where `Q_i` runs over the five quadratic branch factors. At each unpoled `Q_i`, the code enumerates all compatible elliptic two-torsion residues and uses CRT to reconstruct the unique possible `A` of degree below the modulus. None of the 716,364 assignments across the twelve cells gave `deg A=4`; the full curve identity therefore needed no squarehood check in this run.

The [audit](AUDIT.json) checks the twelve masks against the prior carrier census, source hashes, exact model fields, per-shape assignment counts, and zero candidate fields. The cumulative Sage CPU time was 24.532 seconds. [DegreeFourCounts.lean](DegreeFourCounts.lean) checks the twelve-item count and 716,364-assignment arithmetic; Lean does not formalize the denominator parity argument or CRT elimination. The earlier [TraceMinus14Arithmetic.lean](../ghs_trace_minus14_20261116/TraceMinus14Arithmetic.lean) also rechecked successfully.

**Bound:** for each of these twelve exact carriers and this exact source, there is no `K`-morphism of degree four, conditional on the written normalization and the one Sage elimination implementation. The prior geometric exclusion of degrees one through three gives a minimum degree of at least five on these pairs. For the original mask 9735, the separate corrected degree-five through seven receipts remain zero, so its previously stated fixed-pair interval `[8,33]` survives this audit. Howe's published theorem supplies the upper endpoint 33; it does not construct coordinate formulas here. This is a stronger finite cover-degree obstruction, not a vulnerability demonstration.

The old `degree4_all.sage` and its JSON remain as historical evidence of the error; this result supersedes its degree-four coverage claim. The same source lineage should be reviewed when adapting branch factors in other traces. Prior literature and novelty limits are in the [frozen protocol](PROTOCOL.md): [Iijima–Momose–Chao 2015](https://eprint.iacr.org/2015/805.pdf) classifies a different specified direct-cover type, and [Howe 2022](https://arxiv.org/pdf/2110.04221) supplies abstract bounded-degree existence. This finite exclusion is new local evidence, not a new eligible or vulnerable family.
