# Native shared-window contract checks

One rustc standalone test build capped120seconds, then one test invocation
capped30seconds. Four new methods: exact saved windows at Q=O,P,2P,-P on the
original F509/r479 fixture; identity-only/empty/invalid shifts; immutable prepared
reuse and individual-grid agreement; mixed-add exceptional cases.

Golden records are jq projections of the sealed r479 candidate case file.
Case IDs happen to equal original checker scalars, but are consumed only by
test fixture selection: Prepared/window receives public shifted points, no ID
or scalar. No sign/decoder/source-resultant computation runs. This software
contract check does not replace the complete4563-case recovery cohort.
No timing comparison or cloud job. Source/fixture/original-file hashes retained.
Additional public-prefix validation is an explicit cost beyond the old window.
