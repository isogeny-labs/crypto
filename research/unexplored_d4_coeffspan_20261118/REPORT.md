# D4/S5 coefficient-span lattice: bounded falsifier and S3 calibration (2026-11-18)

**Classification:** CONJECTURE for D4/S5; EMPIRICAL, producer-local for a separate S3 sanity control. **No D4 adapted lattice has been implemented or tested. No ECDLP advantage has been demonstrated.**

## Hypothesis and novelty boundary

The corrected D4-invariant Semaev S5 construction uses four **genuinely bounded Y-coordinates**, `Y=(x-c)/(x+c)` with c²=B, and invariant variables `e1(Y²),e2(Y²),e3(Y²),∏Y`. The earlier experiment measured **105 monomial slots but a coefficient-family span of rank 59** as (A,c,xR) vary; at fixed curve varying xR the rank was 9. We ask whether this parameter-family dependency can lead to a **provably root-preserving coefficient-span-adapted modular lattice** with **at least 1.20x lower cold cost** for obtaining four independent exact integer equations and extracting a valid signed four-point S5 witness than the existing unadapted D4/S5 lattice.

**This precise adapted-lattice question is untested** in predecessor `research/ic_mega_pass_20261031/experiments/N2-d4-s5-lattice/AUDIT.md`, finding 5 and closure item 5. It is *not* a claim that D4, S5, small-root lattices, or the coefficient rank observation are novel. Novelty against literature remains unverified. The earlier MP-lattices-4 **small-x** invariant model was audited as invalid; we do **not** reuse its fictitious smallness assumption.

**Mathematical guard:** For nonzero polynomial H in an integral domain, multiplication by H is injective. A rank-59 **family of H coefficients** does **not** automatically make the 105-dimensional modular lattice rank 59. A proposed dimension-reducing projection that drops a root, drops independent constraints, or assumes determinant scaling without proof is a falsification of the proposed preconditioner, not evidence of a win.

## Predicted advantage and best baseline

If (counterfactually, until proved) one replaced 105 effective modular dimensions by 59 **without changing the determinant exponent or root constraints**, the illustrative shortest-vector exponent delta would be
`(104/105)-(58/59)=46/6195`. At a 6144-bit modulus this suggests about **45.62 bits** shorter vectors. This number is an unvalidated heuristic and cannot be converted into a 45-bit complexity reduction. The predecessor measured roughly **35-37 bits** of departure from a *generic determinant estimate*, **not** a controlled rank-59 versus rank-105 result, and did not recover four independent equations. Its finite-p observed reach bracket was δ≈0.00058..0.0011; the programme's four-summand IC target is δ≈0.169. Even a genuine 1.20x component improvement would remain engineering, not a new complexity exponent.

The primary **algorithmic** reference is the original 105-dimensional D4/S5 modular lattice on the same bounded Y model, same plant, same p and resource cap. A second indispensable reference is the **true un-symmetrized Y-box polynomial G(Y)** (the previous Y-box-vs-x-box equality was retracted). Synthetic D4 support-matched, root-conditioned H is a null, not a substitute for either. For end-to-end ECDLP a full cold direct IC cost must also be compared with automorphism-adjusted Pollard rho, with group operation reference `sqrt(pi*r/2)` before symmetry discount for subgroup order r. No per-curve subgroup r, relation/rank pipeline or rho timing was computed here, so **no numeric end-to-end/rho ratio is asserted**.

## Executed tiny control: S3 only

The executed study deliberately tests whether low-dimensional algebraic coefficient-family structure can matter **at all** before building a high-cost D4/S5 lattice. On 96 matched planted S3 relations per parameter, prime `p=2147483647`, curves `y²=x³+Ax²+x`, seed `20261118`, `sympy` LLL, the comparison is against a permutation-symmetric, **root-conditioned**, same-support, same-scaling random S3-shaped polynomial. All 192 group-law relations and 192 mutated-witness negative checks passed; every LLL vector was verified as a valid modular multiple of the test polynomial. Both cells' coefficient span has rank 4 of 9 over F_p. Independent symbolic S3 reconstruction plus raw-output validation passed.

| X bound | Planted cases | Rank | Median first-vector norm, actual / symmetric root-conditioned null | Ratio below 0.5 | First vector exactly vanishes at planted integer coordinates |
|---|---:|---:|---:|---:|---:|
| 4 | 96 | 4/9 | **0.2847615** | 89/96 | 96/96 |
| 8 | 96 | 4/9 | **1.0000** | 2/96 | 16/96 |

**Verdict on the control:** The coefficient-structured source can have much shorter first vectors in one smallness regime and **no median advantage in another**, even with the same rank deficiency. This is evidence that root bounds and structure must be controlled; it neither proves coefficient rank caused the difference nor supports the D4/S5 hypothesis. First-vector vanishing is not full root extraction, relation generation, linear algebra or descent. No unmeasured speedup is implied.

The controlled experiments were run locally on Python 3.13.5, SymPy 1.14.0, no Sage. CPU timings are in the per-case JSON. Original raw data retained in the attached experiment package and reproducible from the frozen script and seed. Pinned SHA-256: script `3d64487fd979d36ba7baafaf28b2724defc52cf8b2cdb3fce7b0082448149876`; raw X4 `e1cdf2c4fdf697d323f3457a7a22e37f5d9d6fd4383aa1dcf28381e84e5eafcf`; raw X8 `7a6f08f6987018526a6f43f0463b23dcd5ff6a488eb30d11c9e43ae86deda433`. `RESULTS_SUMMARY.json` includes all 96 per-case rounded first-vector ratios per cell, and the exact aggregate. Full raw samples are regenerable with the commands below.

```bash
python research/unexplored_d4_coeffspan_20261118/s3_control.py --cases 96 --seed 20261118 --X 4 --output RESULTS-X4.json
python research/unexplored_d4_coeffspan_20261118/s3_control.py --cases 96 --seed 20261118 --X 8 --output RESULTS-X8.json
python research/unexplored_d4_coeffspan_20261118/validate_results.py RESULTS-X4.json RESULTS-X8.json
sha256sum RESULTS-X4.json RESULTS-X8.json
```

## Frozen smallest **D4** disproof (not executed)

See `PROTOCOL.md` for exact conditions. Four planted samples over `nextprime(2^4096)`, fixed seeds `2026111801..04`, genuine bounded |Y|≤6, distinct squares, on-curve points and exact xR, and invariant-value validation. Derive/test coefficient-family relations on held-out curves. Compare existing D4 lattice, a **proof-carrying** root-preserving adapted lattice, a planted-root-conditioned generic invariant null, and the actual full G(Y) Y-box. Charge the construction, LLL, root extraction, negative/timeout cases, verification and downstream IC phases, each arm bounded at 20 CPU-min. Reject the **specific claimed improvement** if B cannot be implemented without violating root preservation or has <1.20x reduction in median complete-root recovery cost. Absence of four independent integer constraints is failure, not success. This gate cannot be replaced by the S3 calibration.

## Sources, limitations and next action

Closure reviewed: `research/ic_mega_pass_20261031/brief/BRIEF.md`, `report/REPORT.md`, `experiments/MP-lattices-4/AUDIT.md`; corrected D4 predecessor `experiments/N2-d4-s5-lattice/{README,AUDIT,PROTOCOL,build_h.py,lattice.py,rank_test.py}`. Context: Semaev, [summation polynomials](https://eprint.iacr.org/2004/031); Petit–Quisquater on Weil descent; Jochemsz–May on multivariate small roots. Repository search found no other completed coefficient-span-adapted D4-S5 experiment; this is not a verified literature novelty claim.

**Runtime barriers:** Research-KB and Conductor task claim tools were not available. The container lacks `gp/cypari2`, `fplll`, and sufficient installed D4 machinery to execute the four 4096-bit S5 arms here; these would require the lab's official runner and a correct preconditioner. GitHub read/search/PR connector was available. No PR claim is marked independently PCC-verified; full raw data is local+reproducible rather than checked into GitHub. Nothing was added to the IC leaderboard.

**Next:** Implement and prove the adapted-row transformation (or formally reject it if impossible), freeze four matching D4 cells, then execute it on a pinned runner and independently replay all roots, exact integer equations, and cost accounting. Close negative outcomes rather than re-nominate them.
