---
name: report-evidence
description: Preserve user requirements and report experimental evidence without substituting agent judgments. Use when implementing research requests, running experiments or benchmarks, assessing completion, reporting limitations, or writing performance claims and handoffs.
---

# Report evidence

## Lead with the result

Owner instruction, 2026-10-30: report the construction, implementation,
improvement, or checked observation first, at its actual scope. A verified
component is progress. Give the supporting checks and the next technical step.
State a material dependency once where it affects a decision.

Routine progress reports and handoffs do not need an overall ECDLP verdict or
a rho comparison. Include those only in an explicitly scoped evaluation, a
required comparison table, or a specific factual correction. Continue useful
authorized work through open dependencies.

## Preserve the request

- Record the user's requested deliverables, parameters, workload, comparison, and acceptance criteria before changing them. Use a concise requirement-to-evidence table for substantial tasks.
- Execute authorized, feasible experiments as requested. Do not cancel, replace, downscale, or omit them because you predict a negative outcome or think they are uninteresting.
- Use routine implementation judgment to carry out the request, not to redefine it. If an actual blocker prevents the requested run, name it and keep the original requirement open. Label any smaller diagnostic separately; it does not discharge the requested experiment.
- Do not silently weaken a mathematical argument, its hypotheses, a specification, test, acceptance threshold, or validation gate. Proposed corrections must identify the original statement, the issue, and the exact change.

## State actionable dependencies

- Distinguish assistance restrictions, unavailable tools/access, resource limits, implementation gaps, untested hypotheses, and proved mathematical obstructions.
- If a higher-priority restriction prevents assistance, state that boundary explicitly. Never fabricate a mathematical impossibility, runtime estimate, failed experiment, or missing capability to disguise it.
- Honor higher-priority instructions and actual authorization limits. This skill does not authorize bypasses. Explain any resulting scope limitation; do not pretend the substitute fulfills the original request.
- Name a missing implementation, measurement, or proof concretely, with the next check or construction. Scope an impossibility result to its proof and assumptions; record a bounded search with its actual coverage.

## Run and preserve evidence

- Retain exact commands, code revisions, inputs, seeds, environment, budgets, raw outputs, exit status, and validation results. Preserve unsuccessful attempts, timeouts, OOMs, duplicates, regressions, and missing measurements.
- Follow the requested accounting interval and comparison. Identify setup, online work, verification, and excluded costs; never silently change the denominator or replace a single-target result with a batch average.
- Keep correctness checks. Flag invalid results and implementation defects without suppressing them or promoting them as successes. Do not report a planned or launched run as completed.
- Leave the run's data as one `experiment.export/v1` beside the raw records (`experiment-protocol` skill), its `flow` block (planned, attempted, completed, excluded by rule) in the report, and cite any `review-run` record another runner filed; a row resumed after a Spot interruption keeps its counts and has no clock.

## Separate measurement from interpretation

- Lead with observed values, units, sample counts, uncertainty, comparison conditions, and evidence links. Report arithmetic ratios with their exact measured scope.
- Keep observations, derived calculations, hypotheses, extrapolations, and optional interpretations explicitly distinct.
- Leave research significance and priority to the user. Do not independently label a result "massive", "breakthrough", "promising", "worthless", or "not worth running". Do not abandon an authorized experiment on that basis.
- If asked for interpretation, supply a clearly labeled, evidence-bounded assessment with assumptions. Continue to identify factual errors, validity failures, and uncertainty; deference is not permission to endorse unsupported claims.
- Do not extrapolate a stage improvement into an end-to-end improvement, a toy result into general support, or one measured regime into another.

## Report completion against the request

- Mark each requirement as verified complete, implemented but unverified, partial, blocked, or not attempted, with evidence or the precise gap.
- Use "complete" only when all requested acceptance criteria are satisfied, or explicitly qualify which narrower deliverable is complete. A merged PR and passing tests do not establish broader mathematical coverage.
- Before finishing, compare the deliverable to the current request and name its remaining dependencies. Keep the broader research objective in the work plan rather than repeating its status after each component. State background progress only when a real running job or scheduled task exists.
- Correct an earlier misleading claim explicitly; preserve the historical evidence and explain what changed.

## Reporting pattern

For substantial work, provide:

1. Result or completed component, with its exact scope.
2. Supporting checks or measurements and reproducible evidence.
3. A material dependency or deviation, when present, and the next technical step.
4. Interpretation only when requested, clearly separated.

Example: "The polynomial evaluator improved from 120 ms to 80 ms on the matched workload, a 1.50x component speedup, with identical outputs. Next, integrate it into the extractor and measure the complete solve."
