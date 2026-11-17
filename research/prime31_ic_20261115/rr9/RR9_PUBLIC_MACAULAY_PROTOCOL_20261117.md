# EXP-R31-RR9-MACAULAY: public-only B8 coefficient-ideal unit certificate

Preregistered 2026-11-17 UTC. [pcc:unverified]. Stacked PR #4548 on #4547.
No claims of a cheaper ECDLP algorithm, non-generic complexity, or unknown-target logarithm recovery.

## Inputs and exact algebra
On the pre-existing ordinary cofactor-one curve E/F_2147483647 with a=847514846, b=2087444724 and n=2147446831, take the **first eight distinct rational x coordinates** of the existing Rust basis. Let H(X)=prod_i(X-x_i), monic degree eight. For any public non-infinite residual R=(r,s), seek unknown A(X)=sum_{i=0}^3 a_i X^i and V(X)=X^3+sum_{i=0}^2 v_i X^i (seven unknowns).
Define M=D_R V²-(X-r)A²-2sAV, D_R=X²+rX+r²+a. Form eight affine quadratic equations q_j=[X^j](M-H)=0, j=0..7. Degree-eight coefficient is automatically zero. This is exactly the squarefree full-B8 stratum; repeated x leaves, other eight-subsets, x(R)=x_i, V3=0 and R=O are not covered by this pilot.

## Falsifiable experiment
Implement in a **new Rust example** linked to the unchanged native prime31 library. The candidate receives only public (E,R,H), builds q_0..q_7, enumerates **coefficient monomials** (not signed supports) and tests whether 1 lies in the span of monomial multiples m*q_j with degree(m)<=3 and total degree<=5 (bounded Macaulay linear algebra). A unit witness implies no solution of the seven-coefficient B8 chart. A non-unit outcome means only 'not excluded at this degree', never 'solution exists'.

Controls: independently reconstruct q_j by evaluation of M at fixed 7-vector test charts; compare exact coefficients. Build public positive R as eight *signed points* (test oracle only), and public negative R outside all 256 signed eight-sums (test oracle only). Candidate must never read masks, target scalar, full group table, or enumeration answer. Assert positives are never certified absent, and any negative unit certificate is consistent with independent curve-group census. Account rows, columns, pivots, field multiplications, inversions, elapsed source runner wall; use exactly same prime31 curve. Include special x(R)=x_i exclusion checks. Do not infer typical yield from planted positives.

This experiment stops at negative-only unit-ideal certificates: not an extractor, not a complete ECDLP solver. If degree 5 has no unit certificate, record 'inconclusive' without post-hoc raising degree or changing stop rule. Do not label a producer pivot calculation as signed independent verification.

## Reproducibility and comparison gate
Pinned source: #4547 parent native Rust example; same unchanged `research/prime31_ic_20261115/native/lib.rs` built at immutable source commit 1df714645aa683cf975bc76fcacb9842cb4780fd; use existing GitHub Actions Rust 1.100 workflow approach. Record toolchain, source SHA, counts, rejection cases, wall, and all failures.

Fully charged historical comparator from #4532: 1024 complete unknown logs, cached 2^20-entry BSGS 170.113ms versus mixed IC 2221.474ms on that same runner [pcc:unverified]. The new certificate is only a feasibility component. No valid speed ratio is defined until an actual public-only relation extractor, exact B31 solver and matched same-host baseline exist.

## Prior art and lead exclusions
Source chain #4466/#4539/#4541/#4545/#4546/#4547 (RR9 chart, multiplicities, signed fibres, seven-anchor genericity, interpolation obstruction); #4373 capped F4; #4228/#4349 quotient; #4513 mixed IC; #4532 cached BSGS. Crypto-autoresearcher #3301/#3305 (negative heat map / Semaev alternatives). Amadori-Pintore-Sala (2018, DOI 10.1016/j.ffa.2018.01.009), prime-field Semaev/Gröbner relation prior art. No novel F4/Macaulay claim. KB MCP and repository CLI unavailable here; fallback GitHub source and primary publisher metadata checked.

## Prospective amendment A: positive recovery from the SAME degree-5 quotient (registered before any numerical outcome)
The degree-5 Macaulay rowspace also has a positive-side question: take normal forms of 1 and each of the seven coordinate functions under **the same** pivot basis, without changing the 792 columns, 960 input rows, trial fixtures, degree cap or group oracle. When every variable normal form is a scalar multiple of the normal form of 1, derive the seven scalar coefficients. Accept only after all eight original quadratic equations evaluate to zero, M=H exactly, and all eight public base x coordinates each choose precisely one curve sign via F(x,y)=0. Verify the signed point sum against R using the independently reused original elliptic curve arithmetic **outside** the algebraic candidate. Return *inconclusive* when normal forms do not identify a unique solution. The candidate never receives signed support, masked signs or an answer scalar. A successful B8 positive-only extraction is a component, not complete ECDLP, and does not establish a speedup; absent extraction must be reported without adapting degree post-data.

This amendment reuses the registered degree-5 source and does not add any new R, support, seed or computation budget. Source initially registered at ad957457; amendment before observing numeric results of the initial CI #44083153755.
