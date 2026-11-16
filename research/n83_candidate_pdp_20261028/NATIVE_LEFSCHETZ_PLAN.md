# Native Lefschetz continuation

Development contract, 2026-10-30, before native controls or production. Preserve the interrupted r1 and all historical Python sources, controls and logs. Implement a separate Rust binary with native structural arithmetic, explicit squarefree upper witness, leading-term gates and source-bound case joins. Never call the Python verifier from native execution.

Controls: exact block Hilbert identities and dimension sum; wrong ell/field rejection; degree17 truncation boundary; witness annihilation and coefficient mutation detection; valid leading weights for m3–9 and invalid tied weights; complete42 keys and duplicate/missing rejection. Check source/base/input/upper binding and normalization; reject mutated audit/source identities. Native output is a new schema; historical byte serialization is not reused. Fresh replay compares all native JSON records and hash joins. No production result before a new frozen protocol, source identity and backend admission.

Fix SIGTERM handling in the new launcher; retain the historical launcher unchanged. Signal termination must record controller status143; unattempted phases125. Test signal and failure paths before production. Existing r1 exit0 is invalid as documented in its sidecar.

All resulting measurements and certificates remain conditional on PRIME_LEFSCHETZ_ARGUMENT.md; no formal proof, new LFD or degree-of-regularity claim. This development stage does not narrow the Koblitz/general-binary/prime m2–9 fixed-base and elite-base objective.
