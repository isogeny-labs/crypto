# CT220 — exact cubic-cover point audit and prime-order Jacobian obstruction

2026-11-18 · existing [crypto PR #4715](https://github.com/isogeny-labs/crypto/pull/4715) · **[pcc:unverified]**

## The incompatibility to test before porting the genus-4 performance win

The concurrently measured [genus-4 index-calculus lane #4693](https://github.com/isogeny-labs/crypto/pull/4693) includes particularly favorable **prime-order Jacobians**. The CT216–219 DLP-preserving cubic-cover construction is a **split-Jacobian** situation. Those are incompatible families at field sizes relevant to this work. A cost advantage established on prime-order genus-4 Jacobians is therefore *not* transferable by simply inserting the elliptic subgroup into that benchmark population.

**Proposition (conditional on the existing rational isogeny factor).** Let `q>=5`, let `C/F_q` be a smooth curve of genus `g>=2`, and suppose `J_C` is `F_q`-isogenous to `E x A` with `E/F_q` an elliptic curve of `#E(F_q)>1` and `dim A=g-1>=1`. Then `#J_C(F_q)=#E(F_q)*#A(F_q)` is composite.

**Proof.** Isogenous abelian varieties over a finite field have the same zeta-function numerator (Frobenius characteristic polynomial), and thus the same number of rational points. The Weil bound for an abelian variety of dimension `d` gives `#A(F_q)>=(sqrt(q)-1)^(2*d)`. For `q>=5` this lower bound is strictly greater than one; hence the positive integer `#A(F_q)>=2`. Both factors in the cardinality product exceed one. This does **not** obstruct a source-subgroup DLP transfer, composite-order index calculus, or a potential complete-cost gain; it rules out treating the relevant destination as a prime-order Jacobian.

On the **existing** `F_101` cubic cover
`C: y²=u⁹+u³+46 -> E: y²=z³+z+46, z=u³`, CT217 already established the pullback/norm composite `beta*alpha=[3]`. Since `#E(F_101)=109`, `73*3=219=1 (mod 109)`, the operator `e=alpha*[73]*beta` is a genuine projector on `J_C(F_101)`. The presence of the `E` factor is part of the earlier CT217 evidence, not established by this standalone point census. The complementary dimension is three. Here `sqrt(101)>10`, so `#A(F_101)>(10-1)^6=531441`, meaning `#A>=531442` and `#J_C(F_101)>=57,927,178`. An exact Jacobian cardinality is **not** claimed.

## Independent finite rational-point census

The new standalone implementation `ct220_split_prime_order_obstruction.py` counts points without importing CT216/CT219's Cantor arithmetic:

- `#E(F_101)=109`: 108 affine points plus infinity; independent repeated EC addition verifies that `G=(2,37)` traverses all 109.
- `#C(F_101)=109`: 108 affine points plus infinity. Since `gcd(3,100)=1`, `u -> u³` permutes `F_101`. The map `(u,y)->(u³,y)` is a **bijection on the rational-point sets**, despite being degree three as a morphism of curves.
- The 54 signed `u`-factor-base symbols in CT219 correspond one-to-one to *all* nonidentity source elliptic points modulo sign. Hence the toy finite factor base grows with the source group itself; CT219's full 54/54 relation rank does not support an asymptotic index-calculus improvement.
- Negative control: over `F_7`, `gcd(3,6)=3`, so cubing is not bijective; the F101 rational-point argument does not indiscriminately generalize.

The exact standalone command passed both Python and `python -O` locally:

```bash
python3 research/chained_jacobian_transfers_20261110/toy_dlp_ct204/ct220_split_prime_order_obstruction.py --self-test
python3 -O research/chained_jacobian_transfers_20261110/toy_dlp_ct204/ct220_split_prime_order_obstruction.py --self-test
python3 research/chained_jacobian_transfers_20261110/toy_dlp_ct204/ct220_split_prime_order_obstruction.py
```

Expected self-test: `CT220_SELFTEST_PASS 109 54 531442`. This is **locally checked algebra and finite enumeration**, not a Lean theorem, a native timing measurement, an independent PCC signature, or a claim about a growing-field family.

## Decision and next experiment

Keep #4715 as a correctness/negative-cost toy record. Do not apply the **prime-order** costs in #4693 directly to this split cover. Instead, reuse its native solver modules and test **composite-order genus-4 Jacobians with a verified source subgroup**, a correctly computed source-primary projector, actual degree-4 relation eligibility, rank, and complete cold single-target pricing against the strongest matched rho/BSGS/IC. The independently validated small-field CT219 solver is already **roughly 10,542x slower than Python direct BSGS**, and this structural finding adds no speedup.

The owner's highest-priority original 695/701-record `C169`, `C13a+C13b`, `D` and norm-point reuse tests remain unexecuted: original-model S3 HTTP 403 custody and the separate documented paid-budget STOP must not be bypassed. No new paid compute, merge or protected-ledger action occurred.

Sources reviewed before the change: [merged #3877](https://github.com/isogeny-labs/crypto/pull/3877), [retained-lift counter work #4215](https://github.com/isogeny-labs/crypto/pull/4215), [norm transfer #1523](https://github.com/isogeny-labs/cryptanalysis/pull/1523), [GPU IC #4178](https://github.com/isogeny-labs/crypto/pull/4178), [genus-4 source #4693](https://github.com/isogeny-labs/crypto/pull/4693), CT217/CT219 reports, and [CT201 complete cold adjudicator #1766](https://github.com/isogeny-labs/cryptanalysis/pull/1766). Repository `AGENTS.md` and current branch session log read. Shared Research-KB MCP and wrapper were unavailable in this runtime; no novelty/absence claim is inferred. Mathematical background: Weil's abelian-variety bounds, Poincaré complete reducibility, and Thériault's genus-3/4 index-calculus work (doi:10.1007/978-3-540-40061-5_5).
