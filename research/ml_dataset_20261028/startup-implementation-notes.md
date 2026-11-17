# 2026-10-29 native startup gate implementation

## What was done (commands, files, features)

Implemented startup_gate.rs with fixed typed inputs, descriptor-confined Linux
projection reads, original UTC/boot-clock deadline, retained authority/SQLite
state, immutable accepted inputs, exact grant/build/domain binding, typed native
activation/generation handoff, active-session recovery and ambiguous-generation
refusal. Added eleven internal tests; these are uncompiled and unexecuted while
T-766 reserves CLI mod.rs. Two budget-helper tests passed before the upstream
merge; raw log and receipt were freshly verified from S3.

Merged main7ffaab06bad locally as c25540c97ed. Only two session logs conflicted;
union resolution preserves every original line from both branches in order,
with hashes and subsequence checks retained in session-log-merge-preservation.json.
The merge is not pushed or runtime validated. The last delivered PR2756 head is
still d30b0ade734, validated at runtime source9c3d14b.

## Decisions made (and why)

Keep grant validation and active-phase inspection inside budget.rs to reuse its
private invariants. Startup waits make no SDK/credential call. A durably active
worker advances locally after a crash rather than paying activation again.
Reject worker-writable frozen inputs and retained-authority hard-link aliases.
Check activation/IRSA bootstrap headroom plus the generator remaining-resource
floor before activation. Retain failure/censoring and avoid any new allowance.
Frozen config needs semantic review against protocol prose; byte/schema checking
does not prove that review. Deployment permission and finite resource lifetime
remain separate prerequisites.

## Open threads (unfinished, pick up next time)

Respect T-766's active mod.rs reservation. A concrete ten-line registration
patch is saved. The user approved T-766 coordination, and the patch/handoff request
was posted to its parent PR2801 (comment6487216936). Await the scope handoff.
Once available, reserve the scope, apply
registration, compile/fix the module, run its tests and actual CLI checks, then
strict lint/full-library validation on frozen combined source. Do not push or
claim this gate functional from source review. Complete grant delivery, finite
regional resource admission and the pending worker IAM decision before a real
pilot. No new worker or cloud data cell exists.

Use UPSTREAM-SOLVER-EXTENSIONS.md for the inspected higher-m F4/HKY integration
opportunities. Large primes still require a wide arithmetic implementation;
exact fall-degree labels still require exclusion/closure certificates.

## Errors (what went wrong, how it was handled)

Review found writable frozen-file and retained-worker alias gaps; both were fixed
before final source review. It also found an allowance that could activate but
necessarily fail generation; bootstrap headroom and pre-driver regressions now
cover it. No runtime result is inferred from the review.

The artifact helper uploaded its two objects but initially failed Git staging
on an index lock. After the lock cleared, retry reused both objects and completed;
fresh GET/hash verification passed. A broad whitespace scan reported inherited
warnings in upstream figures and research logs; it was stopped after the scoped
resolved-log check passed. The checked figure directory matches upstream exactly;
no frozen upstream evidence was reformatted. Copilot did not respond to the merge
request for over25 minutes; fallback was explained on PR2756 before local merge.
