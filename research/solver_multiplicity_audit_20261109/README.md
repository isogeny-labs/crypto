# Bivariate multiplicity and actual msolve controls

Lean proves the actual quotient isomorphism
K[x,y]/(xy,x+y−c) ≃ₐ[K] K[t]/(t²−ct). Its dimension is2 for every c.
At c=0 the equations have a unique point; at nonzero c they have two distinct
points. The closed F5 support1/length2 witness disproves identifying distinct
support with multiplicity. Claim PCC-20261109-ee57d0, unsigned; full local
proof gate passes96declarations,0failing,0trusted-native exceptions.
[pcc:unverified]

All16actual msolve0.6.5 calls passed the preregistered exact fixture checks.
The nonreduced fixture prints qdim2 and eliminant degree1; the split fixture
prints2and2. Every returned GB matches the registered polynomial certificate,
passes Buchberger and reduces the actual input. The existing F2known-degree
control returns{x,y} and selected-pair degree3 in both repeats. This is a
successful calibration, not a new refutation of Diem or a solver bug.
[pcc:unverified]

| Fixture | Field | Points | Quotient/staircase | Parameter degree | Repeats |
|---|---|---:|---:|---:|---:|
| xy,x+y | F5 | 1 | 2 | 1 | 2 |
| xy,x+y−1 | F5 | 2 | 2 | 2 | 2 |
| x,y | F5 | 1 | 1 | 1 | 2 |
| x²+x,y²+y,xy+x+y | F2 | 1 | 1 | 1 | 2 |

A separate source audit of the retained c=2preflight found a concrete
coefficient discrepancy: it announces k^344 but prints−2147483648 for k=2.
Lean proves the announced value is1inF5 and the printed integer is2,
and that the mathematical power exceeds int32_t's maximum. Source casts
floating pow before field reduction, crossing the C11 conversion boundary.
Claim PCC-20261109-c7c353 remains unsigned. See
[exact source/arithmetic scope](SOURCE_OBSERVATIONS.md) and
[source proof report](reports/proof-20261109-coeff/report.pdf).
No C/libm trace or incorrect recovered solution is claimed.
[pcc:unverified]

The successful16-call checks validate bases and the registered counts;
they do not verify complete parameter coordinate maps or every genericity
coefficient. The source finding leaves the frozen protocol, native analysis
and raw output unchanged. [pcc:unverified]

The canonical protocol and analysis were committed in
f0bea543060faeeceb9894a009ad11d88de1239d before affected output. Hash:
cff556238d4049de994e63b2f25896ffaf0e8574c1a9b6b6f358b43053cd35b5.
The separate c=2preflight was seen before the freeze and is in-sample
calibration only. Full flow:16planned/attempted/completed,0not-completed,
0interrupted/resumed/excluded. Export file/hash validation passes.
[pcc:unverified]

Original requirements: item8 now has an actual two-variable multiplicity
control, not the general n!dⁿ proof. Item12 has static source review plus
these exact native checks, not C-to-F4Run refinement. Item14 now has an
actual msolve known-degree run; this does not retrofit earlier censuses.
All other obligations and the44-unread-label paper backlog remain in
[the complete map](../theorem_census_20261109/README.md).
[pcc:unverified]

See [protocol](protocol.json), [narrative](PROTOCOL.md), [export](export.json),
[raw analysis](evidence/analysis.jsonl), [provenance](provenance.json),
[latest proof receipt](evidence/proof-gate-coeff.log), and
[report](reports/run-20261109T0409Z/report.pdf).
Formal source:
[BivariateMultiplicity.lean](../theorem_census_20261109/lean/BivariateMultiplicity.lean).
The earlier degree certificate remains in
../theorem_audit_20261108/lean/DegreeControl.lean. [pcc:unverified]

Reproduce on an unmodified pinned msolve checkout with dependencies from
provenance.json extracted into DEPS. Run build-msolve.sh with BUILD set;
set LD_LIBRARY_PATH to DEPS/usr/lib/x86_64-linux-gnu for transitive libraries.
Compile native/analyze.cpp with the recorded g++ flags. run.sh verifies all
frozen file/binary hashes and refuses to overwrite an existing output.
Changed binary paths may alter build hashes; a new registered run must
retain the new identity. The repository Lean gate command is below.
[pcc:unverified]

```sh
tools/lean/proof-gate.sh research/theorem_census_20261109/lean
python3 /workspace/crypto-autoresearcher/tools/experiment_protocol.py check-export \
  research/solver_multiplicity_audit_20261109/export.json \
  --protocol research/solver_multiplicity_audit_20261109/protocol.json --verify-files
```

One local shared host under the user's approved checkpoint exception,
zero paid spend. No accepted clock metric or statistical population claim.
Default upstream genericity may use time-seeded random forms; epoch
intervals/raw outputs are retained, with exact invariant counts checked.
No second-host or signed validation is asserted. A fresh checkout must
hydrate the two archived preflight logs using the repository artifact pull
before checking export hashes; see SOURCE_OBSERVATIONS.md. Earlier missing-timer
stderr was overwritten before preservation; later loader/build failures
are retained and the development deviation is in the session log.
[pcc:unverified]
