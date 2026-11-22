# Unselected source lead: reuse retained quadratic classes during descent

This is a source review and conditional construction, without a new implementation, experiment, yield estimate or speedup claim. Keep the active41source/protocol/binaries and all original data unchanged. The next experiment is not selected here.

Source facts: jv_cover::weil_forms builds the quotient S=(C²H−A²)/u_R for reduced residual R=(u_R,v_R), with C=x+μ and A=−xv_R+λ0u_R+λ1xu_R−μv_R. Its six Weil equations require S's coefficients to lie in Fp, with S monic degree6. nagao_decompose currently rejects a solution immediately unless S has six distinct Fp roots, before constructing A and their ordinates. Public weil_forms/polys/eval, solve_zero_dim_traced, Fq::count_public and opts_for_mode permit a separate Rust prototype without modifying the active production library. [pcc:unverified]

The existing relation collection already retains anchors of the form

    sum(sA_i * B_i) + cA * D = 0,  cA in {+1,-1},

where D is a canonical reduced degree-two Mumford class over Fp². Once the homogeneous kernel x=h*t is ready, every retained anchor also gives the conditional scaled log of its D by an exact dot product. No extra class identification or collection run is needed to make those anchors available. The class descriptor includes U and V; U alone is insufficient. [pcc:unverified]

For a candidate Nagao solution, suppose the exact quotient factors as four distinct supported Fp roots times monic irreducible quadratic U. Require a valid residual, disjoint squarefree supports, full polynomial identity A²−C²H=−u_R*S, no ramified selected point, and C coprime to every support polynomial. Set V=−A/C modU, verify V²=H modU, and canonicalize D/−D with signed coefficient cD. The principal divisor then gives

    R + sum(sP_i * B_i) + cD * D = 0.

If the exact signed class D has a retained anchor, eliminating it gives the group identity

    R = -sum(sP_i * B_i) + cD*cA*sum(sA_i * B_i).

This fixes the sign: the current full Nagao materializer also negates the principal-function ordinate signs. With R=aG+bQ, the resulting affine row keeps target coefficient−b and RHS a. Store both original inputs and their signed cancellation witness, verify their group equations and combination independently, then project using the actual homogeneous kernel. The existing elliptic target check remains required. [pcc:unverified]

| Variant to investigate | Additional work / uncertainty |
|---|---|
| A: four small points plus one anchored quadratic | Lookup/prove one exact class; actual match rate and rejection cost unknown |
| B: two small points plus two anchored quadratics | Factor the remaining quartic, prove both classes and replay two anchors; joint coverage unknown |
| C: retain unanchored affine quadratic rows | Match later rows/classes through a graph while protecting kernel scale, target and RHS; collision/fill costs unknown |
| D: collect more anchors after homogeneous rank readiness | Trade additional relation work and memory against descent coverage; requires a fresh complete pipeline measurement |

A suitable first component would compare the existing full-six-root materializer with A over an identical residual/cofactor stream, retain every outcome and invalidity, and check actual affine rank/target rather than raw partial counts. Separate extraction RNGs are needed if an added factorization path consumes randomness. This is a proposed control design, not a selected protocol or authorization expansion. [pcc:unverified]

Prior work: Joux–Vitse, [Cover and decomposition index calculus](https://www-fourier.ujf-grenoble.fr/~viva/research/articles/CoverDecomp.pdf), §3.1 describes the residual norm quotient and full-splitting condition; §§3.2–3.3 describe sieving and split double-large-prime variations. Read primary PDF SHA256 7ca88c54ebfcb6fac8d05d313bfe8c26d3cb133081c55c4ab6ea4c1b45729b38, not an abstract. This construction uses the actual code's A+C*y and sign conventions. No novelty or asymptotic claim is made. The KB tool catalog contains no kb_status/search tools; the repository wrapper has no executable. Exact mechanism/negative-form queries therefore cannot be executed; explicit unavailability receipts accompany this review. [pcc:unverified]
