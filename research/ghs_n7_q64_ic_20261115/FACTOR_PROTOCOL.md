# Frozen exact q64 full-Euler-factor check

15 November 2026 UTC, after the successful exact map receipt and before any q64 attack measurement. Hess's published Theorem 15 predicts `L_C(T)=(1+9T+64T²)(1+1226009T⁷+64⁷T¹⁴)` for this mapped genus-eight curve. It gives `#J=4398047737114*74=325455532546436=148*2199023868557`. The map receipt checks the source, genus and nonzero selected subgroup but has not computed the full polynomial.

Commit `factor.magma` before a public calculator call. Reconstruct the exact source and `WeilDescent(E,F64,1)`, compute `EulerFactor(Jacobian(C))` independently, assert equality with the entire predicted degree-16 polynomial and both order evaluations. Preserve source hash, CPU, HTTP status and complete response in immutable `receipts/factor1.json`. Stop on transport error, timeout at 180 seconds, mismatch, or truncation. Do not use cofactor 148 in a costed attack until this check passes. This is an exact-curve check of the published factorization.
