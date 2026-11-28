# Ingress and prefix correction review

Reviewed immutable correction manifest `68abfff2931d226682a37bc4c874a0ffedf7c677e76e734e87c3e4c7eb4e7a04`, patch `77730b5af71785340e6f433b3232235b645242d6d7bd06a727054e6a2a4dd180`; verified all six listed artifact hashes. Source-only review; no main or build-target edits, native execution or cloud operations.

The correction closes both identified production-source gaps. Every Progress field now must be initially empty/default. In particular poisoned lift_choices cannot precede newly authenticated choices. The negative test invokes the real public verifier against a genuinely built CheckedMap and requires Identity before model verification, preserving supplied progress.

After every actual group addition the loop reserves 12 checking work units, then compares the actual prefix identity/x against the named model coordinate. For index zero it checks block zero; internal index i uses block m+i-1 and pattern bit i-1; final index m-1 uses target x/infinity and retains the subsequent exact signed target equality. These indices agree with the separately verified chain. Failure rejects the current mask and retains path/sum until the next admitted mask; admission failure preserves the current partial path/sum. No new budget, deadline, solver or gate bypass is introduced.

Decision: accept this correction for composition and isolated compile/native controls with the original continuation. The two production-source review findings are closed. Activation remains gated as in the original review.

Test scope: the prefix test checks the comparison helper only. It does not exercise the real sign-enumeration loop with an endpoint-valid but intermediate-mismatched model. Keep the requested full verifier/owned-path matched positive and intermediate-mismatch controls as activation obligations; do not count the helper test as fulfilling them. Genuine both-twist nonroot positives and late refusal/capture tests also remain required. No runtime result is claimed by this review.
