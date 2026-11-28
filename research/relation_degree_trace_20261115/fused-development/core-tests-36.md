# Streamed reducer core qualification

The checked streamed shift implementation passes57 default,58 scale-screen and59 scale-screen/count-field-ops release tests, and strict all-target release lint. Logs tests-default-36.log, tests-scale-36.log, tests-counter-36.log and clippy-36.log are the subjects. Differential cases compare exact remainders/provenance/counters for both primes, zero/nonzero shifts, nonmonic non-Groebner reducers, invalid-dimension collision fallback and overflow-before-inverse/panic restoration. No root integration or timing conclusion is made here. [pcc:unverified]
