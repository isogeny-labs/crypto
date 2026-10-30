# Independent solver correction03 closeout

Source-only review: no remaining blocking finding within the reviewed native
policy-domain migration and companion central-replay scope. This is not a
compile/test pass or end-to-end V2 activation approval.

Reviewed stable scratch factor-base-solver-v2-03:

- solvers.rs.proposed:731e261b8b044c688f655c43748bbfa9fa341ce78f3f7b4b4b80abd189a8c633
- native-domain-combined.patch:f3711caf6c15744ff83ea69d171199fb877b954953c92968001445a01b670292
- incremental-from-reviewed-02.patch:56620aca75d042f91149f2b00b64df6f326b543a45581661c37109012f27414e
- held-mod-domain-replay.patch:dda303f0b052e2772b8acd20dc41e5191fd06530c26e50461760689b7b2b4b45

## Findings resolved

PKM height replay now compares the saved definition.t to pkm_height(p,m,work_limit).
The helper preserves the original schedule verbatim: initial v2(p-1) capped at4,
then decrease while height>1 and grid size exceeds min(work_limit,TRACE_EVENT_CAP).
Native construction, admission reconstruction and V2 artifact verification reuse
that schedule. The V1 producer therefore has the same height and domain bytes;
the new strict domain comparison applies only to signed V2.

The new other-height test generates a complete native domain at public p61 with
m2/work16 (t2), and another complete domain on the same source at work4 (t1).
It first proves the lower domain passes the existing mathematical/native-column
checker, then requires rejection under the higher-work configured V2 domain,
while accepting the correct higher domain. This distinguishes a true semantic
substitution from a malformed one-field edit. It sets private_witness to null
before solver invocation. Test source is meaningful but remains unexecuted.

Unsupported-profile replay now requires status=unsupported, no PKM/relation
domain artifact and no true completion/PDP/scalar/exhaustive claims. The new
binary-PKM test admits the honest unsupported result and rejects changed status
and positive claims. Unscoped supervisor interruptions still rely on the prior
verify_policy_solver_outcome check (explicit incomplete status and no positive
claims); the helper's early return is not standalone admission authority.

## Prior checks preserved

Byte comparisons confirm relations.rs.proposed and the held mod companion are
unchanged from reviewed02. The earlier low-work pre-domain gates, full retained
source ordering/x fibers, alternative-domain population checks, empty BSGS
exception, partial-artifact role restrictions and checked worker/final outcomes
remain applicable. Original factor-base-solver-v2-01 reviewed02 source and
patch hashes remain c92a9087/846545d8; prior review02 is untouched.

The combined migration still depends on all reviewed V2 producers/readers and
mandatory custody. Full integrated tests, donor reconciliation, source guards
and actual bounded CLI activation remain root obligations. All10 solver tests
and3 companion tests are authored, not executed by this reviewer. No source
edits, native execution, Cargo, cloud, or Git mutations were performed.
