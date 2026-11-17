# Round17 handoff

## What was done
Implemented Curve reciprocal-context reuse, tested35 release cases and strict
Clippy, froze round17, reconciled remote branch merges, submitted exactly once,
and retained its terminal scheduling failure. Raw records were pushed as
90d1adb048be. Current candidate source is23ff0e6bda63; no measured result exists.

## Decisions
Preserve all150 planned cells and original baseline. Do not invent timing or
execution rows. Last pod observation was pending with no node assignment;
terminal job status is DeadlineExceeded and the expected results object is404.
Zero recorded executions is the observation; it is not a reconstructed trace
of every instant between polls. Read any shorthand statement of no worker
assignment in the failure export in this last-observed-state sense.
The300-second scheduling deadline is not measured algorithm time.

## Open threads
Continue CPU optimization and obtain actual complete candidate measurements
when an eligible execution path is available. Preserve round17 as a failed,
one-attempt panel; any later study needs its own prospective contract.
Native ecbench ic.tensor_contraction_v1 uses ecdl_tensor_ic::Solver with
SparseContractor, a different implementation from this CPU pipeline. It cannot
be substituted for this solver to claim strict-isolation qualification.
The further10x goal, strict isolation, external comparisons and large scale
are incomplete. Check current PR head before the next delivery.

## Errors
Push rejected after a remote merge; fetched and merged without force-pushing
or rewriting frozen evidence. All candidate inputs remained byte-identical.
The initial verifier region hash selected an earlier cfg(test) marker and
hashed an empty slice; corrected before run submission to compare3033 actual
bytes, including verify. No solver source change was needed for that fix.
The job expired before any algorithm outputs were recorded; no retry launched.
Export completeness fails with8 findings; masked analysis was not performed.

The final one-page PDF was rendered and visually inspected. Poppler pdfinfo
printed a Typst metadata warning (`Suspects object is wrong type (boolean)`);
the page opens and its text/diagram are legible. No PDF/A claim is made.
