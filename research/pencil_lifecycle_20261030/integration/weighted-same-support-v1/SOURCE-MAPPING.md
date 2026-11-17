# Literal source mapping and next runtime obligation

Inspected pinned source on 2026-10-30; hashes in source-bindings.json. This is a source inspection, not a compiled-program proof.

The normalization in projective_law_gate.rs:551–568 reads [q,r,s,t,z,w], rejects zero z,w, and returns u=[r/z^4,q/z^2,1], v=[t/(z^5*w),s/(z^3*w)]. add22_weighted at896–930 constructs opposite-input weight powers, then q1*z2^2, q2*z1^2, r1*z2^4, r2*z1^4, s1*z2^3*w2, s2*z1^3*w1, t1*z2^5*w2,t2*z1^5*w1. Under equal cross-scaled q/r, inverse is tested first, equality second, and zero slope third. These are the statement-level definitions supplied to the independent reviewer.

The native library lib.rs imports ../../../src/cryptanalysis/three_leads.rs, rather than a native/src module. Field::validate permits odd prime study fields p<2^31 and n<=2. For the n=1 path used by the genus-2 base field, canonical input residues below p make multiplication products less than2^62 and addition sums less than2^32. Those integer bounds suggest a concrete finite-word refinement obligation: prove canonical residue preservation, equality/zero/negation compatibility and the modular interpretation for the executed n=1 operations; account separately for operation-count updates and prove the caller actually establishes these premises. A proof over arbitrary abstract fields alone does not discharge that obligation. No claim here that this full refinement has been constructed.

Literature/source context read: independent_complete_law/SOURCE_AUDIT.md pins Ozbay Gurler–Hisil2024 DOI10.1007/s13389-024-00351-7 and incorporated author code. Its prior source audit attributes the weighted same-support formulas to equations19–21. This new task uses literal source expressions and already reviewed polynomial identities; it introduces no novelty or timing claim.

An initial lookup guessed native/src/three_leads.rs and failed; inspecting native/lib.rs identified the actual path above. No source edit or executable run followed that failed lookup.
