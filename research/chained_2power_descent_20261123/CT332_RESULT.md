# CT332 — all-vector Frobenius filter and abstract 4/8-adic lift checks

**Status:** executed bounded local *abstract-module* unit check; producer-authored source and report, exact Git/source hashes and raw JSON retained, **not** signed PCC, Lean formalization, genuine Jacobian arithmetic, divisor replay or ECDLP speedup. This check is separate from the concurrent genus-three product-PPAV native study and its RREF owner. [pcc:unverified]

## Source and reproducible receipt (2026-11-23 UTC)

- Protocol was committed **before the final frozen-source validation**: [CT332_PROTOCOL.md](CT332_PROTOCOL.md); the earlier exploratory development run is excluded.
- [Exact source](ct332_independent_frobenius_oracle.py), commit `471c335a2fddb7cc2e671bb7c7f6b2bf4826cb4f`, Git blob `0b81eaa1fc8cf1f3cf2f4898b2b76be862a53fb3`, SHA-256 `d5b7787ac988b54d73364a3356793622bee02f8077024b04145c768231b62e76`.
- [Entire JSON result](CT332_RESULT.json): Git blob `5e631c740255ebdf3ebea161ec3d8286e44cabed`, SHA-256 `09ca0bea44d63be011612fa066da07645f85d99e56c809c69992673a46b81e80`; final JSON was canonicalized to the byte-identical Python `sort_keys=True` stdout after initially being uploaded with JavaScript's integer-like key ordering. The first upload is retained in Git history; no scientific value changed.
- Local bounded commands, source unchanged: `python3 ct332_independent_frobenius_oracle.py > ct332_result.json`; `python3 -O ct332_independent_frobenius_oracle.py > ct332_result_optimized.json`; `cmp ct332_result.json ct332_result_optimized.json`; `sha256sum` source/output. Both executions exited zero, `cmp` exited zero. No native child agents or external validator participated. No EKS/paid jobs, benchmark reservations or original norm records were touched.

## Actual finite observations

| Root permutation | Nonzero 2-torsion orbit sizes | Krylov rank 4 / 15 | Minimum Krylov rank |
|---|---|---:|---:|
| `(5)(1)` | 5, 5, 5 | **15** | 4 |
| `(3)(3)` | 3, 3, 3, 3, 3 | 0 | 2 |
| six-cycle | 3, 6, 6 | **12** | 2 |
| identity mutation | fifteen fixed points | 0 | 1 |

**Discriminating negative control:** 12 of 15 nonzero vectors for the six-cycle produce *full-rank* four-column Krylov matrices, yet its action is reducible (three seeds have rank two). Hence **finding one cyclic vector is insufficient to certify irreducibility**. The correct all-vector criterion, or an independent irreducible-characteristic-polynomial proof, is required before applying CT331's all-length restriction. The `(5)(1)` characteristic polynomial `x^4+x^3+x^2+x+1` is irreducible over `F_2`, matching the separate written proof.

The fixed, explicitly abstract companion lift of that polynomial to `(Z/4)^4` and `(Z/8)^4` was checked to have order five and reduce to the same mod-two Frobenius matrix. For every primitive vector, the four-column orbit determinant was odd: **240/240 vectors mod 4 and 3840/3840 mod 8**. This is an exhaustive finite matrix diagnostic, not a random sample. The particular integral lift is *not* asserted to be the 2-adic Frobenius of any genus-two Jacobian or to respect a chosen Weil pairing or similitude factor.

## Shorter general proof route to formalize

Let `k=F_q` have odd characteristic, `A/k` be an abelian variety and `A[2]` be an irreducible `F_2[Gal(kbar/k)]` module. Suppose `H` is a finite, Frobenius-stable 2-primary subgroup of exponent `2^n` (and nonzero). Choose `v in H` with exact order `2^n`. Write `T=A[2^n](kbar)`, a free `Z/2^n`-module of rank `2g`. The reduction `T/2T` is Galois-equivariantly isomorphic to `A[2]` via multiplication by `2^(n-1)`, and the reduction of `v` is nonzero. Irreducibility makes the Frobenius orbit of that reduction generate `T/2T`. By Nakayama's lemma over the local ring `Z/2^n`, the orbit of `v` generates **all of** `T`. As `H` is Frobenius-stable and contains `v`, `T subset H`; the reverse inclusion follows from its exponent, so `H=T`.

This gives a direct generator/Nakayama route to CT331's finite-stable-kernel statement, without induction on `|H|`. The quotient-to-`k`-isomorphism/Picard bridge and inherited Mathlib group-instance formalization still require proof/checking. In genus two the mod-two `(5)(1)` case excludes nontrivial descended 2-primary *codomain classes*, but **not** faster models of an isomorphic variety, rational six-cycle/3+3 branches, odd-degree maps or higher-dimensional norm/trace transfers. A rational endpoint model without a rational **composite morphism** does not activate this gate.

## Ownership and next gate

No native subagent slots were present. The coordinator handled the mathematics and bounded Python structural oracle, not a signed independent verifier; Rust norm-reuse/Cantor ownership and genus-three RREF/next real mod8 owner remain untouched. Source agreement between local tests and published Git blob is a reproducibility receipt, not independent PCC. The next *nonduplicative* work is a separate review of the Nakayama proof in actual shared Mathlib instances and, by the native owner, a genuine polarised mod-4/8 Jacobian/Frobenius control rather than the abstract lift. End-to-end cold 695/701 original norm-reuse, physical `C169/C13a/C13b/N2_actual/D` pricing and strongest matched complete rho/BSGS/IC remain governed by existing custody/budget holds.

Sources checked: [Castryck–Decru–Smith Richelot paper](https://doi.org/10.1515/jmc-2019-0021), [finite-field Frobenius subgroup schemes](https://doi.org/10.1016/j.ffa.2014.04.001), [CT331](CT331_IRREDUCIBLE_TORSION_GATE.md), [two-hop #4992](https://github.com/isogeny-labs/crypto/pull/4992), [root queue #3904](https://github.com/isogeny-labs/crypto/issues/3904). The project KB MCP/local wrapper was unavailable; this is not a novelty clearance.
