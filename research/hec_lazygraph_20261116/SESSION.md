# Session 2026-11-15

## What was done
Read current sparse controls and original broader plan. Prior turn was progress: pushed60d9c9b503. Reviewed signed graph primary text, harvesting publisher abstract, Wiedemann1986 and Kaltofen/Saunders1991 author-hosted full texts. ClaimedT6565. Implemented deferred graph expressions and forward/transpose operators, independent complete row-space checks and constructed fill fixture.

## Decisions
Retain full mathematical tenfold goal. Do not conflate lower matrix application cost with total solve gain or originality: circuit transposition is explicit prior art. Derive exact LP elimination span, not just witness residuals. No timing. Shared daily log remains reserved by T6448; preserve project handoff here.

## Open threads
Freeze/checkpoint then once-only correctness gate. Curve integration and charged costs need later protocols; no black-box linear solver wired yet.

## Errors
Duke-hosted primary PDF returned403; author NCSU copy succeeded. No execution failures at this point.

## Completed synthetic gate
Frozen ef4473fcf94a/checkpoint043823Z gate executed once:36streams,1440prefixes, independent rank/containment and full operator basis checks pass, exit0/empty stderr. Constructed fixture:98431expandednonzeros,2560nodes,512leafentries,1535sums; conservative multiplication bound3582perapply. This is a representation diagnostic on a synthetic stream. Next actual-curve gate frozen separately, explicit solver retained as correctness oracle.

## Completed actual-curve gate and next decision
Frozenbe5e94cd2808/checkpoint043935Z executed once:12exact reference comparisons and3additional supplied-target recoveries pass, all emitted full operators and transposes pass basis checks, exit0/emptystderr. Circuit multiplication bound exceeds expanded nnz in all15cases (e.g.g2:911versus524, g4:339versus160). This rejects promoting the first circuit representation on these controls. Root cause visible in raw leaves: cumulative walk-count vectors remain repeatedly stored. Next hypothesis: share walk-count prefixes inside the circuit while preserving original relation rows, not difference the rows sent to the solver. Separate protocol/source freeze required before execution.

## Prefix preparation error
The source-generation edit initially expected indentation absent from the authoritative clone; it stopped before creating the new binary source. The subsequent build consequently found no prefix binary. Retained BUILD-PREFIX-INITIAL.txt, corrected markers from actual source and rebuilt before any execution. No scientific gate ran during this preparation error.

## Completed prefix gate and publication decision
Frozen83fa8ce7331f/checkpoint044149Z executed once:12reference-equivalent cases and3supplied-target source checks pass, complete operator/transpose basis checks pass, exit0/emptystderr. Prefixbound improves initial circuit for everyg2/g3case and worsens everyg4case; against expanded rows favorable4/5g2,0/5g3,0/5g4. Do not promote circuit-only work into tenfold evidence or spend timing cohort on this gate. RESULTS.md includes all15cases; two-page REPORT.pdf rendered and every page inspected. Next investigate relation-production/retained-algebra mechanisms with prior anchor work screened first. Goal remains active. No PR merge.
