# Prime31 weighted-M6 formal collision quotient — frozen protocol

2026-11-18 UTC. This protocol is written before the new Rust source and GitHub Actions execution. Continuation of [#4722](https://github.com/isogeny-labs/crypto/pull/4722). **Scope: exact combinatorial support upper bounds**, not an algorithmic speedup or a new completed non-generic ECDLP solver. PCC producer-side until signed independent review.

## Question

The previous fixed-instance bound was `Pr[hit] <= B^6/n` for a uniform group residual, with `B=31`, `n=2147446831` and public scalar weights `(1,3,5,7,11,13)`. Does treating *unavoidable equal integer coefficient vectors* tighten the bound, and how much can changing only six public weights help before any actual EC collisions?

Use the **unchanged** ordinary prime-order cofactor-one original curve over `p=2147483647`, `a=847514846`, `b=2087444724`, native Rust `prime31_algebraic_ic::{Arithmetic,check_curve,PRIME31}`, first-rational-x base B31, and the original weights. Compare with two prospective subset-sum-injective weight controls `(1,2,4,8,16,32)` and `(1,3,7,15,31,63)`. These controls are NOT prespecified faster candidate solvers.

## Exact theorem to prove and implement

For each ordered word `i=(i_1,...,i_m)` in `{1,...,B}^m`, define the **integer formal signature** `c_t(i) = sum_{j:i_j=t} w_j`. Its EC sum is `sum_t [c_t]P_t`. Distinct formal signatures *may* collide in the EC group, but the converse is impossible.

For positive weights `w`, enumerate all **set partitions** of the `m` labeled slots. For each partition let `s` be the multiset of positive block-weight sums; deduplicate equal multisets for each support size `k`. If `mult_s(v)` counts repeated block sums, the exact number of formal signatures is

```
F_B(w) = sum_{k=1}^m sum_{s in distinct block-sum multisets of size k}
          (B)_k / product_v mult_s(v)!,
(B)_k = B * (B-1) * ... * (B-k+1), with zero if k>B.
```

Proof obligation: show every signature corresponds to exactly one distinct support subset plus an assignment of the coefficient multiset, irrespective of how many set partitions yield the same coefficient multiset. Thus `|support(E)| <= F_B(w) <= B^m`, and a uniform residual has `Pr[hit] <= min(1,F_B(w)/n)`.

## Frozen controls

1. Count the 203 set partitions of six labeled slots; record the per-k raw partition counts and distinct coefficient-sum patterns for all three weight choices.
2. Compute `F_B` for `B=2,3,4,8,16,31` via the formula with checked `u128` arithmetic.
3. Independently brute-force all `B^6` index words for B2, B3, B4 and B8, comparing **integer coefficient vectors**, not EC keys. This is an oracle-correctness-only control; not a scalable relation extractor.
4. Use the existing original-curve Rust verifier; reconstruct the B31 canonical first-rational-x points; verify the explicitly planted collision of words `(0,1,1,0,2,3)` and `(1,0,0,1,2,3)` under the original weights by native EC additions/scalar multiplication. A separately implemented Python BigInt group law must replay this.
5. Test the strictness of `F_{31}(w) <= 31^6` for the original weights and equality for both subset-sum-injective controls; independently assert the control weights have distinct subset sums.
6. Never claim a win over BSGS/rho/IC, a new relation, a measured solver runtime or a non-generic extractor from these combinatorial counts. Report actual group support as **unknown**.

## Leads already searched / no-duplication gate

Earlier [#4722](https://github.com/isogeny-labs/crypto/pull/4722) proves the looser word-count ceiling; [#4714](https://github.com/isogeny-labs/crypto/pull/4714) measured complete Rust M6 against shared BSGS and mixed IC and lost; [#4503](https://github.com/isogeny-labs/crypto/pull/4503) has related formal signed mixed-base support; [#4692](https://github.com/isogeny-labs/crypto/pull/4692) pursues nonenumerative RR9/F4; [#4717](https://github.com/isogeny-labs/crypto/pull/4717) examines coefficient multiplicity strata; [#4720](https://github.com/isogeny-labs/crypto/pull/4720) has signed-pair screen. Relevant baseline source: [Yokoyama–Yasuda–Takahashi–Kogure (2020)](https://doi.org/10.1515/jmc-2019-0029), whose conditional complexity analysis cautions against claiming naive Semaev index calculus beats generic ECDLP. The research-KB MCP and Conductor were not available; GitHub file/PR queries and the journal article are the bounded fallback. This elementary partition counting is not asserted as novel mathematical literature.

**Decision gate:** If the revised bound barely improves the old 41.33% ceiling, stop M6 fixed-weight tuning. The next high-value work is a genuinely target-conditioned algebraic relation oracle with no 3+3/4+4 group tables, then rank 31 and 1,024 blinded logs with complete costs against strongest matched IC/BSGS/rho.
