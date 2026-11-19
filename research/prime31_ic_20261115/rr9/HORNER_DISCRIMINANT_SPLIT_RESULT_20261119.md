# Prime31 Horner/F3: exact discriminant splitting closes the extension-root caveat

Date: 2026-11-19. Existing [PR #4812](https://github.com/isogeny-labs/crypto/pull/4812), source-bound original Prime31 Rust arithmetic. **Mathematical correctness finding, not a public-only ECDLP solver or a measured index-calculus speedup.** [pcc:unverified]

## Theorem / derived algebraic identity

Let K be a field of characteristic different from 2 and 3, E: y² = f(x) = x³+a x+b be nonsingular, and

    D(z) = 4 f(z),
    N(z) = z^4 - 2 a z² - 8 b z + a²,
    M(z) = z^6 + 5 a z^4 + 20 b z³ - 5 a² z² - 4 a b z - 8 b² - a³.

The standard doubling identity is

    N(z)^3 + a N(z) D(z)^2 + b D(z)^3 = M(z)^2.

For the eliminated-Horner equation `E(u,z,r) = D(z)^2 F3(u,N(z)/D(z),r)` defined *polynomially* by the cleared expression in the previous report, its quadratic leading coefficient and discriminant in r are exactly

    L_r(E) = (N(z) - u D(z))²
    disc_r(E) = 64 f(u) f(z) M(z)².

Proof: the discriminant of Semaev's F3(u,v,r) as a polynomial in r is exactly 16 f(u) f(v). Scaling its coefficients by D² multiplies its discriminant by D^4. Substitute v=N/D and use f(N/D)=M²/D³. This gives 16 f(u) D M² = 64 f(u) f(z) M². Both identities are *polynomial identities*, with no division in the final form.

**Symbolic cross-check:** Separately authored SymPy expansion over the indeterminates `(u,z,a,b)` (not just sampled Prime31 values) returned `True` for all three exact identities: the leading-coefficient formula, the discriminant formula, and the duplication numerator identity. This is an algebraic-computer check, not a Lean/kernel formal proof. Since every geometric root of the *full* chart has all denominators nonzero, algebraic saturation of its ideal by the product of the seven `D(t_{i+1})` is **already equal to the original full ideal**; explicit inverse variables are unnecessary for geometric correctness. An F4 implementation may nevertheless exhibit different intermediate costs if it introduces redundant saturation constraints.

**Consequences for the specific unchanged Prime31 chart:** The curve over Fp has odd prime order n=2147446831 and cofactor 1; its 31 factor-base points all have ordinary rational x,y, with distinct x values and no order-2 or order-4 points. For every ordinary rational u=x(U), z=x(Z) with U,Z nonidentity:

- D(z), f(u), f(z), M(z) are all nonzero. The discriminant is a nonzero square in Fp; E(u,z,r) is squarefree in r. Its degree is 2, or drops to a nonzero linear polynomial when u=x([2]Z).
- Every root r in the algebraic closure is in Fp. More concretely, it is the x-coordinate of U+[2]Z or U-[2]Z whenever that sum is finite; if one is the identity, only the other finite root remains.
- The eight H_i(u_i) in the **full** 14-variable chart split into *distinct* ordinary Fp base values. Start at t7=u7 and descend i=6,...,0. At every step u_i and t_{i+1} are ordinary Fp x-values, so every algebraic root t_i is again an ordinary Fp x-value. Hence **every geometric affine solution of the complete factor-base-specialized triangular 14-variable system is Fp-rational**, including after imposing t0=x(public Q). Each triangular root is simple, so this finite affine chart is geometrically reduced as well.
- This is stronger than the earlier *isolated-equation* caution. At a nonrational geometric root f(z)=0, the standalone cleared E degenerates to (r-u)² N(z)², exposing a potential 2-torsion branch r=u. **But those branches cannot occur in the complete triangular chart** because its bottom leaf t7 and every subsequent suffix are forced ordinary rational x-values by the full constraint system. It is therefore incorrect to infer that isolated-E extension components create actual full-system roots, although an intermediate F4 elimination may still carry expensive polynomials.

These conclusions apply to the fixed ordinary factor-base chart; they do not establish any claim about unrelated curves, twists, positive-dimensional variants, different factor-base equations, or complete F4 computation cost. The argument concerns the **affine root set**, not an automatic simplification of arbitrary intermediate ideals.

## Experiment and validation

Native code: the existing original-crate [x-only Horner Rust example](../native/examples/prime31_horner_xonly_sign_lift.rs), extended rather than forked. Frozen [protocol](HORNER_DISCRIMINANT_SPLIT_PROTOCOL_20261119.md) was committed as SHA `5aae81de213997ec1844585c1ec582fe494d8ac1` before the Rust patch. Source code patch SHA `0a4a396763bf174d7d9ef203cd150e87ed6b907e`.

**Separately authored exploratory Python preflight (not a preregistered native measurement):**
- 10,000 deterministic uniformly drawn (u,z) pairs and 40,000 evaluations of the reconstructed polynomial coefficients: exact agreement.
- 10,000 independent duplication-polynomial identities: exact agreement.
- All 961 ordered pairs of the first 31 ordinary factor-base points: both signed affine sums are finite and satisfy eliminated E, so 1,922/1,922 positive root certificates; no linear/identity exceptions in this particular 31×31 base-pair cross-product. These sample observations do not prove exceptions absent across arbitrary suffixes.
- Fp-cubic f(z)=z³+a z+b has no Fp root and is irreducible for this odd-order Prime31 curve, so standalone torsion roots first appear in Fp³.

**Source-specific GitHub Actions [#44598103355](https://github.com/isogeny-labs/crypto/actions/runs/44598103355) PASSED** on exact candidate test SHA `0a4a396763bf174d7d9ef203cd150e87ed6b907e` at 2026-11-19 19:28 UTC. Original Rust library 40/40; original x/y tests 2/2; x-only tests 4/4 including `eliminated_horner_discriminant_and_split_fibres`; the existing original-source 1,024 planted chain and Python full-cohort replay gates also passed. [Raw artifact #14567650828](https://github.com/isogeny-labs/crypto/actions/runs/44598103355/artifacts/14567650828), SHA-256 digest `b4248835df12f59cf62bc135d9b559541c4c1a30a77a2ca0043e32fcc5717c09`. The new `eliminated_horner_discriminant_and_split_fibres` unit test checks 8,192 deterministic uniform field pairs, 24,576 polynomial coefficient evaluations, all 31×31 public-base input pairs, exact discriminants, duplication identities, original-Arithmetic signed sums and exceptional-infinity counts. The local environment did not provide `cargo`/`rustc`; the source-specific GitHub CI compiled and exercised the existing original-Rust code. This is not independently signed PCC. [pcc:unverified]

## Prior-work check, competitive boundary and prioritized experiment queue

1. The previous 21-variable / 14-variable Horner charts and exact finite-field rational equivalence are in [#4812](https://github.com/isogeny-labs/crypto/pull/4812). This follow-up closes the algebraic-closure-root caveat for the **whole specialized chart**, not for isolated E factors. Duplication and Semaev F3 identities are standard mathematics, not new cryptanalytic attack primitives.
2. Public-only unknown-index extraction is already owned by [#4692](https://github.com/isogeny-labs/crypto/pull/4692), [#4822](https://github.com/isogeny-labs/crypto/pull/4822), [#4816](https://github.com/isogeny-labs/crypto/pull/4816); arithmetic rootfinders [#4809](https://github.com/isogeny-labs/crypto/pull/4809) and [#4826](https://github.com/isogeny-labs/crypto/pull/4826). Do not create duplicate F4 or rootfinding code. Prior prime-field Semaev approach: Petit, Kosters and Messeng, PKC 2016, DOI 10.1007/978-3-662-49387-8_1. Shared KB server was not accessible through the connected tools in this run; this is a bounded GitHub/publication audit, not a novelty certification.
3. **First next experiment:** In #4692, run *public-only* 14-var and 21-var F4 baselines on the already frozen 46-positive /18-negative B8 corpus. Count Macaulay fill, degree, memory, zero-dimensional roots, false negatives and every failed query; use only Q and public base, not planted witnesses or generic EC-sum tables. Treat all algebraic root candidates as ordinary Fp after verifying all H_i and E_i, but **do not use generic enumeration as an extractor**.
4. **Discriminating secondary experiment:** Compare F4 with/without redundant geometric saturation of the seven D(z) denominators on the same public instances. The theorem predicts identical affine zero sets; a significant F4 difference would be an implementation cost effect, not a change in correctness. Keep all source hashes and operand counters.
5. **Whole-solver gate:** Only if public B8 extraction works under a bounded budget, integrate in the original rank-31/1,024-Q Rust pipeline [#4728](https://github.com/isogeny-labs/crypto/pull/4728), and compare full setup+misses+linear algebra+per-target recovery against the fastest same-host existing IC and shared BSGS, plus optimized rho. [#4714](https://github.com/isogeny-labs/crypto/pull/4714) gives matched complete W8/M6 negative controls, and [merged #4532](https://github.com/isogeny-labs/crypto/pull/4532) recorded a 170.113 ms cold 1,024-Q BSGS result in a separate run. The latter is a historical reference, **not** a paired result against this new code.

**Status:** Exact algebraic splitting result, exploratory independently authored arithmetic checks and **passing exact-source original-Rust CI**. There is no public-only unknown-index extraction, no complete new ECDLP solving benchmark and no speedup measurement associated with this result. [pcc:unverified]