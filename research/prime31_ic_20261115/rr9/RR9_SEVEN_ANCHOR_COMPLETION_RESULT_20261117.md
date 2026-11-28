# Prime31 RR9 seven-anchor completion: generic-equivalence and two-value interpolation obstruction
Date 2026-11-17 UTC · PR #4547 stacked on #4546 · producer only [pcc:unverified]

## New exact findings

### Seven supplied signed leaves have no cryptanalytic advantage
Let E/F_p be the already-certified ordinary prime-order curve from the existing native Rust Prime31 project; let R!=O and let S_1,...,S_7 be **known signed affine points**. Choose F in L(9O) vanishing on the degree-eight divisor D=(S_1)+...+(S_7)+(-R). Riemann–Roch on a genus-one curve gives l(9O-D)=1, because deg(9O-D)=1. If the ninth zero T is affine, the principal-divisor relation is
```
(S_1)+...+(S_7)+(-R)+(T)-9(O)=div(F)
                  => T=R-S_1-...-S_7 in E(F_p).
```
Therefore an RR9 interpolation oracle that accepts seven *supplied signed* leaves and public R, derives the remaining x-root from the norm, and orients its sign by elliptic jets is **generic group subtraction in a different coordinate system**. It is an algebraic certificate, not non-generic relation finding. Treat all 7-of-B anchor searches as support enumeration.

Implementation on the stacked #4546 code: new `rr9_complete_seven_signed_leaves` builds the same Hermite kernel using the existing `rr9_leaves_kernel`, divides `rr9_norm` by the eight known x factors, recovers the linear quotient root and uses *one-extra-zero* Hermite jets to orient a repeated/same-x root. Positive, negative and B31 audits are registered in `RR9_SEVEN_ANCHOR_COMPLETION_PROTOCOL_20261117.md`. The ordinary curve remains p=2^31−1, a=847514846, b=2087444724, n=2147446831, h=1. Do not count this as unknown-target ECDLP recovery or a speed improvement.

### An additional exact obstruction to a naive rational-list decoder
Write `R=(r,s)`, `s!=0`, `F=(x-r)A+(y+s)V`, and `z=A/V` where defined. At any base x with x!=r and two curve y values `±y`, the two allowed rational-function values are
```
z_±(x) = -(s ± y)/(x-r).
```
They are the two *distinct* roots of the universal quadratic
```
Q_0(x,z) = (x-r) z^2 + 2s z - D_R(x),
D_R(x) = x^2 + r x + r^2 + a.
```
This is the elliptic curve equation rewritten, and is true at every admissible base x regardless of any eight-leaf relation.

**Lemma (bounded-degree all-pair obstruction):** Let H_B have b distinct roots x_i in F_p, each x_i != r and y_i !=0. Suppose Q(x,z)=a_2(x)z²+a_1(x)z+a_0(x) has all three coefficient degrees ≤ b−3, and Q(x_i,z_{i,+})=Q(x_i,z_{i,-})=0 for all b x_i. Then Q(x,z) is a polynomial multiple of Q_0(x,z). In particular for B31, degree ≤28 yields no nontrivial quadratic all-pair interpolant.

**Proof:** For each x_i, the two roots are distinct, so the polynomial of degree ≤2 in z is proportional to Q_0(x_i,z); if its leading coefficient is zero, all three coefficients vanish, and the same equations apply. Hence
```
L(x)=(x-r)a_1(x)-2s a_2(x)
C(x)=(x-r)a_0(x)+D_R(x)a_2(x)
```
vanish at every x_i. The assumed coefficient degrees give deg L ≤ b−2 and deg C ≤ b−1, so both polynomials with b distinct roots are identically zero. Since 2s ≠0, x−r divides a_2; put `a_2=(x-r)h`. Then `a_1=2s h` and `a_0=-D_R h`, giving Q=h Q_0. QED.

This result does NOT rule out higher-degree/higher-z-degree interpolation, partial-list constraints, saturations or a genuine public-only extractor. It does show that naive low-degree quadratic interpolation through **both** signs at every B31 base x merely rediscovers the curve equation. A decoder should quotient/saturate the universal component, then measure the remaining ideal rather than claim a new relation oracle.

## Prior work and evidence boundary
- #4466 Hermite/norm; #4539 public 7-coefficient chart; #4541 squarefree/radical; #4545 B8 signed fibres; #4546 exceptional x=r tangent jets; #4373 capped m8 F4; #4228/#4349 quotient/rootfinding.
- #4513 full mixed-base prime31 solver and #4532 matched shared-precomputation comparison. **Historical producer run**: 1,024 complete IC logs 2,221.474 ms; shared BSGS 170.113 ms (13.06x faster). Not remeasured or certified here.
- Crypto-autoresearcher #3301 and #3305 document negative leads and prevent unchanged repetition. Published prime-field IC: Amadori, Pintore, Sala, *Finite Fields Appl.* 51 (2018), 168–182, DOI 10.1016/j.ffa.2018.01.009, https://eprint.iacr.org/2017/609. These identities are algebraic analysis, not a claim of new prior-art priority.
- Research-KB service/CLI unavailable in this execution environment; reviewed committed PRs, searchable repository content and publisher/institutional primary-literature records instead. No exhaustive search claimed.

## Observational status and next experiment
Source code revision 2206f458f486eb84694058196bed0a1fd5dd2fc7 is committed on #4547. The exact-source `.github/workflows/prime31-rr9-evaluation.yml` Rust 1.100 check was initiated at https://github.com/isogeny-labs/crypto/actions/runs/44074481999. Final verdict and raw evidence MUST be recorded separately once observed; a running workflow is not a PASS.

Next decisive test: eliminate **unknown support choices entirely** in the seven-variable target-conditioned RR9 coefficient ideal for B8, with both exceptional sign branches and absent-target negative fibres. Existing Rust quotient/F4 machinery, not a replacement scalar solver, should be used. Pre-register the cost of building ideal, saturation by Q_0, field operations, failed residuals and memory. After exact solution-set validation, B31 must recover unknown Q logarithms via full relation rank and compare entire cold and amortized cost to cached generic BSGS (including setup), tuned rho and the external IC frontier. No leaderboard/PCC promotion without independent replay.

## Final source-specific CI verdict (subsequent to the above registered notes)
- **SUCCESS:** [pinned Rust 1.100.0 GitHub Actions #44075156178](https://github.com/isogeny-labs/crypto/actions/runs/44075156178) on source revision `d91be2fc2a5fead876b938701d592bb0c28c1ad4`. All **40/40 native library tests**, all **9/9 RR9 example tests** including new `rr9_seven_anchor_generic_equivalence_on_actual_prime31`, and the workflow's **64 positive + 64 negative original-curve RR9 predicate panel** passed. The registered new test asserts all 256 B8 sign-mask cases (skipping only infinity residuals, with ≥250 finite), two exceptional R/−R Hermite fixtures, four off-base negatives, and 64 B31 given-seven-support completions. Each returned eighth point equals an independently computed original-curve group sum. All tests are native Rust, using original p31 curve.
- [Raw workflow JSON artifact #14324307382](https://github.com/isogeny-labs/crypto/actions/runs/44075156178/artifacts/14324307382) (14,485 bytes archived); producer-only `[pcc:unverified]`. The workflow stdout prints the old 128-case determinant panels, while the 256/64 assertions execute inside the 9th Rust example test and the additional JSON section. No independent signed PCC verdict.
- Source-only failed predecessors are preserved: #44074341581 first Rust type-alias error, #44074481999 mutable borrow E0499; corrected without changing frozen mathematical gates. The final source test passed. No new complete unknown-target logs or matched full-solver speed comparison. The measured interpolation wall field in raw JSON is diagnostic only, not a claimed generic speedup.
