# CT334 — explicit Prime31 genus-two curves and quadratic-twist Richelot-kernel gate

**2026-11-23 UTC. Scope: executed finite Python source-only construction; [pcc:unverified].** This continues CT333/A1 under draft PR #4997. It is *not* an implemented Richelot isogeny, a descended morphism, a norm-point reuse improvement, an index-calculus solver, a speedup, or a signed PCC verdict.

## Mathematical consequence: twist invariance at two-torsion

Let k have odd characteristic, f in k[x] be separable of degree six, J=Jac(y²=f(x)), and d be a nonsquare. The quadratic twist J^(d)=Jac(y²=d*f(x)) has a Galois action on its geometric torsion differing by the quadratic character times [-1]. Since [-1] acts trivially on J[2], the 2-torsion Galois modules of J and J^(d) are isomorphic. The Weil pairing is respected, so **Frobenius-stable maximal isotropic subgroups and their minimal extension degrees match exactly**. This does not imply equality of the full Frobenius polynomials, rational Jacobian group orders, isogeny codomains or DLP costs.

The six branch-root factor degrees of the squarefree sextic determine Frobenius's cycle partition on its geometric Weierstrass points. The fifteen perfect matchings represent its geometric Richelot kernels. A subgroup *scheme* is rational if Frobenius fixes the unordered triple of pairs; the individual pair classes need not be rational. This is established standard genus-two 2-torsion mathematics, **not claimed as a new theorem or as independent formal verification**.

## Executed source-only fixture construction

For the specific prime **p=2^31-1=2147483647**, a deterministically bounded source constructed **eleven monic, squarefree degree-six polynomials**, one per factor-degree partition of six. The corresponding equations y²=f(x) are smooth genus-two curves. It recorded each original model and its nonsquare (-1) twist y²=-f(x), **22 explicit equations** in total. Every selected factor passed both Rabin's prime-divisor Frobenius criterion and the all-lower-k Frobenius-gcd criterion, using shared modular-polynomial arithmetic, and every degree-six product passed gcd(f,f')=1. These are producer-generated checks, not a second independent implementation. [pcc:unverified]

| Frobenius factor degrees | Rational Richelot kernels | Min extension degree admitting one |
|---|---:|---:|
| 1+1+1+1+1+1 | 15 | 1 |
| 1+1+1+1+2 | 3 | 1 |
| 1+1+1+3 | 0 | 3 |
| 1+1+2+2 | 3 | 1 |
| 1+1+4 | 1 | 1 |
| 1+2+3 | 0 | 3 |
| 1+5 | 0 | 5 |
| 2+2+2 | 7 | 1 |
| 2+4 | 1 | 1 |
| 3+3 | 3 | 1 |
| 6 | 1 | 1 |

These count abstract rational subgroup-schemes *realized on the constructed smooth Jacobians* by their branch-root Galois action. They do not count actual successful Richelot formulas, smooth target Jacobians, or DLP-preserving transfer maps. Each curve's nonsquare quadratic twist has the same counts.

Examples (coefficients in Fp):
- 1+5: f=(x-1)(x^5+x+3)=x^6-x^5+x²+2x-3; **no rational first-step Richelot kernel**, even after quadratic twist. The factor x^5+x+3 passed the two source-bound irreducibility criteria.
- 3+3: f=(x³+x+4)(x³+x+13); **three rational subgroup-schemes**, despite requiring Frobenius-permuted quadratic factors rather than three individually rational pairs.
- 6: f=x^6+x+3 is irreducible; it has **one** rational subgroup-scheme, but not a certified nonsingular codomain or a cost advantage.

The stronger CT331 irreducible-2-torsion all-length obstruction applies only where its hypotheses and *descent of the composite morphism* are actually satisfied, e.g. the 1+5 type. CT333/A1 independently supplied the reference 11-type counts (but shares the mathematical construction framework, so agreement is **not** signed independent PCC).

## Source custody and actual commands

Preregistered protocol [CT334_PROTOCOL.md](CT334_PROTOCOL.md) committed **6ec055494f2fbcea9a3fa4cb33ac32dd3dbdb9ce** before source execution. Source [ct334_prime31_twist_models.py](ct334_prime31_twist_models.py) committed **ab80c3365dfbd29f097350d2419a6021b9a1ceee**, Git blob **b84283a9ac6c378a0edbfd324ee075ecb4437571**, SHA-256 **7cb577add4333c68ac82f078184d39a58f3832bd3144fde8be0f87cf6823064b**. The frozen JSON [CT334_RESULT.json](CT334_RESULT.json) committed **e563452b1696e5ef2b4f1c8f8545900fe5ec319f**, Git blob **01be355bf299c01419ac3a43a53adc4f8f7d61ea**, SHA-256 **efd8238fa60aa089d3c93bf75e7a4114b214ef58df18e749fff67fe584f5fda8**, 11,536 bytes. Local Git blob of the executed source equals the fetched PR blob; fetched JSON blob equals the local raw file, not merely the headline counts.

```sh
python3 -m py_compile ct334_prime31_twist_models.py
python3 ct334_prime31_twist_models.py > CT334_RESULT.json
python3 -O ct334_prime31_twist_models.py > CT334_RESULT.optimized.json
cmp CT334_RESULT.json CT334_RESULT.optimized.json
sha256sum ct334_prime31_twist_models.py CT334_RESULT.json
git hash-object ct334_prime31_twist_models.py CT334_RESULT.json
```

Both execution modes exited successfully and were byte-identical. Three *effective* malformed-data controls rejected a duplicate factor, a reducible quadratic factor, and a tampered Richelot fixed-kernel count. One valid polynomial control was accepted. The 11/11 count agreement and 22 model labels passed. No CLI timeouts, native Rust tests, field-element point arithmetic, CI or paid computation were executed. [pcc:unverified]

## Coordinator / worker integrity and next experimental gate

**One coordinator, zero native agent workers**: mathematics, local producer-source Python and raw-record checks all ran in the coordinator environment. The Rust norm-point reuse/Cantor and independent validator packages were deferred to their already assigned non-overlapping owners; no child-worker consensus, independent replay, formal Lean result or signed PCC is claimed. No benchmark reservation was consumed and no elapsed-time parallel gain is asserted.

Next: an independent finite-field/factorization implementation should regenerate the **exact 11 polynomials** from the frozen selector, verify the quadratic-twist 2-torsion isomorphism, and for any promising rational kernel construct and validate the *actual* Richelot codomain/morphism with singular-target guards. This is a prerequisite, not a substitute, for the reserved authenticated 695/701 Jacobian fixture, physical C169/C13a/C13b/N2_actual/D pricing, Cantor/divisor and public toy-DLP checks and complete same-host cold transfer/IC/rho/BSGS/memory comparisons. Paid STOP, EKS-only original workloads, locked source custody, CI/review limits and stacked #4990/#4992 gates remain unchanged. [pcc:unverified]

**Primary sources:** [Magma rational Richelot kernels](https://magma.maths.usyd.edu.au/magma/handbook/text/1615); [Bruin–Doerksen 2011](https://doi.org/10.4153/CJM-2011-039-3); [2-Selmer parity §1.2 and §5 on torsion/twists](https://doi.org/10.1112/plms.12565); [Dang, November 2026 (restricted superspecial sector)](https://arxiv.org/abs/2611.23694); internal CT331–CT333/A1 and #4992. KB/Conductor MCP and local research-KB wrapper were unavailable; no global novelty clearance attempted.
