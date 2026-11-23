# CT332 — independent abstract Frobenius/Krylov oracle preregistration

**Status:** source-only bounded validation helper, not a native Jacobian/Rust experiment, formal theorem, signed PCC or performance observation. This independent **implementation path** is complementary to the separately owned genus-three native module and its future mod-8 qualification. Do not claim an independent worker or a signed validator. [pcc:unverified]

## Frozen inputs / source (2026-11-23 UTC)

- Existing CT331 written hypothesis: `research/chained_2power_descent_20261123/CT331_IRREDUCIBLE_TORSION_GATE.md`, referenced Git blob `0816b8e36b46038a2c6148b516fb7bca00f98d61`.
- New bounded oracle: `ct332_independent_frobenius_oracle.py`; exact Git blob `0b81eaa1fc8cf1f3cf2f4898b2b76be862a53fb3`, SHA-256 `d5b7787ac988b54d73364a3356793622bee02f8077024b04145c768231b62e76`, source commit `471c335a2fddb7cc2e671bb7c7f6b2bf4826cb4f`.
- Self-contained Python 3 standard library only. No original 695/701 records, locked data, production algebra, EKS job, batch solver, Lean code, or source-dependent benchmarking.
- Literature and prior sources: CT13/CT50, #4992 native two-hop controls, CT331, Castryck–Decru–Smith (2020) Richelot background [doi:10.1515/jmc-2019-0021](https://doi.org/10.1515/jmc-2019-0021), and finite-field Frobenius/Tate-module context [Finite group subschemes of abelian varieties](https://doi.org/10.1016/j.ffa.2014.04.001). KB unavailable: no KB MCP or repository wrapper in this tool runtime. No novelty/absence claim.

## Single frozen validation call

Run at the above SHA, without changing the script:

```sh
python3 ct332_independent_frobenius_oracle.py > ct332_result.json
python3 -O ct332_independent_frobenius_oracle.py > ct332_result_optimized.json
cmp ct332_result.json ct332_result_optimized.json
sha256sum ct332_independent_frobenius_oracle.py ct332_result.json
```

Expected exact structural assertions:
1. The 15 nonzero vectors of the four-dimensional even-subset quotient representation have Frobenius orbits `5,5,5` and **Krylov rank four for all 15** for root cycle `(5)(1)`. An all-cyclic-vectors check is essential: the six-cycle control may have cyclic vectors even though it is reducible.
2. The deliberately reducible `(3)(3)`, `(6)`, and identity control actions each have at least one non-cyclic nonzero vector; enforce fail-closed rejection if not.
3. The explicitly stated **abstract** integral companion lift of `x^4+x^3+x^2+x+1` has order five and reduces to the `(5)(1)` mod-two action. The Krylov determinants are odd for every primitive vector in `(Z/4)^4` (240 vectors) and `(Z/8)^4` (3840 vectors). Counts are finite matrix diagnostics, not samples of Jacobian divisors or actual finite-field Frobenius lifts.
4. Both Python modes produce byte-identical JSON, source hash and assertion verdict. A difference/failure is retained as a negative, never silently patched to match.

A development run occurred **before** this preregistration and is excluded from any claim of pre-registered validation. Subsequent calls must use the exact source just committed/read back. The script's `verified: true` output means only its internal assertions passed; it is **not** a signed independent verification.

## Mathematical use / unresolved gates

For a finite Galois-stable 2-primary subgroup `H` of `A[2^n]`, let `v` have maximal 2-power order. Its mod-two reduction in the free `(Z/2^n)^{2g}` Tate-module quotient is nonzero. If the Frobenius representation on `A[2]` is irreducible, the orbit of the reduction spans `A[2]`; Nakayama then shows the orbit of `v` spans `A[2^n]`. Thus `H=A[2^n]`. This is a **proof sketch to independently formalize**, not verified by a bounded matrix experiment. The genus-two `(5)(1)` special case admits a four-vector Krylov test; it does not generalize as a four-vector test to dimension other than four.

The corresponding Richelot all-length consequence still requires a `k`-defined composite morphism, not merely an endpoint equation. Even for an isomorphic codomain, representation-level algorithmic advantages are separate. The original norm reuse/cold fully charged rho/BSGS/IC/PCC milestones remain owned, unrun here.

**Ownership:** no child agents exposed. Coordinator owns only `ct332_independent_frobenius_oracle.py`, this protocol, the phase-local result note/JSON and its own handoff. The genus-three product-PPAV native module, its independent RREF validation, the authentic norm-reuse 695/701 custody, and 4992/4990 formal/CI obligations remain other owners' work.
