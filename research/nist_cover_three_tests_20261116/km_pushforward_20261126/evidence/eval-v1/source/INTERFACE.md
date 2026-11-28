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
