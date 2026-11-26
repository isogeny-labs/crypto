# Compact projective Veronese chain: cubic generators and a local first-fall trap

2026-11-23 | PR #5087 | producer-only, [pcc:unverified]. Follow-up to merged #5058, #5044 and #5023. Only the Semaev constraint is changed; no factor-base map L, relation collection or ECDLP claim.

## Construction

Let the curve be nonsingular, p>3, and write a Kummer abscissa as [u:z], with infinity [1:0]. Its degree-two Veronese vector is (A,B,C)=(u²,uz,z²). The homogeneous S3 is the symmetric trilinear tensor

    H = sum_{cyclic i,j,k}(A_j A_k C_i - 2 A_i B_j B_k
                         - 2a B_j B_k C_i - 4b B_i C_j C_k)
        + a² C_1 C_2 C_3.

Setting each z=1 recovers the ordinary S3. At an infinity input H=(u_j z_k-z_j u_k)², including the all-infinity point. In particular two infinities and one finite point are impossible. Normalization of internal nodes uses z(z-1)=0, (1-z)(u-1)=0. Consequently uz=u+z-1 and z²=z: no extra product variable needed for B or C. Define A=r with r-u²=0. For affine leaves introduce q_i=x_i². Every tensor entry is now affine-linear in the lifted coordinates, hence every tensor equation is cubic. The remaining equations are quadratic.

Use m-2 internal nodes in an S3 chain. In the geometric arm, internal coordinates may be in the algebraic closure and only the original abscissae need to be Fp-restricted for rational output. In the explicit Fp² arm, choose nonsquare d, use u+vω with ω²=d, impose (1-z)v=0, and define r=u²+d v² and h=2uv. The vector is (r+hω,u+z-1+vω,z), and each tensor equation is reduced modulo ω²-d into two cubic equations over Fp. Neither branch requires field-membership constraints on auxiliary variables to recover the projected geometric set.

**Exactness argument.** For an algebraically closed field, S3=0 is equivalent to the existence of a signed zero sum of lifts. The tensor extends this relation to the complete projective Kummer line, and a path of such triples glues by reversing all signs at each step if necessary. Thus projecting the chain to the original affine inputs recovers the geometric S_(m+1) zero set. Over Fp, all input lifts are in E(Fp²), so every true relation has normalized Fp² intermediate abscissae; conversely a valid chain gives a signed zero sum. Over the algebraic closure the explicit extension algebra splits into two scalar embeddings; either component is a valid geometric chain, and a geometric chain embeds diagonally (v=h=0). This proves the projected-set claim, not equality of nonradical elimination ideals or multiplicities. Adding field equations only on the original inputs is sufficient when extracting rational projected solutions; those equations are separately charged.

For an affine line x_i=c_i t+k_i, the additional geo_shared variant uses q=t² and x_i²=c_i²q+2c_i k_i t+k_i². This is an exact definitional substitution, specific to the one-dimensional slice, not a general factor-base map.

## Size before field equations or slices

| m | geometric variables/generators | explicit Fp² variables/generators | prior #5058 lifted sector variables/generators at Prime31 |
|---:|---:|---:|---:|
| 3 | 11/9 | 13/13 | 568/800 |
| 4 | 16/14 | 20/21 | 715/1020 |
| 5 | 21/19 | 27/29 | 862/1240 |

The exact formulas are 5m-4 variables, 5m-6 generators (geometric), and 7m-8 variables, 8m-11 generators (explicit), including original inputs. Some generator equations may specialize to zero, so counts are allocated generator slots. Maximum raw degree 3, versus degree 4 of the earlier sector system. The raw generator count comparison is not a runtime comparison and rational versus geometric witness requirements must not be conflated. For the one-dimensional shared-square arm after substitution the actual number of variables is 3m-4.

## Correctness gate and actual GB diagnostic

Seed 20261123. Four independent parameter cells (5,0,1,3), (7,1,1,3), (11,1,1,4), (19,2,3,5), each with 50 deterministic random affine lines, all slopes nonzero. Every t in Fp checked against fixed-degree homogeneous Sylvester resultants. Exact Fp² quadratic-root propagation independently recovers the projected set; each positive path evaluated against every explicit Fp² and geometric polynomial generator, with shared-square identities checked. Totals: **200/200 slice sets**, **2,100/2,100 point decisions**, **225 positive certificates**. The oracle deliberately enumerates a small Fp² square-root table: it is not a proposed fast relation solver. Prime31 correctness is not newly measured in this gate; Prime31 below is a symbolic/rank diagnostic.

Only after gates completed: two lines per cell, each arm lex Buchberger via SymPy 1.14.0 with common t^p-t. Monolithic control is an exact symbolic fixed-degree resultant restricted to the line, not an interpolant modulo t^p-t. Timings include construction, solve and projected extraction. All 26 completed solves matched the gated sets; 6 decomposed m=5 solves hit a five-second per-process cap, not exact UNSAT. The third local run was globally terminated before its last two arms; those two timeouts are from a separately rerun identical-seed worker (run 2b). The partial CSV retains that provenance and is not a statistically qualified timing panel. Process high-water RSS includes inherited Python state.

| p | m | median monolithic seconds | median geometric seconds | median shared-square seconds | median explicit Fp² seconds |
|---:|---:|---:|---:|---:|---:|
| 5 | 3 | .0266 | .0431 | .0479 | .1788 |
| 7 | 3 | .0335 | .0732 | .0678 | .1247 |
| 11 | 4 | .1171 | .4205 | .3707 | 1.9936 |
| 19 | 5 | .7887 | >5 cap | >5 cap | >5 cap |

**Negative:** the monolithic arm wins all completed paired slices. The cheaper geometry-only lift reduces overhead relative to the explicit lift in this diagnostic, but does not beat monolithic. No widening advantage over the baseline.

## First-fall invariant and why it is not an asymptotic result

Define V_D as the Fp span of monomial multiples of the *original* generators with total product degree at most D. The measured new direct fall dimension is

    dim(V_D ∩ R_{<=D-1}) - dim(V_{D-1})
    = rank(M_D) - rank(top_D M_D) - rank(M_{D-1}).

This is a direct filtered-span diagnostic, not iterative last-fall closure or observed F4 solving degree. No field equations/slices in this panel. Exact sparse modular ranks at D=2,3, including p=2^31-1, yield zero new falls at D=2 and at D=3 yield m-2 (geometric) or 2(m-2) (explicit). The first direct fall is exactly 3 for these raw presentations, and there is a uniform algebraic explanation:

    (1-z)(r-1) = (1-z)(r-u²) + (u+1)(1-z)(u-1).

For the explicit extension include d v(1-z)v in the right hand side, and additionally (1-z)h=(1-z)(h-2uv)+2u(1-z)v. Each is obtained with products of degree at most 3 but is quadratic. Their leading monomials z_i r_i and z_i h_i are absent from original quadratic leading forms; those forms are linearly independent (distinct leaf squares, z², zu, zv, u²+d v², uv). Therefore no direct degree-2 fall, and a degree-3 fall for all m≥3. At least m-2 or 2(m-2) such independent falls exist; the observed exact count is only measured for m=3–5.

**Critical control:** pre-insert these same redundant chart consequences and repeat the rank diagnostic. The new degree-3 fall dimension becomes zero in every tested cell (p=5,11,19 and Prime31 m=3,4,5), both variants. This proves the observed bounded first fall is a local presentation artifact in this panel, not evidence that global elimination stays bounded. Do not infer regularity, F4 solving degree or asymptotic work from it. A single monolithic polynomial without other constraints has no corresponding direct cancellation; that does not establish a solver disadvantage.

Smallest known failing controls from #5023 still apply: at p=5, E:y²=x³+1, S4(0,0,1,1)=0 needs an infinity intermediate; S4(0,1,0,1)=0 needs a non-base-field intermediate in the ordered chain. Our normalized charts accommodate both; no Fp-only affine-witness equivalence is claimed.

## Reproduce and next

    PYTHONPATH=. python -m summation_split.veronese_bench
    PYTHONPATH=. python -m summation_split.veronese_fall

Use a wall allowance above the sum of per-arm caps. Exact tested local blobs: veronese.py 2cf4beab890a6601917e5c2a391e0943fa9c028e; veronese_bench.py 9fe57662ca64b2561d32f42fbdacd3622258dc44; veronese_fall.py e4c514cbdbfd4242237ead6191c2b716df23e9fe. These were compared to GitHub blob identities. Local producer-only, no independent native CI, no signed PCC, no Sage/FGb/msolve/Singular available, no actual d_F4.

Next: preclose the local chart quadratics, instrument a true F4 implementation and use **two or more free parameters** under matched input-only constraints, because affine-line monolithic solving is univariate. Charge generation, field restriction and projection equally; add a balanced tree control. Use m=3,4,5 and 50 full projected-set gates per configuration before timing. Report actual Macaulay degree and memory, not raw degree or local falls. No success claim yet.

Prior art checked: Semaev 2004 (https://eprint.iacr.org/2004/031), Kosters–Yeo 2015 (https://arxiv.org/abs/1503.08001) and Kousidis–Wiemers 2019 (https://arxiv.org/abs/1906.05594), plus repository #5023, #5044, #5058. The latter first-fall results concern different Weil-descent systems. KB MCP/wrapper and conductor CLI unavailable in this runtime; GitHub search for Veronese summation_split returned no hits before this change, not proof of novelty. This is a compact classical tensor repackaging; no novelty priority asserted.
