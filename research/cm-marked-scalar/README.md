# CM marked scalar: native end-to-end comparison

Conductor task T-1888. This planning PR precedes implementation and comparative execution, as requested by the owner. The open objective is a verified end-to-end improvement over regular scalar multiplication for the marked scalar action. No native speedup has been measured.

The first panel uses P-384 and P-521 and the existing order-seven labels. P-192 remains the primary research target and must receive a compatible native continuation; P-224 and P-256 retain their previously identified map candidates. A win on one curve will be reported on that curve and workload only.

## Exact computation

For each supplied nonzero variable-base point P, compute [mu]P on its original curve. The relation route evaluates the supplied oriented map phi, its normalized opposite map eta, the closing isomorphism sigma(x,y)=(x/ell^2,y/ell^3), then [k], with k=mu/ell mod n. The verifier checks sigma eta phi=[ell] and k ell=mu mod n. This formula is a rational-subgroup evaluator; the global noninteger quotient alpha=pi-c has a different degree from [k]dual(phi).

The fixtures preserve the historical certificates without rewriting them. Their implementation provenance is Python exact arithmetic. They are inputs to a new native implementation, not a native benchmark result. The global residual rational functions and prime-factor kernel chains remain unexpanded.

## Deliverables and acceptance

| Requirement | Evidence required | Current status |
| --- | --- | --- |
| PR before comparative execution | Hosted PR and source revision receipt | PR #3674 opened before implementation and measurement |
| Native source and replay | C/C++ OpenSSL-backed implementation, no Python execution path | 50-case native replay passes; see NATIVE_STATUS.md |
| Exact output | Endpoint, subgroup and scalar reconstruction checks; O and boundary cases | Full polynomial identities and known-answer native fixtures pass; held-out panel pending |
| Matched reference | Same scalar, point, field and backend; optimized public-scalar window method and OpenSSL API reference | Pending |
| Full per-point cost | Setup, map evaluations, closure, correction, conversion and symmetric verification | Historical counted schedules only; native measurement pending |
| End-to-end gain | Frozen paired test with strict improvement beyond A/A noise against the fastest reproduced reference | Unproven |
| Cold and repeated-use accounting | Cold primary; genuine reusable setup reported separately, never silently amortized | Pending |

The historical counts favored direct scalar multiplication. Preserve losses, invalid outputs and timeouts. Do not redefine success as merely publishing a PR or passing a narrower correctness test.

See [BENCHMARK_CONTRACT.md](BENCHMARK_CONTRACT.md) for the intended comparison and [fixtures/PROVENANCE.md](fixtures/PROVENANCE.md) for custody. The execution protocol, source hashes and launch command must be committed and pushed before taking affected measurements.
