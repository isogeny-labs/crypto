# CT217 — exact degree-3 norm projector, offset relation rank, and charged toy recovery

**Date:** 2026-11-18. Continuation of [crypto PR #4715](https://github.com/isogeny-labs/crypto/pull/4715), following [CT216](CT216_GENUS4_COSET_SCREEN.md); no changes to the original 695/701-custody fixture. **[pcc:unverified]**.

## Proven algebraic claim (for the stated public toy model)

Over F101 let E: Y²=X³+X+46, #E(F101)=109, C: y²=x⁹+x³+46 (genus 4), and the degree-three map π(x,y)=(x³,y). Define α=π*: E(F101)→J_C(F101) and β=π_*: J_C(F101)→E(F101). Functorial norm/pullback gives **β α = [3]**. Since 109 is prime to 3, **3⁻¹ mod 109=73**. Consequently s=[73] β is a left inverse of α and

    e = α [73] β,    e²=e,
    J_C(F101) = α(E(F101)) ⊕ ker(β).

This is a direct-product splitting of *rational groups*, not merely a guessed 109-primary cofactor projection. For every D in J_C(F101), D=(D−eD)+eD and β(D−eD)=0. For any offset D0 and source generator G, the exact coset identity is

    D0 + α([k]G) = K0 + α([73]β(D0) + [k]G),
    K0 = D0 − e(D0) ∈ ker β.

The proof uses the morphism degree and #E=109; no value of #J_C, guessed cofactor h, or degree-4 smoothness assumption is needed. It **does not** imply faster ECDLP or prove that the existing genus-ladder's h-projection is safe. The other direct factor can still dominate smoothness and linear algebra costs.

## Execution evidence: exact-head hosted CI

[CT217 oracle job #135199200895](https://github.com/isogeny-labs/crypto/actions/runs/44416819522/job/135199200895) on source 58d7baed4444 passed on 2026-11-18. [Script](ct217_norm_relation_audit.py) uses the CT216 Cantor implementation for divisor products but independently checks via elliptic-group norm sums; thus it is a separate arithmetic check, **not independently signed PCC**.

- All **1,744** identities D0+α([k]G)=K0+α([73]β(D0)+[k]G) hold over 16 frozen offset cosets × 109 source elements; 16/16 offsets have nonzero complements.
- All **33** squarefree split-degree-4 witnesses satisfy β(D0)+[3k]G=Σπ(P_i). Dropping D0 gives a wrong norm in **33/33** cases.
- Before substituting offset construction, 33 relations have rank **33** in **92** columns (84 distinct factor points plus 8 offset symbols), nullity 59. **After** substituting the actual rational-point support of the offsets, the correct reduced 33×84 factor-point matrix has rank **31** over F109, **nullity 53**. This finite roster therefore cannot determine its entire degree-one factor base: it is not a successful IC relation matrix. Other offset rosters, large primes and walks remain open.
- For a public T=(64,19)=[37]G, a fixed pair offset and **3 candidate trials** find a fully split class at walk coefficient a=2. Scalar **37** is recovered and verified using a full exact norm identity. Crucially, the routine separately computes **6 elliptic factor/offset norm DLPs** by cold BSGS; they cost **142 elliptic additions**. A direct cold elliptic BSGS of T uses **25** additions in the same procedure. The transferred toy routine additionally performed at least **6 Jacobian additions**, plus uncharged offset enumeration, polynomial arithmetic, checks and setup.

The 142/25=5.68 relative elliptic-operation count is a *partial-operation negative control*, not a matched wall-time cost ratio or an optimized baseline. Precomputing the six norm logs is itself source-curve ECDLP work, not free index calculus. No new complete IC advantage can be inferred from the recovered scalar.

## Reproduction

    python3 research/chained_jacobian_transfers_20261110/toy_dlp_ct204/ct217_norm_relation_audit.py --self-test
    python3 research/chained_jacobian_transfers_20261110/toy_dlp_ct204/ct217_norm_relation_audit.py > /tmp/ct217.json
    cmp /tmp/ct217.json research/chained_jacobian_transfers_20261110/toy_dlp_ct204/ct217_f101_norm_relation_audit.json

The frozen [machine-readable receipt](ct217_f101_norm_relation_audit.json) is from the above exact-head oracle log. GitHub CI performs the file equality check on the next head; **do not infer that next CI or the native Rust job passed until inspected**.

## Prior-art and adjacency

Existing [CUBIC_NORM_20261111.md](../CUBIC_NORM_20261111.md), [CT216](CT216_GENUS4_COSET_SCREEN.md), [crypto #4215](https://github.com/isogeny-labs/crypto/pull/4215), [cryptanalysis #1523](https://github.com/isogeny-labs/cryptanalysis/pull/1523) and [genus-4 IC #4693](https://github.com/isogeny-labs/crypto/pull/4693) were reviewed; neither the cover nor the degree/norm identity is new. Classical context: [Gaudry, EUROCRYPT 2000](https://doi.org/10.1007/3-540-45539-6_2), [Diem's double-large-prime IC](https://arxiv.org/abs/math/0606607), and [GHS Weil descent](https://doi.org/10.1007/s00145-001-0011-x). Those works do not establish a cold prime-field ECDLP speedup for this specific transfer. Shared Research-KB MCP and repository wrapper are not accessible in this connector runtime, so the review falls back to these original files and references; no exhaustive novelty search is claimed.

## Next concrete experiment and blocked lane

For an actual independent IC solve, extend the exact degree-one/degree-two basis and offset sampler until the **offset-eliminated** relation matrix reaches sufficient rank, track kernel-component cancellation without silently discarding it, solve target and verify scalar against E. Reuse existing Rust genus-ladder/LA rather than treating the norm factor-base DLP oracle as free. Record all setup, transfer, relations, linear algebra, norm, recovery, rho/BSGS/IC references, physical C169/C13a/C13b/D and memory with uncertainty. The original 695/701-record experiment remains blocked on documented original-model S3 HTTP 403; no impersonated fixture or unauthorized EKS/Spot compute is introduced.

**No protected merge, complete cold speedup, native CT217 performance evidence, or independent PCC claim.**
