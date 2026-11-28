# Expectation-free callable interface

Extension frozen before its implementation/call, 2026-11-26. The successful
twelve-case attempt-v1 sources, inputs and results remain unchanged.

The test verifier reads an expected result only after computing B. Provide
a separate pushforward executable which uses the exact frozen evaluator
functions but accepts no expected-answer or generating-scalar input. Its
arguments are the fixed model/backmap, extracted cycle certificate, actual
pool and formal reference. It returns B([E-Eref]) as an elliptic point.
The source is a new main over a build-only header stripped from attempt-v1's
verify.cpp; do not change that tested arithmetic implementation.

For each of the twelve already extracted cases, call the standalone
evaluator and a native comparison mode against the already checked elliptic
result. All must agree. No RR extraction rerun is needed for this interface
qualification. Retain build/evaluation/comparison/whole-wrapper costs and
hashes; use one CPU,1GiB,build<=300s and each native call<=600s.

Provide evaluate-class.sh as the actual external class-space entry point.
It accepts two qualified build trees, an arbitrary supplied class.json and
a fresh output directory. It snapshots that input and sequentially invokes
the existing extractor and standalone pushforward. It reads no expectation
fields. Hash all fixed/source/input/binary subjects before and after calls,
record both stage exits/costs and the complete wrapper, and fail on any
rejection or cap exhaustion. Qualify that full facade with the retained
external public F(G) class, comparing the point to the existing separate
result. This is an API and representation check, not a performance claim.

Reproduction-path clarification frozen before the explicit-path qualification:
run-eval.sh accepts an optional qualified extraction-tree argument. On a fresh
checkout first run run.sh into a fresh directory, then name that same directory
as the second run-eval.sh argument. The historical default and all frozen
arithmetic/criteria remain unchanged. Qualify this mode in eval-v2 using
the same fixed already qualified extraction tree.

Class-only qualification frozen before its affected call: select only the
class_space field from the unchanged public F(G) fixture using JSON metadata
bookkeeping. Pass that single-key file through the full facade. Require its
result to equal the existing separate public-point result in native comparison
mode. Record the fresh wrapper and nested extraction/evaluation/comparison
costs, digests and exits under the same one-CPU/1GiB/600s native caps.
