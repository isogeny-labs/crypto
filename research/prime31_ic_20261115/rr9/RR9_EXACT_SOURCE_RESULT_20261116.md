# 2026-11-16 — RR9 original-curve determinant / first Hermite jet CI evidence

## What passed, and provenance

**Source-exact Rust CI run:** https://github.com/isogeny-labs/crypto/actions/runs/43983584099, job `133705933694`, source branch commit `7cc0be575fe62e51489f834b53a93516672d0345`. GitHub Actions pinned Ubuntu 24.04 + Rust 1.100.0, immutable merged existing prime31 native Rust tree `1df714645aa683cf975bc76fcacb9842cb4780fd`; only RR9 example installed from current branch. CI log shows native **40/40** library tests and RR9 **2/2** example tests passed. Complete JSON runner finished both actual factor bases:

| Factor base | distinct planted eight-leaf true residuals | distinct wrong-residual negative controls | successful draws |
|---|---:|---:|---:|
| First 31 rational x | 32/32 det=0 | 32/32 det !=0 | 32 |
| Actual depth-six Dickson 31-point base | 32/32 det=0 | 32/32 det !=0 | 32 |
| Total | 64/64 | 64/64 | 64 |

**Raw case artifact:** https://github.com/isogeny-labs/crypto/actions/runs/43983584099/artifacts/14284376610. Reported artifact ZIP sha256 `7e93434854e401b479c9fcf165dec7f83838fb898217934e7a5d4135b74eb58c` (GitHub Actions upload output). CI also asserted **for both bases** that a deliberately repeated signed leaf causes a false determinant zero with the naive 9×9 matrix on an incorrect target, while the newly installed **elliptic Hermite tangent derivative row** (on first duplicate, y != 0) yields a zero determinant on the true residual and a nonzero one on the wrong residual. Third occurrences and y=0 local-coordinate cases remain rejected. Group arithmetic used only to construct/check public fixtures outside the determinant.

The result is **producer CI [pcc:unverified]**, not independent signed PCC, not a new algorithmic ECDLP solution. It is a tested deterministic *predicate* for supplied points, not an extractor of unknown factor-base factors from an arbitrary public Q.

## Mathematical interpretation

For any nine pairwise distinct finite rational points, Riemann–Roch space `L(9O)` has basis `1,x,y,x²,xy,x³,x²y,x⁴,x³y`. Matrix determinant zero is equivalent to a nonzero function with zeros at the nine points and pole order at most nine at O. Its divisor must be exactly `Z_1+...+Z_9-9O`, principal iff `sum Z_i=O`. The Hermite order-one jet at a doubled point uses uniformizer `x-x(P)`, `D_x(y)=(3x²+a)/(2y)`; on a length-nine divisor, a zero determinant means a function vanishes with prescribed multiplicities, restoring the same principal-divisor criterion. These are classical theorems, not novel.

**New lead:** use an original-curve-correct compact degree-nine determinant rather than only expanded iterated Semaev resultants as the *equation source* for a target-conditioned signed `S9` ideal with factor-base polynomial `H`. Next must eliminate all leaf coordinates to obtain a factor polynomial `f(T)` from the unknown public residual alone. A determinant check on known factor-base leaves remains generic MITM-equivalent if leaves were enumerated. Verify original-ideal/diagonal saturation equivalence first at B4/B8, then unknown natural B31 residuals. Original `S9` roots/coefficients and independent sign/infinity branches are required for soundness.

## Negative and earlier results preserved

- Earlier [RR9 CI 43983006245](https://github.com/isogeny-labs/crypto/actions/runs/43983006245) failed example compilation with Rust E0499 due nested mutable borrow in the rational point-base builder; fixed by commit `7759eac4`.
- Distinct-only [CI 43983310619](https://github.com/isogeny-labs/crypto/actions/runs/43983310619) already passed **40+2 tests / 128 exact cases**, but did not contain the later Hermite extension; latest above supersedes for diagonal tests.
- Earlier complete APS 24Q composed 74.433895ms vs native unfolded rho32 79.833260ms are historical producer observations, with paired 95% interval crossing parity; no strongest rho, external index calculus or non-generic rank32 victory demonstrated.
- Exact tuned WZ/Brent rho benchmark PR #4468 still requires source-specific completed CI and source+memory-matched interpretation. Root main's missing workspace dependencies prevented earlier attempts; do not infer any new speedup.
- Source triage: merged #4391, #4451, #4454; compact quotient #4228/#4349, m6/m8 F4 censored #4373; Vieta #4410, auxiliary SIMD #1536; the published APS/PKM and classic genus-one interpolation are prior art. Research-KB MCP/Conductor unavailable; bounded repository search used, not novelty proof.

## Next concrete gate

Implement **blinded, no-generic-enumeration B4/B8 exact factor extractor** using RR9 determinant equations + `H(x_i)=0` + on-curve `y_i²=x_i³+ax_i+b`, confluent derivative at duplicate points, signed/infinity fibre controls. Put true/false and known Dickson negatives into formal solution-set-equivalence tests. Only after the algebraic extractor succeeds, attempt complete B31 unknown-target index calculus plus all cold costs versus truly optimized batch WZ and external published IC. **No complete cheaper non-generic ECDLP solver has been established.**
