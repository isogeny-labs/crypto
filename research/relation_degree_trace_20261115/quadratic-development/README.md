# Direct-quadratic development checks

This directory contains development checks, not a new timing campaign.
The closed quadratic resultant agrees exactly with the existing Sylvester
construction in small-prime, scaled and degenerate test cases; its explicit
original-input certificates are checked. Complete bases agree on the fixed
three-/four-variable development fixtures over both requested primes.
The ordered augmented polynomial lists also equal the old constructor's on
those fixtures. Consequently these cases do not predict changed downstream
solver work; only construction/checking can differ `[pcc:unverified]`.

The campaign integration test uses a fake worker with fixed synthetic costs.
It checks actual selected-baseline dispatch for every paired and A/A child,
and preservation of the protocol label. Its numbers are never research data.

`first-build.log` preserves the initial missing closure-argument type
annotation error, corrected before the passing checks. Final normal/counter
test and strict-clippy logs retain the actual outputs. `source.sha256` pins
their source. No checks or correctness requirements were disabled.

`checkpoint-gate.log` records the real S3 upload-transport failure after
creating a complete resume note in a real sparse Git checkout. No new
scientific measurement was launched after that failure. Root AGENTS.md says
"run scripts/checkpoint.sh gate before launching and do not launch if it
fails". A checkpoint upload token or an authorized AWS transport must be
restored before timing. LEAD_RESUME.md is the durable nonsecret handoff note.
