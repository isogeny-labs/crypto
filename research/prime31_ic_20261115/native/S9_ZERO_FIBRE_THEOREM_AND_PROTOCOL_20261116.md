# Prime31 S9: exact zero-fibre theorem from four-sum injectivity

Frozen 2026-11-16 **before updating the B31 Rust multiplicity tests**. [pcc:unverified]

## Sources checked and precise question

Reviewed `isogeny-labs/crypto` AGENTS.md, latest shared session logs, merged [#4391](https://github.com/isogeny-labs/crypto/pull/4391), the successful [B31 4+4 oracle run 43952234526](https://github.com/isogeny-labs/crypto/actions/runs/43952234526), and the prior [B31 net-multiplicity protocol](S9_B31_MULTIPLICITY_PROTOCOL_20261116.md). The source-bound multiplicity runner [43957151566](https://github.com/isogeny-labs/crypto/actions/runs/43957151566) failed its short-base replay test; repaired source is on main, but [43960118653](https://github.com/isogeny-labs/crypto/actions/runs/43960118653) has not yet given an admitted verdict. Also checked crypto #4228/#4349 quotient roots, crypto-autoresearcher Semaev alternatives SUMMARY/ERRATA and cryptanalysis prime-field auxiliary IC. Primary literature is PKM 2016 DOI 10.1007/978-3-662-49387-8_1 and APS 2018 DOI 10.1016/j.ffa.2018.01.009. Research-KB MCP/wrapper, local Conductor/cargo and EKS access are absent from this connector runtime. A negative KB search was not fabricated.

## Proposition — exact finite-support obstruction

Let `G` be an abelian group with chosen elements `P_1,...,P_B`. For even `r>=0`, put `D_r={d in Z^B: ||d||_1<=r, ||d||_1 even}` and `Phi(d)=sum_i d_i P_i`. Suppose `Phi|D_4` is injective. Then **the only net signature** `d in D_8` with `Phi(d)=0` is `d=0`.

**Proof:** Given `d in D_8`, split its `||d||_1=k` oriented unit atoms into two even-sized portions of at most four atoms each: for k=0,2,4,6,8 use respective portion sizes (0,0), (0,2), (2,2), (2,4), (4,4). Their integer net vectors `u,v` lie in D4 and `d=u+v`. If `Phi(d)=0`, then `Phi(u)=Phi(-v)`. Since `-v` also lies in D4, injectivity forces `u=-v` and `d=0`. The reverse implication is trivial. No assumption on field characteristic, elliptic-curve model, group order, automorphisms or randomness is used.

**Corollary for odd-order signed leaves:** For exactly eight signed leaves, the net identity signature `d=0` arises exactly from four cancelling pairs `P_i,-P_i`, with repetitions allowed. The *x-only* degree-eight polynomials `f(T)=prod_i(T-x_i)^{2c_i}` are indexed by weak compositions `sum c_i=4`, hence **C(B+3,4)** distinct polynomials. For B31 this is **46,376**, despite there being just **one** net d for Q=O. There are no other noncancelling identity relations at eight-leaf arity, if the D4 group-point injectivity hypothesis holds.

This rules out **nontrivial factor-base-only eight-leaf equations to the identity on this fixed base**, rather than proving anything about labelled **nonzero** residual targets R=aG+bQ, which remain the route to useful IC relations. The theorem does **not** refute a target-conditioned S9 solver.

## Instance evidence and frozen Rust gate

Prior source-bound [43952234526](https://github.com/isogeny-labs/crypto/actions/runs/43952234526) reports 618,885 distinct D4 group-image points for exactly 618,885 distinct D4 integer signatures (zero extra collisions) for **each** first-rational-x31 and actual **Dickson-depth-six B31** base on PRIME31 (p=2^31-1, n=2147446831). This is producer-only evidence [pcc:unverified]. It supplies the *conditional premise*, not independent signed PCC standing. A fresh successful multiplicity run must count its own D4 image and verify the same premise before asserting the following identity conclusion.

**Protocol amendment, before code edit:** In the existing Rust `prime31_s9_b31_signature_multiplicity.rs` source, after the identity Q=O oracle call, assert exactly **one** eight-net signature d=0 when its independently enumerated `h.extra_group_collisions==0`. Emit explicit `identity_four_sum_injective`, `identity_unique_net_zero`, and `identity_x_polynomial_cancellation_count=46376` values (the count derived mathematically rather than exhaustively enumerated). Strengthen its B4 unit test analogously. Keep both B31 bases, natural/positive target seeds, earlier negative regression cases, original output schema, source semantics, work counters, and all prior evidence frozen; no unconditional absence claim on a failed or capped run. Update exact-source CI schema assertions to reject misleading nonzero identity multiplicity. Do not run a generic support-table lookup inside a proposed S9 algebraic extractor.

PASS only after new exact-source Rust 1.100 CI completes both B31 bases and original-curve replay, including unique-zero assertions. This is an **exact-support diagnostic**, not a completed ECDLP solve or end-to-end speed comparison.

## Solver implication

A completed non-generic ECDLP algorithm still requires actual target-conditioned `f` plus sign/orientation extraction from R using existing Rust #4228/#4349 original-ideal-certified quotient, *without* generic 4+4 or scanning 48,903,492 B31 x multisets. Require useful rank32 signed rows and fully cold unknown-scalar solves, and compare to tuned Wiener-Zuccherato rho and mature external PKM/APS/F4/CAS. Existing same-source 48-Q producer full cold reference: APS 109.734280 ms vs unfolded rho64 99.473873 ms, intervals overlap parity. No further speedup or novel non-generic algorithm is asserted.

## Prospective additional squarefree falsifier (amended before source edit)

A squarefree degree-eight polynomial supported on eight **distinct** base abscissae has `||d||_1=8` under every one of its 256 sign orientations. Therefore the zero-fibre theorem predicts **zero** orientations mapping to infinity on the actual Dickson31 base, conditional on the D4-injectivity result. In contrast, a repeated-root `(T-x_0)^8` has exactly one cancelling orientation class mapping to infinity among its nine net signatures.

Extend the already checked-in native Rust `prime31_s9_orientation_fibres.rs` with a `squarefree_identity_sign_orientations` counter, an assertion of 0 over all 256 masks, and exact-source CI check. Retain all previously frozen 256 masks, sign seeds, eight x and 9 repeated-root cases. This provides a direct falsifier for a selected-root S9 solver which spuriously reports squarefree f solutions to Q=O. This amendment changes no existing dataset, target cases or result claims; no standalone cost measurement is implied. [pcc:unverified]
