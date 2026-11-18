# Prime31 hybrid x/y projection pilot — 2026-11-15

Standing: **[pcc:unverified]**, producer-local diagnostic, not a new index-calculus solver or cold-cost speedup.

## Objective and prior work

Check whether replacing or supplementing the existing x-only Semaev representation with oriented y/slope equations reduces sign ambiguity without causing an unacceptable growth in algebraic lifting multiplicity. Reuse the same ordinary cofactor-one prime31 curve, Dickson base and Rust arithmetic.

Prior repository leads inspected before this pilot: `native/M4_PAIR_SUPPORT_RESULTANT_20261115.md` and the existing `prime31_s3_quotient_gcd.rs` (coordinate-only S3 quotient, B8-to-B16 norm scaling approximately 60x); `native/S3_DISCRIMINANT_CLASSIFICATION_GATE_20261115.md` (known-ordinate quadratic roots can be generic group addition); #4228 (certified quotient), #4275 (dense-versus-tower), #4077 (m8 whole-call gate). Primary context: Petit–Kosters–Messeng, PKC 2016 DOI 10.1007/978-3-662-49387-8_1; Amadori–Pintore–Sala, FFA 2018 DOI 10.1016/j.ffa.2018.01.009. No novelty asserted. Research-KB/Conductor/EKS were unavailable in the tool context; GitHub repo primary files served as bounded fallback.

## Exact structural tradeoff

On `y²=x³+a x+b`, the map `P -> x(P)` generically identifies `P,-P`, with the second coordinate recoverable from `y²=f(x)`. In contrast `P -> y(P)` distinguishes `P,-P` when `y != 0`, but each known y requires solving

    x^3+a*x+(b-y^2)=0

over Fp, allowing **one or three distinct Fp roots** when the cubic is squarefree (and exceptional repeated-root cases otherwise). This is different from the usual two square-root signs in the x projection. A candidate y-only factor base must explicitly charge cubic factorization and any additional lifted branches, not assume it identifies a unique point.

For a nonvertical pair `(x1,y1),(x2,y2)`, retain oriented equations:

    t*(x2-x1)=y2-y1
    x3=t²-x1-x2
    y3=t*(x1-x3)-y1

together with each curve equation. These have low total degree before elimination but introduce another variable. Exceptional vertical, doubling and identity cases must be handled separately in any complete oracle.

## Local exact-arithmetic and finite-field factorization probe

Fixture: `p=2147483647, a=847514846, b=2087444724, n=2147446831`. Reconstructed the depth-six Dickson factor base by the Rust `Base::build` seed scan (first successful seed 3). `B=31` valid nonzero-y points, all with distinct y. For each base ordinate, factored the exact cubic modulo p using SymPy `factor_list(..., modulus=p)`, verified every recovered root by substitution and the base x's inclusion. Results:

- 14 of 31 base y-values have **one** rational x-root.
- 17 of 31 base y-values have **three** distinct rational x-roots.
- Mean rational x-lifts per base y: **65/31 = 2.096774...**. These are curve points sharing y, not necessarily members of the original base.
- 56 ordered nonvertical pairs among the first eight base points satisfied all slope equations and agreed with an independently implemented EC addition formula; 8 diagonal equal-x cases were explicitly excluded (not silently counted as failures).

The reproduction script should factor all 31 cubics and emit the raw per-y root mapping for independent audit. This run is a producer-local Python arithmetic test; it is **not** third-party validation, a performance benchmark, a full Rust F4 comparison, or a PCC-verified theorem.

## Rust source

`native/examples/prime31_xy_slope_gate.rs` reuses the existing crate's `Arithmetic`, `Base`, `PRIME31` and `check_curve`. It checks the cubic identity, distinct y's, the 56 oriented addition identities and eight equal-x exceptions. It **does not** implement cubic factorization, y-only elimination, S4 F4, or full ECDLP recovery.

Reproduce in a complete repository checkout:

    cargo test --locked --release --manifest-path research/prime31_ic_20261115/native/Cargo.toml --example prime31_xy_slope_gate
    cargo run --locked --release --manifest-path research/prime31_ic_20261115/native/Cargo.toml --example prime31_xy_slope_gate

This connector session had no cargo/rustc or full repo checkout; **the checked-in Rust example has not been compiled or executed here**.

## Decision and next controlled comparison

**Do not switch to a y-only summation polynomial yet.** On this exact curve the 17 triple fibers are a concrete risk to elimination degree/multiplicity. The next falsifiable A/B must use the same naturally sampled public targets and identical B4/B8 factor-base slices:

1. Baseline x-only S3/S4 resultant and candidate lift, including no-relation and exceptional outcomes.
2. Hybrid x/y slope constraints preserving oriented y without reducing to known-point sums in the discovery stage; count all cubic lifts, elimination degree, Gröbner matrix width, actual scalar/field work, memory and positive signed relations.
3. Pure y-only resultant as a lower-priority control with full cubic-root and multiplicity cost.
4. Deduplicate projective signed rows and measure incremental useful rank per complete invocation. Advance only on naturally discovered positives and completed cold prime31 logarithms at lower cost than matched optimized Rust rho; planted positives are semantic fixtures only.

The current result tests a **coordinate representation mechanism**, not a new algorithmic ECDLP advantage.


## 2026-11-15 follow-up: exact y-fibre decomposition and B31 support gate

**Standing: producer-local [pcc:unverified]**. Source is [`prime31_y_lift_support_gate.rs`](examples/prime31_y_lift_support_gate.rs), replay is [`y_lift_support_gate.py`](y_lift_support_gate.py), exact summary and SHA256 point-set fingerprints are [`Y_COORDINATE_SUPPORT_GATE_20261115.json`](Y_COORDINATE_SUPPORT_GATE_20261115.json). The Rust gate reuses `Arithmetic`, `Base`, `PRIME31`; all explicit EC additions in this section are **generic enumerative negative controls**, not an index-calculus relation oracle. The GitHub development workflow is wired to execute both Python and Rust tests; do not infer a successful workflow run from the file change.

### Algebraic improvement in coordinate lifting

When one point `P=(x,y)` in a fixed y-fibre is already known, the y-only cubic factors exactly:

```
X³+aX+b-y² = (X-x) (X²+xX+x²+a).
```

The *other* rational x-lifts, if any, are

```
X± = (-x ± sqrt(-3x²-4a))/2  mod p.
```

Thus no general cubic factorizer is needed to classify the fibre: a **single square-root/residuosity test** on `D(x)=-3x²-4a` suffices. For `D(x)` nonsquare, y uniquely determines the point over Fp; for nonzero-square D, the y-fibre has three rational points, except for repeated-root coincidences (which this exact input does not exhibit). This is elementary factorization, not a new algebraic ECDLP complexity result.

The depth-six Dickson fibre of 31 existing canonical points has 14 one-lift y's and 17 three-lift y's, totaling 65 rational points when the **31 ordinates alone** define membership. A y-only oracle that drops x and nevertheless treats its rows as referring to the original 31 point logs would silently introduce **34 additional unknown points**. The frozen calculation explicitly counts these extra points and refuses to promote them to the factor base without accounting for their logarithms.

### Exact y-output incidence / support results

At each of the 465 unordered pairs of **distinct** canonical base y-values, the numbers of rational y-coordinates of the *oriented* sum over their full rational x-fibres are:

| Rational y-sum outputs per pair | Number of distinct-y pairs | Interpretive limit |
| ---: | ---: | --- |
| 1 | 91 | Both fibres have one point |
| 3 | 238 | One triple x-fibre and one singleton |
| **9** | **136** | Both x-fibres have three rational points |

Every one of the 465 pairs had as many *distinct* output y's as the product of its two fibre cardinalities in the exact prime31 control. In particular, a y-only third summation relation can have nine rational outputs in the target y-coordinate for a pair where the x-only S3 relation has at most two target x-roots. This quantifies a relevant projection-degree penalty on the actual requested curve. It is an output-fibre census, **not** a F4 solver-degree theorem or a timing result.

The exact two-summand support counts, for a uniform random target in the verified prime-order group of size `n=2147446831`, are:

| Factor base semantics | Number of base points | Distinct reachable nonidentity targets (signs admitted) | Fraction of group |
| --- | ---: | ---: | ---: |
| Existing canonical x-base with `±` | 31 | **1,922** | 8.9502e-7 |
| y-only membership with **all** rational x-lifts and `±` | 65 | **8,450** | 3.9349e-6 |
| Only the 14 *injective-y* canonical points with `±` | 14 | **392** | 1.8254e-7 |

The original signed support is a subset of the y-lifted support: exactly **6,528/8,450 = 77.2544%** of its target support involves points outside the registered B31 factor base. These are not "false" relations for a *redefined* B65 base, but they would be **false assignments to the original 31 logarithm variables**. Point-set SHA256 digests (sorted decimal `x,y\\n` lines) are in the JSON; these are deterministic producer reproducibility fingerprints, not independent custody signatures.

All counts above use actual generic elliptic-curve additions **only as a support census**, and do not solve unknown scalar logarithms. The 1,922/8,450 support comparison changes both factor-base cardinality and semantics; it must **not** be presented as a faster oracle or larger IC yield at the same number of unknowns.

### A constructive counter-lead: injective-y factor base of size 31

We also built the next-depth `Base::build(p,7)` Dickson fibre with the same seed, containing **64 valid curve points**. Exactly **39** satisfy `D(x)` nonsquare, so selecting the first **31** gives a factor base of size 31 with a **unique rational x-lift for every member's y-coordinate**. Its signed two-sum support is again exactly **1,922** nonidentity points. This is a legitimate candidate *orientation-preserving coordinate representation* with the same useful base cardinality as the original B31, without the 65-point leakage.

**The price:** the selected 31 points are a filtered subset of a 64-leaf Dickson fibre, not a complete low-degree tower variety. One must construct an exact membership polynomial or a certified filter and price that work in the algebraic target-dependent oracle. The fact that this subset has the same *generic* two-sum support count is not a solver win or evidence it has favorable S4/F4 regularity. Precompute/selection and rank/accounting costs remain charged.

For any signed point factor base `F=-F`, if signs in a representation are freely chosen, a tuple representing `R` gives a tuple representing `-R` by reversing every sign. Therefore specifying the target ordinate in addition to its abscissa **does not change existence** of a decomposition into sign-flexible x-factor-base points; it may only alter algebraic representation or the post-solve orientation-lift cost. This is an exact symmetry observation, not a lower bound ruling out every hybrid solver.

### Existing solver benchmark context and decision

The previously executed same-source m4 B16 Rust quotient measurement in [`M4_NO_DUPLICATE_NORM_CI_20261115.json`](M4_NO_DUPLICATE_NORM_CI_20261115.json) measured **106.916 ms** in target-dependent norm/GCD and **0.052 ms** in signed candidate verification on one planted target. Removing only orientation-lifting/verification, with the algebraic norm unchanged, could therefore save less than **0.05% of those two stage times**. A meaningful hybrid victory would have to *reduce the algebraic system work itself*, not merely avoid four sign checks.

The lead ranking following this gate:

1. **Best new y/hybrid candidate:** encode the depth-seven 31-point injective-y subset in a certified low-cost membership polynomial, and compare the target-dependent oriented incidence system with the current x-only quotient on identical naturally sampled public targets. Count all F4/matrix costs and proof checks. This is a newly specified *experiment*, not proven novelty.
2. **Primary solving bottleneck:** use #4228's certified quotient/subresultant strategy and #4275's faster dense membership form on the inherited x-only m4 or m8 target-dependent oracle. #4275 measured dense F4 faster than its tower on decomposition positives but slower than explicit enumeration, so stage wins must be accounted for without conflating algebraic and generic tracks. #4228's quotient successor has no completed independent full-cost prime31 timing.
3. **Lower priority:** naive y-only replacement of Semaev x-coordinates; the 65-lift expansion, 136 distinct-y pairs with nine output roots and 77.25% wrong-base support are concrete disadvantages.
4. **Final pass gate:** same-source Rust A/B with normal/exceptional targets, independent projective relation rank, held-out unknown-scalar recovery and *full cold* costs against the existing optimized rho baseline. Nothing in the present signed-two-sum support census constitutes a complete ECDLP solve.

Additional prior-lead refresh on 2026-11-15: [crypto PR #4228](https://github.com/isogeny-labs/crypto/pull/4228) remains open (certified quotient development), [#4275](https://github.com/isogeny-labs/crypto/pull/4275) is closed (dense-vs-tower experiment), [#4077](https://github.com/isogeny-labs/crypto/pull/4077) is closed (sparse PKM gate), [#3877](https://github.com/isogeny-labs/crypto/pull/3877) is open (chained transfer), [cryptanalysis #1536](https://github.com/isogeny-labs/cryptanalysis/pull/1536) open (auxiliary-map IC), and [crypto-autoresearcher #3305](https://github.com/isogeny-labs/crypto-autoresearcher/pull/3305) open (Semaev alternatives prior-search atlas and audited negative results). Primary background consulted: [Semaev 2004](https://eprint.iacr.org/2004/031.pdf), PKM PKC 2016 DOI above, and Amadori–Pintore–Sala FFA 2018 DOI above. KB status/search functions and Conductor are unavailable in this connector runtime; primary GitHub files and the publishers' publicly accessible summaries were used as bounded fallback. No exhaustive novelty search is claimed.
