# Mumford congruence under affine normalization

T-1135, 2026-10-31. Producer formal proof packet. Independent source-fidelity review pending.

For a field K, nonzero a and any natural d, define T_n(p)=a^(-n)p(aX+b). The checked construction transports u | (f-v²) to T_d(u) | (T_6(f)-T_3(v)²). If f-v²=u*q, its explicit quotient is (a^d/a^6)q(aX+b). Square/difference compatibility, evaluation, root preservation, identity support and zero ordinate are separate declarations.

This matches the intended scaling in research/five_leads_e2e_20261019/native/genus2_model_gate.rs:14–56. The previous scholten-model-normalization packet supplies the concrete curve substitution. These are algebraic bindings; literal Rust-loop/compiler correspondence, monicity and degree bounds, global projective transport and divisor-class interpretation remain separate work.

Fixed rational controls use f=X^5+1,u=X,v=1,q=X^4,a=2,b=3,d=1. At transformed support root -3/2, the ordinate is 1/8 and curve value 1/64. The correct residual is zero; omitting ordinate scaling gives -63/64, nonzero. A zero scale collapses the coordinate and is excluded from the transport theorem. The fixed divisibility instance is checked as well.

Attempt1 failed on polynomial C syntax and a looping simp set; attempt2 corrected these. The first control compile timed out at120 seconds; the identical retry passed. Raw failures and nonfatal warnings are retained. Final statuses and named axiom readback appear in status.json. The launcher checks native exit status and unchanged source/dependency hashes. This is producer Lean checking with existing compiled dependencies, not an independent binary or semantic review.

Primary documentation reviewed: [Mathlib polynomial evaluation/composition](https://leanprover-community.github.io/mathlib4_docs/Mathlib/Algebra/Polynomial/Eval/Defs.html). Existing formulas are reused; no novelty claim. No scientific execution, enumeration, timing or cost ratio is included. The expired campaign envelope is unchanged. The graph is an algebraic dependency diagram because this packet contains no comparative measurements.
