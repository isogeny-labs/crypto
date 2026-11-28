# G2: retained setup and oracle-reference stages

Source c933e1bdef0e98e3c37282d6ec313565c432f3af. This extends the earlier
f866 replay accounting; old receipts validate only their own revisions.
All result observations are unsigned, PCC-20261105-6020c1. [pcc:unverified]

## Changed ownership

The DLP replay caller owns setup's affine Curve and an explicit setup state.
Early parameter/schema refusals and late full-curve-order refusals retain the
instrumented reference prefix. The oracle checker now has separate validation,
submitted-witness and exhaustive-enumeration owners. A caller accumulates all
three stages before propagating Err; attempted/failed oracle-call diagnostics
are distinct from completed-call counts. This also retains witness arithmetic
that the previous successful checker discarded. [pcc:unverified]

The old certificate reference_counts and completed_oracle_counts remain the
legacy enumeration-only view. They overlap attempted enumeration; never add
both. Within a stage, point/inverse diagnostics overlap expanded field children.
Stages are disjoint intervals but their classes are nested views, not prices.
Scalar n remains separate from coordinate p. Legacy affine overflow is still
unqualified. No instructions/cycles/GAE or complete cost is computed.

Nested replay-work schema changes fromv1 tov2 because setup now always emits
an owned count snapshot plus not_started/in_progress/returned/rejected state,
and attempted stage counts replace the old missing-failed-work flag. Outer
replay certificate staysv2; requests and producer outputs stayv1. The actual
CLI checkpoint remains before input/replay. The producer setup was refactored
through the same counting owner: its successful numerical counts remain covered
by regressions, but the old f866 byte-identical prefix claim does NOT apply to
this changed source. No producer timing or speedup is admitted.

## Meaningful controls and retained failure

The fresh full oracle suite has27passes,0failures,0ignored. Four new controls
check success/lost-output conservation across disjoint ledgers, exact corrupt-
witness prefix and duplicate early refusal, retained F257 field scan at the
one-million-tuple budget refusal, and the exact F17 full-order-refusal scan
without oracle/rank work. An existing error control now checks retained oracle
validation after a binding refusal and retained validation after setup schema
refusal. Existing fixed m4/all-target/group/rank controls remain present.
Formatting, CLI build and actual checkpoint-refusal message+expectedexit1 pass;
final scoped Cargo lint/supervisor FAIL101 on two inherited library errors.
Strict lint of the final Cargo example remains unqualified. [pcc:unverified]

Initial sourceb013 failed test compilation because unwrap_err required Debug
on Certificate. ZERO tests ran. The assertion was changed to extract Err with
an explicit expectation and the same message check. Its source/recipe/22object
bindings/logs/terminals remain in initial-b013cb1e3/; it never validates final
code. Final sourcec933 has a NEW recipe/directory/bindings, not an old restart.
[pcc:unverified]

## Remaining coverage and original acceptance

Parameter guards, JSON/binding work, setup constructor helpers/shared relation
encoder, allocation/copy/comparison/branch/serialization/counter overhead,
whole process/memory/journal and qualified runtime/resume remain uncovered.
Only explicitly instrumented reference calls are retained; Result returns are
not panic/signal/crash durability or execution proof. Work owners are internal
and are constructed per attempt by the existing caller, not signed attestation.
Standalone oracle verify still returns its legacy certificate rather than a
new durable process journal. Complete F4/root/logical coverage and a calibrated
nonoverlapping charge frontier remain open. Replay is not free, and its audit
work is not silently folded into producer intervals.

| Goal | Current scope |
|---|---|
| G1 | Partial; multi-curve complete-output qualification open |
| G2 | Partial; new retained reference stages, complete pricing/runtime open |
| G3 | Partial; qualified equal-domain external comparisons open |
| G4 | Unrun held-out optimization panel |
| G5 | Unrun scaling/reach panel |
| G6 | Partial; existing formal scope unchanged, Rust/polynomial/cost bridges open |

No scientific panel; all metrics null/comparison_admissible=false. Full-panel
historical card remains blocked/unfunded/unimplemented. No checkpoint/protocol/
runtime or access/funding/lease/IAM/quota/pool change. Frozen original-ledger
fork remains report-only; no new main merge/rechain or gate waiver. Older lint,
PCC, clock-control failures and full-suite cap remain at their recorded scopes.
No duplicate unchanged library/Lean/process-prerequisite tests were run.

Sources checked before implementation: prior G2 audit/ownership/replay notes,
actual reference check/all and setup/replay code, ICMS units/accounting rules,
and official Rust Result ownership documentation <https://doc.rust-lang.org/std/result/>.
This reuses existing affine arithmetic and exhaustive correctness controls.

Evidence: validation/reference-refusal-20261105/{initial-b013cb1e3,final}/receipt.json.
New two-page PDF/SVG accompany this report. Canonical IC scoreboard/progress-
timeline, curve and performance views remain unchanged: no admitted metric,
curve edge, phase price or reference. The diagram shows software ownership
because there are no comparable performance values.
