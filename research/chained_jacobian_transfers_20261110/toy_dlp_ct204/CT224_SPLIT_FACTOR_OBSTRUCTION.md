# CT224 — the F101 genus-4 target splits again: two elliptic factors and a simple surface

**2026-11-19 UTC. Exact algebraic follow-up to CT221–223; [pcc:unverified].** Existing draft [crypto PR #4715](https://github.com/isogeny-labs/crypto/pull/4715). No new paid compute.

## Finding

CT221 calculated the complementary threefold numerator for
`C: y²=u⁹+u³+46 → E: y²=x³+x+46` over `F101`:

`L_A(T) = 1 + 296 T² + 29896 T⁴ + 1030301 T⁶`.

The polynomial **factors over the integers**:

`L_A(T) = (1+101 T²)(1+195 T²+10201 T⁴)`.

Thus, conditional on CT221's exact point census and its rational isogeny `J_C ~ E×A`, the Frobenius polynomial separates over `F101` into three mutually non-isogenous simple factors:

`J_C ~ E × E_0 × B` (isogeny over F101),

where `#E(F101)=109` (trace -7), `E_0` is an elliptic isogeny factor with trace 0 and `#E_0(F101)=102`, and `B` is an `F101`-simple dimension-two abelian variety with Weil numerator `1+195T²+10201T⁴`, hence `#B(F101)=10397=37·281`. These group orders multiply to `115593846`, matching CT221.

This does **not** establish that `E_0` and `B` arise as explicit geometric quotient curves of `C`; isogeny factors are enough for the following obstruction. An independent model of the elliptic isogeny class `E_0` is `y²=x³+1`: since cubing permutes `F101`, this model has 101 finite points and the identity, so its trace is 0.

## Why B is simple and why the cross-factor map vanishes

Write `P_B(X)=X⁴+195X²+101²`. A rational quadratic factorization with no linear terms would require roots of `z²−195z+101²`, whose discriminant is `195²−4·101²=−2779`, not a rational square. A rational quadratic factorization with opposite nonzero linear coefficients must have equal constants `b=±101`; their linear coefficient would satisfy `a²=2b−195`, namely `7` or `−397`, neither a rational square. Therefore `P_B` is irreducible in `Q[X]`. A degree-four irreducible Weil polynomial corresponds to an `F101`-simple abelian surface, not two rational elliptic factors.

The source elliptic Frobenius polynomial `X²+7X+101`, the trace-zero factor `X²+101`, and `P_B` are distinct irreducible polynomials. By the finite-field isogeny classification and rational Frobenius equivariance of homomorphisms, there is **no nonzero F101-defined abelian-variety homomorphism** from `E` to either `E_0` or `B`, and similarly between the other distinct factors.

Consequently, any multi-hop chain consisting of `F101`-defined group homomorphisms that starts in `E` and ends *entirely in the complementary E_0 or B factor* is zero. It cannot preserve a 109-order DLP. This statement does **not** prohibit the proven nonzero pullback `E→J_C`, a larger target containing the E-isotypic component, maps after a field extension with separately analyzed Frobenius, or index calculus on the full composite Jacobian. It also does not exclude a different genus-2 or genus-4 cover.

## Quadratic-extension no-transfer check

The same simple factor orders remain **pairwise coprime over F101²**. With `#E(F101²)=(101+1)²−(−7)²` and analogous formulas for the trace-zero factor, while `#B(F101²)=L_B(1)L_B(−1)`, the results are:

| Rational group over F101² | Exact order | Factorization |
| --- | ---: | --- |
| E | 10,355 | 5·19·109 |
| E_0 | 10,404 | 2²·3²·17² |
| B | 108,097,609 | 37²·281² |

All three pairwise gcds are 1 (checked in the executable). Even an **arbitrary abstract group homomorphism** from `E(F101²)` into `E_0(F101²)` or `B(F101²)` must be zero. Consequently, simply extending to F101² does not rescue the proposed transfer to the smaller factors. This does *not* establish the same conclusion for all extension degrees, nonlinear encodings, or a different cover. No extension-field Cantor homomorphism was implemented in this audit.

## Three mutually orthogonal CRT projectors on J_C(F101)

Because `109`, `102`, and `10397` are pairwise coprime, CRT supplies scalar idempotents on the **finite rational group** `J_C(F101)`:

| Factor order | Scalar | Residues modulo (109, 102, 10397) |
| --- | ---: | --- |
| 109 (E) | 80597544 | (1, 0, 0) |
| 102 (E_0) | 87262021 | (0, 1, 0) |
| 10397 (B) | 63328128 | (0, 0, 1) |

Their sum is 1 modulo 115593846, each is idempotent, and distinct scalar projectors annihilate one another. This refines the earlier `[80597544]` source projector without asserting that the abstract isogeny gives a canonical isomorphism of integral finite group schemes. The numerical CRT decomposition alone does not construct a faster projector than the CT222 norm map.

## Implementation and local checks

Source: [ct224_split_factor_obstruction.py](ct224_split_factor_obstruction.py), frozen [ct224_f101_split_factor.json](ct224_f101_split_factor.json).

```bash
python3 research/chained_jacobian_transfers_20261110/toy_dlp_ct204/ct224_split_factor_obstruction.py --self-test
python3 -O research/chained_jacobian_transfers_20261110/toy_dlp_ct204/ct224_split_factor_obstruction.py --self-test
python3 research/chained_jacobian_transfers_20261110/toy_dlp_ct204/ct224_split_factor_obstruction.py > /tmp/ct224.json
cmp /tmp/ct224.json research/chained_jacobian_transfers_20261110/toy_dlp_ct204/ct224_f101_split_factor.json
```

Producer-local Python 3 checks passed (normal, `-O`, compilation), yielding `CT224_SELFTEST_PASS [109, 102, 10397] [80597544, 87262021, 63328128] 3`. They independently recount rational points of `E`, `C`, and `E_0`, verify the integer factorization and irreducibility criterion, and reject three one-coefficient mutations plus a wrong scalar projector. The higher extension-field point counts and CT221's splitting premise are **inputs**, not newly proved here. Scope is exact toy arithmetic with no signed independent PCC.

## Impact on next experiments

1. **Stop proposing direct source-to-B or source-to-E_0 transfers over F101.** They must be zero; a smaller-factor index-calculus benchmark could not recover the 109-order source log.
2. Keep the only live toy path as a native composite-order `J_C` solver retaining the 109-primary component, with explicit offset relations and complete single-target cold timing against native rho/BSGS/IC. CT219's Python toy diagnostic already loses badly; do not duplicate it.
3. For a potentially better cover, preregister a source-preserving Frobenius/isogeny factor and measure whether the factor base is genuinely sublinear in the source order. The CT220 F101 cubic-permutation cover's 54 signed rational symbols exhaust the source group modulo sign and do not exhibit scalable sparsity.
4. The original 695/701 authenticated Jacobian fixture and actual `C169/C13a/C13b/D` norm-point reuse pricing are still blocked by S3 403/approved budget constraints. No synthetic fixture may stand in for its provenance.

## Sources reviewed

Repository CT220–223 proofs, #3877, #4215, cryptanalysis #1523, #4693, and [Diem's double-large-prime complexity result](https://arxiv.org/abs/math/0606607). The factorization and the application of finite-field Frobenius theory are elementary, not claimed as new mathematical algorithms. Research-KB MCP was unavailable; this is not an exhaustive novelty search.
