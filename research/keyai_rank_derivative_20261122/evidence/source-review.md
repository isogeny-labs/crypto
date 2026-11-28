# Separate-role source review

Two separate task roles reviewed the frozen production changes and example.
One reviewed the algebraic and row-accounting contracts; the other authored
and executed the independent correctness oracles in the qualifier. These are
same-team, same-host checks, not independent-host or signed PCC verification.
[pcc:unverified]

Reviewed source hashes:

- `src/cryptanalysis/semaev_prime_poly.rs`: `e77ebb1576469b24e399cf2cd62059b80607eb65eeaa486847782e6f11714f9b`
- `src/cryptanalysis/ic_framework/useful_rank.rs`: `38037290feb8974626e7de8632bfcd8c1b16500c296149f820b782058ba33e9e`
- `examples/keyai_rank_derivative_controls.rs`: `64299e12deeb21bc74ed9a57bd93a1ac1be6125dcae5acc0fd6f7a91d383bef3`

The review checked the characteristic and representation guards, repeated
factors, the distinction between support and signed lifting, the coefficient
Jacobian formula and its squarefreeness condition, fixed-target sign and
cofactor normalization, preservation of full equations and RHS, and target
determination by the difference of two ranks. It found no remaining issue
within this bounded integration after the following correction. [pcc:unverified]

An initial cofactor test used an arbitrary multiplier of four on the
cofactor-one Prime31 curve. Although the normalization algebra was valid,
its `InstanceCtx` metadata contradicted the documented cofactor convention.
Before qualification, it was replaced by the actual curve
`y^2=x^3+x` over `F13`, with 20 enumerated points, subgroup order five,
cofactor four and generator `(4,4)`. The qualification contains that corrected
test, including its point census and anchored scalar check. [pcc:unverified]

No result here establishes a new general theorem, a signed verdict, global
solving-degree behavior, or a performance gain. The external Lean sources are
provenance for the adopted controls and are not imported into this Rust PR.
