# Reproduce fixed solver qualification

Verify SOURCES.sha256 against the two archives in FROZEN.md. Rust1.100.0
was used. Run the commands in tools/torus-sparse-elimination/README.md with
a NEW output directory; the executable refuses to replace an existing one.
Pre-call.json records six compiled-in hashes, limits and binary hash BEFORE
running controls. Controls.json retains complete returned assignments,
stop classes, counter vectors and independent row replay. The exact
71-variable/81-equation original q1/k2 input is a separate JSON artifact.

No planted labels, signs, point list, scalar or original witness enters
solve: the public target is encoded in the original System. The harness
constructs a known-answer target separately. This tests correctness, not
uniform-target yield or a large-base success probability.

Original/reference/trace working sources have not changed. Coupled quadratic
controls exercise Buchberger; the actual q1/k2 root uses propagation only.
The106 sqrt comparisons are deterministic unit coverage; the deliberately
forced full-input refusal is not a full solve attempt. No timing, paid job,
solver sampling/campaign or broad root-library rerun occurred.

Counters preserve their limited scope in QUALIFICATION_SCOPE.md. Full cold
accounting, allocation/RSS evidence and any comparative performance result
remain open. Never price a capped/unsupported/uncertified observation as an
absence proof or use a small-case degree/event count as a scaling exponent.
