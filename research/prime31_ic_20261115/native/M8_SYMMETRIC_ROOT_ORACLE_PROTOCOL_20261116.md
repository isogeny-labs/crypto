# EXP-R31-SYM8 — symmetric eight-leaf finite-field root-equivalence gate

**Preregistered 2026-11-16 UTC before implementation/CI.** This is a correctness/falsification experiment for the original unanchored prime31 m8 algebraic lead, **not** a new complete non-generic ECDLP solver. No paid compute, main rewrite, PCC/leaderboard promotion or merge.

## Frozen domain and prior leads

Same ordinary prime-order curve `p=2147483647; E:y²=x³+847514846x+2087444724; n=2147446831`, generator from `PRIME31`, factor base from `Base::build(PRIME31,6)`, B31 canonical distinct liftable abscissae. Use first B=4 and B=8 admitted abscissae only for the small-root equivalence cells. Review prior #4077 gamma/delta whole-call gate, #4188 m4 K/S3 quotient/GCD and 144-fold m8 orbit gate, merged #4373 censored ordered/chained m8 F4, merged #4374 complete APS methodology, open #4391 replicated complete Rust APS 384/384/16384 profile, merged #4228 and open #4349 selected-root certification. Existing 2016 PKM and 2018 APS (DOIs 10.1007/978-3-662-49387-8_1 and 10.1016/j.ffa.2018.01.009) make the Semaev and symmetric-variable idea prior art. The shared research-KB MCP and repository wrapper are unavailable in this connector runtime; GitHub primary files and publisher records are the bounded fallback.

## Algebraic hypothesis: finite-rational (not ideal) exactness

Let `H(T)=prod_(x in B)(T-x)`, and for a candidate **unordered multiset of 8 leaf abscissae** let `f(T)=prod_(j=1)^8 (T-x_j)`, monic. `f | H^8` is necessary and sufficient for its roots, with multiplicity, to belong to the finite rational base. This condition does NOT capture sign orientation or intermediate infinity branches.

Given `f`, reconstruct its roots by monic linear division using only public base abscissae (no public-target group sums or labelled log). Evaluate whether an affine target abscissa `x_R` or the identity can occur among signed eight-leaf sums using recursive Semaev S3 finite-root propagation:
- identity + x lifts to x,
- for distinct affine x,y, S3(x,y,z) gives both finite roots,
- for x=y, S3 degenerates to the finite doubling root **and identity** (the inverse-sign branch),
- use field modular square roots and inversions, reject every nonsquare/unaccounted zero degeneracy.

The recursion uses only field arithmetic in candidate discovery; the original affine group law and its y-values are forbidden there. An independent **test-only** verifier enumerates 2^8 signed EC additions on the same multiset and checks exactly whether R or -R (or identity) is reachable. It never supplies candidate roots to the algebraic recursion. This is a polynomial *evaluation and finite-root equivalence* pilot, **NOT** a nonenumerative method to find f from R: enumerating f is a generic search control and cannot be priced as a non-generic m8 oracle.

## Frozen cells and stop conditions

1. Exhaust the `binom(11,8)=165` nondecreasing B4 eight-multisets. For every f test four point targets: constructed all-positive signed sum, fixed public `[17]G`, fixed public `P[0]`, and identity. Four statuses each; all algebraic Boolean verdicts must agree exactly with independent full-point signed brute-force postcheck. Assert monic degree8, `f|H^8`, reconstruction exactly equals the multiset, and at least one exceptional identity-positive case.
2. B8: 128 deterministically sampled unique multiset index vectors using pinned seed `0x53594d3842303136` and a declared bounded sampler, same four targets. Assert no false positives or negatives; retain every exceptional case and target kind.
3. Tamper controls: one non-base x root in an 8-multiset f must be rejected by `f|H^8`, and a nonmonic or wrong-degree polynomial must be rejected. The forged f is not permitted to enter Semaev recursion.
4. Record source-bound CI, full cold elapsed and counts for these **correctness validator cells only**, separately labelled as *not an IC oracle benchmark*. No extrapolation to B31, where the count of x multisets is `binom(38,8)=48,903,492` before sign choices, nor to the total `31^8` ordered tensor. Do not estimate a non-generic speedup from this data.

**Pass:** complete exact agreement on all cells with tampering rejected. **Fail:** any signed/infinity mismatch, certificate or root reconstruction error; retain the first minimal counterexample and code head. A missing/matrix-censored actual `S9` elimination counts as still incomplete even if all finite-root evaluations pass.

## Next decisive mechanism after this correctness gate

Implement `S9` in elementary symmetric coefficients of `f`, or a certified target-conditioned divided-difference/subresultant circuit, **without candidate-multiset enumeration**, on B4/B8 first. Compare exact same original finite root sets and signed recovery with native F4, #4349 certified quotient and actual PKM/APS baselines on identical natural targets. Only then scale to B31 and independently verify a fully cold unknown scalar across heldouts. Optimized rho is separate whole-solver security control; never compare partial F4 time to full APS solving.
