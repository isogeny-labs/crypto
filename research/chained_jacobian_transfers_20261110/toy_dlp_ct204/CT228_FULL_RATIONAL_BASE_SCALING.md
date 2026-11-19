# CT228 — full rational factor base is a scaling dead end for the fixed cubic-cover family

2026-11-19 · continuation of [crypto PR #4715](https://github.com/isogeny-labs/crypto/pull/4715) after the **negative** CT227 native cold benchmark. **[pcc:unverified]**. This is a mathematical and bounded-census finding, **not** a complete ECDLP measurement or a new attack.

## Exact scoped proposition

Fix the nonsingular elliptic curve \(E_p:y^2=x^3+x+46\) and genus-four curve \(C_p:y^2=u^9+u^3+46\) over an odd prime \(p\equiv2\pmod3\), excluding singular characteristics. The rational map \(\pi(u,y)=(u^3,y)\) bijects \(C_p(\mathbb F_p)\) with \(E_p(\mathbb F_p)\) **as point sets** (not as groups). Indeed \(\gcd(3,p-1)=1\), so cubing permutes \(\mathbb F_p\), and each curve has one rational point at infinity.

Write \(N_p=\#E_p(\mathbb F_p)\) and \(W_p=\#\{x\in\mathbb F_p:x^3+x+46=0\}\), with \(0\le W_p\le3\). The **full rational degree-one signed factor base** (affine points modulo hyperelliptic involution \((u,y)\mapsto(u,-y)\)) has exactly
\[
B_p=\frac{N_p-1+W_p}{2}.
\]
Hasse gives \(N_p\ge p+1-\lfloor2\sqrt p\rfloor\), hence
\[
B_p\ge\frac{p-\lfloor2\sqrt p\rfloor}{2}=\Theta(p).
\]
For a CT226-style linear system that explicitly assigns one formal unknown to **every** such signed factor-base class and insists on full column rank, at least \(B_p\) independent equations are necessary. Collecting/processing that many equations already costs \(\Omega(p)\) elementary operations in this *specified model*, versus the \(O(\sqrt r)\) generic rho reference for a source subgroup of size \(r\asymp p\). Eliminating known two-torsion symbols when \(W_p>0\) can save at most three columns, not the growth order. This is not a lower bound on methods with a smaller factor base, relation hypergraphs/large primes, compressed representations, alternative covers, amortization, or non-generic mechanisms.

Importantly, \(\pi\) being bijective on rational **points** does not make the genus-4 Jacobian isomorphic to \(E\). CT224/CT225 established the complementary-factor obstructions for the original F101 member; this claim concerns a different (factor-base size) bottleneck.

## Executable finite checks

[ct228_full_rational_base_scaling.py](ct228_full_rational_base_scaling.py) independently enumerates both affine curves, checks the cubic permutation, nonsingularity, exact source order, signed-point orbit counts and Hasse bound. It rejects malformed/nonadmissible inputs. The nine measured prime-order instances have no rational Weierstrass point:

| \(p\) | \(\#E_p(\mathbb F_p)\) (prime) | Full signed basis \(B_p\) |
|---:|---:|---:|
| 29 | 23 | 11 |
| 41 | 47 | 23 |
| 59 | 53 | 26 |
| 101 | 109 | 54 |
| 137 | 157 | 78 |
| 149 | 163 | 81 |
| 1019 | 1013 | 506 |
| 1427 | 1433 | 716 |
| 4871 | 4919 | 2459 |

At the checked **31-bit** prime \(p=2,147,483,579\equiv2\pmod3\), Hasse alone forces \(B_p\ge1,073,695,449\) symbols. Materializing even one 32-bit word per symbol would require at least **4,294,781,796 bytes**, before indices, matrix, relation collection, transfers, memory allocator or target recovery. This is a storage implication **only of the explicitly materialized full-base implementation**, not a memory lower bound for arbitrary ECDLP solvers. No \(\#E_p\) calculation or timing was attempted at this prime. The NIST-like \(2^{31}-1\) benchmark prime is \(1\pmod3\), so the cubic-bijection statement **does not apply to it**.

Local reproducer (ordinary Python, no native build or paid compute):
```sh
python3 research/chained_jacobian_transfers_20261110/toy_dlp_ct204/ct228_full_rational_base_scaling.py --self-test
python3 -O research/chained_jacobian_transfers_20261110/toy_dlp_ct204/ct228_full_rational_base_scaling.py --self-test
python3 research/chained_jacobian_transfers_20261110/toy_dlp_ct204/ct228_full_rational_base_scaling.py > /tmp/ct228.json
sha256sum /tmp/ct228.json
# CT228_SELFTEST_PASS 9 54 in both modes
# canonical JSON SHA-256: 8ed4d9cfdcb3cc5ac21a5a45b02f70e1655f3c32dd2d84a8800d1bdf6281b131
```

The exact enumeration was executed locally under both normal Python and \(-O\); source publication and hosted CI, if any, must be checked separately. These counts are independent of CT226's preassigned 54 variables and confirm that its F101 factor base exhausts the rational-point orbits. They do **not** independently reproduce its native cold timing.

## Cross-repository coordination (avoid duplicating genuine norm work)

The original norm-point-reuse implementation and 695-record finite parity **already exist** as unsigned native evidence in [cryptanalysis's published result](https://github.com/isogeny-labs/cryptanalysis/blob/main/experiments/chained-transfer-public-target/reports/norm-point-reuse-result-01/REPORT.md), not merely an unimplemented prediction. What is blocked is **independent new source/custody replay** and qualified physical costs. [CT198](https://github.com/isogeny-labs/cryptanalysis/pull/1748) also shows that the frozen strongest direct degree-169 and retained 13×13 routes have identical norm-doubling counts, so a reuse saving against the *literal sequential* route cannot be credited as saving against the best direct route. [CT191–197](https://github.com/isogeny-labs/cryptanalysis/pull/1735) already implement source-guarded all-48 actual norm operands and prospective pricing/reader gates. [CT201/202](https://github.com/isogeny-labs/cryptanalysis/pull/1766) owns the full cold source/rho/BSGS/IC adjudicator. Do not build a duplicate price harness or fabricate the S3-blocked original source.

## Performance decision and next falsifiable experiment

CT227 measured 26,020× toy slowdown in a narrow unisolated BSGS comparison, with the **unchanged non-relation phases** already 274× slower. This finite native result plus the new full-base \(\Theta(p)\) obstruction argues against spending another cycle optimizing only CT226's fixed full rational base. A defensible next *new* experiment must deliberately shrink/alter the factor base and instrument its smooth yield, rank closure and full cold single-target recovery, or change the transfer family. It must use the existing Rust arithmetic, register a dimension/curve/target panel **before measurements**, compare to strongest same-host rho/BSGS/existing IC, record failures, peak RSS, cold setup, maps, relation cost and LA, and report uncertainty and independent/PCC scope. Do not infer a speedup from a smaller matrix alone.

Until authenticated original source access and approved execution exist, the **highest-priority norm pricing work remains the already allocated CT192/197/201 lanes**, not a duplicate CT228 benchmark. No paid EKS/Spot computation, protected merge, signed PCC or claim of a complete single-target advantage is made.

## Sources/prior art and limits

Checked repo-specific CT220, CT226/227, original crypto [#3877](https://github.com/isogeny-labs/crypto/pull/3877), [#4215](https://github.com/isogeny-labs/crypto/pull/4215), cryptanalysis [#1523](https://github.com/isogeny-labs/cryptanalysis/pull/1523), [#1748](https://github.com/isogeny-labs/cryptanalysis/pull/1748) and [#1766](https://github.com/isogeny-labs/cryptanalysis/pull/1766), and GPU [#4178](https://github.com/isogeny-labs/crypto/pull/4178). For fixed genus and variable field size, cf. [Diem, double-large-prime index calculus](https://arxiv.org/abs/math/0606607), which motivates **smaller** factor bases; high-genus subexponential results such as [Enge–Gaudry–Thomé](https://arxiv.org/abs/0905.2177) have different asymptotic regimes. Research-KB MCP and checkout-local catalog were not exposed in this runtime; there is no priority/novelty claim. [pcc:unverified]
