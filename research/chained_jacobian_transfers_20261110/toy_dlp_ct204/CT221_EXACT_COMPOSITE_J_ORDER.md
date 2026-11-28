# CT221 — exact genus-4 split-cover Jacobian order and source-primary projector

**Date:** 2026-11-18. Continues [crypto PR #4715](https://github.com/isogeny-labs/crypto/pull/4715) after [CT220](CT220_SPLIT_PRIME_ORDER_OBSTRUCTION.md). **[pcc:unverified]**.

## New exact mathematical result

For the existing public toy cover
\[
C/\mathbf F_{101}:\ y^2=u^9+u^3+46,\quad
\pi(u,y)=(u^3,y),\quad
E/\mathbf F_{101}:\ y^2=x^3+x+46
\]
the genus-4 Jacobian has **exact** cardinality
\[
\boxed{\#J_C(\mathbf F_{101})=115\,593\,846
       =2\cdot3\cdot17\cdot37\cdot109\cdot281.}
\]
The source group has \(\#E(\mathbf F_{101})=109\), and the complementary genus-3 isogeny factor has
\[
\#A(\mathbf F_{101})=1\,060\,494=2\cdot3\cdot17\cdot37\cdot281.
\]
This replaces CT220's lower bound for *this* curve with an exact zeta-function-derived order. In particular, the **109-Sylow subgroup of \(J_C(\mathbf F_{101})\) has order exactly 109**: there is no other 109 factor in the rational Jacobian. This is not a claim about a general family or a complexity speedup.

## Derivation and proof obligations

The established degree-three separable map provides rational norm and pullback maps \(\beta=\pi_*\), \(\alpha=\pi^*\) with \(\beta\alpha=[3]\). Hence the elliptic factor divides the Jacobian zeta numerator: \(L_C(T)=L_E(T)L_A(T)\), where \(A\) has dimension three. The existing CT220 census proves \(\#E(\mathbf F_{101})=\#C(\mathbf F_{101})=109\).

Since \(101\equiv-1\bmod3\), the cube map is bijective on \(\mathbf F_{101^n}\) for **every odd \(n\)**. It follows without enumerating the million-element cubic extension that \(\#C(\mathbf F_{101^n})=\#E(\mathbf F_{101^n})\) for \(n=1,3\). Thus the complementary Frobenius power sums are \(S_1(A)=S_3(A)=0\).

A new, standalone \(\mathbf F_{101^2}\) census (two separate nonsquare models \(X^2=2\) and \(X^2=3\), each counted by both direct square multiplicities and independent Euler-character evaluation) obtains:

| Curve | \(\#(\mathbf F_{101^2})\) |
| --- | ---: |
| \(E\) | 10,355 |
| \(C\) | 10,947 |

Hence \(S_2(A)=\#E(\mathbf F_{101^2})-\#C(\mathbf F_{101^2})=-592\). Newton identities give \(a_1=0\), \(a_2=-S_2/2=296\), \(a_3=0\); the genus-three Weil functional equation supplies the remaining coefficients:
\[
L_A(T)=1+296T^2+29\,896T^4+1\,030\,301T^6.
\]
The source trace is \(101+1-109=-7\), so \(L_E(T)=1+7T+101T^2\), and
\[
L_C(T)=1+7T+397T^2+2072T^3+59\,792T^4
+209\,272T^5+4\,049\,797T^6+7\,212\,107T^7
+104\,060\,401T^8.
\]
Evaluating these L-polynomials at \(T=1\) gives the exact point counts above. The implementation checks the genus-four reciprocal coefficient relations and factors the integer with a small exact trial-prime certificate.

## Independent source-primary projection

Write \(h=\#A(\mathbf F_{101})=1\,060\,494\). Since \(h\bmod109=33\) and \(33^{-1}\bmod109=76\), **the scalar \([80\,597\,544]=[76h]\)** is an exact idempotent projector on \(J_C(\mathbf F_{101})\): it is identity on \(\alpha(E(\mathbf F_{101}))\) and zero on \(\ker\beta\). It is equal as a rational-group endomorphism to CT217's norm/pullback projector \(\alpha[73]\beta\). Proof: CT217 establishes \(\beta\alpha=[3]\), making \(\beta\) surjective on \(E(\mathbf F_{101})\); its kernel has order \(h\). Lagrange gives \([h]\ker\beta=0\), and \(76h\equiv1\bmod109\). The pair of direct summands determines the projector uniquely. The integer identity \((76h)^2\equiv76h\bmod(109h)\) is checked independently in the script.

This **does not make the cofactor projector free**. Executing it involves a large Jacobian scalar multiplication, whereas the norm projector uses its own nonzero transfer and arithmetic costs. Either variant must be priced in the cold single-target ledger, not inferred to improve performance from its existence.

## Reproduction / validation

Script: [ct221_zeta.py](ct221_zeta.py).

~~~bash
python3 research/chained_jacobian_transfers_20261110/toy_dlp_ct204/ct221_zeta.py --self-test
python3 -O research/chained_jacobian_transfers_20261110/toy_dlp_ct204/ct221_zeta.py --self-test
python3 research/chained_jacobian_transfers_20261110/toy_dlp_ct204/ct221_zeta.py
~~~

Both self-tests were executed locally and returned **CT221_SELFTEST_PASS 115593846 1**. The script fails closed on reducible quadratic-extension models, mismatched square-table/Euler counts, mismatched models, zeta functional equation errors, factor errors, and invalid CRT projection. These methods share the same explicit curve definition and are not a second-host signature; **independent PCC remains outstanding**. Scoped CI must be inspected at its exact commit SHA before asserting a hosted pass.

Sources checked: existing [CT217](CT217_NORM_PROJECTOR_RANK_AND_COST.md), [CT219](CT219_ANCHOR_FREE_PAIRED_NEGATIVE.md), [CT220](CT220_SPLIT_PRIME_ORDER_OBSTRUCTION.md), [crypto #4693](https://github.com/isogeny-labs/crypto/pull/4693); Weil functional equation and genus-three coefficient shape in [MIT mathematical notes](https://math.mit.edu/~drew/Definitions.html), and [Harvey trace/zeta reference (2022)](https://link.springer.com/article/10.1007/s40993-022-00398-7). Research-KB MCP was not loaded in this runtime; the cross-repo review was via GitHub source/PRs, not a full knowledge-base search. No novelty claim.

## Next concrete experiment and limits

This exact \(\#J\), unique 109-primary factor, and independent scalar projector can be used to sanity-check **composite-order native genus-4** subgroup arithmetic and source projection before moving to a qualified complete-solve benchmark. Check divisor representatives in the existing Cantor code against both projections and the exact group-order annihilator, then port the bounded native relation/LA/target-recovery arm **without** borrowing the prime-order Jacobian cost coefficients of #4693. Reuse existing frozen baselines and cost adjudicators rather than cloning another competing lane.

The previously measured CT215 genus-2 generic BSGS transfer was **61.73× slower**, and CT219's complete Python toy IC was approximately **10,542× slower** than Python direct BSGS (unisolated, producer-only). This exact order computation contains **no physical timings** and does not change those negative results. The original 695/701 fixture remains inaccessible under the documented S3 HTTP 403 and paid STOP; do not bypass custody or budget. \(C_{169}\), \(C_{13a}\), \(C_{13b}\), \(D\), norm-point reuse, strongest matched cold rho/BSGS/IC, memory/CI and signed PCC remain open.

No paid compute, merge or claim of a complete single-target ECDLP advantage. Keep #4715 draft, **[pcc:unverified]**.
