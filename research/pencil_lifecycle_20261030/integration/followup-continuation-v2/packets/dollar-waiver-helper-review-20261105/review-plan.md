# Preread plan: owner dollar-waiver helper

Source-only independent delta review. Candidate19415004f875e345f4f312788ed2916563c78bd3, helperSHA41123bbcbc8f724c68d9bbe825dd077410a3899fd9f89f97c99b7cd458f981b1 announced by coordinator. File names only inspected before this plan; candidate and diff not yet read. Reviewer did not author runtime/materializer helper. Earlier durable workflow reviews and the explicit owner dollar waiver are known context; not blind to motivation.

Claim to inspect: remove only the intended dollar gate, preserve deadline, worker-time, attempt ownership and resource invariants; do not create an execution authorization or funding renewal. Prior: a narrow waiver can legitimately remove dollar-specific fields, but a semantic change to generic validation or permissive default could weaken unrelated limits.

Owned joints and worked attacks:
1. Exact delta/bindings: compare source bytes to announced hash and prior helper. Attack a renamed or different imported helper, stale manifest or proposal accidentally treated as execution.
2. Dollar scope: trace removed keys/checks and every residual caller. Attack missing fields, bool/nonfinite values, or waived branch skipping the entire admission function rather than only dollars.
3. Non-dollar admission: inspect absolute UTC/monotonic deadlines, worker allowance and attempt ownership checks before payload. Attack expired deadline, exhausted allowance, reused identity, malformed permit, wrong invocation or stale receipt.
4. Runtime limits/custody: ensure resource recipes, output ownership, sealed manifests and cleanup behavior are unchanged or retain explicit external obligations. Attack bypass through helper direct call, import configuration mismatch or inferred runtime readiness.
5. Authorization: proposal remains inert, additional attempt unreleased, no broader permissions or deadline reset. No test or runtime execution is performed by reviewer.

Known-false/source-trace countermodels: dollarwaiver absent; arbitrary full-admission skip; renewed worker ceiling; stale native ownership; expired campaign; false passed marker; helper-hash mismatch. Trace these without executing them. Separate source fidelity from machine status and from independent admission review. Any inherited issue must be distinguished from newly introduced delta.

Wait for exact published714e95 proposal before reading/using its diff or metadata; exact prior candidate194150 is already coordinator-confirmed published. KB unavailable by inherited catalog/wrapper checks; fallback exact primary helper and callers. No global log edits or shared index operations without grant. Output neutral PDF/SVG/JSON with pcc:unverified standing, source line references and actual limitations.
