# CT59 — Single-oriented-point Frobenius gate for the CT55 binary target

**Scientific status:** [pcc:unverified]. This is a scoped application of the standard Frobenius equation, not a new ECDLP attack or a uniform impossibility theorem. It strengthens the single-point cost gate for the **fixed** binary model `E0: y²+y=x³` without contradicting the CT55 existence construction.

## Theorem and proof

Let `ell` be an odd prime, let `F` be Frobenius of `E0/F2`, and write `m=ord_ell(-2)`. The smallest `f` such that **some** nonzero point of `E0[ell]` is rational over `F_(2^f)` equals

    f_single = m if m is odd; 2m if m is even.

The smallest `f` such that **all** of `E0[ell]` is rational is `f_full=2m`.

**Proof.** Because `#E0(F2)=3`, the trace is zero, so Cayley–Hamilton implies `F²=[-2]` on odd torsion. For even `f=2k`, `F^f=[(-2)^k]`; a nonzero point is fixed precisely when `m|k`. For odd `f=2k+1`, the operator `F^f=(-2)^k F` has trace zero and determinant `2^f`, hence `det(I-F^f)=1+2^f`. Its kernel is nonzero iff `2^f=-1 (mod ell)`, equivalent, for odd `f`, to `(-2)^f=1`, or `m|f`. Thus if `m` is odd the first admissible degree is `f=m`; if `m` is even no odd degree qualifies and the first even degree is `f=2m`. If `m` is odd, `F^m` has distinct eigenvalues +1 and -1, so it fixes one line but not the full basis. The first full-basis identity is always `F^(2m)`. The characteristic polynomial has discriminant -8, invertible modulo odd `ell`, so a cyclic matrix model also exists. QED.

**Global consequence.** Let `Etilde/Q` have good reduction at 2 isomorphic to `E0/F2`. Let `T∈Etilde(L)` be any nonzero point of odd prime order `ell` over **any** number field `L`, with a place `w|2` of residue degree `f`. The prime-to-characteristic reduction of `T` remains nonzero, so `f_single|f`. Since `f≤[L:Q]`, it follows that

    [L:Q] ≥ f_single.

This applies to the **one oriented point** route even when the curve and number field depend on the source ECDLP instance. For non-Galois `L`, do **not** assert `f|[L:Q]`; only the inequality is universal. The result is NOT a lower bound on the running time of every compressed/symbolic representation.

## Explicit tested bounds

| Prime `ell` | `ord_ell(-2)` | Single-point minimum `f` | Full-basis minimum `f` |
|---:|---:|---:|---:|
| 5 | 4 | 8 | 8 |
| 11 | 5 | 5 | 10 |
| 43 | 7 | 7 | 14 |
| 163 | 81 | 81 | 162 |
| 521 | 260 | 520 | 520 |
| 65537 | 32 | 64 | 64 |
| 2147483647 | 62 | 124 | 124 |
| 4294967291 | 2147483645 | 2147483645 | 4294967290 |

The 32-bit prime `4294967291` forces binary residue degree **2,147,483,645** for even a single nonzero torsion point on this specific supersingular target, and hence the same necessary number-field degree. In contrast `2147483647` requires only 124. This demonstrates the large dependence on the specific subgroup order; no distribution/asymptotic theorem is inferred.

All test inputs are prime by trial division; modular-order minimality is certified by `pow(-2,m,ell)=1` and `pow(-2,m/r,ell) != 1` for each prime `r|m`. For `4294967291`, `m=(ell-1)/2=5·19·22605091`. The need to represent a dense binary-field element of degree >2 billion makes that **particular** representation prohibitive, but there is no claimed common-unit comparison to optimized Pollard rho, nor exclusion of succinct representations or alternate binary targets/higher-genus Jacobians.

**Falsifier for over-generalization.** The alternate binary twist `y²+y=x³+x` has five points over `F2`, hence rational order-five torsion already over degree one. Therefore the `ell=5, f_single=8` gate is **not** valid for all characteristic-two curves.

## Reproduction / independence

```sh
python3 -B research/cross_characteristic_global_torsion_20261112/ct59/ct59_single_point_degree.py
python3 -O -B research/cross_characteristic_global_torsion_20261112/ct59/ct59_single_point_degree.py
node research/cross_characteristic_global_torsion_20261112/ct59/ct59_independent.js
node --no-opt research/cross_characteristic_global_torsion_20261112/ct59/ct59_independent.js
```

Python uses exact modular order plus an **independent point-count recurrence** for `#E0(F_(2^f))` at 25 small prime levels. JavaScript independently steps Frobenius's 2x2 matrix, testing first nontrivial fixed vector and first identity, then uses BigInt modular-order certificates for larger primes. All 28 prime test inputs produce 29 output lines; four local modes and cross-language stdout matched bytewise, SHA256 `7a0cfb903ac5ac3501c0dee186019648b3e6c02f01e178e11dacacd3ef2444ed`. This does **not** establish a signed scientific PCC verification.

## CI defect uncovered in CT57

The prior [CT58 exact-head CI run 43355663994](https://github.com/isogeny-labs/crypto/actions/runs/43355663994) failed at CT57's `cmp ct57-python.txt ct57-js.txt`, line five. The four numeric lines match; the two independently written fifth-line descriptions intentionally differ. The workflow now compares **all four numerical rows** while checking each distinct final sentinel. CT58 was skipped due to the preceding failure, not because its mathematical checks failed. CT59 adds a bounded audit step. Require fresh exact-head CI to determine success.

## Prior sources, search limits, and next gate

- Local [CT55 construction](../ct55/CT55_UNIVERSAL_DIVISION_FIELD.md), [CT58 full-basis theorem](../ct58/CT58_CYCLOTOMIC_FULL_TORSION_GATE.md), [second-prime problem](../../../problems/lift-reduce-second-prime-transfer.md), root PR #3877. The existing CT55 extension-degree table is reused; no first-discovery claim.
- Washington, *Elliptic Curves: Number Theory and Cryptography*, 2nd ed., §10.2, trace-zero example: https://www.oreilly.com/library/view/elliptic-curves-2nd/9781420071474/chapter-75.html .
- Stanford PBC lecture, elliptic Frobenius characteristic polynomial and Hasse–Weil recurrence: https://crypto.stanford.edu/pbc/notes/elliptic/count.html .
- Shared research-KB MCP was unavailable; no local repository checkout to run `scripts/research-kb` or `conductor check`. GitHub fallback searched `crypto`, `cryptanalysis`, `crypto-autoresearcher`. Existing same-characteristic consumer PR #1520 was not modified.
- Next: vary ordinary binary destination / genus and measure individual torsion Frobenius orbit bounds; independently tackle uniform global encoded-input point lifting and complete transfer-plus-solving cost. **No full-cost speedup demonstrated.**
