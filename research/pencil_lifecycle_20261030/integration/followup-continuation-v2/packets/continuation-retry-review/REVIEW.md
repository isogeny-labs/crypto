# Additive retry clarification review

Source clarification1b3579d3925c6c4e1f498b06c14304eb759126ac6d5f7a26322c9ecaf2598d72 and its manifest match; all parent bindings match. Original proposal and original independent review remain unchanged.

The proposal now explicitly permits at most an initial attempt plus one retry, with no process/owner/request reset. Eligibility requires positive authenticated evidence that payload never started. Missing stdout, rows, receipts, an exec error or Job/Pod status cannot establish nonexecution. Started or uncertain-spawn payloads, post-spawn failures, preemption, restore, unknown timeout, scientific timeout/cap or wrong answer fail closed. Eligibility alone is not admission: live-handle reconciliation, unchanged inputs/instrument, remaining ledger and frozen bindings remain required.

All partial failures, controls, warmups, replay, diagnostics, owner work, collection/recovery and active reporting remain charged. Unknown accounting blocks admission;25,200seconds/06:00UTC remain unchanged. The changed-instrument scientific gate is explicitly unapproved and cannot be reopened by this retry wording. Full cohort/gate denominators and null incomplete outcomes are retained.

One precision remains before freeze: counter_scope names the same exact prepared execution, which could permit two attempts per each of18gate slots instead of two for the whole gate/controller request. Bind the intended counter unit to the actual launcher and state whether the limit is gate-wide, job-wide or per prepared execution. This review does not infer the broader allowance. The actual authenticated launch boundary and receipt verifier also remain source-binding obligations.

The specific no-start and failclosed clarification is supported at document scope; this is not operational enforcement, a canonical freeze or execution admission. No build, scientific execution, cloud action or root edit occurred. These findings are additive to the original review.
