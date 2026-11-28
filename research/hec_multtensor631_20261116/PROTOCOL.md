# Compiled quotient multiplication and dual colon contractions

For the actual target quartic, compile structure coefficients mu(i,j,k)for every degreepaira+b<=10directly from its computed normal-form table. Bound total stored sparse entries by1million; construction is in the run, not an old cache input. Multiply forms by tensor contraction instead of repeated inner monomial HashMap lookups. Compare every output coefficient against original multiplication.

For colon constraints, contract each annihilator row once with the multiplication tensor to obtain its bilinear matrix, then apply those matrices to each supplied form. By linearity, row(i)=sum_j u_j sum_k ann_k mu(i,j,k), the same as annihilating the product. Require the resulting canonical nullspace equal the original colon implementation on every call. Preserve nullspace/rank semantics; no group-law shortcuts or scalar oracle.

Execute actual direct quadratic original-input reconstruction with the checked production section shortcut, including explicit degree2base control, all public primary/adjacent/sign/corruption checks and both chart comparisons. Require nonzero multiplication and colon check counters and both unique classes. Post-run byte comparison to independent class recovery is validation only. Freeze all source/dependency/input/binary hashes, commit/checkpoint/gate before one execution. Retain cap/rank/mismatch failures without changing policy.

The comparison build runs references and cannot measure speedup. Shared corpus points to existing Khuri-Makdisi-style repository group law; no novelty asserted. Precomputation/dual maps must be charged in any later measurement. Earlier noisy production cohort and1.621xinverse record remain unchanged. No generic/global-map/genus2/4or10xclaim.
