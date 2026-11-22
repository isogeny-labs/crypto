# CT77 — Exhaustive binary reduction and CRT source compatibility [pcc:unverified]

**Goal:** Distinguish field-of-definition requirements of the *fixed* global curve from what is actually achievable by choosing a different rational global model, while keeping the entire prime-field source curve unchanged.

## Exact 32-model census

Every elliptic curve over F2 has a generalized Weierstrass equation

    y²+a1*x*y+a3*y=x³+a2*x²+a4*x+a6

with five binary coefficients. Exactly **16 of 32** coefficient tuples are nonsingular. The distribution by trace t=3−#E(F2) is:

| t | −2 | −1 | 0 | +1 | +2 |
|---:|---:|---:|---:|---:|---:|
| Number of nonsingular models | 2 | 4 | 4 | 4 | 2 |

Python establishes nonsingularity by the integer Weierstrass discriminant mod 2; independently authored JavaScript checks affine singularity directly via F=F_x=F_y=0 in characteristic two. Both enumerate all affine rational points and the smooth infinity point.

For each of the CT75 source primes p=29,37,59,139 and each good binary tuple b, take the original 11a1 coefficients o=(0,−1,1,−10,−20) and choose the integer coefficients A_i=o_i+p*((b_i−o_i) mod 2). Then A≡o mod p and A≡b mod 2, preserving **exactly the same source E(Fp)** while realizing every nonsingular binary target. Since Δ(o)=−11^5 and p≠11, both reductions are good. This is standard CRT and finite-étale torsion existence; it is **not** a computational map of arbitrary encoded source points.

## Exact minimal existence degree across every binary model

For a binary model with trace t, geometric ell-torsion Frobenius has polynomial X²−tX+2, and on a cyclic torsion basis the companion matrix is [[0,−2],[1,t]] modulo odd ell. The least extension with some nonzero ell-torsion has det(F^f−I)=0, and the least with all torsion is ord(F). Every one of the five possible trace models was checked.

| ell | CT75 source p | Best binary trace | Minimum f_some | Winning model f_full | Fixed 11a1 f_some from CT76 |
|---:|---:|---:|---:|---:|---:|
| 3 | 29 | 0 | 1 | 2 | 8 |
| 5 | 29 | −2 | 1 | 4 | 1 |
| 7 | 37 | +1 | 3 | 21 | 24 |
| 11 | 59 | −1 | 5 | 10 | 40 |
| 13 | 139 | +2 | 3 | 12 | 6 |

**Concrete ell=7 witness:** y²+37xy+y=x³+36x²−10x+17 preserves the CT75 source curve at p=37 and reduces at 2 to an elliptic curve with t=+1 and exactly **14 points over F8**. The Frobenius discriminant is −7≡0 mod7 and the 7-torsion action has a nontrivial Jordan block: some 7-torsion is F8-rational, but **full** 7-torsion needs F_(2^21). An arbitrary oriented generator is *not* guaranteed to land in that smaller eigenspace.

The winning ell=11 model has exactly **44 points over F32** and the winning ell=13 model has exactly **13 points over F8**. Two distinct binary-polynomial representations independently enumerate the full field points. These are field-degree **feasibility gates**, not a source-to-target map or a lower complete ECDLP solving cost.

## Exact, independent replay

Python enumerates the 32 tuples, uses discriminant parity, implements the CRT coefficient lifts, verifies good reduction at p and 2, computes every Frobenius degree and point counts over extension fields. JavaScript independently uses a derivative-based singularity test, separate group/CRT gates, a distinct matrix multiplication and distinct GF(8)/GF(32) polynomial bases. The 16 singular tuples are rejected. All smaller degree alternatives are excluded by exact Frobenius periodicity. The normal/optimized executions of Python and Node have identical 9-line JSONL SHA256:

fd91d33f5b5b845a7255c92a79edc7144661600ff36e360aa924a7a440e61006

    D=research/cross_characteristic_global_torsion_20261112/ct77
    python3 -B "$D/ct77_census.py" >/tmp/ct77-py.jsonl
    python3 -O -B "$D/ct77_census.py" >/tmp/ct77-pyopt.jsonl
    node "$D/ct77_independent.js" >/tmp/ct77-js.jsonl
    node --no-opt "$D/ct77_independent.js" >/tmp/ct77-jsopt.jsonl
    cmp /tmp/ct77-py.jsonl /tmp/ct77-pyopt.jsonl
    cmp /tmp/ct77-py.jsonl /tmp/ct77-js.jsonl
    cmp /tmp/ct77-py.jsonl /tmp/ct77-jsopt.jsonl
    echo 'fd91d33f5b5b845a7255c92a79edc7144661600ff36e360aa924a7a440e61006  /tmp/ct77-py.jsonl' | sha256sum -c

## Existing work and open mathematical gate

CT55 already proves an existential CRT prime-to-binary bridge and examines trace 0. CT76 computed the unchanged 11a1 trace −2 field degrees; CT77 is the exhaustive all-model fixed-source **comparison**, not a novelty claim for standard CRT/Frobenius. Root PR #3877, CT75 and concurrent cryptanalysis #1523 were read and their native genus-two/genus-three code left unchanged. Public background: https://crypto.stanford.edu/pbc/notes/elliptic/weier.html and https://websites.umich.edu/asnowden/teaching/2013/679/L08.html. Shared research-KB MCP/checkout wrapper, Conductor and Lean toolchain were unavailable; fallback literature review is incomplete. Independent PCC science remains [pcc:unverified].

**Next:** supply a uniform global recognition/lift of both *arbitrary encoded* source points with a shared second-prime place, calculate their actual destination coordinates without the source logarithm, then compare full transfer and target solve costs against matched optimized rho/index calculus. A favorable f_some alone is neither a globally selected lift nor a DLP speedup. No paid compute or protected merge.
