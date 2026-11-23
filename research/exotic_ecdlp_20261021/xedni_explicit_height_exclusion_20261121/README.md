# An explicit height exclusion for the fixed-curve rational lift

For `E/ℚ: y²=x³−47x²+751x−3997`, every rational point reducing modulo 47 to the frozen target `T=(10,1)` has rational-coordinate height **`H=max(|a|,b)>2^130`** under the candidate law `x=a/b²`, `gcd(a,b)=1`, `b>0`. The proof combines the certified rank-one generator `Q=(17,10)`, the reduction class `n≡22 (mod 43)`, an exact 65Q coordinate, and the explicit Silverman comparison between the naive and canonical heights on this exact Weierstrass model.

| Ingredient | Exact value or bound | Verification |
| --- | --- | --- |
| Rational points and target class | `E(ℚ)=ℤQ`, `n≡22 (mod 43)`, hence `n²≥441` | [generator packet](../xedni_rank1_generator_certificate_20261121/README.md), Lean integer theorem |
| Model invariants | `Δ=−2835200`, `j=1362944/11075`, `b₂=−188` | Checked Sage and independent exact Fraction audit |
| Witness coordinate | `x(65Q)` numerator has 2,894 bits, denominator 2,891 bits | Checked Sage and independent exact Fraction audit; SHA-256 in `result.json` |
| Height comparison | `|h(x(P))−ĥ(P)|<14` for this model | Exact bounds on Sage's published Silverman formula, conditional on the standard height theorem |
| Height consequence | `h(x(nQ))>785218/4225>182` | Lean real-inequality theorem and independent rational audit |
| Candidate cap | `H≤2^130` implies `h(x(nQ))≤260 log 2<182` | Square-denominator law and Lean log bound |

Here `h(x)=log max(|num(x)|,|den(x)|)` and `ĥ` uses Sage's canonical-height normalization. The explicit comparison formula is documented in [Sage's rational elliptic-curve reference](https://doc.sagemath.org/html/en/reference/arithmetic_curves/sage/schemes/elliptic_curves/ell_rational_field.html), which references Silverman's height-difference theorem and the Bremner correction. The installed Sage source used for this run computes

```
μ = h(Δ)/12 + h∞(j)/12 + h∞(b₂/12)/2 + log(2)/2
B = max(|2(−h(j)/24−μ−0.961)|, |2(μ+1.07)|).
```

Exact invariant comparisons give `h(Δ)<22`, `h∞(j)<7`, `h∞(b₂/12)<4`, and `h(j)<21`, using `log 2<1`. Both branches are then below 14; the independent audit checks the rational inequalities. The default and mwrank implementations each reported `9.822168140347129` as numerical diagnostics, but the derivation uses the conservative rational bound 14.

The numerator of `x(65Q)` has 2,894 bits, so `h(x(65Q))≥2893 log 2`. Canonical-height quadraticity gives `ĥ(65Q)=4225ĥ(Q)`. Every target lift is `nQ` with `n²≥441`; combining the two height inequalities and `2/3<log 2<7/10` yields

```
h(x(nQ)) > 441(2893·(2/3)−14)/4225−14
          = 785218/4225 > 182.
```

If `H≤2^130`, the candidate representation gives `h(x(nQ))≤log max(|a|,b²)≤260 log 2<182`, a contradiction. The height cap, curve, public target, and candidate law were frozen in [`protocol.json`](protocol.json) before execution. The exact input and output receipts are [`result.json`](result.json) and [`audit-result.json`](audit-result.json); checked-Sage runtime information and stderr are retained.

## Proof map

| Result | Lean declaration or evidence | Status |
| --- | --- | --- |
| `2/3<log 2<7/10` | `XedniExplicitHeightExclusion.log_two_bounds` | Lean checked |
| `n mod 43=22 ⇒ n²≥441` | `target_min_square` | Lean checked |
| Sage's displayed formula is below 14 under the exact invariant-height bounds | `silverman_formula_lt14` | Lean checked conditional real inequality; exact invariant inputs independently audited |
| Canonical-height inputs contradict the candidate cap | `no_lift_under_cap` | Lean checked conditional real inequality |
| Mordell–Weil generator and reduction class | [rank-one packet](../xedni_rank1_generator_certificate_20261121/README.md) | Proof-enabled Sage mwrank and independent replay; curve-specific Lean bridge open |
| Silverman's comparison and canonical-height quadraticity on `E` | Sage's documented theorem and standard elliptic-height theory | Applied mathematically; curve-specific Lean bridge open |

The pinned [`formal/HeightExclusion.lean`](formal/HeightExclusion.lean) contains no `sorry`, custom claim axiom, or native-evaluation bypass. Its `#print axioms` receipt lists only `propext`, `Classical.choice`, and `Quot.sound`. The full curve-specific theorem is **partially formalized**: it still needs Lean definitions and proofs for this model's Mordell–Weil generator/reduction certificate and the canonical-height comparison theorem. The written argument and exact arithmetic audit preserve those inputs without replacing them with an assumed curve theorem.

The next construction question is whether a target-dependent rational lift can be found substantially faster than enumerating multiples or bounded public-coordinate search; this exclusion supplies a concrete height scale for that experiment.

Replay from the repository root:

```sh
/Volumes/SSD990/cryptanalysis/sage --runtime-info
/Volumes/SSD990/cryptanalysis/sage -python "$(pwd)/research/exotic_ecdlp_20261021/xedni_explicit_height_exclusion_20261121/run_sage.py"
python3 research/exotic_ecdlp_20261021/xedni_explicit_height_exclusion_20261121/audit.py
sh research/exotic_ecdlp_20261021/xedni_explicit_height_exclusion_20261121/formal/build.sh
```
